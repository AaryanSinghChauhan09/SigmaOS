// SPDX-License-Identifier: MIT
// SigmaOS - Package Updates
// Package update management inspired by Arch (pacman), Debian (apt), and CachyOS

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Update severity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateSeverity {
    Low,
    Normal,
    Important,
    Critical,
    Security,
}

/// Update source
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateSource {
    Official(String),
    Community(String),
    ThirdParty(String),
}

/// Package update record
#[derive(Debug, Clone)]
pub struct PackageUpdate {
    pub package_name: String,
    pub version_old: String,
    pub version_new: String,
    pub severity: UpdateSeverity,
    pub source: UpdateSource,
    pub size_bytes: u64,
    pub description: String,
}

/// Package update manager
pub struct SigmaPackageUpdates {
    updates: Vec<PackageUpdate>,
    auto_update_enabled: bool,
    check_interval_hours: u32,
}

impl SigmaPackageUpdates {
    pub fn new() -> Self {
        Self {
            updates: Vec::new(),
            auto_update_enabled: false,
            check_interval_hours: 24,
        }
    }

    pub fn add_update(&mut self, update: PackageUpdate) {
        self.updates.push(update);
    }

    pub fn get_updates(&self) -> &[PackageUpdate] {
        &self.updates
    }

    pub fn get_critical_updates(&self) -> Vec<&PackageUpdate> {
        self.updates
            .iter()
            .filter(|u| {
                u.severity == UpdateSeverity::Critical || u.severity == UpdateSeverity::Security
            })
            .collect()
    }

    pub fn is_auto_update_enabled(&self) -> bool {
        self.auto_update_enabled
    }

    pub fn set_auto_update(&mut self, enabled: bool) {
        self.auto_update_enabled = enabled;
    }

    pub fn get_check_interval(&self) -> u32 {
        self.check_interval_hours
    }

    pub fn set_check_interval(&mut self, hours: u32) {
        self.check_interval_hours = hours;
    }

    pub fn clear_updates(&mut self) {
        self.updates.clear();
    }
}

impl Default for SigmaPackageUpdates {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize package updates
pub fn init_sigmapackageupdates() {
    // Initialization logic
}

/// Get package updates reference
pub fn get_sigmapackageupdates() -> Result<&'static SigmaPackageUpdates, String> {
    Err("Not initialized".to_string())
}

/// Get mutable package updates reference
pub fn get_sigmapackageupdates_mut() -> Result<&'static mut SigmaPackageUpdates, String> {
    Err("Not initialized".to_string())
}

/// Check for updates
pub fn check_for_updates() -> Result<Vec<PackageUpdate>, String> {
    Ok(Vec::new())
}

/// Get critical updates
pub fn get_critical_updates() -> Result<Vec<PackageUpdate>, String> {
    Ok(Vec::new())
}

/// Get check interval
pub fn get_check_interval() -> Result<u32, String> {
    Ok(24)
}

/// Set auto update
pub fn set_auto_update(enabled: bool) -> Result<(), String> {
    Ok(())
}

/// Set check interval
pub fn set_check_interval(hours: u32) -> Result<(), String> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_updates_creation() {
        let updates = SigmaPackageUpdates::new();
        assert_eq!(updates.get_updates().len(), 0);
        assert!(!updates.is_auto_update_enabled());
        assert_eq!(updates.get_check_interval(), 24);
    }

    #[test]
    fn test_add_update() {
        let mut updates = SigmaPackageUpdates::new();
        let update = PackageUpdate {
            package_name: "test".to_string(),
            version_old: "1.0".to_string(),
            version_new: "2.0".to_string(),
            severity: UpdateSeverity::Normal,
            source: UpdateSource::Official("official".to_string()),
            size_bytes: 1000,
            description: "Test update".to_string(),
        };
        updates.add_update(update);
        assert_eq!(updates.get_updates().len(), 1);
    }

    #[test]
    fn test_critical_updates() {
        let mut updates = SigmaPackageUpdates::new();
        let update1 = PackageUpdate {
            package_name: "critical".to_string(),
            version_old: "1.0".to_string(),
            version_new: "2.0".to_string(),
            severity: UpdateSeverity::Critical,
            source: UpdateSource::Official("official".to_string()),
            size_bytes: 1000,
            description: "Critical update".to_string(),
        };
        let update2 = PackageUpdate {
            package_name: "normal".to_string(),
            version_old: "1.0".to_string(),
            version_new: "2.0".to_string(),
            severity: UpdateSeverity::Normal,
            source: UpdateSource::Official("official".to_string()),
            size_bytes: 1000,
            description: "Normal update".to_string(),
        };
        updates.add_update(update1);
        updates.add_update(update2);
        assert_eq!(updates.get_critical_updates().len(), 1);
    }

    #[test]
    fn test_auto_update() {
        let mut updates = SigmaPackageUpdates::new();
        assert!(!updates.is_auto_update_enabled());
        updates.set_auto_update(true);
        assert!(updates.is_auto_update_enabled());
    }

    #[test]
    fn test_check_interval() {
        let mut updates = SigmaPackageUpdates::new();
        assert_eq!(updates.get_check_interval(), 24);
        updates.set_check_interval(12);
        assert_eq!(updates.get_check_interval(), 12);
    }
}
