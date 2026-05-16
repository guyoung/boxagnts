use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};


// -----------------------------------------------------------------------
// Danger level assigned to each tool type
// -----------------------------------------------------------------------

/// How dangerous a tool operation is — used as the default decision when
/// no explicit rule matches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionLevel {
    /// Read-only operations (Glob, Grep, Read, WebSearch, etc.).
    Read,
    /// File write/edit operations (Write, Edit).
    Write,
    /// Shell command execution (Bash).
    Execute,
    /// Outbound network access (WebFetch).
    Network,
}

impl PermissionLevel {
    /// Derive the permission level from a well-known tool name.
    pub fn for_tool(tool_name: &str) -> Self {
        match tool_name {
            "Bash" | "bash" => Self::Execute,
            "Write" | "Edit" | "NotebookEdit" => Self::Write,
            "WebFetch" => Self::Network,
            _ => Self::Read,
        }
    }
}

// -----------------------------------------------------------------------
// Rule action & scope
// -----------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionAction {
    Allow,
    Deny,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionScope {
    /// Only lasts for the current process session.
    Session,
    /// Saved to settings.json and survives restarts.
    Persistent,
}

// -----------------------------------------------------------------------
// Rule definition
// -----------------------------------------------------------------------

/// A single permission rule.
///
/// Matches requests where:
///   - `tool_name` is `None` (applies to every tool) OR equals the
///     request tool name.
///   - `path_pattern` is `None` OR the glob pattern matches the
///     request path.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionRule {
    /// `None` means "applies to all tools".
    pub tool_name: Option<String>,
    /// Optional glob pattern for file / command paths.
    pub path_pattern: Option<String>,
    pub action: PermissionAction,
    pub scope: PermissionScope,
}

impl PermissionRule {
    /// Returns `true` when this rule matches the given tool name and
    /// optional path argument.
    pub fn matches(&self, tool_name: &str, path: Option<&str>) -> bool {
        // Tool name check
        if let Some(ref rule_tool) = self.tool_name {
            if rule_tool != tool_name {
                return false;
            }
        }
        // Path pattern check — only when a pattern is specified
        if let Some(ref pattern) = self.path_pattern {
            let Some(p) = path else {
                // Rule requires a path but none was provided → no match
                return false;
            };
            let pat = match glob::Pattern::new(pattern) {
                Ok(pat) => pat,
                Err(_) => return false,
            };
            if !pat.matches(p) {
                return false;
            }
        }
        true
    }
}

// -----------------------------------------------------------------------
// Serialised rule (stored in settings.json)
// -----------------------------------------------------------------------

/// Serde-friendly representation of a `PermissionRule` saved to disk.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SerializedPermissionRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_pattern: Option<String>,
    pub action: PermissionAction,
}

impl From<&PermissionRule> for SerializedPermissionRule {
    fn from(r: &PermissionRule) -> Self {
        Self {
            tool_name: r.tool_name.clone(),
            path_pattern: r.path_pattern.clone(),
            action: r.action.clone(),
        }
    }
}

impl From<&SerializedPermissionRule> for PermissionRule {
    fn from(s: &SerializedPermissionRule) -> Self {
        Self {
            tool_name: s.tool_name.clone(),
            path_pattern: s.path_pattern.clone(),
            action: s.action.clone(),
            scope: PermissionScope::Persistent,
        }
    }
}

// -----------------------------------------------------------------------
// Decision type
// -----------------------------------------------------------------------

/// The outcome of evaluating a permission request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionDecision {
    /// Unconditionally allow.
    Allow,
    /// Allow and remember permanently.
    AllowPermanently,
    /// Deny.
    Deny,
    /// Deny and remember permanently.
    DenyPermanently,
    /// Ask the user (show dialog) with an explanation of why.
    Ask { reason: String },
}

// -----------------------------------------------------------------------
// Format a human-readable explanation for the dialog
// -----------------------------------------------------------------------

