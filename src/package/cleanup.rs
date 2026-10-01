// Package Cleanup Manager for SigmaPkg
// Package cleanup operations per Wiki 09-Packaging.md
// Provides autoremove, clean, and old version removal

use std::string::{String, ToString};
use std::vec::Vec;

/// Package cleanup operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CleanupOperation {
    Autoremove,
    CleanCache,
    RemoveOldVersions,
    RemoveOrphans,
    PurgeConfig,
}

impl CleanupOperation {
    pub fn as_str(&self) -> &str {
        match self {
            CleanupOperation::Autoremove => "autoremove",
            CleanupOperation::CleanCache => "clean",
            CleanupOperation::RemoveOldVersions => "remove-old-versions",
            CleanupOperation::RemoveOrphans => "remove-orphans",
            CleanupOperation::PurgeConfig => "purge-config",
        }
    }
}

/// Package cleanup result
#[derive(Debug, Clone)]
pub struct CleanupResult {
    pub operation: CleanupOperation,
    pub packages_removed: Vec<String>,
    pub space_freed: u64, // in bytes
    pub config_files_removed: Vec<String>,
    pub errors: Vec<String>,
}

impl CleanupResult {
    pub fn new(operation: CleanupOperation) -> Self {
        CleanupResult {
            operation,
            packages_removed: Vec::new(),
            space_freed: 0,
            config_files_removed: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn get_summary(&self) -> String {
        let mut summary = format!("Cleanup operation: {}\n", self.operation.as_str());
        summary.push_str(&format!(
            "Packages removed: {}\n",
            self.packages_removed.len()
        ));
        summary.push_str(&format!("Space freed: {} bytes\n", self.space_freed));
        summary.push_str(&format!(
            "Config files removed: {}\n",
            self.config_files_removed.len()
        ));
        summary.push_str(&format!("Errors: {}\n", self.errors.len()));
        summary
    }
}

/// Orphan package (dependency no longer needed)
#[derive(Debug, Clone)]
pub struct OrphanPackage {
    pub name: String,
    pub version: String,
    pub size: u64,
    pub install_time: u64,
}

impl OrphanPackage {
    pub fn new(name: String, version: String, size: u64) -> Self {
        OrphanPackage {
            name,
            version,
            size,
            install_time: 0,
        }
    }
}

/// Cached package
#[derive(Debug, Clone)]
pub struct CachedPackage {
    pub name: String,
    pub version: String,
    pub file_path: String,
    pub size: u64,
    pub last_accessed: u64,
}

impl CachedPackage {
    pub fn new(name: String, version: String, file_path: String, size: u64) -> Self {
        CachedPackage {
            name,
            version,
            file_path,
            size,
            last_accessed: 0,
        }
    }
}

/// Old package version
#[derive(Debug, Clone)]
pub struct OldPackageVersion {
    pub name: String,
    pub old_version: String,
    pub current_version: String,
    pub size: u64,
}

impl OldPackageVersion {
    pub fn new(name: String, old_version: String, current_version: String, size: u64) -> Self {
        OldPackageVersion {
            name,
            old_version,
            current_version,
            size,
        }
    }
}

/// Package cleanup manager
#[derive(Debug, Clone)]
pub struct PackageCleanupManager {
    pub keep_old_versions: u32,
    pub auto_cleanup_enabled: bool,
    pub orphans: Vec<OrphanPackage>,
    pub cached_packages: Vec<CachedPackage>,
    pub old_versions: Vec<OldPackageVersion>,
}

impl Default for PackageCleanupManager {
    fn default() -> Self {
        PackageCleanupManager {
            keep_old_versions: 3,
            auto_cleanup_enabled: false,
            orphans: Vec::new(),
            cached_packages: Vec::new(),
            old_versions: Vec::new(),
        }
    }
}

impl PackageCleanupManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_keep_old_versions(&mut self, count: u32) {
        self.keep_old_versions = count;
    }

    pub fn set_auto_cleanup(&mut self, enabled: bool) {
        self.auto_cleanup_enabled = enabled;
    }

    pub fn add_orphan(&mut self, orphan: OrphanPackage) {
        self.orphans.push(orphan);
    }

    pub fn add_cached_package(&mut self, cached: CachedPackage) {
        self.cached_packages.push(cached);
    }

    pub fn add_old_version(&mut self, old: OldPackageVersion) {
        self.old_versions.push(old);
    }

    pub fn find_orphans(&self) -> Vec<OrphanPackage> {
        self.orphans.clone()
    }

    pub fn find_cached_packages(&self) -> Vec<CachedPackage> {
        self.cached_packages.clone()
    }

    pub fn find_old_versions(&self) -> Vec<OldPackageVersion> {
        self.old_versions.clone()
    }

    pub fn autoremove(&mut self) -> CleanupResult {
        let mut result = CleanupResult::new(CleanupOperation::Autoremove);

        for orphan in &self.orphans {
            result.packages_removed.push(orphan.name.clone());
            result.space_freed += orphan.size;
        }

        self.orphans.clear();
        result
    }

    pub fn clean_cache(&mut self) -> CleanupResult {
        let mut result = CleanupResult::new(CleanupOperation::CleanCache);

        for cached in &self.cached_packages {
            result
                .packages_removed
                .push(format!("{}@{}", cached.name, cached.version));
            result.space_freed += cached.size;
        }

        self.cached_packages.clear();
        result
    }

    pub fn remove_old_versions(&mut self) -> CleanupResult {
        let mut result = CleanupResult::new(CleanupOperation::RemoveOldVersions);

        for old in &self.old_versions {
            result
                .packages_removed
                .push(format!("{}@{}", old.name, old.old_version));
            result.space_freed += old.size;
        }

        self.old_versions.clear();
        result
    }

    pub fn remove_orphans(&mut self) -> CleanupResult {
        let mut result = CleanupResult::new(CleanupOperation::RemoveOrphans);

        for orphan in &self.orphans {
            result.packages_removed.push(orphan.name.clone());
            result.space_freed += orphan.size;
        }

        self.orphans.clear();
        result
    }

    pub fn purge_config(&mut self, package_names: Vec<String>) -> CleanupResult {
        let mut result = CleanupResult::new(CleanupOperation::PurgeConfig);

        for name in &package_names {
            result
                .config_files_removed
                .push(format!("/etc/{}.conf", name));
            result
                .config_files_removed
                .push(format!("/var/lib/{}.db", name));
        }

        result.packages_removed = package_names;
        result
    }

    pub fn cleanup_all(&mut self) -> CleanupResult {
        let mut total_space = 0u64;
        let mut all_packages = Vec::new();
        let mut all_errors = Vec::new();

        let autoremove_result = self.autoremove();
        total_space += autoremove_result.space_freed;
        all_packages.extend(autoremove_result.packages_removed);
        all_errors.extend(autoremove_result.errors);

        let clean_result = self.clean_cache();
        total_space += clean_result.space_freed;
        all_packages.extend(clean_result.packages_removed);
        all_errors.extend(clean_result.errors);

        let old_versions_result = self.remove_old_versions();
        total_space += old_versions_result.space_freed;
        all_packages.extend(old_versions_result.packages_removed);
        all_errors.extend(old_versions_result.errors);

        CleanupResult {
            operation: CleanupOperation::CleanCache,
            packages_removed: all_packages,
            space_freed: total_space,
            config_files_removed: Vec::new(),
            errors: all_errors,
        }
    }

    pub fn get_statistics(&self) -> String {
        let mut stats = String::from("Package Cleanup Statistics\n");
        stats.push_str(&format!("Orphan packages: {}\n", self.orphans.len()));
        stats.push_str(&format!(
            "Cached packages: {}\n",
            self.cached_packages.len()
        ));
        stats.push_str(&format!("Old versions: {}\n", self.old_versions.len()));

        let orphan_size: u64 = self.orphans.iter().map(|o| o.size).sum();
        let cache_size: u64 = self.cached_packages.iter().map(|c| c.size).sum();
        let old_size: u64 = self.old_versions.iter().map(|o| o.size).sum();

        stats.push_str(&format!("Orphan size: {} bytes\n", orphan_size));
        stats.push_str(&format!("Cache size: {} bytes\n", cache_size));
        stats.push_str(&format!("Old versions size: {} bytes\n", old_size));
        stats.push_str(&format!(
            "Total cleanable: {} bytes\n",
            orphan_size + cache_size + old_size
        ));
        stats.push_str(&format!("Keep old versions: {}\n", self.keep_old_versions));
        stats.push_str(&format!("Auto cleanup: {}\n", self.auto_cleanup_enabled));

        stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cleanup_operation_as_str() {
        assert_eq!(CleanupOperation::Autoremove.as_str(), "autoremove");
        assert_eq!(CleanupOperation::CleanCache.as_str(), "clean");
        assert_eq!(
            CleanupOperation::RemoveOldVersions.as_str(),
            "remove-old-versions"
        );
    }

    #[test]
    fn test_cleanup_result_creation() {
        let result = CleanupResult::new(CleanupOperation::Autoremove);
        assert_eq!(result.operation, CleanupOperation::Autoremove);
        assert_eq!(result.packages_removed.len(), 0);
        assert_eq!(result.space_freed, 0);
    }

    #[test]
    fn test_cleanup_result_get_summary() {
        let mut result = CleanupResult::new(CleanupOperation::Autoremove);
        result.packages_removed.push(String::from("package1"));
        result.space_freed = 1024;

        let summary = result.get_summary();
        assert!(summary.contains("autoremove"));
        assert!(summary.contains("Packages removed: 1"));
        assert!(summary.contains("Space freed: 1024 bytes"));
    }

    #[test]
    fn test_orphan_package_creation() {
        let orphan = OrphanPackage::new(String::from("test"), String::from("1.0.0"), 1024);
        assert_eq!(orphan.name, "test");
        assert_eq!(orphan.version, "1.0.0");
        assert_eq!(orphan.size, 1024);
    }

    #[test]
    fn test_cached_package_creation() {
        let cached = CachedPackage::new(
            String::from("test"),
            String::from("1.0.0"),
            String::from("/cache/test-1.0.0.sigpkg"),
            2048,
        );
        assert_eq!(cached.name, "test");
        assert_eq!(cached.file_path, "/cache/test-1.0.0.sigpkg");
        assert_eq!(cached.size, 2048);
    }

    #[test]
    fn test_old_package_version_creation() {
        let old = OldPackageVersion::new(
            String::from("test"),
            String::from("1.0.0"),
            String::from("2.0.0"),
            1024,
        );
        assert_eq!(old.name, "test");
        assert_eq!(old.old_version, "1.0.0");
        assert_eq!(old.current_version, "2.0.0");
    }

    #[test]
    fn test_package_cleanup_manager_creation() {
        let manager = PackageCleanupManager::new();
        assert_eq!(manager.keep_old_versions, 3);
        assert!(!manager.auto_cleanup_enabled);
        assert_eq!(manager.orphans.len(), 0);
    }

    #[test]
    fn test_package_cleanup_manager_set_keep_old_versions() {
        let mut manager = PackageCleanupManager::new();
        manager.set_keep_old_versions(5);
        assert_eq!(manager.keep_old_versions, 5);
    }

    #[test]
    fn test_package_cleanup_manager_set_auto_cleanup() {
        let mut manager = PackageCleanupManager::new();
        manager.set_auto_cleanup(true);
        assert!(manager.auto_cleanup_enabled);
    }

    #[test]
    fn test_package_cleanup_manager_add_orphan() {
        let mut manager = PackageCleanupManager::new();
        manager.add_orphan(OrphanPackage::new(
            String::from("test"),
            String::from("1.0.0"),
            1024,
        ));
        assert_eq!(manager.orphans.len(), 1);
    }

    #[test]
    fn test_package_cleanup_manager_add_cached_package() {
        let mut manager = PackageCleanupManager::new();
        manager.add_cached_package(CachedPackage::new(
            String::from("test"),
            String::from("1.0.0"),
            String::from("/cache/test.sigpkg"),
            2048,
        ));
        assert_eq!(manager.cached_packages.len(), 1);
    }

    #[test]
    fn test_package_cleanup_manager_add_old_version() {
        let mut manager = PackageCleanupManager::new();
        manager.add_old_version(OldPackageVersion::new(
            String::from("test"),
            String::from("1.0.0"),
            String::from("2.0.0"),
            1024,
        ));
        assert_eq!(manager.old_versions.len(), 1);
    }

    #[test]
    fn test_package_cleanup_manager_autoremove() {
        let mut manager = PackageCleanupManager::new();
        manager.add_orphan(OrphanPackage::new(
            String::from("test1"),
            String::from("1.0.0"),
            1024,
        ));
        manager.add_orphan(OrphanPackage::new(
            String::from("test2"),
            String::from("1.0.0"),
            2048,
        ));
        manager.add_orphan(OrphanPackage::new(String::from("test1"), String::from("1.0.0"), 1024));
        manager.add_orphan(OrphanPackage::new(String::from("test2"), String::from("1.0.0"), 2048));

        let result = manager.autoremove();
        assert_eq!(result.packages_removed.len(), 2);
        assert_eq!(result.space_freed, 3072);
        assert_eq!(manager.orphans.len(), 0);
    }

    #[test]
    fn test_package_cleanup_manager_clean_cache() {
        let mut manager = PackageCleanupManager::new();
        manager.add_cached_package(CachedPackage::new(
            String::from("test1"),
            String::from("1.0.0"),
            String::from("/cache/test1.sigpkg"),
            1024,
        ));
        manager.add_cached_package(CachedPackage::new(
            String::from("test2"),
            String::from("1.0.0"),
            String::from("/cache/test2.sigpkg"),
            2048,
        ));

        let result = manager.clean_cache();
        assert_eq!(result.packages_removed.len(), 2);
        assert_eq!(result.space_freed, 3072);
        assert_eq!(manager.cached_packages.len(), 0);
    }

    #[test]
    fn test_package_cleanup_manager_remove_old_versions() {
        let mut manager = PackageCleanupManager::new();
        manager.add_old_version(OldPackageVersion::new(
            String::from("test1"),
            String::from("1.0.0"),
            String::from("2.0.0"),
            1024,
        ));
        manager.add_old_version(OldPackageVersion::new(
            String::from("test2"),
            String::from("1.5.0"),
            String::from("2.0.0"),
            2048,
        ));

        let result = manager.remove_old_versions();
        assert_eq!(result.packages_removed.len(), 2);
        assert_eq!(result.space_freed, 3072);
        assert_eq!(manager.old_versions.len(), 0);
    }

    #[test]
    fn test_package_cleanup_manager_remove_orphans() {
        let mut manager = PackageCleanupManager::new();
        manager.add_orphan(OrphanPackage::new(
            String::from("test1"),
            String::from("1.0.0"),
            1024,
        ));
        manager.add_orphan(OrphanPackage::new(String::from("test1"), String::from("1.0.0"), 1024));

        let result = manager.remove_orphans();
        assert_eq!(result.packages_removed.len(), 1);
        assert_eq!(result.space_freed, 1024);
        assert_eq!(manager.orphans.len(), 0);
    }

    #[test]
    fn test_package_cleanup_manager_purge_config() {
        let mut manager = PackageCleanupManager::new();
        let packages = vec![String::from("test1"), String::from("test2")];

        let result = manager.purge_config(packages.clone());
        assert_eq!(result.packages_removed.len(), 2);
        assert_eq!(result.config_files_removed.len(), 4);
    }

    #[test]
    fn test_package_cleanup_manager_cleanup_all() {
        let mut manager = PackageCleanupManager::new();
        manager.add_orphan(OrphanPackage::new(
            String::from("test1"),
            String::from("1.0.0"),
            1024,
        ));
        manager.add_cached_package(CachedPackage::new(
            String::from("test2"),
            String::from("1.0.0"),
            String::from("/cache/test2.sigpkg"),
            2048,
        ));
        manager.add_old_version(OldPackageVersion::new(
            String::from("test3"),
            String::from("1.0.0"),
            String::from("2.0.0"),
            4096,
        ));

        let result = manager.cleanup_all();
        assert_eq!(result.packages_removed.len(), 3);
        assert_eq!(result.space_freed, 7168);
        assert_eq!(manager.orphans.len(), 0);
        assert_eq!(manager.cached_packages.len(), 0);
        assert_eq!(manager.old_versions.len(), 0);
    }

    #[test]
    fn test_package_cleanup_manager_get_statistics() {
        let mut manager = PackageCleanupManager::new();
        manager.add_orphan(OrphanPackage::new(
            String::from("test1"),
            String::from("1.0.0"),
            1024,
        ));
        manager.add_cached_package(CachedPackage::new(
            String::from("test2"),
            String::from("1.0.0"),
            String::from("/cache/test2.sigpkg"),
            2048,
        ));
        manager.add_old_version(OldPackageVersion::new(
            String::from("test3"),
            String::from("1.0.0"),
            String::from("2.0.0"),
            4096,
        ));

        let stats = manager.get_statistics();
        assert!(stats.contains("Orphan packages: 1"));
        assert!(stats.contains("Cached packages: 1"));
        assert!(stats.contains("Old versions: 1"));
        assert!(stats.contains("Orphan size: 1024 bytes"));
        assert!(stats.contains("Cache size: 2048 bytes"));
        assert!(stats.contains("Old versions size: 4096 bytes"));
        assert!(stats.contains("Total cleanable: 7168 bytes"));
    }
}
