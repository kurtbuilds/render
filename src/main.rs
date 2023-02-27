use anyhow::Result;
use clap::{Parser, Subcommand};
use futures::{stream, StreamExt};
use render_api::RenderClient;

use command::*;

mod command;


#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    command: Command,

    #[clap(short, long)]
    verbose: bool,

    #[clap(long, global = true, env = "RENDER_TOKEN")]
    token: Option<String>,

    /// Specify the owner. Can be an id (e.g. usr_<id> or tea_<id>)
    /// or a lowercase prefix of a team (e.g. `blazing` would match the team `Blazing Fast Startup`)
    #[clap(long, global = true, env = "RENDER_OWNER")]
    owner: Option<String>,
}

impl Cli {
    pub async fn resolve_owner_id(&self, client: &RenderClient) -> Option<String> {
        if let Some(owner) = &self.owner {
            if owner.starts_with("usr-") || owner.starts_with("tea-") {
                return Some(owner.clone());
            }
            let owners_and_teams = client.list_authorized_users_and_teams().await.unwrap();
            let owner = owner.to_lowercase();
            let owner = owners_and_teams.into_iter().find(|o| o.owner.name.to_lowercase().starts_with(&owner)).unwrap();
            Some(owner.owner.id)
        } else {
            None
        }
    }

    pub fn build_client(&self) -> RenderClient {
        RenderClient::new("https://api.render.com/v1", render_api::RenderAuthentication::ApiKeyAuth{
            api_key_auth: self.token.as_ref().expect("--token or RENDER_TOKEN must be set.").clone(),
        })
            .with_middleware(httpclient::middleware::LoggerMiddleware::new())
    }
}

#[derive(Subcommand, Debug)]
enum Command {
    /// List services
    #[clap(alias = "ls")]
    List(List),
    /// Set environment variables for a service or group
    PutEnv(PutEnv),
    /// Deploy a service
    Deploy(Deploy),
    /// Suspend a service
    Suspend(Suspend),
    /// List env groups
    ListEnvGroups(ListEnvGroups),
    /// Get variables for an env group
    GetEnv(GetEnv),
    /// Create an env group
    CreateEnvGroup(CreateEnvGroup),
    /// List user and teams
    Teams(GetTeams),
}


fn main() -> Result<()> {
    let args = Cli::parse();
    match &args.command {
        Command::List(l) => l.run(&args),
        Command::PutEnv(p) => p.run(&args),
        Command::Deploy(d) => d.run(&args),
        Command::Suspend(s) => s.run(&args),
        Command::ListEnvGroups(l) => l.run(&args),
        Command::GetEnv(g) => g.run(&args),
        Command::CreateEnvGroup(c) => c.run(&args),
        Command::Teams(t) => t.run(&args),
    }
}