use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

// ---- Hook configuration ----------------------------------------------

/// Events that can trigger hooks.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
pub enum HookEvent {
    /// Fires before a tool is executed.
    PreToolUse,
    /// Fires after a tool has returned its result.
    PostToolUse,
    /// Fires when the model finishes its turn (stop).
    Stop,
    /// Fires after the model samples a response, before tool execution.
    /// Corresponds to `hooks.PostModelTurn` in settings.json.
    PostModelTurn,
    /// Fires when the user submits a prompt.
    UserPromptSubmit,
    /// General-purpose notification event.
    Notification,
}

/// A single hook entry: a shell command to run on a specific event.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HookEntry {
    /// Shell command to execute. Receives event JSON on stdin.
    pub command: String,
    /// Optional tool name filter — only run for this tool (PreToolUse/PostToolUse).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_filter: Option<String>,
    /// If true, a non-zero exit code blocks the operation.
    #[serde(default)]
    pub blocking: bool,
}

// ---- AgentDefinition -------------------------------------------------

fn default_agent_access() -> String {
    "full".to_string()
}

fn default_true() -> bool {
    true
}

/// Definition of a named agent with per-agent model, permissions,
/// temperature, and system prompt.
pub fn api_key_env_vars_for_provider(provider_id: &str) -> &'static [&'static str] {
    match provider_id {
        "anthropic" => &["ANTHROPIC_API_KEY"],
        "openai" => &["OPENAI_API_KEY"],
        "google" | "google-vertex" => &["GOOGLE_API_KEY", "GOOGLE_GENERATIVE_AI_API_KEY"],
        "github-copilot" => &["GITHUB_TOKEN"],
        "groq" => &["GROQ_API_KEY"],
        "cerebras" => &["CEREBRAS_API_KEY"],
        "sambanova" => &["SAMBANOVA_API_KEY"],
        "deepseek" => &["DEEPSEEK_API_KEY"],
        "mistral" => &["MISTRAL_API_KEY"],
        "openrouter" => &["OPENROUTER_API_KEY"],
        "togetherai" | "together-ai" => &["TOGETHER_API_KEY"],
        "perplexity" => &["PERPLEXITY_API_KEY"],
        "cohere" => &["COHERE_API_KEY"],
        "xai" => &["XAI_API_KEY"],
        "deepinfra" => &["DEEPINFRA_API_KEY"],
        "azure" => &["AZURE_API_KEY"],
        "gitlab" => &["GITLAB_TOKEN"],
        "huggingface" => &["HF_TOKEN"],
        "nvidia" => &["NVIDIA_API_KEY"],
        "alibaba" | "qwen" => &["DASHSCOPE_API_KEY"],
        "venice" => &["VENICE_API_KEY"],
        "moonshot" | "moonshotai" => &["MOONSHOT_API_KEY"],
        "zhipu" | "zhipuai" => &["ZHIPU_API_KEY"],
        "zai" => &["ZAI_API_KEY"],
        "siliconflow" => &["SILICONFLOW_API_KEY"],
        "nebius" => &["NEBIUS_API_KEY"],
        "novita" => &["NOVITA_API_KEY"],
        "minimax" => &["MINIMAX_API_KEY"],
        "ovhcloud" => &["OVHCLOUD_API_KEY"],
        "scaleway" => &["SCALEWAY_API_KEY"],
        "vultr" | "vultr-ai" => &["VULTR_API_KEY"],
        "baseten" => &["BASETEN_API_KEY"],
        "friendli" => &["FRIENDLI_TOKEN"],
        "upstage" => &["UPSTAGE_API_KEY"],
        "stepfun" => &["STEPFUN_API_KEY"],
        "fireworks" => &["FIREWORKS_API_KEY"],
        "cloudflare" | "cloudflare-ai-gateway" | "cloudflare-workers-ai" => {
            &["CLOUDFLARE_API_TOKEN"]
        }
        "vercel" => &["AI_GATEWAY_API_KEY"],
        "helicone" => &["HELICONE_API_KEY"],
        "sap" | "sap-ai-core" => &["AICORE_SERVICE_KEY"],
        _ => &[],
    }
}

