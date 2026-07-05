// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS Environment Manager - Development environment management

use serde::{Deserialize, Serialize};

/// Environment Manager for development environments
pub struct EnvironmentManager {
    environments: Vec<DevelopmentEnvironment>,
    active_environment: String,
}

impl EnvironmentManager {
    /// Create a new Environment Manager
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let environments = Self::load_environments()?;
        let active_environment = "python".to_string();
        
        Ok(Self {
            environments,
            active_environment,
        })
    }

    /// Load development environments
    fn load_environments() -> Result<Vec<DevelopmentEnvironment>, Box<dyn std::error::Error>> {
        Ok(vec![
            DevelopmentEnvironment {
                id: "python".to_string(),
                name: "Python".to_string(),
                language: "Python".to_string(),
                version: "3.11".to_string(),
                packages: vec!["numpy".to_string(), "pandas".to_string()],
                status: EnvironmentStatus::Active,
            },
            DevelopmentEnvironment {
                id: "rust".to_string(),
                name: "Rust".to_string(),
                language: "Rust".to_string(),
                version: "1.75".to_string(),
                packages: vec!["serde".to_string(), "tokio".to_string()],
                status: EnvironmentStatus::Inactive,
            },
            DevelopmentEnvironment {
                id: "go".to_string(),
                name: "Go".to_string(),
                language: "Go".to_string(),
                version: "1.21".to_string(),
                packages: vec!["gin".to_string(), "gorm".to_string()],
                status: EnvironmentStatus::Inactive,
            },
        ])
    }

    /// Activate an environment
    pub fn activate_environment(&mut self, environment_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(env) = self.environments.iter_mut().find(|e| e.id == environment_id) {
            // Deactivate current environment
            if let Some(current) = self.environments.iter_mut().find(|e| e.id == self.active_environment) {
                current.status = EnvironmentStatus::Inactive;
            }
            
            // Activate new environment
            env.status = EnvironmentStatus::Active;
            self.active_environment = environment_id.to_string();
            Ok(())
        } else {
            Err(format!("Environment {} not found", environment_id).into())
        }
    }

    /// Create a new environment
    pub fn create_environment(&mut self, config: EnvironmentConfig) -> Result<String, Box<dyn std::error::Error>> {
        let environment_id = config.language.to_lowercase().replace(' ', "_");
        
        let environment = DevelopmentEnvironment {
            id: environment_id.clone(),
            name: config.name,
            language: config.language,
            version: config.version,
            packages: config.packages,
            status: EnvironmentStatus::Inactive,
        };
        
        self.environments.push(environment);
        Ok(environment_id)
    }

    /// Install a package in an environment
    pub fn install_package(&mut self, environment_id: &str, package: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(env) = self.environments.iter_mut().find(|e| e.id == environment_id) {
            env.packages.push(package.to_string());
            Ok(())
        } else {
            Err(format!("Environment {} not found", environment_id).into())
        }
    }

    /// Remove a package from an environment
    pub fn remove_package(&mut self, environment_id: &str, package: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(env) = self.environments.iter_mut().find(|e| e.id == environment_id) {
            env.packages.retain(|p| p != package);
            Ok(())
        } else {
            Err(format!("Environment {} not found", environment_id).into())
        }
    }

    /// Delete an environment
    pub fn delete_environment(&mut self, environment_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        if environment_id == self.active_environment {
            return Err("Cannot delete active environment".into());
        }
        
        if let Some(pos) = self.environments.iter().position(|e| e.id == environment_id) {
            self.environments.remove(pos);
            Ok(())
        } else {
            Err(format!("Environment {} not found", environment_id).into())
        }
    }

    /// Get all environments
    pub fn get_environments(&self) -> Vec<DevelopmentEnvironment> {
        self.environments.clone()
    }

    /// Get active environment
    pub fn get_active_environment(&self) -> String {
        self.active_environment.clone()
    }

    /// Get environment by ID
    pub fn get_environment(&self, environment_id: &str) -> Option<&DevelopmentEnvironment> {
        self.environments.iter().find(|e| e.id == environment_id)
    }
}

/// Development environment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentEnvironment {
    pub id: String,
    pub name: String,
    pub language: String,
    pub version: String,
    pub packages: Vec<String>,
    pub status: EnvironmentStatus,
}

/// Environment status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnvironmentStatus {
    Active,
    Inactive,
    Error,
}

/// Environment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentConfig {
    pub name: String,
    pub language: String,
    pub version: String,
    pub packages: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_manager_creation() {
        let manager = EnvironmentManager::new();
        assert!(manager.is_ok());
    }

    #[test]
    fn test_activate_environment() {
        let mut manager = EnvironmentManager::new().unwrap();
        let result = manager.activate_environment("rust");
        assert!(result.is_ok());
        assert_eq!(manager.get_active_environment(), "rust");
    }
}
