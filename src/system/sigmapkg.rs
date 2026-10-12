// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaPkg - Package Manager
//!
//! SigmaPkg provides package management for SigmaOS, inspired by apt/dnf from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Package state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageState {
    /// Installed
    Installed,
    /// Not installed
    NotInstalled,
    /// Config files present
    ConfigFiles,
    /// Half installed
    HalfInstalled,
    /// Unpacked
    Unpacked,
}

impl fmt::Display for PackageState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PackageState::Installed => write!(f, "installed"),
            PackageState::NotInstalled => write!(f, "not_installed"),
            PackageState::ConfigFiles => write!(f, "config_files"),
            PackageState::HalfInstalled => write!(f, "half_installed"),
            PackageState::Unpacked => write!(f, "unpacked"),
        }
    }
}

/// Package action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageAction {
    /// Install
    Install,
    /// Remove
    Remove,
    /// Upgrade
    Upgrade,
    /// Downgrade
    Downgrade,
    /// Purge
    Purge,
}

impl fmt::Display for PackageAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PackageAction::Install => write!(f, "install"),
            PackageAction::Remove => write!(f, "remove"),
            PackageAction::Upgrade => write!(f, "upgrade"),
            PackageAction::Downgrade => write!(f, "downgrade"),
            PackageAction::Purge => write!(f, "purge"),
        }
    }
}

/// Package
#[derive(Debug, Clone)]
pub struct Package {
    /// Package name
    pub name: String,
    /// Version
    pub version: String,
    /// Description
    pub description: String,
    /// State
    pub state: PackageState,
    /// Size in bytes
    pub size: u64,
    /// Dependencies
    pub dependencies: Vec<String>,
}

impl Package {
    /// Create a new package
    pub fn new(name: String, version: String, description: String) -> Self {
        Self {
            name,
            version,
            description,
            state: PackageState::NotInstalled,
            size: 0,
            dependencies: Vec::new(),
        }
    }

    /// Set state
    pub fn with_state(mut self, state: PackageState) -> Self {
        self.state = state;
        self
    }

    /// Set size
    pub fn with_size(mut self, size: u64) -> Self {
        self.size = size;
        self
    }

    /// Set dependencies
    pub fn with_dependencies(mut self, dependencies: Vec<String>) -> Self {
        self.dependencies = dependencies;
        self
    }
}

/// SigmaPkg - Package Manager
pub struct SigmaPkg {
    /// Packages (name -> package)
    packages: BTreeMap<String, Package>,
    /// Running state
    running: bool,
    /// Action counter
    action_counter: u32,
}

impl SigmaPkg {
    /// Create a new SigmaPkg instance
    pub fn new() -> Self {
        Self {
            packages: BTreeMap::new(),
            running: false,
            action_counter: 0,
        }
    }

    /// Start the package manager
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the package manager
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Add a package
    pub fn add_package(&mut self, package: Package) -> Result<(), String> {
        if self.packages.contains_key(&package.name) {
            return Err(format!("Package '{}' already exists", package.name));
        }
        
        self.packages.insert(package.name.clone(), package);
        Ok(())
    }

    /// Remove a package
    pub fn remove_package(&mut self, name: &str) -> Result<(), String> {
        if self.packages.remove(name).is_none() {
            return Err(format!("Package '{}' not found", name));
        }
        Ok(())
    }

    /// Get a package
    pub fn get_package(&self, name: &str) -> Option<&Package> {
        self.packages.get(name)
    }

    /// Install a package
    pub fn install(&mut self, name: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaPkg not running".to_string());
        }
        
        match self.packages.get_mut(name) {
            Some(package) => {
                package.state = PackageState::Installed;
                self.action_counter += 1;
                Ok(())
            }
            None => Err(format!("Package '{}' not found", name)),
        }
    }

    /// Remove a package
    pub fn remove(&mut self, name: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaPkg not running".to_string());
        }
        
        match self.packages.get_mut(name) {
            Some(package) => {
                package.state = PackageState::NotInstalled;
                self.action_counter += 1;
                Ok(())
            }
            None => Err(format!("Package '{}' not found", name)),
        }
    }

    /// Upgrade a package
    pub fn upgrade(&mut self, name: &str, new_version: String) -> Result<(), String> {
        if !self.running {
            return Err("SigmaPkg not running".to_string());
        }
        
        match self.packages.get_mut(name) {
            Some(package) => {
                package.version = new_version;
                self.action_counter += 1;
                Ok(())
            }
            None => Err(format!("Package '{}' not found", name)),
        }
    }

    /// Purge a package
    pub fn purge(&mut self, name: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaPkg not running".to_string());
        }
        
        match self.packages.get_mut(name) {
            Some(package) => {
                package.state = PackageState::NotInstalled;
                self.action_counter += 1;
                Ok(())
            }
            None => Err(format!("Package '{}' not found", name)),
        }
    }

    /// List all packages
    pub fn list_packages(&self) -> Vec<&Package> {
        self.packages.values().collect()
    }

    /// List installed packages
    pub fn list_installed(&self) -> Vec<&Package> {
        self.packages.values()
            .filter(|p| p.state == PackageState::Installed)
            .collect()
    }

    /// Get package count
    pub fn package_count(&self) -> usize {
        self.packages.len()
    }

    /// Get package manager statistics
    pub fn pkg_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert("total_packages".to_string(), self.package_count().to_string());
        stats.insert("installed_packages".to_string(), self.list_installed().len().to_string());
        stats.insert("total_actions".to_string(), self.action_counter.to_string());
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
}

