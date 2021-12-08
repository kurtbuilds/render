use std::collections::HashMap;
use clap::Parser;
use crate::envfile::EnvFile;
use serde_json;
use std::env;
use reqwest::Error;
use serde::{Serialize, Deserialize};
use crate::api::update_env_vars;

mod envfile;
mod api;

#[derive(Parser)]
#[clap(about, version, author)] // Pull these from `Cargo.toml`
struct CommandArgs {
    #[clap(about = "The API key. Can be set with env var RENDER_TOKEN.", long = "token")]
    token: String,
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
        let token = &parent_args.token;
        let env = EnvFile::read(&self.file);
        let mut pairs = Vec::new();
        for (k, v) in &env {
            pairs.push([
                ("key".to_string(), k.to_string()),
                ("value".to_string(), v.to_string()),
            ].into_iter().collect::<HashMap<_, _>>());
        }

        let services = api::list_services(&token);
        let service = services.iter().filter(|s| s.name == self.service).next().unwrap();

        api::update_env_vars(token, &service.id, &pairs)
            .map(|mut res| {
                println!("{}", res.text().unwrap());
            })
            .map_err(|e| {
                eprintln!("Failed to create request: {}", e);
                e
            })
    }
}


fn main() {
    let mut args = CommandArgs::parse();
    if args.token.is_empty() {
        args.token = env::var("RENDER_TOKEN").unwrap();
    }

    match args.subcommand {
        Subcommand::PutEnv(ref put_env) => {
            put_env.run(&args).unwrap();
        }
    }
}