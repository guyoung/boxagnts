use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Context;
use futures_util::future::BoxFuture;
use serde_json::Value;
use tokio::sync::{mpsc, oneshot};
use tokio_util::sync::CancellationToken;
use tracing::debug;
use uuid::Uuid;

use boxagnts_core::cost::CostTracker;
use boxagnts_query::{QueryEvent, QueryOutcome};
use boxagnts_tools::ToolContext;
use boxagnts_workspace::config::{Config, PermissionMode, Settings};

use crate::api::mcp::connect_mcp_manager_arc;
use crate::api::tool::build_tools_with_mcp;

pub struct QuerySessionHandle {
    cancel: CancellationToken,
    result_rx: oneshot::Receiver<anyhow::Result<Value>>,
}

impl QuerySessionHandle {
    pub fn cancel(&self) {
        self.cancel.cancel();
    }

    pub fn cancellation_token(&self) -> CancellationToken {
        self.cancel.clone()
    }

    pub async fn await_result(self) -> anyhow::Result<Value> {
        match self.result_rx.await {
            Ok(res) => res,
            Err(_) => Err(anyhow::anyhow!("Chat query task result channel closed")),
        }
    }
}

pub async fn execute<OutCb, ErrCb>(
    prompt: String,
    model: Option<String>,
    session_id: Option<String>,
    out_callback_fn: OutCb,
    err_callback_fn: ErrCb,
) -> anyhow::Result<QuerySessionHandle>
where
    OutCb: Fn(Value) -> BoxFuture<'static, ()> + Send + Sync + 'static,
    ErrCb: Fn(Value) -> BoxFuture<'static, ()> + Send + Sync + 'static,
{

    let settings = Settings::load().await?;
    let mut config = settings.effective_config();

    if let Some(ref model) = model {
        config.model = Some(model.clone());
    }
    config.verbose = false;
    config.output_format = boxagnts_workspace::config::OutputFormat::StreamJson;
    config.disable_claude_mds = false;
    config.permission_mode = PermissionMode::BypassPermissions;
    config.additional_dirs = vec![];
    config.auto_compact = true;
    config.project_dir = Some(PathBuf::from("/"));

    let ctx_builder = boxagnts_workspace::context::ContextBuilder::new(PathBuf::from("/"))
        .disable_claude_mds(config.disable_claude_mds);
    let system_ctx = ctx_builder.build_system_context().await;
    let user_ctx = ctx_builder.build_user_context().await;

    let mut system_parts = vec![
        include_str!("../system_prompt.txt").to_string(),
        system_ctx,
        user_ctx,
    ];
    if let Some(ref custom) = config.custom_system_prompt {
        system_parts[0] = custom.clone();
    }
    if let Some(ref append) = config.append_system_prompt {
        system_parts.push(append.clone());
    }
    let system_prompt = system_parts.join("\n\n");

    let active_provider = config.selected_provider_id();
    debug!("active_provider = {}", active_provider);

    config.provider = Some(active_provider.to_string());


    /***
    let (api_key, use_bearer_auth) = if active_provider == "anthropic" {
        match config.resolve_anthropic_auth_async().await {
            Some(auth) => auth,
            None => {
                anyhow::bail!("No API key found. Options:");
            }
        }
    } else {
        (String::new(), false)
    };
    ***/

    let (api_key, use_bearer_auth) = (String::new(), false);

    let client_config = boxagnts_api::client::ClientConfig {
        api_key: api_key.clone(),
        api_base: config.resolve_anthropic_api_base(),
        use_bearer_auth,
        ..Default::default()
    };

    let client = Arc::new(
        boxagnts_api::AnthropicClient::new(client_config.clone())
            .context("Failed to create API client")?,
    );

    let provider_registry =
        boxagnts_api::ProviderRegistry::from_config(&config, client_config).await;

    /***
    let permission_manager = Arc::new(std::sync::Mutex::new(
        boxagnts_workspace::permissions::PermissionManager::new(
            config.permission_mode.clone(),
            &settings,
        ),
    ));

    let permission_handler: Arc<dyn boxagnts_workspace::permissions::PermissionHandler> =
        Arc::new(AutoPermissionHandler {
            mode: config.permission_mode.clone(),
        });

     ***/

    let cost_tracker = CostTracker::new();

    let session_id = session_id
        .clone()
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    /***
    let file_history = Arc::new(ParkingMutex::new(
        boxagnts_core::file_history::FileHistory::new(),
    ));
    ***/
    let current_turn = Arc::new(std::sync::atomic::AtomicUsize::new(0));

    let mcp_manager_arc = connect_mcp_manager_arc(&config).await;

    /***
    let pending_permissions = Arc::new(ParkingMutex::new(
        boxagnts_tools::PendingPermissionStore::default(),
    ));
    ***/

    let tool_ctx = ToolContext {
        // cwd: cwd.clone(),
        permission_mode: config.permission_mode.clone(),
        // permission_handler: permission_handler.clone(),
        cost_tracker: cost_tracker.clone(),
        session_id: Some(session_id.clone()),
        /***
        file_history: file_history.clone(),
        ***/
        current_turn: current_turn.clone(),
        non_interactive: true,
        mcp_manager: mcp_manager_arc.clone(),
        config: config.clone(),
        managed_agent_config: config.managed_agents.clone(),
        // completion_notifier: None,
        // pending_permissions: Some(pending_permissions.clone()),
        // permission_manager: Some(permission_manager.clone()),
        allowed_outbound_hosts: config.allowed_outbound_hosts.clone(),
    };

    /***
    // Register the cc-query-backed agent runner so TeamCreateTool can spawn real
    // sub-agents.  Must be called before any tool execution begins.
    // The function is idempotent if already registered (panics only on double-call,
    // but we guard with a std::sync::OnceLock internally).
    {
        static SWARM_INIT: std::sync::OnceLock<()> = std::sync::OnceLock::new();
        SWARM_INIT.get_or_init(|| boxagnts_query::init_team_swarm_runner());
    }
    ***/

    // Build the full tool list: built-ins from cc-tools plus AgentTool from cc-query
    // (AgentTool lives in cc-query to avoid a circular cc-tools ↔ cc-query dependency).
    // Wrap in Arc so the list can be shared by the main loop AND the cron scheduler.
    let tools = build_tools_with_mcp(mcp_manager_arc.clone());

    // Build model registry for dynamic model/provider resolution.
    // The registry is pre-populated with a hardcoded snapshot and enriched
    // from the models.dev cache if available.
    let model_registry = load_cached_model_registry();

    let mut query_config =
        boxagnts_query::QueryConfig::from_config_with_registry(&config, &model_registry);
    query_config.model_registry = Some(model_registry.clone());
    query_config.max_turns = 5;
    query_config.system_prompt = Some(system_prompt);
    query_config.append_system_prompt = None;
    query_config.working_directory = Some("/".to_string());

    // Wire in the provider registry so non-Anthropic providers can be dispatched.
    let provider_registry = Arc::new(provider_registry);
    query_config.provider_registry = Some(provider_registry.clone());

    let cancel = CancellationToken::new();
    let cancel_for_inner = cancel.clone();

    let (result_tx, result_rx) = oneshot::channel();

    tokio::spawn(async move {
        let result = execute_inner(
            config,
            settings,
            client,
            tools,
            tool_ctx,
            query_config,
            cost_tracker,
            session_id,
            model_registry,
            prompt,
            cancel_for_inner,
            out_callback_fn,
            err_callback_fn,
        )
        .await;

        let _ = result_tx.send(result);
    });

    Ok(QuerySessionHandle { cancel, result_rx })
}

