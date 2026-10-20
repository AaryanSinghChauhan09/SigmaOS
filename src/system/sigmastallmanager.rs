// SPDX-License-Identifier: MIT
// SigmaOS - Install Manager
// Installation management inspired by Omarchy install-* commands and Linux package managers

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Install type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallType {
    System,
    User,
    Optional,
}

/// Install status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallStatus {
    NotInstalled,
    Installing,
    Installed,
    Failed,
}

/// Software item
#[derive(Debug, Clone)]
pub struct SoftwareItem {
    pub id: String,
    pub name: String,
    pub install_type: InstallType,
    pub status: InstallStatus,
    pub version: String,
}

/// Install manager
pub struct SigmaInstallManager {
    items: Vec<SoftwareItem>,
}

impl SigmaInstallManager {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: SoftwareItem) {
        self.items.push(item);
    }

    pub fn get_items(&self) -> &[SoftwareItem] {
        &self.items
    }

    pub fn get_item(&self, id: &str) -> Option<&SoftwareItem> {
        self.items.iter().find(|i| i.id == id)
    }

    pub fn install_item(&mut self, id: String) -> Result<(), String> {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.status = InstallStatus::Installing;
            item.status = InstallStatus::Installed;
            Ok(())
        } else {
            Err(format!("Item {} not found", id))
        }
    }

    pub fn remove_item(&mut self, id: String) -> Result<(), String> {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.status = InstallStatus::NotInstalled;
            Ok(())
        } else {
            Err(format!("Item {} not found", id))
        }
    }

    pub fn get_installed_items(&self) -> Vec<&SoftwareItem> {
        self.items
            .iter()
            .filter(|i| i.status == InstallStatus::Installed)
            .collect()
    }

    pub fn get_items_by_type(&self, install_type: InstallType) -> Vec<&SoftwareItem> {
        self.items
            .iter()
            .filter(|i| i.install_type == install_type)
            .collect()
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
    fn test_install_manager_creation() {
        let manager = SigmaInstallManager::new();
        assert_eq!(manager.get_items().len(), 0);
    }

    #[test]
    fn test_add_item() {
        let mut manager = SigmaInstallManager::new();
        let item = SoftwareItem {
            id: "firefox".to_string(),
            name: "Firefox".to_string(),
            install_type: InstallType::Optional,
            status: InstallStatus::NotInstalled,
            version: "1.0".to_string(),
        };
        manager.add_item(item);
        assert_eq!(manager.get_items().len(), 1);
    }

    #[test]
    fn test_install_item() {
        let mut manager = SigmaInstallManager::new();
        let item = SoftwareItem {
            id: "firefox".to_string(),
            name: "Firefox".to_string(),
            install_type: InstallType::Optional,
            status: InstallStatus::NotInstalled,
            version: "1.0".to_string(),
        };
        manager.add_item(item);
        manager.install_item("firefox".to_string()).unwrap();
        assert_eq!(manager.get_items()[0].status, InstallStatus::Installed);
    }

    #[test]
    fn test_remove_item() {
        let mut manager = SigmaInstallManager::new();
        let item = SoftwareItem {
            id: "firefox".to_string(),
            name: "Firefox".to_string(),
            install_type: InstallType::Optional,
            status: InstallStatus::Installed,
            version: "1.0".to_string(),
        };
        manager.add_item(item);
        manager.remove_item("firefox".to_string()).unwrap();
        assert_eq!(manager.get_items()[0].status, InstallStatus::NotInstalled);
    }

    #[test]
    fn test_installed_items() {
        let mut manager = SigmaInstallManager::new();
        let item1 = SoftwareItem {
            id: "firefox".to_string(),
            name: "Firefox".to_string(),
            install_type: InstallType::Optional,
            status: InstallStatus::Installed,
            version: "1.0".to_string(),
        };
        let item2 = SoftwareItem {
            id: "chrome".to_string(),
            name: "Chrome".to_string(),
            install_type: InstallType::Optional,
            status: InstallStatus::NotInstalled,
            version: "1.0".to_string(),
        };
        manager.add_item(item1);
        manager.add_item(item2);
        assert_eq!(manager.get_installed_items().len(), 1);
    }

    #[test]
    fn test_items_by_type() {
        let mut manager = SigmaInstallManager::new();
        let item1 = SoftwareItem {
            id: "system-tool".to_string(),
            name: "System Tool".to_string(),
            install_type: InstallType::System,
            status: InstallStatus::Installed,
            version: "1.0".to_string(),
        };
        let item2 = SoftwareItem {
            id: "optional-tool".to_string(),
            name: "Optional Tool".to_string(),
            install_type: InstallType::Optional,
            status: InstallStatus::Installed,
            version: "1.0".to_string(),
        };
        manager.add_item(item1);
        manager.add_item(item2);
        assert_eq!(manager.get_items_by_type(InstallType::System).len(), 1);
    }
}
