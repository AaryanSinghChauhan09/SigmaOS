// SPDX-License-Identifier: MIT
// SigmaOS Pacman-inspired Package Manager
// Arch Linux pacman-inspired package management system

use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;

/// Pacman-inspired package information
#[derive(Debug, Clone, PartialEq)]
pub struct PacmanPackage {
    pub name: String,
    pub version: String,
    pub description: String,
    pub url: String,
    pub license: Vec<String>,
    pub groups: Vec<String>,
    pub depends: Vec<String>,
    pub optdepends: Vec<String>,
    pub provides: Vec<String>,
    pub conflicts: Vec<String>,
    pub replaces: Vec<String>,
    pub size: u64,
    pub installed_size: u64,
    pub architecture: String,
    pub build_date: String,
    pub install_date: String,
    pub packager: String,
    pub files: Vec<String>,
    pub backup: Vec<String>,
    pub install_reason: String,
    pub validation: String,
}

impl PacmanPackage {
    pub fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            description: String::new(),
            url: String::new(),
            license: Vec::new(),
            groups: Vec::new(),
            depends: Vec::new(),
            optdepends: Vec::new(),
            provides: Vec::new(),
            conflicts: Vec::new(),
            replaces: Vec::new(),
            size: 0,
            installed_size: 0,
            architecture: String::from("x86_64"),
            build_date: String::new(),
            install_date: String::new(),
            packager: String::new(),
            files: Vec::new(),
            backup: Vec::new(),
            install_reason: String::from("explicit"),
            validation: String::from("none"),
        }
    }

    /// Add dependency
    pub fn add_dependency(&mut self, dep: String) {
        self.depends.push(dep);
    }

    /// Add optional dependency
    pub fn add_opt_dependency(&mut self, dep: String) {
        self.optdepends.push(dep);
    }

    /// Add conflict
    pub fn add_conflict(&mut self, conflict: String) {
        self.conflicts.push(conflict);
    }

    /// Add file
    pub fn add_file(&mut self, file: String) {
        self.files.push(file);
    }
}

/// Pacman-inspired package database
#[derive(Debug, Clone)]
pub struct PacmanDatabase {
    pub packages: BTreeMap<String, PacmanPackage>,
    pub local_packages: BTreeMap<String, PacmanPackage>,
    pub sync_databases: BTreeMap<String, Vec<String>>,
    pub cache_dir: String,
    pub config_dir: String,
    pub database_dir: String,
}

impl PacmanDatabase {
    pub fn new() -> Self {
        Self {
            packages: BTreeMap::new(),
            local_packages: BTreeMap::new(),
            sync_databases: BTreeMap::new(),
            cache_dir: String::from("/var/cache/pacman/pkg"),
            config_dir: String::from("/etc/pacman.conf"),
            database_dir: String::from("/var/lib/pacman"),
        }
    }

    /// Add package to database
    pub fn add_package(&mut self, package: PacmanPackage) {
        let name = package.name.clone();
        self.packages.insert(name, package);
    }

    /// Add local package
    pub fn add_local_package(&mut self, package: PacmanPackage) {
        let name = package.name.clone();
        self.local_packages.insert(name, package);
    }

    /// Add sync database
    pub fn add_sync_database(&mut self, name: String, packages: Vec<String>) {
        self.sync_databases.insert(name, packages);
    }

    /// Get package by name
    pub fn get_package(&self, name: &str) -> Option<&PacmanPackage> {
        self.packages.get(name)
    }

    /// Get local package by name
    pub fn get_local_package(&self, name: &str) -> Option<&PacmanPackage> {
        self.local_packages.get(name)
    }

    /// Search packages by pattern
    pub fn search_packages(&self, pattern: &str) -> Vec<String> {
        self.packages
            .keys()
            .filter(|name| name.contains(pattern))
            .cloned()
            .collect()
    }

    /// Get package dependencies
    pub fn get_dependencies(&self, name: &str) -> Vec<String> {
        if let Some(pkg) = self.packages.get(name) {
            pkg.depends.clone()
        } else {
            Vec::new()
        }
    }

    /// Check if package is installed
    pub fn is_installed(&self, name: &str) -> bool {
        self.local_packages.contains_key(name)
    }

    /// List all packages
    pub fn list_packages(&self) -> Vec<String> {
        self.packages.keys().cloned().collect()
    }

    /// List installed packages
    pub fn list_installed_packages(&self) -> Vec<String> {
        self.local_packages.keys().cloned().collect()
    }

    /// Get package size
    pub fn get_package_size(&self, name: &str) -> Option<u64> {
        self.packages.get(name).map(|p| p.size)
    }

    /// Get installed package size
    pub fn get_installed_size(&self, name: &str) -> Option<u64> {
        self.local_packages.get(name).map(|p| p.installed_size)
    }
}

impl Default for PacmanDatabase {
    fn default() -> Self {
        Self::new()
    }
}

/// Pacman-inspired package manager
#[derive(Debug, Clone)]
pub struct PacmanPackageManager {
    pub database: PacmanDatabase,
    pub operation: String,
    pub noconfirm: bool,
    pub needed: bool,
    pub overwrite: bool,
    pub asdeps: bool,
    pub asexplicit: bool,
    pub quiet: bool,
    pub verbose: bool,
}

