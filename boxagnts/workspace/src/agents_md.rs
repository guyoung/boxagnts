use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AgentsMdStore(String);


impl AgentsMdStore {
    pub async fn path() -> PathBuf {
        let dir = crate::path::get_saved_dir().await;
        dir.join("AGENTS.md")
    }

    pub async fn init() -> anyhow::Result<()> {
        let path = Self::path().await;

        if !path.exists() {
            if let Some(parent) = path.parent() {
                tokio::fs::create_dir_all(parent).await?;
            }

            let content = "";
            tokio::fs::write(&path, content).await?;
        }

        Ok(())
    }

    pub async fn load() -> anyhow::Result<Self> {
        let path = Self::path().await;

        if path.exists() {
            let content = tokio::fs::read_to_string(&path).await?;

            Ok(AgentsMdStore(content))
        } else {
            Ok(AgentsMdStore::default())
        }
    }

    pub async fn save(content: &str) -> anyhow::Result<()> {
        let path = Self::path().await;

        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }       
        tokio::fs::write(&path, content).await?;

        Ok(())
    }
    
    pub fn content(&self) -> String {
        self.0.clone()
    }
}