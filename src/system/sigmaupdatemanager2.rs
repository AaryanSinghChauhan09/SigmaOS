// SPDX-License-Identifier: MIT
// SigmaOS - Update Manager 2
// Update management inspired by Omarchy update-* commands and Linux update systems

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Update type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateType {
    System,
    Package,
    Security,
    Feature,
}

/// Update status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateStatus {
    Available,
    Downloading,
    Installing,
    Installed,
    Failed,
}

/// Update
#[derive(Debug, Clone)]
pub struct Update {
    pub id: String,
    pub name: String,
    pub update_type: UpdateType,
    pub status: UpdateStatus,
    pub version: String,
    pub size_bytes: u64,
}

/// Update manager
pub struct SigmaUpdateManager2 {
    updates: Vec<Update>,
    auto_update_enabled: bool,
}

impl SigmaUpdateManager2 {
    pub fn new() -> Self {
        Self {
            updates: Vec::new(),
            auto_update_enabled: false,
        }
    }

    pub fn add_update(&mut self, update: Update) {
        self.updates.push(update);
    }

    pub fn get_updates(&self) -> &[Update] {
        &self.updates
    }

    pub fn get_update(&self, id: &str) -> Option<&Update> {
        self.updates.iter().find(|u| u.id == id)
    }

    pub fn start_update(&mut self, id: String) -> Result<(), String> {
        if let Some(update) = self.updates.iter_mut().find(|u| u.id == id) {
            update.status = UpdateStatus::Downloading;
            update.status = UpdateStatus::Installing;
            update.status = UpdateStatus::Installed;
            Ok(())
        } else {
            Err(format!("Update {} not found", id))
        }
    }

    pub fn get_available_updates(&self) -> Vec<&Update> {
        self.updates
            .iter()
            .filter(|u| u.status == UpdateStatus::Available)
            .collect()
    }

    pub fn get_security_updates(&self) -> Vec<&Update> {
        self.updates
            .iter()
            .filter(|u| u.update_type == UpdateType::Security)
            .collect()
    }

    pub fn set_auto_update(&mut self, enabled: bool) {
        self.auto_update_enabled = enabled;
    }

    pub fn is_auto_update_enabled(&self) -> bool {
        self.auto_update_enabled
    }
}

impl Default for SigmaUpdateManager2 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_manager_creation() {
        let manager = SigmaUpdateManager2::new();
        assert_eq!(manager.get_updates().len(), 0);
        assert!(!manager.is_auto_update_enabled());
    }

    #[test]
    fn test_add_update() {
        let mut manager = SigmaUpdateManager2::new();
        let update = Update {
            id: "sys-001".to_string(),
            name: "System Update".to_string(),
            update_type: UpdateType::System,
            status: UpdateStatus::Available,
            version: "1.0".to_string(),
            size_bytes: 1000,
        };
        manager.add_update(update);
        assert_eq!(manager.get_updates().len(), 1);
    }

    #[test]
    fn test_start_update() {
        let mut manager = SigmaUpdateManager2::new();
        let update = Update {
            id: "sys-001".to_string(),
            name: "System Update".to_string(),
            update_type: UpdateType::System,
            status: UpdateStatus::Available,
            version: "1.0".to_string(),
            size_bytes: 1000,
        };
        manager.add_update(update);
        manager.start_update("sys-001".to_string()).unwrap();
        assert_eq!(manager.get_updates()[0].status, UpdateStatus::Installed);
    }

    #[test]
    fn test_available_updates() {
        let mut manager = SigmaUpdateManager2::new();
        let update1 = Update {
            id: "sys-001".to_string(),
            name: "System Update".to_string(),
            update_type: UpdateType::System,
            status: UpdateStatus::Available,
            version: "1.0".to_string(),
            size_bytes: 1000,
        };
        let update2 = Update {
            id: "sys-002".to_string(),
            name: "System Update 2".to_string(),
            update_type: UpdateType::System,
            status: UpdateStatus::Installed,
            version: "1.0".to_string(),
            size_bytes: 1000,
        };
        manager.add_update(update1);
        manager.add_update(update2);
        assert_eq!(manager.get_available_updates().len(), 1);
    }

    #[test]
    fn test_security_updates() {
        let mut manager = SigmaUpdateManager2::new();
        let update1 = Update {
            id: "sec-001".to_string(),
            name: "Security Update".to_string(),
            update_type: UpdateType::Security,
            status: UpdateStatus::Available,
            version: "1.0".to_string(),
            size_bytes: 1000,
        };
        let update2 = Update {
            id: "feat-001".to_string(),
            name: "Feature Update".to_string(),
            update_type: UpdateType::Feature,
            status: UpdateStatus::Available,
            version: "1.0".to_string(),
            size_bytes: 1000,
        };
        manager.add_update(update1);
        manager.add_update(update2);
        assert_eq!(manager.get_security_updates().len(), 1);
    }

    #[test]
    fn test_auto_update() {
        let mut manager = SigmaUpdateManager2::new();
        assert!(!manager.is_auto_update_enabled());
        manager.set_auto_update(true);
        assert!(manager.is_auto_update_enabled());
    }
}
