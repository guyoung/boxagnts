// cc-query: The core agentic query loop.
//
// This crate implements the main conversation loop that:
// 1. Sends messages to the Anthropic API
// 2. Processes streaming responses
// 3. Detects tool-use requests and dispatches them
// 4. Feeds tool results back to the model
// 5. Handles auto-compact when the context window fills up
// 6. Manages stop conditions (end_turn, max_turns, cancellation)

mod compact;
mod query;
mod managed_orchestrator;

use tokio::sync::mpsc;
use serde_json::Value;
use tracing::{debug, warn};

use boxagnts_core::types::{ContentBlock, Message, ToolResultContent, UsageInfo};
use boxagnts_core::error::ClaudeError;
use boxagnts_api::{ AnthropicStreamEvent, SystemPrompt, StreamHandler,  };


use boxagnts_workspace::config::Config;
use boxagnts_tools::{Tool, ToolContext, ToolResult};

pub use query::run_query_loop;
pub use query::run_single_query;


/// Outcome of a single query-loop run.
#[derive(Debug)]
pub enum QueryOutcome {
    /// The model finished its turn (end_turn stop reason).
    EndTurn { message: Message, usage: UsageInfo },
    /// The model hit max_tokens.
    MaxTokens {
        partial_message: Message,
        usage: UsageInfo,
    },
    /// The conversation was cancelled by the user.
    Cancelled,
    /// An unrecoverable error occurred.
    Error(ClaudeError),
    /// The configured USD budget was exceeded.
    BudgetExceeded { cost_usd: f64, limit_usd: f64 },
}


/// Configuration for a single query-loop invocation.
#[derive(Clone)]
pub struct QueryConfig {
    pub model: String,
    pub max_tokens: u32,
    pub max_turns: u32,
    pub system_prompt: Option<String>,
    pub append_system_prompt: Option<String>,
    pub output_style: boxagnts_core::system_prompt::OutputStyle,
    pub output_style_prompt: Option<String>,
    pub working_directory: Option<String>,
    pub thinking_budget: Option<u32>,
    pub temperature: Option<f32>,
    /// Maximum cumulative character count of all tool results in the message
    /// history before older results are replaced with a truncation notice.
    /// Mirrors the TS `applyToolResultBudget` mechanism.  Default: 50_000.
    pub tool_result_budget: usize,
    /// Optional effort level.  When set and `thinking_budget` is `None`,
    /// the effort level's `thinking_budget_tokens()` is used as the
    /// thinking budget.  Also provides a temperature override when the
    /// level specifies one.
    pub effort_level: Option<boxagnts_core::effort::EffortLevel>,
    /// T1-4: Optional shared command queue.
    ///
    /// When set, the query loop drains this queue before each API call and
    /// injects any resulting messages into the conversation.  The queue is
    /// shared (Arc-backed) so the TUI input thread can push commands while the
    /// loop is waiting for a model response.
    // pub command_queue: Option<CommandQueue>,
    /// T1-5: Optional shared skill index.
    ///
    /// When set, `prefetch_skills` is spawned once before the loop begins and
    /// the resulting index is used to inject a skill listing attachment into
    /// the conversation context.
    // pub skill_index: Option<SharedSkillIndex>,
    /// Optional USD spend cap. The query loop checks accumulated cost after
    /// each turn and aborts with `QueryOutcome::BudgetExceeded` when exceeded.
    pub max_budget_usd: Option<f64>,
    /// Fallback model name. Used when the primary model returns overloaded /
    /// rate-limit errors (mirrors TS `--fallback-model`).
    pub fallback_model: Option<String>,
    /// Optional ProviderRegistry for dispatching to non-Anthropic providers.
    /// When `config.provider` is set to something other than "anthropic" and
    /// this registry contains that provider, the registry's provider is used
    /// instead of `AnthropicClient`.
    pub provider_registry: Option<std::sync::Arc<boxagnts_api::ProviderRegistry>>,
    /// Active agent name (e.g., "build", "plan", "explore", or None for default).
    pub agent_name: Option<String>,
    /// Resolved agent definition for the current session.
    pub agent_definition: Option<boxagnts_workspace::config::AgentDefinition>,
    /// Optional shared model registry for dynamic provider and model resolution.
    /// When set, the query loop uses this instead of constructing a fresh registry.
    pub model_registry: Option<std::sync::Arc<boxagnts_api::ModelRegistry>>,
    /// Managed agent (manager-executor) configuration.
    pub managed_agents: Option<boxagnts_workspace::config::ManagedAgentConfig>,
    pub log_context: bool,
}