pub fn primary_api_key_env_var_for_provider(provider_id: &str) -> Option<&'static str> {
    api_key_env_vars_for_provider(provider_id).first().copied()
}

pub fn api_base_env_var_for_provider(provider_id: &str) -> Option<&'static str> {
    match provider_id {
        "anthropic" => Some("ANTHROPIC_BASE_URL"),
        "openai" => Some("OPENAI_BASE_URL"),
        "minimax" => Some("MINIMAX_BASE_URL"),
        "ollama" => Some("OLLAMA_HOST"),
        "lmstudio" | "lm-studio" => Some("LM_STUDIO_HOST"),
        "llamacpp" | "llama-cpp" | "llama-server" => Some("LLAMA_CPP_HOST"),
        _ => None,
    }
}

pub fn default_api_base_for_provider(provider_id: &str) -> Option<&'static str> {
    match provider_id {
        "anthropic" => Some(boxagnts_core::constants::ANTHROPIC_API_BASE),
        "openai" => Some("https://api.openai.com"),
        "minimax" => Some("https://api.minimax.io/anthropic"),
        "ollama" => Some("http://localhost:11434"),
        "lmstudio" | "lm-studio" => Some("http://localhost:1234"),
        "llamacpp" | "llama-cpp" | "llama-server" => Some("http://localhost:8080"),
        _ => None,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDefinition {
    /// Display name / description
    pub description: Option<String>,
    /// Model override for this agent (e.g., "anthropic/claude-haiku-4-5")
    pub model: Option<String>,
    /// Temperature override
    pub temperature: Option<f64>,
    /// System prompt prefix (prepended before the main system prompt)
    pub prompt: Option<String>,
    /// Permission restriction: "full", "read-only", "search-only"
    #[serde(default = "default_agent_access")]
    pub access: String,
    /// Whether to show in @agent autocomplete
    #[serde(default = "default_true")]
    pub visible: bool,
    /// Max agentic turns for this agent (overrides global)
    pub max_turns: Option<u32>,
    /// ANSI color for display: "cyan", "magenta", "green", etc.
    pub color: Option<String>,
}

impl Default for AgentDefinition {
    fn default() -> Self {
        Self {
            description: None,
            model: None,
            temperature: None,
            prompt: None,
            access: default_agent_access(),
            visible: true,
            max_turns: None,
            color: None,
        }
    }
}

// ---- ManagedAgentConfig ----------------------------------------------

/// Budget allocation strategy between manager and executor agents.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BudgetSplitPolicy {
    /// Shared pool — no split (default).
    SharedPool,
    /// Manager gets manager_pct% of total budget.
    Percentage { manager_pct: u8 },
    /// Hard USD caps per role.
    FixedCaps { manager_usd: f64, executor_usd: f64 },
}

impl Default for BudgetSplitPolicy {
    fn default() -> Self {
        BudgetSplitPolicy::SharedPool
    }
}

/// Configuration for manager-executor agent architecture.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedAgentConfig {
    pub enabled: bool,
    /// "provider/model" string, e.g. "anthropic/claude-opus-4-6"
    pub manager_model: String,
    /// "provider/model" string, e.g. "anthropic/claude-sonnet-4-6"
    pub executor_model: String,
    #[serde(default = "default_executor_max_turns")]
    pub executor_max_turns: u32,
    #[serde(default = "default_max_concurrent_executors")]
    pub max_concurrent_executors: u32,
    #[serde(default)]
    pub budget_split: BudgetSplitPolicy,
    #[serde(default)]
    pub total_budget_usd: Option<f64>,
    #[serde(default)]
    pub preset_name: Option<String>,
    #[serde(default)]
    pub executor_isolation: bool,
}

fn default_executor_max_turns() -> u32 {
    10
}
fn default_max_concurrent_executors() -> u32 {
    4
}

/// A named preset for common manager-executor configurations.
pub struct ManagedAgentPreset {
    pub name: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub manager_model: &'static str,
    pub executor_model: &'static str,
    pub executor_max_turns: u32,
    pub max_concurrent_executors: u32,
}

