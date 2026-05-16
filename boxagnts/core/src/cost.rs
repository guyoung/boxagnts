use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Per-model pricing tiers (USD per million tokens).
#[derive(Debug, Clone, Copy)]
pub struct ModelPricing {
    pub input_per_mtk: f64,
    pub output_per_mtk: f64,
    pub cache_creation_per_mtk: f64,
    pub cache_read_per_mtk: f64,
}

impl ModelPricing {
    /// Pricing for Claude Opus 4 family.
    pub const OPUS: Self = Self {
        input_per_mtk: 15.0,
        output_per_mtk: 75.0,
        cache_creation_per_mtk: 18.75,
        cache_read_per_mtk: 1.5,
    };

    /// Pricing for Claude Sonnet 4 family.
    pub const SONNET: Self = Self {
        input_per_mtk: 3.0,
        output_per_mtk: 15.0,
        cache_creation_per_mtk: 3.75,
        cache_read_per_mtk: 0.3,
    };

    /// Pricing for Claude Haiku family.
    pub const HAIKU: Self = Self {
        input_per_mtk: 0.80,
        output_per_mtk: 4.0,
        cache_creation_per_mtk: 1.0,
        cache_read_per_mtk: 0.08,
    };

    /// Default pricing is Opus (most capable, highest cost).
    pub fn default_pricing() -> Self {
        Self::OPUS
    }

    /// Pick pricing based on model name substring matching.
    pub fn for_model(model: &str) -> Self {
        if model.contains("opus") {
            Self::OPUS
        } else if model.contains("haiku") {
            Self::HAIKU
        } else {
            // Default to Sonnet pricing for unknown models
            Self::SONNET
        }
    }
}

impl Default for ModelPricing {
    fn default() -> Self {
        Self::OPUS
    }
}

/// Thread-safe, lock-free cost tracker that accumulates token usage.
#[derive(Debug, Default)]
pub struct CostTracker {
    input_tokens: AtomicU64,
    output_tokens: AtomicU64,
    cache_creation_tokens: AtomicU64,
    cache_read_tokens: AtomicU64,
    pricing: parking_lot::RwLock<ModelPricing>,
}

// We need a default for RwLock<ModelPricing> -- use Opus as default.
impl CostTracker {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            pricing: parking_lot::RwLock::new(ModelPricing::OPUS),
            ..Default::default()
        })
    }

    pub fn with_model(model: &str) -> Arc<Self> {
        Arc::new(Self {
            pricing: parking_lot::RwLock::new(ModelPricing::for_model(model)),
            ..Default::default()
        })
    }

    pub fn set_model(&self, model: &str) {
        *self.pricing.write() = ModelPricing::for_model(model);
    }

    pub fn add_usage(
        &self,
        input: u64,
        output: u64,
        cache_creation: u64,
        cache_read: u64,
    ) {
        self.input_tokens.fetch_add(input, Ordering::Relaxed);
        self.output_tokens.fetch_add(output, Ordering::Relaxed);
        self.cache_creation_tokens
            .fetch_add(cache_creation, Ordering::Relaxed);
        self.cache_read_tokens
            .fetch_add(cache_read, Ordering::Relaxed);
    }

    pub fn total_cost_usd(&self) -> f64 {
        let pricing = *self.pricing.read();
        let input = self.input_tokens.load(Ordering::Relaxed) as f64;
        let output = self.output_tokens.load(Ordering::Relaxed) as f64;
        let cache_creation = self.cache_creation_tokens.load(Ordering::Relaxed) as f64;
        let cache_read = self.cache_read_tokens.load(Ordering::Relaxed) as f64;

        (input * pricing.input_per_mtk
            + output * pricing.output_per_mtk
            + cache_creation * pricing.cache_creation_per_mtk
            + cache_read * pricing.cache_read_per_mtk)
            / 1_000_000.0
    }

    pub fn total_tokens(&self) -> u64 {
        self.input_tokens.load(Ordering::Relaxed)
            + self.output_tokens.load(Ordering::Relaxed)
            + self.cache_creation_tokens.load(Ordering::Relaxed)
            + self.cache_read_tokens.load(Ordering::Relaxed)
    }

    pub fn input_tokens(&self) -> u64 {
        self.input_tokens.load(Ordering::Relaxed)
    }

    pub fn output_tokens(&self) -> u64 {
        self.output_tokens.load(Ordering::Relaxed)
    }

    pub fn cache_creation_tokens(&self) -> u64 {
        self.cache_creation_tokens.load(Ordering::Relaxed)
    }

    pub fn cache_read_tokens(&self) -> u64 {
        self.cache_read_tokens.load(Ordering::Relaxed)
    }

    /// Produce a human-readable summary string, e.g. for display in the TUI.
    pub fn summary(&self) -> String {
        let cost = self.total_cost_usd();
        let total = self.total_tokens();
        if cost < 0.01 {
            format!("{} tokens (<$0.01)", total)
        } else {
            format!("{} tokens (${:.2})", total, cost)
        }
    }
}