impl Default for QueryConfig {
    fn default() -> Self {
        Self {
            model: boxagnts_core::constants::DEFAULT_MODEL.to_string(),
            max_tokens: boxagnts_core::constants::DEFAULT_MAX_TOKENS,
            max_turns: boxagnts_core::constants::MAX_TURNS_DEFAULT,
            system_prompt: None,
            append_system_prompt: None,
            output_style: boxagnts_core::system_prompt::OutputStyle::Default,
            output_style_prompt: None,
            working_directory: None,
            thinking_budget: None,
            temperature: None,
            tool_result_budget: 50_000,
            effort_level: None,
            // command_queue: None,
            // skill_index: None,
            max_budget_usd: None,
            fallback_model: None,
            provider_registry: None,
            agent_name: None,
            agent_definition: None,
            model_registry: None,
            managed_agents: None,
            log_context: false,
        }
    }
}

impl QueryConfig {
    pub fn from_config(cfg: &Config) -> Self {
        Self {
            model: cfg.effective_model().to_string(),
            max_tokens: cfg.effective_max_tokens(),
            output_style: cfg.effective_output_style(),
            output_style_prompt: cfg.resolve_output_style_prompt(),
            working_directory: cfg.project_dir.as_ref().map(|p| p.display().to_string()),
            managed_agents: cfg.managed_agents.clone(),
            ..Default::default()
        }
    }

    /// Build a QueryConfig using dynamic model resolution from the model registry.
    ///
    /// Prefers the best model for the configured provider (from models.dev data)
    /// over the hardcoded defaults.
    pub fn from_config_with_registry(cfg: &Config, registry: &boxagnts_api::ModelRegistry) -> Self {
        // We can't move the Arc here, but we need a clone for the query loop.
        // Callers typically wrap the registry in an Arc already.
        Self {
            model: boxagnts_api::effective_model_for_config(cfg, registry),
            max_tokens: cfg.effective_max_tokens(),
            output_style: cfg.effective_output_style(),
            output_style_prompt: cfg.resolve_output_style_prompt(),
            working_directory: cfg.project_dir.as_ref().map(|p| p.display().to_string()),
            managed_agents: cfg.managed_agents.clone(),
            ..Default::default()
        }
    }
}

/// Events emitted by the query loop for the TUI to render.
#[derive(Debug, Clone)]
pub enum QueryEvent {
    /// A stream event from the API.
    Stream(AnthropicStreamEvent),
    /// A tool is about to be executed.
    ToolStart {
        tool_name: String,
        tool_id: String,
        input_json: String,
    },
    /// A tool has finished executing.
    ToolEnd {
        tool_name: String,
        tool_id: String,
        result: String,
        is_error: bool,
    },
    /// The model finished a turn.
    TurnComplete {
        turn: u32,
        stop_reason: String,
        usage: Option<UsageInfo>,
    },
    /// An informational status message.
    Status(String),
    /// An error.
    Error(String),
    /// Token usage has crossed a warning threshold.
    /// `state` is Warning (≥ 80 %) or Critical (≥ 95 %).
    /// `pct_used` is the fraction of the context window consumed (0.0–1.0).
    TokenWarning {
        state: compact::TokenWarningState,
        pct_used: f64,
    },
}


/// Stream handler that forwards events to an unbounded channel.
struct ChannelStreamHandler {
    tx: mpsc::UnboundedSender<QueryEvent>,
}

