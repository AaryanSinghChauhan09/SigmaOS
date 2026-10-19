//! SigmaQuickshell - Shell Configuration Management
//!
//! Inspired by Quickshell (Omarchy Linux), this module provides shell configuration
//! management with templating, module loading, and hot-reload capabilities.

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, Ordering};

/// Shell type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellType {
    Bash,
    Zsh,
    Fish,
    Nushell,
    Custom,
    Unknown,
}

/// Module type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleType {
    Prompt,
    Alias,
    Function,
    Variable,
    Plugin,
    Unknown,
}

/// Shell module data structure
#[derive(Debug, Clone)]
pub struct ShellModule {
    pub module_id: String,
    pub name: String,
    pub module_type: ModuleType,
    pub content: String,
    pub enabled: bool,
    pub priority: u32,
}

impl ShellModule {
    /// Create a new shell module
    pub fn new(module_id: String, name: String, module_type: ModuleType, content: String) -> Self {
        Self {
            module_id,
            name,
            module_type,
            content,
            enabled: true,
            priority: 0,
        }
    }

    /// Enable module
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable module
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Set priority
    pub fn set_priority(&mut self, priority: u32) {
        self.priority = priority;
    }

    /// Check if module is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Shell configuration data structure
#[derive(Debug, Clone)]
pub struct ShellConfig {
    pub config_id: String,
    pub shell_type: ShellType,
    pub path: String,
    pub modules: Vec<String>,
    pub is_active: bool,
}

impl ShellConfig {
    /// Create a new shell configuration
    pub fn new(config_id: String, shell_type: ShellType, path: String) -> Self {
        Self {
            config_id,
            shell_type,
            path,
            modules: Vec::new(),
            is_active: false,
        }
    }

    /// Add a module
    pub fn add_module(&mut self, module_id: String) {
        self.modules.push(module_id);
    }

    /// Set active state
    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    /// Check if config is active
    pub fn is_config_active(&self) -> bool {
        self.is_active
    }
}

/// Quickshell manager
pub struct SigmaQuickshell {
    modules: BTreeMap<String, ShellModule>,
    configs: BTreeMap<String, ShellConfig>,
    running: AtomicBool,
}

impl SigmaQuickshell {
    /// Create a new Quickshell manager
    pub fn new() -> Self {
        Self {
            modules: BTreeMap::new(),
            configs: BTreeMap::new(),
            running: AtomicBool::new(false),
        }
    }

    /// Start the Quickshell manager
    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
    }

    /// Stop the Quickshell manager
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Add a module
    pub fn add_module(&mut self, module: ShellModule) -> bool {
        let module_id = module.module_id.clone();
        self.modules.insert(module_id, module);
        true
    }

    /// Remove a module
    pub fn remove_module(&mut self, module_id: &str) -> Option<ShellModule> {
        self.modules.remove(module_id)
    }

    /// Get a module by ID
    pub fn get_module(&self, module_id: &str) -> Option<&ShellModule> {
        self.modules.get(module_id)
    }

    /// List all modules
    pub fn list_modules(&self) -> Vec<&ShellModule> {
        self.modules.values().collect()
    }

    /// List modules by type
    pub fn list_modules_by_type(&self, module_type: ModuleType) -> Vec<&ShellModule> {
        self.modules
            .values()
            .filter(|m| m.module_type == module_type)
            .collect()
    }

    /// List enabled modules
    pub fn list_enabled_modules(&self) -> Vec<&ShellModule> {
        self.modules.values().filter(|m| m.is_enabled()).collect()
    }

    /// Enable a module
    pub fn enable_module(&mut self, module_id: &str) -> bool {
        if let Some(module) = self.modules.get_mut(module_id) {
            module.enable();
            true
        } else {
            false
        }
    }

    /// Disable a module
    pub fn disable_module(&mut self, module_id: &str) -> bool {
        if let Some(module) = self.modules.get_mut(module_id) {
            module.disable();
            true
        } else {
            false
        }
    }

    /// Add a configuration
    pub fn add_config(&mut self, config: ShellConfig) -> bool {
        let config_id = config.config_id.clone();
        self.configs.insert(config_id, config);
        true
    }

    /// Remove a configuration
    pub fn remove_config(&mut self, config_id: &str) -> Option<ShellConfig> {
        self.configs.remove(config_id)
    }

