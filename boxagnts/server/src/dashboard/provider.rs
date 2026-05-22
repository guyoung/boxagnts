use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{
    Json,
    extract::{Path, State},
};
use serde_json::{Value, json};
use std::collections::HashMap;

use boxagnts_gateway::config::app_state::AppState;
use boxagnts_gateway::config::model::CreateModelReq;
use boxagnts_gateway::config::model::CreateProviderReq;
use boxagnts_gateway::config::model::UpdateDefaultModelReq;
use boxagnts_gateway::config::model::UpdateModelReq;
use boxagnts_gateway::config::model::UpdateProviderReq;
use boxagnts_workspace::config::ModelConfig;
use boxagnts_workspace::config::ProviderConfig;

pub async fn list_providers(
    State(state): State<AppState>,
) -> Result<Json<Vec<ProviderConfig>>, Response> {
    let settings = state.read().await;

    let providers = settings.config.provider_configs.values().cloned().collect();

    Ok(Json(providers))
}

pub async fn get_provider(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ProviderConfig>, Response> {
    let settings = state.read().await;

    if let Some(provider) = settings.config.provider_configs.get(&id) {
        Ok(Json(provider.clone()))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "provider not found" })),
        )
            .into_response())
    }
}

pub async fn create_provider(
    State(state): State<AppState>,
    Json(req): Json<CreateProviderReq>,
) -> Result<Json<ProviderConfig>, Response> {
    {
        let settings = state.read().await;

        if settings.config.provider_configs.contains_key(&req.id) {
            return Err((
                StatusCode::CONFLICT,
                Json(json!({ "error": "provider id already exists" })),
            )
                .into_response());
        }
    }

    let provider_cfg = ProviderConfig {
        id: req.id,
        name: req.name,
        api_base: req.api_base,
        enabled: req.enabled,
        models: Vec::new(),
        options: HashMap::new(),
    };
    {
        let mut settings = state.write().await;

        settings
            .config
            .provider_configs
            .insert(provider_cfg.id.clone(), provider_cfg.clone());

        if let Err(e) = settings.save().await {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("{}", e) })),
            )
                .into_response());
        }

        if let Some(api_key) = req.api_key {
            if let Err(e) = set_api_key(&provider_cfg.id, &api_key).await {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": format!("{}", e) })),
                )
                    .into_response());
            }
        }
    }

    Ok(Json(provider_cfg))
}

pub async fn update_provider(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<UpdateProviderReq>,
) -> Result<Json<ProviderConfig>, Response> {
    let old_provider = {
        let settings = state.read().await;
        settings
            .config
            .provider_configs
            .get(&id)
            .cloned()
            .ok_or_else(|| {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({ "error": "provider not found" })),
                )
                    .into_response()
            })?
    };

    let mut new_provider = old_provider.clone();

    if let Some(name) = req.name {
        new_provider.name = name;
    }
    new_provider.api_base = req.api_base;

    if let Some(enabled) = req.enabled {
        new_provider.enabled = enabled;
    }

    {
        let mut settings = state.write().await;
        settings
            .config
            .provider_configs
            .insert(id.clone(), new_provider.clone());

        if let Err(e) = settings.save().await {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("{}", e) })),
            )
                .into_response());
        }

        if let Some(api_key) = req.api_key {
            if let Err(e) = set_api_key(&id, &api_key).await {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": format!("{}", e) })),
                )
                    .into_response());
            }
        }
    }

    Ok(Json(new_provider))
}

pub async fn delete_provider(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Value>, Response> {
    {
        let settings = state.read().await;

        if !settings.config.provider_configs.contains_key(&id) {
            return Err((
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "provider not found" })),
            )
                .into_response());
        }
    }

    {
        let mut settings = state.write().await;
        settings.config.provider_configs.remove(&id);

        if let Err(e) = settings.save().await {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("{}", e) })),
            )
                .into_response());
        }

        if let Err(e) = remove_api_key(&id).await {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("{}", e) })),
            )
                .into_response());
        }
    }

    Ok(Json(json!({
        "message": "deleted"
    })))
}

pub async fn list_provider_options(
    State(_state): State<AppState>,
) -> Result<Json<Vec<Value>>, Response> {
    let provider_options = boxagnts_gateway::config::provider_util::get_provider_options();

    Ok(Json(provider_options))
}

