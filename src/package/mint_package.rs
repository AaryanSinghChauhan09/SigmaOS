//! Linux Mint-inspired Package Management
//! 
//! This module implements package management features inspired by Linux Mint's
//! mintupdate and mintinstall, including update levels, multi-source support,
//! repository mirrors, and snapshot integration.

#![no_std]
#![allow(dead_code)]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Update level classification (from Linux Mint's Update Manager)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MintUpdateLevel {
    /// Security updates - critical and should be applied immediately
    Security = 3,
    /// Recommended updates - stable and tested updates
    Recommended = 2,
    /// Optional updates - new features or less critical fixes
    Optional = 1,
    /// Unsafe/Experimental updates - may cause instability
    Unsafe = 0,
}

impl MintUpdateLevel {
    /// Get display name for the update level
    pub fn display_name(&self) -> &'static str {
        match self {
            MintUpdateLevel::Security => "Security",
            MintUpdateLevel::Recommended => "Recommended",
            MintUpdateLevel::Optional => "Optional",
            MintUpdateLevel::Unsafe => "Unsafe",
        }
    }

    /// Get color code for terminal display
    pub fn color_code(&self) -> &'static str {
        match self {
            MintUpdateLevel::Security => "\x1b[31m", // Red
            MintUpdateLevel::Recommended => "\x1b[33m", // Yellow
            MintUpdateLevel::Optional => "\x1b[36m", // Cyan
            MintUpdateLevel::Unsafe => "\x1b[35m", // Magenta
        }
    }
}

/// Package source type (multi-source support like MintInstall)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MintPackageSource {
    /// Debian/Ubuntu APT repository
    Apt,
    /// Flatpak sandboxed applications
    Flatpak,
    /// Snap universal packages
    Snap,
    /// SigmaOS native packages
    SigmaPkg,
    /// Arch Linux AUR (via compatibility layer)
    Aur,
    /// AppImage portable applications
    AppImage,
}

impl MintPackageSource {
    /// Get display name for the source
    pub fn display_name(&self) -> &'static str {
        match self {
            MintPackageSource::Apt => "APT",
            MintPackageSource::Flatpak => "Flatpak",
            MintPackageSource::Snap => "Snap",
            MintPackageSource::SigmaPkg => "SigmaPkg",
            MintPackageSource::Aur => "AUR",
            MintPackageSource::AppImage => "AppImage",
        }
    }

    /// Check if source is sandboxed
    pub fn is_sandboxed(&self) -> bool {
        matches!(self, MintPackageSource::Flatpak | MintPackageSource::Snap)
    }
}

/// Package metadata with Mint-specific information
#[derive(Debug, Clone)]
pub struct MintPackageMetadata {
    /// Package name
    pub name: String,
    /// Package version
    pub version: String,
    /// Package description
    pub description: String,
    /// Package source
    pub source: MintPackageSource,
    /// Update level
    pub update_level: MintUpdateLevel,
    /// Package size in bytes
    pub size: u64,
    /// Installed size in bytes
    pub installed_size: u64,
    /// Whether package is currently installed
    pub installed: bool,
    /// Whether package is an update to an installed package
    pub is_update: bool,
    /// Repository name
    pub repository: String,
    /// Package license
    pub license: String,
    /// Package maintainer
    pub maintainer: String,
}

/// Repository mirror configuration (inspired by Mint's mirror management)
#[derive(Debug, Clone)]
pub struct MintRepositoryMirror {
    /// Mirror URL
    pub url: String,
    /// Mirror country code
    pub country: String,
    /// Mirror region
    pub region: String,
    /// Mirror latency in milliseconds
    pub latency_ms: u32,
    /// Mirror score (lower is better)
    pub score: u32,
    /// Whether mirror is currently active
    pub active: bool,
    /// Mirror protocol (http, https, ftp)
    pub protocol: String,
}

/// Mint Update Manager - manages system updates with level-based filtering
#[derive(Debug)]
pub struct MintUpdateManager {
    /// Available updates
    updates: Vec<MintPackageMetadata>,
    /// Configured update levels (which levels to show)
    enabled_levels: Vec<MintUpdateLevel>,
    /// Whether to auto-install security updates
    auto_security: bool,
    /// Whether to auto-install recommended updates
    auto_recommended: bool,
    /// Update check interval in seconds
    check_interval: u32,
    /// Last update check timestamp
    last_check: u64,
}

impl MintUpdateManager {
    /// Create a new Mint Update Manager
    pub fn new() -> Self {
        Self {
            updates: Vec::new(),
            enabled_levels: vec![
                MintUpdateLevel::Security,
                MintUpdateLevel::Recommended,
                MintUpdateLevel::Optional,
            ],
            auto_security: true,
            auto_recommended: false,
            check_interval: 3600, // 1 hour
            last_check: 0,
        }
    }

