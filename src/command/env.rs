use anyhow::Result;
use std::collections::HashMap;
use crate::{api, Cli, EnvFile, EnvVar};
use crate::command::util;
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

#[derive(Parser, Debug)]
pub struct PutEnv {
    service: String,
    env_files: Vec<String>,
}

impl PutEnv {
    pub fn run(&self, args: &Cli) -> Result<()> {
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
#[derive(Parser, Debug)]
pub struct GetEnvGroup {
    /// The group to get, can be a name or an id
    group: String,
}

impl GetEnvGroup {
    pub fn run(&self, args: &Cli) -> Result<()> {
        let runtime = util::runtime();
        let client = render_api::RenderClient::new("https://api.render.com/v1", render_api::Authentication::Token(args.token.clone()));
        let env_group = runtime.block_on(client.get_env_group(&self.group).send())?;
        println!("{:#?}", env_group);
        Ok(())
    }
}

#[derive(Parser, Debug)]
pub struct ListEnvGroups {
}

impl ListEnvGroups {
    pub fn run(&self, args: &Cli) -> Result<()> {
        let runtime = util::runtime();
        let client = render_api::RenderClient::new("https://api.render.com/v1", render_api::Authentication::Token(args.token.clone()));
        let env_groups = runtime.block_on(client.list_env_groups().send())?;
        println!("{:#?}", env_groups);
        Ok(())
    }
}

#[derive(Parser, Debug)]
pub struct CreateEnvGroup {
    /// The name of the environment group
    name: String,
    /// The environment variables to set
    #[clap()]
    env_vars: Vec<String>,
}


impl CreateEnvGroup {
    pub fn run(&self, args: &Cli) -> Result<()> {
        let runtime = util::runtime();
        let client = render_api::RenderClient::new("https://api.render.com/v1", render_api::Authentication::Token(args.token.clone()));
        let env_group = runtime.block_on(client.create_env_var_group(&self.name, &self.env_vars).send())?;
        println!("{:#?}", env_group);
        Ok(())
    }
}