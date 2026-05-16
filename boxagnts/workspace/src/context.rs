use std::path::PathBuf;

/// Builds the system-level and user-level context that gets prepended to
/// every conversation with the model.
pub struct ContextBuilder {
    cwd: PathBuf,
    disable_claude_mds: bool,
}

impl ContextBuilder {
    pub fn new(cwd: PathBuf) -> Self {
        Self {
            cwd,
            disable_claude_mds: false,
        }
    }

    pub fn disable_claude_mds(mut self, val: bool) -> Self {
        self.disable_claude_mds = val;
        self
    }

    /// System context (git status, platform, IDE, etc.)
    pub async fn build_system_context(&self) -> String {
        let mut parts = vec![];

        // Platform information
        parts.push(format!("Platform: {}", "linux"));
        parts.push(format!("Working directory: {}", self.cwd.display()));

        /***
        if let Some(git_context) = self.get_git_context().await {
            parts.push(git_context);
        }
        ***/

        /***
        // IDE context — injected when an IDE extension is connected.
        // Mirrors TS getContextAttachments() → IdeContext attachment.
        if let Some(ide_ctx) = boxagnts_core::attachments::get_ide_context() {
            parts.push(format!("# IDE Context\n{}", ide_ctx));
        }
        ***/

        parts.join("\n\n")
    }

    /// User context (date, AGENTS.md memories, etc.)
    pub async fn build_user_context(&self) -> String {
        let mut parts = vec![];

        let date = chrono::Local::now().format("%A, %B %d, %Y").to_string();
        parts.push(format!("Today's date is {}.", date));

        if !self.disable_claude_mds {
            if let Some(claude_md) = self.find_and_read_claude_md().await {
                parts.push(claude_md);
            }
        }

        parts.join("\n\n")
    }

    /// Gather short git status + recent log.
    /***
    async fn get_git_context(&self) -> Option<String> {
        let output = tokio::process::Command::new("git")
            .args(["status", "--short", "--branch"])
            .current_dir(&self.cwd)
            .output()
            .await
            .ok()?;

        if !output.status.success() {
            return None;
        }

        let status = String::from_utf8_lossy(&output.stdout).to_string();

        let log_output = tokio::process::Command::new("git")
            .args(["log", "--oneline", "-5"])
            .current_dir(&self.cwd)
            .output()
            .await
            .ok()?;

        let log = String::from_utf8_lossy(&log_output.stdout).to_string();

        let mut result = format!("# Git Status\n{}", status.trim());
        if !log.trim().is_empty() {
            result.push_str(&format!("\n\n# Recent Commits\n{}", log.trim()));
        }

        Some(result)
    }
    ***/

    /// Walk up from cwd looking for AGENTS.md files and the global one.
    async fn find_and_read_claude_md(&self) -> Option<String> {
        let mut claude_mds = vec![];

        let app_dir = crate::path::get_app_dir().await;

        let global_claude_md = app_dir.join("AGENTS.md");

        if global_claude_md.exists() {
            if let Ok(content) = tokio::fs::read_to_string(&global_claude_md).await {
                claude_mds.push(format!(
                    "# Memory (from {})\n{}",
                    global_claude_md.display(),
                    content
                ));
            }
        }

        let saved_dir = crate::path::get_saved_dir().await;

        let workspace_claude_md = saved_dir.join("AGENTS.md");

        if workspace_claude_md.exists() {
            if let Ok(content) = tokio::fs::read_to_string(&workspace_claude_md).await {
                claude_mds.push(format!(
                    "# Project Memory (from {})\n{}",
                    workspace_claude_md.display(),
                    content
                ));
            }
        }

        if claude_mds.is_empty() {
            None
        } else {
            Some(claude_mds.join("\n\n"))
        }
    }
}
