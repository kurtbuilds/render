use anyhow::Result;
use crate::{Cli};
use crate::command::util;
use crate::ext::ServiceCursorExt;

#[derive(clap::Parser, Debug)]
pub struct Deploy {
    service: String,

    #[clap(short='t', long)]
    image_tag: Option<String>,
}

impl Deploy {
    pub fn run(&self, cli: &Cli) -> Result<()> {
        let runtime = util::runtime();
        let client = cli.build_client();
        let services = runtime.block_on(client.list_services().send())?;
        let service = services.iter().find(|s| s.service.name == self.service).expect("No service matching that name found.");
        let mut deploy = client.trigger_deploy(&service.service.id);
        if let Some(t) = &self.image_tag {
            let t = if !t.contains(":") {
                let path = service.service.image_path.as_ref().expect("Service has no image path.");
                let path = path.split_once(':').expect("Service image path has no tag.").0;
                format!("{path}:{t}")
            } else {
                t.to_owned()
            };
            deploy.image_url = Some(t);
        }
        let deploy = runtime.block_on(deploy.send())?;
        println!("Watch deploy at {}", service.service.deploy_url(&deploy.id));
        Ok(())
    }
}