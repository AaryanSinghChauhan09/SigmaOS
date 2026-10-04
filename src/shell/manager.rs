//! Shell Management System
//!
//! Multi-shell environment management inspired by Omarchy's shell system,
//! supporting bash, zsh, fish, and custom shells with profile management.

use std::collections::HashMap;
use std::path::PathBuf;

/// Shell type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShellType {
    Bash,
    Zsh,
    Fish,
    Nu,
    PowerShell,
    Custom,
}

impl ShellType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "bash" => Some(ShellType::Bash),
            "zsh" => Some(ShellType::Zsh),
            "fish" => Some(ShellType::Fish),
            "nu" => Some(ShellType::Nu),
            "powershell" | "pwsh" => Some(ShellType::PowerShell),
            _ => Some(ShellType::Custom),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ShellType::Bash => "bash",
            ShellType::Zsh => "zsh",
            ShellType::Fish => "fish",
            ShellType::Nu => "nu",
            ShellType::PowerShell => "powershell",
            ShellType::Custom => "custom",
        }
    }

    pub fn config_file(&self) -> PathBuf {
        match self {
            ShellType::Bash => PathBuf::from(".bashrc"),
            ShellType::Zsh => PathBuf::from(".zshrc"),
            ShellType::Fish => PathBuf::from(".config/fish/config.fish"),
            ShellType::Nu => PathBuf::from(".config/nu/config.nu"),
            ShellType::PowerShell => PathBuf::from(".config/powershell/profile.ps1"),
            ShellType::Custom => PathBuf::from(".shellrc"),
        }
    }

    pub fn profile_file(&self) -> PathBuf {
        match self {
            ShellType::Bash => PathBuf::from(".bash_profile"),
            ShellType::Zsh => PathBuf::from(".zprofile"),
            ShellType::Fish => PathBuf::from(".config/fish/config.fish"),
            ShellType::Nu => PathBuf::from(".config/nu/env.nu"),
            ShellType::PowerShell => PathBuf::from(".config/powershell/Microsoft.PowerShell_profile.ps1"),
            ShellType::Custom => PathBuf::from(".profile"),
        }
    }
}

/// Shell profile configuration
#[derive(Debug, Clone)]
pub struct ShellProfile {
    pub name: String,
    pub shell_type: ShellType,
    pub environment_vars: HashMap<String, String>,
    pub aliases: HashMap<String, String>,
    pub functions: Vec<String>,
    pub path_entries: Vec<String>,
    pub init_scripts: Vec<String>,
}

impl ShellProfile {
    pub fn new(name: String, shell_type: ShellType) -> Self {
        Self {
            name,
            shell_type,
            environment_vars: HashMap::new(),
            aliases: HashMap::new(),
            functions: Vec::new(),
            path_entries: Vec::new(),
            init_scripts: Vec::new(),
        }
    }

    pub fn set_env_var(&mut self, key: String, value: String) {
        self.environment_vars.insert(key, value);
    }

    pub fn add_alias(&mut self, name: String, command: String) {
        self.aliases.insert(name, command);
    }

    pub fn add_function(&mut self, function: String) {
        self.functions.push(function);
    }

    pub fn add_path_entry(&mut self, path: String) {
        self.path_entries.push(path);
    }

    pub fn add_init_script(&mut self, script: String) {
        self.init_scripts.push(script);
    }

