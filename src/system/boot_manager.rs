//! Startup Manager
//!
//! System startup and boot management inspired by Linux Mint's startup applications
//! and Omarchy's boot utilities, managing boot services and startup programs.

use std::collections::HashMap;

/// Service startup type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartupType {
    Auto,
    Manual,
    Disabled,
}

impl StartupType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "auto" => Some(StartupType::Auto),
            "manual" => Some(StartupType::Manual),
            "disabled" => Some(StartupType::Disabled),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            StartupType::Auto => "Auto",
            StartupType::Manual => "Manual",
            StartupType::Disabled => "Disabled",
        }
    }
}

/// Startup entry
#[derive(Debug, Clone)]
pub struct StartupEntry {
    pub name: String,
    pub command: String,
    pub description: String,
    pub startup_type: StartupType,
    pub delay_seconds: u32,
    pub is_enabled: bool,
}

impl StartupEntry {
    pub fn new(name: String, command: String, description: String) -> Self {
        Self {
            name,
            command,
            description,
            startup_type: StartupType::Auto,
            delay_seconds: 0,
            is_enabled: true,
        }
    }

    pub fn set_startup_type(&mut self, startup_type: StartupType) {
        self.startup_type = startup_type;
    }

    pub fn set_delay(&mut self, delay: u32) {
        self.delay_seconds = delay;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.is_enabled = enabled;
    }

    pub fn should_start(&self) -> bool {
        self.is_enabled && self.startup_type == StartupType::Auto
    }
}

/// Startup manager
#[derive(Debug)]
pub struct BootManager {
    entries: HashMap<String, StartupEntry>,
    boot_time: u64,
}

impl BootManager {
    pub fn new() -> Self {
        let mut manager = Self {
            entries: HashMap::new(),
            boot_time: 0,
        };

        manager.add_default_entries();
        manager
    }

    /// Add default startup entries
    fn add_default_entries(&mut self) {
        let default_entries = vec![
            ("Network Manager", "/usr/bin/sigma-network-manager", "Network connection management"),
            ("Power Manager", "/usr/bin/sigma-power-manager", "Power management daemon"),
            ("Display Manager", "/usr/bin/sigma-display-manager", "Display and login manager"),
            ("Bluetooth", "/usr/bin/sigma-bluetooth", "Bluetooth daemon"),
            ("Audio", "/usr/bin/sigma-audio", "Audio system"),
        ];

        for (name, command, description) in default_entries {
            let entry = StartupEntry::new(
                name.to_string(),
                command.to_string(),
                description.to_string(),
            );
            self.entries.insert(name.to_string(), entry);
        }
    }

    /// Set boot time
    pub fn set_boot_time(&mut self, boot_time: u64) {
        self.boot_time = boot_time;
    }

    /// Get boot time
    pub fn get_boot_time(&self) -> u64 {
        self.boot_time
    }

    /// Add a startup entry
    pub fn add_entry(&mut self, entry: StartupEntry) {
        self.entries.insert(entry.name.clone(), entry);
    }

    /// Get a startup entry
    pub fn get_entry(&self, name: &str) -> Option<&StartupEntry> {
        self.entries.get(name)
    }

    /// Get a startup entry mutably
    pub fn get_entry_mut(&mut self, name: &str) -> Option<&mut StartupEntry> {
        self.entries.get_mut(name)
    }

    /// List all startup entries
    pub fn list_entries(&self) -> Vec<&StartupEntry> {
        self.entries.values().collect()
    }

    /// List entries by startup type
    pub fn list_by_type(&self, startup_type: StartupType) -> Vec<&StartupEntry> {
        self.entries.values()
            .filter(|e| e.startup_type == startup_type)
            .collect()
    }

    /// List enabled entries
    pub fn list_enabled(&self) -> Vec<&StartupEntry> {
        self.entries.values()
            .filter(|e| e.is_enabled)
            .collect()
    }

    /// List entries that should start
    pub fn list_should_start(&self) -> Vec<&StartupEntry> {
        self.entries.values()
            .filter(|e| e.should_start())
            .collect()
    }

