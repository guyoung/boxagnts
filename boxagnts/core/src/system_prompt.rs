//! Modular system prompt assembly with caching support.
//!
//! cacheable (static) sections are placed before `SYSTEM_PROMPT_DYNAMIC_BOUNDARY`;
//! volatile, session-specific sections follow it.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Dynamic boundary marker
// ---------------------------------------------------------------------------

/// Marker that splits the cached vs dynamic parts of the system prompt.
/// Everything before this marker can be prompt-cached by the API.
/// Matches the TypeScript constant `SYSTEM_PROMPT_DYNAMIC_BOUNDARY`.
pub const SYSTEM_PROMPT_DYNAMIC_BOUNDARY: &str = "__SYSTEM_PROMPT_DYNAMIC_BOUNDARY__";


// ---------------------------------------------------------------------------
// Output style
// ---------------------------------------------------------------------------

/// Output styles that affect the system prompt.
/// Serialised as lowercase strings to match settings.json.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum OutputStyle {
    #[default]
    Default,
    Explanatory,
    Learning,
    Concise,
    Formal,
    Casual,
}

impl OutputStyle {
    /// Returns the system-prompt suffix for this style, or `None` for Default.
    pub fn prompt_suffix(self) -> Option<&'static str> {
        match self {
            OutputStyle::Explanatory => Some(
                "When explaining code or concepts, be thorough and educational. \
                Include reasoning, alternatives considered, and potential pitfalls. \
                Err on the side of over-explaining.",
            ),
            OutputStyle::Learning => Some(
                "This user is learning. Explain concepts as you implement them. \
                Point out patterns, best practices, and why you made each decision. \
                Use analogies when helpful.",
            ),
            OutputStyle::Concise => Some(
                "Be maximally concise. Skip preamble, summaries, and filler. \
                Lead with the answer. One sentence is better than three.",
            ),
            OutputStyle::Formal => Some(
                "Maintain a formal, professional tone. Use precise technical language.",
            ),
            OutputStyle::Casual => Some("Use a casual, conversational tone."),
            OutputStyle::Default => None,
        }
    }

    /// Parse from a string (case-insensitive).
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "explanatory" => Self::Explanatory,
            "learning" => Self::Learning,
            "concise" => Self::Concise,
            "formal" => Self::Formal,
            "casual" => Self::Casual,
            _ => Self::Default,
        }
    }
}



// ---------------------------------------------------------------------------
// Build options
// ---------------------------------------------------------------------------

/// All options controlling what goes into the assembled system prompt.
#[derive(Debug, Clone, Default)]
pub struct SystemPromptOptions {
    /// Whether the session is non-interactive (SDK / pipe mode).
    pub is_non_interactive: bool,
    /// Whether --append-system-prompt is set (affects prefix detection).
    pub has_append_system_prompt: bool,
    /// Output style to inject.
    pub output_style: OutputStyle,
    /// Optional custom output-style prompt loaded from disk or plugins.
    /// When present, this overrides the built-in enum-derived suffix.
    pub custom_output_style_prompt: Option<String>,
    /// Absolute path to the working directory (injected as dynamic section).
    pub working_directory: Option<String>,
    /// Pre-built memory content from memdir (injected as dynamic section).
    pub memory_content: String,
    /// Custom system prompt (--system-prompt flag or settings).
    pub custom_system_prompt: Option<String>,
    /// Additional text appended after everything else (--append-system-prompt).
    pub append_system_prompt: Option<String>,
    /// If true and `custom_system_prompt` is set, the entire default prompt is
    /// replaced — only the custom text + dynamic boundary are emitted.
    pub replace_system_prompt: bool,
    /// Inject the coordinator-mode section.
    pub coordinator_mode: bool,
    /// Skip auto-injecting platform/shell/date env info (set true only in tests).
    pub skip_env_info: bool,
}

// ---------------------------------------------------------------------------
// Main assembly function
// ---------------------------------------------------------------------------

/// Build the complete system prompt string.
///
/// The returned string contains `SYSTEM_PROMPT_DYNAMIC_BOUNDARY` as an
/// internal marker.  Callers (e.g. `buildSystemPromptBlocks` in cc-query)
/// split on this marker to determine which portions are eligible for
/// Anthropic prompt-caching.
pub fn build_system_prompt(opts: &SystemPromptOptions) -> String {
    // Replace mode: skip all default sections.
    if opts.replace_system_prompt {
        if let Some(custom) = &opts.custom_system_prompt {
            return format!("{}\n\n{}", custom, SYSTEM_PROMPT_DYNAMIC_BOUNDARY);
        }
    }



    let mut parts: Vec<String> = Vec::new();

    // ------------------------------------------------------------------ //
    // CACHEABLE sections (before the dynamic boundary)                   //
    // ------------------------------------------------------------------ //

    // Custom system prompt addition (appended to cacheable block)
    if let Some(custom) = &opts.custom_system_prompt {
        parts.push(format!(
            "{}\n\n",
            custom
        ));
    }

    // Output style (cacheable when non-Default; its content is stable)
    if let Some(style_text) = opts
        .custom_output_style_prompt
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .or_else(|| opts.output_style.prompt_suffix())
    {
        parts.push(format!("\n## Output Style\n{}", style_text));
    }

    // Dynamic boundary marker
    parts.push(SYSTEM_PROMPT_DYNAMIC_BOUNDARY.to_string());

    // ------------------------------------------------------------------ //
    // DYNAMIC / UNCACHEABLE sections (after the boundary)                //
    // ------------------------------------------------------------------ //

    // Environment info (platform, OS version, shell, date)
    if !opts.skip_env_info {
        parts.push(build_env_info_section(opts.working_directory.as_deref()));
    }

    // Working directory (legacy XML tag kept for caching compat)
    if let Some(cwd) = &opts.working_directory {
        parts.push(format!("\n<working_directory>{}</working_directory>", cwd));
    }

    // Memory injection (from memdir)
    if !opts.memory_content.is_empty() {
        parts.push(format!(
            "\n<memory>\n{}\n</memory>",
            opts.memory_content
        ));
    }

    // Appended system prompt (--append-system-prompt)
    if let Some(append) = &opts.append_system_prompt {
        parts.push(format!("\n{}", append));
    }

    parts.join("\n")
}

/// Build the dynamic environment-info section injected after the boundary.
fn build_env_info_section(working_dir: Option<&str>) -> String {
    // Platform string
    let platform = "linux";

    // OS version string
    let os_version = "Linux 6.17.0-22-generic";

    // Shell line
    let shell_line = format!("Shell: {}", "bash");

    // Is git repo?
    let is_git = false;

    // Build the section
    let cwd_line = working_dir
        .map(|d| format!("\nWorking directory: {}", d))
        .unwrap_or_default();

    // Platform-specific guidance so the model uses the right commands.
    let os_note = format!(
        "\nThe user is on Linux ({}). Use Linux-compatible commands.",
        os_version
    );

    format!(
        "\n<env>{}\nIs directory a git repo: {}\nPlatform: {}\nOS Version: {}\n{}{}\n</env>",
        cwd_line,
        if is_git { "Yes" } else { "No" },
        platform,
        os_version,
        shell_line,
        os_note,
    )
}


