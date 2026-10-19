//! SigmaPackageManager - Package Management System
//!
//! Provides comprehensive package management inspired by Arch (pacman), Debian (apt), and CachyOS.
//! Supports package installation, removal, updates, repositories, and dependency resolution.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Package status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageStatus {
    Installed,
    NotInstalled,
    UpdateAvailable,
    Broken,
}

/// Package type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageType {
    Binary,
    Source,
    Meta,
}

/// Repository
#[derive(Debug, Clone)]
pub struct Repository {
    pub id: String,
    pub name: String,
    pub url: String,
    pub enabled: bool,
    pub priority: u32,
}

impl Repository {
    pub fn new(id: String, name: String, url: String, priority: u32) -> Self {
        Self {
            id,
            name,
            url,
            enabled: true,
            priority,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

/// Package
#[derive(Debug, Clone)]
pub struct Package {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub package_type: PackageType,
    pub status: PackageStatus,
    pub installed_version: Option<String>,
    pub latest_version: String,
    pub size: u64, // in bytes
    pub dependencies: Vec<String>,
    pub repository: String,
}

impl Package {
    pub fn new(
        id: String,
        name: String,
        version: String,
        description: String,
        package_type: PackageType,
        repository: String,
    ) -> Self {
        Self {
            id,
            name,
            version,
            description,
            package_type,
            status: PackageStatus::NotInstalled,
            installed_version: None,
            latest_version: version,
            size: 0,
            dependencies: Vec::new(),
            repository,
        }
    }

    pub fn set_status(&mut self, status: PackageStatus) {
        self.status = status;
    }

    pub fn set_installed_version(&mut self, version: Option<String>) {
        self.installed_version = version;
    }

    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }

    pub fn add_dependency(&mut self, dep: String) {
        self.dependencies.push(dep);
    }

    pub fn needs_update(&self) -> bool {
        self.status == PackageStatus::UpdateAvailable
    }

    pub fn is_installed(&self) -> bool {
        self.status == PackageStatus::Installed || self.status == PackageStatus::UpdateAvailable
    }
}

/// SigmaPackageManager - Package management
#[derive(Debug)]
pub struct SigmaPackageManager {
    packages: BTreeMap<String, Package>,
    repositories: BTreeMap<String, Repository>,
}

impl SigmaPackageManager {
    pub fn new() -> Self {
        Self {
            packages: BTreeMap::new(),
            repositories: BTreeMap::new(),
        }
    }

    pub fn add_package(&mut self, package: Package) {
        self.packages.insert(package.id.clone(), package);
    }

    pub fn remove_package(&mut self, id: &str) -> bool {
        self.packages.remove(id).is_some()
    }

    pub fn get_package(&self, id: &str) -> Option<&Package> {
        self.packages.get(id)
    }

    pub fn get_package_mut(&mut self, id: &str) -> Option<&mut Package> {
        self.packages.get_mut(id)
    }

    pub fn add_repository(&mut self, repo: Repository) {
        self.repositories.insert(repo.id.clone(), repo);
    }

    pub fn remove_repository(&mut self, id: &str) -> bool {
        self.repositories.remove(id).is_some()
    }

    pub fn get_repository(&self, id: &str) -> Option<&Repository> {
        self.repositories.get(id)
    }

    pub fn search_packages(&self, query: &str) -> Vec<&Package> {
        let query_lower = query.to_lowercase();
        self.packages
            .values()
            .filter(|pkg| {
                pkg.name.to_lowercase().contains(&query_lower)
                    || pkg.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    pub fn list_installed(&self) -> Vec<&Package> {
        self.packages
            .values()
            .filter(|pkg| pkg.is_installed())
            .collect()
    }

    pub fn list_updates(&self) -> Vec<&Package> {
        self.packages
            .values()
            .filter(|pkg| pkg.needs_update())
            .collect()
    }

    pub fn list_by_repository(&self, repo_name: &str) -> Vec<&Package> {
        self.packages
            .values()
            .filter(|pkg| pkg.repository == repo_name)
            .collect()
    }

    pub fn list_enabled_repositories(&self) -> Vec<&Repository> {
        self.repositories
            .values()
            .filter(|repo| repo.enabled)
            .collect()
    }

    pub fn total_download_size(&self, package_ids: &[String]) -> u64 {
        package_ids
            .iter()
            .filter_map(|id| self.packages.get(id))
            .map(|pkg| pkg.size)
            .sum()
    }

    pub fn package_count(&self) -> usize {
        self.packages.len()
    }

    pub fn repository_count(&self) -> usize {
        self.repositories.len()
    }
}

impl Default for SigmaPackageManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaPackageManager instance
static mut SIGMAPACKAGEMANAGER_INSTANCE: Option<SigmaPackageManager> = None;

/// Initialize the global SigmaPackageManager instance
pub fn init_sigmapackagemanager() {
    unsafe {
        SIGMAPACKAGEMANAGER_INSTANCE = Some(SigmaPackageManager::new());
    }
}

/// Get the global SigmaPackageManager instance
pub fn get_sigmapackagemanager() -> Option<&'static SigmaPackageManager> {
    unsafe { SIGMAPACKAGEMANAGER_INSTANCE.as_ref() }
}

/// Get the global SigmaPackageManager instance (mutable)
pub fn get_sigmapackagemanager_mut() -> Option<&'static mut SigmaPackageManager> {
    unsafe { SIGMAPACKAGEMANAGER_INSTANCE.as_mut() }
}

/// Convenience function to add a package
pub fn add_package(package: Package) -> bool {
    if let Some(manager) = get_sigmapackagemanager_mut() {
        manager.add_package(package);
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_creation() {
        let repo = Repository::new(
            "core".to_string(),
            "Core".to_string(),
            "https://mirror.example.com/core".to_string(),
            1,
        );
        assert_eq!(repo.id, "core");
        assert!(repo.enabled);
    }

    #[test]
    fn test_repository_modification() {
        let mut repo = Repository::new(
            "extra".to_string(),
            "Extra".to_string(),
            "https://mirror.example.com/extra".to_string(),
            2,
        );
        repo.set_enabled(false);
        assert!(!repo.enabled);
    }

    #[test]
    fn test_package_creation() {
        let pkg = Package::new(
            "pkg1".to_string(),
            "bash".to_string(),
            "5.1.016".to_string(),
            "GNU Bourne Again shell".to_string(),
            PackageType::Binary,
            "core".to_string(),
        );
        assert_eq!(pkg.id, "pkg1");
        assert_eq!(pkg.status, PackageStatus::NotInstalled);
    }

    #[test]
    fn test_package_modification() {
        let mut pkg = Package::new(
            "pkg1".to_string(),
            "test".to_string(),
            "1.0".to_string(),
            "Test package".to_string(),
            PackageType::Binary,
            "extra".to_string(),
        );
        pkg.set_status(PackageStatus::Installed);
        pkg.set_installed_version(Some("1.0".to_string()));
        pkg.add_dependency("dep1".to_string());
        assert!(pkg.is_installed());
        assert_eq!(pkg.dependencies.len(), 1);
    }

    #[test]
    fn test_sigmapackagemanager_creation() {
        let manager = SigmaPackageManager::new();
        assert_eq!(manager.package_count(), 0);
        assert_eq!(manager.repository_count(), 0);
    }

    #[test]
    fn test_sigmapackagemanager_management() {
        let mut manager = SigmaPackageManager::new();
        let pkg = Package::new(
            "pkg1".to_string(),
            "test".to_string(),
            "1.0".to_string(),
            "Test".to_string(),
            PackageType::Binary,
            "core".to_string(),
        );
        manager.add_package(pkg);
        assert_eq!(manager.package_count(), 1);
    }

    #[test]
    fn test_package_search() {
        let mut manager = SigmaPackageManager::new();
        let pkg = Package::new(
            "pkg1".to_string(),
            "bash".to_string(),
            "5.1".to_string(),
            "GNU Bourne Again shell".to_string(),
            PackageType::Binary,
            "core".to_string(),
        );
        manager.add_package(pkg);
        let results = manager.search_packages("shell");
        assert_eq!(results.len(), 1);
    }
}
