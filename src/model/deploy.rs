
use serde::{Serialize, Deserialize};
use super::Commit;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deploy {
    #[serde(default)]
    pub commit: Option<Commit>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
    pub id: String,
    pub status: String,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
impl std::fmt::Display for Deploy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}