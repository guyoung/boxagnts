
use boxagnts_core::types::Message;
use serde::{Deserialize, Serialize};

/// A checkpoint snapshot of conversation messages at a specific point in time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionCheckpoint {
    /// The message index this checkpoint was taken at (exclusive upper bound).
    pub message_idx: usize,
    /// Optional human-readable label.
    pub label: Option<String>,
    /// When this checkpoint was created.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Snapshot of all messages up to (and including) `message_idx - 1`.
    pub snapshot: Vec<Message>,
}

/// A single persisted conversation session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationSession {
    pub id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub messages: Vec<Message>,
    pub model: String,
    pub title: Option<String>,
    //pub working_dir: Option<String>,
    /// Tags for filtering / searching sessions.
    #[serde(default)]
    pub tags: Vec<String>,
    /// ID of the session this was branched from, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_from: Option<String>,
    /// Message index in the parent session at which this branch was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_at_message: Option<usize>,
    /// Remote bridge URL if this session is mirrored to a remote endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_session_url: Option<String>,
    /// Accumulated USD cost for this session.
    #[serde(default)]
    pub total_cost: f64,
    /// Accumulated token count for this session.
    #[serde(default)]
    pub total_tokens: u64,
    /// Saved checkpoints (rewind points) within this session.
    #[serde(default)]
    pub checkpoints: Vec<SessionCheckpoint>,
    /// ID of the parent session this was forked from (via /fork).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_session_id: Option<String>,
    /// Message index in the parent session at which this fork was created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fork_point_message_index: Option<usize>,
}

impl ConversationSession {
    pub fn new(model: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: now,
            updated_at: now,
            messages: vec![],
            model,
            title: None,
            tags: vec![],
            branch_from: None,
            branch_at_message: None,
            remote_session_url: None,
            total_cost: 0.0,
            total_tokens: 0,
            checkpoints: vec![],
            parent_session_id: None,
            fork_point_message_index: None,
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
        self.updated_at = chrono::Utc::now();
    }

    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    pub fn last_user_message(&self) -> Option<&Message> {
        self.messages
            .iter()
            .rev()
            .find(|m| m.role == boxagnts_core::types::Role::User)
    }
}

// -------------------------------------------------------------------------
// Checkpoint helpers (synchronous, operate on a mutable session in-memory)
// -------------------------------------------------------------------------

/// Create a checkpoint at the current end of the session's message list.
/// The checkpoint captures all messages currently in the session.
pub fn create_checkpoint(session: &mut ConversationSession, label: Option<&str>) {
    let idx = session.messages.len();
    let checkpoint = SessionCheckpoint {
        message_idx: idx,
        label: label.map(|s| s.to_string()),
        created_at: chrono::Utc::now(),
        snapshot: session.messages.clone(),
    };
    session.checkpoints.push(checkpoint);
    session.updated_at = chrono::Utc::now();
}

/// Restore the session's messages to those saved in checkpoint `idx`.
///
/// Returns the messages that were replaced (i.e. the messages discarded by
/// the rewind).  The session's `messages` field is replaced with the
/// checkpoint snapshot; `updated_at` is refreshed.
///
/// # Panics
/// Panics if `idx` is out of bounds (i.e. >= `session.checkpoints.len()`).
pub fn restore_checkpoint(session: &mut ConversationSession, idx: usize) -> Vec<Message> {
    let snapshot = session.checkpoints[idx].snapshot.clone();
    let replaced = std::mem::replace(&mut session.messages, snapshot);
    session.updated_at = chrono::Utc::now();
    replaced
}

// -------------------------------------------------------------------------
// Persistent storage helpers
// -------------------------------------------------------------------------

/// The on-disk directory for conversation sessions.
async fn get_sessions_dir() -> std::path::PathBuf {
    crate::path::get_saved_dir().await.join("sessions")
}

/// Save a session to `<cwd>/sessions/<id>.json`.
pub async fn save_session(session: &ConversationSession) -> anyhow::Result<()> {
    let dir = get_sessions_dir().await;
    tokio::fs::create_dir_all(&dir).await?;
    let path = dir.join(format!("{}.json", session.id));
    let content = serde_json::to_string_pretty(session)?;
    tokio::fs::write(&path, content).await?;
    Ok(())
}

