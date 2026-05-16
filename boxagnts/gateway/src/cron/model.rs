use std::fmt::{Display, Formatter};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


#[derive(Debug)]
pub enum CronError {
    NotFound(String),
    BadRequest(String),
    Conflict(String),
    Internal(String),
}

impl Display for CronError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CronError::NotFound(msg) => write!(f, "{}", msg),
            CronError::BadRequest(msg) => write!(f, "{}", msg),
            CronError::Conflict(msg) => write!(f, "{}", msg),
            CronError::Internal(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for CronError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub cron: String,
    pub enabled: bool,
    #[serde(default)]
    pub timeout: Option<u64>,
    pub prompt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobLog {
    pub id: String,
    pub job_id: String,
    pub job_name: String,
    pub executed_at: DateTime<Utc>,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateJobReq {
    #[serde(default = "default_id")]
    pub id: String,
    pub name: String,
    pub description: String,
    pub cron: String,
    pub enabled: bool,
    #[serde(default)]
    pub timeout: Option<u64>,
    pub prompt: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateJobReq {
    pub name: Option<String>,
    pub description: Option<String>,
    pub cron: Option<String>,
    pub enabled: Option<bool>,
    pub timeout: Option<u64>,
    pub prompt: Option<String>,
}


#[derive(Default, Debug, Serialize)]
pub struct ListJobRes {
    pub id: String,
    pub name: String,
    pub description: String,
    pub cron: String,
    pub enabled: bool,
    pub last_run_at: Option<DateTime<Utc>>,
    pub last_run_success: Option<bool>,

}

fn default_id() -> String {
    uuid::Uuid::new_v4().to_string()
}