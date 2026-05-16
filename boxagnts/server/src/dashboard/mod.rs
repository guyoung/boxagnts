mod chat;
mod cron;
mod file;
pub mod ws;
mod site;
mod skill;
mod tool;

use std::path::PathBuf;

use axum::Router;
use axum::response::Html;
use axum::routing::get;
use axum::routing::post;
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

pub async fn crate_router(
    ws_state: ws::WSAppState,
    cron_state: boxagnts_gateway::cron::app_state::AppState,
    site_state: boxagnts_gateway::site::app_state::AppState,
) -> Router {
    let assets = get_dashboard_dir().await.join("assets");
    let assets = ServeDir::new(assets.clone());

    let cron_router = create_cron_router();
    let site_router = create_site_router();

    // Create router with API endpoints
    let router = Router::new()
        .route("/", get(dashboard))
        .route("/index.html", get(dashboard))
        // WebSocket endpoint for real-time execution
       
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
        .route("/api/files/download", get(file::download))
        .route("/api/files/root_sub_folders", get(file::list_root_sub_folders))
        // Tool
        .route("/api/tools", get(tool::list_tools))
        // Skill
        .route("/api/skills", get(skill::list_skills))
        // cron
        .nest("/api/cron", cron_router)
        .with_state(cron_state)
        //
        .nest("/api/site", site_router)
        .with_state(site_state)
        // ws
        .route("/ws", get(ws::handle_websocket))
        .with_state(ws_state)
        // Serve static assets
        .nest_service("/assets", assets);

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


async fn dashboard() -> Html<String> {
    let text = tokio::fs::read_to_string(get_dashboard_dir().await.join("index.html"))
        .await
        .unwrap_or_else(|_| "File not found".to_string());

    Html(text)
}

async fn get_dashboard_dir() -> PathBuf {
    let web_dir = boxagnts_workspace::path::get_app_dir().await.join("dashboard-web");

    web_dir
}
