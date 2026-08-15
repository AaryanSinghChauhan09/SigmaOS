//! System Configuration Management (NixOS/Guix Inspiration)
//! Declarative configuration with atomic upgrades and rollback support

#![no_std]

extern crate alloc;

use crate::klib::{Vec, String};
use alloc::vec::Vec;
use alloc::string::String;

/// Configuration state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigState {
    Active,
    Inactive,
    Building,
    Error,
}

/// Configuration module
#[derive(Debug, Clone)]
pub struct ConfigModule {
    pub name: String,
    pub enabled: bool,
    pub dependencies: Vec<String>,
    pub config: String,
}

impl ConfigModule {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            enabled: true,
            dependencies: Vec::new(),
            config: String::new(),
        }
    }

    pub fn add_dependency(&mut self, dep: &str) {
        self.dependencies.push(dep.to_string());
    }

    pub fn set_config(&mut self, config: &str) {
        self.config = config.to_string();
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

/// System configuration
#[derive(Debug, Clone)]
pub struct SystemConfig {
    pub modules: Vec<ConfigModule>,
    pub packages: Vec<String>,
    pub services: Vec<String>,
    pub settings: Vec<(String, String)>,
}

impl SystemConfig {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
            packages: Vec::new(),
            services: Vec::new(),
            settings: Vec::new(),
        }
    }

    pub fn add_module(&mut self, module: ConfigModule) {
        self.modules.push(module);
    }

    pub fn add_package(&mut self, package: &str) {
        self.packages.push(package.to_string());
    }

    pub fn add_service(&mut self, service: &str) {
        self.services.push(service.to_string());
    }

    pub fn add_setting(&mut self, key: &str, value: &str) {
        self.settings.push((key.to_string(), value.to_string()));
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        // Validate configuration dependencies
        for module in &self.modules {
            for dep in &module.dependencies {
                if !self.modules.iter().any(|m| m.name == *dep) {
                    return Err(ConfigError::DependencyNotFound);
                }
            }
        }
        Ok(())
    }
}

/// Configuration generation
#[derive(Debug, Clone)]
pub struct ConfigGeneration {
    pub id: String,
    pub config: SystemConfig,
    pub state: ConfigState,
    pub timestamp: u64,
}

impl ConfigGeneration {
    pub fn new(config: SystemConfig) -> Self {
        Self {
            id: Self::generate_id(),
            config,
            state: ConfigState::Building,
            timestamp: 0,
        }
    }

    fn generate_id() -> String {
        "gen_abcdef1234567890".to_string()
    }

    pub fn activate(&mut self) -> Result<(), ConfigError> {
        self.config.validate()?;
        self.state = ConfigState::Active;
        Ok(())
    }

    pub fn deactivate(&mut self) {
        self.state = ConfigState::Inactive;
    }
}

/// Configuration manager
pub struct ConfigManager {
    pub config: SystemConfig,
    pub modules: Vec<ConfigModule>,
    pub generations: Vec<ConfigGeneration>,
    pub current_generation: Option<String>,
}

impl ConfigManager {
    pub fn new() -> Self {
        Self {
            config: SystemConfig::new(),
            modules: Vec::new(),
            generations: Vec::new(),
            current_generation: None,
        }
    }

    pub fn add_module(&mut self, module: ConfigModule) {
        self.modules.push(module);
    }

    pub fn create_generation(&mut self) -> Result<String, ConfigError> {
        let generation = ConfigGeneration::new(self.config.clone());
        let id = generation.id.clone();
        self.generations.push(generation);
        Ok(id)
    }

    pub fn activate_generation(&mut self, id: &str) -> Result<(), ConfigError> {
        if let Some(gen) = self.generations.iter_mut().find(|g| g.id == id) {
            gen.activate()?;
            self.current_generation = Some(id.to_string());
            Ok(())
        } else {
            Err(ConfigError::GenerationNotFound)
        }
    }

    pub fn rollback(&mut self) -> Result<(), ConfigError> {
        if let Some(current_id) = &self.current_generation {
            let current_index = self.generations.iter().position(|g| g.id == *current_id);
            if let Some(index) = current_index {
                if index > 0 {
                    let prev_gen = &self.generations[index - 1];
                    self.activate_generation(&prev_gen.id)?;
                }
            }
        }
        Ok(())
    }

    pub fn list_generations(&self) -> Vec<&ConfigGeneration> {
        self.generations.iter().collect()
    }

    pub fn get_config_stats(&self) -> ConfigStats {
        ConfigStats {
            total_modules: self.modules.len(),
            total_generations: self.generations.len(),
            active_generation: self.current_generation.clone(),
            total_packages: self.config.packages.len(),
            total_services: self.config.services.len(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConfigStats {
    pub total_modules: usize,
    pub total_generations: usize,
    pub active_generation: Option<String>,
    pub total_packages: usize,
    pub total_services: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigError {
    DependencyNotFound,
    GenerationNotFound,
    ValidationFailed,
    BuildFailed,
    ActivationFailed,
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_module() {
        let mut module = ConfigModule::new("test-module");
        module.add_dependency("dep1");
        assert_eq!(module.dependencies.len(), 1);
    }

    #[test]
    fn test_system_config() {
        let mut config = SystemConfig::new();
        config.add_package("test-package");
        assert_eq!(config.packages.len(), 1);
    }

    #[test]
    fn test_config_generation() {
        let config = SystemConfig::new();
        let mut gen = ConfigGeneration::new(config);
        assert!(gen.activate().is_ok());
    }

    #[test]
    fn test_config_manager() {
        let mut manager = ConfigManager::new();
        let module = ConfigModule::new("test-module");
        manager.add_module(module);
        assert_eq!(manager.modules.len(), 1);
    }

    #[test]
    fn test_rollback() {
        let mut manager = ConfigManager::new();
        let gen1_id = manager.create_generation().unwrap();
        let gen2_id = manager.create_generation().unwrap();
        manager.activate_generation(&gen2_id).unwrap();
        assert!(manager.rollback().is_ok());
    }
}