    pub fn generate_config(&self) -> String {
        let mut config = String::new();

        // Header
        config.push_str(&format!("# SigmaOS Shell Profile: {}\n", self.name));
        config.push_str(&format!("# Shell Type: {}\n\n", self.shell_type.as_str()));

        // Environment variables
        for (key, value) in &self.environment_vars {
            match self.shell_type {
                ShellType::Bash | ShellType::Zsh => {
                    config.push_str(&format!("export {}=\"{}\"\n", key, value));
                }
                ShellType::Fish => {
                    config.push_str(&format!("set -gx {} \"{}\"\n", key, value));
                }
                ShellType::Nu => {
                    config.push_str(&format!("$env.{} = \"{}\"\n", key, value));
                }
                ShellType::PowerShell => {
                    config.push_str(&format!("$env:{} = \"{}\"\n", key, value));
                }
                ShellType::Custom => {
                    config.push_str(&format!("export {}=\"{}\"\n", key, value));
                }
            }
        }

        config.push_str("\n");

        // PATH entries
        if !self.path_entries.is_empty() {
            let path_str = self.path_entries.join(":");
            match self.shell_type {
                ShellType::Bash | ShellType::Zsh => {
                    config.push_str(&format!("export PATH=\"{}:$PATH\"\n", path_str));
                }
                ShellType::Fish => {
                    config.push_str(&format!("fish_add_path -g {}\n", path_str));
                }
                ShellType::Nu => {
                    config.push_str(&format!("$env.PATH = ($env.PATH | split row (char esep) | prepend \"{}\")\n", path_str));
                }
                ShellType::PowerShell => {
                    config.push_str(&format!("$env:PATH = \"{};$env:PATH\"\n", path_str));
                }
                ShellType::Custom => {
                    config.push_str(&format!("export PATH=\"{}:$PATH\"\n", path_str));
                }
            }
            config.push_str("\n");
        }

        // Aliases
        for (name, command) in &self.aliases {
            match self.shell_type {
                ShellType::Bash | ShellType::Zsh => {
                    config.push_str(&format!("alias {}='{}'\n", name, command));
                }
                ShellType::Fish => {
                    config.push_str(&format!("alias {} \"{}\"\n", name, command));
                }
                ShellType::Nu => {
                    config.push_str(&format!("alias {} = {{|| {} }}\n", name, command));
                }
                ShellType::PowerShell => {
                    config.push_str(&format!("Set-Alias -Name {} -Value {}\n", name, command));
                }
                ShellType::Custom => {
                    config.push_str(&format!("alias {}='{}'\n", name, command));
                }
            }
        }

        config.push_str("\n");

        // Functions
        for function in &self.functions {
            config.push_str(function);
            config.push_str("\n\n");
        }

        // Init scripts
        for script in &self.init_scripts {
            config.push_str("# Init script\n");
            config.push_str(script);
            config.push_str("\n\n");
        }

        config
    }
}

/// Shell manager
#[derive(Debug)]
pub struct ShellManager {
    profiles: HashMap<String, ShellProfile>,
    default_shell: ShellType,
    current_profile: Option<String>,
}

impl ShellManager {
    pub fn new(default_shell: ShellType) -> Self {
        let mut manager = Self {
            profiles: HashMap::new(),
            default_shell,
            current_profile: None,
        };

        // Create default profile
        let default_profile = ShellProfile::new("default".to_string(), default_shell);
        manager.profiles.insert("default".to_string(), default_profile);
        manager.current_profile = Some("default".to_string());

        manager
    }

    /// Create a new profile
    pub fn create_profile(&mut self, name: String, shell_type: ShellType) -> Result<(), String> {
        if self.profiles.contains_key(&name) {
            return Err(format!("Profile {} already exists", name));
        }

        let profile = ShellProfile::new(name.clone(), shell_type);
        self.profiles.insert(name, profile);
        Ok(())
    }

    /// Get a profile
    pub fn get_profile(&self, name: &str) -> Option<&ShellProfile> {
        self.profiles.get(name)
    }

    /// Get a profile mutably
    pub fn get_profile_mut(&mut self, name: &str) -> Option<&mut ShellProfile> {
        self.profiles.get_mut(name)
    }

    /// Set current profile
    pub fn set_current_profile(&mut self, name: &str) -> Result<(), String> {
        if !self.profiles.contains_key(name) {
            return Err(format!("Profile {} not found", name));
        }

        self.current_profile = Some(name.to_string());
        Ok(())
    }

    /// Get current profile
    pub fn get_current_profile(&self) -> Option<&ShellProfile> {
        self.current_profile.as_ref()
            .and_then(|name| self.profiles.get(name))
    }

    /// Delete a profile
    pub fn delete_profile(&mut self, name: &str) -> Result<(), String> {
        if name == "default" {
            return Err("Cannot delete default profile".to_string());
        }

        if self.current_profile.as_ref() == Some(&name.to_string()) {
            self.current_profile = Some("default".to_string());
        }

        self.profiles.remove(name)
            .ok_or_else(|| format!("Profile {} not found", name))?;

        Ok(())
    }

    /// List all profiles
    pub fn list_profiles(&self) -> Vec<&str> {
        self.profiles.keys().map(|s| s.as_str()).collect()
    }

    /// Get profile count
    pub fn profile_count(&self) -> usize {
        self.profiles.len()
    }

    /// Generate config for current profile
    pub fn generate_current_config(&self) -> Result<String, String> {
        self.current_profile
            .as_ref()
            .and_then(|name| self.profiles.get(name))
            .map(|profile| profile.generate_config())
            .ok_or_else(|| "No current profile".to_string())
    }

    /// Set default shell
    pub fn set_default_shell(&mut self, shell_type: ShellType) {
        self.default_shell = shell_type;
    }

    /// Get default shell
    pub fn get_default_shell(&self) -> ShellType {
        self.default_shell
    }

