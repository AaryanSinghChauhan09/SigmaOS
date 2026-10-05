//! Update Manager
//!
//! System update management inspired by Linux Mint's mintupdate, supporting
//! kernel updates, package updates, security patches, and version management.

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Update level (matching Linux Mint's classification)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UpdateLevel {
    Safety = 1,
    Recommended = 2,
    Feature = 3,
    Unstable = 4,
}

impl UpdateLevel {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "safety" => Some(UpdateLevel::Safety),
            "recommended" => Some(UpdateLevel::Recommended),
            "feature" => Some(UpdateLevel::Feature),
            "unstable" => Some(UpdateLevel::Unstable),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            UpdateLevel::Safety => "Safety",
            UpdateLevel::Recommended => "Recommended",
            UpdateLevel::Feature => "Feature",
            UpdateLevel::Unstable => "Unstable",
        }
    }
}

/// Update category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateCategory {
    Kernel,
    Security,
    Bugfix,
    Feature,
    Dependency,
}

impl UpdateCategory {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "kernel" => Some(UpdateCategory::Kernel),
            "security" => Some(UpdateCategory::Security),
            "bugfix" => Some(UpdateCategory::Bugfix),
            "feature" => Some(UpdateCategory::Feature),
            "dependency" => Some(UpdateCategory::Dependency),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            UpdateCategory::Kernel => "Kernel",
            UpdateCategory::Security => "Security",
            UpdateCategory::Bugfix => "Bugfix",
            UpdateCategory::Feature => "Feature",
            UpdateCategory::Dependency => "Dependency",
        }
    }
}

/// Update package information
#[derive(Debug, Clone)]
pub struct UpdatePackage {
    pub name: String,
    pub old_version: String,
    pub new_version: String,
    pub level: UpdateLevel,
    pub category: UpdateCategory,
    pub size: u64,
    pub description: String,
    pub source: String,
    pub is_security: bool,
    pub timestamp: u64,
}

impl UpdatePackage {
    pub fn new(
        name: String,
        old_version: String,
        new_version: String,
        level: UpdateLevel,
        category: UpdateCategory,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            name,
            old_version,
            new_version,
            level,
            category,
            size: 0,
            description: String::new(),
            source: "unknown".to_string(),
            is_security: category == UpdateCategory::Security,
            timestamp,
        }
    }

    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }

    pub fn set_description(&mut self, desc: String) {
        self.description = desc;
    }

    pub fn set_source(&mut self, source: String) {
        self.source = source;
    }

    pub fn mark_security(&mut self) {
        self.is_security = true;
    }
}

/// Update manager configuration
#[derive(Debug, Clone)]
pub struct UpdateConfig {
    pub auto_check: bool,
    pub auto_install_safety: bool,
    pub auto_install_recommended: bool,
    pub auto_install_security: bool,
    pub check_interval_hours: u32,
    pub notify_updates: bool,
    pub blacklist: Vec<String>,
}

impl Default for UpdateConfig {
    fn default() -> Self {
        Self {
            auto_check: true,
            auto_install_safety: true,
            auto_install_recommended: false,
            auto_install_security: true,
            check_interval_hours: 24,
            notify_updates: true,
            blacklist: Vec::new(),
        }
    }
}

/// Update manager
#[derive(Debug)]
pub struct UpdateManager {
    available_updates: HashMap<String, UpdatePackage>,
    installed_updates: Vec<String>,
    config: UpdateConfig,
    update_count: u32,
    security_count: u32,
}

impl UpdateManager {
    pub fn new() -> Self {
        Self {
            available_updates: HashMap::new(),
            installed_updates: Vec::new(),
            config: UpdateConfig::default(),
            update_count: 0,
            security_count: 0,
        }
    }

    pub fn with_config(config: UpdateConfig) -> Self {
        Self {
            available_updates: HashMap::new(),
            installed_updates: Vec::new(),
            config,
            update_count: 0,
            security_count: 0,
        }
    }

    /// Set configuration
    pub fn set_config(&mut self, config: UpdateConfig) {
        self.config = config;
    }

    /// Get configuration
    pub fn get_config(&self) -> &UpdateConfig {
        &self.config
    }

    /// Add a blacklist entry
    pub fn add_blacklist(&mut self, package: String) {
        if !self.config.blacklist.contains(&package) {
            self.config.blacklist.push(package);
        }
    }

    /// Remove a blacklist entry
    pub fn remove_blacklist(&mut self, package: &str) {
        self.config.blacklist.retain(|p| p != package);
    }

    /// Check for updates (simulated)
    pub fn check_updates(&mut self) -> Vec<&UpdatePackage> {
        self.available_updates.clear();

        // Simulate update discovery
        let sample_updates = vec![
            ("linux-kernel", "6.5.0", "6.6.0", UpdateLevel::Safety, UpdateCategory::Kernel),
            ("openssl", "3.0.8", "3.0.9", UpdateLevel::Safety, UpdateCategory::Security),
            ("bash", "5.1.0", "5.2.0", UpdateLevel::Recommended, UpdateCategory::Bugfix),
            ("firefox", "120.0", "121.0", UpdateLevel::Feature, UpdateCategory::Feature),
            ("libssl", "1.1.1", "3.0.9", UpdateLevel::Recommended, UpdateCategory::Dependency),
        ];

        for (name, old, new, level, category) in sample_updates {
            if !self.config.blacklist.contains(&name.to_string()) {
                let update = UpdatePackage::new(
                    name.to_string(),
                    old.to_string(),
                    new.to_string(),
                    level,
                    category,
                );
                self.available_updates.insert(name.to_string(), update);
            }
        }

        self.update_count = self.available_updates.len() as u32;
        self.security_count = self.available_updates.values()
            .filter(|u| u.is_security)
            .count() as u32;

        self.available_updates.values().collect()
    }

