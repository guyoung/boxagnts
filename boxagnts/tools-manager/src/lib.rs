use serde_json::json;

use boxagnts_tools::{PermissionLevel, Tool};

/// Return all built-in tools (excluding AgentTool, which lives in cc-query).
pub fn all_tools() -> Vec<Box<dyn Tool>> {
    vec![

        // boxagnts-tools
        Box::new(boxagnts_tools::ask_user::AskUserQuestionTool),
        Box::new(boxagnts_tools::brief::BriefTool),
        Box::new(boxagnts_tools::plan_mode::EnterPlanModeTool),
        Box::new(boxagnts_tools::plan_mode::ExitPlanModeTool),
        Box::new(boxagnts_tools::sleep::SleepTool),
        Box::new(boxagnts_tools::mcp::ListMcpResourcesTool),
        Box::new(boxagnts_tools::mcp::ReadMcpResourceTool),
        Box::new(boxagnts_tools::mcp::McpAuthTool),
        Box::new(boxagnts_tools::skill::SkillTool),
        Box::new(boxagnts_tools::tool_search::ToolSearchTool),

        // wasm-tools
        Box::new(boxagnts_wasm_tools::wasm_tool::WasmTool::new(
            "read".to_string(),
            "file-read-component.wasm".to_string(),
            "Reads a file from the local filesystem using wasm. You can access any file directly. \
         By default reads up to 2000 lines from the beginning. Results are returned \
         with line numbers starting at 1. This tool can read images (PNG, JPG) and \
         PDF files."
                .to_string(),
            PermissionLevel::ReadOnly,
            json!({
                "type": "object",
                "properties": {
                    "file_path": {
                        "type": "string",
                        "description": "The absolute path to the file to read"
                    },
                    "offset": {
                        "type": "number",
                        "description": "The line number to start reading from (1-based). Only provide if the file is too large to read at once."
                    },
                    "limit": {
                        "type": "number",
                        "description": "The number of lines to read. Only provide if the file is too large to read at once."
                    }
                },
                "required": ["file_path"]
            }),
        )),
        Box::new(boxagnts_wasm_tools::wasm_tool::WasmTool::new(
            "write".to_string(),
            "file-write-component.wasm".to_string(),
            "Writes a file to the local filesystem using wasm. This tool will overwrite the existing \
         file if there is one. Prefer the Edit tool for modifying existing files. \
         Only use this tool to create new files or for complete rewrites."
                .to_string(),
            PermissionLevel::Write,
            json!({
                "type": "object",
                "properties": {
                    "file_path": {
                        "type": "string",
                        "description": "The absolute path to the file to write"
                    },
                    "content": {
                        "type": "string",
                        "description": "The content to write to the file"
                    }
                },
                "required": ["file_path", "content"]
            }),
        )),
        Box::new(boxagnts_wasm_tools::wasm_tool::WasmTool::new(
            "edit".to_string(),
            "file-edit-component.wasm".to_string(),
            "Performs exact string replacements in files using wasm. The edit will FAIL if \
         `old_string` is not unique in the file (unless `replace_all` is true). \
         You MUST read the file first before editing. Preserve the exact \
         indentation as it appears in the file."
                .to_string(),
            PermissionLevel::Write,
            json!({
                "type": "object",
                "properties": {
                    "file_path": {
                        "type": "string",
                        "description": "The absolute path to the file to modify"
                    },
                    "old_string": {
                        "type": "string",
                        "description": "The text to replace (must be unique in the file unless replace_all is true)"
                    },
                    "new_string": {
                        "type": "string",
                        "description": "The text to replace it with (must be different from old_string)"
                    },
                    "replace_all": {
                        "type": "boolean",
                        "description": "Replace all occurrences of old_string (default false)"
                    }
                },
                "required": ["file_path", "old_string", "new_string"]
            }),
        )),
        Box::new(boxagnts_wasm_tools::wasm_tool::WasmTool::new(
            "glob".to_string(),
            "file-glob-component.wasm".to_string(),
            "Fast file pattern matching tool that works with any codebase size using wasm. \
         Supports glob patterns like \"**/*.rs\" or \"src/**/*.ts\". Returns \
         matching file paths sorted by modification time. Use this tool when \
         you need to find files by name patterns."
                .to_string(),
            PermissionLevel::ReadOnly,
            json!({
               "type": "object",
                "properties": {
                    "pattern": {
                        "type": "string",
                        "description": "The glob pattern to match files against"
                    },
                    "path": {
                        "type": "string",
                        "description": "The directory to search in. Defaults to working directory."
                    }
                },
                "required": ["pattern"]
            }),
        )),
        Box::new(boxagnts_wasm_tools::wasm_tool::WasmTool::new(
            "grep".to_string(),
            "file-grep-component.wasm".to_string(),
            "A powerful search tool built on regex using wasm. Supports full regex syntax. \
Filter files with the `glob` parameter or `type` parameter. Output \
modes: \"content\" shows matching lines, \"files_with_matches\" shows \
only file paths (default), \"count\" shows match counts."
                .to_string(),
            PermissionLevel::ReadOnly,
            json!({
               "type": "object",
                "properties": {
                    "pattern": {
                        "type": "string",
                        "description": "The regular expression pattern to search for"
                    },
                    "path": {
                        "type": "string",
                        "description": "File or directory to search in. Defaults to working directory."
                    },
                    "type": {
                        "type": "string",
                        "description": "File type to search (e.g. js, py, rust, go)"
                    },
                    "glob": {
                        "type": "string",
                        "description": "Glob pattern to filter files (e.g. \"*.js\")"
                    },
                    "output_mode": {
                        "type": "string",
                        "enum": ["content", "files_with_matches", "count"],
                        "description": "Output mode (default: files_with_matches)"
                    },
                    "context": {
                        "type": "number",
                        "description": "Number of context lines before and after each match"
                    },
                    "case_insensitive": {
                        "type": "boolean",
                        "description": "Case insensitive search"
                    },
                    "show_line_numbers": {
                        "type": "boolean",
                        "description": "Show line numbers (for content mode)"
                    },
                    "head_limit": {
                        "type": "number",
                        "description": "Limit output to first N entries (default 250)"
                    },
                    "multiline": {
                        "type": "boolean",
                        "description": "Enable multiline mode where . matches newlines"
                    }
                },
                "required": ["pattern"]
            }),
        )),

        Box::new(boxagnts_wasm_tools::wasm_tool::WasmTool::new(
            "web-fetch".to_string(),
            "web-fetch-component.wasm".to_string(),
            "Fetches a web page URL and returns its content as text using wasm. HTML is \
         automatically converted to plain text. Use this for reading documentation, \
         APIs, and other web resources."
                .to_string(),
            PermissionLevel::Write,
            json!({
                "type": "object",
                "properties": {
                    "url": {
                        "type": "string",
                        "description": "URL to fetch"
                    },
                    "max_chars": {
                        "type": "number",
                        "description": "Max characters to return from fetched content"
                    },
                    "max_chars_cap": {
                        "type": "number",
                        "description": "Hard cap for maxChars"
                    },
                    "max_response_bytes": {
                        "type": "number",
                        "description": "Max download size before truncation (bytes)"
                    },
                     "timeout_seconds": {
                        "type": "number",
                        "description": "Timeout in seconds for fetch requests"
                    },
                    "max_redirects": {
                        "type": "number",
                        "description": "Maximum number of redirects to follow"
                    },
                    "user_agent": {
                        "type": "string",
                        "description": "Override User-Agent header for fetch requests"
                    },
                    "readability": {
                        "type": "boolean",
                        "description": "Use Mozilla Readability algorithm to extract main content"
                    },
                    "extract_mode": {
                        "type": "string",
                        "description": "Extract mode: text or markdown"
                    },


                },
                "required": ["url"]
            }),
        )),

        Box::new(boxagnts_wasm_tools::wasm_tool::WasmTool::new(
            "bash".to_string(),
            "bash-component.wasm".to_string(),
            "Executes a given bash command and returns its output using wasm. The working directory \
persists between commands, but shell state does not.  Avoid using interactive commands. \
Use this tool for running shell commands.Support shell commands: \
arch,b2sum,base32,base64,basename,basenc,cat,cksum,comm,cp,csplit,curl,cut,date,dir, \
dircolors,dirname,echo,expand,factor,false,fmt,fold,head,join,link,ln,ls,md5sum, mkdir, \
mv,nl,nproc,numfmt,od,paste,pathchk,pr,printenv,printf,ptx,pwd,readlink,realpath,rm, rmdir, \
seq,sha1sum,sha224sum,sha256sum,sha384sum,sha512sum,shred,shuf,sleep,sort,split,sum,tail, \
tar,tee,touch,tr,true,truncate,tsort,uname,unexpand,uniq,unlink,vdir,wc,yes"
                .to_string(),
            PermissionLevel::Execute,
            json!({
                "type": "object",
                "properties": {
                    "command": {
                        "type": "string",
                        "description": "The bash command to execute"
                    },
                },
                "required": ["command"]
            }),

        )),

        Box::new(boxagnts_wasm_tools::wasm_tool::WasmTool::new(
            "jsexec".to_string(),
            "boxedjs-execute-component.wasm".to_string(),
            "Execute JavaScript code using wasm.Largely compatible with Node.js built-in modules and behavior. \
The entry point of JavaScript must be the main function, and arguments can be specified."
                .to_string(),
            PermissionLevel::Execute,
            json!({
                "type": "object",
                "properties": {
                    "code": {
                        "type": "string",
                        "description": "the js code to execute"
                    },
                    "file_path": {
                        "type": "string",
                        "description": "The absolute path to the js file to execute"
                    },

                },
                "required": []
            }),

        )),
    ]
}

/// Find a tool by name (case-sensitive).
pub fn find_tool(name: &str) -> Option<Box<dyn Tool>> {
    all_tools().into_iter().find(|t| t.name() == name)
}
