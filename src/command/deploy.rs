use clap::ArgMatches;
use crate::api;
use crate::command::util;

pub fn deploy(token: &str, args: &ArgMatches) -> anyhow::Result<()> {
    let runtime = util::runtime();

    let services = runtime.block_on(api::list_services(token))?;
    let service = args.value_of("service").unwrap();
    let service = services.iter().find(|s| s.name == service).unwrap();

    runtime.block_on(async {
        api::trigger_deploy(token, &service.id)
            .await
            .map(|deploy| {
                println!("Watch deploy at https://dashboard.render.com/{}/{}/deploys/{}", service.type_, service.id, deploy.id)
            })
    })
}
