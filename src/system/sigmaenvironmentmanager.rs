// SPDX-License-Identifier: MIT
// SigmaOS - Environment Manager
// Environment variable management inspired by Omarchy environment and Linux env systems

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Environment variable
#[derive(Debug, Clone)]
pub struct EnvironmentVariable {
    pub key: String,
    pub value: String,
}

/// Environment manager
pub struct SigmaEnvironmentManager {
    variables: Vec<EnvironmentVariable>,
}

impl SigmaEnvironmentManager {
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
        }
    }

    pub fn set_variable(&mut self, key: String, value: String) {
        if let Some(idx) = self.variables.iter().position(|v| v.key == key) {
            self.variables[idx].value = value;
        } else {
            self.variables.push(EnvironmentVariable { key, value });
        }
    }

    pub fn get_variable(&self, key: &str) -> Option<&String> {
        self.variables
            .iter()
            .find(|v| v.key == key)
            .map(|v| &v.value)
    }

    pub fn get_variables(&self) -> &[EnvironmentVariable] {
        &self.variables
    }

    pub fn unset_variable(&mut self, key: String) -> Result<(), String> {
        if let Some(idx) = self.variables.iter().position(|v| v.key == key) {
            self.variables.remove(idx);
            Ok(())
        } else {
            Err(format!("Variable {} not found", key))
        }
    }
}

impl Default for SigmaEnvironmentManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_manager_creation() {
        let manager = SigmaEnvironmentManager::new();
        assert_eq!(manager.get_variables().len(), 0);
    }

    #[test]
    fn test_set_variable() {
        let mut manager = SigmaEnvironmentManager::new();
        manager.set_variable("PATH".to_string(), "/usr/bin".to_string());
        assert_eq!(manager.get_variables().len(), 1);
    }

    #[test]
    fn test_get_variable() {
        let mut manager = SigmaEnvironmentManager::new();
        manager.set_variable("PATH".to_string(), "/usr/bin".to_string());
        assert_eq!(manager.get_variable("PATH"), Some(&"/usr/bin".to_string()));
    }

    #[test]
    fn test_unset_variable() {
        let mut manager = SigmaEnvironmentManager::new();
        manager.set_variable("PATH".to_string(), "/usr/bin".to_string());
        manager.unset_variable("PATH".to_string()).unwrap();
        assert_eq!(manager.get_variables().len(), 0);
    }
}
