use std::collections::HashMap;
use reqwest::Error;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    pub id: String,
    #[serde(rename = "autoDeploy")]
    pub auto_deploy: String,
    pub branch: String,
    #[serde(rename = "type")]
    pub typ: String,
    pub name: String,
    pub slug: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceWrapper {
    pub service: Service,
    cursor: String,
}


pub fn list_services(token: &str) -> Vec<Service> {
    let url = "https://api.render.com/v1/services";
    reqwest::blocking::Client::new()
        .get(url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .bearer_auth(token)
        .send()
        .map(|mut res| {
            let text = res.text().unwrap();
            let data: Vec<ServiceWrapper> = serde_json::from_str(&text).unwrap();
            data.into_iter().map(|wrapper| wrapper.service).collect::<_>()
        })
        .unwrap()
}


pub fn update_env_vars(token: &str, service_id: &str, pairs: &Vec<HashMap<String, String>>) -> Result<reqwest::blocking::Response, Error> {
    let url = format!("https://api.render.com/v1/services/{}/env-vars", service_id);
    reqwest::blocking::Client::new()
        .put(url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&pairs).unwrap())
        .bearer_auth(token)
        .send()
}
