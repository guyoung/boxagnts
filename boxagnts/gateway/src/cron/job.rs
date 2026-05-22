use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context, anyhow};
use serde_json::Value;
use tokio::sync::{mpsc, oneshot};
use tokio_util::sync::CancellationToken;
//use tracing::debug;

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
    #[allow(dead_code)]
    pub fn cancel(&self) {
        self.cancel.cancel();
    }

    pub fn cancellation_token(&self) -> CancellationToken {
        self.cancel.clone()
    }

    pub async fn await_result(self) -> anyhow::Result<Value> {
        match self.result_rx.await {
            Ok(res) => res,
            Err(_) => Err(anyhow::anyhow!("Job query task result channel closed")),
        }
    }
}

pub async fn execute(prompt: String, model: Option<String>) -> anyhow::Result<QuerySessionHandle> {
    let settings = Settings::load().await?;
    let mut config = settings.config.clone();

    let available_models = {
        let mut models: Vec<String> = Vec::new();
        for provider in settings.config.provider_configs.values() {
            if provider.enabled {
                for model in provider.models.iter() {
                    models.push(model.id.clone())
                }
            }
        }

        models
    };

    if let Some(ref model) = model {
        if available_models.contains(model) {
            config.model = Some(model.clone());
        }
    }


    if config.model.is_none() && available_models.len() > 0{
        config.model = Some(available_models[0].clone());
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

    let provider_registry = boxagnts_api::ProviderRegistry::from_config(&config, client_config)
        .await
        .map_err(|e| anyhow!("{:?}", e))?;

    let cost_tracker = CostTracker::new();

    let current_turn = Arc::new(std::sync::atomic::AtomicUsize::new(0));

    let mcp_manager_arc = connect_mcp_manager_arc(&config).await;

    let tool_ctx = ToolContext {
        permission_mode: config.permission_mode.clone(),
        cost_tracker: cost_tracker.clone(),
        session_id: None,
        current_turn: current_turn.clone(),
        non_interactive: true,
        mcp_manager: mcp_manager_arc.clone(),
        config: config.clone(),
        managed_agent_config: config.managed_agents.clone(),
        allowed_outbound_hosts: config.allowed_outbound_hosts.clone(),
    };

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
            prompt,
            cancel_for_inner,
        )
        .await;

        let _ = result_tx.send(result);
    });

    Ok(QuerySessionHandle { cancel, result_rx })
}

async fn execute_inner(
    _config: Config,
    _settings: Settings,
    client: Arc<boxagnts_api::AnthropicClient>,
    tools: Arc<Vec<Box<dyn boxagnts_tools::Tool>>>,
    tool_ctx: ToolContext,
    query_config: boxagnts_query::QueryConfig,
    cost_tracker: Arc<CostTracker>,
    prompt: String,
    cancel: CancellationToken,
) -> anyhow::Result<Value> {
    let messages = vec![boxagnts_core::types::Message::user(prompt)];

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

    while let Some(_event) = event_rx.recv().await {}

    let messages = msgs_arc.lock().await.clone();

    let outcome = query_handle.await.unwrap_or(QueryOutcome::Error(
        boxagnts_core::error::ClaudeError::Other("Job query task panicked".to_string()),
    ));

    let val = match outcome {
        QueryOutcome::EndTurn { usage, .. } => {
            serde_json::json!({
                "type": "finished",
                "messages": messages,
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
                "messages": messages,
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
                "messages": messages,
            })
        }
        QueryOutcome::Error(e) => {
            serde_json::json!({
                "type": "error",
                "messages": messages,
                "error": e.to_string()
            })
        }
        QueryOutcome::BudgetExceeded {
            cost_usd,
            limit_usd,
        } => {
            serde_json::json!({
                "type": "exceeded",
                "messages": messages,
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
