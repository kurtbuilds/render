use crate::Cli;
use crate::command::{resolve_services, util};

#[derive(clap::Parser, Debug)]
pub struct Suspend {
    services: Vec<String>,
}

impl Suspend {
    pub fn run(&self, cli: &Cli) -> anyhow::Result<()> {
        let runtime = util::runtime();
        let client = cli.build_client();
        let services = runtime.block_on(resolve_services(&client, &self.services))?;

        for service in services {
            runtime.block_on(client.suspend_service(&service.id).send()).unwrap();
            println!("Suspended service: {}", service.name);
        }
        Ok(())
    }
}