    /// Get all available updates
    pub fn get_updates(&self) -> Vec<&UpdatePackage> {
        self.available_updates.values().collect()
    }

    /// Get updates by level
    pub fn get_updates_by_level(&self, level: UpdateLevel) -> Vec<&UpdatePackage> {
        self.available_updates.values()
            .filter(|u| u.level == level)
            .collect()
    }

    /// Get security updates
    pub fn get_security_updates(&self) -> Vec<&UpdatePackage> {
        self.available_updates.values()
            .filter(|u| u.is_security)
            .collect()
    }

    /// Get kernel updates
    pub fn get_kernel_updates(&self) -> Vec<&UpdatePackage> {
        self.available_updates.values()
            .filter(|u| u.category == UpdateCategory::Kernel)
            .collect()
    }

    /// Install specific update
    pub fn install_update(&mut self, package_name: &str) -> Result<(), String> {
        let update = self.available_updates.get(package_name)
            .ok_or_else(|| format!("Update {} not found", package_name))?;

        // Simulate installation
        self.installed_updates.push(package_name.to_string());
        self.available_updates.remove(package_name);

        Ok(())
    }

    /// Install all safety updates
    pub fn install_safety_updates(&mut self) -> usize {
        let safety_names: Vec<String> = self.available_updates.values()
            .filter(|u| u.level == UpdateLevel::Safety)
            .map(|u| u.name.clone())
            .collect();

        let count = safety_names.len();
        for name in safety_names {
            let _ = self.install_update(&name);
        }

        count
    }

    /// Install all security updates
    pub fn install_security_updates(&mut self) -> usize {
        let security_names: Vec<String> = self.available_updates.values()
            .filter(|u| u.is_security)
            .map(|u| u.name.clone())
            .collect();

        let count = security_names.len();
        for name in security_names {
            let _ = self.install_update(&name);
        }

        count
    }

    /// Install all updates
    pub fn install_all_updates(&mut self) -> usize {
        let all_names: Vec<String> = self.available_updates.keys()
            .cloned()
            .collect();

        let count = all_names.len();
        for name in all_names {
            let _ = self.install_update(&name);
        }

        count
    }

    /// Get statistics
    pub fn get_statistics(&self) -> UpdateStatistics {
        let total_size: u64 = self.available_updates.values()
            .map(|u| u.size)
            .sum();

        UpdateStatistics {
            total_updates: self.available_updates.len(),
            security_updates: self.security_count as usize,
            kernel_updates: self.get_kernel_updates().len(),
            safety_updates: self.get_updates_by_level(UpdateLevel::Safety).len(),
            recommended_updates: self.get_updates_by_level(UpdateLevel::Recommended).len(),
            feature_updates: self.get_updates_by_level(UpdateLevel::Feature).len(),
            total_size,
            blacklist_count: self.config.blacklist.len(),
            installed_count: self.installed_updates.len(),
        }
    }
}

impl Default for UpdateManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Update statistics
#[derive(Debug, Clone)]
pub struct UpdateStatistics {
    pub total_updates: usize,
    pub security_updates: usize,
    pub kernel_updates: usize,
    pub safety_updates: usize,
    pub recommended_updates: usize,
    pub feature_updates: usize,
    pub total_size: u64,
    pub blacklist_count: usize,
    pub installed_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_level_from_str() {
        assert_eq!(UpdateLevel::from_str("safety"), Some(UpdateLevel::Safety));
        assert_eq!(UpdateLevel::from_str("unstable"), Some(UpdateLevel::Unstable));
    }

    #[test]
    fn test_update_category_from_str() {
        assert_eq!(UpdateCategory::from_str("kernel"), Some(UpdateCategory::Kernel));
        assert_eq!(UpdateCategory::from_str("security"), Some(UpdateCategory::Security));
    }

    #[test]
    fn test_update_package_creation() {
        let pkg = UpdatePackage::new(
            "test".to_string(),
            "1.0".to_string(),
            "2.0".to_string(),
            UpdateLevel::Recommended,
            UpdateCategory::Bugfix,
        );
        assert_eq!(pkg.name, "test");
        assert_eq!(pkg.level, UpdateLevel::Recommended);
    }

    #[test]
    fn test_update_manager_creation() {
        let manager = UpdateManager::new();
        assert_eq!(manager.get_updates().len(), 0);
    }

    #[test]
    fn test_check_updates() {
        let mut manager = UpdateManager::new();
        let updates = manager.check_updates();
        assert!(updates.len() > 0);
    }

    #[test]
    fn test_install_update() {
        let mut manager = UpdateManager::new();
        manager.check_updates();
        assert!(manager.install_update("linux-kernel").is_ok());
    }

    #[test]
    fn test_install_safety_updates() {
        let mut manager = UpdateManager::new();
        manager.check_updates();
        let count = manager.install_safety_updates();
        assert!(count > 0);
    }

    #[test]
    fn test_blacklist() {
        let mut manager = UpdateManager::new();
        manager.add_blacklist("test".to_string());
        assert_eq!(manager.get_config().blacklist.len(), 1);
        manager.remove_blacklist("test");
        assert_eq!(manager.get_config().blacklist.len(), 0);
    }

    #[test]
    fn test_statistics() {
        let mut manager = UpdateManager::new();
        manager.check_updates();
        let stats = manager.get_statistics();
        assert!(stats.total_updates > 0);
    }
}
