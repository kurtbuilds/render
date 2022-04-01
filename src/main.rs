#![allow(unused)]
use std::borrow::Cow;
use std::collections::HashMap;
use clap::ArgMatches;
use crate::envfile::EnvFile;
use futures::stream::FuturesUnordered;

use std::{env, thread};
use anyhow::anyhow;
use anyhow::Result;
use colored::Colorize;
use futures::{stream, StreamExt};
use slice_group_by::GroupBy;
use tabular::Row;
use command::{deploy, list, put_env};
use crate::api::{DeployCursor, EnvVar};

mod envfile;
mod api;
mod command;

type ResultVec<A, B> = Result<Vec<A>, B>;


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
            .arg(clap::Arg::new("env_files")
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
            put_env::put_env(token, args)
        }
        ("deploy", args) => {
            deploy::deploy(token, args)
        }
        ("list", args) => {
            list::list_services(token)
        }
        _ => unreachable!()
    }
}