use render_api::model::{Service};

pub trait ServiceCursorExt {
    fn short_type(&self) -> &str;
    fn service_url(&self) -> String;
    fn deploy_url(&self, deploy_id: &str) -> String;
}

impl ServiceCursorExt for Service {
    fn short_type(&self) -> &str {
        match self.type_.as_str() {
            "static_site" => "static",
            "web_service" => "web",
            "background_worker" => "worker",
            "cron_job" => "cron",
            z => z,
        }
    }

    fn service_url(&self) -> String {
        format!("https://dashboard.render.com/{}/{}", self.short_type(), self.id)
    }

    fn deploy_url(&self, deploy_id: &str) -> String {
        format!("{}/deploys/{}", self.service_url(), deploy_id)
    }
}