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
pub struct ServiceCursor {
    pub service: Service,
    cursor: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct EnvVar {
    pub key: String,
    pub value: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct EnvVarCursor {
    #[serde(rename = "envVar")]
    pub env_var: EnvVar,
    cursor: String,
}


pub fn list_services(token: &str) -> Result<Vec<Service>, Error> {
    let url = "https://api.render.com/v1/services";
    reqwest::blocking::Client::new()
        .get(url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .bearer_auth(token)
        .send()
        .map(|res| {
            res.json::<Vec<ServiceCursor>>()
                .unwrap()
                .into_iter()
                .map(|wrapper| wrapper.service)
                .collect::<_>()
        })
}


pub fn update_env_vars(token: &str, service_id: &str, pairs: &Vec<EnvVar>) -> Result<Vec<EnvVar>, Error> {
    let url = format!("https://api.render.com/v1/services/{}/env-vars", service_id);
    reqwest::blocking::Client::new()
        .put(url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&pairs).unwrap())
        .bearer_auth(token)
        .send()
        .map(|res| {
            res.json::<Vec<EnvVarCursor>>()
                .unwrap()
                .into_iter()
                .map(|cursor| cursor.env_var)
                .collect::<_>()
        })
}
