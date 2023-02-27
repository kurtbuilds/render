use anyhow::{Result};
use render_api::{GqlEnvVar};
use env2::EnvFile;
use crate::{Cli};
use crate::command::resolve_services;
use crate::command::util::runtime;

fn build_vars(env_pairs_or_files: &[String]) -> Vec<render_api::model::EnvVar> {
    let mut result = vec![];
    for item in env_pairs_or_files {
        if item.contains('=') {
            let pair = env2::Pair::from(item.as_str());
            result.push(render_api::model::EnvVar {
                key: pair.key,
                value: pair.value,
            });
        } else {
            let env = EnvFile::read(item);
            for (key, value) in &env {
                result.push(render_api::model::EnvVar {
                    key: key.to_string(),
                    value: value.to_string(),
                });
            }
        }
    }
    result.dedup_by(|a, b| a.key == b.key);
    result
}

#[derive(clap::Parser, Debug)]
pub struct PutEnv {
    /// By default, this is a service. If you want to put-env for a group, use the -g option.
    name_or_id: String,

    /// put-env for a group (rather than a service, the default)
    #[clap(short, long)]
    group: bool,

    /// For each value, if it contains a =, it will be parsed as a key=value pair. Otherwise, it will be treated as a file path.
    env_pair_or_files: Vec<String>,
}

impl PutEnv {
    pub fn run(&self, cli: &Cli) -> Result<()> {
        let runtime = runtime();
        let client = cli.build_client();
        let vars = build_vars(&self.env_pair_or_files);

        let service = runtime.block_on(resolve_services(&client, &vec![self.name_or_id.clone()]))?.remove(0);

        runtime.block_on(client.update_environment_variables(vars, &service.id).send()).unwrap();
        Ok(())
    }
}

#[derive(clap::Parser, Debug)]
pub struct GetEnv {
    /// The group to get, can be a name or an id
    id_or_name: String,

    #[clap(short, long)]
    group: bool
}

impl GetEnv {
    pub fn run(&self, cli: &Cli) -> Result<()> {
        let runtime = runtime();
        let client = cli.build_client();
        if self.group {
            let env_group = runtime.block_on(client.get_env_group(&self.id_or_name))?;
            println!("{:#?}", env_group);
        } else {

        }
        Ok(())
    }
}

#[derive(clap::Parser, Debug)]
pub struct ListEnvGroups {
}

impl ListEnvGroups {
    pub fn run(&self, cli: &Cli) -> Result<()> {
        let runtime = runtime();
        let client = cli.build_client();
        let owner: String = runtime.block_on(cli.resolve_owner_id(&client)).unwrap_or("".to_string());
        let env_groups = runtime.block_on(client.get_env_groups(&owner))?;
        println!("{:#?}", env_groups);
        Ok(())
    }
}

#[derive(clap::Parser, Debug)]
pub struct CreateEnvGroup {
    /// The name of the environment group
    name: String,

    /// The environment variables to set. If it contains a =, it will be parsed as a key=value pair.
    /// Otherwise, it will be treated as a file path.
    env_vars_or_files: Vec<String>,
}


impl CreateEnvGroup {
    pub fn run(&self, cli: &Cli) -> Result<()> {
        let runtime = runtime();
        let client = cli.build_client();

        let owner = runtime.block_on(cli.resolve_owner_id(&client)).unwrap_or("".to_string());

        let vars = build_vars(&self.env_vars_or_files)
            .into_iter()
            .map(|v| GqlEnvVar::from((v.key.as_str(), v.value.as_str())))
            .collect::<Vec<_>>();
        let env_group = runtime.block_on(client.create_env_var_group(&owner, &self.name, &vars))?;
        println!("{:#?}", env_group);
        Ok(())
    }
}