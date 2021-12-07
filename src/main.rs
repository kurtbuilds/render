use std::collections::HashMap;
use clap::Parser;
use crate::envfile::EnvFile;
use reqwest::{blocking as reqwest};
use serde_json;
use std::env;

mod envfile;


#[derive(Parser)]
#[clap(about, version, author)] // Pull these from `Cargo.toml`
struct Cli {
    file: String,
    url: String,
}

fn main() {
    let args = Cli::parse();
    let env = EnvFile::read(&args.file);

    let mut pairs = Vec::new();
    for (k, v) in &env {
        pairs.push([
            ("key".to_string(), k.to_string()),
            ("value".to_string(), v.to_string()),
        ].into_iter().collect::<HashMap<_, _>>());
    }

    eprintln!("Pushing {} to {}", serde_json::to_string(&pairs).unwrap(), args.url);

    reqwest::Client::new()
        .put(args.url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&pairs).unwrap())
        .bearer_auth(env::var("MODENV_TOKEN").unwrap())
        .send()
        .map(|mut res| {
            println!("{}", res.text().unwrap());
        })
        .map_err(|e| eprintln!("Failed to create request: {}", e));
}
