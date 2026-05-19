#![allow(unused)]
use serde_json::{ json, Value };


pub fn get_provider_options() -> Vec<Value> {
    vec![
        json!({ "id": "openai", "title": "OpenAI", "description": "(API key)", "category": "Popular", "badge": null }),
        json!({ "id": "openai-codex", "title": "OpenAI Codex", "description": "(ChatGPT Plus/Pro — browser login)", "category": "Popular", "badge": null }),
        json!({ "id": "github-copilot", "title": "GitHub Copilot", "description": "(GitHub subscription or token)", "category": "Popular", "badge": null }),
        json!({ "id": "google", "title": "Google", "description": "(API key)", "category": "Popular", "badge": null }),
        json!({ "id": "anthropic", "title": "Anthropic", "description": "(API key)", "category": "Popular", "badge": null }),
        json!({ "id": "custom-openai", "title": "Custom OpenAI-Compatible", "description": "Custom URL + API key", "category": "Advanced", "badge": null }),
        json!({ "id": "openrouter", "title": "OpenRouter", "description": "100+ models with one key", "category": "Popular", "badge": null }),
        json!({ "id": "vercel", "title": "Vercel AI Gateway", "description": "Gateway for AI SDK models", "category": "Popular", "badge": null }),
        json!({ "id": "groq", "title": "Groq", "description": "Fast hosted inference", "category": "Popular", "badge": Some("FREE") }),
        json!({ "id": "ollama", "title": "Ollama", "description": "Run models locally", "category": "Popular", "badge": Some("LOCAL") }),
        json!({ "id": "zai", "title": "Z.AI", "description": "GLM-5.1 / GLM-5 / GLM-4.7 Coding Plan", "category": "Popular", "badge": null }),
        json!({ "id": "opencode-go", "title": "OpenCode Go", "description": "$10/mo flat-rate · Kimi · DeepSeek · GLM · MiniMax", "category": "Popular", "badge": null }),
        json!({ "id": "cerebras", "title": "Cerebras", "description": "Fast hosted inference", "category": "Other", "badge": Some("FREE") }),
        json!({ "id": "sambanova", "title": "SambaNova", "description": "Fast hosted inference", "category": "Other", "badge": Some("FREE") }),
        json!({ "id": "lmstudio", "title": "LM Studio", "description": "Local model server", "category": "Other", "badge": Some("LOCAL") }),
        json!({ "id": "llamacpp", "title": "llama.cpp", "description": "Local inference server", "category": "Other", "badge": Some("LOCAL") }),
        json!({ "id": "deepseek", "title": "DeepSeek", "description": "Reasoning and coding models", "category": "Other", "badge": null }),
        json!({ "id": "mistral", "title": "Mistral", "description": "Hosted Mistral models", "category": "Other", "badge": null }),
        json!({ "id": "togetherai", "title": "Together AI", "description": "Open model hosting", "category": "Other", "badge": null }),
        json!({ "id": "perplexity", "title": "Perplexity", "description": "Search-augmented models", "category": "Other", "badge": null }),
        json!({ "id": "cohere", "title": "Cohere", "description": "Command models", "category": "Other", "badge": null }),
        json!({ "id": "xai", "title": "xAI", "description": "Grok models", "category": "Other", "badge": null }),
        json!({ "id": "deepinfra", "title": "DeepInfra", "description": "Hosted open models", "category": "Other", "badge": null }),
        json!({ "id": "azure", "title": "Azure OpenAI", "description": "Enterprise OpenAI deployments", "category": "Other", "badge": null }),
        json!({ "id": "amazon-bedrock", "title": "AWS Bedrock", "description": "Enterprise foundation models", "category": "Other", "badge": null }),
        json!({ "id": "google-vertex", "title": "Google Vertex AI", "description": "Enterprise Google models", "category": "Other", "badge": null }),
        json!({ "id": "sap-ai-core", "title": "SAP AI Core", "description": "Enterprise AI platform", "category": "Other", "badge": null }),
        json!({ "id": "gitlab", "title": "GitLab Duo", "description": "AI in GitLab", "category": "Other", "badge": null }),
        json!({ "id": "cloudflare-ai-gateway", "title": "Cloudflare AI Gateway", "description": "Gateway for multiple providers", "category": "Other", "badge": null }),
        json!({ "id": "cloudflare-workers-ai", "title": "Cloudflare Workers AI", "description": "Edge AI inference", "category": "Other", "badge": null }),
        json!({ "id": "helicone", "title": "Helicone", "description": "AI gateway and observability", "category": "Other", "badge": null }),
        json!({ "id": "huggingface", "title": "Hugging Face", "description": "Hosted community models", "category": "Other", "badge": null }),
        json!({ "id": "nvidia", "title": "NVIDIA", "description": "Hosted NVIDIA models", "category": "Other", "badge": null }),
        json!({ "id": "alibaba", "title": "Alibaba", "description": "Qwen and hosted models", "category": "Other", "badge": null }),
        json!({ "id": "venice", "title": "Venice AI", "description": "Privacy-first AI", "category": "Other", "badge": null }),
        json!({ "id": "moonshotai", "title": "Moonshot AI", "description": "Hosted Moonshot models", "category": "Other", "badge": null }),
        json!({ "id": "zhipuai", "title": "Zhipu AI", "description": "Hosted GLM models", "category": "Other", "badge": null }),
        json!({ "id": "siliconflow", "title": "SiliconFlow", "description": "Hosted open models", "category": "Other", "badge": null }),
        json!({ "id": "nebius", "title": "Nebius", "description": "Cloud inference", "category": "Other", "badge": null }),
        json!({ "id": "novita", "title": "Novita", "description": "Cloud inference", "category": "Other", "badge": null }),
        json!({ "id": "minimax", "title": "MiniMax", "description": "Anthropic-compatible (M2.7)", "category": "Other", "badge": null }),
        json!({ "id": "ovhcloud", "title": "OVHcloud", "description": "EU-hosted AI", "category": "Other", "badge": null }),
        json!({ "id": "scaleway", "title": "Scaleway", "description": "EU cloud AI", "category": "Other", "badge": null }),
        json!({ "id": "vultr", "title": "Vultr", "description": "Cloud inference", "category": "Other", "badge": null }),
        json!({ "id": "baseten", "title": "Baseten", "description": "Model serving", "category": "Other", "badge": null }),
        json!({ "id": "friendli", "title": "Friendli", "description": "Serverless inference", "category": "Other", "badge": null }),
        json!({ "id": "upstage", "title": "Upstage", "description": "Hosted Upstage models", "category": "Other", "badge": null }),
        json!({ "id": "stepfun", "title": "StepFun", "description": "Hosted reasoning models", "category": "Other", "badge": null }),
        json!({ "id": "fireworks", "title": "Fireworks AI", "description": "Fast inference", "category": "Other", "badge": null }),
    ]
}