impl StreamHandler for ChannelStreamHandler {
    fn on_event(&self, event: &AnthropicStreamEvent) {
        let _ = self.tx.send(QueryEvent::Stream(event.clone()));
    }
}


/// Build the system prompt from config.
///
/// Delegates to `boxagnts_core::system_prompt::build_system_prompt` so that all
/// default content (capabilities, safety guidelines, dynamic-boundary marker,
/// etc.) is assembled in one place.  The `QueryConfig` fields map directly to
/// `SystemPromptOptions`:
///
/// - `system_prompt`        → `custom_system_prompt` (added to cacheable block)
/// - `append_system_prompt` → `append_system_prompt` (added after boundary)
fn build_system_prompt(config: &QueryConfig) -> SystemPrompt {
    use boxagnts_core::system_prompt::SystemPromptOptions;

    let opts = SystemPromptOptions {
        custom_system_prompt: config.system_prompt.clone(),
        append_system_prompt: config.append_system_prompt.clone(),
        // All other fields use sensible defaults:
        // - prefix:                auto-detect from env
        // - memory_content:        empty (callers inject via append if needed)
        // - replace_system_prompt: false (additive mode)
        // - coordinator_mode:      false
        output_style: config.output_style,
        custom_output_style_prompt: config.output_style_prompt.clone(),
        working_directory: config.working_directory.clone(),
        ..Default::default()
    };

    let text = boxagnts_core::system_prompt::build_system_prompt(&opts);


    SystemPrompt::Text(text)
}


/// When the cumulative tool-result content exceeds `budget` characters, walk
/// the message list from oldest to newest and replace individual
/// `ToolResult` content with a placeholder until the running total is back
/// under budget.  Returns the (possibly modified) message list and the
/// number of results that were truncated.
///
/// Mirrors the spirit of the TypeScript `applyToolResultBudget` /
/// `enforceToolResultBudget` logic, simplified to a straightforward
/// oldest-first eviction without the session-persistence layer.
fn apply_tool_result_budget(messages: Vec<Message>, budget: usize) -> (Vec<Message>, usize) {
    let total = total_tool_result_chars(&messages);
    if total <= budget {
        return (messages, 0);
    }

    let mut to_shed = total - budget;
    let mut truncated = 0usize;
    let mut result = messages;

    'outer: for msg in result.iter_mut() {
        if msg.role != boxagnts_core::types::Role::User {
            continue;
        }
        let blocks = match &mut msg.content {
            boxagnts_core::types::MessageContent::Blocks(b) => b,
            _ => continue,
        };
        for block in blocks.iter_mut() {
            if let ContentBlock::ToolResult { content, .. } = block {
                let size = match &*content {
                    ToolResultContent::Text(t) => t.len(),
                    ToolResultContent::Blocks(inner) => inner
                        .iter()
                        .map(|b| {
                            if let ContentBlock::Text { text } = b {
                                text.len()
                            } else {
                                0
                            }
                        })
                        .sum(),
                };
                if size == 0 {
                    continue;
                }
                *content =
                    ToolResultContent::Text("[tool result truncated to save context]".to_string());
                truncated += 1;
                if size > to_shed {
                    break 'outer;
                }
                to_shed -= size;
            }
        }
    }

    (result, truncated)
}

/// Return the combined character count of all tool-result content blocks found
/// in `messages`.  Only user messages are examined (tool results always live
/// in user turns).
fn total_tool_result_chars(messages: &[Message]) -> usize {
    messages
        .iter()
        .filter(|m| m.role == boxagnts_core::types::Role::User)
        .flat_map(|m| match &m.content {
            boxagnts_core::types::MessageContent::Blocks(blocks) => blocks.as_slice(),
            _ => &[],
        })
        .filter_map(|b| {
            if let ContentBlock::ToolResult { content, .. } = b {
                Some(match content {
                    ToolResultContent::Text(t) => t.len(),
                    ToolResultContent::Blocks(blocks) => blocks
                        .iter()
                        .map(|b| {
                            if let ContentBlock::Text { text } = b {
                                text.len()
                            } else {
                                0
                            }
                        })
                        .sum(),
                })
            } else {
                None
            }
        })
        .sum()
}

