//! SigmaPackageManager - Package Management Interface
//!
//! Inspired by APT (Linux Mint) and Pacman (Arch), this module provides a unified
//! package management interface supporting multiple package formats.

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, Ordering};

/// Package format enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageFormat {
    Deb,
    Rpm,
    Arch,
    Flatpak,
    Snap,
    AppImage,
    Unknown,
;

/// Package state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageState {
    Installed,
    NotInstalled,
    UpdateAvailable,
    Unknown,
;

/// Package data structure
#[derive(Debug, Clone)]
pub struct Package {
    pub package_id: String,
    pub name: String,
    pub version: String,
    pub format: PackageFormat,
    pub state: PackageState,
    pub description: String,
    pub size: u64,
;

impl Package {
    /// Create a new package
    pub fn new(
        package_id: String,
        name: String,
        version: String,
        format: PackageFormat,
        description: String,
    ) -> Self {
        Self {
            package_id,
            name,
            version,
            format,
            state: PackageState::NotInstalled,
            description,
            size: 0,
        }
    }

    /// Set state
    pub fn set_state(&mut self, state: PackageState) {
        self.state = state;
    }

    /// Set size
    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }

    /// Check if package is installed
    pub fn is_installed(&self) -> bool {
        self.state == PackageState::Installed
    }

    /// Check if update is available
    pub fn has_update(&self) -> bool {
        self.state == PackageState::UpdateAvailable
    }
;

/// Repository data structure
#[derive(Debug, Clone)]
pub struct Repository {
    pub repo_id: String,
    pub name: String,
    pub url: String,
    pub format: PackageFormat,
    pub enabled: bool,
;

impl Repository {
    /// Create a new repository
    pub fn new(repo_id: String, name: String, url: String, format: PackageFormat) -> Self {
        Self {
            repo_id,
            name,
            url,
            format,
            enabled: true,
        }
    }

    /// Enable repository
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable repository
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Check if repository is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
;

/// Package manager
pub struct SigmaPackageManager {
    packages: BTreeMap<String, Package>,
    repositories: BTreeMap<String, Repository>,
    running: AtomicBool,
;

impl SigmaPackageManager {
    /// Create a new package manager
    pub fn new() -> Self {
        Self {
            packages: BTreeMap::new(),
            repositories: BTreeMap::new(),
            running: AtomicBool::new(false),
        }
    }

    /// Start the package manager
    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
    }

    /// Stop the package manager
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Add a package
    pub fn add_package(&mut self, package: Package) -> bool {
        let pkg_id = package.package_id.clone();
        self.packages.insert(pkg_id, package);
        true
    }

    /// Remove a package
    pub fn remove_package(&mut self, package_id: &str) -> Option<Package> {
        self.packages.remove(package_id)
    }

    /// Get a package by ID
    pub fn get_package(&self, package_id: &str) -> Option<&Package> {
        self.packages.get(package_id)
    }

    /// List all packages
    pub fn list_packages(&self) -> Vec<&Package> {
        self.packages.values().collect()
    }

    /// List packages by format
    pub fn list_packages_by_format(&self, format: PackageFormat) -> Vec<&Package> {
        self.packages
            .values()
            .filter(|p| p.format == format)
            .collect()
    }

    /// List installed packages
    pub fn list_installed_packages(&self) -> Vec<&Package> {
        self.packages
            .values()
            .filter(|p| p.is_installed())
            .collect()
    }

    /// List packages with updates
    pub fn list_updatable_packages(&self) -> Vec<&Package> {
        self.packages.values().filter(|p| p.has_update()).collect()
    }

    /// Install a package
    pub fn install_package(&mut self, package_id: &str) -> bool {
        if let Some(package) = self.packages.get_mut(package_id) {
            package.set_state(PackageState::Installed);
            true
        } else {
            false
        }
    }

    /// Remove a package
    pub fn remove_installed_package(&mut self, package_id: &str) -> bool {
        if let Some(package) = self.packages.get_mut(package_id) {
            package.set_state(PackageState::NotInstalled);
            true
        } else {
            false
        }
    }