/// A single model entry shown in the picker.
#[derive(Debug, Clone)]
pub struct ModelEntry {
    pub id: String,
    pub display_name: String,
    pub description: String,
    /// Whether this is the currently active model.
    pub is_current: bool,
}

/// Helper to build a `ModelEntry` with `is_current = false`.
fn model_entry(id: &str, name: &str, desc: &str) -> ModelEntry {
    ModelEntry {
        id: id.to_string(),
        display_name: name.to_string(),
        description: desc.to_string(),
        is_current: false,
    }
}

/// Get models for a provider from the model registry (models.dev data).
///
/// Falls back to the hardcoded `models_for_provider()` list when the registry
/// has no entries for this provider.  This makes models.dev the single source
/// of truth once the background fetch completes, while still providing a good
/// experience before the fetch finishes.
pub fn models_for_provider_from_registry(
    provider_id: &str,
    registry: &boxagnts_api::ModelRegistry,
) -> Vec<ModelEntry> {
    let entries = registry.list_by_provider(provider_id);
    if !entries.is_empty() {
        entries
            .iter()
            .map(|e| {
                let cost_str = match (e.cost_input, e.cost_output) {
                    (Some(ci), Some(co)) => format!("{} | ${:.2}/${:.2} per M", format_context_window(e.info.context_window), ci, co),
                    _ => format_context_window(e.info.context_window),
                };
                ModelEntry {
                    id: e.info.id.to_string(),
                    display_name: e.info.name.clone(),
                    description: cost_str,
                    is_current: false,
                }
            })
            .collect()
    } else {
        // Fall back to hardcoded
        models_for_provider(provider_id)
    }
}


