use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::Value;

use boxagnts_core::cost::CostTracker;
use boxagnts_core::types::ToolDefinition;
use boxagnts_workspace::config::PermissionMode;


/// The result of executing a tool.
#[derive(Debug, Clone, Deserialize)]
pub struct ToolResult {
    /// Content to send back to the model as the tool result.
    pub content: String,
    /// Whether this invocation was an error.
    pub is_error: bool,
    /// Optional structured metadata (for the TUI to render diffs, etc.).
    pub metadata: Option<Value>,
}

impl ToolResult {
    pub fn success(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            is_error: false,
            metadata: None,
        }
    }

    pub fn error(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            is_error: true,
            metadata: None,
        }
    }

    pub fn with_metadata(mut self, meta: Value) -> Self {
        self.metadata = Some(meta);
        self
    }
}

/// Permission level required by a tool.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionLevel {
    /// No permission needed (read-only, purely informational).
    None,
    /// Read-only access to the filesystem or network.
    ReadOnly,
    /// Write access to the filesystem.
    Write,
    /// Arbitrary command execution.
    Execute,
    /// Potentially dangerous (e.g., bypass sandbox).
    Dangerous,
    /// Unconditionally forbidden — the action must never be executed regardless
    /// of permission mode.  Used by BashTool when the classifier identifies a
    /// `Critical`-risk command (e.g. `rm -rf /`, fork-bomb, `dd if=…`).
    Forbidden,
}

/***
#[derive(Debug)]
pub struct PendingPermissionRequest {
    pub tool_use_id: String,
    pub request: PermissionRequest,
    pub reason: String,
    pub decision_tx: Option<tokio::sync::oneshot::Sender<PermissionDecision>>,
}

#[derive(Default)]
pub struct PendingPermissionStore {
    pub queue: VecDeque<PendingPermissionRequest>,
    pub waiting: HashMap<String, PendingPermissionRequest>,
}
***/

/***
/// A cloneable handle for injecting notification messages into the next agent turn.
/// Used by background tasks with `notify_on_complete` to signal completion without polling.
#[derive(Clone)]
pub struct CompletionNotifier(Arc<dyn Fn(String) + Send + Sync>);

impl CompletionNotifier {
    pub fn new(f: impl Fn(String) + Send + Sync + 'static) -> Self {
        Self(Arc::new(f))
    }
    pub fn notify(&self, msg: String) {
        (self.0)(msg);
    }
}
***/

/// Shared context passed to every tool invocation.
#[derive(Clone)]
pub struct ToolContext {
    //pub cwd: PathBuf,
    pub permission_mode: PermissionMode,
    //pub permission_handler: Arc<dyn PermissionHandler>,
    pub cost_tracker: Arc<CostTracker>,
    pub session_id: Option<String>,
    /***
    pub file_history: Arc<parking_lot::Mutex<boxagnts_core::file_history::FileHistory>>,    
    ***/
    pub current_turn: Arc<AtomicUsize>,
    /// If true, suppress interactive prompts (batch / CI mode).
    pub non_interactive: bool,
    /// Optional MCP manager for ListMcpResources / ReadMcpResource tools.
    pub mcp_manager: Option<Arc<boxagnts_mcp::McpManager>>,
    /// Configured event hooks (PreToolUse, PostToolUse, etc.).
    pub config: boxagnts_workspace::config::Config,
    /// Managed agent (manager-executor) configuration, if active.
    pub managed_agent_config: Option<boxagnts_workspace::config::ManagedAgentConfig>,
    // /// Optional notifier for injecting completion messages into the next agent turn.
    // /// Set when the query loop has a command queue wired up.
    // pub completion_notifier: Option<CompletionNotifier>,
    // /// Queue used by interactive mode to surface permission dialogs to the TUI.
    // pub pending_permissions: Option<Arc<parking_lot::Mutex<PendingPermissionStore>>>,
    // /// Shared permission manager so the interactive loop can record session/persistent approvals.
    // pub permission_manager: Option<Arc<std::sync::Mutex<boxagnts_workspace::permissions::PermissionManager>>>,
    /// Allowed outbound hosts
    pub allowed_outbound_hosts: Vec<String>,
    ///
    pub block_url: Option<String>,
}

