use axum::{
    extract::{Path, State},
    Json,
};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

use boxagnts_gateway::site::{
    app_state::AppState,
    model::{CreateSiteReq, SiteConfig, UpdateSiteReq, SiteError},
    store,
};

fn error_into_response(err: SiteError) -> Response {
    let (status, msg) = match err {
        SiteError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
        SiteError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
        SiteError::Conflict(msg) => (StatusCode::CONFLICT, msg),
        SiteError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
    };

    (status, Json(json!({ "error": msg }))).into_response()
}

pub async fn list_sites(
    State(state): State<AppState>,
) -> Result<Json<Vec<SiteConfig>>, Response> {
    let sites = state.sites.read().await;
    Ok(Json(sites.values().cloned().collect()))
}

pub async fn get_site(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<SiteConfig>, Response> {
    let sites = state.sites.read().await;
    let job = sites
        .get(&id)
        .cloned()
        .ok_or_else(|| error_into_response(SiteError::NotFound("job not found".to_string())))?;
    Ok(Json(job))
}

pub async fn create_site(
    State(state): State<AppState>,
    Json(req): Json<CreateSiteReq>,
) -> Result<Json<SiteConfig>, Response> {
    {
        let sites = state.sites.read().await;
        if sites.contains_key(&req.id) {
            return Err(error_into_response(SiteError::Conflict("job id already exists".to_string())));
        }
    }

    let site_cfg = SiteConfig {
        id: req.id,
        name: req.name,
        title: req.title,
        description: req.description,
        path: req.path,
        component: req.component,
        entry_point: req.entry_point,
        enable_auth: req.enable_auth,
        auth_user: req.auth_user,
        auth_pass: req.auth_pass,
        enabled: req.enabled,       
    };



    {
        let mut sites = state.sites.write().await;
        sites.insert(site_cfg.id.clone(), site_cfg.clone());

        if let Err(e) = store::save_sites(&sites).await {
            return Err(error_into_response(e));
        }
    }

    Ok(Json(site_cfg))
}

pub async fn update_site(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<UpdateSiteReq>,
) -> Result<Json<SiteConfig>, Response> {
    let old_site = {
        let sites = state.sites.read().await;
        sites.get(&id)
            .cloned()
            .ok_or_else(|| error_into_response(SiteError::NotFound("job not found".to_string())))?
    };

    let mut new_site = old_site.clone();

    if let Some(name) = req.name {
        new_site.name = name;
    }
    if let Some(title) = req.title {
        new_site.title = title;
    }
    if let Some(description) = req.description {
        new_site.description = description;
    }
    if let Some(path) = req.path {
        new_site.path = path;
    }
    if let Some(component) = req.component {
        new_site.component = component;
    }
    new_site.entry_point = req.entry_point;
    new_site.enable_auth = req.enable_auth;
    new_site.auth_user = req.auth_user;
    new_site.auth_pass = req.auth_pass;
    if let Some(enabled) = req.enabled {
        new_site.enabled = enabled;
    }

    {
        let mut sites = state.sites.write().await;
        sites.insert(id.clone(), new_site.clone());

        if let Err(e) = store::save_sites(&sites).await {
            sites.insert(id, old_site);
            return Err(error_into_response(e));
        }
    }

    Ok(Json(new_site))
}

pub async fn delete_site(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, Response> {
    {
        let sites = state.sites.read().await;
        if !sites.contains_key(&id) {
            return Err(error_into_response(SiteError::NotFound("job not found".to_string())));
        }
    }

    {
        let mut sites = state.sites.write().await;
        sites.remove(&id);
        store::save_sites(&sites).await.map_err(
            |e|error_into_response(e)
        )?;
    }


    Ok(Json(json!({
        "message": "deleted"
    })))
}