/// Build the model list for a given provider.
///
/// Returns a curated set of well-known models for major providers so the
/// `/model` picker shows relevant choices regardless of whether the API
/// returned a live model list.
pub fn models_for_provider(provider_id: &str) -> Vec<ModelEntry> {
    match provider_id {
        "anthropic" => vec![
            model_entry("claude-opus-4-6", "Claude Opus 4.6", "Most capable — best for complex reasoning and analysis"),
            model_entry("claude-sonnet-4-6", "Claude Sonnet 4.6", "Balanced performance and speed — great for coding tasks"),
            model_entry("claude-haiku-4-5-20251001", "Claude Haiku 4.5", "Fast and efficient — ideal for quick completions"),
        ],
        "openai" => vec![
            model_entry("gpt-4o", "GPT-4o", "128K context"),
            model_entry("gpt-4o-mini", "GPT-4o mini", "128K context"),
            model_entry("gpt-4.1", "GPT-4.1", "1M context"),
            model_entry("gpt-4.1-mini", "GPT-4.1 mini", "1M context"),
            model_entry("gpt-4.1-nano", "GPT-4.1 nano", "1M context"),
            model_entry("o3", "o3", "200K context"),
            model_entry("o3-mini", "o3 mini", "200K context"),
            model_entry("o4-mini", "o4 mini", "200K context"),
            model_entry("gpt-4-turbo", "GPT-4 Turbo", "128K context"),
        ],
        "google" => vec![
            model_entry("gemini-2.5-pro", "Gemini 2.5 Pro", "1M context"),
            model_entry("gemini-2.5-flash", "Gemini 2.5 Flash", "1M context"),
            model_entry("gemini-2.0-flash", "Gemini 2.0 Flash", "1M context"),
        ],
        "minimax" => vec![
            model_entry("MiniMax-M2.7", "MiniMax M2.7", "Anthropic-compatible (128K context)"),
        ],
        "groq" => vec![
            model_entry("llama-3.3-70b-versatile", "Llama 3.3 70B", "128K context"),
            model_entry("llama-3.1-8b-instant", "Llama 3.1 8B", "128K context"),
            model_entry("mixtral-8x7b-32768", "Mixtral 8x7B", "32K context"),
            model_entry("gemma2-9b-it", "Gemma 2 9B", "8K context"),
        ],
        "cerebras" => vec![
            model_entry("llama-3.3-70b", "Llama 3.3 70B", "128K context"),
            model_entry("llama-3.1-8b", "Llama 3.1 8B", "128K context"),
        ],
        "deepseek" => vec![
            model_entry(
                "deepseek-v4-pro",
                "DeepSeek V4 Pro",
                "1M context, 384K output",
            ),
            model_entry(
                "deepseek-v4-flash",
                "DeepSeek V4 Flash",
                "1M context, 384K output",
            ),
        ],
        "mistral" => vec![
            model_entry("mistral-large-latest", "Mistral Large", "128K context"),
            model_entry("mistral-small-latest", "Mistral Small", "128K context"),
            model_entry("codestral-latest", "Codestral", "32K context"),
        ],
        "xai" => vec![
            model_entry("grok-2", "Grok 2", "128K context"),
            model_entry("grok-3", "Grok 3", "128K context"),
            model_entry("grok-3-mini", "Grok 3 mini", "128K context"),
        ],
        "openrouter" => vec![
            model_entry("anthropic/claude-sonnet-4", "Claude Sonnet 4", "via OpenRouter"),
            model_entry("openai/gpt-4o", "GPT-4o", "via OpenRouter"),
            model_entry("google/gemini-2.5-pro", "Gemini 2.5 Pro", "via OpenRouter"),
            model_entry("meta-llama/llama-3.3-70b-instruct", "Llama 3.3 70B", "via OpenRouter"),
        ],
        "codex" | "openai-codex" => vec![
            model_entry("gpt-5.2-codex", "GPT-5.2 Codex", "OAuth-backed Codex default"),
            model_entry("gpt-5.1-codex", "GPT-5.1 Codex", "Previous Codex generation"),
            model_entry("gpt-5.1-codex-mini", "GPT-5.1 Codex Mini", "Smaller Codex model"),
            model_entry("gpt-5.1-codex-max", "GPT-5.1 Codex Max", "Larger Codex model"),
            model_entry("gpt-5.4", "GPT-5.4", "General frontier model via Codex auth"),
            model_entry("gpt-5.2", "GPT-5.2", "General model via Codex auth"),
        ],
        "github-copilot" => vec![
            model_entry("claude-sonnet-4.6", "Claude Sonnet 4.6", "via Copilot"),
            model_entry("claude-sonnet-4.5", "Claude Sonnet 4.5", "via Copilot"),
            model_entry("claude-haiku-4.5", "Claude Haiku 4.5", "via Copilot"),
            model_entry("gpt-4.1", "GPT-4.1", "via Copilot"),
            model_entry("gpt-4o", "GPT-4o", "via Copilot"),
            model_entry("gpt-4o-mini", "GPT-4o mini", "via Copilot"),
            model_entry("gpt-5.4", "GPT-5.4", "via Copilot"),
            model_entry("gpt-5-mini", "GPT-5 mini", "via Copilot"),
            model_entry("o3-mini", "o3 mini", "via Copilot"),
            model_entry("o4-mini", "o4 mini", "via Copilot"),
            model_entry("gemini-3-flash-preview", "Gemini 3 Flash", "via Copilot"),
        ],
        "cohere" => vec![
            model_entry("command-r-plus", "Command R+", "128K context"),
            model_entry("command-r", "Command R", "128K context"),
        ],
        "perplexity" => vec![
            model_entry("sonar-pro", "Sonar Pro", "search-augmented"),
            model_entry("sonar", "Sonar", "search-augmented"),
        ],
        "togetherai" | "together-ai" => vec![
            model_entry("meta-llama/Llama-3.3-70B-Instruct-Turbo", "Llama 3.3 70B Turbo", "128K context"),
            model_entry("meta-llama/Llama-3.1-8B-Instruct-Turbo", "Llama 3.1 8B Turbo", "128K context"),
            model_entry("Qwen/Qwen2.5-72B-Instruct-Turbo", "Qwen 2.5 72B Turbo", "128K context"),
        ],
        "deepinfra" => vec![
            model_entry("meta-llama/Llama-3.3-70B-Instruct", "Llama 3.3 70B", "128K context"),
            model_entry("meta-llama/Llama-3.1-8B-Instruct", "Llama 3.1 8B", "128K context"),
        ],
        "venice" => vec![
            model_entry("llama-3.3-70b", "Llama 3.3 70B", "128K context"),
        ],
        "ollama" => vec![
            model_entry("qwen2.5-coder", "Qwen 2.5 Coder", "local — best for coding tasks"),
            model_entry("deepseek-coder-v2", "DeepSeek Coder V2", "local — coding model"),
            model_entry("codellama", "Code Llama", "local — coding model"),
            model_entry("llama3.2", "Llama 3.2", "local — general purpose"),
            model_entry("mistral", "Mistral", "local — general purpose"),
            model_entry("gemma2", "Gemma 2", "local — general purpose"),
            model_entry("phi3", "Phi-3", "local — general purpose"),
            model_entry("qwen2.5", "Qwen 2.5", "local — general purpose"),
        ],
        "azure" => vec![
            model_entry("gpt-4o", "GPT-4o (Azure)", "128K context"),
            model_entry("gpt-4o-mini", "GPT-4o mini (Azure)", "128K context"),
        ],
        "custom-openai" => vec![
            model_entry("default", "Default model", "OpenAI-compatible endpoint"),
        ],
        "amazon-bedrock" => vec![
            model_entry("anthropic.claude-sonnet-4-6-v1", "Claude Sonnet 4.6 (Bedrock)", "200K context"),
            model_entry("anthropic.claude-haiku-4-5-20251001-v1", "Claude Haiku 4.5 (Bedrock)", "200K context"),
        ],
        "lmstudio" => vec![
            model_entry("default", "Default model", "local"),
        ],
        "llamacpp" => vec![
            model_entry("default", "Default model", "local"),
        ],
        _ => vec![
            model_entry("default", "Default model", ""),
        ],
    }
}