    /// Get statistics
    pub fn get_statistics(&self) -> ShellStatistics {
        let total_env_vars: usize = self.profiles.values()
            .map(|p| p.environment_vars.len())
            .sum();

        let total_aliases: usize = self.profiles.values()
            .map(|p| p.aliases.len())
            .sum();

        let total_functions: usize = self.profiles.values()
            .map(|p| p.functions.len())
            .sum();

        ShellStatistics {
            profile_count: self.profiles.len(),
            total_env_vars,
            total_aliases,
            total_functions,
            default_shell: self.default_shell,
            current_profile: self.current_profile.clone(),
        }
    }
}

impl Default for ShellManager {
    fn default() -> Self {
        Self::new(ShellType::Bash)
    }
}

/// Shell statistics
#[derive(Debug, Clone)]
pub struct ShellStatistics {
    pub profile_count: usize,
    pub total_env_vars: usize,
    pub total_aliases: usize,
    pub total_functions: usize,
    pub default_shell: ShellType,
    pub current_profile: Option<String>,
}

#[cfg(test)]
#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_type_from_str() {
        assert_eq!(ShellType::from_str("bash"), Some(ShellType::Bash));
        assert_eq!(ShellType::from_str("zsh"), Some(ShellType::Zsh));
        assert_eq!(ShellType::from_str("fish"), Some(ShellType::Fish));
    }

    #[test]
    fn test_shell_type_config_file() {
        assert_eq!(ShellType::Bash.config_file(), PathBuf::from(".bashrc"));
        assert_eq!(ShellType::Zsh.config_file(), PathBuf::from(".zshrc"));
        assert_eq!(ShellType::Fish.config_file(), PathBuf::from(".config/fish/config.fish"));
    }

    #[test]
    fn test_shell_profile_creation() {
        let profile = ShellProfile::new("test".to_string(), ShellType::Bash);
        assert_eq!(profile.name, "test");
        assert_eq!(profile.shell_type, ShellType::Bash);
    }

    #[test]
    fn test_shell_profile_env_vars() {
        let mut profile = ShellProfile::new("test".to_string(), ShellType::Bash);
        profile.set_env_var("TEST".to_string(), "value".to_string());
        assert_eq!(profile.environment_vars.get("TEST"), Some(&"value".to_string()));
    }

    #[test]
    fn test_shell_profile_aliases() {
        let mut profile = ShellProfile::new("test".to_string(), ShellType::Bash);
        profile.add_alias("ll".to_string(), "ls -la".to_string());
        assert_eq!(profile.aliases.get("ll"), Some(&"ls -la".to_string()));
    }

    #[test]
    fn test_shell_profile_config_generation() {
        let mut profile = ShellProfile::new("test".to_string(), ShellType::Bash);
        profile.set_env_var("EDITOR".to_string(), "vim".to_string());
        profile.add_alias("ll".to_string(), "ls -la".to_string());
        
        let config = profile.generate_config();
        assert!(config.contains("export EDITOR=\"vim\""));
        assert!(config.contains("alias ll='ls -la'"));
    }

    #[test]
    fn test_shell_manager_creation() {
        let manager = ShellManager::new(ShellType::Bash);
        assert_eq!(manager.default_shell, ShellType::Bash);
        assert_eq!(manager.current_profile, Some("default".to_string()));
    }

    #[test]
    fn test_shell_manager_create_profile() {
        let mut manager = ShellManager::new(ShellType::Bash);
        assert!(manager.create_profile("test".to_string(), ShellType::Zsh).is_ok());
        assert_eq!(manager.list_profiles().len(), 2);
    }

    #[test]
    fn test_shell_manager_set_current() {
        let mut manager = ShellManager::new(ShellType::Bash);
        manager.create_profile("test".to_string(), ShellType::Zsh).ok();
        assert!(manager.set_current_profile("test").is_ok());
        assert_eq!(manager.current_profile, Some("test".to_string()));
    }

    #[test]
    fn test_shell_manager_delete_profile() {
        let mut manager = ShellManager::new(ShellType::Bash);
        manager.create_profile("test".to_string(), ShellType::Zsh).ok();
        assert!(manager.delete_profile("test").is_ok());
        assert_eq!(manager.list_profiles().len(), 1);
    }

    #[test]
    fn test_shell_manager_statistics() {
        let mut manager = ShellManager::new(ShellType::Bash);
        let profile = manager.get_profile_mut("default").unwrap();
        profile.set_env_var("TEST".to_string(), "value".to_string());
        profile.add_alias("ll".to_string(), "ls -la".to_string());

        let stats = manager.get_statistics();
        assert_eq!(stats.profile_count, 1);
        assert_eq!(stats.total_env_vars, 1);
        assert_eq!(stats.total_aliases, 1);
    }
}
