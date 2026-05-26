use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    Router,
    routing::{any_service, get},
};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;


/// start_web_server
pub async fn start_web_server(    
    host: String,
    port: u16,
    admin_user: Option<String>,
    admin_pass: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let base_url = format!("http://{}:{}/dashboard", host, port); 
    
    boxagnts_workspace::init(&base_url).await?;

    boxagnts_gateway::cron::store::init_storage().await?;
    boxagnts_gateway::site::store::init_storage().await?;

    let workspace_dir = boxagnts_workspace::path::get_workspace_dir().await;
    let root_dir = workspace_dir.join("root");
    let root_dir = std::fs::canonicalize(root_dir)?;

    if !root_dir.exists() {
        std::fs::create_dir_all(&root_dir).expect("failed to create root directory");
    }

    boxagnts_gateway::api::fs_events::start_watcher(root_dir)
        .await
        .expect("failed to start fs watcher");


    let ws_state = crate::dashboard::chat_ws::ChatWSAppState {
        ws_instances: Arc::new(Mutex::new(HashMap::new())),
        running_queries: Arc::new(Mutex::new(HashMap::new())),
    };

    let settings = boxagnts_workspace::config::Settings::load().await?;
    let config_state = boxagnts_gateway::config::app_state::AppState::new(RwLock::new(settings));

    let jobs = boxagnts_gateway::cron::store::load_jobs().await?;

    let cron_state = boxagnts_gateway::cron::app_state::AppState {
        jobs: Arc::new(RwLock::new(jobs)),
        job_map: Arc::new(RwLock::new(HashMap::new())),
    };

    let sites = boxagnts_gateway::site::store::load_sites().await?;

    let site_state = boxagnts_gateway::site::app_state::AppState {
        sites: Arc::new(RwLock::new(sites)),
        site_map: Arc::new(RwLock::new(HashMap::new())),
    };

    boxagnts_gateway::cron::scheduler::init_scheduler().await?;
    boxagnts_gateway::cron::scheduler::reload_all_jobs(cron_state.clone()).await?;

    // CORS layer to allow requests from phone browsers
    // let cors = CorsLayer::new()
    //     .allow_origin(Any)
    //     .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    //     .allow_headers(Any);
    let cors = CorsLayer::permissive();

    let admin_auth_state = crate::dashboard::AdminAuthState {
        admin_user,
        admin_pass,
    };

    // Create router with API endpoints
    let app = Router::new()
        .route("/", get(crate::sites::sites_home))
        .route("/index.html", get(crate::sites::sites_home))
        .route(
            "/api/get_site_nav_items",
            get(crate::sites::get_site_nav_items),
        )
        .nest(
            "/dashboard",
            crate::dashboard::crate_router(ws_state, cron_state, site_state, config_state, admin_auth_state).await,
        )
        .route_service(
            "/sites/{site}/{*path}",
            any_service(crate::sites::make_dynamic_service()),
        )
        .layer(cors);

    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
    println!("🌐 Web server running on http://{}:{}", host, port);
    println!("💻 Dashboard url http://{}:{}/dashboard", host, port);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
