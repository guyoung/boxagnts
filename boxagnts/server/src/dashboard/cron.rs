use axum::{
    extract::{Path, State},
    Json,
};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

use boxagnts_gateway::cron::{
    app_state::AppState,
    model::{CreateJobReq, JobConfig, ListJobRes, JobLog, UpdateJobReq, CronError},
    scheduler,
    store,
};

fn error_into_response(err: CronError) -> Response {
    let (status, msg) = match err {
        CronError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
        CronError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
        CronError::Conflict(msg) => (StatusCode::CONFLICT, msg),
        CronError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
    };

    (status, Json(json!({ "error": msg }))).into_response()
}

pub async fn list_jobs(
    State(state): State<AppState>,
) -> Result<Json<Vec<ListJobRes>>, Response> {
    let jobs = state.jobs.read().await;

    let jobs: Vec<JobConfig> = jobs.values().cloned().collect();

    let mut result: Vec<ListJobRes> = Vec::new();

    for job in jobs {
        let log = store::read_last_job_log(&job.id).await.map_err(
            |e|error_into_response(e)
        )?;

        let mut j = ListJobRes::default();
        j.id = job.id;
        j.name = job.name.clone();
        j.description = job.description.clone();
        j.cron = job.cron.clone();
        j.enabled = job.enabled;

        if let Some(log) = log {
            j.last_run_at = Some(log.executed_at);
            j.last_run_success = Some(log.success);
        }

        result.push(j);
    }

    Ok(Json(result))
}

pub async fn get_job(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<JobConfig>, Response> {
    let jobs = state.jobs.read().await;
    let job = jobs
        .get(&id)
        .cloned()
        .ok_or_else(|| error_into_response(CronError::NotFound("job not found".to_string())))?;
    Ok(Json(job))
}

pub async fn create_job(
    State(state): State<AppState>,
    Json(req): Json<CreateJobReq>,
) -> Result<Json<JobConfig>, Response> {
    {
        let jobs = state.jobs.read().await;
        if jobs.contains_key(&req.id) {
            return Err(error_into_response(CronError::Conflict("job id already exists".to_string())));
        }
    }

    let job_cfg = JobConfig {
        id: req.id,
        name: req.name,
        description: req.description,
        cron: req.cron,
        enabled: req.enabled,
        timeout: req.timeout,
        prompt: req.prompt,
    };

    scheduler::schedule_job(state.clone(), job_cfg.clone()).await.map_err(
        |e|error_into_response(e)
    )?;

    {
        let mut jobs = state.jobs.write().await;
        jobs.insert(job_cfg.id.clone(), job_cfg.clone());

        if let Err(e) = store::save_jobs(&jobs).await {
            let _ = scheduler::unschedule_job(state.clone(), &job_cfg.id).await;
            return Err(error_into_response(e));
        }
    }

    Ok(Json(job_cfg))
}

pub async fn update_job(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<UpdateJobReq>,
) -> Result<Json<JobConfig>, Response> {
    let old_job = {
        let jobs = state.jobs.read().await;
        jobs.get(&id)
            .cloned()
            .ok_or_else(|| error_into_response(CronError::NotFound("job not found".to_string())))?
    };

    let mut new_job = old_job.clone();

    if let Some(name) = req.name {
        new_job.name = name;
    }
    if let Some(description) = req.description {
        new_job.description = description;
    }
    if let Some(cron) = req.cron {
        new_job.cron = cron;
    }
    if let Some(enabled) = req.enabled {
        new_job.enabled = enabled;
    }

    new_job.timeout = req.timeout;

    if let Some(prompt) = req.prompt {
        new_job.prompt = prompt;
    }

    scheduler::unschedule_job(state.clone(), &id).await.map_err(
        |e|error_into_response(e)
    )?;

    if let Err(e) = scheduler::schedule_job(state.clone(), new_job.clone()).await {
        let _ = scheduler::schedule_job(state.clone(), old_job.clone()).await;
        return Err(error_into_response(e));
    }

    {
        let mut jobs = state.jobs.write().await;
        jobs.insert(id.clone(), new_job.clone());

        if let Err(e) = store::save_jobs(&jobs).await {
            let _ = scheduler::unschedule_job(state.clone(), &id).await;
            let _ = scheduler::schedule_job(state.clone(), old_job.clone()).await;
            jobs.insert(id, old_job);
            return Err(error_into_response(e));
        }
    }

    Ok(Json(new_job))
}

pub async fn delete_job(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, Response> {
    {
        let jobs = state.jobs.read().await;
        if !jobs.contains_key(&id) {
            return Err(error_into_response(CronError::NotFound("job not found".to_string())));
        }
    }

    scheduler::unschedule_job(state.clone(), &id).await.map_err(
        |e|error_into_response(e)
    )?;

    {
        let mut jobs = state.jobs.write().await;
        jobs.remove(&id);
        store::save_jobs(&jobs).await.map_err(
            |e|error_into_response(e)
        )?;
    }

    store::delete_job_log(&id).await.map_err(
        |e|error_into_response(e)
    )?;

    Ok(Json(json!({
        "message": "deleted"
    })))
}

pub async fn list_job_logs(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Vec<JobLog>>, Response> {
    {
        let jobs = state.jobs.read().await;
        if !jobs.contains_key(&id) {
            return Err(error_into_response(CronError::NotFound("job not found".to_string())));
        }
    }

    let logs = store::load_job_logs(&id).await.map_err(
        |e|error_into_response(e)
    )?;

    Ok(Json(logs))
}