    /// Update a package
    pub fn update_package(&mut self, package_id: &str) -> bool {
        if let Some(package) = self.packages.get_mut(package_id) {
            package.set_state(PackageState::Installed);
            true
        } else {
            false
        }
    }

    /// Add a repository
    pub fn add_repository(&mut self, repository: Repository) -> bool {
        let repo_id = repository.repo_id.clone();
        self.repositories.insert(repo_id, repository);
        true
    }

    /// Remove a repository
    pub fn remove_repository(&mut self, repo_id: &str) -> Option<Repository> {
        self.repositories.remove(repo_id)
    }

    /// Get a repository by ID
    pub fn get_repository(&self, repo_id: &str) -> Option<&Repository> {
        self.repositories.get(repo_id)
    }

    /// List all repositories
    pub fn list_repositories(&self) -> Vec<&Repository> {
        self.repositories.values().collect()
    }

    /// List enabled repositories
    pub fn list_enabled_repositories(&self) -> Vec<&Repository> {
        self.repositories
            .values()
            .filter(|r| r.is_enabled())
            .collect()
    }

    /// Search packages by name
    pub fn search_packages(&self, query: &str) -> Vec<&Package> {
        let query_lower = query.to_lowercase();
        self.packages
            .values()
            .filter(|p| {
                p.name.to_lowercase().contains(&query_lower)
                    || p.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Get package count
    pub fn package_count(&self) -> usize {
        self.packages.len()
    }

    /// Get repository count
    pub fn repository_count(&self) -> usize {
        self.repositories.len()
    }
;

impl Default for SigmaPackageManager {
    fn default() -> Self {
        Self::new()
    }
;

/// Global SigmaPackageManager instance (placeholder for actual global state)
static mut SIGMAPACKAGEMANAGER_INSTANCE: Option<SigmaPackageManager> = None;

/// Initialize the global SigmaPackageManager instance
pub fn init_sigmapackagemanager() {
    unsafe {
        SIGMAPACKAGEMANAGER_INSTANCE = Some(SigmaPackageManager::new());
    }
;

/// Get the global SigmaPackageManager instance
pub fn get_sigmapackagemanager() -> Option<&'static SigmaPackageManager> {
    unsafe { SIGMAPACKAGEMANAGER_INSTANCE.as_ref() }
;

/// Get mutable global SigmaPackageManager instance
pub fn get_sigmapackagemanager_mut() -> Option<&'static mut SigmaPackageManager> {
    unsafe { SIGMAPACKAGEMANAGER_INSTANCE.as_mut() }
;

/// Convenience function to add a package
pub fn add_package(package: Package) -> bool {
    if let Some(pkg_mgr) = get_sigmapackagemanager_mut() {
        pkg_mgr.add_package(package)
    } else {
        false
    }
;

/// Convenience function to install a package
pub fn install_package(package_id: &str) -> bool {
    if let Some(pkg_mgr) = get_sigmapackagemanager_mut() {
        pkg_mgr.install_package(package_id)
    } else {
        false
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_creation() {
        let package = Package::new(
            "pkg1".to_string(),
            "firefox".to_string(),
            "120.0".to_string(),
            PackageFormat::Deb,
            "Web browser".to_string(),
        );

        assert_eq!(package.package_id, "pkg1");
        assert_eq!(package.format, PackageFormat::Deb);
        assert!(!package.is_installed());
    }

    #[test]
    fn test_package_state() {
        let mut package = Package::new(
            "pkg1".to_string(),
            "firefox".to_string(),
            "120.0".to_string(),
            PackageFormat::Deb,
            "Web browser".to_string(),
        );

        package.set_state(PackageState::Installed);
        assert!(package.is_installed());

        package.set_state(PackageState::UpdateAvailable);
        assert!(package.has_update());
    }

    #[test]
    fn test_repository_creation() {
        let repo = Repository::new(
            "repo1".to_string(),
            "Main".to_string(),
            "http://archive.ubuntu.com/ubuntu".to_string(),
            PackageFormat::Deb,
        );

        assert_eq!(repo.repo_id, "repo1");
        assert!(repo.is_enabled());
    }

    #[test]
    fn test_sigmapackagemanager_creation() {
        let pkg_mgr = SigmaPackageManager::new();
        assert!(!pkg_mgr.is_running());
        assert_eq!(pkg_mgr.package_count(), 0);
        assert_eq!(pkg_mgr.repository_count(), 0);
    }

    #[test]
    fn test_sigmapackagemanager_start_stop() {
        let mut pkg_mgr = SigmaPackageManager::new();
        pkg_mgr.start();
        assert!(pkg_mgr.is_running());
        pkg_mgr.stop();
        assert!(!pkg_mgr.is_running());
    }

    #[test]
    fn test_sigmapackagemanager_add_package() {
        let mut pkg_mgr = SigmaPackageManager::new();
        let package = Package::new(
            "pkg1".to_string(),
            "firefox".to_string(),
            "120.0".to_string(),
            PackageFormat::Deb,
            "Web browser".to_string(),
        );

        let added = pkg_mgr.add_package(package);
        assert!(added);
        assert_eq!(pkg_mgr.package_count(), 1);
    }

    #[test]
    fn test_sigmapackagemanager_install_package() {
        let mut pkg_mgr = SigmaPackageManager::new();

        pkg_mgr.add_package(Package::new(
            "pkg1".to_string(),
            "firefox".to_string(),
            "120.0".to_string(),
            PackageFormat::Deb,
            "Web browser".to_string(),
        ));

        let installed = pkg_mgr.install_package("pkg1");
        assert!(installed);

        let package = pkg_mgr.get_package("pkg1").unwrap();
        assert!(package.is_installed());
    }

    #[test]
    fn test_sigmapackagemanager_add_repository() {
        let mut pkg_mgr = SigmaPackageManager::new();
        let repo = Repository::new(
            "repo1".to_string(),
            "Main".to_string(),
            "http://archive.ubuntu.com/ubuntu".to_string(),
            PackageFormat::Deb,
        );

        let added = pkg_mgr.add_repository(repo);
        assert!(added);
        assert_eq!(pkg_mgr.repository_count(), 1);
    }

    #[test]
    fn test_sigmapackagemanager_search_packages() {
        let mut pkg_mgr = SigmaPackageManager::new();

        pkg_mgr.add_package(Package::new(
            "pkg1".to_string(),
            "firefox".to_string(),
            "120.0".to_string(),
            PackageFormat::Deb,
            "Web browser".to_string(),
        ));

        pkg_mgr.add_package(Package::new(
            "pkg2".to_string(),
            "chromium".to_string(),
            "119.0".to_string(),
            PackageFormat::Deb,
            "Web browser".to_string(),
        ));

        let results = pkg_mgr.search_packages("browser");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_global_instance() {
        init_sigmapackagemanager();

        let pkg_mgr = get_sigmapackagemanager();
        assert!(pkg_mgr.is_some());

        let mut pkg_mgr_mut = get_sigmapackagemanager_mut();
        assert!(pkg_mgr_mut.is_some());

        if let Some(pkg_mgr) = pkg_mgr_mut {
            pkg_mgr.start();
            assert!(pkg_mgr.is_running());
        }
    }

    #[test]
    fn test_convenience_add_package() {
        init_sigmapackagemanager();

        let package = Package::new(
            "pkg1".to_string(),
            "firefox".to_string(),
            "120.0".to_string(),
            PackageFormat::Deb,
            "Web browser".to_string(),
        );

        let added = add_package(package);
        assert!(added);

        let pkg_mgr = get_sigmapackagemanager().unwrap();
        assert_eq!(pkg_mgr.package_count(), 1);
    }

    #[test]
    fn test_convenience_install_package() {
        init_sigmapackagemanager();

        let package = Package::new(
            "pkg1".to_string(),
            "firefox".to_string(),
            "120.0".to_string(),
            PackageFormat::Deb,
            "Web browser".to_string(),
        );

        add_package(package);
        let installed = install_package("pkg1");
        assert!(installed);

        let pkg_mgr = get_sigmapackagemanager().unwrap();
        let pkg = pkg_mgr.get_package("pkg1").unwrap();
        assert!(pkg.is_installed());
    }
;
