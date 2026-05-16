// ToolSearchTool: search for tools by name or keyword.
//
// This is used by the model to discover "deferred" tools that are not yet
// loaded into context. In the Rust port there is no deferred-tool mechanism
// (all tools are always available), but this tool still provides a useful
// search interface for the model to discover available capabilities.
//
// Supports two query modes:
//   - "select:ToolName"  → direct lookup by exact name
//   - "keyword search"   → fuzzy name + description match with scoring


use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{PermissionLevel, Tool, ToolContext, ToolResult};

pub struct ToolSearchTool;

#[derive(Debug, Deserialize)]
struct ToolSearchInput {
    query: String,
    #[serde(default = "default_max")]
    max_results: usize,
}

fn default_max() -> usize { 5 }

/// A minimal catalog entry describing one tool.
#[derive(Debug, Clone)]
struct ToolEntry {
    name: &'static str,
    description: &'static str,
    keywords: &'static [&'static str],
}

/// Static catalog of all built-in tools with keywords for scoring.
static TOOL_CATALOG: &[ToolEntry] = &[
    ToolEntry { name: "ask-user-question", description: "Ask the user a question", keywords: &["ask", "question", "user", "input", "clarify"] },
    ToolEntry { name: "brief", description: "Send a formatted message to the user", keywords: &["brief", "message", "notify", "proactive", "status", "update"] },
    ToolEntry { name: "list-mcp-resources", description: "List MCP server resources", keywords: &["mcp", "resource", "list", "server"] },
    ToolEntry { name: "read-mcp-resource", description: "Read an MCP resource", keywords: &["mcp", "resource", "read", "server"] },
    ToolEntry { name: "mcp-auth", description: "MCP Server Authorization", keywords: &["mcp", "server", "auth"] },
    ToolEntry { name: "enter-plan-mode", description: "Enter planning mode", keywords: &["plan", "mode", "planning"] },
    ToolEntry { name: "exit-plan-mode", description: "Exit planning mode", keywords: &["plan", "exit", "mode"] },
    ToolEntry { name: "sleep", description: "Wait for a duration", keywords: &["sleep", "wait", "delay", "pause"] },

    ToolEntry { name: "skill-tool", description: "Execute a skill prompt template", keywords: &["skill", "command", "template", "prompt", "slash", "custom"] },

    ToolEntry { name: "read", description: "Read file contents using wasm", keywords: &["file", "read", "cat", "content"] },
    ToolEntry { name: "write", description: "Write or create files using wasm", keywords: &["file", "write", "create", "save"] },
    ToolEntry { name: "edit", description: "Edit existing files with string replacement using wasm", keywords: &["file", "edit", "modify", "replace", "patch"] },
    ToolEntry { name: "glob", description: "Find files by pattern using wasm", keywords: &["find", "pattern", "search", "files", "glob"] },
    ToolEntry { name: "grep", description: "Search file contents with regex using wasm", keywords: &["search", "regex", "grep", "find", "content"] },
    ToolEntry { name: "web-fetch", description: "Fetch web page content using wasm", keywords: &["web", "fetch", "http", "url", "browser"] },
    ToolEntry { name: "bash", description: "Execute shell commands using wasm", keywords: &["shell", "run", "command", "exec", "terminal"] },
    ToolEntry { name: "jsexec", description: "Execute JavaScript code using wasm", keywords: &["js", "nodejs", "exec", "terminal"] },
];
#[async_trait]
impl Tool for ToolSearchTool {
    fn name(&self) -> &str { "tool-search" }

    fn description(&self) -> &str {
        "Search for available tools by name or keyword. Use 'select:ToolName' for direct \
         lookup or provide keywords for fuzzy search. Returns matching tool names and their \
         descriptions. Max 5 results by default."
    }

    fn permission_level(&self) -> PermissionLevel { PermissionLevel::None }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "Query: use 'select:ToolName' for direct selection, or keywords to search"
                },
                "max_results": {
                    "type": "number",
                    "description": "Maximum results to return (default: 5)"
                }
            },
            "required": ["query"]
        })
    }

    async fn execute(&self, input: Value, _ctx: &ToolContext) -> ToolResult {
        let params: ToolSearchInput = match serde_json::from_value(input) {
            Ok(p) => p,
            Err(e) => return ToolResult::error(format!("Invalid input: {}", e)),
        };

        let query = params.query.trim();
        let max = params.max_results.min(20);

        // select: prefix — direct lookup
        if let Some(names_str) = query.strip_prefix("select:").map(str::trim) {
            let requested: Vec<&str> = names_str.split(',').map(str::trim).collect();
            let mut found = Vec::new();
            let mut missing = Vec::new();

            for name in requested {
                if let Some(entry) = TOOL_CATALOG.iter().find(|e| {
                    e.name.eq_ignore_ascii_case(name)
                }) {
                    found.push(format!("{}: {}", entry.name, entry.description));
                } else {
                    missing.push(name.to_string());
                }
            }

            if found.is_empty() {
                return ToolResult::success(format!(
                    "No matching tools found for: {}",
                    missing.join(", ")
                ));
            }

            let mut out = found.join("\n");
            if !missing.is_empty() {
                out.push_str(&format!("\n\nNot found: {}", missing.join(", ")));
            }
            return ToolResult::success(out);
        }

        // Keyword search with scoring
        let q_lower = query.to_lowercase();
        let terms: Vec<&str> = q_lower.split_whitespace().collect();

        let mut scored: Vec<(usize, &ToolEntry)> = TOOL_CATALOG
            .iter()
            .filter_map(|entry| {
                let mut score = 0usize;
                let name_lower = entry.name.to_lowercase();
                let desc_lower = entry.description.to_lowercase();

                for term in &terms {
                    // Exact name match
                    if name_lower == *term {
                        score += 20;
                    } else if name_lower.contains(term) {
                        score += 10;
                    }

                    // Description match
                    if desc_lower.contains(term) {
                        score += 5;
                    }

                    // Keyword match
                    for &kw in entry.keywords {
                        if kw == *term {
                            score += 8;
                        } else if kw.contains(term) {
                            score += 3;
                        }
                    }
                }

                if score > 0 { Some((score, entry)) } else { None }
            })
            .collect();

        scored.sort_by(|a, b| b.0.cmp(&a.0));
        scored.truncate(max);

        if scored.is_empty() {
            return ToolResult::success(format!(
                "No tools found matching '{}'. Try broader keywords or use 'select:ToolName'.",
                query
            ));
        }

        let lines: Vec<String> = scored
            .iter()
            .map(|(_, e)| format!("{}: {}", e.name, e.description))
            .collect();

        ToolResult::success(format!(
            "Tools matching '{}':\n\n{}\n\nTotal tools available: {}",
            query,
            lines.join("\n"),
            TOOL_CATALOG.len()
        ))
    }
}
