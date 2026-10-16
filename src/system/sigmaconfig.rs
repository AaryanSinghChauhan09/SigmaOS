// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaConfig - System Configuration Manager
//!
//! SigmaConfig provides system configuration management for SigmaOS, inspired by sysconfig/gconf from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Configuration value type
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigValue {
    /// String value
    String(String),
    /// Integer value
    Integer(i64),
    /// Boolean value
    Boolean(bool),
    /// Float value
    Float(f64),
}

impl fmt::Display for ConfigValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigValue::String(s) => write!(f, "{}", s),
            ConfigValue::Integer(i) => write!(f, "{}", i),
            ConfigValue::Boolean(b) => write!(f, "{}", b),
            ConfigValue::Float(fl) => write!(f, "{}", fl),
        }
    }
}

/// Configuration entry
#[derive(Debug, Clone)]
pub struct ConfigEntry {
    /// Configuration key
    pub key: String,
    /// Configuration value
    pub value: ConfigValue,
    /// Description
    pub description: String,
    /// Read-only
    pub read_only: bool,
}

impl ConfigEntry {
    /// Create a new configuration entry
    pub fn new(key: String, value: ConfigValue, description: String) -> Self {
        Self {
            key,
            value,
            description,
            read_only: false,
        }
    }

    /// Set read-only
    pub fn with_read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }
}

/// SigmaConfig - System Configuration Manager
pub struct SigmaConfig {
    /// Configuration entries (key -> entry)
    entries: BTreeMap<String, ConfigEntry>,
    /// Running state
    running: bool,
    /// Entry counter
    entry_counter: u32,
}

impl SigmaConfig {
    /// Create a new SigmaConfig instance
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
            running: false,
            entry_counter: 0,
        }
    }

    /// Start the configuration manager
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the configuration manager
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Add a configuration entry
    pub fn add_entry(&mut self, entry: ConfigEntry) -> Result<(), String> {
        if self.entries.contains_key(&entry.key) {
            return Err(format!("Entry '{}' already exists", entry.key));
        }

        self.entries.insert(entry.key.clone(), entry);
        self.entry_counter += 1;
        Ok(())
    }

    /// Remove a configuration entry
    pub fn remove_entry(&mut self, key: &str) -> Result<(), String> {
        match self.entries.get(key) {
            Some(entry) if entry.read_only => {
                return Err(format!("Entry '{}' is read-only", key));
            }
            _ => {}
        }

        if self.entries.remove(key).is_none() {
            return Err(format!("Entry '{}' not found", key));
        }
        Ok(())
    }

    /// Get a configuration entry
    pub fn get_entry(&self, key: &str) -> Option<&ConfigEntry> {
        self.entries.get(key)
    }

    /// Set a configuration value
    pub fn set_value(&mut self, key: &str, value: ConfigValue) -> Result<(), String> {
        if !self.running {
            return Err("SigmaConfig not running".to_string());
        }

        match self.entries.get_mut(key) {
            Some(entry) => {
                if entry.read_only {
                    return Err(format!("Entry '{}' is read-only", key));
                }
                entry.value = value;
                Ok(())
            }
            None => Err(format!("Entry '{}' not found", key)),
        }
    }

    /// Get a configuration value
    pub fn get_value(&self, key: &str) -> Option<&ConfigValue> {
        self.entries.get(key).map(|entry| &entry.value)
    }

    /// List all entries
    pub fn list_entries(&self) -> Vec<&ConfigEntry> {
        self.entries.values().collect()
    }

    /// List entries by pattern
    pub fn list_by_pattern(&self, pattern: &str) -> Vec<&ConfigEntry> {
        self.entries
            .values()
            .filter(|e| e.key.contains(pattern))
            .collect()
    }

    /// Get entry count
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    /// Get configuration manager statistics
    pub fn config_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert("total_entries".to_string(), self.entry_count().to_string());
        stats.insert(
            "read_only_entries".to_string(),
            self.entries
                .values()
                .filter(|e| e.read_only)
                .count()
                .to_string(),
        );
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
}

impl Default for SigmaConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaConfig instance (placeholder for actual global state)
static mut SIGMACONFIG_INSTANCE: Option<SigmaConfig> = None;

/// Initialize the global SigmaConfig instance
pub fn init_sigmaconfig() {
    unsafe {
        SIGMACONFIG_INSTANCE = Some(SigmaConfig::new());
    }
}