fn build_provider_options(
    provider_id: &str,
    model_id: &str,
    effort_level: Option<boxagnts_core::effort::EffortLevel>,
    thinking_budget: Option<u32>,
) -> Value {
    let mut options = serde_json::Map::new();
    let model_id = model_id.to_ascii_lowercase();

    if provider_id == "github-copilot" {
        if model_id.contains("claude") {
            options.insert(
                "thinking_budget".to_string(),
                serde_json::json!(thinking_budget.unwrap_or(4_000)),
            );
        } else if model_id.starts_with("gpt-5") && !model_id.contains("gpt-5-pro") {
            let reasoning_effort = effort_level
                .map(reasoning_effort_for_level)
                .unwrap_or("medium");
            options.insert(
                "reasoningEffort".to_string(),
                serde_json::json!(reasoning_effort),
            );
            options.insert("reasoningSummary".to_string(), serde_json::json!("auto"));
            options.insert(
                "include".to_string(),
                serde_json::json!(["reasoning.encrypted_content"]),
            );

            if model_id.contains("gpt-5.")
                && !model_id.contains("codex")
                && !model_id.contains("-chat")
            {
                options.insert("textVerbosity".to_string(), serde_json::json!("low"));
            }
        }
    }

    if provider_id == "google" && model_id.contains("gemini") {
        if model_id.contains("2.5") {
            if let Some(budget) = thinking_budget {
                options.insert(
                    "thinkingConfig".to_string(),
                    serde_json::json!({
                        "includeThoughts": true,
                        "thinkingBudget": budget,
                    }),
                );
            }
        } else if model_id.contains("3.") || model_id.contains("gemini-3") {
            options.insert(
                "thinkingConfig".to_string(),
                serde_json::json!({
                    "includeThoughts": true,
                    "thinkingLevel": google_thinking_level_for_effort(effort_level),
                }),
            );
        }
    }

    if provider_id == "amazon-bedrock" {
        if model_id.contains("anthropic") || model_id.contains("claude") {
            if let Some(budget) = thinking_budget {
                options.insert(
                    "reasoningConfig".to_string(),
                    serde_json::json!({
                        "type": "enabled",
                        "budgetTokens": budget.min(31_999),
                    }),
                );
            }
        } else if let Some(level) = effort_level {
            options.insert(
                "reasoningConfig".to_string(),
                serde_json::json!({
                    "type": "enabled",
                    "maxReasoningEffort": reasoning_effort_for_level(level),
                }),
            );
        }
    }

    if is_openaiish_provider(provider_id) && is_openai_reasoning_model(&model_id) {
        let reasoning_effort = effort_level
            .map(reasoning_effort_for_level)
            .unwrap_or("medium");
        options.insert(
            "reasoningEffort".to_string(),
            serde_json::json!(reasoning_effort),
        );

        if model_id.starts_with("gpt-5")
            && model_id.contains("gpt-5.")
            && !model_id.contains("codex")
            && !model_id.contains("-chat")
            && provider_id != "azure"
        {
            options.insert("textVerbosity".to_string(), serde_json::json!("low"));

            // DeepSeek V4 thinking mode: map effort level to thinking/reasoning_effort params.
            // DeepSeek docs: thinking={"type":"enabled/disabled"}, reasoning_effort="high"|"max"
            // low/medium are mapped to "high" by the API; xhigh mapped to "max".
            if provider_id == "deepseek" {
                match effort_level {
                    None
                    | Some(boxagnts_core::effort::EffortLevel::Medium)
                    | Some(boxagnts_core::effort::EffortLevel::High) => {
                        options.insert(
                            "thinking".to_string(),
                            serde_json::json!({"type": "enabled"}),
                        );
                        options.insert("reasoningEffort".to_string(), serde_json::json!("high"));
                    }
                    Some(boxagnts_core::effort::EffortLevel::Max) => {
                        options.insert(
                            "thinking".to_string(),
                            serde_json::json!({"type": "enabled"}),
                        );
                        options.insert("reasoningEffort".to_string(), serde_json::json!("max"));
                    }
                    Some(boxagnts_core::effort::EffortLevel::Low) => {
                        options.insert(
                            "thinking".to_string(),
                            serde_json::json!({"type": "disabled"}),
                        );
                    }
                }
            }
        }
    }

    if provider_id == "openrouter" {
        options.insert("usage".to_string(), serde_json::json!({ "include": true }));
        if model_id.contains("gemini-3") {
            options.insert(
                "reasoning".to_string(),
                serde_json::json!({ "effort": "high" }),
            );
        }
    }

    if provider_id == "qwen" && thinking_budget.is_some() && !model_id.contains("kimi-k2-thinking")
    {
        options.insert("enable_thinking".to_string(), serde_json::json!(true));
    }

    if (provider_id == "zhipu" || provider_id == "zai") && thinking_budget.is_some() {
        options.insert(
            "thinking".to_string(),
            serde_json::json!({
                "type": "enabled",
                "clear_thinking": false,
            }),
        );
    }

    if options.is_empty() {
        Value::Null
    } else {
        Value::Object(options)
    }
}



