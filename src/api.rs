use std::any;
use std::collections::HashMap;
use std::fmt::Display;
use serde::{Serialize, Deserialize};
use anyhow::Result;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub enum ServiceType {
    #[serde(alias = "background_worker")]
    Worker,
    #[serde(alias = "web_service")]
    Web,
    #[serde(alias = "static_site")]
    Static,
    Unrecognized(String),
}

impl Display for ServiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceType::Worker => write!(f, "worker"),
            ServiceType::Web => write!(f, "web"),
            ServiceType::Static => write!(f, "static"),
            ServiceType::Unrecognized(s) => write!(f, "{}", s),
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    pub id: String,
    #[serde(rename = "autoDeploy")]
    pub auto_deploy: String,
    pub branch: String,
    #[serde(rename = "type")]
    pub type_: ServiceType,
    pub name: String,
    pub slug: String,
}

impl Service {
    pub fn url(&self) -> String {
        format!("https://dashboard.render.com/{}/{}", self.type_, self.id)
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceCursor {
    pub service: Service,
    pub cursor: String,
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
    pub cursor: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployCursor {
    pub deploy: Deploy,
    pub cursor: String,
}

pub async fn list_services(token: &str) -> Result<Vec<Service>> {
    let url = "https://api.render.com/v1/services";
    let res = httpclient::Client::new(None)
        .get(url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .bearer_auth(token)
        .send()
        .await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json::<Vec<ServiceCursor>>().await?
            .into_iter()
            .map(|cur| cur.service)
            .collect::<_>()),
        Err(_) => Err(anyhow::anyhow!("{}", res.text().await.unwrap()))
    }
}


pub async fn list_deploys(token: &str, service_id: &str, limit: usize) -> Result<Vec<Deploy>> {
    if !(1 <= limit && limit <= 100) {
        return Err(anyhow::anyhow!("limit must be between 1 and 100"));
    }
    // println!("listing deploys for service {}", service_id);
    let url = format!("https://api.render.com/v1/services/{}/deploys?limit={}", service_id, limit);
    let client = httpclient::Client::new(None);
    let res = client.get(&url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .bearer_auth(token)
        ;
    let res = res.send()
        .await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json::<Vec<DeployCursor>>().await?
            .into_iter()
            .map(|cur| cur.deploy)
            .collect::<_>()),
        Err(_) => Err(anyhow::anyhow!("{}", res.text().await.unwrap()))
    }
}


pub async fn update_env_vars(token: &str, service_id: &str, pairs: &Vec<EnvVar>) -> Result<Vec<EnvVar>> {
    let url = format!("https://api.render.com/v1/services/{}/env-vars", service_id);
    let res = httpclient::Client::new(None)
        .request(http::Method::PUT, &url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&pairs)
        .bearer_auth(token)
        .send().await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json::<Vec<EnvVarCursor>>().await?
            .into_iter()
            .map(|wrapper| wrapper.env_var)
            .collect::<_>()),
        Err(_) => Err(anyhow::anyhow!("{}", res.text().await.unwrap()))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub id: String,
    pub message: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Deploy {
    pub id: String,
    pub commit: Commit,
    pub status: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    #[serde(rename = "finishedAt")]
    pub finished_at: Option<String>,
}


pub async fn trigger_deploy(token: &str, service_id: &str) -> Result<Deploy> {
    let url = format!("https://api.render.com/v1/services/{}/deploys", service_id);
    let body_params = HashMap::from([
        ("clearCache", "do_not_clear"),
    ]);
    let res = httpclient::Client::new(None)
        .post(&url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .bearer_auth(token)
        .json(&body_params)
        .send().await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json::<Deploy>().await?),
        Err(_) => Err(anyhow::anyhow!("{}: {}", res.status(), res.text().await.unwrap())),
    }
}

pub async fn suspend(token: &str, service_id: &str) -> Result<()> {
    let url = format!("https://api.render.com/v1/services/{}/suspend", service_id);
    let res = httpclient::Client::new(None)
        .post(&url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .bearer_auth(token)
        .send().await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(()),
        Err(_) => Err(anyhow::anyhow!("{}: {}", res.status(), res.text().await.unwrap())),
    }
}