/// Build the explanation paragraph shown in the permission dialog.
///
/// Mirrors the TS `createPermissionRequestMessage` / `permissionExplainer`
/// output style.
pub fn format_permission_reason(
    tool_name: &str,
    description: &str,
    path: Option<&str>,
    level: PermissionLevel,
) -> String {
    match level {
        PermissionLevel::Execute => description.to_string(),
        PermissionLevel::Write => {
            let target = path.unwrap_or(description);
            let extra = if target.contains("/etc/") || target.contains("\\etc\\") {
                "\nModifying system files could affect network resolution \
                     and system configuration."
            } else if target.starts_with("~/.") || target.contains("/.") {
                "\nThis is a hidden/configuration file."
            } else {
                "\nThis will write to the filesystem."
            };
            format!("{} wants to write to `{}`{}", tool_name, target, extra)
        }
        PermissionLevel::Network => {
            let url = path.unwrap_or(description);
            format!(
                "WebFetch wants to fetch: `{}`\nThis will make an outbound HTTP request.",
                url
            )
        }
        PermissionLevel::Read => {
            let target = path.unwrap_or(description);
            format!("{} wants to read: `{}`", tool_name, target)
        }
    }
}

// -----------------------------------------------------------------------
// PermissionManager
// -----------------------------------------------------------------------

/// Returns true when `path` falls under the active workspace roots.
fn is_path_within_allowed_roots(
    path: &str,
    working_dir: Option<&std::path::Path>,
    allowed_roots: &[std::path::PathBuf],
) -> bool {
    let canonical_path =
        std::fs::canonicalize(path).unwrap_or_else(|_| std::path::PathBuf::from(path));

    let mut roots: Vec<std::path::PathBuf> = Vec::new();
    if let Some(root) = working_dir {
        roots.push(std::fs::canonicalize(root).unwrap_or_else(|_| root.to_path_buf()));
    }
    roots.extend(
        allowed_roots
            .iter()
            .map(|root| std::fs::canonicalize(root).unwrap_or_else(|_| root.clone())),
    );

    roots.iter().any(|root| canonical_path.starts_with(root))
}

/// Pending permission request waiting for resolution (e.g. from a bridge
/// remote peer or the interactive TUI dialog).
pub struct PendingPermission {
    pub tool_use_id: String,
    pub created_at: std::time::Instant,
    pub resolve_tx: tokio::sync::oneshot::Sender<PermissionDecision>,
}

/// Central permission manager: holds mode, session rules, persistent
/// rules, and any in-flight pending decisions.
pub struct PermissionManager {
    pub mode: crate::config::PermissionMode,
    /// Rules added during this session only.
    pub session_rules: Vec<PermissionRule>,
    /// Rules loaded from / saved to settings.json.
    pub persistent_rules: Vec<PermissionRule>,
    /// Pending interactive decisions keyed by tool_use_id.
    pending: Vec<PendingPermission>,
}

impl PermissionManager {
    /// Construct from a mode and the current settings (which may contain
    /// previously-persisted rules).
    pub fn new(mode: crate::config::PermissionMode, settings: &crate::config::Settings) -> Self {
        let persistent_rules = settings
            .permission_rules
            .iter()
            .map(PermissionRule::from)
            .collect();
        Self {
            mode,
            session_rules: Vec::new(),
            persistent_rules,
            pending: Vec::new(),
        }
    }

    // ----------------------------------------------------------------
    // Evaluation (ported from TS hasPermissionsToUseTool)
    // ----------------------------------------------------------------

