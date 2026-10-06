// SPDX-License-Identifier: MIT
// SigmaOS MintUpdate-Inspired Update Manager
// Linux Mint MintUpdate-inspired automatic update management

use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;

/// Update type
#[derive(Debug, Clone, PartialEq)]
pub enum UpdateType {
    System,
    Security,
    Kernel,
    Flatpak,
    Spice,
    Custom(String),
}

/// Update state
#[derive(Debug, Clone, PartialEq)]
pub enum UpdateState {
    Available,
    Downloading,
    Downloaded,
    Installing,
    Installed,
    Failed(String),
}

/// Update information
#[derive(Debug, Clone)]
pub struct Update {
    pub id: String,
    pub name: String,
    pub version: String,
    pub update_type: UpdateType,
    pub state: UpdateState,
    pub size: u64,
    pub description: String,
    pub priority: u32,
    pub reboot_required: bool,
}

impl Update {
    pub fn new(id: String, name: String, update_type: UpdateType) -> Self {
        Self {
            id,
            name,
            version: String::new(),
            update_type,
            state: UpdateState::Available,
            size: 0,
            description: String::new(),
            priority: 0,
            reboot_required: false,
        }
    }

    /// Check if update is installed
    pub fn is_installed(&self) -> bool {
        matches!(self.state, UpdateState::Installed)
    }

    /// Check if update is ready to install
    pub fn is_ready_to_install(&self) -> bool {
        matches!(self.state, UpdateState::Downloaded | UpdateState::Available)
    }
}

/// Automation policy
#[derive(Debug, Clone, PartialEq)]
pub enum AutomationPolicy {
    None,
    DownloadOnly,
    Automatic,
}

/// MintUpdate-inspired update manager
#[derive(Debug, Clone)]
pub struct MintUpdateManager {
    pub updates: BTreeMap<String, Update>,
    pub automation_policy: AutomationPolicy,
    pub auto_refresh_interval: u32, // in minutes
    pub refresh_in_progress: bool,
    pub current_task: Option<String>,
    pub download_path: String,
    pub history: Vec<String>,
}

impl MintUpdateManager {
    pub fn new() -> Self {
        Self {
            updates: BTreeMap::new(),
            automation_policy: AutomationPolicy::None,
            auto_refresh_interval: 15, // 15 minutes default
            refresh_in_progress: false,
            current_task: None,
            download_path: String::from("/var/cache/apt/archives"),
            history: Vec::new(),
        }
    }

    /// Add update
    pub fn add_update(&mut self, update: Update) {
        let id = update.id.clone();
        self.updates.insert(id, update);
    }

    /// Check for updates
    pub fn check_updates(&mut self) -> Result<Vec<String>, &'static str> {
        self.refresh_in_progress = true;
        self.current_task = Some(String::from("Checking for updates"));

        // In real implementation, would check actual repositories
        // For now, simulate finding updates
        let mut found = Vec::new();
        
        // Simulate finding a system update
        let mut update = Update::new(
            String::from("sys-upgrade-1"),
            String::from("System Upgrade"),
            UpdateType::System
        );
        update.priority = 100;
        self.add_update(update);
        found.push(String::from("sys-upgrade-1"));

        self.refresh_in_progress = false;
        self.current_task = None;

