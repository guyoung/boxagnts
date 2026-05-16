use serde::{Deserialize, Serialize};
use serde_json::Value;

// ---- Roles -----------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,

}

impl Role {
    pub fn is_assistant(&self) -> bool {
        todo!()
    }
}
// ---- Content blocks --------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    Text {
        text: String,
    },
    Image {
        source: ImageSource,
    },
    ToolUse {
        id: String,
        name: String,
        input: Value,
    },
    ToolResult {
        tool_use_id: String,
        content: ToolResultContent,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_error: Option<bool>,
    },
    Thinking {
        thinking: String,
        signature: String,
    },
    RedactedThinking {
        data: String,
    },
    Document {
        source: DocumentSource,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        context: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        citations: Option<CitationsConfig>,
    },
    /// A `!`-prefixed shell command invoked by the user, with its captured output.
    /// Rendered as a faint gray block with a `!command` header.
    UserLocalCommandOutput {
        command: String,
        output: String,
    },
    /// A skill/slash-command invocation entered by the user.
    /// Rendered as `▸ name args` with cyan styling.
    UserCommand {
        name: String,
        args: String,
    },
    /// A memory key/value written by the user (e.g. via `/memory`).
    /// Rendered as `# key: value` in cyan with a `Got it.` footer.
    UserMemoryInput {
        key: String,
        value: String,
    },
    /// A system-level API error, rendered as a red-bordered block.
    /// Shows first 5 lines with `[expand]` hint when truncated, and an
    /// optional `Retrying in Ns...` countdown line when `retry_secs` is set.
    SystemAPIError {
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        retry_secs: Option<u32>,
    },
    /// A collapsed summary of multiple read/search tool calls.
    /// Rendered as `▸ Read N files (+ M more)` on a single line.
    CollapsedReadSearch {
        tool_name: String,
        paths: Vec<String>,
        n_hidden: usize,
    },
    /// A sub-task assignment in an agentic workflow.
    /// Rendered as a cyan-bordered box with Task ID, subject, and description.
    TaskAssignment {
        id: String,
        subject: String,
        description: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolResultContent {
    Text(String),
    Blocks(Vec<ContentBlock>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageSource {
    #[serde(rename = "type")]
    pub source_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentSource {
    #[serde(rename = "type")]
    pub source_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationsConfig {
    pub enabled: bool,
}

// ---- Messages --------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: MessageContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<MessageCost>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContent {
    Text(String),
    Blocks(Vec<ContentBlock>),
}

impl Message {
    /// Create a simple user text message.
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: MessageContent::Text(content.into()),
            uuid: None,
            cost: None,
        }
    }

    /// Create a user message composed of multiple content blocks.
    pub fn user_blocks(blocks: Vec<ContentBlock>) -> Self {
        Self {
            role: Role::User,
            content: MessageContent::Blocks(blocks),
            uuid: None,
            cost: None,
        }
    }

    /// Create a simple assistant text message.
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: MessageContent::Text(content.into()),
            uuid: None,
            cost: None,
        }
    }

    /// Create an assistant message composed of multiple content blocks.
    pub fn assistant_blocks(blocks: Vec<ContentBlock>) -> Self {
        Self {
            role: Role::Assistant,
            content: MessageContent::Blocks(blocks),
            uuid: None,
            cost: None,
        }
    }

    /// Extract the first text content from this message.
    pub fn get_text(&self) -> Option<&str> {
        match &self.content {
            MessageContent::Text(t) => Some(t.as_str()),
            MessageContent::Blocks(blocks) => blocks.iter().find_map(|b| {
                if let ContentBlock::Text { text } = b {
                    Some(text.as_str())
                } else {
                    None
                }
            }),
        }
    }

    /// Collect all text content blocks into one concatenated string.
    pub fn get_all_text(&self) -> String {
        match &self.content {
            MessageContent::Text(t) => t.clone(),
            MessageContent::Blocks(blocks) => blocks
                .iter()
                .filter_map(|b| {
                    if let ContentBlock::Text { text } = b {
                        Some(text.as_str())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .join(""),
        }
    }

    /// Return references to all `ToolUse` blocks in this message.
    pub fn get_tool_use_blocks(&self) -> Vec<&ContentBlock> {
        match &self.content {
            MessageContent::Blocks(blocks) => blocks
                .iter()
                .filter(|b| matches!(b, ContentBlock::ToolUse { .. }))
                .collect(),
            _ => vec![],
        }
    }

    /// Return references to all `ToolResult` blocks in this message.
    pub fn get_tool_result_blocks(&self) -> Vec<&ContentBlock> {
        match &self.content {
            MessageContent::Blocks(blocks) => blocks
                .iter()
                .filter(|b| matches!(b, ContentBlock::ToolResult { .. }))
                .collect(),
            _ => vec![],
        }
    }

    /// Return references to all `Thinking` blocks in this message.
    pub fn get_thinking_blocks(&self) -> Vec<&ContentBlock> {
        match &self.content {
            MessageContent::Blocks(blocks) => blocks
                .iter()
                .filter(|b| matches!(b, ContentBlock::Thinking { .. }))
                .collect(),
            _ => vec![],
        }
    }

    /// Returns all content blocks (wrapping a single text into a vec).
    pub fn content_blocks(&self) -> Vec<ContentBlock> {
        match &self.content {
            MessageContent::Text(t) => vec![ContentBlock::Text { text: t.clone() }],
            MessageContent::Blocks(b) => b.clone(),
        }
    }

    /// Check whether this message has any tool use blocks.
    pub fn has_tool_use(&self) -> bool {
        !self.get_tool_use_blocks().is_empty()
    }

    /// Create a user message representing a `!`-prefixed local shell command with output.
    pub fn user_local_command_output(command: impl Into<String>, output: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: MessageContent::Blocks(vec![ContentBlock::UserLocalCommandOutput {
                command: command.into(),
                output: output.into(),
            }]),
            uuid: None,
            cost: None,
        }
    }

    /// Create a user message representing a skill/slash-command invocation.
    pub fn user_command(name: impl Into<String>, args: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: MessageContent::Blocks(vec![ContentBlock::UserCommand {
                name: name.into(),
                args: args.into(),
            }]),
            uuid: None,
            cost: None,
        }
    }

    /// Create a user message representing a memory key/value entry.
    pub fn user_memory_input(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: MessageContent::Blocks(vec![ContentBlock::UserMemoryInput {
                key: key.into(),
                value: value.into(),
            }]),
            uuid: None,
            cost: None,
        }
    }

    /// Create a system message representing an API error (red-bordered block).
    pub fn system_api_error(message: impl Into<String>, retry_secs: Option<u32>) -> Self {
        Self {
            role: Role::User,
            content: MessageContent::Blocks(vec![ContentBlock::SystemAPIError {
                message: message.into(),
                retry_secs,
            }]),
            uuid: None,
            cost: None,
        }
    }

    /// Create a system message representing a collapsed read/search summary.
    pub fn collapsed_read_search(
        tool_name: impl Into<String>,
        paths: Vec<String>,
        n_hidden: usize,
    ) -> Self {
        Self {
            role: Role::User,
            content: MessageContent::Blocks(vec![ContentBlock::CollapsedReadSearch {
                tool_name: tool_name.into(),
                paths,
                n_hidden,
            }]),
            uuid: None,
            cost: None,
        }
    }

    /// Create a system message representing a sub-task assignment.
    pub fn task_assignment(
        id: impl Into<String>,
        subject: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            role: Role::User,
            content: MessageContent::Blocks(vec![ContentBlock::TaskAssignment {
                id: id.into(),
                subject: subject.into(),
                description: description.into(),
            }]),
            uuid: None,
            cost: None,
        }
    }
}

// ---- Cost / usage ----------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MessageCost {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_creation_input_tokens: u64,
    pub cache_read_input_tokens: u64,
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UsageInfo {
    pub input_tokens: u64,
    pub output_tokens: u64,
    #[serde(default)]
    pub cache_creation_input_tokens: u64,
    #[serde(default)]
    pub cache_read_input_tokens: u64,
}

impl UsageInfo {
    pub fn total_input(&self) -> u64 {
        self.input_tokens + self.cache_creation_input_tokens + self.cache_read_input_tokens
    }

    pub fn total(&self) -> u64 {
        self.total_input() + self.output_tokens
    }
}