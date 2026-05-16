use super::model::{CronError, JobConfig, JobLog};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};


pub async fn init_storage() -> Result<(), CronError> {
    let saved_dir = boxagnts_workspace::path::get_saved_dir().await;
    let cron_config_file = saved_dir.join("cron.config.json");
    let cron_log_dir = saved_dir.join("cron-logs");

    if !cron_log_dir.exists() {
        tokio::fs::create_dir_all(&cron_log_dir)
            .await
            .map_err(|e| CronError::Internal(format!("create cron log dir error: {}", e)))?;
    }

    if !cron_config_file.exists() {
        let empty: HashMap<String, JobConfig> = HashMap::new();
        atomic_write_json(&cron_config_file, &empty).await?;
    }

    Ok(())
}

fn tmp_path(path: &Path) -> PathBuf {
    let mut os = path.as_os_str().to_os_string();
    os.push(".tmp");
    PathBuf::from(os)
}

pub async fn atomic_write_json<T: serde::Serialize + ?Sized>(
    path: impl AsRef<Path>,
    value: &T,
) -> Result<(), CronError> {
    let path = path.as_ref();
    let tmp = tmp_path(path);

    let content = serde_json::to_vec_pretty(value)
        .map_err(|e| CronError::Internal(format!("serialize json error: {}", e)))?;

    let mut file = tokio::fs::File::create(&tmp)
        .await
        .map_err(|e| CronError::Internal(format!("create temp file error: {}", e)))?;

    use tokio::io::AsyncWriteExt;
    file.write_all(&content)
        .await
        .map_err(|e| CronError::Internal(format!("write temp file error: {}", e)))?;
    file.flush()
        .await
        .map_err(|e| CronError::Internal(format!("flush temp file error: {}", e)))?;
    drop(file);

    #[cfg(target_os = "windows")]
    {
        if path.exists() {
            let _ = tokio::fs::remove_file(path).await;
        }
    }

    tokio::fs::rename(&tmp, path)
        .await
        .map_err(|e| CronError::Internal(format!("rename temp file error: {}", e)))?;

    Ok(())
}

pub async fn load_jobs() -> Result<HashMap<String, JobConfig>, CronError> {
    let saved_dir = boxagnts_workspace::path::get_saved_dir().await;
    let cron_config_file = saved_dir.join("cron.config.json");

    let content = tokio::fs::read_to_string(cron_config_file)
        .await
        .map_err(|e| CronError::Internal(format!("read jobs file error: {}", e)))?;

    if content.trim().is_empty() {
        return Ok(HashMap::new());
    }

    serde_json::from_str(&content)
        .map_err(|e| CronError::Internal(format!("parse jobs file error: {}", e)))
}

pub async fn save_jobs(jobs: &HashMap<String, JobConfig>) -> Result<(), CronError> {
    let saved_dir = boxagnts_workspace::path::get_saved_dir().await;
    let cron_config_file = saved_dir.join("cron.config.json");

    atomic_write_json(cron_config_file, jobs).await
}

pub async fn job_log_file(job_id: &str) -> PathBuf {
    let saved_dir = boxagnts_workspace::path::get_saved_dir().await;
    let cron_log_dir = saved_dir.join("cron-logs");

    cron_log_dir.join(format!("{}.jsonl", job_id))
}

pub async fn load_job_logs(job_id: &str) -> Result<Vec<JobLog>, CronError> {
    use tokio::io::AsyncBufReadExt;

    let path = job_log_file(job_id).await;

    if !path.exists() {
        return Ok(vec![]);
    }

    let file = tokio::fs::File::open(path)
        .await
        .map_err(|e| CronError::Internal(format!("open file error: {}", e)))?;

    let reader = tokio::io::BufReader::new(file);
    let mut lines = reader.lines();

    let mut result = Vec::new();

    while let Some(line) = lines
        .next_line()
        .await
        .map_err(|e| CronError::Internal(format!("read line error: {}", e)))?
    {
        if line.trim().is_empty() {
            continue;
        }

        let item: JobLog = serde_json::from_str(&line)
            .map_err(|e| CronError::Internal(format!("parse job log error: {}", e)))?;
        result.push(item);
    }

    Ok(result)
}

pub async fn append_job_log(job_id: &str, log: JobLog) -> Result<(), CronError> {
    use tokio::io::AsyncWriteExt;

    let path = job_log_file(job_id).await;

    let mut file = tokio::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .await
        .map_err(|e| CronError::Internal(format!("open file error: {}", e)))?;

    let line = serde_json::to_string(&log)
        .map_err(|e| CronError::Internal(format!("serialize job log error: {}", e)))?;
    file.write_all(line.as_bytes())
        .await
        .map_err(|e| CronError::Internal(format!("write line error: {}", e)))?;
    file.write("\n".as_bytes())
        .await
        .map_err(|e| CronError::Internal(format!("write error: {}", e)))?;
    file.flush()
        .await
        .map_err(|e| CronError::Internal(format!("flush temp file error: {}", e)))?;

    Ok(())
}

pub async fn delete_job_log(job_id: &str) -> Result<(), CronError> {
    let path = job_log_file(job_id).await;

    if path.exists() {
        tokio::fs::remove_file(path)
            .await
            .map_err(|e| CronError::Internal(format!("delete job log file error: {}", e)))?;
    }

    Ok(())
}

pub async fn read_last_job_log(job_id: &str) -> Result<Option<JobLog>, CronError> {
    use tokio::io::AsyncBufReadExt;

    let path = job_log_file(job_id).await;

    if !path.exists() {
        return Ok(None);
    }

    let file = tokio::fs::File::open(path)
        .await
        .map_err(|e| CronError::Internal(format!("open file error: {}", e)))?;

    let reader = tokio::io::BufReader::new(file);
    let mut lines = reader.lines();

    let mut last = None;

    while let Some(line) = lines
        .next_line()
        .await
        .map_err(|e| CronError::Internal(format!("read line error: {}", e)))?
    {
        if line.trim().is_empty() {
            continue;
        }

        let item: JobLog = serde_json::from_str(&line)
            .map_err(|e| CronError::Internal(format!("parse job log error: {}", e)))?;
        last = Some(item);
    }

    Ok(last)
}