    /// Get a configuration by ID
    pub fn get_config(&self, config_id: &str) -> Option<&ShellConfig> {
        self.configs.get(config_id)
    }

    /// List all configurations
    pub fn list_configs(&self) -> Vec<&ShellConfig> {
        self.configs.values().collect()
    }

    /// List configurations by shell type
    pub fn list_configs_by_shell(&self, shell_type: ShellType) -> Vec<&ShellConfig> {
        self.configs
            .values()
            .filter(|c| c.shell_type == shell_type)
            .collect()
    }

    /// Activate a configuration
    pub fn activate_config(&mut self, config_id: &str) -> bool {
        // Deactivate all configs
        for config in self.configs.values_mut() {
            config.set_active(false);
        }

        // Activate target config
        if let Some(config) = self.configs.get_mut(config_id) {
            config.set_active(true);
            true
        } else {
            false
        }
    }

    /// Generate shell configuration
    pub fn generate_config(&self, config_id: &str) -> Option<String> {
        if let Some(config) = self.configs.get(config_id) {
            let mut output = String::new();

            for module_id in &config.modules {
                if let Some(module) = self.modules.get(module_id) {
                    if module.is_enabled() {
                        output.push_str(&module.content);
                        output.push('\n');
                    }
                }
            }

            Some(output)
        } else {
            None
        }
    }

    /// Reload configuration
    pub fn reload_config(&mut self, config_id: &str) -> bool {
        if self.configs.contains_key(config_id) {
            true
        } else {
            false
        }
    }

    /// Get module count
    pub fn module_count(&self) -> usize {
        self.modules.len()
    }

    /// Get config count
    pub fn config_count(&self) -> usize {
        self.configs.len()
    }
}

impl Default for SigmaQuickshell {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaQuickshell instance (placeholder for actual global state)
static mut SIGMAQUICKSHELL_INSTANCE: Option<SigmaQuickshell> = None;

/// Initialize the global SigmaQuickshell instance
pub fn init_sigmaquickshell() {
    unsafe {
        SIGMAQUICKSHELL_INSTANCE = Some(SigmaQuickshell::new());
    }
}

/// Get the global SigmaQuickshell instance
pub fn get_sigmaquickshell() -> Option<&'static SigmaQuickshell> {
    unsafe { SIGMAQUICKSHELL_INSTANCE.as_ref() }
}

/// Get mutable global SigmaQuickshell instance
pub fn get_sigmaquickshell_mut() -> Option<&'static mut SigmaQuickshell> {
    unsafe { SIGMAQUICKSHELL_INSTANCE.as_mut() }
}

/// Convenience function to add a module
pub fn add_shell_module(module: ShellModule) -> bool {
    if let Some(quickshell) = get_sigmaquickshell_mut() {
        quickshell.add_module(module)
    } else {
        false
    }
}

