// Kernel Module Loading Control for SigmaOS
// Kernel module loading control per Wiki 07-Security.md
// Provides control over kernel module loading for security

use std::string::{String, ToString};
use std::vec::Vec;

/// Module loading state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleLoadingState {
    Enabled,
    Disabled,
    Restricted,
}

impl ModuleLoadingState {
    pub fn as_str(&self) -> &str {
        match self {
            ModuleLoadingState::Enabled => "enabled",
            ModuleLoadingState::Disabled => "disabled",
            ModuleLoadingState::Restricted => "restricted",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "enabled" => Some(ModuleLoadingState::Enabled),
            "disabled" => Some(ModuleLoadingState::Disabled),
            "restricted" => Some(ModuleLoadingState::Restricted),
            _ => None,
        }
    }
}

/// Module loading policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleLoadingPolicy {
    AllowAll,
    AllowSigned,
    AllowWhitelist,
    DenyAll,
}

impl ModuleLoadingPolicy {
    pub fn as_str(&self) -> &str {
        match self {
            ModuleLoadingPolicy::AllowAll => "allow_all",
            ModuleLoadingPolicy::AllowSigned => "allow_signed",
            ModuleLoadingPolicy::AllowWhitelist => "allow_whitelist",
            ModuleLoadingPolicy::DenyAll => "deny_all",
        }
    }
}

/// Kernel module info
#[derive(Debug, Clone)]
pub struct KernelModule {
    pub name: String,
    pub version: String,
    pub loaded: bool,
    pub signature_verified: bool,
    pub load_time: Option<u64>,
}

impl KernelModule {
    pub fn new(name: String, version: String) -> Self {
        KernelModule {
            name,
            version,
            loaded: false,
            signature_verified: false,
            load_time: None,
        }
    }

    pub fn set_loaded(&mut self, loaded: bool) {
        self.loaded = loaded;
    }

    pub fn set_signature_verified(&mut self, verified: bool) {
        self.signature_verified = verified;
    }

    pub fn set_load_time(&mut self, time: u64) {
        self.load_time = Some(time);
    }
}

/// Module loading rule
#[derive(Debug, Clone)]
pub struct ModuleLoadingRule {
    pub module_name: String,
    pub allowed: bool,
    pub requires_signature: bool,
    pub description: String,
}

impl ModuleLoadingRule {
    pub fn new(
        module_name: String,
        allowed: bool,
        requires_signature: bool,
        description: String,
    ) -> Self {
        ModuleLoadingRule {
            module_name,
            allowed,
            requires_signature,
            description,
        }
    }
}

/// Kernel module loading controller
#[derive(Debug, Clone)]
pub struct KernelModuleLoadingController {
    pub loading_state: ModuleLoadingState,
    pub loading_policy: ModuleLoadingPolicy,
    pub modules: Vec<KernelModule>,
    pub rules: Vec<ModuleLoadingRule>,
    pub signature_checking_enabled: bool,
}

impl Default for KernelModuleLoadingController {
    fn default() -> Self {
        KernelModuleLoadingController {
            loading_state: ModuleLoadingState::Enabled,
            loading_policy: ModuleLoadingPolicy::AllowAll,
            modules: Vec::new(),
            rules: Vec::new(),
            signature_checking_enabled: false,
        }
    }
}

impl KernelModuleLoadingController {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set module loading state
    pub fn set_loading_state(&mut self, state: ModuleLoadingState) {
        self.loading_state = state;
    }

    /// Set module loading policy
    pub fn set_loading_policy(&mut self, policy: ModuleLoadingPolicy) {
        self.loading_policy = policy;
    }

    /// Enable/disable signature checking
    pub fn set_signature_checking(&mut self, enabled: bool) {
        self.signature_checking_enabled = enabled;
    }

    /// Enable module loading
    pub fn enable_module_loading(&mut self) {
        self.loading_state = ModuleLoadingState::Enabled;
    }

