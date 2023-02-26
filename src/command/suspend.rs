use crate::{api, Cli};
use std::collections::HashMap;
use clap::ArgMatches;
use crate::command::util;
use anyhow::anyhow;
use clap_derive::parser as Parser;

#[derive(Parser, Debug)]
pub struct Suspend {
    services: Vec<String>,
}

impl Suspend {
    pub fn run(&self, args: &Cli) -> anyhow::Result<()> {
        let token = &args.token;
        let runtime = util::runtime();
        let services = self.services.iter();
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
                    })
            }).unwrap();
        }
        Ok(())
    }
}