pub fn builtin_managed_agent_presets() -> Vec<ManagedAgentPreset> {
    vec![
        ManagedAgentPreset {
            name: "anthropic-tiered",
            label: "Anthropic Tiered",
            description: "Opus 4.6 manages, Sonnet 4.6 executes (best quality)",
            manager_model: "anthropic/claude-opus-4-6",
            executor_model: "anthropic/claude-sonnet-4-6",
            executor_max_turns: 10,
            max_concurrent_executors: 4,
        },
        ManagedAgentPreset {
            name: "anthropic-budget",
            label: "Anthropic Budget",
            description: "Sonnet 4.6 manages, Haiku 4.5 executes (cost-optimized)",
            manager_model: "anthropic/claude-sonnet-4-6",
            executor_model: "anthropic/claude-haiku-4-5-20251001",
            executor_max_turns: 10,
            max_concurrent_executors: 6,
        },
        ManagedAgentPreset {
            name: "google-tiered",
            label: "Google Tiered",
            description: "Gemini 2.5 Pro manages, Flash executes",
            manager_model: "google/gemini-2.5-pro",
            executor_model: "google/gemini-2.5-flash",
            executor_max_turns: 10,
            max_concurrent_executors: 4,
        },
        ManagedAgentPreset {
            name: "cross-opus-flash",
            label: "Cross: Opus + Flash",
            description: "Anthropic Opus manages, Google Flash executes (cheapest executors)",
            manager_model: "anthropic/claude-opus-4-6",
            executor_model: "google/gemini-2.5-flash",
            executor_max_turns: 10,
            max_concurrent_executors: 6,
        },
        ManagedAgentPreset {
            name: "openai-tiered",
            label: "OpenAI Tiered",
            description: "o3 manages, gpt-4o executes",
            manager_model: "openai/o3",
            executor_model: "openai/gpt-4o",
            executor_max_turns: 10,
            max_concurrent_executors: 4,
        },
        ManagedAgentPreset {
            name: "cross-openai-anthropic",
            label: "Cross: OpenAI + Anthropic",
            description: "o3 manages, Sonnet 4.6 executes",
            manager_model: "openai/o3",
            executor_model: "anthropic/claude-sonnet-4-6",
            executor_max_turns: 10,
            max_concurrent_executors: 4,
        },
    ]
}

// ---- ProviderConfig --------------------------------------------------

/// Per-provider configuration: API keys, base URLs, and options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// API key (overrides environment variable)
    pub api_key: Option<String>,
    /// Override the default base URL for this provider
    pub api_base: Option<String>,
    /// Whether this provider is enabled (default: true)
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Model ID whitelist (empty = allow all)
    #[serde(default)]
    pub models_whitelist: Vec<String>,
    /// Model ID blacklist
    #[serde(default)]
    pub models_blacklist: Vec<String>,
    /// Provider-specific options (passed through to provider implementation)
    #[serde(default)]
    pub options: HashMap<String, serde_json::Value>,
}

impl Default for ProviderConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            api_base: None,
            enabled: true,
            models_whitelist: Vec::new(),
            models_blacklist: Vec::new(),
            options: HashMap::new(),
        }
    }
}

// ---- Config ----------------------------------------------------------

