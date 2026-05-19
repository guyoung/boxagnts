use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    Router,
    response::Html,
    routing::{ get, any_service },
};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;
use tokio::sync::RwLock;



/// Start web server
pub async fn start_web_server(port: Option<u16>) -> Result<(), Box<dyn std::error::Error>> {
    let port = port.unwrap_or(30001);

    create_web_server(port).await
}


/// Create the web server
async fn create_web_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {

    boxagnts_workspace::config::Settings::init().await?;
    boxagnts_workspace::auth_store::AuthStore::init().await?;


    boxagnts_gateway::cron::store::init_storage().await?;
    boxagnts_gateway::site::store::init_storage().await?;

    let ws_state = crate::dashboard::ws::WSAppState {
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

    // Create router with API endpoints
    let app = Router::new()
        .route("/", get(sites_index))
        .route("/index.html", get(sites_index))
        .nest("/dashboard", crate::dashboard::crate_router(ws_state, cron_state, site_state, config_state ).await)
        .route_service(
            "/sites/{site}/{*path}",
            any_service(crate::sites::make_dynamic_service()),
        )
        .layer(cors)
        ;


    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("🌐 Web server running on http://127.0.0.1:{}", port);
    println!("💻 Dashboard url http://127.0.0.1:{}/dashboard", port);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn sites_index() -> Html<&'static str> {
    Html("Sites")
}

