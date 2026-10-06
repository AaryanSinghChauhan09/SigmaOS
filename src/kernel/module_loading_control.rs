// Kernel Module Loading Control Subsystem for SigmaOS
// Kernel module control per Wiki 04-Kernel.md
// Provides kernel module loading, signing, and security policy enforcement

use std::collections::HashMap;
use std::string::{String, ToString};
use std::vec::Vec;

/// Module signature state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleSignatureState {
    Valid,
    Invalid,
    Unsigned,
    UnknownKey,
}

impl ModuleSignatureState {
    pub fn as_str(&self) -> &str {
        match self {
            ModuleSignatureState::Valid => "valid",
            ModuleSignatureState::Invalid => "invalid",
            ModuleSignatureState::Unsigned => "unsigned",
            ModuleSignatureState::UnknownKey => "unknown_key",
        }
    }
}

/// Module loading policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleLoadPolicy {
    AllowAll,
    SignedOnly,
    SignedAndVerified,
    Disabled,
}

impl ModuleLoadPolicy {
    pub fn as_str(&self) -> &str {
        match self {
            ModuleLoadPolicy::AllowAll => "allow_all",
            ModuleLoadPolicy::SignedOnly => "signed_only",
            ModuleLoadPolicy::SignedAndVerified => "signed_and_verified",
            ModuleLoadPolicy::Disabled => "disabled",
        }
    }
}

/// Kernel module info
#[derive(Debug, Clone)]
pub struct KernelModuleInfo {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub license: String,
    pub dependencies: Vec<String>,
    pub signature_state: ModuleSignatureState,
    pub loaded: bool,
    pub ref_count: u32,
    pub address: usize,
    pub size: usize,
}

impl KernelModuleInfo {
    pub fn new(name: String, version: String) -> Self {
        KernelModuleInfo {
            name,
            version,
            author: String::new(),
            description: String::new(),
            license: String::from("GPL"),
            dependencies: Vec::new(),
            signature_state: ModuleSignatureState::Unsigned,
            loaded: false,
            ref_count: 0,
            address: 0,
            size: 0,
        }
    }

    pub fn with_author(mut self, author: String) -> Self {
        self.author = author;
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn with_license(mut self, license: String) -> Self {
        self.license = license;
        self
    }

    pub fn with_dependencies(mut self, dependencies: Vec<String>) -> Self {
        self.dependencies = dependencies;
        self
    }

    pub fn with_signature(mut self, signature_state: ModuleSignatureState) -> Self {
        self.signature_state = signature_state;
        self
    }
}

/// Module loading rule
#[derive(Debug, Clone)]
pub struct ModuleRule {
    pub module_name: String,
    pub allowed: bool,
    pub reason: String,
}

impl ModuleRule {
    pub fn allow(module_name: String, reason: String) -> Self {
        ModuleRule {
            module_name,
            allowed: true,
            reason,
        }
    }

    pub fn deny(module_name: String, reason: String) -> Self {
        ModuleRule {
            module_name,
            allowed: false,
            reason,
        }
    }
}

/// Kernel module loading controller
#[derive(Debug, Clone)]
pub struct ModuleLoadingController {
    pub policy: ModuleLoadPolicy,
    pub modules: HashMap<String, KernelModuleInfo>,
    pub rules: HashMap<String, ModuleRule>,
    pub trusted_keys: Vec<String>,
    pub locked: bool,
}

impl Default for ModuleLoadingController {
    fn default() -> Self {
        ModuleLoadingController {
            policy: ModuleLoadPolicy::SignedAndVerified,
            modules: HashMap::new(),
            rules: HashMap::new(),
            trusted_keys: Vec::new(),
            locked: false,
        }
    }
}

impl ModuleLoadingController {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_policy(&mut self, policy: ModuleLoadPolicy) -> Result<(), &'static str> {
        if self.locked {
            return Err("Module loading policy is locked");
        }
        self.policy = policy;
        Ok(())
    }

    pub fn lock_policy(&mut self) {
        self.locked = true;
    }