    /// Disable module loading
    pub fn disable_module_loading(&mut self) {
        self.loading_state = ModuleLoadingState::Disabled;
    }

    /// Add module loading rule
    pub fn add_rule(&mut self, rule: ModuleLoadingRule) {
        self.rules.push(rule);
    }

    /// Remove module loading rule
    pub fn remove_rule(&mut self, module_name: &str) -> bool {
        if let Some(pos) = self.rules.iter().position(|r| r.module_name == module_name) {
            self.rules.remove(pos);
            true
        } else {
            false
        }
    }

    /// Get rule for module
    pub fn get_rule(&self, module_name: &str) -> Option<&ModuleLoadingRule> {
        self.rules.iter().find(|r| r.module_name == module_name)
    }

    /// Check if module can be loaded
    pub fn can_load_module(
        &self,
        module_name: &str,
        signature_verified: bool,
    ) -> Result<bool, String> {
        if self.loading_state == ModuleLoadingState::Disabled {
            return Err(String::from("Module loading is disabled"));
        }

        match self.loading_policy {
            ModuleLoadingPolicy::DenyAll => {
                return Err(String::from("Module loading policy denies all modules"));
            }
            ModuleLoadingPolicy::AllowSigned => {
                if !signature_verified {
                    return Err(String::from("Module signature verification required"));
                }
            }
            ModuleLoadingPolicy::AllowWhitelist => {
                if let Some(rule) = self.get_rule(module_name) {
                    if !rule.allowed {
                        return Err(format!("Module {} is not whitelisted", module_name));
                    }
                    if rule.requires_signature && !signature_verified {
                        return Err(format!(
                            "Module {} requires signature verification",
                            module_name
                        ));
                    }
                } else {
                    return Err(format!("Module {} is not in whitelist", module_name));
                }
            }
            ModuleLoadingPolicy::AllowAll => {
                // Allow all modules
            }
        }

        Ok(true)
    }

    /// Register module
    pub fn register_module(&mut self, module: KernelModule) {
        self.modules.push(module);
    }

    /// Load module
    pub fn load_module(
        &mut self,
        module_name: &str,
        signature_verified: bool,
    ) -> Result<(), String> {
        self.can_load_module(module_name, signature_verified)?;

        if let Some(module) = self.modules.iter_mut().find(|m| m.name == module_name) {
            module.set_loaded(true);
            module.set_signature_verified(signature_verified);
            module.set_load_time(0); // Would be actual timestamp
            Ok(())
        } else {
            Err(format!("Module {} not found", module_name))
        }
    }

    /// Unload module
    pub fn unload_module(&mut self, module_name: &str) -> Result<(), String> {
        if let Some(module) = self.modules.iter_mut().find(|m| m.name == module_name) {
            if !module.loaded {
                return Err(format!("Module {} is not loaded", module_name));
            }
            module.set_loaded(false);
            module.load_time = None;
            Ok(())
        } else {
            Err(format!("Module {} not found", module_name))
        }
    }

    /// Get module
    pub fn get_module(&self, module_name: &str) -> Option<&KernelModule> {
        self.modules.iter().find(|m| m.name == module_name)
    }

    /// List loaded modules
    pub fn list_loaded_modules(&self) -> Vec<String> {
        self.modules
            .iter()
            .filter(|m| m.loaded)
            .map(|m| {
                format!(
                    "{} {} ({})",
                    m.name,
                    m.version,
                    if m.signature_verified {
                        "signed"
                    } else {
                        "unsigned"
                    }
                )
            })
            .collect()
    }

    /// List all modules
    pub fn list_all_modules(&self) -> Vec<String> {
        self.modules
            .iter()
            .map(|m| {
                format!(
                    "{} {} ({}, {})",
                    m.name,
                    m.version,
                    if m.loaded { "loaded" } else { "not loaded" },
                    if m.signature_verified {
                        "signed"
                    } else {
                        "unsigned"
                    }
                )
            })
        self.modules.iter()
            .map(|m| format!("{} {} ({}, {})",
                m.name,
                m.version,
                if m.loaded { "loaded" } else { "not loaded" },
                if m.signature_verified { "signed" } else { "unsigned" }
            ))
            .collect()
    }

