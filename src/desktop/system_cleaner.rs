// SPDX-License-Identifier: MIT
// SigmaOS - System Cleaner
// Inspired by BleachBit and CCleaner
// Provides system cleaning and privacy features

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Cleaner category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CleanerCategory {
    Cache,
    Cookies,
    History,
    TemporaryFiles,
    Logs,
    Thumbnails,
    BrokenPackages,
    OldKernels,
    Trash,
}

/// Cleaner option
#[derive(Debug, Clone)]
pub struct CleanerOption {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub category: CleanerCategory,
    pub enabled: bool,
    pub space_freed: u64,
}

/// Clean result
#[derive(Debug, Clone)]
pub struct CleanResult {
    pub options_cleaned: Vec<String>,
    pub total_space_freed: u64,
    pub errors: Vec<String>,
}

/// System cleaner
pub struct SystemCleaner {
    pub options: BTreeMap<u32, CleanerOption>,
    pub custom_options: Vec<CleanerOption>,
    pub auto_clean: bool,
    pub schedule_interval: u32,
}

impl SystemCleaner {
    pub fn new() -> Self {
        let mut options = BTreeMap::new();

        // Default cleaner options
        options.insert(
            1,
            CleanerOption {
                id: 1,
                name: "Browser Cache".to_string(),
                description: "Clear browser cache files".to_string(),
                category: CleanerCategory::Cache,
                enabled: true,
                space_freed: 0,
            },
        );

        options.insert(
            2,
            CleanerOption {
                id: 2,
                name: "Cookies".to_string(),
                description: "Clear browser cookies".to_string(),
                category: CleanerCategory::Cookies,
                enabled: false,
                space_freed: 0,
            },
        );

        options.insert(
            3,
            CleanerOption {
                id: 3,
                name: "Browser History".to_string(),
                description: "Clear browser history".to_string(),
                category: CleanerCategory::History,
                enabled: false,
                space_freed: 0,
            },
        );

        options.insert(
            4,
            CleanerOption {
                id: 4,
                name: "Temporary Files".to_string(),
                description: "Clear system temporary files".to_string(),
                category: CleanerCategory::TemporaryFiles,
                enabled: true,
                space_freed: 0,
            },
        );

        options.insert(
            5,
            CleanerOption {
                id: 5,
                name: "System Logs".to_string(),
                description: "Clear system log files".to_string(),
                category: CleanerCategory::Logs,
                enabled: false,
                space_freed: 0,
            },
        );

        options.insert(
            6,
            CleanerOption {
                id: 6,
                name: "Thumbnail Cache".to_string(),
                description: "Clear thumbnail cache".to_string(),
                category: CleanerCategory::Thumbnails,
                enabled: true,
                space_freed: 0,
            },
        );

        options.insert(
            7,
            CleanerOption {
                id: 7,
                name: "Broken Packages".to_string(),
                description: "Remove broken package dependencies".to_string(),
                category: CleanerCategory::BrokenPackages,
                enabled: false,
                space_freed: 0,
            },
        );

        options.insert(
            8,
            CleanerOption {
                id: 8,
                name: "Old Kernels".to_string(),
                description: "Remove old kernel versions".to_string(),
                category: CleanerCategory::OldKernels,
                enabled: false,
                space_freed: 0,
            },
        );

        options.insert(
            9,
            CleanerOption {
                id: 9,
                name: "Trash".to_string(),
                description: "Empty trash directories".to_string(),
                category: CleanerCategory::Trash,
                enabled: true,
                space_freed: 0,
            },
        );

        Self {
            options,
            custom_options: Vec::new(),
            auto_clean: false,
            schedule_interval: 86400, // 24 hours
        }
    }

