use std::sync::Arc;

use chrono::Utc;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tokio_cron_scheduler::{Job, JobScheduler};
use tokio::pin;
use tokio::time::{timeout, Duration};
use uuid::Uuid;

use super::{
    app_state::AppState,
    model::{CronError, JobConfig, JobLog},
    store,
};

pub static SCHEDULER: Lazy<Mutex<Option<Arc<JobScheduler>>>> = Lazy::new(|| Mutex::new(None));

pub async fn init_scheduler() -> Result<(), CronError> {
    let scheduler = Arc::new(
        JobScheduler::new()
            .await
            .map_err(|e| CronError::Internal(format!("create scheduler error: {}", e)))?,
    );

    scheduler
        .start()
        .await
        .map_err(|e| CronError::Internal(format!("start scheduler error: {}", e)))?;

    let mut guard = SCHEDULER.lock().await;
    *guard = Some(scheduler);

    Ok(())
}

pub async fn get_scheduler() -> Result<Arc<JobScheduler>, CronError> {
    let guard = SCHEDULER.lock().await;
    guard
        .as_ref()
        .cloned()
        .ok_or_else(|| CronError::Internal("scheduler not initialized".to_string()))
}

pub async fn append_execution_log(
    job_id: String,
    job_name: String,
    success: bool,
    message: String,
) -> Result<(), CronError> {
    let log = JobLog {
        id: Uuid::new_v4().to_string(),
        job_id: job_id.clone(),
        job_name,
        executed_at: Utc::now(),
        success,
        message,
    };

    store::append_job_log(&job_id, log).await
}

pub async fn schedule_job(state: AppState, job_cfg: JobConfig) -> Result<(), CronError> {
    if !job_cfg.enabled {
        return Ok(());
    }

    let job_id = job_cfg.id.clone();
    let job_name = job_cfg.name.clone();    
    let cron_expr = job_cfg.cron.clone();
    let job_timeout = job_cfg.timeout.unwrap_or(180);
    let model = job_cfg.model.clone();
    let prompt = job_cfg.prompt.clone();
    
    let cron_job = Job::new_async(cron_expr.as_str(), move |_uuid, _lock| {
        let job_id = job_id.clone();
        let job_name = job_name.clone();
        let model = model.clone();
        let prompt = prompt.clone();

        Box::pin(async move {
            println!(
                "[job exec] id={}, name={}, prompt={}",
                job_id, job_name, prompt
            );

            let handle = super::job::execute(prompt, model).await;

            match handle {
                Ok(handle) => {
                    let cancel_token = handle.cancellation_token();

                    let fut = handle.await_result();
                    pin!(fut);

                    let result = match timeout(Duration::from_secs(job_timeout), &mut fut).await {
                        Ok(result) => result,
                        Err(_) => {
                            cancel_token.cancel();
                            fut.await
                        }
                    };

                    match result {
                        Ok(result) => {
                            println!("[job exec] result={:?}", result);

                            let result_type = result["type"].as_str().unwrap_or_default();

                            let _ = append_execution_log(
                                job_id,
                                job_name,
                                true,
                                format!("type: {}", result_type),
                            )
                            .await;
                        }
                        Err(e) => {
                            println!("[job exec] result={:?}", e);

                            let _ = append_execution_log(job_id, job_name, false, format!("{}", e))
                                .await;
                        }
                    }
                }
                Err(e) => {
                    println!("[job exec] result={:?}", e);

                    let _ = append_execution_log(job_id, job_name, false, format!("{}", e)).await;
                }
            }
        })
    })
    .map_err(|e| CronError::BadRequest(format!("invalid cron expression: {}", e)))?;

    let guid = cron_job.guid();

    let scheduler = get_scheduler().await?;
    scheduler
        .add(cron_job)
        .await
        .map_err(|e| CronError::Internal(format!("add scheduler job error: {}", e)))?;

    state.job_map.write().await.insert(job_cfg.id.clone(), guid);

    Ok(())
}

pub async fn unschedule_job(state: AppState, job_id: &str) -> Result<(), CronError> {
    let maybe_guid = {
        let mut map = state.job_map.write().await;
        map.remove(job_id)
    };

    if let Some(guid) = maybe_guid {
        let scheduler = get_scheduler().await?;
        scheduler
            .remove(&guid)
            .await
            .map_err(|e| CronError::Internal(format!("remove scheduler job error: {}", e)))?;
    }

    Ok(())
}

pub async fn reload_all_jobs(state: AppState) -> Result<(), CronError> {
    let scheduler = get_scheduler().await?;

    // remove all the registered jobs recorded in the current job_map
    let old_guids = {
        let map = state.job_map.read().await;
        map.values().cloned().collect::<Vec<_>>()
    };

    for guid in old_guids {
        let _ = scheduler.remove(&guid).await;
    }

    // Clear job_map
    state.job_map.write().await.clear();

    // Reregister according to the jobs configuration
    let all_jobs = state.jobs.read().await.clone();
    for (_, job_cfg) in all_jobs {
        schedule_job(state.clone(), job_cfg).await?;
    }

    Ok(())
}
