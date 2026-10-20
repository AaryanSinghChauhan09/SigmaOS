// SPDX-License-Identifier: MIT
// SigmaOS - Config Manager
// Configuration management inspired by Omarchy config structure and Linux config systems

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Config type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigType {
    System,
    User,
    Application,
    Theme,
}

/// Config entry
#[derive(Debug, Clone)]
pub struct ConfigEntry {
    pub key: String,
    pub value: String,
    pub config_type: ConfigType,
}

/// Config manager
pub struct SigmaConfigManager {
    configs: Vec<ConfigEntry>,
}

impl SigmaConfigManager {
    pub fn new() -> Self {
        Self {
            configs: Vec::new(),
        }
    }

    pub fn set_config(&mut self, key: String, value: String, config_type: ConfigType) {
        if let Some(idx) = self.configs.iter().position(|c| c.key == key) {
            self.configs[idx].value = value;
            self.configs[idx].config_type = config_type;
        } else {
            let entry = ConfigEntry {
                key,
                value,
                config_type,
            };
            self.configs.push(entry);
        }
    }

    pub fn get_config(&self, key: &str) -> Option<&ConfigEntry> {
        self.configs.iter().find(|c| c.key == key)
    }

    pub fn get_config_value(&self, key: &str) -> Option<&String> {
        self.get_config(key).map(|c| &c.value)
    }

    pub fn get_configs(&self) -> &[ConfigEntry] {
        &self.configs
    }

    pub fn get_configs_by_type(&self, config_type: ConfigType) -> Vec<&ConfigEntry> {
        self.configs.iter().filter(|c| c.config_type == config_type).collect()
    }

    pub fn remove_config(&mut self, key: String) -> Result<(), String> {
        if let Some(idx) = self.configs.iter().position(|c| c.key == key) {
            self.configs.remove(idx);
            Ok(())
        } else {
            Err(format!("Config {} not found", key))
        }
    }

    pub fn refresh_config(&mut self, default_path: String, user_path: String) -> Result<(), String> {
        // Simulated refresh - in real implementation would copy from default to user
        Ok(())
    }
}

impl Default for SigmaConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_manager_creation() {
        let manager = SigmaConfigManager::new();
        assert_eq!(manager.get_configs().len(), 0);
    }

    #[test]
    fn test_set_config() {
        let mut manager = SigmaConfigManager::new();
        manager.set_config("test-key".to_string(), "test-value".to_string(), ConfigType::System);
        assert_eq!(manager.get_configs().len(), 1);
    }

    #[test]
    fn test_get_config_value() {
        let mut manager = SigmaConfigManager::new();
        manager.set_config("test-key".to_string(), "test-value".to_string(), ConfigType::System);
        assert_eq!(manager.get_config_value("test-key"), Some(&"test-value".to_string()));
    }

    #[test]
    fn test_update_config() {
        let mut manager = SigmaConfigManager::new();
        manager.set_config("test-key".to_string(), "old-value".to_string(), ConfigType::System);
        manager.set_config("test-key".to_string(), "new-value".to_string(), ConfigType::System);
        assert_eq!(manager.get_config_value("test-key"), Some(&"new-value".to_string()));
        assert_eq!(manager.get_configs().len(), 1);
    }

    #[test]
    fn test_remove_config() {
        let mut manager = SigmaConfigManager::new();
        manager.set_config("test-key".to_string(), "test-value".to_string(), ConfigType::System);
        manager.remove_config("test-key".to_string()).unwrap();
        assert_eq!(manager.get_configs().len(), 0);
    }

    #[test]
    fn test_configs_by_type() {
        let mut manager = SigmaConfigManager::new();
        manager.set_config("sys-key".to_string(), "sys-value".to_string(), ConfigType::System);
        manager.set_config("user-key".to_string(), "user-value".to_string(), ConfigType::User);
        assert_eq!(manager.get_configs_by_type(ConfigType::System).len(), 1);
    }
}
