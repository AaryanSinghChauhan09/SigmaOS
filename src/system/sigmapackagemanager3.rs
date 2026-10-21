// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Aaryan Singh Chauhan

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Package installation state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageState {
    Available,
    Installed,
    Updating,
    Removing,
}

/// Package repository
#[derive(Debug, Clone)]
pub struct PackageRepo {
    pub id: String,
    pub name: String,
    pub url: String,
    pub enabled: bool,
}

/// Package metadata
#[derive(Debug, Clone)]
pub struct Package {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub state: PackageState,
    pub dependencies: Vec<String>,
    pub repo_id: String,
}

/// Package manager for managing software packages
/// Inspired by Omarchy's pkg- commands, Arch pacman, and Debian apt
#[derive(Debug, Clone)]
pub struct SigmaPackageManager3 {
    packages: Vec<Package>,
    repos: Vec<PackageRepo>,
}

impl SigmaPackageManager3 {
    pub fn new() -> Self {
        Self {
            packages: Vec::new(),
            repos: Vec::new(),
        }
    }

    /// Register a package repository
    pub fn register_repo(&mut self, repo: PackageRepo) {
        self.repos.push(repo);
    }

    /// Register a package
    pub fn register_package(&mut self, package: Package) {
        self.packages.push(package);
    }

    /// Get package by ID
    pub fn get_package(&self, id: &str) -> Option<&Package> {
        self.packages.iter().find(|p| p.id == id)
    }

    /// Get repository by ID
    pub fn get_repo(&self, id: &str) -> Option<&PackageRepo> {
        self.repos.iter().find(|r| r.id == id)
    }

    /// Get all packages
    pub fn get_packages(&self) -> &[Package] {
        &self.packages
    }

    /// Get all repositories
    pub fn get_repos(&self) -> &[PackageRepo] {
        &self.repos
    }

    /// Install a package
    pub fn install_package(&mut self, id: &str) -> Result<String, String> {
        if let Some(package) = self.packages.iter_mut().find(|p| p.id == id) {
            package.state = PackageState::Installed;
            Ok(format!("Installed package: {}", package.name))
        } else {
            Err(format!("Package not found: {}", id))
        }
    }

    /// Remove a package
    pub fn remove_package(&mut self, id: &str) -> Result<String, String> {
        if let Some(package) = self.packages.iter_mut().find(|p| p.id == id) {
            package.state = PackageState::Removing;
            Ok(format!("Removing package: {}", package.name))
        } else {
            Err(format!("Package not found: {}", id))
        }
    }

    /// Update a package
    pub fn update_package(&mut self, id: &str) -> Result<String, String> {
        if let Some(package) = self.packages.iter_mut().find(|p| p.id == id) {
            package.state = PackageState::Updating;
            Ok(format!("Updating package: {}", package.name))
        } else {
            Err(format!("Package not found: {}", id))
        }
    }

    /// Enable a repository
    pub fn enable_repo(&mut self, id: &str) -> Result<String, String> {
        if let Some(repo) = self.repos.iter_mut().find(|r| r.id == id) {
            repo.enabled = true;
            Ok(format!("Enabled repository: {}", repo.name))
        } else {
            Err(format!("Repository not found: {}", id))
        }
    }

    /// Disable a repository
    pub fn disable_repo(&mut self, id: &str) -> Result<String, String> {
        if let Some(repo) = self.repos.iter_mut().find(|r| r.id == id) {
            repo.enabled = false;
            Ok(format!("Disabled repository: {}", repo.name))
        } else {
            Err(format!("Repository not found: {}", id))
        }
    }

    /// Get installed packages
    pub fn get_installed_packages(&self) -> Vec<&Package> {
        self.packages
            .iter()
            .filter(|p| p.state == PackageState::Installed)
            .collect()
    }

    /// Get available packages
    pub fn get_available_packages(&self) -> Vec<&Package> {
        self.packages
            .iter()
            .filter(|p| p.state == PackageState::Available)
            .collect()
    }

    /// Get enabled repositories
    pub fn get_enabled_repos(&self) -> Vec<&PackageRepo> {
        self.repos.iter().filter(|r| r.enabled).collect()
    }

    /// Check if package dependencies are satisfied
    pub fn check_dependencies(&self, id: &str) -> Result<bool, String> {
        if let Some(package) = self.get_package(id) {
            let missing: Vec<_> = package
                .dependencies
                .iter()
                .filter(|dep| !self.packages.iter().any(|p| &p.id == *dep))
                .collect();
            if missing.is_empty() {
                Ok(true)
            } else {
                Err(format!("Missing dependencies: {:?}", missing))
            }
        } else {
            Err(format!("Package not found: {}", id))
        }
    }
}

