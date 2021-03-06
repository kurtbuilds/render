use tabular::Row;
use std::borrow::Cow;
use chrono::Utc;
use slice_group_by::GroupBy;
use colored::Colorize;
use crate::{api, stream, StreamExt};
use crate::command::util;
use relativetime::RelativeTime;


pub fn list_services(token: &str) -> anyhow::Result<()> {
    let runtime = util::runtime();

    let services = runtime.block_on(api::list_services(token))?;

    let client = httpclient::Client::new(None);
    let fetches = stream::iter(services)
        .map(|service| async {
            let deploys = api::list_deploys(token, &service.id, 1).await;
            (service, deploys)
        })
        .buffer_unordered(16).collect::<Vec<_>>();

    let mut rows = runtime.block_on(fetches);

    let mut table = tabular::Table::new("{:<}  {:<}  {:<}  {:<}  {:<}");
    rows.sort_by(|a, b| a.0.name.cmp(&b.0.name));
    let groups = rows.linear_group_by_key(|(service, deploy)| service.name.splitn(2, '.').next().unwrap().to_string())
        .collect::<Vec<_>>();

    table.add_row(Row::new()
        .with_cell("SERVICE")
        .with_cell("STATUS")
        .with_cell("UPDATED")
        .with_cell("SERVICE ID")
        .with_cell("URL")
    );

    for rows in groups {
        for (service, deploy) in rows {
            let deploy = deploy.as_ref().unwrap().get(0).unwrap();
            table.add_row(Row::new()
                .with_cell(service.name.clone())
                .with_cell(match deploy.status.as_ref() {
                    "live" => Cow::Owned("LIVE".green().to_string()),
                    "build_failed" => Cow::Owned("BUILD FAILED".red().to_string()),
                    "update_failed" => Cow::Owned("UPDATE FAILED".red().to_string()),
                    "update_in_progress" => Cow::Owned("UPDATING".yellow().to_string()),
                    "build_in_progress" => Cow::Owned("BUILDING".yellow().to_string()),
                    s => Cow::Borrowed(s),
                })
                .with_cell(deploy.updated_at.to_relative())
                .with_cell(service.id.clone())
                .with_cell(service.url())
            );
        }
        table.add_heading("");
    }
    print!("{}", table);
    Ok(())
}
