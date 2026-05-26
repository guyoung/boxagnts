use std::sync::Arc;

pub mod chat;
pub mod chat_session;
pub mod file;
pub mod mcp;

pub mod project;
pub mod tool;
pub mod skill;
pub mod fs_events;

pub fn load_cached_model_registry() -> Arc<boxagnts_api::ModelRegistry> {
    let mut reg = boxagnts_api::ModelRegistry::new();

    reg.load_cache();

    Arc::new(reg)
}