async fn execute_inner<OutCb, ErrCb>(
    config: Config,
    _settings: boxagnts_workspace::config::Settings,
    client: Arc<boxagnts_api::AnthropicClient>,
    tools: Arc<Vec<Box<dyn boxagnts_tools::Tool>>>,
    tool_ctx: ToolContext,
    query_config: boxagnts_query::QueryConfig,
    cost_tracker: Arc<CostTracker>,
    session_id: String,
    model_registry: Arc<boxagnts_api::ModelRegistry>,
    prompt: String,
    cancel: CancellationToken,
    out_callback_fn: OutCb,
    err_callback_fn: ErrCb,
) -> anyhow::Result<Value>
where
    OutCb: Fn(Value) -> BoxFuture<'static, ()> + Send + Sync + 'static,
    ErrCb: Fn(Value) -> BoxFuture<'static, ()> + Send + Sync + 'static,
{
    let mut session = match boxagnts_workspace::history::load_session(&session_id).await {
        Ok(session) => session,
        Err(_e) => {
            let mut session = boxagnts_workspace::history::ConversationSession::new(
                boxagnts_api::effective_model_for_config(&config, &model_registry),
            );
            session.id = session_id.clone();
            session
        }
    };

    let mut messages = session.messages;
    let mut message = boxagnts_core::types::Message::user(prompt);
    let message_uuid = Some(Uuid::new_v4().simple().to_string());
    message.uuid = message_uuid.clone();
    messages.push(message);

    let (event_tx, mut event_rx) = mpsc::unbounded_channel::<QueryEvent>();
    let client_clone = client.clone();
    let tool_ctx_clone = tool_ctx.clone();
    let qcfg = query_config.clone();
    let tracker_clone = cost_tracker.clone();
    let event_tx_clone = event_tx.clone();
    let cancel_clone = cancel.clone();

    let msgs_arc = Arc::new(tokio::sync::Mutex::new(messages.clone()));
    let msgs_arc_clone = msgs_arc.clone();

    let query_handle = tokio::spawn(async move {
        let mut msgs = msgs_arc_clone.lock().await.clone();
        let outcome = boxagnts_query::run_query_loop(
            client_clone.as_ref(),
            &mut msgs,
            tools.as_slice(),
            &tool_ctx_clone,
            &qcfg,
            tracker_clone,
            Some(event_tx_clone),
            cancel_clone,
            None,
        )
        .await;

        *msgs_arc_clone.lock().await = msgs;
        outcome
    });

    // Drop the original tx so the channel closes when the task drops its clone
    drop(event_tx);

    let mut _full_text = String::new();

    let current_message_id: Option<String> = None;
    let current_message_id = Arc::new(tokio::sync::Mutex::new(current_message_id));

    while let Some(event) = event_rx.recv().await {
        let current_message_id = current_message_id.clone();

        match &event {
            QueryEvent::Stream(boxagnts_api::AnthropicStreamEvent::MessageStart { id, .. }) => {
                *current_message_id.lock().await = Some(id.to_string());
            }

            QueryEvent::Stream(boxagnts_api::AnthropicStreamEvent::ContentBlockDelta {
                delta: boxagnts_api::streaming::ContentDelta::TextDelta { text },
                ..
            }) => {
                _full_text.push_str(text);

                let chunk = serde_json::json!({
                    "type": "text_delta",
                    "text": text,
                    "uuid": current_message_id.lock().await.clone().unwrap_or_default(),
                });
                out_callback_fn(chunk).await;
            }

            QueryEvent::ToolStart {
                tool_name,
                tool_id: _tool_id,
                input_json,
            } => {
                let ev = serde_json::json!({
                    "type": "tool_start",
                    "tool": tool_name,
                    "input_json": input_json,
                    "uuid": current_message_id.lock().await.clone().unwrap_or_default(),
                });
                out_callback_fn(ev).await;
            }

            QueryEvent::ToolEnd {
                tool_name,
                tool_id: _tool_id,
                result: _result,
                is_error,
            } => {
                let ev = serde_json::json!({
                    "type": "tool_end",
                    "tool": tool_name,
                    "is_error": is_error,
                    "uuid": current_message_id.lock().await.clone().unwrap_or_default(),
                });
                out_callback_fn(ev).await;
            }

            QueryEvent::Error(msg) => {
                let ev = serde_json::json!({
                    "type": "error",
                    "error": msg
                });
                err_callback_fn(ev).await;
            }

            _ => {}
        }
    }

    let outcome = query_handle.await.unwrap_or(QueryOutcome::Error(
        boxagnts_core::error::ClaudeError::Other("Chat query task panicked".to_string()),
    ));

    let messages = msgs_arc.lock().await.clone();

    session.messages = messages.clone();
    session.updated_at = chrono::Utc::now();

    let _ = boxagnts_workspace::history::save_session(&session).await;

    let val = match outcome {
        QueryOutcome::EndTurn { usage, .. } => {
            serde_json::json!({
                "type": "finished",
                "user_message_uuid": message_uuid.clone(),
                "usage": {
                    "input_tokens": usage.input_tokens,
                    "output_tokens": usage.output_tokens,
                },
                "cost_usd": cost_tracker.total_cost_usd(),
            })
        }
        QueryOutcome::MaxTokens { usage, .. } => {
            serde_json::json!({
                "type": "max_tokens",
                "user_message_uuid": message_uuid.clone(),
                "usage": {
                    "input_tokens": usage.input_tokens,
                    "output_tokens": usage.output_tokens,
                },
                "cost_usd": cost_tracker.total_cost_usd(),
            })
        }
        QueryOutcome::Cancelled => {
            serde_json::json!({
                "type": "cancelled",
                "user_message_uuid": message_uuid.clone(),
            })
        }
        QueryOutcome::Error(e) => {
            serde_json::json!({
                "type": "error",
                "user_message_uuid": message_uuid.clone(),
                "error": e.to_string()
            })
        }
        QueryOutcome::BudgetExceeded {
            cost_usd,
            limit_usd,
        } => {
            serde_json::json!({
                "type": "exceeded",
                "user_message_uuid": message_uuid.clone(),
                "cost_usd": cost_usd,
                "limit_usd": limit_usd
            })
        }
    };

    Ok(val)
}

fn load_cached_model_registry() -> Arc<boxagnts_api::ModelRegistry> {
    let mut reg = boxagnts_api::ModelRegistry::new();

    reg.load_cache();

    Arc::new(reg)
}