/// Get the global SigmaConfig instance
pub fn get_sigmaconfig() -> Option<&'static mut SigmaConfig> {
    unsafe { SIGMACONFIG_INSTANCE.as_mut() }
}

/// Convenience function to set a configuration value
pub fn set_config_value(key: &str, value: ConfigValue) -> Result<(), String> {
    if let Some(config) = get_sigmaconfig() {
        config.set_value(key, value)
    } else {
        Err("SigmaConfig not initialized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_value_display() {
        assert_eq!(ConfigValue::String("test".to_string()).to_string(), "test");
        assert_eq!(ConfigValue::Integer(42).to_string(), "42");
    }

    #[test]
    fn test_config_entry() {
        let entry = ConfigEntry::new(
            "system.hostname".to_string(),
            ConfigValue::String("sigmaos".to_string()),
            "System hostname".to_string(),
        )
        .with_read_only(false);

        assert_eq!(entry.key, "system.hostname");
        assert!(!entry.read_only);
    }

    #[test]
    fn test_sigmaconfig() {
        let mut config = SigmaConfig::new();
        assert!(!config.is_running());
        config.start();
        assert!(config.is_running());
        config.stop();
        assert!(!config.is_running());
    }

    #[test]
    fn test_sigmaconfig_add_entry() {
        let mut config = SigmaConfig::new();

        let entry = ConfigEntry::new(
            "system.hostname".to_string(),
            ConfigValue::String("sigmaos".to_string()),
            "System hostname".to_string(),
        );

        assert!(config.add_entry(entry).is_ok());
        assert_eq!(config.entry_count(), 1);
    }

    #[test]
    fn test_sigmaconfig_set_value() {
        let mut config = SigmaConfig::new();
        config.start();

        let entry = ConfigEntry::new(
            "system.hostname".to_string(),
            ConfigValue::String("sigmaos".to_string()),
            "System hostname".to_string(),
        );

        config.add_entry(entry).unwrap();
        assert!(config
            .set_value(
                "system.hostname",
                ConfigValue::String("newname".to_string())
            )
            .is_ok());

        let entry = config.get_entry("system.hostname").unwrap();
        assert_eq!(entry.value, ConfigValue::String("newname".to_string()));
    }

    #[test]
    fn test_sigmaconfig_read_only() {
        let mut config = SigmaConfig::new();

        let entry = ConfigEntry::new(
            "system.hostname".to_string(),
            ConfigValue::String("sigmaos".to_string()),
            "System hostname".to_string(),
        )
        .with_read_only(true);

        config.add_entry(entry).unwrap();
        assert!(config
            .set_value(
                "system.hostname",
                ConfigValue::String("newname".to_string())
            )
            .is_err());
    }

    #[test]
    fn test_sigmaconfig_remove_entry() {
        let mut config = SigmaConfig::new();

        let entry = ConfigEntry::new(
            "system.hostname".to_string(),
            ConfigValue::String("sigmaos".to_string()),
            "System hostname".to_string(),
        );

        config.add_entry(entry).unwrap();
        assert!(config.remove_entry("system.hostname").is_ok());
        assert_eq!(config.entry_count(), 0);
    }

    #[test]
    fn test_sigmaconfig_list_by_pattern() {
        let mut config = SigmaConfig::new();

        let entry = ConfigEntry::new(
            "system.hostname".to_string(),
            ConfigValue::String("sigmaos".to_string()),
            "System hostname".to_string(),
        );

        config.add_entry(entry).unwrap();

        let matches = config.list_by_pattern("system");
        assert_eq!(matches.len(), 1);
    }

    #[test]
    fn test_sigmaconfig_stats() {
        let mut config = SigmaConfig::new();

        let entry = ConfigEntry::new(
            "system.hostname".to_string(),
            ConfigValue::String("sigmaos".to_string()),
            "System hostname".to_string(),
        );

        config.add_entry(entry).unwrap();

        let stats = config.config_stats();
        assert_eq!(stats.get("total_entries"), Some(&"1".to_string()));
    }

    #[test]
    fn test_set_config_value_convenience() {
        init_sigmaconfig();

        let mut config = get_sigmaconfig().unwrap();
        config.start();

        let entry = ConfigEntry::new(
            "system.hostname".to_string(),
            ConfigValue::String("sigmaos".to_string()),
            "System hostname".to_string(),
        );

        config.add_entry(entry).unwrap();

        let result = set_config_value(
            "system.hostname",
            ConfigValue::String("newname".to_string()),
        );
        assert!(result.is_ok());
    }
}
