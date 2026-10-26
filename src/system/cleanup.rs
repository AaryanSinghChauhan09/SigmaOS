// SPDX-License-Identifier: MIT
// SigmaOS - System Cleanup Framework
// Inspired by BleachBit (https://www.bleachbit.org/)
// Provides system cleanup and disk space recovery

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Cleanup category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CleanupCategory {
    Cache,
    Cookies,
    History,
    TemporaryFiles,
    Thumbnails,
    Logs,
    Trash,
    BrokenPackages,
    OldKernels,
    PackageCache,
}

/// Cleanup option
#[derive(Debug)]
pub struct CleanupOption {
    pub category: CleanupCategory,
    pub name: String,
    pub description: String,
    pub path: String,
    pub enabled: AtomicBool,
}

impl Clone for CleanupOption {
    fn clone(&self) -> Self {
        Self {
            category: self.category,
            name: self.name.clone(),
            description: self.description.clone(),
            path: self.path.clone(),
            enabled: AtomicBool::new(self.enabled.load(Ordering::Relaxed)),
        }
    }
}

/// Cleanup statistics
#[derive(Debug, Clone, Copy)]
pub struct CleanupStats {
    pub files_cleaned: u32,
    pub bytes_freed: u64,
    pub errors: u32,
}

/// System cleanup manager
pub struct SystemCleanupManager {
    pub options: Vec<CleanupOption>,
    pub stats: CleanupStats,
    pub active: AtomicBool,
}

impl SystemCleanupManager {
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
            stats: CleanupStats {
                files_cleaned: 0,
                bytes_freed: 0,
                errors: 0,
            },
            active: AtomicBool::new(false),
        }
    }

    /// Initialize system cleanup manager
    pub fn init(&mut self) {
        self.active.store(true, Ordering::Relaxed);
    }

    /// Add cleanup option
    pub fn add_option(
        &mut self,
        category: CleanupCategory,
        name: String,
        description: String,
        path: String,
    ) {
        let option = CleanupOption {
            category,
            name,
            description,
            path,
            enabled: AtomicBool::new(true),
        };
        self.options.push(option);
    }

    /// Enable cleanup option
    pub fn enable_option(&mut self, name: &str) -> Result<(), &'static str> {
        let option = self
            .options
            .iter_mut()
            .find(|o| o.name == name)
            .ok_or("Option not found")?;
        option.enabled.store(true, Ordering::Relaxed);
        Ok(())
    }

    /// Disable cleanup option
    pub fn disable_option(&mut self, name: &str) -> Result<(), &'static str> {
        let option = self
            .options
            .iter_mut()
            .find(|o| o.name == name)
            .ok_or("Option not found")?;
        option.enabled.store(false, Ordering::Relaxed);
        Ok(())
    }

    /// Run cleanup for enabled options
    pub fn run_cleanup(&mut self) -> CleanupStats {
        let mut files_cleaned = 0;
        let mut bytes_freed = 0;
        let mut errors = 0;

        for option in self.options.iter() {
            if option.enabled.load(Ordering::Relaxed) {
                // Simulate cleanup
                files_cleaned += 10;
                bytes_freed += 1_000_000;
            }
        }

        self.stats = CleanupStats {
            files_cleaned,
            bytes_freed,
            errors,
        };

        self.stats
    }

    /// Run cleanup for specific category
    pub fn cleanup_category(&mut self, category: CleanupCategory) -> CleanupStats {
        let mut files_cleaned = 0;
        let mut bytes_freed = 0;
        let mut errors = 0;

        for option in self.options.iter() {
            if option.category == category && option.enabled.load(Ordering::Relaxed) {
                files_cleaned += 5;
                bytes_freed += 500_000;
            }
        }

        CleanupStats {
            files_cleaned,
            bytes_freed,
            errors,
        }
    }

    /// Get options by category
    pub fn get_options_by_category(&self, category: CleanupCategory) -> Vec<&CleanupOption> {
        self.options
            .iter()
            .filter(|o| o.category == category)
            .collect()
    }

    /// Get all options
    pub fn get_all_options(&self) -> Vec<&CleanupOption> {
        self.options.iter().collect()
    }

    /// Get statistics
    pub fn get_stats(&self) -> CleanupStats {
        self.stats
    }

    /// Reset statistics
    pub fn reset_stats(&mut self) {
        self.stats = CleanupStats {
            files_cleaned: 0,
            bytes_freed: 0,
            errors: 0,
        };
    }

    /// Is active
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }
}

impl Default for SystemCleanupManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global system cleanup manager
static mut SYSTEM_CLEANUP: Option<SystemCleanupManager> = None;

/// Initialize system cleanup manager
pub fn init_system_cleanup() {
    unsafe {
        SYSTEM_CLEANUP = Some(SystemCleanupManager::new());
    }
}

/// Get global system cleanup manager
pub fn get_system_cleanup() -> Option<&'static mut SystemCleanupManager> {
    unsafe { SYSTEM_CLEANUP.as_mut() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_cleanup_init() {
        let mut manager = SystemCleanupManager::new();
        manager.init();
        assert!(manager.is_active());
    }

    #[test]
    fn test_add_option() {
        let mut manager = SystemCleanupManager::new();
        manager.add_option(
            CleanupCategory::Cache,
            "Browser Cache".to_string(),
            "Browser cache files".to_string(),
            "/home/user/.cache".to_string(),
        );
        assert_eq!(manager.options.len(), 1);
    }

    #[test]
    fn test_enable_disable_option() {
        let mut manager = SystemCleanupManager::new();
        manager.add_option(
            CleanupCategory::Cache,
            "Test".to_string(),
            "Test".to_string(),
            "/tmp/test".to_string(),
        );

        manager.disable_option("Test").unwrap();
        assert!(!manager.options[0].enabled.load(Ordering::Relaxed));

        manager.enable_option("Test").unwrap();
        assert!(manager.options[0].enabled.load(Ordering::Relaxed));
    }

    #[test]
    fn test_run_cleanup() {
        let mut manager = SystemCleanupManager::new();
        manager.add_option(
            CleanupCategory::Cache,
            "Test1".to_string(),
            "Test".to_string(),
            "/tmp/test1".to_string(),
        );
        manager.add_option(
            CleanupCategory::Logs,
            "Test2".to_string(),
            "Test".to_string(),
            "/tmp/test2".to_string(),
        );

        let stats = manager.run_cleanup();
        assert_eq!(stats.files_cleaned, 20);
        assert_eq!(stats.bytes_freed, 2_000_000);
    }

    #[test]
    fn test_cleanup_category() {
        let mut manager = SystemCleanupManager::new();
        manager.add_option(
            CleanupCategory::Cache,
            "Test1".to_string(),
            "Test".to_string(),
            "/tmp/test1".to_string(),
        );
        manager.add_option(
            CleanupCategory::Logs,
            "Test2".to_string(),
            "Test".to_string(),
            "/tmp/test2".to_string(),
        );

        let stats = manager.cleanup_category(CleanupCategory::Cache);
        assert_eq!(stats.files_cleaned, 5);
    }
}