    /// List rules
    pub fn list_rules(&self) -> Vec<String> {
        self.rules
            .iter()
            .map(|r| {
                format!(
                    "{}: {} (signature required: {})",
                    r.module_name,
                    if r.allowed { "allowed" } else { "denied" },
                    r.requires_signature
                )
            })
        self.rules.iter()
            .map(|r| format!("{}: {} (signature required: {})",
                r.module_name,
                if r.allowed { "allowed" } else { "denied" },
                r.requires_signature
            ))
            .collect()
    }

    /// Get statistics
    pub fn get_statistics(&self) -> String {
        let mut stats = String::from("Kernel Module Loading Statistics:\n");
        stats.push_str(&format!("Loading state: {}\n", self.loading_state.as_str()));
        stats.push_str(&format!(
            "Loading policy: {}\n",
            self.loading_policy.as_str()
        ));
        stats.push_str(&format!(
            "Signature checking: {}\n",
            if self.signature_checking_enabled {
                "enabled"
            } else {
                "disabled"
            }
        ));
        stats.push_str(&format!("Loading policy: {}\n", self.loading_policy.as_str()));
        stats.push_str(&format!("Signature checking: {}\n", if self.signature_checking_enabled { "enabled" } else { "disabled" }));

        let total_modules = self.modules.len();
        let loaded_modules = self.modules.iter().filter(|m| m.loaded).count();
        let signed_modules = self.modules.iter().filter(|m| m.signature_verified).count();

        stats.push_str(&format!("Total modules: {}\n", total_modules));
        stats.push_str(&format!("Loaded modules: {}\n", loaded_modules));
        stats.push_str(&format!("Signed modules: {}\n", signed_modules));
        stats.push_str(&format!("Rules: {}\n", self.rules.len()));

        stats
    }

    /// Check if module loading is enabled
    pub fn is_loading_enabled(&self) -> bool {
        self.loading_state == ModuleLoadingState::Enabled
    }