impl PacmanPackageManager {
    pub fn new() -> Self {
        Self {
            database: PacmanDatabase::new(),
            operation: String::new(),
            noconfirm: false,
            needed: false,
            overwrite: false,
            asdeps: false,
            asexplicit: false,
            quiet: false,
            verbose: false,
        }
    }

    /// Install package
    pub fn install(&mut self, package_name: &str) -> Result<(), &'static str> {
        if let Some(pkg) = self.database.get_package(package_name) {
            if self.database.is_installed(package_name) && self.needed {
                return Err("Package already installed and --needed flag set");
            }

            let mut local_pkg = pkg.clone();
            local_pkg.install_date = format!(
                "{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
            );
            local_pkg.install_reason = if self.asdeps {
                String::from("dependency")
            } else {
                String::from("explicit")
            };

            self.database.add_local_package(local_pkg);
            Ok(())
        } else {
            Err("Package not found in database")
        }
    }

    /// Remove package
    pub fn remove(&mut self, package_name: &str) -> Result<(), &'static str> {
        if !self.database.is_installed(package_name) {
            return Err("Package not installed");
        }

        self.database.local_packages.remove(package_name);
        Ok(())
    }

    /// Update package
    pub fn update(&mut self, package_name: &str) -> Result<(), &'static str> {
        if let Some(pkg) = self.database.get_package(package_name) {
            if let Some(local_pkg) = self.database.get_local_package(package_name) {
                if pkg.version == local_pkg.version {
                    return Err("Package already up to date");
                }
            }

            self.remove(package_name)?;
            self.install(package_name)?;
            Ok(())
        } else {
            Err("Package not found in database")
        }
    }

    /// Sync database
    pub fn sync(&mut self) -> Result<(), &'static str> {
        // In real implementation, would download package databases
        Ok(())
    }

    /// Upgrade all packages
    pub fn upgrade(&mut self) -> Result<Vec<String>, &'static str> {
        let mut upgraded = Vec::new();

        for package_name in self.database.list_installed_packages() {
            if let Ok(_) = self.update(&package_name) {
                upgraded.push(package_name);
            }
        }

        Ok(upgraded)
    }

    /// Query package information
    pub fn query(&self, package_name: &str) -> Option<&PacmanPackage> {
        self.database.get_package(package_name)
    }

    /// Search packages
    pub fn search(&self, pattern: &str) -> Vec<String> {
        self.database.search_packages(pattern)
    }

    /// Get package dependencies
    pub fn get_deps(&self, package_name: &str) -> Vec<String> {
        self.database.get_dependencies(package_name)
    }

    /// Check package files
    pub fn check_files(&self, package_name: &str) -> Vec<String> {
        if let Some(pkg) = self.database.get_local_package(package_name) {
            pkg.files.clone()
        } else {
            Vec::new()
        }
    }

    /// Set noconfirm flag
    pub fn set_noconfirm(&mut self, noconfirm: bool) {
        self.noconfirm = noconfirm;
    }

    /// Set needed flag
    pub fn set_needed(&mut self, needed: bool) {
        self.needed = needed;
    }

    /// Set asdeps flag
    pub fn set_asdeps(&mut self, asdeps: bool) {
        self.asdeps = asdeps;
    }

    /// Set asexplicit flag
    pub fn set_asexplicit(&mut self, asexplicit: bool) {
        self.asexplicit = asexplicit;
    }
}

impl Default for PacmanPackageManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pacman_package() {
        let mut pkg = PacmanPackage::new(String::from("test"), String::from("1.0.0"));
        pkg.description = String::from("Test package");
        pkg.add_dependency(String::from("libc"));

        assert_eq!(pkg.name, String::from("test"));
        assert!(pkg.depends.contains(&String::from("libc")));
    }

    #[test]
    fn test_pacman_database() {
        let mut db = PacmanDatabase::new();
        let pkg = PacmanPackage::new(String::from("nginx"), String::from("1.24.0"));
        db.add_package(pkg);

        assert!(db.get_package("nginx").is_some());
        assert!(db.search_packages("ngi").contains(&String::from("nginx")));
    }

    #[test]
    fn test_pacman_install() {
        let mut manager = PacmanPackageManager::new();
        let pkg = PacmanPackage::new(String::from("vim"), String::from("9.0"));
        manager.database.add_package(pkg);

        assert!(manager.install("vim").is_ok());
        assert!(manager.database.is_installed("vim"));
    }

    #[test]
    fn test_pacman_remove() {
        let mut manager = PacmanPackageManager::new();
        let pkg = PacmanPackage::new(String::from("nano"), String::from("7.0"));
        manager.database.add_package(pkg.clone());
        manager.database.add_local_package(pkg);

        assert!(manager.remove("nano").is_ok());
        assert!(!manager.database.is_installed("nano"));
    }

    #[test]
    fn test_pacman_update() {
        let mut manager = PacmanPackageManager::new();
        let old_pkg = PacmanPackage::new(String::from("git"), String::from("2.39"));
        let new_pkg = PacmanPackage::new(String::from("git"), String::from("2.40"));
        manager.database.add_package(new_pkg);
        manager.database.add_local_package(old_pkg);

        assert!(manager.update("git").is_ok());
    }

    #[test]
    fn test_pacman_search() {
        let mut manager = PacmanPackageManager::new();
        let pkg = PacmanPackage::new(String::from("docker"), String::from("24.0"));
        manager.database.add_package(pkg);

        let results = manager.search("doc");
        assert!(results.contains(&String::from("docker")));
    }
}
