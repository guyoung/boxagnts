use thiserror::Error;

/// The unified error type for Boxagnts.
#[derive(Error, Debug)]
pub enum ClaudeError {
    #[error("API error: {0}")]
    Api(String),

    #[error("API error {status}: {message}")]
    ApiStatus { status: u16, message: String },

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Tool error: {0}")]
    Tool(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Rate limit exceeded")]
    RateLimit,

    #[error("Context window exceeded")]
    ContextWindowExceeded,

    #[error("Max tokens reached")]
    MaxTokensReached,

    #[error("Cancelled")]
    Cancelled,

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("MCP error: {0}")]
    Mcp(String),

    #[error("{0}")]
    Other(String),
}

/// Convenience alias used throughout the project.
pub type Result<T> = std::result::Result<T, ClaudeError>;

impl ClaudeError {
    /// Return `true` when the caller should retry the request.
    pub fn is_retryable(&self) -> bool {
        matches!(
                self,
                ClaudeError::RateLimit
                    | ClaudeError::ApiStatus { status: 429, .. }
                    | ClaudeError::ApiStatus { status: 529, .. }
            )
    }

    /// Return `true` for errors that mean the conversation cannot continue
    /// without intervention (e.g. compaction or context-window reset).
    pub fn is_context_limit(&self) -> bool {
        matches!(
                self,
                ClaudeError::ContextWindowExceeded | ClaudeError::MaxTokensReached
            )
    }
}