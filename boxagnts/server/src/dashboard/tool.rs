use axum::Json;

use boxagnts_gateway::api::tool::ToolConfig;


use super::ApiResponse;

pub async fn list_tools() -> Json<ApiResponse<Vec<ToolConfig>>> {
    let tools = boxagnts_gateway::api::tool::load_tools().await;

    match tools {
        Ok(tools) =>  Json(ApiResponse::success(tools.values().cloned().collect())),
        Err(e) => Json(ApiResponse::error(e.to_string())),
    }
}
