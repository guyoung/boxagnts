
use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;
use uuid::Uuid;

use super::model::JobConfig;

#[derive(Clone)]
pub struct AppState {
    pub jobs: Arc<RwLock<HashMap<String, JobConfig>>>,    
    pub job_map: Arc<RwLock<HashMap<String, Uuid>>>, // job_id -> scheduler uuid

}