impl ToolContext {
    /// Resolve a potentially relative path against the working directory.
    /***
    pub fn resolve_path(&self, path: &str) -> PathBuf {
        let p = PathBuf::from(path);
        if p.is_absolute() {
            p
        } else {
            self.working_dir.join(p)
        }
    }
    ***/

    /***
    fn permission_allowed_roots(&self) -> Vec<PathBuf> {
        let mut roots = self.config.workspace_paths.clone();
        roots.extend(self.config.additional_dirs.clone());
        roots
    }


    fn build_permission_request(
        &self,
        tool_name: &str,
        description: &str,
        details: Option<String>,
        is_read_only: bool,
        path: Option<PathBuf>,
    ) -> PermissionRequest {
        PermissionRequest {
            tool_name: tool_name.to_string(),
            description: description.to_string(),
            details,
            is_read_only,
            path: path.map(|p| p.display().to_string()),
            working_dir: Some(self.cwd.clone()),
            allowed_roots: self.permission_allowed_roots(),
            context_description: None,
        }
    }

    fn request_permission_inner(
        &self,
        request: PermissionRequest,
    ) -> Result<(), boxagnts_core::error::ClaudeError> {
        let interactive_reason = request.details.clone();
        let decision = self.permission_handler.request_permission(&request);
        match decision {
            PermissionDecision::Allow | PermissionDecision::AllowPermanently => Ok(()),
            PermissionDecision::Ask { reason } if self.non_interactive => Err(
                boxagnts_core::error::ClaudeError::PermissionDenied(format!(
                    "Permission denied for tool '{}': {}",
                    request.tool_name,
                    interactive_reason.unwrap_or(reason)
                )),
            ),
            PermissionDecision::Ask { reason } => {
                let Some(queue) = &self.pending_permissions else {
                    return Err(boxagnts_core::error::ClaudeError::PermissionDenied(format!(
                        "Permission denied for tool '{}'",
                        request.tool_name
                    )));
                };

                let (tx, rx) = tokio::sync::oneshot::channel();
                queue.lock().queue.push_back(PendingPermissionRequest {
                    tool_use_id: format!(
                        "perm-{}-{}",
                        self.session_id,
                        self.current_turn.fetch_add(1, Ordering::Relaxed)
                    ),
                    request,
                    reason: interactive_reason.unwrap_or(reason),
                    decision_tx: Some(tx),
                });

                let decision = tokio::task::block_in_place(|| rx.blocking_recv());
                match decision {
                    Ok(PermissionDecision::Allow | PermissionDecision::AllowPermanently) => Ok(()),
                    _ => Err(boxagnts_core::error::ClaudeError::PermissionDenied(
                        "Permission denied by user".to_string(),
                    )),
                }
            }
            _ => Err(boxagnts_core::error::ClaudeError::PermissionDenied(format!(
                "Permission denied for tool '{}'",
                request.tool_name
            ))),
        }
    }

    /// Check permissions for a tool invocation.
    pub fn check_permission(
        &self,
        tool_name: &str,
        description: &str,
        is_read_only: bool,
    ) -> Result<(), boxagnts_core::error::ClaudeError> {
        let request = self.build_permission_request(tool_name, description, None, is_read_only, None);
        self.request_permission_inner(request)
    }

    pub fn check_permission_for_path(
        &self,
        tool_name: &str,
        description: &str,
        path: PathBuf,
        is_read_only: bool,
    ) -> Result<(), boxagnts_core::error::ClaudeError> {
        let request = self.build_permission_request(tool_name, description, None, is_read_only, Some(path));
        self.request_permission_inner(request)
    }

    /// Like `check_permission` but also passes structured `details` text
    /// (e.g. a risk explanation) that the TUI permission dialog can display.
    pub fn check_permission_with_details(
        &self,
        tool_name: &str,
        description: &str,
        details: &str,
        is_read_only: bool,
    ) -> Result<(), boxagnts_core::error::ClaudeError> {
        let request = self.build_permission_request(
            tool_name,
            description,
            Some(details.to_string()),
            is_read_only,
            None,
        );
        self.request_permission_inner(request).map_err(|_| {
            boxagnts_core::error::ClaudeError::PermissionDenied(format!(
                "Permission denied for tool '{}': {}",
                tool_name, details
            ))
        })
    }

    pub fn path_is_within_workspace(&self, path: &std::path::Path) -> bool {
        let resolved = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
        let mut roots = vec![
            std::fs::canonicalize(&self.cwd).unwrap_or_else(|_| self.cwd.clone()),
        ];
        roots.extend(
            self.permission_allowed_roots()
                .into_iter()
                .map(|root| std::fs::canonicalize(&root).unwrap_or(root)),
        );
        roots.iter().any(|root| resolved.starts_with(root))
    }

    pub fn check_permission_with_details_and_path(
        &self,
        tool_name: &str,
        description: &str,
        details: &str,
        path: PathBuf,
        is_read_only: bool,
    ) -> Result<(), boxagnts_core::error::ClaudeError> {
        let request = self.build_permission_request(
            tool_name,
            description,
            Some(details.to_string()),
            is_read_only,
            Some(path),
        );
        self.request_permission_inner(request).map_err(|_| {
            boxagnts_core::error::ClaudeError::PermissionDenied(format!(
                "Permission denied for tool '{}': {}",
                tool_name, details
            ))
        })
    }

     ***/