    /// Add an update to the manager
    pub fn add_update(&mut self, update: MintPackageMetadata) {
        self.updates.push(update);
    }

    /// Get updates filtered by enabled levels
    pub fn get_filtered_updates(&self) -> Vec<&MintPackageMetadata> {
        self.updates
            .iter()
            .filter(|pkg| self.enabled_levels.contains(&pkg.update_level))
            .collect()
    }

    /// Get security updates only
    pub fn get_security_updates(&self) -> Vec<&MintPackageMetadata> {
        self.updates
            .iter()
            .filter(|pkg| pkg.update_level == MintUpdateLevel::Security)
            .collect()
    }

    /// Get recommended updates only
    pub fn get_recommended_updates(&self) -> Vec<&MintPackageMetadata> {
        self.updates
            .iter()
            .filter(|pkg| pkg.update_level == MintUpdateLevel::Recommended)
            .collect()
    }

    /// Get total size of all pending updates
    pub fn get_total_update_size(&self) -> u64 {
        self.get_filtered_updates()
            .iter()
            .map(|pkg| pkg.size)
            .sum()
    }

    /// Set which update levels are enabled
    pub fn set_enabled_levels(&mut self, levels: Vec<MintUpdateLevel>) {
        self.enabled_levels = levels;
    }

    /// Enable auto-install of security updates
    pub fn set_auto_security(&mut self, enabled: bool) {
        self.auto_security = enabled;
    }

    /// Enable auto-install of recommended updates
    pub fn set_auto_recommended(&mut self, enabled: bool) {
        self.auto_recommended = enabled;
    }

    /// Set update check interval
    pub fn set_check_interval(&mut self, interval_seconds: u32) {
        self.check_interval = interval_seconds;
    }

    /// Check if auto-update should run
    pub fn should_auto_update(&self) -> bool {
        if self.auto_security {
            let security_count = self.get_security_updates().len();
            if security_count > 0 {
                return true;
            }
        }
        if self.auto_recommended {
            let recommended_count = self.get_recommended_updates().len();
            if recommended_count > 0 {
                return true;
            }
        }
        false
    }
}

impl Default for MintUpdateManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Mint Install Manager - software manager with multi-source support
#[derive(Debug)]
pub struct MintInstallManager {
    /// Available packages from all sources
    packages: Vec<MintPackageMetadata>,
    /// Enabled package sources
    enabled_sources: Vec<MintPackageSource>,
    /// Package search index (name -> package indices)
    search_index: BTreeMap<String, Vec<usize>>,
}

impl MintInstallManager {
    /// Create a new Mint Install Manager
    pub fn new() -> Self {
        Self {
            packages: Vec::new(),
            enabled_sources: vec![
                MintPackageSource::Apt,
                MintPackageSource::Flatpak,
                MintPackageSource::SigmaPkg,
            ],
            search_index: BTreeMap::new(),
        }
    }

    /// Add a package to the manager
    pub fn add_package(&mut self, package: MintPackageMetadata) {
        let idx = self.packages.len();
        self.packages.push(package.clone());
        
        // Update search index
        let name_lower = package.name.to_lowercase();
        self.search_index
            .entry(name_lower)
            .or_insert_with(Vec::new)
            .push(idx);
    }

    /// Search for packages by name
    pub fn search(&self, query: &str) -> Vec<&MintPackageMetadata> {
        let query_lower = query.to_lowercase();
        self.packages
            .iter()
            .filter(|pkg| pkg.name.to_lowercase().contains(&query_lower))
            .collect()
    }

    /// Get packages from a specific source
    pub fn get_packages_by_source(&self, source: MintPackageSource) -> Vec<&MintPackageMetadata> {
        self.packages
            .iter()
            .filter(|pkg| pkg.source == source)
            .collect()
    }

    /// Get installed packages
    pub fn get_installed_packages(&self) -> Vec<&MintPackageMetadata> {
        self.packages.iter().filter(|pkg| pkg.installed).collect()
    }

    /// Get available updates
    pub fn get_available_updates(&self) -> Vec<&MintPackageMetadata> {
        self.packages.iter().filter(|pkg| pkg.is_update).collect()
    }

    /// Enable or disable a package source
    pub fn set_source_enabled(&mut self, source: MintPackageSource, enabled: bool) {
        if enabled {
            if !self.enabled_sources.contains(&source) {
                self.enabled_sources.push(source);
            }
        } else {
            self.enabled_sources.retain(|s| s != &source);
        }
    }

    /// Check if a source is enabled
    pub fn is_source_enabled(&self, source: MintPackageSource) -> bool {
        self.enabled_sources.contains(&source)
    }
}

