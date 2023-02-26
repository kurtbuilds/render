use anyhow::Result;
use crate::{api, Cli};
use crate::command::util;

#[derive(Parser, Debug)]
pub struct Deploy {
    service: String,
}

impl Deploy {
    pub fn run(&self, args: &Cli) -> Result<()> {
        let runtime = util::runtime();
        let client = render_api::RenderClient::new("https://api.render.com/v1", render_api::Authentication::Token(args.token.clone()));
        let services = runtime.block_on(client.list_services().send())?;
        let service = services.iter().find(|s| s.service.name == self.service).expect("No service matching that name found.");
        let deploy = runtime.block_on(client.trigger_deploy(&service.service.id).send())?;
        println!("Watch deploy at https://dashboard.render.com/{}/{}/deploys/{}", service.service.type_, service.service.id, deploy.id);
        Ok(())
    }
}