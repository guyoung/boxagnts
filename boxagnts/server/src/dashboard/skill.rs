use axum::Json;

use boxagnts_gateway::api::skill::SkillConfig;


use super::ApiResponse;

pub async fn list_skills() -> Json<ApiResponse<Vec<SkillConfig>>> {
    let skills = boxagnts_gateway::api::skill::load_skills().await;

    match skills {
        Ok(skills) =>  Json(ApiResponse::success(skills.values().cloned().collect())),
        Err(e) => Json(ApiResponse::error(e.to_string())),
    }
}
