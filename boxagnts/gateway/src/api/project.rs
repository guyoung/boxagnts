use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{ Serialize, Deserialize};
use anyhow::Result;


/// Represents a project in the projects directory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// The project ID (derived from the directory name)
    pub id: String,
    /// The original project path (decoded from the directory name)
    pub path: String,
    /// List of session IDs (JSONL file names without extension)
    pub sessions: Vec<String>,
    /// Unix timestamp when the project directory was created
    pub created_at: u64,
    /// Unix timestamp of the most recent session (if any)
    pub most_recent_session: Option<u64>,
}


pub async fn get_current_project() -> Result<Project> {

    let cwd = std::env::current_dir().map_err(|e| anyhow::anyhow!("Unable to obtain the current directory.: {}", e))?;

    let project_id = cwd.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(".");
    let project_path = ".";

    // Get creation time
    let metadata = fs::metadata(&cwd)
        .map_err(|e| anyhow::anyhow!("Failed to read directory metadata: {}", e))?;

    let created_at = metadata
        .created()
        .or_else(|_| metadata.modified())
        .unwrap_or(SystemTime::UNIX_EPOCH)
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();


    // Return the created project
    Ok(Project {
        id: project_id.to_string(),
        path: project_path.to_string(),
        sessions: Vec::new(),
        created_at,
        most_recent_session: None,
    })
}