    /// Evaluate whether `tool_name` should be allowed to run.
    ///
    /// Evaluation order (faithful to TS behaviour):
    /// 1. BypassPermissions → always Allow.
    /// 2. Check deny rules (persistent first, then session) → if any
    ///    matched, Deny.
    /// 3. Check allow rules (persistent first, then session) → if any
    ///    matched, Allow.
    /// 4. AcceptEdits → Allow (auto-accept file edits).
    /// 5. Plan mode → Allow reads; deny everything else.
    /// 6. Default → derive from tool danger level.
    pub fn evaluate(
        &self,
        tool_name: &str,
        description: &str,
        path: Option<&str>,
        working_dir: Option<&std::path::Path>,
        allowed_roots: &[std::path::PathBuf],
    ) -> PermissionDecision {
        use crate::config::PermissionMode;

        // Step 1 — bypass everything
        if self.mode == PermissionMode::BypassPermissions {
            return PermissionDecision::Allow;
        }

        // Steps 2–3 — evaluate explicit rules (deny has priority over
        // allow; persistent rules evaluated before session rules within
        // each polarity, matching TS rule-source ordering)
        let all_rules = self
            .persistent_rules
            .iter()
            .chain(self.session_rules.iter());

        let mut deny_matched = false;
        let mut allow_matched = false;

        for rule in all_rules {
            if rule.matches(tool_name, path) {
                match rule.action {
                    PermissionAction::Deny => {
                        deny_matched = true;
                    }
                    PermissionAction::Allow => {
                        allow_matched = true;
                    }
                }
            }
        }

        if deny_matched {
            return PermissionDecision::Deny;
        }

        if allow_matched {
            return PermissionDecision::Allow;
        }

        let level = match PermissionLevel::for_tool(tool_name) {
            PermissionLevel::Read
                if !matches!(
                    tool_name,
                    "Read"
                        | "Glob"
                        | "Grep"
                        | "ListMcpResources"
                        | "ReadMcpResource"
                        | "LSP"
                        | "Skill"
                ) =>
            {
                PermissionLevel::Execute
            }
            other => other,
        };
        let read_in_workspace = path
            .is_some_and(|target| is_path_within_allowed_roots(target, working_dir, allowed_roots));
        let should_ask_read = match tool_name {
            "ListMcpResources" | "ReadMcpResource" => true,
            _ if matches!(level, PermissionLevel::Read) && path.is_some() => !read_in_workspace,
            _ => false,
        };

        // Step 4 — AcceptEdits: only auto-allow Edit; everything else keeps normal checks.
        if self.mode == PermissionMode::AcceptEdits && tool_name == "Edit" {
            return PermissionDecision::Allow;
        }

        // Step 5 — Plan mode: reads only
        if self.mode == PermissionMode::Plan {
            return match level {
                PermissionLevel::Read => PermissionDecision::Allow,
                _ => PermissionDecision::Deny,
            };
        }

        // Step 6 — Default / remaining AcceptEdits behavior.
        match level {
            PermissionLevel::Read if !should_ask_read => PermissionDecision::Allow,
            PermissionLevel::Read
            | PermissionLevel::Write
            | PermissionLevel::Execute
            | PermissionLevel::Network => {
                let reason = format_permission_reason(tool_name, description, path, level);
                PermissionDecision::Ask { reason }
            }
        }
    }

    // ----------------------------------------------------------------
    // Rule management
    // ----------------------------------------------------------------

    /// Add an arbitrary rule to this manager.
    pub fn add_rule(&mut self, rule: PermissionRule) {
        match rule.scope {
            PermissionScope::Session => self.session_rules.push(rule),
            PermissionScope::Persistent => self.persistent_rules.push(rule),
        }
    }

    /// Allow `tool_name` for the rest of this session.
    pub fn add_session_allow(&mut self, tool_name: &str) {
        self.session_rules.push(PermissionRule {
            tool_name: Some(tool_name.to_string()),
            path_pattern: None,
            action: PermissionAction::Allow,
            scope: PermissionScope::Session,
        });
    }

    /// Allow `tool_name` on `path` (glob) for the rest of this session.
    pub fn add_session_allow_path(&mut self, tool_name: &str, path: &str) {
        self.session_rules.push(PermissionRule {
            tool_name: Some(tool_name.to_string()),
            path_pattern: Some(path.to_string()),
            action: PermissionAction::Allow,
            scope: PermissionScope::Session,
        });
    }

