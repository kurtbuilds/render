
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    pub auto_deploy: String,
    #[serde(default)]
    pub branch: Option<String>,
    #[serde(default)]
    pub image_path: Option<String>,
    pub created_at: String,
    pub id: String,
    pub name: String,
    pub notify_on_fail: String,
    pub owner_id: String,
    #[serde(default)]
    pub repo: Option<String>,
    pub slug: String,
    pub suspended: String,
    pub suspenders: Vec<String>,
    #[serde(rename = "type")]
    pub type_: String,
    pub updated_at: String,
}
impl std::fmt::Display for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}