impl Default for MintInstallManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Repository Mirror Manager - manages repository mirrors
#[derive(Debug)]
pub struct MintMirrorManager {
    /// Available mirrors
    mirrors: Vec<MintRepositoryMirror>,
    /// Currently selected mirror
    active_mirror: Option<String>,
    /// Last mirror refresh timestamp
    last_refresh: u64,
}

impl MintMirrorManager {
    /// Create a new Mirror Manager
    pub fn new() -> Self {
        Self {
            mirrors: Vec::new(),
            active_mirror: None,
            last_refresh: 0,
        }
    }

    /// Add a mirror
    pub fn add_mirror(&mut self, mirror: MintRepositoryMirror) {
        self.mirrors.push(mirror);
    }

    /// Get the best mirror (lowest score)
    pub fn get_best_mirror(&self) -> Option<&MintRepositoryMirror> {
        self.mirrors
            .iter()
            .filter(|m| m.active)
            .min_by_key(|m| m.score)
    }

    /// Get mirrors by country
    pub fn get_mirrors_by_country(&self, country: &str) -> Vec<&MintRepositoryMirror> {
        self.mirrors
            .iter()
            .filter(|m| m.country == country)
            .collect()
    }

    /// Set active mirror by URL
    pub fn set_active_mirror(&mut self, url: String) {
        self.active_mirror = Some(url.clone());
        // Update active flags
        for mirror in &mut self.mirrors {
            mirror.active = mirror.url == url;
        }
    }

    /// Auto-select best mirror based on latency
    pub fn auto_select_best_mirror(&mut self) {
        if let Some(best) = self.get_best_mirror() {
            self.set_active_mirror(best.url.clone());
        }
    }
}

impl Default for MintMirrorManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Snapshot Integration - Timeshift-like snapshot management
#[derive(Debug, Clone)]
pub struct MintSnapshotConfig {
    /// Whether snapshots are enabled
    pub enabled: bool,
    /// Snapshot location
    pub location: String,
    /// Snapshot schedule (hourly, daily, weekly, monthly)
    pub schedule: String,
    /// Maximum number of snapshots to keep
    pub max_snapshots: u32,
    /// Whether to take snapshot before updates
    pub snapshot_before_update: bool,
}

impl Default for MintSnapshotConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            location: "/var/lib/sigmaos/snapshots".to_string(),
            schedule: "daily".to_string(),
            max_snapshots: 30,
            snapshot_before_update: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_level_ordering() {
        assert!(MintUpdateLevel::Security > MintUpdateLevel::Recommended);
        assert!(MintUpdateLevel::Recommended > MintUpdateLevel::Optional);
        assert!(MintUpdateLevel::Optional > MintUpdateLevel::Unsafe);
    }

    #[test]
    fn test_update_manager_filtering() {
        let mut manager = MintUpdateManager::new();
        
        manager.add_update(MintPackageMetadata {
            name: "security-pkg".to_string(),
            version: "1.0.0".to_string(),
            description: "Security fix".to_string(),
            source: MintPackageSource::Apt,
            update_level: MintUpdateLevel::Security,
            size: 1024,
            installed_size: 2048,
            installed: false,
            is_update: true,
            repository: "main".to_string(),
            license: "GPL".to_string(),
            maintainer: "SigmaOS".to_string(),
        });

        manager.add_update(MintPackageMetadata {
            name: "optional-pkg".to_string(),
            version: "2.0.0".to_string(),
            description: "Optional feature".to_string(),
            source: MintPackageSource::Flatpak,
            update_level: MintUpdateLevel::Optional,
            size: 5120,
            installed_size: 10240,
            installed: false,
            is_update: true,
            repository: "community".to_string(),
            license: "MIT".to_string(),
            maintainer: "Community".to_string(),
        });

        let filtered = manager.get_filtered_updates();
        assert_eq!(filtered.len(), 2);

        let security = manager.get_security_updates();
        assert_eq!(security.len(), 1);
        assert_eq!(security[0].name, "security-pkg");
    }

    #[test]
    fn test_install_manager_search() {
        let mut manager = MintInstallManager::new();
        
        manager.add_package(MintPackageMetadata {
            name: "test-package".to_string(),
            version: "1.0.0".to_string(),
            description: "Test package".to_string(),
            source: MintPackageSource::SigmaPkg,
            update_level: MintUpdateLevel::Recommended,
            size: 1024,
            installed_size: 2048,
            installed: true,
            is_update: false,
            repository: "main".to_string(),
            license: "GPL".to_string(),
            maintainer: "SigmaOS".to_string(),
        });

        let results = manager.search("test");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "test-package");
    }

    #[test]
    fn test_package_source_sandboxed() {
        assert!(MintPackageSource::Flatpak.is_sandboxed());
        assert!(MintPackageSource::Snap.is_sandboxed());
        assert!(!MintPackageSource::Apt.is_sandboxed());
        assert!(!MintPackageSource::SigmaPkg.is_sandboxed());
    }
}
