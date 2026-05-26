mod chat;
mod cron;
mod file;
pub mod chat_ws;
mod site;
mod skill;
mod tool;
mod provider;
mod config;
mod file_ws;

use std::path::PathBuf;
use axum::body::Body;
use axum::extract::{Request, State};
use axum::middleware::{self, Next};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use axum::Router;
use base64::Engine;
use base64::engine::general_purpose;
use http::{header, StatusCode};
use serde::Serialize;
use tower_http::services::ServeDir;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

#[derive(Clone)]
pub struct AdminAuthState {
    pub admin_user: Option<String>,
    pub admin_pass: Option<String>,
}

pub async fn crate_router(
    chat_ws_state: chat_ws::ChatWSAppState,
    cron_state: boxagnts_gateway::cron::app_state::AppState,
    site_state: boxagnts_gateway::site::app_state::AppState,
    config_state: boxagnts_gateway::config::app_state::AppState,
    admin_auth_state: AdminAuthState,
) -> Router {
    let assets = get_dashboard_dir().await.join("assets");
    let assets = ServeDir::new(assets.clone());

    let cron_router = create_cron_router();
    let site_router = create_site_router();
    let config_router = create_config_router();

    let router = Router::new()
        .route("/", get(dashboard))
        .route("/index.html", get(dashboard))
        // Project
        .route("/api/project", get(chat::get_current_project))
        // Session
        .route("/api/sessions", get(chat::get_sessions))
        .route(
            "/api/sessions/{session_id}",
            get(chat::load_session_history),
        )
        .route(
            "/api/delete_session/{session_id}",
            get(chat::delete_session),
        )
        .route(
            "/api/update_session_title/{session_id}",
            post(chat::update_session_title),
        )
        .route(
            "/api/delete_session_messages/{session_id}",
            post(chat::delete_session_messages),
        )
        .route(
            "/api/clear_session_messages/{session_id}",
            get(chat::clear_session_messages),
        )
        // File
        .route("/api/files", get(file::list_files))
        .route("/api/files/mkdir", post(file::mkdir))
        .route("/api/files/upload", post(file::upload))
        .route("/api/files/delete", post(file::delete))
        .route("/api/files/rename", post(file::rename))
        .route("/api/files/copy", post(file::copy))
        .route("/api/files/move", post(file::move_item))
        .route("/api/files/download", get(file::download))
        .route("/api/files/root_sub_folders", get(file::list_root_sub_folders))
        // Tool
        .route("/api/tools", get(tool::list_tools))
        // Skill
        .route("/api/skills", get(skill::list_skills))
        // cron
        .nest("/api/cron", cron_router)
        .with_state(cron_state)
        // site
        .nest("/api/site", site_router)
        .with_state(site_state)
        // config
        .nest("/api/config", config_router)
        .with_state(config_state)
        // ws
        .route("/chat_ws", get(chat_ws::handle_websocket))
        .with_state(chat_ws_state)
        .route("/file_ws", get(file_ws::handle_websocket))
        // static
        .nest_service("/assets", assets)
        // Basic Auth only protects /dashboard/*
        .layer(middleware::from_fn_with_state(
           admin_auth_state,
           basic_auth,
        ))
        ;

    router
}

fn create_cron_router() -> Router<boxagnts_gateway::cron::app_state::AppState> {
    Router::new()
        .route("/jobs", get(cron::list_jobs))
        .route("/jobs/{id}", get(cron::get_job))
        .route("/jobs/create_job", post(cron::create_job))
        .route("/jobs/update_job/{id}", post(cron::update_job))
        .route("/jobs/delete_job/{id}", post(cron::delete_job))
        .route("/jobs/{id}/logs", get(cron::list_job_logs))
}

fn create_site_router() -> Router<boxagnts_gateway::site::app_state::AppState> {
    Router::new()
        .route("/sites", get(site::list_sites))
        .route("/sites/{id}", get(site::get_site))
        .route("/sites/create_site", post(site::create_site))
        .route("/sites/update_site/{id}", post(site::update_site))
        .route("/sites/delete_site/{id}", post(site::delete_site))
}

fn create_config_router() -> Router<boxagnts_gateway::config::app_state::AppState> {
    Router::new()
        .route("/providers", get(provider::list_providers))
        .route("/providers/{id}", get(provider::get_provider))
        .route("/providers/create_provider", post(provider::create_provider))
        .route("/providers/update_provider/{id}", post(provider::update_provider))
        .route("/providers/delete_provider/{id}", post(provider::delete_provider))
        .route("/provider_options", get(provider::list_provider_options))
        .route("/providers/{provider_id}/create_model", post(provider::create_provider_model))
        .route("/providers/{provider_id}/update_model/{model_id}", post(provider::update_provider_model))
        .route("/providers/{provider_id}/delete_model/{model_id}", post(provider::delete_provider_model))
        .route("/update_default_model", post(provider::update_default_model))
        .route("/get_models", get(provider::get_models))
        .route("/get_allowed_outbound_hosts", get(config::get_allowed_outbound_hosts))
        .route("/update_allowed_outbound_hosts", post(config::update_allowed_outbound_hosts))
        .route("/get_agents_md", get(config::get_agents_md))
        .route("/update_agents_md", post(config::update_agents_md))
}

async fn dashboard() -> Html<String> {
    let text = tokio::fs::read_to_string(get_dashboard_dir().await.join("index.html"))
        .await
        .unwrap_or_else(|_| "File not found".to_string());

    Html(text)
}

async fn get_dashboard_dir() -> PathBuf {
    boxagnts_workspace::path::get_app_dir().await.join("dashboard-web")
}

fn unauthorized_response() -> Response {
    (
        StatusCode::UNAUTHORIZED,
        [(header::WWW_AUTHENTICATE, r#"Basic realm="dashboard""#)],
        "Unauthorized",
    )
        .into_response()
}

async fn basic_auth(
    State(state): State<AdminAuthState>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, Response> {
    if state.admin_user.is_none() || state.admin_pass.is_none() {
        return Ok(next.run(req).await)
    }

    let expected_user = &state.admin_user.unwrap_or_default();
    let expected_pass = &state.admin_pass.unwrap_or_default();

    let auth = match req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
    {
        Some(v) => v,
        None => return Err(unauthorized_response()),
    };

    let encoded = match auth.strip_prefix("Basic ") {
        Some(v) => v,
        None => return Err(unauthorized_response()),
    };

    let decoded = match general_purpose::STANDARD.decode(encoded) {
        Ok(v) => v,
        Err(_) => return Err(unauthorized_response()),
    };

    let decoded = match String::from_utf8(decoded) {
        Ok(v) => v,
        Err(_) => return Err(unauthorized_response()),
    };

    let (user, pass) = match decoded.split_once(':') {
        Some(v) => v,
        None => return Err(unauthorized_response()),
    };

    if user == expected_user && pass == expected_pass {
        Ok(next.run(req).await)
    } else {
        Err(unauthorized_response())
    }
}
