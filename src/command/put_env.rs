use clap::ArgMatches;
use std::collections::HashMap;
use crate::{api, EnvFile, EnvVar};
use crate::command::util::runtime;

pub fn put_env(token: &str, args: &ArgMatches) -> anyhow::Result<()> {
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

    let runtime = runtime();

    let services = runtime.block_on(api::list_services(token))?;
    let service = args.value_of("service").unwrap();
    let service = services.iter().find(|s| s.name == service).expect("Service not found. Have you created it on render.com yet?");

    runtime.block_on(async {
        api::update_env_vars(token, &service.id, &env_vars)
            .await
            .map(|_env_vars| {
                println!("Updated environment variables.")
            })
            .map_err(|e| {
                eprintln!("Failed to create request: {}", e);
                e
            })
    })
}
