/// https://github.com/spinframework/spin/blob/main/crates/expressions/src/provider.rs

use std::fmt::Debug;

use async_trait::async_trait;

use super::Key;

/// A config provider.
#[async_trait]
pub trait Provider: Debug + Send + Sync {
    /// Returns the value at the given config path, if it exists.
    async fn get(&self, key: &Key) -> anyhow::Result<Option<String>>;

    /// Returns true if the given key _might_ be resolvable by this Provider.
    ///
    /// Dynamic resolvers will typically return true unconditionally, which is
    /// the default implementation.
    fn may_resolve(&self, key: &Key) -> bool {
        let _ = key;
        true
    }
}