fn is_openaiish_provider(provider_id: &str) -> bool {
    matches!(
        provider_id,
        "openai"
            | "azure"
            | "groq"
            | "mistral"
            | "deepseek"
            | "xai"
            | "openrouter"
            | "togetherai"
            | "together-ai"
            | "perplexity"
            | "cerebras"
            | "deepinfra"
            | "venice"
            | "huggingface"
            | "nvidia"
            | "siliconflow"
            | "sambanova"
            | "moonshot"
            | "zhipu"
            | "zai"
            | "qwen"
            | "nebius"
            | "novita"
            | "ovhcloud"
            | "scaleway"
            | "vultr"
            | "vultr-ai"
            | "baseten"
            | "friendli"
            | "upstage"
            | "stepfun"
            | "fireworks"
            | "ollama"
            | "codex"
            | "openai-codex"
            | "lmstudio"
            | "lm-studio"
            | "llamacpp"
            | "llama-cpp"
    )
}

fn is_openai_reasoning_model(model_id: &str) -> bool {
    let model_id = model_id.to_ascii_lowercase();
    model_id.starts_with("gpt-5")
        || model_id.starts_with("o1")
        || model_id.starts_with("o3")
        || model_id.starts_with("o4")
}

fn reasoning_effort_for_level(effort_level: boxagnts_core::effort::EffortLevel) -> &'static str {
    match effort_level {
        boxagnts_core::effort::EffortLevel::Low => "low",
        boxagnts_core::effort::EffortLevel::Medium => "medium",
        boxagnts_core::effort::EffortLevel::High | boxagnts_core::effort::EffortLevel::Max => "high",
    }
}

fn google_thinking_level_for_effort(
    effort_level: Option<boxagnts_core::effort::EffortLevel>,
) -> &'static str {
    match effort_level.unwrap_or(boxagnts_core::effort::EffortLevel::High) {
        boxagnts_core::effort::EffortLevel::Low => "low",
        boxagnts_core::effort::EffortLevel::Medium => "medium",
        boxagnts_core::effort::EffortLevel::High | boxagnts_core::effort::EffortLevel::Max => "high",
    }
}

// ---------------------------------------------------------------------------
// Provider stream event mapping
// ---------------------------------------------------------------------------