    /// Allow `tool_name` persistently and save to settings.
    pub async fn add_persistent_allow(
        &mut self,
        tool_name: &str,
        settings: &mut crate::config::Settings,
    ) -> boxagnts_core::error::Result<()> {
        let rule = PermissionRule {
            tool_name: Some(tool_name.to_string()),
            path_pattern: None,
            action: PermissionAction::Allow,
            scope: PermissionScope::Persistent,
        };
        let serialized = SerializedPermissionRule::from(&rule);
        settings.permission_rules.push(serialized);
        settings
            .save()
            .await
            .map_err(|e| boxagnts_core::error::ClaudeError::Config(e.to_string()))?;
        self.persistent_rules.push(rule);
        Ok(())
    }

    /// Allow `tool_name` persistently on `path` and save settings.
    pub async fn add_persistent_allow_path(
        &mut self,
        tool_name: &str,
        path: &str,
        settings: &mut crate::config::Settings,
    ) -> boxagnts_core::error::Result<()> {
        let rule = PermissionRule {
            tool_name: Some(tool_name.to_string()),
            path_pattern: Some(path.to_string()),
            action: PermissionAction::Allow,
            scope: PermissionScope::Persistent,
        };
        let serialized = SerializedPermissionRule::from(&rule);
        settings.permission_rules.push(serialized);
        settings
            .save()
            .await
            .map_err(|e| boxagnts_core::error::ClaudeError::Config(e.to_string()))?;
        self.persistent_rules.push(rule);
        Ok(())
    }

    /// Remove a persistent rule by index and save settings.
    pub async fn remove_rule(
        &mut self,
        idx: usize,
        settings: &mut crate::config::Settings,
    ) -> boxagnts_core::error::Result<()> {
        if idx >= settings.permission_rules.len() {
            return Err(boxagnts_core::error::ClaudeError::Config(format!(
                "Rule index {} out of bounds",
                idx
            )));
        }
        settings.permission_rules.remove(idx);
        settings
            .save()
            .await
            .map_err(|e| boxagnts_core::error::ClaudeError::Config(e.to_string()))?;
        // Rebuild persistent_rules from the updated settings
        self.persistent_rules = settings
            .permission_rules
            .iter()
            .map(PermissionRule::from)
            .collect();
        Ok(())
    }

    // ----------------------------------------------------------------
    // Bridge / async pending permissions
    // ----------------------------------------------------------------

    /// Register a pending permission and return a receiver.  The caller
    /// awaits the receiver and gets a `PermissionDecision` when the user
    /// (or a bridge peer) resolves the request.
    pub fn register_pending(
        &mut self,
        id: String,
    ) -> tokio::sync::oneshot::Receiver<PermissionDecision> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.pending.push(PendingPermission {
            tool_use_id: id,
            created_at: std::time::Instant::now(),
            resolve_tx: tx,
        });
        rx
    }

    /// Resolve a pending permission by `tool_use_id`, delivering
    /// `decision` to the waiting receiver.  No-op if the ID is unknown.
    pub fn resolve_pending(&mut self, id: &str, decision: PermissionDecision) {
        if let Some(pos) = self.pending.iter().position(|p| p.tool_use_id == id) {
            let pending = self.pending.remove(pos);
            let _ = pending.resolve_tx.send(decision);
        }
    }
}

// -----------------------------------------------------------------------
// PermissionRequest (passed to handlers & TUI)
// -----------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct PermissionRequest {
    pub tool_name: String,
    pub description: String,
    pub details: Option<String>,
    pub is_read_only: bool,
    /// Canonical or resolved target path when the permission decision is path-sensitive.
    pub path: Option<String>,
    /// Current workspace root used for path-boundary checks.
    pub working_dir: Option<std::path::PathBuf>,
    /// Additional workspace roots considered in-bounds for file access.
    pub allowed_roots: Vec<std::path::PathBuf>,
    /// Context-aware description showing user WHY the tool needs permission.
    /// E.g. "bash: execute `ls -la /home`", "write file: /path/to/.bashrc", "fetch: https://example.com"
    pub context_description: Option<String>,
}