/// Top-level configuration values, merged from CLI args + settings file + env.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub api_key: Option<String>,
    pub model: Option<String>,
    pub max_tokens: Option<u32>,
    pub permission_mode: PermissionMode,
    pub theme: Theme,
    #[serde(default)]
    pub output_style: Option<String>,
    pub auto_compact: bool,
    pub compact_threshold: f32,
    pub verbose: bool,
    pub output_format: OutputFormat,
    pub mcp_servers: Vec<McpServerConfig>,
    #[serde(default)]
    // pub lsp_servers: Vec<boxagnts_core::lsp::LspServerConfig>,
    pub allowed_tools: Vec<String>,
    pub disallowed_tools: Vec<String>,
    pub env: HashMap<String, String>,
    pub enable_all_mcp_servers: bool,
    pub custom_system_prompt: Option<String>,
    pub append_system_prompt: Option<String>,
    pub disable_claude_mds: bool,
    #[serde(default = "default_max_auto_turns")]
    pub max_auto_turns: u32,
    pub project_dir: Option<PathBuf>,
    #[serde(default)]
    pub workspace_paths: Vec<PathBuf>,
    /// Additional directories granted access via --add-dir.
    #[serde(default)]
    pub additional_dirs: Vec<PathBuf>,
    /// Event hooks: map of event → list of hook commands.
    #[serde(default)]
    pub hooks: HashMap<HookEvent, Vec<HookEntry>>,
    /// Active provider ID (default: "anthropic")
    #[serde(default)]
    pub provider: Option<String>,
    /// Per-provider configurations
    #[serde(default)]
    pub provider_configs: HashMap<String, ProviderConfig>,
    /// Formatter configurations (copied from Settings on load).
    #[serde(default)]
    pub formatter: HashMap<String, FormatterConfig>,
    /// User-defined command templates (copied from Settings on load).
    #[serde(default)]
    pub commands: HashMap<String, CommandTemplate>,
    /// Named agent definitions (copied from Settings on load).
    #[serde(default)]
    pub agents: HashMap<String, AgentDefinition>,
    /// Skill-discovery configuration (copied from Settings on load).
    #[serde(default)]
    pub skills: SkillsConfig,
    /// Managed agent (manager-executor) configuration.
    #[serde(default)]
    pub managed_agents: Option<ManagedAgentConfig>,
    /// Allowed outbound hosts
    #[serde(default)]
    pub allowed_outbound_hosts: Vec<String>,
}

fn default_max_auto_turns() -> u32 {
    5
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum PermissionMode {
    #[default]
    Default,
    AcceptEdits,
    BypassPermissions,
    Plan,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum Theme {
    #[default]
    Default,
    Dark,
    Light,
    Custom(String),
    Deuteranopia,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    #[default]
    Text,
    Json,
    StreamJson,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerConfig {
    pub name: String,
    pub command: Option<String>,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: HashMap<String, String>,
    pub url: Option<String>,
    #[serde(rename = "type", default = "default_mcp_type")]
    pub server_type: String,
}

fn default_mcp_type() -> String {
    "stdio".to_string()
}

// ---- SkillsConfig ----------------------------------------------------

/// Configuration for the skill-discovery system.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SkillsConfig {
    /// Additional directories to search for skill `.md` files.
    #[serde(default)]
    pub paths: Vec<String>,
    /// Git repository URLs to fetch skills from (cloned once, then cached).
    #[serde(default)]
    pub urls: Vec<String>,
}

// ---- Settings --------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Settings {
    #[serde(default)]
    pub config: Config,
    pub version: Option<u32>,
    #[serde(default)]
    pub projects: HashMap<String, ProjectSettings>,
    #[serde(default, rename = "remoteControlAtStartup")]
    pub remote_control_at_startup: bool,
    /// Persisted permission rules saved by the user across sessions.
    #[serde(default, rename = "permissionRules")]
    pub permission_rules: Vec<crate::permissions::SerializedPermissionRule>,
    /// Names of plugins that have been explicitly enabled by the user.
    #[serde(default, rename = "enabledPlugins")]
    pub enabled_plugins: std::collections::HashSet<String>,
    /// Names of plugins that have been explicitly disabled by the user.
    #[serde(default, rename = "disabledPlugins")]
    pub disabled_plugins: std::collections::HashSet<String>,
    /// Whether the user has completed the first-launch onboarding flow.
    /// Mirrors TS `hasAcknowledgedSafetyNotice` / `hasCompletedOnboarding`.
    #[serde(default, rename = "hasCompletedOnboarding")]
    pub has_completed_onboarding: bool,
    /// App version at last launch — used to detect upgrades and show release notes.
    #[serde(default, rename = "lastSeenVersion")]
    pub last_seen_version: Option<String>,
    /// Active provider ID at the settings level (e.g. "anthropic", "openai").
    #[serde(default)]
    pub provider: Option<String>,
    /// Per-provider configurations stored in settings.json.
    #[serde(default)]
    pub providers: HashMap<String, ProviderConfig>,
    /// User-defined slash command templates.
    #[serde(default)]
    pub commands: HashMap<String, CommandTemplate>,
    /// Formatter configurations keyed by a user-defined name.
    #[serde(default)]
    pub formatter: HashMap<String, FormatterConfig>,
    /// Named agent definitions (overrides built-in defaults).
    #[serde(default)]
    pub agents: HashMap<String, AgentDefinition>,
    /// Skill-discovery configuration (extra paths and git URLs).
    #[serde(default)]
    pub skills: SkillsConfig,
    /// Managed agent (manager-executor) configuration.
    #[serde(default)]
    pub managed_agents: Option<ManagedAgentConfig>,
}

/// A user-defined slash command template.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommandTemplate {
    /// The template string; `$ARGUMENTS` gets replaced with user input.
    pub template: String,
    /// Optional description shown in /help.
    pub description: Option<String>,
    /// Optional agent to use (e.g. "plan").
    pub agent: Option<String>,
    /// Optional model override (e.g. "anthropic/claude-haiku-4-5").
    pub model: Option<String>,
}

