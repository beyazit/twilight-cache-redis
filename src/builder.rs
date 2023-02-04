use super::{
    config::{Config, ResourceType},
    RedisCache,
};

/// Builder to configure and construct an [`RedisCache`].
#[derive(Debug, Default)]
#[must_use = "has no effect if not built"]
pub struct RedisCacheBuilder(Config);

impl RedisCacheBuilder {
    /// Creates a builder to configure and construct an [`RedisCache`].
    pub const fn new() -> Self {
        Self(Config::new())
    }

    /// Consume the builder, returning a configured cache.
    pub fn build(self) -> RedisCache {
        RedisCache::new_with_config(self.0)
    }

    /// Sets the list of resource types for the cache to handle.
    ///
    /// Defaults to all types.
    pub const fn resource_types(mut self, resource_types: ResourceType) -> Self {
        self.0.resource_types = resource_types;

        self
    }

    /// Sets the number of messages to cache per channel.
    ///
    /// Defaults to 100.
    pub const fn message_cache_size(mut self, message_cache_size: usize) -> Self {
        self.0.message_cache_size = message_cache_size;

        self
    }
}

#[cfg(test)]
mod tests {
    use super::RedisCacheBuilder;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(RedisCacheBuilder: Debug, Default, Send, Sync);
}
