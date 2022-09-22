use crate::{api};
use std::collections::HashMap;
use clap::ArgMatches;
use crate::command::util;
use anyhow::anyhow;


pub fn suspend(token: &str, args: &ArgMatches) -> anyhow::Result<()> {
    let runtime = util::runtime();

    let services = args.values_of("services").unwrap();
    let available_services = runtime.block_on(async {
        api::list_services(token).await
    })?;
    let available_services = available_services.into_iter().map(|s| (s.name, s.id)).collect::<HashMap<String, String>>();

    let services = services.map(|s| {
        Ok::<_, anyhow::Error>((s, available_services.get(s).ok_or_else(|| anyhow!("Service {} not found.", s))?.as_str()))
    }).collect::<Result<Vec<_>,_>>()?;

    for (name, id) in services {
        runtime.block_on(async {
            api::suspend(token, id)
                .await
                .map(|_env_vars| {
                    println!("Suspended service: {}", name)
                })
                .map_err(|e| {
                    eprintln!("Failed to suspend service {}: {}", name, e);
                    e
                });
        })
    }
    Ok(())
}