    pub fn add_rule(&mut self, rule: ModuleRule) -> Result<(), &'static str> {
        if self.locked {
            return Err("Module loading policy is locked");
        }
        self.rules.insert(rule.module_name.clone(), rule);
        Ok(())
    }

    pub fn add_trusted_key(&mut self, key_id: String) {
        if !self.trusted_keys.contains(&key_id) {
            self.trusted_keys.push(key_id);
        }
    }

    pub fn register_module(&mut self, module: KernelModuleInfo) -> Result<(), &'static str> {
        if self.modules.contains_key(&module.name) {
            return Err("Module already registered");
        }
        self.modules.insert(module.name.clone(), module);
        Ok(())
    }

    pub fn is_module_allowed(&self, name: &str) -> (bool, String) {
        if self.policy == ModuleLoadPolicy::Disabled {
            return (false, String::from("Module loading is disabled"));
        }

        if let Some(rule) = self.rules.get(name) {
            return (rule.allowed, rule.reason.clone());
        }

        if let Some(module) = self.modules.get(name) {
            match self.policy {
                ModuleLoadPolicy::AllowAll => (true, String::from("Policy allows all modules")),
                ModuleLoadPolicy::SignedOnly => match module.signature_state {
                    ModuleSignatureState::Valid | ModuleSignatureState::UnknownKey => {
                        (true, String::from("Module is signed"))
                    }
                    _ => (false, String::from("Module is not signed")),
                },
                ModuleLoadPolicy::SignedAndVerified => match module.signature_state {
                    ModuleSignatureState::Valid => (true, String::from("Module signature is valid")),
                    _ => (false, String::from("Module signature is not valid")),
                },
                ModuleLoadPolicy::Disabled => (false, String::from("Module loading is disabled")),
            }
        } else {
            (false, String::from("Module not found"))
        }
    }

    pub fn load_module(&mut self, name: &str) -> Result<String, String> {
        let (allowed, reason) = self.is_module_allowed(name);
        if !allowed {
            return Err(format!("Cannot load module {}: {}", name, reason));
        }

        if let Some(module) = self.modules.get_mut(name) {
            if module.loaded {
                module.ref_count += 1;
                Ok(format!("Module {} ref count increased to {}", name, module.ref_count))
            } else {
                module.loaded = true;
                module.ref_count = 1;
                Ok(format!("Module {} loaded successfully", name))
            }
        } else {
            Err(format!("Module {} not found", name))
        }
    }

    pub fn unload_module(&mut self, name: &str) -> Result<String, String> {
        if let Some(module) = self.modules.get_mut(name) {
            if !module.loaded {
                return Err(format!("Module {} is not loaded", name));
            }

            if module.ref_count > 1 {
                module.ref_count -= 1;
                Ok(format!("Module {} ref count decreased to {}", name, module.ref_count))
            } else {
                module.loaded = false;
                module.ref_count = 0;
                Ok(format!("Module {} unloaded successfully", name))
            }
        } else {
            Err(format!("Module {} not found", name))
        }
    }

    pub fn get_module_info(&self, name: &str) -> Option<&KernelModuleInfo> {
        self.modules.get(name)
    }

    pub fn list_loaded_modules(&self) -> Vec<String> {
        self.modules
            .values()
            .filter(|m| m.loaded)
            .map(|m| format!("{} (v{}) - ref: {}", m.name, m.version, m.ref_count))
            .collect()
    }

    pub fn list_all_modules(&self) -> Vec<String> {
        self.modules
            .values()
            .map(|m| {
                format!(
                    "{} (v{}) [{}] - {}",
                    m.name,
                    m.version,
                    if m.loaded { "loaded" } else { "unloaded" },
                    m.signature_state.as_str()
                )
            })
            .collect()
    }

    pub fn get_statistics(&self) -> String {
        let total = self.modules.len();
        let loaded = self.modules.values().filter(|m| m.loaded).count();
        let valid_sig = self
            .modules
            .values()
            .filter(|m| m.signature_state == ModuleSignatureState::Valid)
            .count();

        format!(
            "Module Loading Controller Stats:\nTotal registered: {}\nLoaded: {}\nValid signature: {}\nPolicy: {}\nLocked: {}",
            total, loaded, valid_sig, self.policy.as_str(), self.locked
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_loading_controller() {
        let mut controller = ModuleLoadingController::new();
        let module = KernelModuleInfo::new(String::from("test_mod"), String::from("1.0.0"))
            .with_signature(ModuleSignatureState::Valid);

        assert!(controller.register_module(module).is_ok());
        assert!(controller.load_module("test_mod").is_ok());
        assert_eq!(controller.list_loaded_modules().len(), 1);
    }
}
