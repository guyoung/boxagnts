use std::sync::Arc;
use tokio::sync::RwLock;

pub type AppState = Arc<RwLock<boxagnts_workspace::config::Settings>>;