// -----------------------------------------------------------------------
// PermissionHandler trait + handlers
// -----------------------------------------------------------------------

/// Trait implemented by anything that can decide whether to allow a tool.
pub trait PermissionHandler: Send + Sync {
    fn check_permission(&self, request: &PermissionRequest) -> PermissionDecision;
    fn request_permission(&self, request: &PermissionRequest) -> PermissionDecision;
}

/// Handler for non-interactive / headless modes.
///
/// Uses simple mode-based rules.  For rule-based evaluation backed by a
/// `PermissionManager`, use `ManagedAutoPermissionHandler` instead.
pub struct AutoPermissionHandler {
    pub mode: crate::config::PermissionMode,
}

impl PermissionHandler for AutoPermissionHandler {
    fn check_permission(&self, request: &PermissionRequest) -> PermissionDecision {
        use crate::config::PermissionMode;
        match self.mode {
            PermissionMode::BypassPermissions => PermissionDecision::Allow,
            PermissionMode::AcceptEdits => {
                if request.tool_name == "Edit" {
                    PermissionDecision::Allow
                } else if request.is_read_only {
                    PermissionDecision::Allow
                } else {
                    PermissionDecision::Deny
                }
            }
            PermissionMode::Plan => {
                if request.is_read_only {
                    PermissionDecision::Allow
                } else {
                    PermissionDecision::Deny
                }
            }
            PermissionMode::Default => {
                if request.is_read_only {
                    PermissionDecision::Allow
                } else {
                    PermissionDecision::Deny
                }
            }
        }
    }

    fn request_permission(&self, request: &PermissionRequest) -> PermissionDecision {
        self.check_permission(request)
    }
}

/// Permission handler for interactive (TUI) mode.
///
/// Uses simple mode-based rules.  For rule-based evaluation backed by a
/// `PermissionManager`, use `ManagedInteractivePermissionHandler`.
pub struct InteractivePermissionHandler {
    pub mode: crate::config::PermissionMode,
}

impl PermissionHandler for InteractivePermissionHandler {
    fn check_permission(&self, request: &PermissionRequest) -> PermissionDecision {
        use crate::config::PermissionMode;
        match self.mode {
            PermissionMode::Plan => {
                if request.is_read_only {
                    PermissionDecision::Allow
                } else {
                    PermissionDecision::Deny
                }
            }
            // In Default / AcceptEdits / BypassPermissions the user is
            // watching the TUI so we allow all.
            _ => PermissionDecision::Allow,
        }
    }

    fn request_permission(&self, request: &PermissionRequest) -> PermissionDecision {
        self.check_permission(request)
    }
}

// ---- Manager-backed handlers -----------------------------------------

/// Non-interactive handler backed by a shared `PermissionManager`.
///
/// Delegates to `PermissionManager::evaluate`; converts `Ask` decisions
/// into `Deny` (no interactive prompt available in headless mode).
pub struct ManagedAutoPermissionHandler {
    pub manager: Arc<Mutex<PermissionManager>>,
}

impl ManagedAutoPermissionHandler {
    pub fn new(manager: Arc<Mutex<PermissionManager>>) -> Self {
        Self { manager }
    }
}

impl PermissionHandler for ManagedAutoPermissionHandler {
    fn check_permission(&self, request: &PermissionRequest) -> PermissionDecision {
        if let Ok(m) = self.manager.lock() {
            let decision = m.evaluate(
                &request.tool_name,
                &request.description,
                request.path.as_deref(),
                request.working_dir.as_deref(),
                &request.allowed_roots,
            );
            return match decision {
                PermissionDecision::Ask { .. } => PermissionDecision::Deny,
                other => other,
            };
        }
        PermissionDecision::Deny
    }

