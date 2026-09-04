// SigmaOS Checkupdates Engine
// Implements Arch Linux's checkupdates functionality
// Scans repository index diffs without locking the primary package database

use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;

/// Package update information
#[derive(Debug, Clone)]
pub struct PackageUpdate {
    pub name: String,
    pub current_version: String,
    pub new_version: String,
    pub repository: String,
    pub size: u64,
}

/// Checkupdates engine
pub struct CheckupdatesEngine {
    pub current_packages: BTreeMap<String, String>,
    pub available_updates: Vec<PackageUpdate>,
}

impl CheckupdatesEngine {
    pub fn new() -> Self {
        Self {
            current_packages: BTreeMap::new(),
            available_updates: Vec::new(),
        }
    }

    /// Set current package versions
    pub fn set_current_packages(&mut self, packages: BTreeMap<String, String>) {
        self.current_packages = packages;
    }

    /// Check for updates
    pub fn check_updates(&mut self) -> Vec<&PackageUpdate> {
        // In real implementation, would scan repository index diffs
        // For now, simulate with sample data

        self.available_updates = vec![
            PackageUpdate {
                name: "linux-kernel".to_string(),
                current_version: "6.0.0".to_string(),
                new_version: "6.1.0".to_string(),
                repository: "core".to_string(),
                size: 1024 * 1024 * 50,
            },
            PackageUpdate {
                name: "firefox".to_string(),
                current_version: "120.0".to_string(),
                new_version: "121.0".to_string(),
                repository: "extra".to_string(),
                size: 1024 * 1024 * 100,
            },
        ];

        self.available_updates.iter().collect()
    }

    /// Get updates by repository
    pub fn get_updates_by_repo(&self, repo: &str) -> Vec<&PackageUpdate> {
        self.available_updates
            .iter()
            .filter(|u| u.repository == repo)
            .collect()
    }

    /// Get total size of updates
    pub fn get_total_size(&self) -> u64 {
        self.available_updates.iter().map(|u| u.size).sum()
    }

    /// Get update count
    pub fn get_update_count(&self) -> usize {
        self.available_updates.len()
    }

    /// Get update summary
    pub fn get_summary(&self) -> String {
        format!(
            "Available Updates: {}\nTotal Size: {} MB",
            self.get_update_count(),
            self.get_total_size() / (1024 * 1024)
        )
    }
}

impl Default for CheckupdatesEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkupdates() {
        let mut engine = CheckupdatesEngine::new();
        let updates = engine.check_updates();
        assert_eq!(updates.len(), 2);
    }

    #[test]
    fn test_update_summary() {
        let mut engine = CheckupdatesEngine::new();
        engine.check_updates();
        let summary = engine.get_summary();
        assert!(summary.contains("Available Updates"));
    }
}
