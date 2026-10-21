#![no_std]
extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Kernel manager for kernel parameter and module management
/// Inspired by Linux sysctl, modprobe, and kernel parameter management
#[derive(Debug, Clone)]
pub struct KernelManager {
    parameters: Vec<KernelParameter>,
    modules: Vec<KernelModule>,
}

/// Kernel parameter
#[derive(Debug, Clone)]
pub struct KernelParameter {
    name: String,
    value: String,
    param_type: ParameterType,
    state: ParameterState,
}

/// Parameter type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterType {
    Integer,
    String,
    Boolean,
    Hex,
}

/// Parameter state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterState {
    Active,
    Default,
    ReadOnly,
}

/// Kernel module
#[derive(Debug, Clone)]
pub struct KernelModule {
    name: String,
    state: ModuleState,
    dependencies: Vec<String>,
}

/// Module state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleState {
    Loaded,
    Unloaded,
    BuiltIn,
    Blacklisted,
}

impl KernelManager {
    pub fn new() -> Self {
        Self {
            parameters: Vec::new(),
            modules: Vec::new(),
        }
    }

    /// Set a kernel parameter
    pub fn set_parameter(&mut self, name: String, value: String, param_type: ParameterType) -> Result<(), String> {
        if let Some(param) = self.parameters.iter_mut().find(|p| p.name == name) {
            if param.state == ParameterState::ReadOnly {
                return Err(format!("Parameter {} is read-only", name));
            }
            param.value = value;
            param.param_type = param_type;
            Ok(())
        } else {
            let param = KernelParameter {
                name,
                value,
                param_type,
                state: ParameterState::Active,
            };
            self.parameters.push(param);
            Ok(())
        }
    }

    /// Get a kernel parameter
    pub fn get_parameter(&self, name: &str) -> Option<&KernelParameter> {
        self.parameters.iter().find(|p| p.name == name)
    }

    /// Remove a parameter
    pub fn remove_parameter(&mut self, name: &str) -> Result<(), String> {
        let pos = self.parameters.iter().position(|p| p.name == name)
            .ok_or_else(|| format!("Parameter {} not found", name))?;
        self.parameters.remove(pos);
        Ok(())
    }

    /// Load a kernel module
    pub fn load_module(&mut self, name: String, dependencies: Vec<String>) -> Result<(), String> {
        if self.modules.iter().any(|m| m.name == name) {
            return Err(format!("Module {} already exists", name));
        }
        
        let module = KernelModule {
            name,
            state: ModuleState::Loaded,
            dependencies,
        };
        self.modules.push(module);
        Ok(())
    }

    /// Unload a kernel module
    pub fn unload_module(&mut self, name: &str) -> Result<(), String> {
        let module = self.modules.iter_mut().find(|m| m.name == name)
            .ok_or_else(|| format!("Module {} not found", name))?;
        if module.state == ModuleState::BuiltIn {
            return Err(format!("Module {} is built-in and cannot be unloaded", name));
        }
        module.state = ModuleState::Unloaded;
        Ok(())
    }

    /// Blacklist a module
    pub fn blacklist_module(&mut self, name: &str) -> Result<(), String> {
        let module = self.modules.iter_mut().find(|m| m.name == name)
            .ok_or_else(|| format!("Module {} not found", name))?;
        module.state = ModuleState::Blacklisted;
        Ok(())
    }

    /// Get all parameters
    pub fn get_parameters(&self) -> &[KernelParameter] {
        &self.parameters
    }

    /// Get all modules
    pub fn get_modules(&self) -> &[KernelModule] {
        &self.modules
    }

    /// Get loaded modules
    pub fn get_loaded_modules(&self) -> Vec<&KernelModule> {
        self.modules.iter().filter(|m| m.state == ModuleState::Loaded).collect()
    }

    /// Get module by name
    pub fn get_module(&self, name: &str) -> Option<&KernelModule> {
        self.modules.iter().find(|m| m.name == name)
    }
}

impl Default for KernelManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_parameter() {
        let mut manager = KernelManager::new();
        let result = manager.set_parameter("kernel.max_pid".to_string(), "32768".to_string(), ParameterType::Integer);
        assert!(result.is_ok());
        assert_eq!(manager.get_parameters().len(), 1);
    }

    #[test]
    fn test_get_parameter() {
        let mut manager = KernelManager::new();
        manager.set_parameter("kernel.max_pid".to_string(), "32768".to_string(), ParameterType::Integer).unwrap();
        
        let param = manager.get_parameter("kernel.max_pid");
        assert!(param.is_some());
        assert_eq!(param.unwrap().value, "32768");
    }

    #[test]
    fn test_readonly_parameter() {
        let mut manager = KernelManager::new();
        manager.set_parameter("kernel.hostname".to_string(), "sigmaos".to_string(), ParameterType::String).unwrap();
        
        // Mark as read-only by removing and re-adding with read-only state
        manager.remove_parameter("kernel.hostname").unwrap();
        let param = KernelParameter {
            name: "kernel.hostname".to_string(),
            value: "sigmaos".to_string(),
            param_type: ParameterType::String,
            state: ParameterState::ReadOnly,
        };
        manager.parameters.push(param);
        
        let result = manager.set_parameter("kernel.hostname".to_string(), "newhost".to_string(), ParameterType::String);
        assert!(result.is_err());
    }

    #[test]
    fn test_load_module() {
        let mut manager = KernelManager::new();
        let result = manager.load_module("ext4".to_string(), vec!["crc32c".to_string()]);
        assert!(result.is_ok());
        assert_eq!(manager.get_modules().len(), 1);
    }

    #[test]
    fn test_unload_module() {
        let mut manager = KernelManager::new();
        manager.load_module("ext4".to_string(), vec![]).unwrap();
        
        manager.unload_module("ext4").unwrap();
        assert_eq!(manager.get_module("ext4").unwrap().state, ModuleState::Unloaded);
    }

    #[test]
    fn test_blacklist_module() {
        let mut manager = KernelManager::new();
        manager.load_module("btrfs".to_string(), vec![]).unwrap();
        
        manager.blacklist_module("btrfs").unwrap();
        assert_eq!(manager.get_module("btrfs").unwrap().state, ModuleState::Blacklisted);
    }
}