    /// Check if module loading is disabled
    pub fn is_loading_disabled(&self) -> bool {
        self.loading_state == ModuleLoadingState::Disabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_loading_state_as_str() {
        assert_eq!(ModuleLoadingState::Enabled.as_str(), "enabled");
        assert_eq!(ModuleLoadingState::Disabled.as_str(), "disabled");
    }

    #[test]
    fn test_module_loading_state_from_str() {
        assert_eq!(
            ModuleLoadingState::from_str("enabled"),
            Some(ModuleLoadingState::Enabled)
        );
        assert_eq!(ModuleLoadingState::from_str("invalid"), None);
    }

    #[test]
    fn test_module_loading_policy_as_str() {
        assert_eq!(ModuleLoadingPolicy::AllowAll.as_str(), "allow_all");
        assert_eq!(ModuleLoadingPolicy::DenyAll.as_str(), "deny_all");
    }

    #[test]
    fn test_kernel_module_creation() {
        let module = KernelModule::new(String::from("test_module"), String::from("1.0.0"));
        assert_eq!(module.name, "test_module");
        assert!(!module.loaded);
    }

    #[test]
    fn test_kernel_module_set_loaded() {
        let mut module = KernelModule::new(String::from("test_module"), String::from("1.0.0"));
        module.set_loaded(true);
        assert!(module.loaded);
    }

    #[test]
    fn test_module_loading_rule_creation() {
        let rule = ModuleLoadingRule::new(
            String::from("test_module"),
            true,
            false,
            String::from("Test module rule"),
        );
        assert_eq!(rule.module_name, "test_module");
        assert!(rule.allowed);
    }

    #[test]
    fn test_kernel_module_loading_controller_creation() {
        let controller = KernelModuleLoadingController::new();
        assert_eq!(controller.loading_state, ModuleLoadingState::Enabled);
        assert_eq!(controller.loading_policy, ModuleLoadingPolicy::AllowAll);
    }

    #[test]
    fn test_kernel_module_loading_controller_set_loading_state() {
        let mut controller = KernelModuleLoadingController::new();
        controller.set_loading_state(ModuleLoadingState::Disabled);
        assert_eq!(controller.loading_state, ModuleLoadingState::Disabled);
    }

    #[test]
    fn test_kernel_module_loading_controller_enable_module_loading() {
        let mut controller = KernelModuleLoadingController::new();
        controller.disable_module_loading();
        controller.enable_module_loading();
        assert_eq!(controller.loading_state, ModuleLoadingState::Enabled);
    }

    #[test]
    fn test_kernel_module_loading_controller_disable_module_loading() {
        let mut controller = KernelModuleLoadingController::new();
        controller.disable_module_loading();
        assert_eq!(controller.loading_state, ModuleLoadingState::Disabled);
    }

    #[test]
    fn test_kernel_module_loading_controller_can_load_module_disabled() {
        let mut controller = KernelModuleLoadingController::new();
        controller.disable_module_loading();
        assert!(controller.can_load_module("test", true).is_err());
    }

    #[test]
    fn test_kernel_module_loading_controller_can_load_module_deny_all() {
        let mut controller = KernelModuleLoadingController::new();
        controller.set_loading_policy(ModuleLoadingPolicy::DenyAll);
        assert!(controller.can_load_module("test", true).is_err());
    }

    #[test]
    fn test_kernel_module_loading_controller_can_load_module_allow_signed() {
        let mut controller = KernelModuleLoadingController::new();
        controller.set_loading_policy(ModuleLoadingPolicy::AllowSigned);
        assert!(controller.can_load_module("test", false).is_err());
        assert!(controller.can_load_module("test", true).is_ok());
    }

    #[test]
    fn test_kernel_module_loading_controller_register_module() {
        let mut controller = KernelModuleLoadingController::new();
        controller.register_module(KernelModule::new(String::from("test"), String::from("1.0")));
        assert_eq!(controller.modules.len(), 1);
    }

    #[test]
    fn test_kernel_module_loading_controller_load_module() {
        let mut controller = KernelModuleLoadingController::new();
        controller.register_module(KernelModule::new(String::from("test"), String::from("1.0")));
        assert!(controller.load_module("test", true).is_ok());
    }

    #[test]
    fn test_kernel_module_loading_controller_unload_module() {
        let mut controller = KernelModuleLoadingController::new();
        controller.register_module(KernelModule::new(String::from("test"), String::from("1.0")));
        controller.load_module("test", true).unwrap();
        assert!(controller.unload_module("test").is_ok());
    }

    #[test]
    fn test_kernel_module_loading_controller_list_loaded_modules() {
        let mut controller = KernelModuleLoadingController::new();
        controller.register_module(KernelModule::new(String::from("test"), String::from("1.0")));
        controller.load_module("test", true).unwrap();
        let loaded = controller.list_loaded_modules();
        assert_eq!(loaded.len(), 1);
    }

    #[test]
    fn test_kernel_module_loading_controller_get_statistics() {
        let mut controller = KernelModuleLoadingController::new();
        controller.register_module(KernelModule::new(String::from("test"), String::from("1.0")));
        let stats = controller.get_statistics();
        assert!(stats.contains("Total modules: 1"));
    }

    #[test]
    fn test_kernel_module_loading_controller_is_loading_enabled() {
        let controller = KernelModuleLoadingController::new();
        assert!(controller.is_loading_enabled());
    }

    #[test]
    fn test_kernel_module_loading_controller_is_loading_disabled() {
        let mut controller = KernelModuleLoadingController::new();
        controller.disable_module_loading();
        assert!(controller.is_loading_disabled());
    }
}
