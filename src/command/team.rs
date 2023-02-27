use tabular2::{Row, Table};
use crate::Cli;
use crate::command::{util};

#[derive(clap::Parser, Debug)]
pub struct GetTeams {
    services: Vec<String>,
}

impl GetTeams {
    pub fn run(&self, cli: &Cli) -> anyhow::Result<()> {
        let runtime = util::runtime();
        let client = cli.build_client();
        let owners = runtime.block_on(client.list_authorized_users_and_teams().send())?;

        let mut table = Table::new()
            .header("Name")
            .header("Type")
            .header("ID")
            .end_header();
        for owner in owners {
            let owner = owner.owner;
            table = table.row(Row::new()
                .cell(&owner.name)
                .cell(&owner.type_)
                .cell(&owner.id)
            );
        }
        println!("{}", table);
        eprintln!("Set the owner with `export RENDER_OWNER=<id>`");
        Ok(())
    }
}