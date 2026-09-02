// SigmaOS Clear Linux-inspired Stateless Engine
// Implements Intel Clear Linux's stateless configuration and immutable root layers
// Inspired by Clear Linux's performance-optimized architecture

use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

/// Configuration file location
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigLocation {
    Defaults,  // /usr/share/defaults
    System,    // /etc
    Runtime,   // /run
}

/// Configuration state
#[derive(Debug, Clone)]
pub struct ConfigState {
    pub file_path: String,
    pub default_content: String,
    pub current_content: String,
    pub location: ConfigLocation,
}

/// Stateless configuration manager
pub struct ClearLinuxStatelessEngine {
    pub configs: BTreeMap<String, ConfigState>,
    pub readonly_root: bool,
}

impl ClearLinuxStatelessEngine {
    pub fn new() -> Self {
        Self {
            configs: BTreeMap::new(),
            readonly_root: true,
        }
    }

    /// Add default configuration
    pub fn add_default_config(&mut self, file_path: String, default_content: String) {
        let state = ConfigState {
            file_path: file_path.clone(),
            default_content,
            current_content: String::new(),
            location: ConfigLocation::Defaults,
        };
        self.configs.insert(file_path, state);
    }

    /// Override configuration in /etc
    pub fn override_config(&mut self, file_path: String, custom_content: String) {
        if let Some(state) = self.configs.get_mut(&file_path) {
            state.current_content = custom_content;
            state.location = ConfigLocation::System;
        }
    }

    /// Get effective configuration
    pub fn get_config(&self, file_path: &str) -> Option<String> {
        if let Some(state) = self.configs.get(file_path) {
            if state.location == ConfigLocation::System && !state.current_content.is_empty() {
                Some(state.current_content.clone())
            } else {
                Some(state.default_content.clone())
            }
        } else {
            None
        }
    }

    /// Reset configuration to default
    pub fn reset_config(&mut self, file_path: &str) -> Result<(), String> {
        if let Some(state) = self.configs.get_mut(file_path) {
            state.current_content = String::new();
            state.location = ConfigLocation::Defaults;
            Ok(())
        } else {
            Err(format!("Configuration {} not found", file_path))
        }
    }

    /// Enable read-only root
    pub fn enable_readonly_root(&mut self) {
        self.readonly_root = true;
    }

    /// Disable read-only root
    pub fn disable_readonly_root(&mut self) {
        self.readonly_root = false;
    }

    /// Get state status
    pub fn get_status(&self) -> String {
        let overridden = self.configs.values()
            .filter(|s| s.location == ConfigLocation::System)
            .count();
        
        format!(
            "Clear Linux Stateless Status\nRead-only Root: {}\nTotal Configs: {}\nOverridden: {}",
            self.readonly_root,
            self.configs.len(),
            overridden
        )
    }
}

impl Default for ClearLinuxStatelessEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Swupd bundle manager (Clear Linux's update system)
#[derive(Debug, Clone)]
pub struct SwupdBundle {
    pub name: String,
    pub version: String,
    pub size: u64,
    pub dependencies: Vec<String>,
}

/// Swupd update manager
pub struct SwupdUpdateManager {
    pub bundles: BTreeMap<String, SwupdBundle>,
    pub installed_bundles: Vec<String>,
}

impl SwupdUpdateManager {
    pub fn new() -> Self {
        Self {
            bundles: BTreeMap::new(),
            installed_bundles: Vec::new(),
        }
    }

    /// Add bundle
    pub fn add_bundle(&mut self, bundle: SwupdBundle) {
        self.bundles.insert(bundle.name.clone(), bundle);
    }

    /// Install bundle
    pub fn install_bundle(&mut self, bundle_name: &str) -> Result<(), String> {
        if let Some(bundle) = self.bundles.get(bundle_name) {
            // Install dependencies first
            for dep in &bundle.dependencies {
                if !self.installed_bundles.contains(dep) {
                    self.install_bundle(dep)?;
                }
            }

            println!("Installing bundle: {} ({})", bundle_name, bundle.version);
            self.installed_bundles.push(bundle_name.to_string());
            Ok(())
        } else {
            Err(format!("Bundle {} not found", bundle_name))
        }
    }

    /// Remove bundle
    pub fn remove_bundle(&mut self, bundle_name: &str) -> Result<(), String> {
        if let Some(pos) = self.installed_bundles.iter().position(|b| b == bundle_name) {
            self.installed_bundles.remove(pos);
            println!("Removed bundle: {}", bundle_name);
            Ok(())
        } else {
            Err(format!("Bundle {} not installed", bundle_name))
        }
    }

    /// List installed bundles
    pub fn list_installed(&self) -> Vec<&String> {
        self.installed_bundles.iter().collect()
    }

    /// Search bundles
    pub fn search_bundles(&self, query: &str) -> Vec<&SwupdBundle> {
        let query_lower = query.to_lowercase();
        self.bundles.values()
            .filter(|b| b.name.to_lowercase().contains(&query_lower))
            .collect()
    }
}

impl Default for SwupdUpdateManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stateless_engine() {
        let mut engine = ClearLinuxStatelessEngine::new();
        engine.add_default_config("/etc/hostname".to_string(), "sigmaos".to_string());
        
        let config = engine.get_config("/etc/hostname");
        assert_eq!(config, Some("sigmaos".to_string()));
    }

    #[test]
    fn test_config_override() {
        let mut engine = ClearLinuxStatelessEngine::new();
        engine.add_default_config("/etc/hostname".to_string(), "sigmaos".to_string());
        engine.override_config("/etc/hostname".to_string(), "custom-host".to_string());
        
        let config = engine.get_config("/etc/hostname");
        assert_eq!(config, Some("custom-host".to_string()));
    }

    #[test]
    fn test_swupd_manager() {
        let mut manager = SwupdUpdateManager::new();
        
        let bundle = SwupdBundle {
            name: "os-core".to_string(),
            version: "12345".to_string(),
            size: 1024 * 1024 * 100,
            dependencies: vec![],
        };
        
        manager.add_bundle(bundle);
        let result = manager.install_bundle("os-core");
        assert!(result.is_ok());
    }
}