// Return the provider-prefixed default model name for a given provider.
///
/// This is used when connecting to a new provider so the status bar
/// immediately shows the right model.
pub fn default_model_for_provider(provider_id: &str) -> String {
    match provider_id {
        "anthropic" => "claude-opus-4-6".to_string(),
        "openai" => "openai/gpt-4o".to_string(),
        "google" => "google/gemini-2.5-flash".to_string(),
        "minimax" => "minimax/MiniMax-M2.7".to_string(),
        "groq" => "groq/llama-3.3-70b-versatile".to_string(),
        "cerebras" => "cerebras/llama-3.3-70b".to_string(),
        "deepseek" => "deepseek/deepseek-v4-pro".to_string(),
        "mistral" => "mistral/mistral-large-latest".to_string(),
        "xai" => "xai/grok-2".to_string(),
        "openrouter" => "openrouter/anthropic/claude-sonnet-4".to_string(),
        "github-copilot" => "github-copilot/gpt-4o".to_string(),
        "codex" => "codex/gpt-5.2-codex".to_string(),
        "openai-codex" => "openai-codex/gpt-5.2-codex".to_string(),
        "cohere" => "cohere/command-r-plus".to_string(),
        "perplexity" => "perplexity/sonar-pro".to_string(),
        "togetherai" | "together-ai" => "togetherai/meta-llama/Llama-3.3-70B-Instruct-Turbo".to_string(),
        "deepinfra" => "deepinfra/meta-llama/Llama-3.3-70B-Instruct".to_string(),
        "venice" => "venice/llama-3.3-70b".to_string(),
        "ollama" => "ollama/llama3.2".to_string(),
        "lmstudio" => "lmstudio/default".to_string(),
        "llamacpp" => "llamacpp/default".to_string(),
        "azure" => "azure/gpt-4o".to_string(),
        "amazon-bedrock" => "amazon-bedrock/anthropic.claude-sonnet-4-6-v1".to_string(),
        other => format!("{}/default", other),
    }
}

