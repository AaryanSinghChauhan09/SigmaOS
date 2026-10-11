// SPDX-License-Identifier: MIT
// SigmaOS - Update Manager (inspired by Linux Mint Update Manager)
// Inspired by Linux Mint Update Manager (https://github.com/linuxmint/mintupdate)
// Provides system update management

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};

/// Update type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateType {
    Security,
    Kernel,
    Application,
    Package,
}

/// Update priority
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdatePriority {
    Critical,
    High,
    Medium,
    Low,
}

/// System update
#[derive(Debug, Clone)]
pub struct SystemUpdate {
    pub name: String,
    pub version: String,
    pub update_type: UpdateType,
    pub priority: UpdatePriority,
    pub size: u64, // bytes
    pub description: String,
    pub installed: AtomicBool,
}

/// Update manager
pub struct UpdateManager {
    pub updates: BTreeMap<String, SystemUpdate>,
    pub active: AtomicBool,
    pub auto_update_enabled: AtomicBool,
}

impl UpdateManager {
    pub fn new() -> Self {
        Self {
            updates: BTreeMap::new(),
            active: AtomicBool::new(false),
            auto_update_enabled: AtomicBool::new(true),
        }
    }
    
    /// Initialize update manager
    pub fn init(&mut self) {
        self.active.store(true, Ordering::Relaxed);
        self.check_for_updates();
    }
    
    /// Check for updates
    pub fn check_for_updates(&mut self) {
        // In real implementation, would check package repositories
        // (apt update, dnf check-update, pacman -Syu, etc.)
        
        // Add some sample updates
        self.add_update(SystemUpdate {
            name: "linux-firmware".to_string(),
            version: "2023.10".to_string(),
            update_type: UpdateType::Kernel,
            priority: UpdatePriority::High,
            size: 10 * 1024 * 1024, // 10MB
            description: "Firmware packages for Linux kernel".to_string(),
            installed: AtomicBool::new(false),
        });
        
        self.add_update(SystemUpdate {
            name: "openssl".to_string(),
            version: "3.1.0".to_string(),
            update_type: UpdateType::Security,
            priority: UpdatePriority::Critical,
            size: 5 * 1024 * 1024, // 5MB
            description: "Secure SSL/TLS library".to_string(),
            installed: AtomicBool::new(false),
        });
        
        self.add_update(SystemUpdate {
            name: "firefox".to_string(),
            version: "121.0".to_string(),
            update_type: UpdateType::Application,
            priority: UpdatePriority::Medium,
            size: 100 * 1024 * 1024, // 100MB
            description: "Web Browser".to_string(),
            installed: AtomicBool::new(false),
        });
    }
    
    /// Add update
    pub fn add_update(&mut self, update: SystemUpdate) {
        self.updates.insert(update.name.clone(), update);
    }
    
    /// Get update
    pub fn get_update(&self, name: &str) -> Option<&SystemUpdate> {
        self.updates.get(name)
    }
    
    /// Install update
    pub fn install_update(&mut self, name: &str) -> Result<(), &'static str> {
        let update = self.updates.get_mut(name).ok_or("Update not found")?;
        
        // In real implementation, would run:
        // apt upgrade {name}
        // or dnf upgrade {name}
        // or pacman -S {name}
        
        update.installed.store(true, Ordering::Relaxed);
        
        Ok(())
    }
    
    /// Install all updates
    pub fn install_all_updates(&mut self) -> Result<(), &'static str> {
        for update in self.updates.values_mut() {
            if !update.installed.load(Ordering::Relaxed) {
                update.installed.store(true, Ordering::Relaxed);
            }
        }
        Ok(())
    }
    
    /// Get updates by priority
    pub fn get_updates_by_priority(&self, priority: UpdatePriority) -> Vec<&SystemUpdate> {
        self.updates.values()
            .filter(|u| u.priority == priority)
            .collect()
    }
    
    /// Get security updates
    pub fn get_security_updates(&self) -> Vec<&SystemUpdate> {
        self.updates.values()
            .filter(|u| u.update_type == UpdateType::Security)
            .collect()
    }
    
    /// Get kernel updates
    pub fn get_kernel_updates(&self) -> Vec<&SystemUpdate> {
        self.updates.values()
            .filter(|u| u.update_type == UpdateType::Kernel)
            .collect()
    }
    
    /// Get pending updates
    pub fn get_pending_updates(&self) -> Vec<&SystemUpdate> {
        self.updates.values()
            .filter(|u| !u.installed.load(Ordering::Relaxed))
            .collect()
    }
    
    /// Enable auto update
    pub fn enable_auto_update(&self) {
        self.auto_update_enabled.store(true, Ordering::Relaxed);
    }
    
    /// Disable auto update
    pub fn disable_auto_update(&self) {
        self.auto_update_enabled.store(false, Ordering::Relaxed);
    }
    
    /// Is auto update enabled
    pub fn is_auto_update_enabled(&self) -> bool {
        self.auto_update_enabled.load(Ordering::Relaxed)
    }
    
    /// Is active
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }
}

impl Default for UpdateManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global update manager
static mut UPDATE_MANAGER: Option<UpdateManager> = None;

/// Initialize update manager
pub fn init_update_manager() {
    unsafe {
        UPDATE_MANAGER = Some(UpdateManager::new());
    }
}

/// Get global update manager
pub fn get_update_manager() -> Option<&'static mut UpdateManager> {
    unsafe {
        UPDATE_MANAGER.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_update_manager_init() {
        let mut manager = UpdateManager::new();
        manager.init();
        assert!(manager.is_active());
        assert!(manager.updates.len() > 0);
    }
    
    #[test]
    fn test_install_update() {
        let mut manager = UpdateManager::new();
        manager.check_for_updates();
        manager.install_update("openssl").unwrap();
        assert!(manager.get_update("openssl").unwrap().installed.load(Ordering::Relaxed));
    }
    
    #[test]
    fn test_install_all_updates() {
        let mut manager = UpdateManager::new();
        manager.check_for_updates();
        manager.install_all_updates().unwrap();
        let pending = manager.get_pending_updates();
        assert_eq!(pending.len(), 0);
    }
    
    #[test]
    fn test_get_security_updates() {
        let mut manager = UpdateManager::new();
        manager.check_for_updates();
        let security = manager.get_security_updates();
        assert!(security.len() > 0);
    }
    
    #[test]
    fn test_get_kernel_updates() {
        let mut manager = UpdateManager::new();
        manager.check_for_updates();
        let kernel = manager.get_kernel_updates();
        assert!(kernel.len() > 0);
    }
}
