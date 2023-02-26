use std::{env, thread};
use std::borrow::Cow;
use std::collections::HashMap;

use anyhow::anyhow;
use anyhow::Result;
use clap::{Args, Parser, Subcommand, ValueEnum};
use colored::Colorize;
use futures::{stream, StreamExt};
use futures::stream::FuturesUnordered;
use slice_group_by::GroupBy;

use command::*;

use crate::envfile::EnvFile;

mod envfile;
mod api;
mod command;


#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    #[clap(short, long)]
    verbose: bool,

    #[clap(long, global = true, env = "RENDER_TOKEN")]
    token: String,

    /// Specify the owner. Can be an id (e.g. usr_<id> or tea_<id>)
    /// or a lowercase prefix of a team (e.g. `blazing` would match the team `Blazing Fast Startup`)
    #[clap(long, global = true, env = "RENDER_OWNER")]
    owner: Option<String>,
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
    GetEnvGroup(GetEnvGroup),
    /// Create an env group
    CreateEnvGroup(CreateEnvGroup),
}


fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Command::List(l) => l.run(&args),
        Command::PutEnv(p) => p.run(&args),
        Command::Deploy(d) => d.run(&args),
        Command::Suspend(s) => s.run(&args),
        Command::ListEnvGroups(l) => l.run(&args),
        Command::GetEnvGroup(g) => g.run(&args),
        Command::CreateEnvGroup(c) => c.run(&args),
    }
}