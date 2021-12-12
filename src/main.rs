use std::collections::HashMap;
use clap::Parser;
use crate::envfile::EnvFile;

use std::env;
use anyhow::anyhow;
use reqwest::Error;
use crate::api::EnvVar;

mod envfile;
mod api;


#[derive(Parser)]
#[clap(about, version, author)] // Pull these from `Cargo.toml`
struct CommandArgs {
    /// The API key. Can be set with env var RENDER_TOKEN.
    #[clap(long = "token")]
    token: Option<String>,

    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(Parser)]
enum Subcommand {
    /// Update the service environment variables
    #[clap(name = "put-env")]
    PutEnv(PutEnv),
    Deploy(Deploy),
}


#[derive(Parser)]
struct PutEnv {
    service: String,
    env_files: Vec<String>,
}

#[derive(Parser)]
struct Deploy {
    service: String,
}

trait RunnableSubCommand<ParentArgs> {
    fn run(&self, parent_args: &ParentArgs) -> Result<(), anyhow::Error>;
}


impl RunnableSubCommand<CommandArgs> for PutEnv {
    fn run(&self, parent_args: &CommandArgs) -> Result<(), anyhow::Error> {
        let token = parent_args.token.as_ref().unwrap();
        let mut map = HashMap::new();
        for file in &self.env_files {
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
        let service = services.iter().find(|s| s.name == self.service).unwrap();

        api::update_env_vars(token, &service.id, &env_vars)
            .map(|_env_vars| {
                println!("Updated environment variables.")
            })
            .map_err(|e| {
                eprintln!("Failed to create request: {}", e);
                e
            })
    }
}


impl RunnableSubCommand<CommandArgs> for Deploy {
    fn run(&self, parent_args: &CommandArgs) -> Result<(), anyhow::Error> {
        let token = parent_args.token.as_ref().unwrap();
        let services = api::list_services(token)?;
        let service = services.iter().find(|s| s.name == self.service).unwrap();

        api::trigger_deploy(token, &service.id)
            .map(|deploy| {
                println!("Watch deploy at https://dashboard.render.com/{}/{}/deploys/{}", service.typ, service.id, deploy.id)
            })
    }
}


fn main() {
    let mut args = CommandArgs::parse();
    if args.token.is_none() {
        args.token = Some(env::var("RENDER_TOKEN").expect("You must supply --token in your args or RENDER_TOKEN in your env."));
    }

    match args.subcommand {
        Subcommand::PutEnv(ref put_env) => {
            put_env.run(&args).unwrap();
        }
        Subcommand::Deploy(ref deploy) => {
            deploy.run(&args).unwrap();
        }
    }
}
