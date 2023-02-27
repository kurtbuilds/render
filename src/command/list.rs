use std::borrow::Cow;
use colored::Colorize;
use render_api::model::{Deploy, Service};
use tabular2::Row;
use crate::{Cli, stream, StreamExt};
use relativetime::RelativeTime;
use crate::command::util::runtime;

pub fn service_status<'a>(service: &'a Service, deploy: &'a Deploy) -> Cow<'a, str> {
    if service.suspended == "suspended" {
        return "SUSPENDED".dimmed().to_string().into();
    }
    match deploy.status.as_ref() {
        "live" => Cow::Owned("LIVE".green().to_string()),
        "build_failed" => Cow::Owned("BUILD FAILED".red().to_string()),
        "update_failed" => Cow::Owned("UPDATE FAILED".red().to_string()),
        "update_in_progress" => Cow::Owned("UPDATING".yellow().to_string()),
        "build_in_progress" => Cow::Owned("BUILDING".yellow().to_string()),
        "deactivated" => Cow::Owned("DEACTIVATED".dimmed().to_string()),
        "canceled" => Cow::Owned("CANCELED".dimmed().to_string()),
        s => Cow::Borrowed(s),
    }
}

pub fn url(service: &Service) -> String {
    let code = match service.type_.as_str() {
        "static_site" => "static",
        "web_service" => "web",
        "background_worker" => "worker",
        "cron_job" => "cron",
        z => z,
    };
    format!("https://dashboard.render.com/{}/{}", code, service.id)
}

#[derive(clap::Parser, Debug)]
pub struct List {
}

impl List {
    pub fn run(&self, cli: &Cli) -> anyhow::Result<()> {
        let runtime = runtime();
        let client = cli.build_client();
        let mut list_services = client.list_services();
        let owner_id  = runtime.block_on(cli.resolve_owner_id(&client));
        if let Some(owner_id) = &owner_id {
            list_services = list_services.owner_id(owner_id);
        }
        let mut services = runtime.block_on(list_services.send())?;
        match owner_id.as_ref() {
            Some(z) if z.starts_with("tea-") => {
                services = services.into_iter().filter(|s| s.service.owner_id == *z).collect::<Vec<_>>();
            }
            _ => {}
        }
        let service_deploys = stream::iter(services)
            .map(|service| async {
                let mut deploys = client.list_deploys(&service.service.id).limit(1).await.unwrap();
                let deploy = deploys.remove(0).deploy;
                (service.service, deploy)
            })
            .buffer_unordered(16)
            .collect::<Vec<_>>();
        let service_deploys = runtime.block_on(service_deploys);
        let mut table = tabular2::Table::new()
            .header("SERVICE")
            .header("STATUS")
            .header("UPDATED")
            .header("SERVICE ID")
            .header("URL")
            .end_header();
        for (service, deploy) in service_deploys.iter() {
            table = table.row(Row::new()
                .cell(&service.name)
                .cell(&service_status(service, deploy))
                .cell(&deploy.updated_at.to_relative())
                .cell(&service.id)
                .cell(&url(service))
            );
        }
        print!("{}", table);
        Ok(())
    }
}