pub async fn create_provider_model(
    State(state): State<AppState>,
    Path(provider_id): Path<String>,
    Json(req): Json<CreateModelReq>,
) -> Result<Json<ModelConfig>, Response> {
    {
        let settings = state.read().await;

        if !settings.config.provider_configs.contains_key(&provider_id) {
            return Err((
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "provider not found" })),
            )
                .into_response());
        }

        let provider = settings
            .config
            .provider_configs
            .get(&provider_id)
            .clone()
            .unwrap();
        let models = provider.models.clone();

        if let Some(_m) = models.iter().find(|m| m.id == req.id) {
            return Err((
                StatusCode::CONFLICT,
                Json(json!({ "error": "provider id already exists" })),
            )
                .into_response());
        }
    }

    let model_cfg = ModelConfig {
        id: req.id,
        name: req.name,
        context_window: 4096,
        max_tokens: 2048,
        temperature: 0.7,
    };
    {
        let mut settings = state.write().await;

        let mut new_provider = settings
            .config
            .provider_configs
            .get(&provider_id)
            .unwrap()
            .clone();
        new_provider.models.push(model_cfg.clone());

        settings
            .config
            .provider_configs
            .insert(provider_id.clone(), new_provider.clone());

        if let Err(e) = settings.save().await {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("{}", e) })),
            )
                .into_response());
        }
    }

    Ok(Json(model_cfg))
}

pub async fn update_provider_model(
    State(state): State<AppState>,
    Path((provider_id, model_id)): Path<(String, String)>,
    Json(req): Json<UpdateModelReq>,
) -> Result<Json<ModelConfig>, Response> {
    let old_model = {
        let settings = state.read().await;

        if !settings.config.provider_configs.contains_key(&provider_id) {
            return Err((
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "provider not found" })),
            )
                .into_response());
        }

        let provider = settings
            .config
            .provider_configs
            .get(&provider_id)
            .clone()
            .unwrap();
        let models = provider.models.clone();

        if let Some(m) = models.iter().find(|m| m.id == model_id.clone()) {
            m.clone()
        } else {
            return Err((
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "model not found" })),
            )
                .into_response());
        }
    };

    let mut new_model = old_model.clone();

    if let Some(name) = req.name {
        new_model.name = name;
    }

    {
        let mut settings = state.write().await;

        let mut new_provider = settings
            .config
            .provider_configs
            .get(&provider_id)
            .unwrap()
            .clone();
        for model in new_provider.models.iter_mut() {
            *model = new_model.clone();
        }

        settings
            .config
            .provider_configs
            .insert(provider_id.clone(), new_provider.clone());

        if let Err(e) = settings.save().await {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("{}", e) })),
            )
                .into_response());
        }
    }

    Ok(Json(new_model))
}

pub async fn delete_provider_model(
    State(state): State<AppState>,
    Path((provider_id, model_id)): Path<(String, String)>,
) -> Result<Json<Value>, Response> {
    {
        let settings = state.read().await;

        if !settings.config.provider_configs.contains_key(&provider_id) {
            return Err((
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "provider not found" })),
            )
                .into_response());
        }

        let provider = settings
            .config
            .provider_configs
            .get(&provider_id)
            .clone()
            .unwrap();
        let models = provider.models.clone();

        if let Some(m) = models.iter().find(|m| m.id == model_id.clone()) {
            m.clone()
        } else {
            return Err((
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "model not found" })),
            )
                .into_response());
        }
    };

    {
        let mut settings = state.write().await;

        let mut new_provider = settings
            .config
            .provider_configs
            .get(&provider_id)
            .unwrap()
            .clone();
        let mut models = new_provider.models.clone();
        models.retain(|m| m.id != model_id);
        new_provider.models = models;

        settings
            .config
            .provider_configs
            .insert(provider_id.clone(), new_provider.clone());

        if let Some(config_model) = settings.config.model.as_deref() {
            if config_model == model_id.as_str() {
                settings.config.model = None;
            }
        }

        if let Err(e) = settings.save().await {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("{}", e) })),
            )
                .into_response());
        }
    }

    Ok(Json(json!({
        "message": "deleted"
    })))
}

pub async fn update_default_model(
    State(state): State<AppState>,
    Json(req): Json<UpdateDefaultModelReq>,
) -> Result<Json<Value>, Response> {
    {
        let mut settings = state.write().await;

        settings.config.model = Some(req.id.clone());

        if let Err(e) = settings.save().await {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("{}", e) })),
            )
                .into_response());
        }
    }

    Ok(Json(json!({
        "message": "updated"
    })))
}

pub async fn get_models(State(state): State<AppState>) -> Result<Json<Vec<Value>>, Response> {
    let models = {
        let mut models: Vec<Value> = Vec::new();

        let settings = state.read().await;

        for provider in settings.config.provider_configs.values() {
            if provider.enabled {
                for model in provider.models.iter() {
                    models.push(model.id.clone().into())
                }
            }
        }

        models
    };

    Ok(Json(models))
}

async fn set_api_key(provider_id: &str, api_key: &str) -> anyhow::Result<()> {
    let mut auth_store = boxagnts_workspace::auth_store::AuthStore::load().await?;

    let cred = boxagnts_workspace::auth_store::StoredCredential::ApiKey {
        key: api_key.to_string(),
    };

    auth_store.set(provider_id, cred).await?;

    Ok(())
}

async fn remove_api_key(provider_id: &str) -> anyhow::Result<()> {
    let mut auth_store = boxagnts_workspace::auth_store::AuthStore::load().await?;

    auth_store.remove(provider_id).await?;

    Ok(())
}