/// Configuration for a file formatter tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatterConfig {
    /// Command to run, e.g. `["prettier", "--write"]`.
    pub command: Vec<String>,
    /// File extensions this formatter handles, e.g. `[".ts", ".tsx", ".js"]`.
    pub extensions: Vec<String>,
    /// Whether this formatter is disabled.
    #[serde(default)]
    pub disabled: bool,
}

impl Default for FormatterConfig {
    fn default() -> Self {
        Self {
            command: Vec::new(),
            extensions: Vec::new(),
            disabled: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectSettings {
    #[serde(default)]
    pub allowed_tools: Vec<String>,
    #[serde(default)]
    pub mcp_servers: Vec<McpServerConfig>,
    pub custom_system_prompt: Option<String>,
}

/// Return the three built-in named agent definitions.
/// User-defined agents in `settings.json` can override these by name.
pub fn default_agents() -> HashMap<String, AgentDefinition> {
    let mut m = HashMap::new();
    m.insert("build".to_string(), AgentDefinition {
        description: Some("Full-access agent for implementing features and fixing bugs".to_string()),
        model: None,
        temperature: None,
        prompt: Some("You are the build agent. You have full access to read, write, and execute. Focus on implementing the requested changes completely and correctly.".to_string()),
        access: "full".to_string(),
        visible: true,
        max_turns: None,
        color: Some("cyan".to_string()),
    });
    m.insert("plan".to_string(), AgentDefinition {
        description: Some("Read-only agent for analyzing code and planning changes".to_string()),
        model: None,
        temperature: None,
        prompt: Some("You are the plan agent. You can read files and analyze code but cannot write files or execute commands. Focus on understanding the codebase and describing what changes should be made.".to_string()),
        access: "read-only".to_string(),
        visible: true,
        max_turns: Some(20),
        color: Some("yellow".to_string()),
    });
    m.insert("explore".to_string(), AgentDefinition {
        description: Some("Fast search-only agent for code exploration".to_string()),
        model: None,
        temperature: None,
        prompt: Some("You are the explore agent. You can search and read files. Focus on quickly finding relevant code and answering questions about the codebase.".to_string()),
        access: "search-only".to_string(),
        visible: true,
        max_turns: Some(15),
        color: Some("green".to_string()),
    });
    m
}

impl Config {
    pub fn selected_provider_id(&self) -> &str {
        self.model
            .as_deref()
            .and_then(|model| model.split_once('/').map(|(provider, _)| provider))
            .or_else(|| self.provider.as_deref())
            .unwrap_or("anthropic")
    }

    /// Resolve the effective model, falling back to a provider-appropriate default.
    ///
    /// When a non-Anthropic provider is active and no model is explicitly set,
    /// returns that provider's canonical default model instead of `DEFAULT_MODEL`
    /// (which is Claude-specific).
    pub fn effective_model(&self) -> &str {
        if let Some(ref m) = self.model {
            return m;
        }
        match self.provider.as_deref() {
            Some("openai") => "gpt-4o",
            Some("google") => "gemini-2.5-flash",
            Some("groq") => "llama-3.3-70b-versatile",
            Some("cerebras") => "llama-3.3-70b",
            Some("deepseek") => "deepseek-v4-pro",
            Some("mistral") => "mistral-large-latest",
            Some("xai") => "grok-2",
            Some("openrouter") => "anthropic/claude-sonnet-4",
            Some("togetherai") | Some("together-ai") => "meta-llama/Llama-3.3-70B-Instruct-Turbo",
            Some("perplexity") => "sonar-pro",
            Some("cohere") => "command-r-plus",
            Some("deepinfra") => "meta-llama/Llama-3.3-70B-Instruct",
            Some("github-copilot") => "gpt-4o",
            Some("ollama") => "llama3.2",
            Some("lmstudio") => "default",
            Some("llamacpp") => "default",
            Some("custom-openai") => "default",
            Some("azure") => "gpt-4o",
            Some("amazon-bedrock") => "anthropic.claude-sonnet-4-6-v1",
            Some("venice") => "llama-3.3-70b",
            _ => boxagnts_core::constants::DEFAULT_MODEL, // Anthropic default
        }
    }

    /// Resolve the effective max-tokens.
    pub fn effective_max_tokens(&self) -> u32 {
        self.max_tokens
            .unwrap_or(boxagnts_core::constants::DEFAULT_MAX_TOKENS)
    }

    /// Resolve the effective compact threshold (0.0 - 1.0).
    pub fn effective_compact_threshold(&self) -> f32 {
        if self.compact_threshold > 0.0 {
            self.compact_threshold
        } else {
            boxagnts_core::constants::DEFAULT_COMPACT_THRESHOLD
        }
    }

    /// Resolve the effective output style for system-prompt assembly.
    pub fn effective_output_style(&self) -> boxagnts_core::system_prompt::OutputStyle {
        self.output_style
            .as_deref()
            .map(boxagnts_core::system_prompt::OutputStyle::from_str)
            .unwrap_or_default()
    }

    /// Resolve the prompt text for the selected output style, including
    /// user-defined styles loaded from `<cwd>/.boxagnts/output-styles/`.
    pub fn resolve_output_style_prompt(&self) -> Option<String> {
        let style_name = self.output_style.as_deref().unwrap_or("default");
        let styles = crate::output_styles::all_styles();
        crate::output_styles::find_style(&styles, style_name)
            .map(|style| style.prompt.clone())
            .filter(|prompt| !prompt.trim().is_empty())
    }

    pub fn resolve_provider_api_key(&self, provider_id: &str) -> Option<String> {
        let provider_cfg = self.provider_configs.get(provider_id);

        if provider_cfg.is_some_and(|provider| !provider.enabled) {
            return None;
        }

        if provider_id == self.selected_provider_id() {
            if let Some(key) = self.api_key.clone().filter(|key| !key.is_empty()) {
                return Some(key);
            }
        }

        if let Some(key) = provider_cfg
            .and_then(|provider| provider.api_key.clone())
            .filter(|key| !key.is_empty())
        {
            return Some(key);
        }

        if let Some(key) = api_key_env_vars_for_provider(provider_id)
            .iter()
            .find_map(|var| std::env::var(var).ok().filter(|v| !v.is_empty()))
        {
            return Some(key);
        }

        futures::executor::block_on(async {
            crate::auth_store::AuthStore::load()
                .await
                .api_key_for(provider_id)
        })
    }

    pub fn resolve_anthropic_api_key(&self) -> Option<String> {
        self.api_key
            .clone()
            .filter(|key| !key.is_empty())
            .or_else(|| {
                self.provider_configs
                    .get("anthropic")
                    .and_then(|provider| provider.api_key.clone())
                    .filter(|key| !key.is_empty())
            })
            .or_else(|| {
                api_key_env_vars_for_provider("anthropic")
                    .iter()
                    .find_map(|var| std::env::var(var).ok().filter(|v| !v.is_empty()))
            })
    }

    /// Resolve the API key for the active provider.
    pub async fn resolve_api_key(&self) -> Option<String> {
        self.resolve_provider_api_key(self.selected_provider_id())
    }

    /// Async variant: also checks `<cwd>/oauth_tokens.json`.
    /// Returns `(credential, use_bearer_auth)`.
    /// - For Console OAuth flow: credential is the stored API key, bearer=false.
    /// - For Claude.ai OAuth flow: credential is the access token, bearer=true.
    /// Silently attempts token refresh when the access token is expired.
    pub async fn resolve_auth_async(&self) -> Option<(String, bool)> {
        if self.selected_provider_id() != "anthropic" {
            return self.resolve_api_key().await.map(|key| (key, false));
        }

        /***
        self.resolve_anthropic_auth_async().await
        ***/

        None
    }

    /***
    pub async fn resolve_anthropic_auth_async(&self) -> Option<(String, bool)> {
        if let Some(key) = self.resolve_anthropic_api_key() {
            return Some((key, false));
        }

        let tokens = boxagnts_core::oauth::OAuthTokens::load().await?;

        // If expired and we have a refresh token, attempt silent refresh.
        // Clone the refresh token up-front so we don't borrow `tokens` during the async call.
        let refresh_token_owned = tokens.refresh_token.clone();
        let tokens = if tokens.is_expired() {
            if let Some(rt) = refresh_token_owned {
                // Inline the refresh HTTP call (cc_core can't depend on cc_cli::oauth_flow).
                let body = serde_json::json!({
                        "grant_type": "refresh_token",
                        "refresh_token": rt,
                        "client_id": boxagnts_core::oauth::CLIENT_ID,
                        "scope": boxagnts_core::oauth::ALL_SCOPES.join(" "),
                    });
                let refreshed = 'refresh: {
                    let Ok(client) = reqwest::Client::builder()
                        .timeout(std::time::Duration::from_secs(30))
                        .build() else { break 'refresh None; };
                    let Ok(resp) = client
                        .post(boxagnts_core::oauth::TOKEN_URL)
                        .header("content-type", "application/json")
                        .json(&body)
                        .send()
                        .await else { break 'refresh None; };
                    if !resp.status().is_success() { break 'refresh None; }
                    let Ok(data) = resp.json::<serde_json::Value>().await else { break 'refresh None; };
                    let new_at = data["access_token"].as_str().unwrap_or("").to_string();
                    if new_at.is_empty() { break 'refresh None; }
                    let new_rt = data["refresh_token"].as_str().map(String::from);
                    let exp_in = data["expires_in"].as_u64().unwrap_or(3600);
                    let exp_ms = chrono::Utc::now().timestamp_millis() + (exp_in as i64 * 1000);
                    let scopes: Vec<String> = data["scope"]
                        .as_str().unwrap_or("").split_whitespace().map(String::from).collect();
                    let mut r = tokens.clone();
                    r.access_token = new_at;
                    if let Some(nrt) = new_rt { r.refresh_token = Some(nrt); }
                    r.expires_at_ms = Some(exp_ms);
                    r.scopes = scopes;
                    let _ = r.save().await;
                    Some(r)
                };
                refreshed.unwrap_or(tokens)
            } else {
                tokens // expired, no refresh token → can't fix
            }
        } else {
            tokens
        };

        if let Some(cred) = tokens.effective_credential() {
            Some((cred.to_string(), tokens.uses_bearer_auth()))
        } else {
            None
        }
    }

    ***/

    pub fn resolve_provider_api_base(&self, provider_id: &str) -> Option<String> {
        let provider_cfg = self.provider_configs.get(provider_id);
        if provider_cfg.is_some_and(|provider| !provider.enabled) {
            return None;
        }

        provider_cfg
            .and_then(|provider| provider.api_base.clone())
            .filter(|base| !base.is_empty())
            .or_else(|| {
                api_base_env_var_for_provider(provider_id)
                    .and_then(|name| std::env::var(name).ok())
                    .filter(|base| !base.is_empty())
            })
            .or_else(|| default_api_base_for_provider(provider_id).map(str::to_owned))
    }

    pub fn resolve_anthropic_api_base(&self) -> String {
        self.resolve_provider_api_base("anthropic")
            .unwrap_or_else(|| boxagnts_core::constants::ANTHROPIC_API_BASE.to_string())
    }

    /// Resolve the API base URL for the active provider.
    pub fn resolve_api_base(&self) -> String {
        self.resolve_provider_api_base(self.selected_provider_id())
            .unwrap_or_else(|| self.resolve_anthropic_api_base())
    }
}

impl Settings {
    /// Full path to settings JSON file.
    pub async fn get_settings_path() -> PathBuf {
        crate::path::get_saved_dir().await.join("settings.json")
    }

    /// Load settings from disk, returning defaults when the file is missing.
    pub async fn load() -> anyhow::Result<Self> {
        let path = Self::get_settings_path().await;

        if path.exists() {
            let content = tokio::fs::read_to_string(&path).await?;

            let settings: Settings = serde_json::from_str(&content).unwrap_or_default();

            Ok(settings)
        } else {
            let settings: Settings = Settings::default();

            Ok(settings)
        }
    }

    /// Persist settings to disk.
    pub async fn save(&self) -> anyhow::Result<()> {
        let path = Self::get_settings_path().await;
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        let content = serde_json::to_string_pretty(self)?;
        tokio::fs::write(&path, content).await?;

        Ok(())
    }

    /// Return the effective `Config`, merging top-level provider settings
    /// into the embedded `config` field.
    ///
    /// - `settings.provider` wins over `settings.config.provider` (if set).
    /// - `settings.providers` entries are merged into `config.provider_configs`,
    ///   with the embedded config values taking precedence for keys already present.
    pub fn effective_config(&self) -> Config {
        let mut config = self.config.clone();
        // Top-level `provider` key overrides config.provider when set.
        if self.provider.is_some() && config.provider.is_none() {
            config.provider = self.provider.clone();
        }
        // Merge top-level `providers` map into config.provider_configs.
        for (id, pc) in &self.providers {
            config
                .provider_configs
                .entry(id.clone())
                .or_insert_with(|| pc.clone());
        }
        // Copy top-level formatters and commands into config.
        for (k, v) in &self.formatter {
            config
                .formatter
                .entry(k.clone())
                .or_insert_with(|| v.clone());
        }
        for (k, v) in &self.commands {
            config
                .commands
                .entry(k.clone())
                .or_insert_with(|| v.clone());
        }
        // Copy top-level agent definitions into config.
        for (k, v) in &self.agents {
            config.agents.entry(k.clone()).or_insert_with(|| v.clone());
        }
        // Copy skills config into effective config (paths and urls merged).
        for p in &self.skills.paths {
            if !config.skills.paths.contains(p) {
                config.skills.paths.push(p.clone());
            }
        }
        for u in &self.skills.urls {
            if !config.skills.urls.contains(u) {
                config.skills.urls.push(u.clone());
            }
        }
        config
    }
}

/// Strip `//` line-comments and `/* */` block-comments from a JSON string
/// (JSONC format), preserving newlines for error-message line numbers.
pub fn strip_jsonc_comments(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    let mut in_string = false;
    let mut prev_char = '\0';

    while let Some(ch) = chars.next() {
        if in_string {
            if ch == '"' && prev_char != '\\' {
                in_string = false;
            }
            result.push(ch);
            prev_char = ch;
            continue;
        }
        if ch == '"' {
            in_string = true;
            result.push(ch);
            prev_char = ch;
            continue;
        }
        if ch == '/' {
            match chars.peek() {
                Some('/') => {
                    // Line comment — skip to end of line.
                    for c in chars.by_ref() {
                        if c == '\n' {
                            result.push('\n');
                            break;
                        }
                    }
                }
                Some('*') => {
                    // Block comment — skip until `*/`.
                    chars.next();
                    let mut prev = '\0';
                    for c in chars.by_ref() {
                        if prev == '*' && c == '/' {
                            break;
                        }
                        if c == '\n' {
                            result.push('\n');
                        }
                        prev = c;
                    }
                }
                _ => result.push(ch),
            }
            prev_char = '\0';
            continue;
        }
        result.push(ch);
        prev_char = ch;
    }
    result
}

/// Replace `{env:VARNAME}` patterns in a string with environment variable
/// values.  Missing variables are replaced with an empty string.
pub fn substitute_env_vars(s: &str) -> String {
    let mut result = s.to_string();
    loop {
        match result.find("{env:") {
            None => break,
            Some(start) => match result[start..].find('}') {
                None => break,
                Some(rel_end) => {
                    let var_name = result[start + 5..start + rel_end].to_string();
                    let value = std::env::var(&var_name).unwrap_or_default();
                    result.replace_range(start..start + rel_end + 1, &value);
                }
            },
        }
    }
    result
}