/// Format context window tokens for display in the model picker.
pub fn format_context_window(context_window: u32) -> String {
    if context_window >= 1_000_000 {
        if context_window % 1_000_000 == 0 {
            format!("{}M context", context_window / 1_000_000)
        } else {
            format!("{:.1}M context", context_window as f64 / 1_000_000.0)
        }
    } else {
        format!("{}K context", context_window / 1000)
    }
}

fn infer_provider_from_model(model: &str) -> Option<String> {
    if let Some((provider, _)) = model.split_once('/') {
        let known = [
            "anthropic",
            "openai",
            "google",
            "groq",
            "cerebras",
            "deepseek",
            "mistral",
            "xai",
            "openrouter",
            "github-copilot",
            "codex",
            "cohere",
            "perplexity",
            "togetherai",
            "together-ai",
            "deepinfra",
            "venice",
            "minimax",
            "ollama",
            "lmstudio",
            "llamacpp",
            "azure",
            "amazon-bedrock",
        ];
        if known.contains(&provider) {
            return Some(provider.to_string());
        }
    }

    if model.starts_with("claude") {
        Some("anthropic".to_string())
    } else if model.starts_with("gpt-")
        || model.starts_with("o1")
        || model.starts_with("o3")
        || model.starts_with("o4")
    {
        Some("openai".to_string())
    } else if model.starts_with("gemini") || model.starts_with("gemma") {
        Some("google".to_string())
    } else {
        None
    }
}