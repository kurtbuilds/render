use anyhow::{Result};
use env2::EnvFile;
use tokio::runtime::Runtime;
use tabular2::{Table, Row};
use crate::{Cli};
use crate::command::resolve_services;
use crate::command::util::runtime;
use render_api::RenderClient;

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

async fn resolve_env_group_id(id_or_name: &str, runtime: &Runtime, client: &RenderClient) -> String {
    if id_or_name.starts_with("evg-") {
        id_or_name.to_string()
    } else {
        let groups = runtime.block_on(client.list_env_groups().send()).unwrap();
        let env_group = groups
            .into_iter()
            .find(|g| g.env_group.name.to_lowercase().starts_with(&id_or_name.to_lowercase()))
            .expect("No environment group found with that name.");
        env_group.env_group.id
    }
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
            let id = runtime.block_on(resolve_env_group_id(&self.id_or_name, &runtime, &client));
            let env_group = runtime.block_on(client.retrieve_env_group(&id).send())?;
            for env_var in env_group.env_vars {
                println!("{}={}", env_var.key, env_var.value);
            }
        } else {
            let id = if self.id_or_name.starts_with("svc-") ||
                self.id_or_name.starts_with("crn") ||
                self.id_or_name.starts_with("wrk")
            {
                self.id_or_name.clone()
            } else {
                let services = runtime.block_on(resolve_services(&client, &vec![self.id_or_name.clone()]))?;
                services[0].id.clone()
            };
            let env_vars = runtime.block_on(client.retrieve_environment_variables(&id).send())?;
            for env_var in env_vars {
                println!("{}={}", env_var["key"], env_var["value"]);
            }
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
        // let owner: String = runtime.block_on(cli.resolve_owner_id(&client)).unwrap_or("".to_string());
        let env_groups = runtime.block_on(client.list_env_groups().send())?;
        let mut table = Table::new()
            .header("NAME")
            .header("ID")
            .header("OWNER_ID")
            .end_header();
        for env_group in env_groups {
            let env_group = env_group.env_group;
            table = table.row(Row::new()
                .cell(&env_group.name)
                .cell(&env_group.id)
                .cell(&env_group.owner_id)
            );
        }
        print!("{}", table);
        Ok(())
    }
}

// #[derive(clap::Parser, Debug)]
// pub struct CreateEnvGroup {
//     /// The name of the environment group
//     name: String,
//
//     /// The environment variables to set. If it contains a =, it will be parsed as a key=value pair.
//     /// Otherwise, it will be treated as a file path.
//     env_vars_or_files: Vec<String>,
// }


// impl CreateEnvGroup {
//     pub fn run(&self, cli: &Cli) -> Result<()> {
//         let runtime = runtime();
//         let client = cli.build_client();
//
//         let owner = runtime.block_on(cli.resolve_owner_id(&client)).unwrap_or("".to_string());
//
//         let vars = build_vars(&self.env_vars_or_files)
//             .into_iter()
//             .map(|v| GqlEnvVar::from((v.key.as_str(), v.value.as_str())))
//             .collect::<Vec<_>>();
//         let env_group = runtime.block_on(client.create_env_var_group(&owner, &self.name, &vars))?;
//         println!("{:#?}", env_group);
//         Ok(())
//     }
// }