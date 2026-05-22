#![allow(unused)]
use serde_json::{ json, Value };


pub fn get_provider_options() -> Vec<Value> {
    vec![
        json!({ "id": "free", "title": "Free Mode", "description": "OpenCode Zen → OpenRouter free fallback (no spend)", "category": "Popular", "badge": Some("FREE") }),
        json!({ "id": "openai", "title": "OpenAI", "description": "(API key)", "category": "Popular", "badge": null }),
        json!({ "id": "openai-codex", "title": "OpenAI Codex", "description": "(ChatGPT Plus/Pro — browser login)", "category": "Popular", "badge": null }),
        json!({ "id": "github-copilot", "title": "GitHub Copilot", "description": "(GitHub subscription or token)", "category": "Popular", "badge": null }),
        json!({ "id": "google", "title": "Google", "description": "(API key)", "category": "Popular", "badge": null }),
        json!({ "id": "anthropic", "title": "Anthropic", "description": "(API key)", "category": "Popular", "badge": null }),
        json!({ "id": "custom-openai", "title": "Custom OpenAI-Compatible", "description": "Custom URL + API key", "category": "Advanced", "badge": null }),
        json!({ "id": "openrouter", "title": "OpenRouter", "description": "100+ models with one key", "category": "Popular", "badge": null }),
        json!({ "id": "vercel", "title": "Vercel AI Gateway", "description": "Gateway for AI SDK models", "category": "Popular", "badge": null }),
        json!({ "id": "groq", "title": "Groq", "description": "Fast hosted inference", "category": "Popular", "badge": Some("FREE") }),
        json!({ "id": "ollama", "title": "Ollama", "description": "Local inference + cloud models", "category": "Popular", "badge": Some("LOCAL") }),
        json!({ "id": "zai", "title": "Z.AI", "description": "GLM-5.1 / GLM-5 / GLM-4.7 Coding Plan", "category": "Popular", "badge": null }),
        json!({ "id": "opencode-go", "title": "OpenCode Go", "description": "$10/mo flat-rate · Kimi · DeepSeek · GLM · MiniMax", "category": "Popular", "badge": null }),
        json!({ "id": "opencode-zen", "title": "OpenCode Zen", "description": "Free models + paid · Nemotron · Ring · MiniMax · DeepSeek", "category": "Popular", "badge": Some("FREE") }),
        json!({ "id": "synthetic", "title": "Synthetic.dev", "description": "Hosted open weights", "category": "Popular", "badge": null }),
        json!({ "id": "routing", "title": "routing.run", "description": "Hosted open weights · DeepSeek · Llama · Mixtral · Qwen", "category": "Popular", "badge": null }),
        json!({ "id": "neuralwatt", "title": "NeuralWatt", "description": "Hosted open weights - energy-efficient", "category": "Popular", "badge": null }),
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
/// Builds picker entries from the bundled / network-refreshed registry.
/// The registry is always populated (the embedded models.dev snapshot
/// contains ~118 providers / ~4500 models), so the only time the result
/// is empty is when the caller passed a truly unknown provider id — in
/// which case we synthesize a single `"default"` placeholder so the
/// picker isn't blank.
pub fn models_for_provider_from_registry(
    provider_id: &str,
    registry: &boxagnts_api::ModelRegistry,
) -> Vec<ModelEntry> {
    // "free" is the composite Zen → OpenRouter provider; the upstream
    // models.dev catalog has nothing under this id, so serve a curated list
    // directly.  `free/auto` is the default routing entry; the rest pin a
    // specific upstream model for users who care.
    if provider_id == "free" {
        return free_provider_models();
    }
    // Codex (ChatGPT-authenticated OpenAI) is not in the models.dev catalog —
    // serve the curated CODEX_MODELS list so the picker isn't empty.
    if provider_id == "codex" {
        return codex_provider_models();
    }

    let mut entries = registry.list_visible_by_provider(provider_id);

    // Fall back to all entries (including alpha/deprecated) if the visible
    // filter wiped the list — better to show something than nothing.
    if entries.is_empty() {
        entries = registry.list_by_provider(provider_id);
    }

    if entries.is_empty() {
        // Truly unknown provider — keep the picker non-empty so /model still
        // works against e.g. self-hosted endpoints.
        return vec![model_entry(
            "default",
            "Default model",
            "no catalog entry for this provider",
        )];
    }

    // Sort: most recently released first, then alphabetical by id.
    entries.sort_by(|a, b| {
        let rd_a = a.release_date.as_deref().unwrap_or("");
        let rd_b = b.release_date.as_deref().unwrap_or("");
        rd_b.cmp(rd_a).then_with(|| (*a.info.id).cmp(&*b.info.id))
    });

    entries
        .iter()
        .map(|e| {
            let cost_str = match (e.cost_input, e.cost_output) {
                (Some(ci), Some(co)) => format!(
                    "{} | ${:.2}/${:.2} per M",
                    format_context_window(e.info.context_window),
                    ci,
                    co
                ),
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
}

/// Curated free-mode model list used by `models_for_provider_from_registry`.
/// Always shows `free/auto` first; one pin entry per catalog upstream so the
/// user can target a specific provider when they need to.
fn free_provider_models() -> Vec<ModelEntry> {
    let mut entries = vec![ModelEntry {
        id: "free/auto".to_string(),
        display_name: "Auto (round-robin across configured providers)".to_string(),
        description: "stacks every free-tier key you've added · $0.00 per M".to_string(),
        is_current: false,
    }];

    for upstream in boxagnts_api::FREE_CATALOG {
        entries.push(ModelEntry {
            id: format!("{}/{}", upstream.id, upstream.default_model),
            display_name: format!("{} \u{2014} {}", upstream.title, upstream.default_model),
            description: format!("{} · $0.00 per M", upstream.note),
            is_current: false,
        });
    }

    entries
}

/// Curated Codex (ChatGPT-authenticated OpenAI) model list used by
/// `models_for_provider_from_registry` because models.dev does not catalog
/// these endpoints.
fn codex_provider_models() -> Vec<ModelEntry> {
    boxagnts_workspace::codex_oauth::CODEX_MODELS
        .iter()
        .map(|(id, name)| {
            let ctx = match *id {
                "gpt-5.4" | "gpt-5.2" | "gpt-5.2-codex" | "gpt-5.1-codex"
                | "gpt-5.1-codex-mini" | "gpt-5.1-codex-max" => "400K ctx",
                _ => "128K ctx",
            };
            ModelEntry {
                id: id.to_string(),
                display_name: name.to_string(),
                description: format!("{} | ChatGPT-authenticated", ctx),
                is_current: false,
            }
        })
        .collect()
}

/// Return the provider-prefixed default model name for a given provider,
/// consulting the registry first and falling back to a `provider/default`
/// placeholder for unknown providers.
///
/// **Anthropic exception** — anthropic models are emitted bare (no
/// `anthropic/` prefix) for backward-compatibility with config files that
/// pre-date the multi-provider era.
///
/// **Free exception** — the composite Zen → OpenRouter provider ships with
/// a synthetic `free/auto` default that the wrapper translates per upstream.
pub fn default_model_for_provider(
    provider_id: &str,
    registry: &boxagnts_api::ModelRegistry,
) -> String {
    if provider_id == "free" {
        return "free/auto".to_string();
    }
    if let Some(best) = registry.best_model_for_provider(provider_id) {
        if provider_id == "anthropic" {
            best
        } else {
            format!("{}/{}", provider_id, best)
        }
    } else {
        format!("{}/default", provider_id)
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

/// Format a model display line with optional context window and cost info.
///
/// Example: `"gpt-4o  128K ctx  $5.00/M"`
pub fn format_model_line(model_str: &str, context_window: Option<u32>, cost_per_1m: Option<f64>) -> String {
    let mut parts = vec![model_str.to_string()];
    if let Some(ctx) = context_window {
        parts.push(format_context_window(ctx).replace(" context", " ctx"));
    }
    if let Some(cost) = cost_per_1m {
        if cost == 0.0 {
            parts.push("free".to_string());
        } else {
            parts.push(format!("${:.2}/M", cost));
        }
    }
    parts.join("  ")
}

fn infer_provider_from_model(model: &str) -> Option<String> {
    // Free-mode synthetic IDs always route back through the "free"
    // composite provider so the Zen → OpenRouter fallback kicks in.
    if model == "free/auto"
        || model.starts_with("free/")
        || model.starts_with("zen/")
        || model.starts_with("opencode-zen/")
    {
        return Some("free".to_string());
    }
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
            "free",
            "opencode-zen",
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


/// Return the environment variable name for a given provider ID.
fn get_env_var_for_provider(id: &str) -> &'static str {
    match id {
        "anthropic" => "ANTHROPIC_API_KEY",
        "openai" => "OPENAI_API_KEY",
        "google" | "google-vertex" => "GOOGLE_API_KEY",
        "github-copilot" => "GITHUB_TOKEN",
        "groq" => "GROQ_API_KEY",
        "cerebras" => "CEREBRAS_API_KEY",
        "sambanova" => "SAMBANOVA_API_KEY",
        "deepseek" => "DEEPSEEK_API_KEY",
        "mistral" => "MISTRAL_API_KEY",
        "openrouter" => "OPENROUTER_API_KEY",
        "togetherai" => "TOGETHER_API_KEY",
        "perplexity" => "PERPLEXITY_API_KEY",
        "cohere" => "COHERE_API_KEY",
        "xai" => "XAI_API_KEY",
        "deepinfra" => "DEEPINFRA_API_KEY",
        "azure" => "AZURE_API_KEY",
        "amazon-bedrock" => "AWS_ACCESS_KEY_ID",
        "sap-ai-core" => "AICORE_SERVICE_KEY",
        "gitlab" => "GITLAB_TOKEN",
        "cloudflare-ai-gateway" | "cloudflare-workers-ai" => "CLOUDFLARE_API_TOKEN",
        "vercel" => "AI_GATEWAY_API_KEY",
        "helicone" => "HELICONE_API_KEY",
        "huggingface" => "HF_TOKEN",
        "nvidia" => "NVIDIA_API_KEY",
        "alibaba" => "DASHSCOPE_API_KEY",
        "venice" => "VENICE_API_KEY",
        "moonshotai" => "MOONSHOT_API_KEY",
        "zhipuai" => "ZHIPU_API_KEY",
        "zai" => "ZAI_API_KEY",
        "siliconflow" => "SILICONFLOW_API_KEY",
        "nebius" => "NEBIUS_API_KEY",
        "novita" => "NOVITA_API_KEY",
        "minimax" => "MINIMAX_API_KEY",
        "ovhcloud" => "OVHCLOUD_API_KEY",
        "scaleway" => "SCALEWAY_API_KEY",
        "vultr" => "VULTR_API_KEY",
        "baseten" => "BASETEN_API_KEY",
        "friendli" => "FRIENDLI_TOKEN",
        "upstage" => "UPSTAGE_API_KEY",
        "stepfun" => "STEPFUN_API_KEY",
        "fireworks" => "FIREWORKS_API_KEY",
        _ => "API_KEY",
    }
}

/// Return a URL hint for obtaining an API key from a given provider.
fn get_url_for_provider(id: &str) -> &'static str {
    match id {
        "anthropic" => "console.anthropic.com",
        "openai" => "platform.openai.com/api-keys",
        "google" => "aistudio.google.com/apikey",
        "github-copilot" => "github.com/settings/tokens",
        "groq" => "console.groq.com/keys",
        "cerebras" => "cloud.cerebras.ai",
        "sambanova" => "cloud.sambanova.ai",
        "deepseek" => "platform.deepseek.com/api_keys",
        "mistral" => "console.mistral.ai/api-keys",
        "openrouter" => "openrouter.ai/keys",
        "togetherai" => "api.together.xyz/settings/api-keys",
        "perplexity" => "perplexity.ai/settings/api",
        "cohere" => "dashboard.cohere.com/api-keys",
        "xai" => "console.x.ai",
        "deepinfra" => "deepinfra.com/dash/api_keys",
        "azure" => "portal.azure.com",
        "amazon-bedrock" => "console.aws.amazon.com/bedrock",
        "minimax" => "platform.minimaxi.com",
        "huggingface" => "huggingface.co/settings/tokens",
        "nvidia" => "build.nvidia.com",
        "venice" => "venice.ai/settings/api",
        "zai" => "z.ai/manage-apikey/apikey-list",
        _ => "the provider's website",
    }
}