    /// Enable option
    pub fn enable_option(&mut self, id: u32) -> Result<(), &'static str> {
        let option = self.options.get_mut(&id).ok_or("Option not found")?;
        option.enabled = true;
        Ok(())
    }

    /// Disable option
    pub fn disable_option(&mut self, id: u32) -> Result<(), &'static str> {
        let option = self.options.get_mut(&id).ok_or("Option not found")?;
        option.enabled = false;
        Ok(())
    }

    /// Get enabled options
    pub fn get_enabled_options(&self) -> Vec<&CleanerOption> {
        self.options.values().filter(|o| o.enabled).collect()
    }

    /// Get options by category
    pub fn get_options_by_category(&self, category: CleanerCategory) -> Vec<&CleanerOption> {
        self.options
            .values()
            .filter(|o| o.category == category)
            .collect()
    }

    /// Add custom option
    pub fn add_custom_option(
        &mut self,
        name: String,
        description: String,
        category: CleanerCategory,
    ) -> u32 {
        let id = 100 + self.custom_options.len() as u32;
        let option = CleanerOption {
            id,
            name,
            description,
            category,
            enabled: true,
            space_freed: 0,
        };
        self.custom_options.push(option);
        id
    }

    /// Remove custom option
    pub fn remove_custom_option(&mut self, id: u32) -> Result<(), &'static str> {
        let index = self
            .custom_options
            .iter()
            .position(|o| o.id == id)
            .ok_or("Option not found")?;
        self.custom_options.remove(index);
        Ok(())
    }

    /// Preview clean
    pub fn preview_clean(&self) -> CleanResult {
        let enabled_options = self.get_enabled_options();
        let total_space_freed = enabled_options.iter().map(|_| 1024 * 1024).sum(); // Simplified

        CleanResult {
            options_cleaned: enabled_options.iter().map(|o| o.name.clone()).collect(),
            total_space_freed,
            errors: Vec::new(),
        }
    }

    /// Perform clean
    pub fn clean(&mut self) -> CleanResult {
        let mut result = self.preview_clean();

        // Simulate cleaning
        for option in self.options.values_mut() {
            if option.enabled {
                option.space_freed = 1024 * 1024; // 1MB
            }
        }

        result
    }

    /// Set auto clean
    pub fn set_auto_clean(&mut self, enabled: bool) {
        self.auto_clean = enabled;
    }

    /// Set schedule interval
    pub fn set_schedule_interval(&mut self, interval: u32) {
        self.schedule_interval = interval.max(3600); // Minimum 1 hour
    }

    /// Get all options
    pub fn get_all_options(&self) -> Vec<&CleanerOption> {
        self.options
            .values()
            .chain(self.custom_options.iter())
            .collect()
    }

    /// Format size
    pub fn format_size(size: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;

        if size >= GB {
            format!("{:.2} GB", size as f64 / GB as f64)
        } else if size >= MB {
            format!("{:.2} MB", size as f64 / MB as f64)
        } else if size >= KB {
            format!("{:.2} KB", size as f64 / KB as f64)
        } else {
            format!("{} B", size)
        }
    }
}

/// Global system cleaner
static mut SYSTEM_CLEANER: Option<SystemCleaner> = None;

/// Initialize system cleaner
pub fn init_system_cleaner() {
    unsafe {
        SYSTEM_CLEANER = Some(SystemCleaner::new());
    }
}

/// Get system cleaner
pub fn get_system_cleaner() -> &'static mut SystemCleaner {
    unsafe {
        SYSTEM_CLEANER
            .as_mut()
            .expect("System cleaner not initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_cleaner_creation() {
        let cleaner = SystemCleaner::new();
        assert_eq!(cleaner.get_all_options().len(), 9);
    }

    #[test]
    fn test_enable_option() {
        let mut cleaner = SystemCleaner::new();
        cleaner.enable_option(2).unwrap();
        let enabled = cleaner.get_enabled_options();
        assert!(enabled.iter().any(|o| o.id == 2));
    }

    #[test]
    fn test_disable_option() {
        let mut cleaner = SystemCleaner::new();
        cleaner.disable_option(1).unwrap();
        let enabled = cleaner.get_enabled_options();
        assert!(!enabled.iter().any(|o| o.id == 1));
    }

    #[test]
    fn test_add_custom_option() {
        let mut cleaner = SystemCleaner::new();
        let id = cleaner.add_custom_option(
            "Custom".to_string(),
            "Description".to_string(),
            CleanerCategory::Cache,
        );
        assert!(id >= 100);
    }

    #[test]
    fn test_preview_clean() {
        let cleaner = SystemCleaner::new();
        let result = cleaner.preview_clean();
        assert!(result.total_space_freed > 0);
    }

    #[test]
    fn test_format_size() {
        assert_eq!(SystemCleaner::format_size(1024), "1.00 KB");
        assert_eq!(SystemCleaner::format_size(1024 * 1024), "1.00 MB");
    }
}
