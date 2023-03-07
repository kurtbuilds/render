use anyhow::Result;
use crate::{Cli};
use crate::command::util;
use crate::ext::ServiceCursorExt;

#[derive(clap::Parser, Debug)]
pub struct Deploy {
    service: String,
}

impl Deploy {
    pub fn run(&self, cli: &Cli) -> Result<()> {
        let runtime = util::runtime();
        let client = cli.build_client();
        let services = runtime.block_on(client.list_services().send())?;
        let service = services.iter().find(|s| s.service.name == self.service).expect("No service matching that name found.");
        let deploy = runtime.block_on(client.trigger_deploy(&service.service.id).send())?;
        println!("Watch deploy at {}", service.service.deploy_url(&deploy.id));
        Ok(())
    }
}