impl Default for SigmaPackageManager3 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_package() {
        let mut manager = SigmaPackageManager3::new();
        let package = Package {
            id: "test-pkg".to_string(),
            name: "Test Package".to_string(),
            version: "1.0.0".to_string(),
            description: "A test package".to_string(),
            state: PackageState::Available,
            dependencies: Vec::new(),
            repo_id: "main".to_string(),
        };
        manager.register_package(package);
        assert_eq!(manager.get_packages().len(), 1);
    }

    #[test]
    fn test_install_package() {
        let mut manager = SigmaPackageManager3::new();
        let package = Package {
            id: "test-pkg".to_string(),
            name: "Test Package".to_string(),
            version: "1.0.0".to_string(),
            description: "A test package".to_string(),
            state: PackageState::Available,
            dependencies: Vec::new(),
            repo_id: "main".to_string(),
        };
        manager.register_package(package);
        manager.install_package("test-pkg").unwrap();
        assert_eq!(
            manager.get_package("test-pkg").unwrap().state,
            PackageState::Installed
        );
    }

    #[test]
    fn test_remove_package() {
        let mut manager = SigmaPackageManager3::new();
        let package = Package {
            id: "test-pkg".to_string(),
            name: "Test Package".to_string(),
            version: "1.0.0".to_string(),
            description: "A test package".to_string(),
            state: PackageState::Installed,
            dependencies: Vec::new(),
            repo_id: "main".to_string(),
        };
        manager.register_package(package);
        manager.remove_package("test-pkg").unwrap();
        assert_eq!(
            manager.get_package("test-pkg").unwrap().state,
            PackageState::Removing
        );
    }

    #[test]
    fn test_update_package() {
        let mut manager = SigmaPackageManager3::new();
        let package = Package {
            id: "test-pkg".to_string(),
            name: "Test Package".to_string(),
            version: "1.0.0".to_string(),
            description: "A test package".to_string(),
            state: PackageState::Installed,
            dependencies: Vec::new(),
            repo_id: "main".to_string(),
        };
        manager.register_package(package);
        manager.update_package("test-pkg").unwrap();
        assert_eq!(
            manager.get_package("test-pkg").unwrap().state,
            PackageState::Updating
        );
    }

    #[test]
    fn test_enable_repo() {
        let mut manager = SigmaPackageManager3::new();
        let repo = PackageRepo {
            id: "extra".to_string(),
            name: "Extra".to_string(),
            url: "https://example.com/extra".to_string(),
            enabled: false,
        };
        manager.register_repo(repo);
        manager.enable_repo("extra").unwrap();
        assert!(manager.get_repo("extra").unwrap().enabled);
    }

    #[test]
    fn test_get_installed_packages() {
        let mut manager = SigmaPackageManager3::new();
        let pkg1 = Package {
            id: "pkg1".to_string(),
            name: "Package 1".to_string(),
            version: "1.0.0".to_string(),
            description: "Package 1".to_string(),
            state: PackageState::Installed,
            dependencies: Vec::new(),
            repo_id: "main".to_string(),
        };
        let pkg2 = Package {
            id: "pkg2".to_string(),
            name: "Package 2".to_string(),
            version: "1.0.0".to_string(),
            description: "Package 2".to_string(),
            state: PackageState::Available,
            dependencies: Vec::new(),
            repo_id: "main".to_string(),
        };
        manager.register_package(pkg1);
        manager.register_package(pkg2);
        assert_eq!(manager.get_installed_packages().len(), 1);
    }

    #[test]
    fn test_check_dependencies() {
        let mut manager = SigmaPackageManager3::new();
        let dep = Package {
            id: "dep".to_string(),
            name: "Dependency".to_string(),
            version: "1.0.0".to_string(),
            description: "Dependency".to_string(),
            state: PackageState::Installed,
            dependencies: Vec::new(),
            repo_id: "main".to_string(),
        };
        let pkg = Package {
            id: "pkg".to_string(),
            name: "Package".to_string(),
            version: "1.0.0".to_string(),
            description: "Package".to_string(),
            state: PackageState::Available,
            dependencies: vec!["dep".to_string()],
            repo_id: "main".to_string(),
        };
        manager.register_package(dep);
        manager.register_package(pkg);
        assert!(manager.check_dependencies("pkg").is_ok());
    }
}
