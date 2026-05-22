use axum::Json;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use boxagnts_gateway::config::app_state::AppState;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ConfigAllowedOutboundHosts {
    pub allowed_outbound_hosts: Vec<String>,
}

pub async fn get_allowed_outbound_hosts(
    State(state): State<AppState>,
) -> Result<Json<ConfigAllowedOutboundHosts>, Response> {
    let res = {
        let settings = state.read().await;

        let allowed_outbound_hosts = settings.config.allowed_outbound_hosts.clone();

        ConfigAllowedOutboundHosts {
            allowed_outbound_hosts,
        }
    };

    Ok(Json(res))
}

pub async fn update_allowed_outbound_hosts(
    State(state): State<AppState>,
    Json(req): Json<ConfigAllowedOutboundHosts>,
) -> Result<Json<Value>, Response> {
    {
        let mut settings = state.write().await;

        settings.config.allowed_outbound_hosts = req.allowed_outbound_hosts.clone();

        if let Err(e) = settings.save().await {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("{}", e) })),
            )
                .into_response());
        }
    };

    Ok(Json(json!({
        "message": "updated"
    })))
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ConfigAgentsMd {
    pub content: String,
}

pub async fn get_agents_md(
    State(state): State<AppState>,
) -> Result<Json<ConfigAgentsMd>, Response> {
    let res = {
        let _settings = state.read().await;

        let store = boxagnts_workspace::agents_md::AgentsMdStore::load()
            .await
            .unwrap_or_default();

        ConfigAgentsMd {
            content: store.content(),
        }
    };

    Ok(Json(res))
}

pub async fn update_agents_md(
    State(state): State<AppState>,
    Json(req): Json<ConfigAgentsMd>,
) -> Result<Json<Value>, Response> {
    {
        let _settings = state.write().await;

        if let Err(e) = boxagnts_workspace::agents_md::AgentsMdStore::save(&req.content).await {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("{}", e) })),
            )
                .into_response());
        }
    };

    Ok(Json(json!({
        "message": "updated"
    })))
}