        Ok(found)
    }

    /// Download update
    pub fn download_update(&mut self, id: String) -> Result<(), &'static str> {
        if let Some(update) = self.updates.get_mut(&id) {
            if !update.is_ready_to_install() {
                return Err("Update not ready to download");
            }

            update.state = UpdateState::Downloading;
            self.current_task = Some(format!("Downloading {}", update.name));

            // In real implementation, would download update
            update.state = UpdateState::Downloaded;
            self.current_task = None;

            Ok(())
        } else {
            Err("Update not found")
        }
    }

    /// Install update
    pub fn install_update(&mut self, id: String) -> Result<(), &'static str> {
        if let Some(update) = self.updates.get_mut(&id) {
            if !update.is_ready_to_install() {
                return Err("Update not ready to install");
            }

            update.state = UpdateState::Installing;
            self.current_task = Some(format!("Installing {}", update.name));

            // In real implementation, would install update
            update.state = UpdateState::Installed;
            self.current_task = None;

            // Add to history
            self.history.push(format!("Installed {} version {}", update.name, update.version));

            Ok(())
        } else {
            Err("Update not found")
        }
    }

    /// Download all updates
    pub fn download_all(&mut self) -> Result<Vec<String>, &'static str> {
        let mut downloaded = Vec::new();
        let ids: Vec<_> = self.updates
            .iter()
            .filter(|(_, u)| u.is_ready_to_install())
            .map(|(id, _)| id.clone())
            .collect();

        for id in ids {
            if self.download_update(id.clone()).is_ok() {
                downloaded.push(id);
            }
        }

        Ok(downloaded)
    }

    /// Install all updates
    pub fn install_all(&mut self) -> Result<Vec<String>, &'static str> {
        let mut installed = Vec::new();
        let ids: Vec<_> = self.updates
            .iter()
            .filter(|(_, u)| u.is_ready_to_install())
            .map(|(id, _)| id.clone())
            .collect();

        for id in ids {
            if self.install_update(id.clone()).is_ok() {
                installed.push(id);
            }
        }

        Ok(installed)
    }

    /// Run automation based on policy
    pub fn run_automation(&mut self) -> Result<Vec<String>, &'static str> {
        match self.automation_policy {
            AutomationPolicy::None => Ok(Vec::new()),
            AutomationPolicy::DownloadOnly => self.download_all(),
            AutomationPolicy::Automatic => {
                self.download_all()?;
                self.install_all()
            }
        }
    }

    /// Set automation policy
    pub fn set_automation_policy(&mut self, policy: AutomationPolicy) {
        self.automation_policy = policy;
    }

    /// Set auto refresh interval
    pub fn set_auto_refresh_interval(&mut self, interval: u32) {
        self.auto_refresh_interval = interval;
    }

    /// Get updates by type
    pub fn get_by_type(&self, update_type: &UpdateType) -> Vec<&Update> {
        self.updates
            .values()
            .filter(|u| &u.update_type == update_type)
            .collect()
    }

    /// Get security updates
    pub fn get_security_updates(&self) -> Vec<&Update> {
        self.get_by_type(&UpdateType::Security)
    }

    /// Get kernel updates
    pub fn get_kernel_updates(&self) -> Vec<&Update> {
        self.get_by_type(&UpdateType::Kernel)
    }

    /// Get all available updates
    pub fn get_available_updates(&self) -> Vec<&Update> {
        self.updates
            .values()
            .filter(|u| u.is_ready_to_install())
            .collect()
    }

    /// Get update by ID
    pub fn get_update(&self, id: &str) -> Option<&Update> {
        self.updates.get(id)
    }

    /// Get current task
    pub fn get_current_task(&self) -> Option<&String> {
        self.current_task.as_ref()
    }

    /// Get update history
    pub fn get_history(&self) -> &[String] {
        &self.history
    }

    /// Check if reboot is required
    pub fn is_reboot_required(&self) -> bool {
        self.updates.values().any(|u| u.reboot_required && u.is_installed())
    }
}

impl Default for MintUpdateManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_creation() {
        let update = Update::new(
            String::from("test-update"),
            String::from("Test Update"),
            UpdateType::System
        );
        
        assert_eq!(update.name, "Test Update");
        assert!(!update.is_installed());
    }

    #[test]
    fn test_automation_policy() {
        let mut manager = MintUpdateManager::new();
        
        assert_eq!(manager.automation_policy, AutomationPolicy::None);
        manager.set_automation_policy(AutomationPolicy::Automatic);
        assert_eq!(manager.automation_policy, AutomationPolicy::Automatic);
    }

    #[test]
    fn test_add_update() {
        let mut manager = MintUpdateManager::new();
        let update = Update::new(
            String::from("sys-upgrade"),
            String::from("System Upgrade"),
            UpdateType::System
        );
        
        manager.add_update(update);
        assert_eq!(manager.updates.len(), 1);
    }

    #[test]
    fn test_check_updates() {
        let mut manager = MintUpdateManager::new();
        
        let found = manager.check_updates().unwrap();
        assert!(!found.is_empty());
    }

    #[test]
    fn test_download_install() {
        let mut manager = MintUpdateManager::new();
        let update = Update::new(
            String::from("test-pkg"),
            String::from("Test Package"),
            UpdateType::System
        );
        manager.add_update(update);
        
        assert!(manager.download_update(String::from("test-pkg")).is_ok());
        assert!(manager.install_update(String::from("test-pkg")).is_ok());
    }

    #[test]
    fn test_get_by_type() {
        let mut manager = MintUpdateManager::new();
        let update = Update::new(
            String::from("sec-update"),
            String::from("Security Update"),
            UpdateType::Security
        );
        manager.add_update(update);
        
        let security = manager.get_security_updates();
        assert_eq!(security.len(), 1);
    }
}