#![allow(unused)]
use std::borrow::Cow;
use std::collections::HashMap;
use clap::{ArgMatches};
use crate::envfile::EnvFile;
use futures::stream::FuturesUnordered;

use std::{env, thread};
use anyhow::anyhow;
use anyhow::Result;
use colored::Colorize;
use futures::{stream, StreamExt};
use slice_group_by::GroupBy;
use tabular::Row;
use crate::api::{DeployCursor, EnvVar};

mod envfile;
mod api;

type ResultVec<A, B> = Result<Vec<A>, B>;

fn put_env(token: &str, args: &ArgMatches) -> Result<()> {
    let mut map = HashMap::new();
    let env_files = args.values_of("env_files").unwrap();
    for file in env_files {
        let env = EnvFile::read(file);
        for (key, value) in &env {
            map.insert(key.to_string(), value.to_string());
        }
    }
    let env_vars: Vec<EnvVar> = map.into_iter().map(|(k, v)| {
        EnvVar {
            key: k,
            value: v,
        }
    })
        .collect::<Vec<EnvVar>>();

    let services = api::list_services(token)?;
    let service = args.value_of("service").unwrap();
    let service = services.iter().find(|s| s.name == service).unwrap();

    api::update_env_vars(token, &service.id, &env_vars)
        .map(|_env_vars| {
            println!("Updated environment variables.")
        })
        .map_err(|e| {
            eprintln!("Failed to create request: {}", e);
            e
        })
}

fn deploy(token: &str, args: &ArgMatches) -> Result<()> {
    let services = api::list_services(token)?;
    let service = args.value_of("service").unwrap();
    let service = services.iter().find(|s| s.name == service).unwrap();

    api::trigger_deploy(token, &service.id)
        .map(|deploy| {
            println!("Watch deploy at https://dashboard.render.com/{}/{}/deploys/{}", service.type_, service.id, deploy.id)
        })
}

fn list_services(token: &str) -> Result<()> {
    let services = api::list_services(token)?;

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()
        .unwrap();

    let client = httpclient::Client::new(None);
    let fetches = stream::iter(services)
        .map(|service| async {
            let deploys = api::list_deploys(token, &service.id, 1).await;
            (service, deploys)
        })
        .buffer_unordered(16).collect::<Vec<_>>();

    let mut rows = runtime.block_on(fetches);

    let mut table = tabular::Table::new("{:<} {:<} {:<} {:<}");
    // let mut rows = Vec::new();
    // for (service, deploys) in services.into_iter().zip(outputs.into_iter()) {
    //     let deploy = deploys.unwrap().pop().unwrap();
    //     rows.push((service, deploy));
    // }
    rows.sort_by(|a, b| a.0.name.cmp(&b.0.name));
    let groups = rows.linear_group_by_key(|(service, deploy)| service.name.splitn(2, '.').next().unwrap().to_string())
        .collect::<Vec<_>>();

    table.add_row(tabular::row!("SERVICE", "STATUS", "SERVICE ID", "URL"));
    for rows in groups {
        for (service, deploy) in rows {
            let deploy = deploy.as_ref().unwrap().get(0).unwrap();
            table.add_row(Row::new()
                .with_cell(service.name.clone())
                .with_cell(match deploy.status.as_ref() {
                    "live" => Cow::Owned("LIVE".green().to_string()),
                    "build_failed" => Cow::Owned("BUILD FAILED".red().to_string()),
                    "update_failed" => Cow::Owned("UPDATE FAILED".red().to_string()),
                    s => Cow::Borrowed(s),
                })
                .with_cell(service.id.clone())
                .with_cell(service.url())
            );
        }
        table.add_heading("");
    }
    print!("{}", table);
    Ok(())
}


const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

fn main() -> anyhow::Result<()> {
    let args = clap::Command::new(NAME)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(clap::Arg::new("token")
            .env("RENDER_TOKEN")
        )
        .subcommand(clap::Command::new("put-env")
            .arg(clap::Arg::new("service")
                .required(true)
                .help("The service name")
            )
            .arg(clap::Arg::new("env-files")
                .required(true)
                .multiple_values(true)
                .help("The env files to read")
            )
        )
        .subcommand(clap::Command::new("deploy")
            .arg(clap::Arg::new("service")
                .required(true)
                .takes_value(true)
                .help("The service name")
            )
        )
        .subcommand(clap::Command::new("list")
            .alias("ls")
        )
        .get_matches();

    let token = args.value_of("token").unwrap();
    match args.subcommand().unwrap() {
        ("put-env", args) => {
            put_env(token, args)
        }
        ("deploy", args) => {
            deploy(token, args)
        }
        ("list", args) => {
            list_services(token)
        }
        _ => unreachable!()
    }
}