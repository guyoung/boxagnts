use axum::Json;
use axum::extract::Path;
use serde_json::Value;

use super::ApiResponse;

pub async fn get_current_project() -> Json<ApiResponse<boxagnts_gateway::api::project::Project>> {
    match boxagnts_gateway::api::project::get_current_project().await {
        Ok(project) => Json(ApiResponse::success(project)),
        Err(e) => Json(ApiResponse::error(e.to_string())),
    }
}

pub async fn get_sessions() -> Json<ApiResponse<Vec<boxagnts_gateway::api::chat_session::Session>>> {
    match boxagnts_gateway::api::chat_session::get_sessions().await {
        Ok(sessions) => Json(ApiResponse::success(sessions)),
        Err(e) => Json(ApiResponse::error(e.to_string())),
    }
}

pub async fn load_session_history(Path(session_id): Path<String>) -> Json<ApiResponse<Value>> {
    match boxagnts_gateway::api::chat_session::load_session_history(session_id).await {
        Ok(history) => Json(ApiResponse::success(history)),
        Err(e) => Json(ApiResponse::error(e.to_string())),
    }
}

pub async fn delete_session(Path(session_id): Path<String>) -> Json<ApiResponse<()>> {
    match boxagnts_gateway::api::chat_session::delete_session(session_id).await {
        Ok(_) => Json(ApiResponse::success(())),
        Err(e) => Json(ApiResponse::error(e.to_string())),
    }
}

pub async fn update_session_title(
    Path(session_id): Path<String>,
    Json(data): Json<Value>,
) -> Json<ApiResponse<()>> {
    let title = data["title"].as_str();

    if title.is_none() {
        return Json(ApiResponse::error("title is empty".to_string()));
    }

    let title = title.unwrap().to_string();

    match boxagnts_gateway::api::chat_session::update_session_title(session_id, title).await {
        Ok(_) => Json(ApiResponse::success(())),
        Err(e) => Json(ApiResponse::error(e.to_string())),
    }
}

pub async fn delete_session_messages(
    Path(session_id): Path<String>,
    Json(data): Json<Value>,
) -> Json<ApiResponse<()>> {
    let messages = data["messages"].as_array();

    if messages.is_none() {
        return Json(ApiResponse::error("messages is empty".to_string()));
    }

    let mut message_ids: Vec<String> = Vec::new();

    for message in messages.unwrap() {
        let id = message.as_str();

        if let Some(id) = id {
            message_ids.push(id.to_string());
        }
    }

    match boxagnts_gateway::api::chat_session::delete_session_messages(session_id, message_ids).await {
        Ok(_) => Json(ApiResponse::success(())),
        Err(e) => Json(ApiResponse::error(e.to_string())),
    }
}

pub async fn clear_session_messages(Path(session_id): Path<String>) -> Json<ApiResponse<()>> {
    match boxagnts_gateway::api::chat_session::clear_session_messages(session_id).await {
        Ok(_) => Json(ApiResponse::success(())),
        Err(e) => Json(ApiResponse::error(e.to_string())),
    }
}