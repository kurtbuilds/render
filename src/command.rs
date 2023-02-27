mod env;
mod deploy;
mod list;
mod util;
mod suspend;
mod team;

pub use deploy::*;
pub use list::*;
pub use env::*;
use render_api::RenderClient;
pub use suspend::*;
pub use team::*;

async fn resolve_services(client: &RenderClient, id_or_name: &[String]) -> anyhow::Result<Vec<render_api::model::Service>> {
    let services = client.list_services().await?;
    let services = services.into_iter().filter_map(|s| {
        if id_or_name.iter().any(|id| id == &s.service.id || id == &s.service.name) {
            Some(s.service)
        } else {
            None
        }
    }).collect();
    Ok(services)
}