    fn request_permission(&self, request: &PermissionRequest) -> PermissionDecision {
        self.check_permission(request)
    }
}

/// Interactive (TUI) handler backed by a shared `PermissionManager`.
///
/// Delegates to `PermissionManager::evaluate`; passes `Ask` decisions
/// through so the TUI dialog can display them.
pub struct ManagedInteractivePermissionHandler {
    pub manager: Arc<Mutex<PermissionManager>>,
}

impl ManagedInteractivePermissionHandler {
    pub fn new(manager: Arc<Mutex<PermissionManager>>) -> Self {
        Self { manager }
    }
}

impl PermissionHandler for ManagedInteractivePermissionHandler {
    fn check_permission(&self, request: &PermissionRequest) -> PermissionDecision {
        if let Ok(m) = self.manager.lock() {
            return m.evaluate(
                &request.tool_name,
                &request.description,
                request.path.as_deref(),
                request.working_dir.as_deref(),
                &request.allowed_roots,
            );
        }
        // If the lock is poisoned fall back to allow (user is watching)
        PermissionDecision::Allow
    }

    fn request_permission(&self, request: &PermissionRequest) -> PermissionDecision {
        self.check_permission(request)
    }
}

// Convenience constructor aliases used by the spec
impl InteractivePermissionHandler {
    /// Build a manager-backed interactive handler.
    pub fn with_manager(
        manager: Arc<Mutex<PermissionManager>>,
    ) -> ManagedInteractivePermissionHandler {
        ManagedInteractivePermissionHandler::new(manager)
    }
}

impl AutoPermissionHandler {
    /// Build a manager-backed auto handler.
    pub fn with_manager(manager: Arc<Mutex<PermissionManager>>) -> ManagedAutoPermissionHandler {
        ManagedAutoPermissionHandler::new(manager)
    }
}

// -----------------------------------------------------------------------
// Unit tests
// -----------------------------------------------------------------------

#[cfg(test)]
mod perm_tests {
    use super::*;
    use crate::config::{PermissionMode, Settings};

    fn mgr(mode: PermissionMode) -> PermissionManager {
        PermissionManager::new(mode, &Settings::default())
    }

    #[test]
    fn bypass_always_allows() {
        let m = mgr(PermissionMode::BypassPermissions);
        assert_eq!(
            m.evaluate("Bash", "rm -rf /", None, None, &[]),
            PermissionDecision::Allow
        );
    }

    #[test]
    fn default_read_allows_workspace_paths() {
        let m = mgr(PermissionMode::Default);
        let cwd = std::path::Path::new("/workspace");
        assert_eq!(
            m.evaluate(
                "Read",
                "read file",
                Some("/workspace/src/lib.rs"),
                Some(cwd),
                &[],
            ),
            PermissionDecision::Allow
        );
    }

    #[test]
    fn default_read_asks_outside_workspace() {
        let m = mgr(PermissionMode::Default);
        let cwd = std::path::Path::new("/workspace");
        match m.evaluate(
            "Read",
            "read file",
            Some("/tmp/outside.txt"),
            Some(cwd),
            &[],
        ) {
            PermissionDecision::Ask { .. } => {}
            other => panic!("Expected Ask, got {:?}", other),
        }
    }

    #[test]
    fn default_read_allows_additional_workspace_roots() {
        let m = mgr(PermissionMode::Default);
        let cwd = std::path::Path::new("/workspace");
        let extra = vec![std::path::PathBuf::from("/external")];
        assert_eq!(
            m.evaluate(
                "Read",
                "read file",
                Some("/external/notes.txt"),
                Some(cwd),
                &extra,
            ),
            PermissionDecision::Allow
        );
    }

    #[test]
    fn default_bash_asks() {
        let m = mgr(PermissionMode::Default);
        match m.evaluate("Bash", "echo hello", None, None, &[]) {
            PermissionDecision::Ask { .. } => {}
            other => panic!("Expected Ask, got {:?}", other),
        }
    }