/// Map a unified `StreamEvent` (from a non-Anthropic provider) onto the
/// equivalent `AnthropicStreamEvent` so that the TUI stream consumer sees a
/// single, consistent event type regardless of which provider produced it.
fn map_to_anthropic_event(
    evt: &boxagnts_api::StreamEvent,
) -> Option<boxagnts_api::AnthropicStreamEvent> {
    use boxagnts_api::streaming::{AnthropicStreamEvent, ContentDelta};
    use boxagnts_api::StreamEvent;

    match evt {
        StreamEvent::MessageStart { id, model, usage } => {
            Some(AnthropicStreamEvent::MessageStart {
                id: id.clone(),
                model: model.clone(),
                usage: usage.clone(),
            })
        }
        StreamEvent::ContentBlockStart {
            index,
            content_block,
        } => Some(AnthropicStreamEvent::ContentBlockStart {
            index: *index,
            content_block: content_block.clone(),
        }),
        StreamEvent::TextDelta { index, text } => Some(AnthropicStreamEvent::ContentBlockDelta {
            index: *index,
            delta: ContentDelta::TextDelta { text: text.clone() },
        }),
        StreamEvent::ThinkingDelta { index, thinking } => {
            Some(AnthropicStreamEvent::ContentBlockDelta {
                index: *index,
                delta: ContentDelta::ThinkingDelta {
                    thinking: thinking.clone(),
                },
            })
        }
        StreamEvent::ReasoningDelta { index, reasoning } => {
            Some(AnthropicStreamEvent::ContentBlockDelta {
                index: *index,
                delta: ContentDelta::ThinkingDelta {
                    thinking: reasoning.clone(),
                },
            })
        }
        StreamEvent::InputJsonDelta {
            index,
            partial_json,
        } => Some(AnthropicStreamEvent::ContentBlockDelta {
            index: *index,
            delta: ContentDelta::InputJsonDelta {
                partial_json: partial_json.clone(),
            },
        }),
        StreamEvent::SignatureDelta { index, signature } => {
            Some(AnthropicStreamEvent::ContentBlockDelta {
                index: *index,
                delta: ContentDelta::SignatureDelta {
                    signature: signature.clone(),
                },
            })
        }
        StreamEvent::ContentBlockStop { index } => {
            Some(AnthropicStreamEvent::ContentBlockStop { index: *index })
        }
        StreamEvent::MessageDelta { stop_reason, usage } => {
            // Convert the unified StopReason to the string form used by
            // AnthropicStreamEvent::MessageDelta.
            let stop_reason_str = stop_reason.as_ref().map(|r| match r {
                boxagnts_api::provider_types::StopReason::ToolUse => "tool_use".to_string(),
                boxagnts_api::provider_types::StopReason::MaxTokens => "max_tokens".to_string(),
                boxagnts_api::provider_types::StopReason::StopSequence => {
                    "stop_sequence".to_string()
                }
                boxagnts_api::provider_types::StopReason::EndTurn => "end_turn".to_string(),
                boxagnts_api::provider_types::StopReason::ContentFiltered => {
                    "content_filtered".to_string()
                }
                boxagnts_api::provider_types::StopReason::Other(s) => s.clone(),
            });
            Some(AnthropicStreamEvent::MessageDelta {
                stop_reason: stop_reason_str,
                usage: usage.clone(),
            })
        }
        StreamEvent::MessageStop => Some(AnthropicStreamEvent::MessageStop),
        StreamEvent::Error {
            error_type,
            message,
        } => Some(AnthropicStreamEvent::Error {
            error_type: error_type.clone(),
            message: message.clone(),
        }),
    }
}

/// Execute a single tool invocation.
async fn execute_tool(
    name: &str,
    input: &Value,
    tools: &[Box<dyn Tool>],
    ctx: &ToolContext,
) -> ToolResult {
    let tool = tools.iter().find(|t| t.name() == name);

    match tool {
        Some(tool) => {
            debug!(tool = name, "Executing tool");
            tool.execute(input.clone(), ctx).await
        }
        None => {
            warn!(tool = name, "Unknown tool requested");
            ToolResult::error(format!("Unknown tool: {}", name))
        }
    }
}