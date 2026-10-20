// SPDX-License-Identifier: MIT
// SigmaOS - Software Manager
// Software installation and management inspired by Arch (pacman), Debian (apt), and CachyOS

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Software installation status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallStatus {
    NotInstalled,
    Installing,
    Installed,
    Updating,
    Removing,
    Failed,
}

/// Software source
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SoftwareSource {
    Repository(String),
    Local(String),
    Flatpak(String),
    Snap(String),
}

/// Software package
#[derive(Debug, Clone)]
pub struct SoftwarePackage {
    pub name: String,
    pub version: String,
    pub description: String,
    pub source: SoftwareSource,
    pub size_bytes: u64,
    pub status: InstallStatus,
    pub dependencies: Vec<String>,
}

/// Software manager
pub struct SigmaSoftwareManager {
    packages: Vec<SoftwarePackage>,
}

impl SigmaSoftwareManager {
    pub fn new() -> Self {
        Self {
            packages: Vec::new(),
        }
    }

    pub fn add_package(&mut self, package: SoftwarePackage) {
        self.packages.push(package);
    }

    pub fn get_packages(&self) -> &[SoftwarePackage] {
        &self.packages
    }

    pub fn get_package(&self, name: &str) -> Option<&SoftwarePackage> {
        self.packages.iter().find(|p| p.name == name)
    }

    pub fn get_package_mut(&mut self, name: &str) -> Option<&mut SoftwarePackage> {
        self.packages.iter_mut().find(|p| p.name == name)
    }

    pub fn install_package(&mut self, name: String) -> Result<(), String> {
        if let Some(pkg) = self.get_package_mut(&name) {
            pkg.status = InstallStatus::Installing;
            pkg.status = InstallStatus::Installed;
            Ok(())
        } else {
            Err(format!("Package {} not found", name))
        }
    }

    pub fn remove_package(&mut self, name: String) -> Result<(), String> {
        if let Some(pkg) = self.get_package_mut(&name) {
            pkg.status = InstallStatus::Removing;
            pkg.status = InstallStatus::NotInstalled;
            Ok(())
        } else {
            Err(format!("Package {} not found", name))
        }
    }

    pub fn update_package(&mut self, name: String) -> Result<(), String> {
        if let Some(pkg) = self.get_package_mut(&name) {
            pkg.status = InstallStatus::Updating;
            pkg.status = InstallStatus::Installed;
            Ok(())
        } else {
            Err(format!("Package {} not found", name))
        }
    }

    pub fn get_installed_packages(&self) -> Vec<&SoftwarePackage> {
        self.packages
            .iter()
            .filter(|p| p.status == InstallStatus::Installed)
            .collect()
    }

    pub fn search_packages(&self, query: &str) -> Vec<&SoftwarePackage> {
        self.packages
            .iter()
            .filter(|p| p.name.contains(query) || p.description.contains(query))
            .collect()
    }
}

impl Default for SigmaSoftwareManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_software_manager_creation() {
        let manager = SigmaSoftwareManager::new();
        assert_eq!(manager.get_packages().len(), 0);
    }

    #[test]
    fn test_add_package() {
        let mut manager = SigmaSoftwareManager::new();
        let pkg = SoftwarePackage {
            name: "test".to_string(),
            version: "1.0".to_string(),
            description: "Test package".to_string(),
            source: SoftwareSource::Repository("repo".to_string()),
            size_bytes: 1000,
            status: InstallStatus::NotInstalled,
            dependencies: Vec::new(),
        };
        manager.add_package(pkg);
        assert_eq!(manager.get_packages().len(), 1);
    }

    #[test]
    fn test_install_package() {
        let mut manager = SigmaSoftwareManager::new();
        let pkg = SoftwarePackage {
            name: "test".to_string(),
            version: "1.0".to_string(),
            description: "Test package".to_string(),
            source: SoftwareSource::Repository("repo".to_string()),
            size_bytes: 1000,
            status: InstallStatus::NotInstalled,
            dependencies: Vec::new(),
        };
        manager.add_package(pkg);
        manager.install_package("test".to_string()).unwrap();
        assert_eq!(manager.get_package("test").unwrap().status, InstallStatus::Installed);
    }

    #[test]
    fn test_remove_package() {
        let mut manager = SigmaSoftwareManager::new();
        let pkg = SoftwarePackage {
            name: "test".to_string(),
            version: "1.0".to_string(),
            description: "Test package".to_string(),
            source: SoftwareSource::Repository("repo".to_string()),
            size_bytes: 1000,
            status: InstallStatus::Installed,
            dependencies: Vec::new(),
        };
        manager.add_package(pkg);
        manager.remove_package("test".to_string()).unwrap();
        assert_eq!(manager.get_package("test").unwrap().status, InstallStatus::NotInstalled);
    }

    #[test]
    fn test_update_package() {
        let mut manager = SigmaSoftwareManager::new();
        let pkg = SoftwarePackage {
            name: "test".to_string(),
            version: "1.0".to_string(),
            description: "Test package".to_string(),
            source: SoftwareSource::Repository("repo".to_string()),
            size_bytes: 1000,
            status: InstallStatus::Installed,
            dependencies: Vec::new(),
        };
        manager.add_package(pkg);
        manager.update_package("test".to_string()).unwrap();
        assert_eq!(manager.get_package("test").unwrap().status, InstallStatus::Installed);
    }

    #[test]
    fn test_search_packages() {
        let mut manager = SigmaSoftwareManager::new();
        let pkg1 = SoftwarePackage {
            name: "firefox".to_string(),
            version: "1.0".to_string(),
            description: "Web browser".to_string(),
            source: SoftwareSource::Repository("repo".to_string()),
            size_bytes: 1000,
            status: InstallStatus::Installed,
            dependencies: Vec::new(),
        };
        let pkg2 = SoftwarePackage {
            name: "thunderbird".to_string(),
            version: "1.0".to_string(),
            description: "Email client".to_string(),
            source: SoftwareSource::Repository("repo".to_string()),
            size_bytes: 1000,
            status: InstallStatus::Installed,
            dependencies: Vec::new(),
        };
        manager.add_package(pkg1);
        manager.add_package(pkg2);
        assert_eq!(manager.search_packages("fire").len(), 1);
        assert_eq!(manager.search_packages("browser").len(), 1);
    }
}
