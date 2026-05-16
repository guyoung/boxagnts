use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use super::model::{SiteError, SiteConfig};

pub async fn init_storage() -> Result<(), SiteError> {
    let saved_dir = boxagnts_workspace::path::get_saved_dir().await;
    let site_config_file = saved_dir.join("site.config.json");

    if !site_config_file.exists() {
        let empty: HashMap<String, SiteConfig> = HashMap::new();
        atomic_write_json(&site_config_file, &empty).await?;
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
) -> Result<(), SiteError> {
    let path = path.as_ref();
    let tmp = tmp_path(path);

    let content = serde_json::to_vec_pretty(value)
        .map_err(|e| SiteError::Internal(format!("serialize json error: {}", e)))?;

    let mut file = tokio::fs::File::create(&tmp)
        .await
        .map_err(|e| SiteError::Internal(format!("create temp file error: {}", e)))?;

    use tokio::io::AsyncWriteExt;
    file.write_all(&content)
        .await
        .map_err(|e| SiteError::Internal(format!("write temp file error: {}", e)))?;
    file.flush()
        .await
        .map_err(|e| SiteError::Internal(format!("flush temp file error: {}", e)))?;
    drop(file);

    #[cfg(target_os = "windows")]
    {
        if path.exists() {
            let _ = tokio::fs::remove_file(path).await;
        }
    }

    tokio::fs::rename(&tmp, path)
        .await
        .map_err(|e| SiteError::Internal(format!("rename temp file error: {}", e)))?;

    Ok(())
}

pub async fn load_sites() -> Result<HashMap<String, SiteConfig>, SiteError> {
    let saved_dir = boxagnts_workspace::path::get_saved_dir().await;
    let site_config_file = saved_dir.join("site.config.json");

    let content = tokio::fs::read_to_string(site_config_file)
        .await
        .map_err(|e| SiteError::Internal(format!("read sites file error: {}", e)))?;

    if content.trim().is_empty() {
        return Ok(HashMap::new());
    }

    serde_json::from_str(&content)
        .map_err(|e| SiteError::Internal(format!("parse sites file error: {}", e)))
}

pub async fn save_sites(sites: &HashMap<String, SiteConfig>) -> Result<(), SiteError> {
    let saved_dir = boxagnts_workspace::path::get_saved_dir().await;
    let site_config_file = saved_dir.join("site.config.json");

    atomic_write_json(site_config_file, sites).await
}

pub async fn find_site_name(name: &str) -> Result<Option<SiteConfig>, SiteError> {
    let saved_dir = boxagnts_workspace::path::get_saved_dir().await;
    let site_config_file = saved_dir.join("site.config.json");

    let content = tokio::fs::read_to_string(site_config_file)
        .await
        .map_err(|e| SiteError::Internal(format!("read sites file error: {}", e)))?;

    if content.trim().is_empty() {
        return Ok(None);
    }

    let sites: HashMap<String, SiteConfig > = serde_json::from_str(&content)
        .map_err(|e| SiteError::Internal(format!("parse sites file error: {}", e)))?;
    
    for (_k, v) in sites {
        if v.name.as_str() == name {
            return Ok(Some(v));
        }
    }
    
    
    Ok(None)
}