    #[test]
    fn session_allow_overrides_default() {
        let mut m = mgr(PermissionMode::Default);
        m.add_session_allow("Bash");
        assert_eq!(
            m.evaluate("Bash", "echo hi", None, None, &[]),
            PermissionDecision::Allow
        );
    }

    #[test]
    fn deny_beats_allow() {
        let mut m = mgr(PermissionMode::Default);
        m.add_session_allow("Bash");
        m.add_rule(PermissionRule {
            tool_name: Some("Bash".to_string()),
            path_pattern: None,
            action: PermissionAction::Deny,
            scope: PermissionScope::Session,
        });
        assert_eq!(
            m.evaluate("Bash", "echo hi", None, None, &[]),
            PermissionDecision::Deny
        );
    }

    #[test]
    fn plan_denies_writes() {
        let m = mgr(PermissionMode::Plan);
        assert_eq!(
            m.evaluate("Write", "write file", Some("/tmp/foo"), None, &[]),
            PermissionDecision::Deny
        );
    }

    #[test]
    fn plan_allows_reads() {
        let m = mgr(PermissionMode::Plan);
        assert_eq!(
            m.evaluate("Read", "read file", Some("/tmp/foo"), None, &[]),
            PermissionDecision::Allow
        );
    }

    #[test]
    fn accept_edits_only_allows_edit() {
        let m = mgr(PermissionMode::AcceptEdits);
        assert_eq!(
            m.evaluate(
                "Edit",
                "edit file",
                Some("/workspace/src/lib.rs"),
                None,
                &[]
            ),
            PermissionDecision::Allow
        );
        match m.evaluate("Bash", "rm -rf /tmp", None, None, &[]) {
            PermissionDecision::Ask { .. } => {}
            other => panic!("Expected Ask, got {:?}", other),
        }
    }

    #[test]
    fn glob_path_allow_matches() {
        let mut m = mgr(PermissionMode::Default);
        m.add_rule(PermissionRule {
            tool_name: Some("Write".to_string()),
            path_pattern: Some("/tmp/**".to_string()),
            action: PermissionAction::Allow,
            scope: PermissionScope::Session,
        });
        assert_eq!(
            m.evaluate("Write", "write", Some("/tmp/foo/bar.txt"), None, &[]),
            PermissionDecision::Allow
        );
    }

    #[test]
    fn glob_path_no_match_asks() {
        let mut m = mgr(PermissionMode::Default);
        m.add_rule(PermissionRule {
            tool_name: Some("Write".to_string()),
            path_pattern: Some("/tmp/**".to_string()),
            action: PermissionAction::Allow,
            scope: PermissionScope::Session,
        });
        match m.evaluate("Write", "write", Some("/etc/hosts"), None, &[]) {
            PermissionDecision::Ask { .. } => {}
            other => panic!("Expected Ask, got {:?}", other),
        }
    }

    #[test]
    fn format_reason_bash() {
        let s = format_permission_reason(
            "Bash",
            "This will execute a shell command.",
            None,
            PermissionLevel::Execute,
        );
        assert_eq!(s, "This will execute a shell command.");
    }

    #[test]
    fn format_reason_powershell() {
        let s = format_permission_reason(
            "PowerShell",
            "[High risk] This may modify system-wide security policy.",
            None,
            PermissionLevel::Execute,
        );
        assert_eq!(
            s,
            "[High risk] This may modify system-wide security policy."
        );
    }

    #[test]
    fn format_reason_write_etc() {
        let s =
            format_permission_reason("Write", "write", Some("/etc/hosts"), PermissionLevel::Write);
        assert!(s.contains("/etc/hosts"));
        assert!(s.contains("system files"));
    }

    #[test]
    fn format_reason_webfetch() {
        let s = format_permission_reason(
            "WebFetch",
            "fetch",
            Some("https://example.com"),
            PermissionLevel::Network,
        );
        assert!(s.contains("https://example.com"));
        assert!(s.contains("HTTP request"));
    }
}
