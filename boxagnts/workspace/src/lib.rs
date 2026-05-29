pub mod context;
pub mod history;
pub mod config;
pub mod permissions;
pub mod auth_store;
pub mod codex_oauth;
pub mod oauth_config;
pub mod output_styles;
pub mod agents_md;
pub mod oauth;

pub mod path {
    use std::path::{Path, PathBuf};

    static WORKSPACE_DIR: once_cell::sync::Lazy<tokio::sync::Mutex<PathBuf>> =
        once_cell::sync::Lazy::new(|| tokio::sync::Mutex::new(PathBuf::from(".")));

    static APP_DIR: once_cell::sync::Lazy<tokio::sync::Mutex<PathBuf>> =
        once_cell::sync::Lazy::new(|| tokio::sync::Mutex::new(PathBuf::from(".")));


    pub async fn get_workspace_dir() -> PathBuf {
        let worksapce_dir = WORKSPACE_DIR.lock().await;

        worksapce_dir.clone()
    }

    pub async fn set_worksapce_dir(worksapce_dir: &Path) {
        let mut worksapce_dir_write = WORKSPACE_DIR.lock().await;
        let worksapce_dir = worksapce_dir.to_path_buf();

        *worksapce_dir_write = worksapce_dir.clone();
    }

    pub async fn get_saved_dir() -> PathBuf {
        let worksapce_dir = WORKSPACE_DIR.lock().await;
        let worksapce_dir = worksapce_dir.clone();

        let saved_dir = worksapce_dir.join(".boxagnts");

        if !saved_dir.exists() {
            std::fs::create_dir_all(&saved_dir).expect("Could not create saved dir");
        }

        saved_dir
    }
    
    pub fn get_default_app_dir() -> PathBuf {
        let exe_path = std::env::current_exe()
            .expect("Unable to obtain the path of the executable file.")
            .parent()
            .expect("Unable to obtain the parent directory")
            .to_path_buf();
        
        
        exe_path
    }

    pub async fn get_app_dir() -> PathBuf {
        let app_dir = APP_DIR.lock().await;

        app_dir.clone()
    }

    pub async fn set_app_dir(app_dir: &Path) {
        let mut app_dir_write = APP_DIR.lock().await;
        let app_dir = app_dir.to_path_buf();

        *app_dir_write = app_dir.clone();
    }

    pub async fn get_app_extensions_dir() -> PathBuf {
        let app_dir = get_app_dir().await;

        app_dir.join("extensions")
    }

    pub async fn get_app_cache_dir() -> PathBuf {
        let app_dir = get_app_dir().await;

        let cache_dir = app_dir.join("caches");


        if !cache_dir.exists() {
            std::fs::create_dir_all(&cache_dir).expect("Could not create cache dir");
        }

        cache_dir
    }
}

pub async fn init(base_url: &str) -> anyhow::Result<()> {
    config::Settings::init(base_url).await?;
    auth_store::AuthStore::init().await?;
    agents_md::AgentsMdStore::init().await?;
    
    Ok(())
}