    /***
 pub fn current_turn_index(&self) -> usize {
     self.current_turn.load(Ordering::Relaxed)
 }


 pub fn record_file_change(
     &self,
     path: PathBuf,
     before_content: &[u8],
     after_content: &[u8],
     tool_name: &str,
 ) {
     self.file_history.lock().record_modification(
         path,
         before_content,
         after_content,
         self.current_turn_index(),
         tool_name,
     );
 }
 ***/
    
    pub async fn get_work_dir(&self) -> PathBuf {
        let cwd = boxagnts_workspace::path::get_workspace_dir().await;

        let work_dir = cwd.join("root");
        
        if !work_dir.exists() {
            std::fs::create_dir_all(&work_dir).expect("Could not create work dir");
        }

        work_dir
    }

    pub async fn get_app_extensions_dir(&self) -> PathBuf {
        let app_extensions_dir = boxagnts_workspace::path::get_app_extensions_dir().await;
        
        app_extensions_dir
    }

    pub async fn get_workspace_extensions_dir(&self) -> PathBuf {
        let cwd = boxagnts_workspace::path::get_workspace_dir().await;

        let extensions_dir = cwd.join("extensions");

        extensions_dir
    }

    pub async fn get_app_cache_dir(&self) -> PathBuf {
        let app_cache_dir = boxagnts_workspace::path::get_app_cache_dir().await;

        app_cache_dir
    }

    
    pub fn get_allowed_outbound_hosts(&self) -> Vec<String> {
      self.allowed_outbound_hosts.clone()
    }
    
    
}

/// The trait every tool must implement.
#[async_trait]
pub trait Tool: Send + Sync {
    /// Human-readable name (matches the constant in boxagnts_core::constants).
    fn name(&self) -> &str;

    /// One-line description shown to the LLM.
    fn description(&self) -> &str;

    /// The permission level the tool requires.
    fn permission_level(&self) -> PermissionLevel;

    /// JSON Schema describing the tool's input parameters.
    fn input_schema(&self) -> Value;

    /// Execute the tool with the given JSON input.
    async fn execute(
        &self,
        input: Value,
        ctx: &ToolContext,
    ) -> ToolResult;

    /// Produce a `ToolDefinition` suitable for sending to the API.
    fn to_definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: self.name().to_string(),
            description: self.description().to_string(),
            input_schema: self.input_schema(),
        }
    }
}