impl Default for SigmaPkg {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaPkg instance (placeholder for actual global state)
static unsafe mut SIGMAPKG_INSTANCE: Option<SigmaPkg> = None;

/// Initialize the global SigmaPkg instance
pub fn init_sigmapkg() {
    unsafe {
        SIGMAPKG_INSTANCE = Some(SigmaPkg::new());
    }
}

/// Get the global SigmaPkg instance
pub fn get_sigmapkg() -> Option<&'static mut SigmaPkg> {
    unsafe { SIGMAPKG_INSTANCE.as_mut() }
}

/// Convenience function to install a package
pub fn install_package(name: &str) -> Result<(), String> {
    if let Some(pkg) = get_sigmapkg() {
        pkg.install(name)
    } else {
        Err("SigmaPkg not initialized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_state_display() {
        assert_eq!(PackageState::Installed.to_string(), "installed");
        assert_eq!(PackageState::NotInstalled.to_string(), "not_installed");
    }

    #[test]
    fn test_package_action_display() {
        assert_eq!(PackageAction::Install.to_string(), "install");
        assert_eq!(PackageAction::Remove.to_string(), "remove");
    }

    #[test]
    fn test_package() {
        let package = Package::new(
            "vim".to_string(),
            "1.0.0".to_string(),
            "Text editor".to_string()
        )
        .with_state(PackageState::Installed)
        .with_size(1024)
        .with_dependencies(vec!["libc".to_string()]);

        assert_eq!(package.name, "vim");
        assert_eq!(package.state, PackageState::Installed);
        assert_eq!(package.size, 1024);
    }

    #[test]
    fn test_sigmapkg() {
        let mut pkg = SigmaPkg::new();
        assert!(!pkg.is_running());
        pkg.start();
        assert!(pkg.is_running());
        pkg.stop();
        assert!(!pkg.is_running());
    }

    #[test]
    fn test_sigmapkg_add_package() {
        let mut pkg = SigmaPkg::new();
        
        let package = Package::new(
            "vim".to_string(),
            "1.0.0".to_string(),
            "Text editor".to_string()
        );
        
        assert!(pkg.add_package(package).is_ok());
        assert_eq!(pkg.package_count(), 1);
    }

    #[test]
    fn test_sigmapkg_install() {
        let mut pkg = SigmaPkg::new();
        pkg.start();
        
        let package = Package::new(
            "vim".to_string(),
            "1.0.0".to_string(),
            "Text editor".to_string()
        );
        
        pkg.add_package(package).unwrap();
        assert!(pkg.install("vim").is_ok());
        
        let package = pkg.get_package("vim").unwrap();
        assert_eq!(package.state, PackageState::Installed);
    }

    #[test]
    fn test_sigmapkg_remove() {
        let mut pkg = SigmaPkg::new();
        pkg.start();
        
        let package = Package::new(
            "vim".to_string(),
            "1.0.0".to_string(),
            "Text editor".to_string()
        )
        .with_state(PackageState::Installed);
        
        pkg.add_package(package).unwrap();
        assert!(pkg.remove("vim").is_ok());
        
        let package = pkg.get_package("vim").unwrap();
        assert_eq!(package.state, PackageState::NotInstalled);
    }

    #[test]
    fn test_sigmapkg_upgrade() {
        let mut pkg = SigmaPkg::new();
        pkg.start();
        
        let package = Package::new(
            "vim".to_string(),
            "1.0.0".to_string(),
            "Text editor".to_string()
        );
        
        pkg.add_package(package).unwrap();
        assert!(pkg.upgrade("vim", "2.0.0".to_string()).is_ok());
        
        let package = pkg.get_package("vim").unwrap();
        assert_eq!(package.version, "2.0.0");
    }

    #[test]
    fn test_sigmapkg_list_installed() {
        let mut pkg = SigmaPkg::new();
        
        let package = Package::new(
            "vim".to_string(),
            "1.0.0".to_string(),
            "Text editor".to_string()
        )
        .with_state(PackageState::Installed);
        
        pkg.add_package(package).unwrap();
        
        let installed = pkg.list_installed();
        assert_eq!(installed.len(), 1);
    }

    #[test]
    fn test_sigmapkg_stats() {
        let mut pkg = SigmaPkg::new();
        
        let package = Package::new(
            "vim".to_string(),
            "1.0.0".to_string(),
            "Text editor".to_string()
        )
        .with_state(PackageState::Installed);
        
        pkg.add_package(package).unwrap();
        
        let stats = pkg.pkg_stats();
        assert_eq!(stats.get("total_packages"), Some(&"1".to_string()));
        assert_eq!(stats.get("installed_packages"), Some(&"1".to_string()));
    }

    #[test]
    fn test_install_package_convenience() {
        init_sigmapkg();
        
        let mut pkg = get_sigmapkg().unwrap();
        pkg.start();
        
        let package = Package::new(
            "vim".to_string(),
            "1.0.0".to_string(),
            "Text editor".to_string()
        );
        
        pkg.add_package(package).unwrap();
        
        let result = install_package("vim");
        assert!(result.is_ok());
    }
}
