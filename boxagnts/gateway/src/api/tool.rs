use std::sync::Arc;

use tracing::debug;

use std::collections::HashMap;

use serde::{ Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolConfig {
    pub name: String,
    pub description: String,
}


pub async fn load_tools() -> anyhow::Result<HashMap<String, ToolConfig>> {
    let mut tools: HashMap<String, ToolConfig> = HashMap::new();

    let all_tools: Vec<Box<dyn boxagnts_tools::Tool>> = boxagnts_tools_manager::all_tools();

    for t in all_tools {
        let tool = ToolConfig {
            name: t.name().to_string(),
            description: t.description().to_string(),
        };

        tools.insert(t.name().to_string(), tool);
    }

    Ok(tools)
}



pub fn build_tools_with_mcp(
    mcp_manager: Option<Arc<boxagnts_mcp::McpManager>>,
) -> Arc<Vec<Box<dyn boxagnts_tools::Tool>>> {
    let mut v: Vec<Box<dyn boxagnts_tools::Tool>> = boxagnts_tools_manager::all_tools();
    /***
    v.push(Box::new(boxagnts_tools::AgentTool));
    ***/

    if let Some(ref manager_arc) = mcp_manager {
        for (server_name, tool_def) in manager_arc.all_tool_definitions() {
            let wrapper = crate::api::mcp::McpToolWrapper {
                tool_def,
                server_name,
                manager: manager_arc.clone(),
            };
            v.push(Box::new(wrapper));
        }
        debug!(total_tools = v.len(), "MCP tools registered");
    }

    Arc::new(v)
}

/// Filter the tool list based on the agent's access level.
/// - "full"        → all tools allowed (no filtering)
/// - "read-only"   → only ReadOnly/None permission tools and AskUserQuestion
/// - "search-only" → only Grep, Glob, Read, WebSearch, WebFetch tools
#[allow(dead_code)]
pub fn filter_tools_for_agent(
    tools: Arc<Vec<Box<dyn boxagnts_tools::Tool>>>,
    access: &str,
) -> Arc<Vec<Box<dyn boxagnts_tools::Tool>>> {
    use boxagnts_tools::PermissionLevel as PL;
    match access {
        "read-only" => {
            // Collect names of tools that are read-only, then rebuild from all_tools
            // (Box<dyn Tool> is not Clone so we can't directly filter-and-keep).
            let allowed_names: Vec<String> = tools
                .iter()
                .filter(|t| {
                    matches!(t.permission_level(), PL::ReadOnly | PL::None)
                        || t.name() == "AskUserQuestion"
                })
                .map(|t| t.name().to_string())
                .collect();
            let filtered: Vec<Box<dyn boxagnts_tools::Tool>> = boxagnts_tools_manager::all_tools()
                .into_iter()
                .filter(|t| allowed_names.iter().any(|n| n == t.name()))
                .collect();
            Arc::new(filtered)
        }
        "search-only" => {
            const SEARCH_TOOLS: &[&str] = &["Grep", "Glob", "Read", "WebSearch", "WebFetch"];
            let filtered: Vec<Box<dyn boxagnts_tools::Tool>> = boxagnts_tools_manager::all_tools()
                .into_iter()
                .filter(|t| SEARCH_TOOLS.contains(&t.name()))
                .collect();
            Arc::new(filtered)
        }
        _ => tools, // "full" — allow all tools unchanged
    }
}