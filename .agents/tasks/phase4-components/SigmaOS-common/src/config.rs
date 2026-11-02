//! Configuration loading and parsing for SigmaOS components.

use std::collections::HashMap;

/// A loaded configuration map.
#[derive(Debug, Clone, Default)]
pub struct Config {
    /// Key-value configuration entries.
    pub values: HashMap<String, String>,
}

impl Config {
    /// Create an empty configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get a configuration value by key.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(|s| s.as_str())
    }

    /// Set a configuration value.
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.values.insert(key.into(), value.into());
    }
}
