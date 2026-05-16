use std::path::PathBuf;
use std::fs;
use std::time::SystemTime;

use serde::{ Serialize, Deserialize};
use anyhow::{anyhow, Result};

use boxagnts_core::types::Role;

/// Represents a session with its metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// The session ID (UUID)
    pub id: String,
    /// The session title
    pub title: String,
    /// Unix timestamp when the session file was created
    pub created_at: u64,
    /// First user message content (if available)
    pub first_message: Option<String>,
    /// UUID of the first user message (if available)
    pub message_uuid: Option<String>,
}




/// Gets sessions
pub async fn get_sessions() -> Result<Vec<Session>> {
    let saved_dir = boxagnts_workspace::path::get_saved_dir().await;

    let sessions_dir = saved_dir.join("sessions");

    let mut sessions = Vec::new();

    // Read all JSON files in the project directory
    let entries = fs::read_dir(&sessions_dir)
        .map_err(|e| anyhow!("Failed to read project directory: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| anyhow!("Failed to read directory entry: {}", e))?;
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
            if let Some(session_id) = path.file_stem().and_then(|s| s.to_str()) {
                // Get file creation time
                let metadata = fs::metadata(&path)
                    .map_err(|e| anyhow!("Failed to read file metadata: {}", e))?;

                let created_at = metadata
                    .created()
                    .or_else(|_| metadata.modified())
                    .unwrap_or(SystemTime::UNIX_EPOCH)
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();

                // Extract first user message and uuid
                let (title, first_message, message_uuid) = parse_session(&path)?;

                sessions.push(Session {
                    id: session_id.to_string(),
                    title: title.unwrap_or("New Session".to_string()),
                    created_at,
                    first_message,
                    message_uuid,
                });
            }
        }
    }

    // Sort sessions by creation time (newest first)
    sessions.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    log::info!(
        "Found {} sessions ",
        sessions.len()
    );
    Ok(sessions)
}


pub async fn load_session_history(
    session_id: String,
) -> Result<serde_json::Value> {
    let saved_dir = boxagnts_workspace::path::get_saved_dir().await;

    let session_path = saved_dir
        .join("sessions")
        .join(format!("{}.json", session_id));

    if !session_path.exists() {
        return Err(anyhow!("Session file not found: {}", session_id));
    }

    let json_str = fs::read_to_string(&session_path).map_err(|e| anyhow!("Failed to open session file: {}", e))?;

    let session: boxagnts_workspace::history::ConversationSession = serde_json::from_str(&json_str)?;

    let val = serde_json::to_value(&session)?;

    Ok(val)
}

pub async fn delete_session(
    session_id: String,
) -> Result<()> {
    let saved_dir = boxagnts_workspace::path::get_saved_dir().await;

    let session_path = saved_dir
        .join("sessions")
        .join(format!("{}.json", session_id));

    fs::remove_file(&session_path).map_err(|e| anyhow!("Failed to delete session file: {}", e))?;

    Ok(())
}

pub async fn update_session_title(
    session_id: String,
    title: String,
) -> Result<()> {
    let saved_dir = boxagnts_workspace::path::get_saved_dir().await;

    let session_path = saved_dir
        .join("sessions")
        .join(format!("{}.json", session_id));

    let json_str = fs::read_to_string(&session_path).map_err(|e| anyhow!("Failed to open session file: {}", e))?;

    let mut session: boxagnts_workspace::history::ConversationSession = serde_json::from_str(&json_str)?;

    session.title = Some(title);

    let val = serde_json::to_value(&session)?;

    fs::write(&session_path, val.to_string()).map_err(|e| anyhow!("Failed to update session file: {}", e))?;

    Ok(())
}

pub async fn delete_session_messages(
    session_id: String,
    message_ids: Vec<String>,
) -> Result<()> {
    let saved_dir = boxagnts_workspace::path::get_saved_dir().await;

    let session_path = saved_dir
        .join("sessions")
        .join(format!("{}.json", session_id));

    let json_str = fs::read_to_string(&session_path).map_err(|e| anyhow!("Failed to open session file: {}", e))?;

    let mut session: boxagnts_workspace::history::ConversationSession = serde_json::from_str(&json_str)?;

    let mut messages = session.messages.clone();

    let mut i = 0;
    while i < messages.len() {
        let uuid = messages[i].uuid.clone();

        if let Some(uuid) = uuid {
            if message_ids.contains(&uuid.to_string()) {
                messages.remove(i);
            }
            else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    session.messages = messages;

    let val = serde_json::to_value(&session)?;

    fs::write(&session_path, val.to_string()).map_err(|e| anyhow!("Failed to update session file: {}", e))?;

    Ok(())
}

pub async fn clear_session_messages(
    session_id: String,
) -> Result<()> {
    let saved_dir = boxagnts_workspace::path::get_saved_dir().await;

    let session_path = saved_dir
        .join("sessions")
        .join(format!("{}.json", session_id));

    let json_str = fs::read_to_string(&session_path).map_err(|e| anyhow!("Failed to open session file: {}", e))?;

    let mut session: boxagnts_workspace::history::ConversationSession = serde_json::from_str(&json_str)?;

    session.messages = vec![];

    let val = serde_json::to_value(&session)?;

    fs::write(&session_path, val.to_string()).map_err(|e| anyhow!("Failed to update session file: {}", e))?;

    Ok(())
}


fn parse_session(session_path: &PathBuf) -> Result<(Option<String>, Option<String>, Option<String>)> {
    let json_str = fs::read_to_string(&session_path).map_err(|e| anyhow!("Failed to open session file: {}", e))?;

    let session: boxagnts_workspace::history::ConversationSession = serde_json::from_str(&json_str)?;

    let msg = session.messages
        .iter()
        .find(|msg| msg.role == Role::User);

    let title = {
        if let Some(title) = session.title {
            Some(title)
        } else {
            if let Some(msg) = msg {
                if let Some(text) = msg.get_text() {
                    let text = truncate_with_ellipsis(text, 18);
                    Some(text)
                } else {
                    None
                }
            } else {
                None
            }
        }
    };

    if let Some(msg) = msg {
        Ok((title, msg.get_text().map(|s| s.to_string()), msg.uuid.clone()))
    } else {
        Ok((title, None, None))
    }
}



fn truncate_with_ellipsis(s: &str, max_chars: usize) -> String {
    let char_count = s.chars().count();

    if char_count <= max_chars {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max_chars).collect();
        format!("{}...", truncated)
    }
}