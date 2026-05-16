pub const APP_NAME: &str = "Boxagnts";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

// Models
pub const DEFAULT_MODEL: &str = "claude-opus-4-6";
pub const SONNET_MODEL: &str = "claude-sonnet-4-6";
pub const HAIKU_MODEL: &str = "claude-haiku-4-5-20251001";
pub const OPUS_MODEL: &str = "claude-opus-4-6";

// Token limits
pub const DEFAULT_MAX_TOKENS: u32 = 32_000;
pub const MAX_TOKENS_HARD_LIMIT: u32 = 65_536;
pub const DEFAULT_COMPACT_THRESHOLD: f32 = 0.9;
pub const MAX_TURNS_DEFAULT: u32 = 10;
pub const MAX_TOOL_ERRORS: u32 = 3;

// API endpoints & headers
pub const ANTHROPIC_API_BASE: &str = "https://api.anthropic.com";
pub const ANTHROPIC_API_VERSION: &str = "2023-06-01";
pub const ANTHROPIC_BETA_HEADER: &str =
    "interleaved-thinking-2025-05-14,token-efficient-tools-2025-02-19,files-api-2025-04-14,\
         effort-2025-11-24";






// Retry budget
pub const MAX_OUTPUT_TOKENS_RETRIES: u32 = 3;
pub const MAX_COMPACT_RETRIES: u32 = 3;

// Stop sequences
pub const STOP_SEQUENCE_END_OF_TURN: &str = "\n\nHuman:";