    /// Enable a startup entry
    pub fn enable_entry(&mut self, name: &str) -> Result<(), String> {
        let entry = self.entries.get_mut(name)
            .ok_or_else(|| format!("Startup entry {} not found", name))?;

        entry.set_enabled(true);
        Ok(())
    }

    /// Disable a startup entry
    pub fn disable_entry(&mut self, name: &str) -> Result<(), String> {
        let entry = self.entries.get_mut(name)
            .ok_or_else(|| format!("Startup entry {} not found", name))?;

        entry.set_enabled(false);
        Ok(())
    }

    /// Set startup type
    pub fn set_startup_type(&mut self, name: &str, startup_type: StartupType) -> Result<(), String> {
        let entry = self.entries.get_mut(name)
            .ok_or_else(|| format!("Startup entry {} not found", name))?;

        entry.set_startup_type(startup_type);
        Ok(())
    }

    /// Remove a startup entry
    pub fn remove_entry(&mut self, name: &str) -> Result<(), String> {
        self.entries.remove(name)
            .ok_or_else(|| format!("Startup entry {} not found", name))?;
        Ok(())
    }

    /// Simulate startup sequence
    pub fn simulate_startup(&self) -> Vec<String> {
        let mut started = Vec::new();
        let mut entries: Vec<_> = self.list_should_start();
        entries.sort_by_key(|e| e.delay_seconds);

        for entry in entries {
            started.push(format!("Starting: {} ({})", entry.name, entry.command));
        }

        started
    }

    /// Get statistics
    pub fn get_statistics(&self) -> StartupStatistics {
        let total_entries = self.entries.len();
        let enabled_entries = self.entries.values().filter(|e| e.is_enabled).count();
        let auto_start = self.entries.values().filter(|e| e.startup_type == StartupType::Auto).count();
        let manual_start = self.entries.values().filter(|e| e.startup_type == StartupType::Manual).count();
        let disabled = self.entries.values().filter(|e| e.startup_type == StartupType::Disabled).count();

        StartupStatistics {
            total_entries,
            enabled_entries,
            auto_start,
            manual_start,
            disabled,
            boot_time: self.boot_time,
        }
    }
}

impl Default for BootManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Startup statistics
#[derive(Debug, Clone)]
pub struct StartupStatistics {
    pub total_entries: usize,
    pub enabled_entries: usize,
    pub auto_start: usize,
    pub manual_start: usize,
    pub disabled: usize,
    pub boot_time: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_startup_type_from_str() {
        assert_eq!(StartupType::from_str("auto"), Some(StartupType::Auto));
        assert_eq!(StartupType::from_str("manual"), Some(StartupType::Manual));
    }

    #[test]
    fn test_startup_entry_creation() {
        let entry = StartupEntry::new(
            "Test".to_string(),
            "/usr/bin/test".to_string(),
            "Test entry".to_string(),
        );
        assert_eq!(entry.name, "Test");
    }

    #[test]
    fn test_startup_manager_creation() {
        let manager = BootManager::new();
        assert!(manager.list_entries().len() >= 5);
    }

    #[test]
    fn test_add_entry() {
        let mut manager = BootManager::new();
        let entry = StartupEntry::new(
            "Custom".to_string(),
            "/usr/bin/custom".to_string(),
            "Custom entry".to_string(),
        );
        manager.add_entry(entry);
        assert!(manager.get_entry("Custom").is_some());
    }

    #[test]
    fn test_enable_entry() {
        let mut manager = BootManager::new();
        manager.disable_entry("Network Manager").ok();
        assert!(manager.enable_entry("Network Manager").is_ok());
    }

    #[test]
    fn test_disable_entry() {
        let mut manager = BootManager::new();
        assert!(manager.disable_entry("Network Manager").is_ok());
    }

    #[test]
    fn test_set_startup_type() {
        let mut manager = BootManager::new();
        assert!(manager.set_startup_type("Network Manager", StartupType::Manual).is_ok());
    }

    #[test]
    fn test_simulate_startup() {
        let manager = BootManager::new();
        let sequence = manager.simulate_startup();
        assert!(sequence.len() > 0);
    }

    #[test]
    fn test_statistics() {
        let manager = BootManager::new();
        let stats = manager.get_statistics();
        assert!(stats.total_entries >= 5);
    }
}
