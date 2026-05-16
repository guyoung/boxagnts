use std::sync::Arc;

use async_trait::async_trait;
use tracing::info;

use boxagnts_core::types::ToolDefinition;
use boxagnts_workspace::config::Config;
use boxagnts_tools::{PermissionLevel, Tool, ToolContext, ToolResult};

/// MCP tool wrapper: makes MCP server tools look like native cc-tools.
pub struct McpToolWrapper {
    pub tool_def: ToolDefinition,
    pub server_name: String,
    pub manager: Arc<boxagnts_mcp::McpManager>,
}

#[async_trait]
impl Tool for McpToolWrapper {
    fn name(&self) -> &str {
        &self.tool_def.name
    }

    fn description(&self) -> &str {
        &self.tool_def.description
    }

    fn permission_level(&self) -> PermissionLevel {
        // MCP tools run external processes – treat as Execute.
        PermissionLevel::Execute
    }

    fn input_schema(&self) -> serde_json::Value {
        self.tool_def.input_schema.clone()
    }

    async fn execute(&self, input: serde_json::Value, _ctx: &ToolContext) -> ToolResult {
        // Strip the server-name prefix to get the bare tool name.
        let prefix = format!("{}_", self.server_name);
        let bare_name = self
            .tool_def
            .name
            .strip_prefix(&prefix)
            .unwrap_or(&self.tool_def.name);

        let args = if input.is_null() { None } else { Some(input) };

        match self.manager.call_tool(&self.tool_def.name, args).await {
            Ok(result) => {
                let text = boxagnts_mcp::mcp_result_to_string(&result);
                if result.is_error {
                    ToolResult::error(text)
                } else {
                    ToolResult::success(text)
                }
            }
            Err(e) => ToolResult::error(format!("MCP tool '{}' failed: {}", bare_name, e)),
        }
    }
}

pub async fn connect_mcp_manager_arc(config: &Config) -> Option<Arc<boxagnts_mcp::McpManager>> {
    if config.mcp_servers.is_empty() {
        return None;
    }

    info!(
        count = config.mcp_servers.len(),
        "Connecting to MCP servers"
    );
    let mcp_manager = boxagnts_mcp::McpManager::connect_all(&config.mcp_servers).await;
    Some(Arc::new(mcp_manager))
}
