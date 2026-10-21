// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Aaryan Singh Chauhan

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Installation status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallStatus {
    NotInstalled,
    Installing,
    Installed,
    Failed,
    Updating,
    Removing,
}

/// Installation type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallType {
    SystemPackage,
    Flatpak,
    Snap,
    AppImage,
    Custom,
}

/// Software item to install
#[derive(Debug, Clone)]
pub struct SoftwareItem {
    pub id: String,
    pub name: String,
    pub version: String,
    pub install_type: InstallType,
    pub status: InstallStatus,
    pub dependencies: Vec<String>,
    pub source: String,
}

/// Install manager for managing software installations
/// Inspired by Omarchy install- commands and various package managers
#[derive(Debug, Clone)]
pub struct SigmaInstallManager {
    items: Vec<SoftwareItem>,
}

impl SigmaInstallManager {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    /// Register a software item
    pub fn register_item(&mut self, item: SoftwareItem) {
        self.items.push(item);
    }

    /// Get item by ID
    pub fn get_item(&self, id: &str) -> Option<&SoftwareItem> {
        self.items.iter().find(|i| i.id == id)
    }

    /// Get all items
    pub fn get_items(&self) -> &[SoftwareItem] {
        &self.items
    }

    /// Install a software item
    pub fn install(&mut self, id: &str) -> Result<String, String> {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.status = InstallStatus::Installing;
            Ok(format!("Installing: {}", item.name))
        } else {
            Err(format!("Item not found: {}", id))
        }
    }

    /// Complete installation
    pub fn complete_install(&mut self, id: &str) -> Result<String, String> {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.status = InstallStatus::Installed;
            Ok(format!("Installation complete: {}", item.name))
        } else {
            Err(format!("Item not found: {}", id))
        }
    }

    /// Remove a software item
    pub fn remove(&mut self, id: &str) -> Result<String, String> {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.status = InstallStatus::Removing;
            Ok(format!("Removing: {}", item.name))
        } else {
            Err(format!("Item not found: {}", id))
        }
    }

    /// Update a software item
    pub fn update(&mut self, id: &str) -> Result<String, String> {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.status = InstallStatus::Updating;
            Ok(format!("Updating: {}", item.name))
        } else {
            Err(format!("Item not found: {}", id))
        }
    }

    /// Get installed items
    pub fn get_installed(&self) -> Vec<&SoftwareItem> {
        self.items
            .iter()
            .filter(|i| i.status == InstallStatus::Installed)
            .collect()
    }

    /// Get items by install type
    pub fn get_by_type(&self, install_type: InstallType) -> Vec<&SoftwareItem> {
        self.items
            .iter()
            .filter(|i| i.install_type == install_type)
            .collect()
    }

    /// Check if item is installed
    pub fn is_installed(&self, id: &str) -> bool {
        self.items
            .iter()
            .any(|i| i.id == id && i.status == InstallStatus::Installed)
    }

    /// Get install queue (items to install)
    pub fn get_install_queue(&self) -> Vec<&SoftwareItem> {
        self.items
            .iter()
            .filter(|i| i.status == InstallStatus::Installing)
            .collect()
    }

    /// Check dependencies
    pub fn check_dependencies(&self, id: &str) -> Result<bool, String> {
        if let Some(item) = self.get_item(id) {
            let missing: Vec<_> = item
                .dependencies
                .iter()
                .filter(|dep| !self.is_installed(dep))
                .collect();
            if missing.is_empty() {
                Ok(true)
            } else {
                Err(format!("Missing dependencies: {:?}", missing))
            }
        } else {
            Err(format!("Item not found: {}", id))
        }
    }
}

impl Default for SigmaInstallManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_item() {
        let mut manager = SigmaInstallManager::new();
        let item = SoftwareItem {
            id: "chrome".to_string(),
            name: "Chromium".to_string(),
            version: "120.0".to_string(),
            install_type: InstallType::SystemPackage,
            status: InstallStatus::NotInstalled,
            dependencies: Vec::new(),
            source: "repo".to_string(),
        };
        manager.register_item(item);
        assert_eq!(manager.get_items().len(), 1);
    }

    #[test]
    fn test_install() {
        let mut manager = SigmaInstallManager::new();
        let item = SoftwareItem {
            id: "chrome".to_string(),
            name: "Chromium".to_string(),
            version: "120.0".to_string(),
            install_type: InstallType::SystemPackage,
            status: InstallStatus::NotInstalled,
            dependencies: Vec::new(),
            source: "repo".to_string(),
        };
        manager.register_item(item);
        manager.install("chrome").unwrap();
        assert_eq!(
            manager.get_item("chrome").unwrap().status,
            InstallStatus::Installing
        );
    }

    #[test]
    fn test_complete_install() {
        let mut manager = SigmaInstallManager::new();
        let item = SoftwareItem {
            id: "chrome".to_string(),
            name: "Chromium".to_string(),
            version: "120.0".to_string(),
            install_type: InstallType::SystemPackage,
            status: InstallStatus::Installing,
            dependencies: Vec::new(),
            source: "repo".to_string(),
        };
        manager.register_item(item);
        manager.complete_install("chrome").unwrap();
        assert_eq!(
            manager.get_item("chrome").unwrap().status,
            InstallStatus::Installed
        );
    }

    #[test]
    fn test_remove() {
        let mut manager = SigmaInstallManager::new();
        let item = SoftwareItem {
            id: "chrome".to_string(),
            name: "Chromium".to_string(),
            version: "120.0".to_string(),
            install_type: InstallType::SystemPackage,
            status: InstallStatus::Installed,
            dependencies: Vec::new(),
            source: "repo".to_string(),
        };
        manager.register_item(item);
        manager.remove("chrome").unwrap();
        assert_eq!(
            manager.get_item("chrome").unwrap().status,
            InstallStatus::Removing
        );
    }

    #[test]
    fn test_get_installed() {
        let mut manager = SigmaInstallManager::new();
        let item1 = SoftwareItem {
            id: "chrome".to_string(),
            name: "Chromium".to_string(),
            version: "120.0".to_string(),
            install_type: InstallType::SystemPackage,
            status: InstallStatus::Installed,
            dependencies: Vec::new(),
            source: "repo".to_string(),
        };
        let item2 = SoftwareItem {
            id: "firefox".to_string(),
            name: "Firefox".to_string(),
            version: "121.0".to_string(),
            install_type: InstallType::Flatpak,
            status: InstallStatus::NotInstalled,
            dependencies: Vec::new(),
            source: "flathub".to_string(),
        };
        manager.register_item(item1);
        manager.register_item(item2);
        assert_eq!(manager.get_installed().len(), 1);
    }

    #[test]
    fn test_check_dependencies() {
        let mut manager = SigmaInstallManager::new();
        let dep = SoftwareItem {
            id: "libx11".to_string(),
            name: "libX11".to_string(),
            version: "1.8".to_string(),
            install_type: InstallType::SystemPackage,
            status: InstallStatus::Installed,
            dependencies: Vec::new(),
            source: "repo".to_string(),
        };
        let item = SoftwareItem {
            id: "app".to_string(),
            name: "App".to_string(),
            version: "1.0".to_string(),
            install_type: InstallType::SystemPackage,
            status: InstallStatus::NotInstalled,
            dependencies: vec!["libx11".to_string()],
            source: "repo".to_string(),
        };
        manager.register_item(dep);
        manager.register_item(item);
        assert!(manager.check_dependencies("app").is_ok());
    }
}
