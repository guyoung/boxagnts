use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};


#[derive(Debug)]
pub enum SiteError {
    NotFound(String),
    BadRequest(String),
    Conflict(String),
    Internal(String),
}

impl Display for SiteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SiteError::NotFound(msg) => write!(f, "{}", msg),
            SiteError::BadRequest(msg) => write!(f, "{}", msg),
            SiteError::Conflict(msg) => write!(f, "{}", msg),
            SiteError::Internal(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for SiteError {}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SiteConfig {
    pub id: String,
    pub name: String,
    pub title: String,
    pub description: String,
    pub path: String,
    pub component: String,
    #[serde(default)]
    pub entry_point: Option<String>,
    #[serde(default)]
    pub enable_auth: Option<bool>,
    #[serde(default)]
    pub auth_user: Option<String>,
    #[serde(default)]
    pub auth_pass: Option<String>,
    pub enabled: bool,

}

#[derive(Default, Debug, Deserialize)]
pub struct CreateSiteReq {
    #[serde(default = "default_id")]
    pub id: String,
    pub name: String,
    pub title: String,
    pub description: String,
    pub path: String,
    pub component: String,
    #[serde(default)]
    pub entry_point: Option<String>,
    #[serde(default)]
    pub enable_auth: Option<bool>,
    #[serde(default)]
    pub auth_user: Option<String>,
    #[serde(default)]
    pub auth_pass: Option<String>,
    pub enabled: bool,
}

#[derive(Default, Debug, Deserialize)]
pub struct UpdateSiteReq {
    pub name: Option<String>,
    pub description: Option<String>,
    pub title: Option<String>,
    pub path: Option<String>,
    pub component: Option<String>,
    #[serde(default)]
    pub entry_point: Option<String>,
    #[serde(default)]
    pub enable_auth: Option<bool>,
    #[serde(default)]
    pub auth_user: Option<String>,
    #[serde(default)]
    pub auth_pass: Option<String>,
    pub enabled: Option<bool>,
}

fn default_id() -> String {
    uuid::Uuid::new_v4().to_string()
}