
use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;
use uuid::Uuid;

use super::model::SiteConfig;

#[derive(Clone)]
pub struct AppState {
    pub sites: Arc<RwLock<HashMap<String, SiteConfig>>>,
    
    pub site_map: Arc<RwLock<HashMap<String, Uuid>>>, // job_id -> scheduler uuid
}
