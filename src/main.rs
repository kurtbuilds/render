use std::collections::HashMap;
use clap::Parser;
use crate::envfile::EnvFile;

use std::env;
use reqwest::Error;
use crate::api::EnvVar;


mod envfile;
mod api;

#[derive(Parser)]
#[clap(about, version, author)] // Pull these from `Cargo.toml`
struct CommandArgs {
    #[clap(about = "The API key. Can be set with env var RENDER_TOKEN.", long = "token")]
    token: Option<String>,
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(Parser)]
enum Subcommand {
    #[clap(about = "Update the service environment variables", name = "put-env")]
    PutEnv(PutEnv),
}


#[derive(Parser)]
struct PutEnv {
    service: String,
    file: String,
}


trait RunnableSubCommand<ParentArgs> {
    fn run(&self, parent_args: &ParentArgs) -> Result<(), Error>;
}


impl RunnableSubCommand<CommandArgs> for PutEnv {
    fn run(&self, parent_args: &CommandArgs) -> Result<(), Error> {
        let token = parent_args.token.as_ref().unwrap();
        let env = EnvFile::read(&self.file);
        let mut pairs = Vec::new();
        let env_vars = &env.into_iter().map(|(k, v)| {
            EnvVar {
                key: k.to_string(),
                value: v.to_string(),
            }
        })
            .collect::<Vec<EnvVar>>();

        let services = api::list_services(token)?;
        let service = services.iter().find(|s| s.name == self.service).unwrap();

        api::update_env_vars(token, &service.id, &pairs)
            .map(|env_vars| {
                println!("{:?}", env_vars)
            })
            .map_err(|e| {
                eprintln!("Failed to create request: {}", e);
                e
            })
    }
}


fn main() {
    let mut args = CommandArgs::parse();
    if args.token.is_none() {
        args.token = Some(env::var("RENDER_TOKEN").unwrap());
    }

    match args.subcommand {
        Subcommand::PutEnv(ref put_env) => {
            put_env.run(&args).unwrap();
        }
    }
}