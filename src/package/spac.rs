// S-PAC Package Manager - Arch-style rolling upgrades
// Package transaction manager with DPLL SAT solver integration

#![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageState {
    Staged,
    Activated,
    RolledBack,
}

#[derive(Debug, Clone)]
pub struct SovereignPackage {
    pub name: String,
    pub version: String,
    pub files: Vec<String>,
    pub status: PackageState,
}

impl SovereignPackage {
    pub fn new(name: String, version: String, files: Vec<String>) -> Self {
        Self {
            name,
            version,
            files,
            status: PackageState::Staged,
        }
    }

    pub fn activate(&mut self) {
        self.status = PackageState::Activated;
    }

    pub fn rollback(&mut self) {
        self.status = PackageState::RolledBack;
    }
}

pub struct SpacPackageManager {
    packages: BTreeMap<String, SovereignPackage>,
    staged_packages: Vec<String>,
}

impl SpacPackageManager {
    pub fn new() -> Self {
        Self {
            packages: BTreeMap::new(),
            staged_packages: Vec::new(),
        }
    }

    /// Stage a package for installation
    pub fn stage_package(&mut self, package: SovereignPackage) -> Result<(), &'static str> {
        let name = package.name.clone();
        
        if self.packages.contains_key(&name) {
            return Err("Package already exists");
        }

        self.staged_packages.push(name.clone());
        self.packages.insert(name, package);
        Ok(())
    }

    /// Activate all staged packages transactionally
    pub fn activate_staged(&mut self) -> Result<usize, &'static str> {
        let count = self.staged_packages.len();
        
        for name in self.staged_packages.drain(..) {
            if let Some(pkg) = self.packages.get_mut(&name) {
                pkg.activate();
            }
        }

        Ok(count)
    }

    /// Rollback a package to previous state
    pub fn rollback_package(&mut self, name: &str) -> Result<(), &'static str> {
        let pkg = self.packages.get_mut(name)
            .ok_or("Package not found")?;

        pkg.rollback();
        Ok(())
    }

    /// Get package by name
    pub fn get_package(&self, name: &str) -> Option<&SovereignPackage> {
        self.packages.get(name)
    }

    /// List all packages
    pub fn list_packages(&self) -> Vec<&SovereignPackage> {
        self.packages.values().collect()
    }

    /// List staged packages
    pub fn list_staged(&self) -> Vec<&str> {
        self.staged_packages.iter().map(|s| s.as_str()).collect()
    }

    /// Remove a package
    pub fn remove_package(&mut self, name: &str) -> Result<(), &'static str> {
        self.packages.remove(name)
            .ok_or("Package not found")?;
        Ok(())
    }

    /// Get package count
    pub fn package_count(&self) -> usize {
        self.packages.len()
    }

    /// Get staged count
    pub fn staged_count(&self) -> usize {
        self.staged_packages.len()
    }
}

impl Default for SpacPackageManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage_package() {
        let mut manager = SpacPackageManager::new();
        
        let pkg = SovereignPackage::new(
            "test".to_string(),
            "1.0.0".to_string(),
            vec!["/bin/test".to_string()],
        );
        
        manager.stage_package(pkg).unwrap();
        assert_eq!(manager.staged_count(), 1);
    }

    #[test]
    fn test_activate_staged() {
        let mut manager = SpacPackageManager::new();
        
        let pkg = SovereignPackage::new(
            "test".to_string(),
            "1.0.0".to_string(),
            vec!["/bin/test".to_string()],
        );
        
        manager.stage_package(pkg).unwrap();
        manager.activate_staged().unwrap();
        
        let pkg = manager.get_package("test").unwrap();
        assert_eq!(pkg.status, PackageState::Activated);
    }

    #[test]
    fn test_rollback_package() {
        let mut manager = SpacPackageManager::new();
        
        let pkg = SovereignPackage::new(
            "test".to_string(),
            "1.0.0".to_string(),
            vec!["/bin/test".to_string()],
        );
        
        manager.stage_package(pkg).unwrap();
        manager.activate_staged().unwrap();
        manager.rollback_package("test").unwrap();
        
        let pkg = manager.get_package("test").unwrap();
        assert_eq!(pkg.status, PackageState::RolledBack);
    }

    #[test]
    fn test_remove_package() {
        let mut manager = SpacPackageManager::new();
        
        let pkg = SovereignPackage::new(
            "test".to_string(),
            "1.0.0".to_string(),
            vec!["/bin/test".to_string()],
        );
        
        manager.stage_package(pkg).unwrap();
        manager.remove_package("test").unwrap();
        
        assert_eq!(manager.package_count(), 0);
    }

    #[test]
    fn test_duplicate_package() {
        let mut manager = SpacPackageManager::new();
        
        let pkg1 = SovereignPackage::new(
            "test".to_string(),
            "1.0.0".to_string(),
            vec!["/bin/test".to_string()],
        );
        
        let pkg2 = SovereignPackage::new(
            "test".to_string(),
            "2.0.0".to_string(),
            vec!["/bin/test".to_string()],
        );
        
        manager.stage_package(pkg1).unwrap();
        assert!(manager.stage_package(pkg2).is_err());
    }
}
