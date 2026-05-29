/// OAuth 2.0 PKCE authentication support.
///
/// Supports two login paths mirroring the TypeScript implementation:
/// - **Console** (`org:create_api_key` scope): exchanges access token for an API key.
/// - **Claude.ai** (`user:inference` scope): uses the access token as a Bearer credential.


use serde::{Deserialize, Serialize};

// ---- Production OAuth endpoints & constants ----

// Claude Code client ID, used in stealth-impersonation mode (see
// `claurst_core::oauth_config` for the matching request-time headers and
// system-prompt prefix wired into `claurst_api::AnthropicClient`).
pub const CLIENT_ID: &str = "9d1c250a-e61b-44d9-88ed-5944d1962f5e";
pub const CONSOLE_AUTHORIZE_URL: &str = "https://platform.claude.com/oauth/authorize";
pub const CLAUDE_AI_AUTHORIZE_URL: &str = "https://claude.com/cai/oauth/authorize";
pub const TOKEN_URL: &str = "https://platform.claude.com/v1/oauth/token";
pub const API_KEY_URL: &str =
    "https://api.anthropic.com/api/oauth/claude_cli/create_api_key";
pub const MANUAL_REDIRECT_URL: &str =
    "https://platform.claude.com/oauth/code/callback";
pub const CLAUDEAI_SUCCESS_URL: &str =
    "https://platform.claude.com/oauth/code/success?app=claude-code";
pub const CONSOLE_SUCCESS_URL: &str = "https://platform.claude.com/buy_credits\
        ?returnUrl=/oauth/code/success%3Fapp%3Dclaude-code";

/// All scopes requested during login (union of Console + Claude.ai scopes).
pub const ALL_SCOPES: &[&str] = &[
    "org:create_api_key",
    "user:profile",
    "user:inference",
    "user:sessions:claude_code",
    "user:mcp_servers",
    "user:file_upload",
];

/// Scope that identifies a Claude.ai subscription token (uses Bearer auth).
pub const CLAUDE_AI_INFERENCE_SCOPE: &str = "user:inference";

// ---- Stored token struct ----

/// Persisted OAuth tokens (saved to `~/.claurst/oauth_tokens.json`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OAuthTokens {
    pub access_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    /// Unix timestamp in milliseconds when the access token expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at_ms: Option<i64>,
    pub scopes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_type: Option<String>,
    /// API key created for Console-flow users (exchanged from access token).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
}

impl OAuthTokens {
    /// Returns true if the token requires Bearer-style authorization
    /// (i.e. Claude.ai subscription with `user:inference` scope).
    pub fn uses_bearer_auth(&self) -> bool {
        self.scopes.iter().any(|s| s == CLAUDE_AI_INFERENCE_SCOPE)
    }

    /// The credential to present to the Anthropic API:
    /// - Console flow: the stored `api_key` (sk-ant-…)
    /// - Claude.ai flow: the `access_token` itself (Bearer)
    pub fn effective_credential(&self) -> Option<&str> {
        if self.uses_bearer_auth() {
            if self.access_token.is_empty() { None } else { Some(&self.access_token) }
        } else {
            self.api_key.as_deref()
        }
    }

    /// True if the access token has passed (or is within 5 minutes of) its expiry.
    pub fn is_expired(&self) -> bool {
        if let Some(exp) = self.expires_at_ms {
            let buffer_ms: i64 = 5 * 60 * 1000;
            let now_ms = chrono::Utc::now().timestamp_millis();
            (now_ms + buffer_ms) >= exp
        } else {
            false
        }
    }

    pub async fn token_file_path() -> std::path::PathBuf {
        let saved_dir = crate::path::get_saved_dir().await;

        saved_dir
            .join("oauth_tokens.json")
    }

    pub async fn save(&self) -> anyhow::Result<()> {
        let path = Self::token_file_path().await;
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        tokio::fs::write(&path, serde_json::to_string_pretty(self)?).await?;
        Ok(())
    }

    pub async fn load() -> Option<Self> {
        let path = Self::token_file_path().await;
        let content = tokio::fs::read_to_string(&path).await.ok()?;
        serde_json::from_str(&content).ok()
    }

    pub async fn clear() -> anyhow::Result<()> {
        let path = Self::token_file_path().await;
        if path.exists() {
            tokio::fs::remove_file(&path).await?;
        }
        Ok(())
    }
}

// ---- PKCE helpers ----

/// Generate a 32-byte random code verifier, base64url-encoded (no padding).
pub fn generate_code_verifier() -> String {
    use base64::Engine;
    let mut bytes = [0u8; 32];
    let u1 = uuid::Uuid::new_v4();
    let u2 = uuid::Uuid::new_v4();
    bytes[..16].copy_from_slice(u1.as_bytes());
    bytes[16..].copy_from_slice(u2.as_bytes());
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

/// Derive the PKCE code challenge from a verifier: BASE64URL(SHA256(verifier)).
pub fn generate_code_challenge(verifier: &str) -> String {
    use base64::Engine;
    use sha2::{Digest, Sha256};
    let hash = Sha256::digest(verifier.as_bytes());
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(hash)
}

/// Generate a random OAuth state parameter for CSRF protection.
pub fn generate_state() -> String {
    use base64::Engine;
    let mut bytes = [0u8; 32];
    let u1 = uuid::Uuid::new_v4();
    let u2 = uuid::Uuid::new_v4();
    bytes[..16].copy_from_slice(u1.as_bytes());
    bytes[16..].copy_from_slice(u2.as_bytes());
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

// ---- URL builder ----

/// Build an OAuth authorization URL with all required PKCE parameters.
pub fn build_auth_url(
    authorize_base: &str,
    code_challenge: &str,
    state: &str,
    callback_port: u16,
    is_manual: bool,
) -> String {
    let mut u = url::Url::parse(authorize_base)
        .expect("valid OAuth authorize base URL");
    {
        let mut q = u.query_pairs_mut();
        q.append_pair("code", "true"); // tells the login page to show Claude Max upsell
        q.append_pair("client_id", CLIENT_ID);
        q.append_pair("response_type", "code");
        let redirect = if is_manual {
            MANUAL_REDIRECT_URL.to_string()
        } else {
            format!("http://localhost:{}/callback", callback_port)
        };
        q.append_pair("redirect_uri", &redirect);
        q.append_pair("scope", &ALL_SCOPES.join(" "));
        q.append_pair("code_challenge", code_challenge);
        q.append_pair("code_challenge_method", "S256");
        q.append_pair("state", state);
    }
    u.to_string()
}