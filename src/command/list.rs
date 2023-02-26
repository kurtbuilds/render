use tabular::Row;
use std::borrow::Cow;
use chrono::Utc;
use colored::Colorize;
use crate::{api, Cli, stream, StreamExt};
use crate::command::util;
use relativetime::RelativeTime;
use clap_derive::parser as Parser;
use crate::api::{Deploy, Service};

pub fn service_status<'a>(service: &'a Service, deploy: &'a Deploy) -> Cow<'a, str> {
    if matches!(service.suspended, api::Suspended::Suspended) {
        return "SUSPENDED".dimmed().to_string().into();
    }
    match deploy.status.as_ref() {
        "live" => Cow::Owned("LIVE".green().to_string()),
        "build_failed" => Cow::Owned("BUILD FAILED".red().to_string()),
        "update_failed" => Cow::Owned("UPDATE FAILED".red().to_string()),
        "update_in_progress" => Cow::Owned("UPDATING".yellow().to_string()),
        "build_in_progress" => Cow::Owned("BUILDING".yellow().to_string()),
        s => Cow::Borrowed(s),
    }
}

pub fn list_services(token: &str) -> anyhow::Result<()> {

    for rows in groups {
        for (service, deploys) in rows {
            let deploy = deploys.as_ref().unwrap().get(0).unwrap();
            table.add_row(Row::new()
                .with_cell(service.name.clone())
                .with_cell(service_status(service, deploy))
                .with_cell(deploy.updated_at.to_relative())
                .with_cell(service.id.clone())
                .with_cell(service.url())
            );
        }
        // table.add_heading("");
    }
    print!("{}", table);
    Ok(())
}

#[derive(Parser, Debug)]
pub struct List {
}

impl List {
    pub fn run(&self, args: &Cli) -> anyhow::Result<()> {
        let runtime = util::runtime();
        let client = render_api::RenderClient::new("https://api.render.com/v1", render_api::Authentication::Token(args.token.clone()));
        let services = runtime.block_on(client.list_services().send())?;
        let service_deploys = stream::iter(services)
            .map(|service| async {
                let deploys = client.list_deploys(&service.service.id).limit(1).await;
                (service, deploys)
            })
            .buffer_unordered(16)
            .collect::<Vec<_>>();
        let service_deploys = runtime.block_on(service_deploys);
        let mut table = tabular2::Table::new()
            .header("SERVICE")
            .header("STATUS")
            .header("UPDATED")
            .header("SERVICE ID")
            .header("URL");
        Ok(())
    }
}