/// Load a specific session by ID.
pub async fn load_session(id: &str) -> anyhow::Result<ConversationSession> {
    let path = get_sessions_dir().await.join(format!("{}.json", id));
    let content = tokio::fs::read_to_string(&path).await?;
    Ok(serde_json::from_str(&content)?)
}

/// List all sessions, sorted by most-recently-updated first.
pub async fn list_sessions() -> Vec<ConversationSession> {
    let dir = get_sessions_dir().await;
    if !dir.exists() {
        return vec![];
    }

    let mut sessions = vec![];
    match tokio::fs::read_dir(&dir).await {
        Ok(mut entries) => {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Ok(content) = tokio::fs::read_to_string(&path).await {
                        if let Ok(session) = serde_json::from_str::<ConversationSession>(&content) {
                            sessions.push(session);
                        }
                    }
                }
            }
        }
        Err(_) => {}
    }

    sessions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    sessions
}

/// Delete a session by ID.
pub async fn delete_session(id: &str) -> anyhow::Result<()> {
    let path = get_sessions_dir().await.join(format!("{}.json", id));
    if path.exists() {
        tokio::fs::remove_file(&path).await?;
    }
    Ok(())
}

/// Rename (set the title of) a session.
pub async fn rename_session(id: &str, new_title: &str) -> anyhow::Result<()> {
    let mut session = load_session(id).await?;
    session.title = Some(new_title.to_string());
    session.updated_at = chrono::Utc::now();
    save_session(&session).await
}

/// Add a tag to a session (idempotent — duplicate tags are ignored).
pub async fn tag_session(id: &str, tag: &str) -> anyhow::Result<()> {
    let mut session = load_session(id).await?;
    let tag_str = tag.to_string();
    if !session.tags.contains(&tag_str) {
        session.tags.push(tag_str);
        session.updated_at = chrono::Utc::now();
        save_session(&session).await?;
    }
    Ok(())
}

/// Remove a tag from a session (no-op if tag is not present).
pub async fn untag_session(id: &str, tag: &str) -> anyhow::Result<()> {
    let mut session = load_session(id).await?;
    let before_len = session.tags.len();
    session.tags.retain(|t| t != tag);
    if session.tags.len() != before_len {
        session.updated_at = chrono::Utc::now();
        save_session(&session).await?;
    }
    Ok(())
}

/// Create a new session that is a branch of `source_id` at message index
/// `at_message_idx`.  The new session starts with messages
/// `[0, at_message_idx)` copied from the source.
pub async fn branch_session(
    source_id: &str,
    at_message_idx: usize,
    new_title: Option<&str>,
) -> anyhow::Result<ConversationSession> {
    let source = load_session(source_id).await?;
    let clamped_idx = at_message_idx.min(source.messages.len());
    let now = chrono::Utc::now();
    let branched = ConversationSession {
        id: uuid::Uuid::new_v4().to_string(),
        created_at: now,
        updated_at: now,
        messages: source.messages[..clamped_idx].to_vec(),
        model: source.model.clone(),
        title: new_title
            .map(|t| t.to_string())
            .or_else(|| source.title.as_ref().map(|t| format!("{} (branch)", t))),
        tags: source.tags.clone(),
        branch_from: Some(source_id.to_string()),
        branch_at_message: Some(clamped_idx),
        remote_session_url: None,
        total_cost: 0.0,
        total_tokens: 0,
        checkpoints: vec![],
        parent_session_id: None,
        fork_point_message_index: None,
    };
    save_session(&branched).await?;
    Ok(branched)
}

/// Search sessions whose title or tags contain `query` (case-insensitive
/// substring match).  Results are sorted by `updated_at` descending.
pub async fn search_sessions(query: &str) -> Vec<ConversationSession> {
    let lower_query = query.to_lowercase();
    let all = list_sessions().await;
    all.into_iter()
        .filter(|s| {
            // Check title
            if let Some(ref title) = s.title {
                if title.to_lowercase().contains(&lower_query) {
                    return true;
                }
            }
            // Check tags
            if s.tags
                .iter()
                .any(|t| t.to_lowercase().contains(&lower_query))
            {
                return true;
            }
            false
        })
        .collect()
}