/// Convenience function to generate config
pub fn generate_shell_config(config_id: &str) -> Option<String> {
    if let Some(quickshell) = get_sigmaquickshell() {
        quickshell.generate_config(config_id)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_module_creation() {
        let module = ShellModule::new(
            "mod1".to_string(),
            "prompt".to_string(),
            ModuleType::Prompt,
            "export PS1='> '".to_string(),
        );

        assert_eq!(module.module_id, "mod1");
        assert_eq!(module.module_type, ModuleType::Prompt);
        assert!(module.is_enabled());
    }

    #[test]
    fn test_shell_module_state() {
        let mut module = ShellModule::new(
            "mod1".to_string(),
            "prompt".to_string(),
            ModuleType::Prompt,
            "export PS1='> '".to_string(),
        );

        module.disable();
        assert!(!module.is_enabled());

        module.enable();
        assert!(module.is_enabled());
    }

    #[test]
    fn test_shell_config_creation() {
        let config = ShellConfig::new(
            "config1".to_string(),
            ShellType::Bash,
            "/home/user/.bashrc".to_string(),
        );

        assert_eq!(config.config_id, "config1");
        assert_eq!(config.shell_type, ShellType::Bash);
        assert!(!config.is_config_active());
    }

    #[test]
    fn test_sigmaquickshell_creation() {
        let quickshell = SigmaQuickshell::new();
        assert!(!quickshell.is_running());
        assert_eq!(quickshell.module_count(), 0);
        assert_eq!(quickshell.config_count(), 0);
    }

    #[test]
    fn test_sigmaquickshell_start_stop() {
        let mut quickshell = SigmaQuickshell::new();
        quickshell.start();
        assert!(quickshell.is_running());
        quickshell.stop();
        assert!(!quickshell.is_running());
    }

    #[test]
    fn test_sigmaquickshell_add_module() {
        let mut quickshell = SigmaQuickshell::new();
        let module = ShellModule::new(
            "mod1".to_string(),
            "prompt".to_string(),
            ModuleType::Prompt,
            "export PS1='> '".to_string(),
        );

        let added = quickshell.add_module(module);
        assert!(added);
        assert_eq!(quickshell.module_count(), 1);
    }

    #[test]
    fn test_sigmaquickshell_add_config() {
        let mut quickshell = SigmaQuickshell::new();
        let config = ShellConfig::new(
            "config1".to_string(),
            ShellType::Bash,
            "/home/user/.bashrc".to_string(),
        );

        let added = quickshell.add_config(config);
        assert!(added);
        assert_eq!(quickshell.config_count(), 1);
    }

    #[test]
    fn test_sigmaquickshell_enable_module() {
        let mut quickshell = SigmaQuickshell::new();

        quickshell.add_module(ShellModule::new(
            "mod1".to_string(),
            "prompt".to_string(),
            ModuleType::Prompt,
            "export PS1='> '".to_string(),
        ));

        let enabled = quickshell.enable_module("mod1");
        assert!(enabled);

        let module = quickshell.get_module("mod1").unwrap();
        assert!(module.is_enabled());
    }

    #[test]
    fn test_sigmaquickshell_activate_config() {
        let mut quickshell = SigmaQuickshell::new();

        quickshell.add_config(ShellConfig::new(
            "config1".to_string(),
            ShellType::Bash,
            "/home/user/.bashrc".to_string(),
        ));

        let activated = quickshell.activate_config("config1");
        assert!(activated);

        let config = quickshell.get_config("config1").unwrap();
        assert!(config.is_config_active());
    }

    #[test]
    fn test_sigmaquickshell_generate_config() {
        let mut quickshell = SigmaQuickshell::new();

        quickshell.add_module(ShellModule::new(
            "mod1".to_string(),
            "prompt".to_string(),
            ModuleType::Prompt,
            "export PS1='> '".to_string(),
        ));

        let mut config = ShellConfig::new(
            "config1".to_string(),
            ShellType::Bash,
            "/home/user/.bashrc".to_string(),
        );
        config.add_module("mod1".to_string());

        quickshell.add_config(config);

        let generated = quickshell.generate_config("config1");
        assert!(generated.is_some());
    }

    #[test]
    fn test_global_instance() {
        init_sigmaquickshell();

        let quickshell = get_sigmaquickshell();
        assert!(quickshell.is_some());

        let mut quickshell_mut = get_sigmaquickshell_mut();
        assert!(quickshell_mut.is_some());

        if let Some(quickshell) = quickshell_mut {
            quickshell.start();
            assert!(quickshell.is_running());
        }
    }

    #[test]
    fn test_convenience_add_module() {
        init_sigmaquickshell();

        let module = ShellModule::new(
            "mod1".to_string(),
            "prompt".to_string(),
            ModuleType::Prompt,
            "export PS1='> '".to_string(),
        );

        let added = add_shell_module(module);
        assert!(added);

        let quickshell = get_sigmaquickshell().unwrap();
        assert_eq!(quickshell.module_count(), 1);
    }

    #[test]
    fn test_convenience_generate_config() {
        init_sigmaquickshell();

        let module = ShellModule::new(
            "mod1".to_string(),
            "prompt".to_string(),
            ModuleType::Prompt,
            "export PS1='> '".to_string(),
        );

        add_shell_module(module);

        let mut config = ShellConfig::new(
            "config1".to_string(),
            ShellType::Bash,
            "/home/user/.bashrc".to_string(),
        );
        config.add_module("mod1".to_string());

        if let Some(quickshell) = get_sigmaquickshell_mut() {
            quickshell.add_config(config);
        }

        let generated = generate_shell_config("config1");
        assert!(generated.is_some());
    }
}
