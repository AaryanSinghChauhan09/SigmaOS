// SPDX-License-Identifier: MIT
// SigmaOS - Software Manager (inspired by Linux Mint Software Manager)
// Inspired by Linux Mint Software Manager (https://github.com/linuxmint/mintinstall)
// Provides software installation and management

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};

/// Software package
#[derive(Debug)]
pub struct SoftwarePackage {
    pub name: String,
    pub version: String,
    pub description: String,
    pub category: SoftwareCategory,
    pub installed: AtomicBool,
    pub size: u64, // bytes
    pub rating: u32, // 0-5 stars
}

/// Software category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoftwareCategory {
    AudioVideo,
    Development,
    Education,
    Games,
    Graphics,
    Network,
    Office,
    Science,
    System,
    Utilities,
    Other,
}

/// Software manager
pub struct SoftwareManager {
    pub packages: BTreeMap<String, SoftwarePackage>,
    pub installed_packages: Vec<String>,
    pub active: AtomicBool,
}

impl SoftwareManager {
    pub fn new() -> Self {
        Self {
            packages: BTreeMap::new(),
            installed_packages: Vec::new(),
            active: AtomicBool::new(false),
        }
    }
    
    /// Initialize software manager
    pub fn init(&mut self) {
        self.active.store(true, Ordering::Relaxed);
        self.load_package_cache();
    }
    
    /// Load package cache
    pub fn load_package_cache(&mut self) {
        // In real implementation, would load from package database
        // (apt, dnf, pacman, etc.)
        
        // Add some sample packages
        self.add_package(SoftwarePackage {
            name: "firefox".to_string(),
            version: "120.0".to_string(),
            description: "Web Browser".to_string(),
            category: SoftwareCategory::Network,
            installed: AtomicBool::new(true),
            size: 100 * 1024 * 1024, // 100MB
            rating: 5,
        });
        
        self.add_package(SoftwarePackage {
            name: "vlc".to_string(),
            version: "3.0.18".to_string(),
            description: "Media Player".to_string(),
            category: SoftwareCategory::AudioVideo,
            installed: AtomicBool::new(false),
            size: 50 * 1024 * 1024, // 50MB
            rating: 4,
        });
        
        self.add_package(SoftwarePackage {
            name: "libreoffice".to_string(),
            version: "7.6.0".to_string(),
            description: "Office Suite".to_string(),
            category: SoftwareCategory::Office,
            installed: AtomicBool::new(true),
            size: 500 * 1024 * 1024, // 500MB
            rating: 5,
        });
        
        self.add_package(SoftwarePackage {
            name: "steam".to_string(),
            version: "1.0.0".to_string(),
            description: "Steam Client".to_string(),
            category: SoftwareCategory::Games,
            installed: AtomicBool::new(false),
            size: 20 * 1024 * 1024, // 20MB
            rating: 5,
        });
    }
    
    /// Add package
    pub fn add_package(&mut self, package: SoftwarePackage) {
        if package.installed.load(Ordering::Relaxed) {
            self.installed_packages.push(package.name.clone());
        }
        self.packages.insert(package.name.clone(), package);
    }
    
    /// Get package
    pub fn get_package(&self, name: &str) -> Option<&SoftwarePackage> {
        self.packages.get(name)
    }
    
    /// Install package
    pub fn install_package(&mut self, name: &str) -> Result<(), &'static str> {
        let package = self.packages.get_mut(name).ok_or("Package not found")?;
        
        // In real implementation, would run:
        // apt install {name}
        // or dnf install {name}
        // or pacman -S {name}
        
        package.installed.store(true, Ordering::Relaxed);
        self.installed_packages.push(name.to_string());
        
        Ok(())
    }
    
    /// Remove package
    pub fn remove_package(&mut self, name: &str) -> Result<(), &'static str> {
        let package = self.packages.get_mut(name).ok_or("Package not found")?;
        
        // In real implementation, would run:
        // apt remove {name}
        // or dnf remove {name}
        // or pacman -R {name}
        
        package.installed.store(false, Ordering::Relaxed);
        self.installed_packages.retain(|n| n != name);
        
        Ok(())
    }
    
    /// Get packages by category
    pub fn get_packages_by_category(&self, category: SoftwareCategory) -> Vec<&SoftwarePackage> {
        self.packages.values()
            .filter(|p| p.category == category)
            .collect()
    }
    
    /// Get installed packages
    pub fn get_installed_packages(&self) -> Vec<&SoftwarePackage> {
        self.installed_packages.iter()
            .filter_map(|name| self.packages.get(name))
            .collect()
    }
    
    /// Search packages
    pub fn search_packages(&self, query: &str) -> Vec<&SoftwarePackage> {
        let query_lower = query.to_lowercase();
        self.packages.values()
            .filter(|p| {
                p.name.to_lowercase().contains(&query_lower) ||
                p.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    }
    
    /// Is active
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }
}

impl Default for SoftwareManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global software manager
static mut SOFTWARE_MANAGER: Option<SoftwareManager> = None;

/// Initialize software manager
pub fn init_software_manager() {
    unsafe {
        SOFTWARE_MANAGER = Some(SoftwareManager::new());
    }
}

/// Get global software manager
pub fn get_software_manager() -> Option<&'static mut SoftwareManager> {
    unsafe {
        SOFTWARE_MANAGER.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_software_manager_init() {
        let mut manager = SoftwareManager::new();
        manager.init();
        assert!(manager.is_active());
        assert!(manager.packages.len() > 0);
    }
    
    #[test]
    fn test_install_package() {
        let mut manager = SoftwareManager::new();
        manager.load_package_cache();
        manager.install_package("vlc").unwrap();
        assert!(manager.get_package("vlc").unwrap().installed.load(Ordering::Relaxed));
    }
    
    #[test]
    fn test_remove_package() {
        let mut manager = SoftwareManager::new();
        manager.load_package_cache();
        manager.install_package("vlc").unwrap();
        manager.remove_package("vlc").unwrap();
        assert!(!manager.get_package("vlc").unwrap().installed.load(Ordering::Relaxed));
    }
    
    #[test]
    fn test_search_packages() {
        let mut manager = SoftwareManager::new();
        manager.load_package_cache();
        let results = manager.search_packages("fire");
        assert!(results.len() > 0);
    }
    
    #[test]
    fn test_get_packages_by_category() {
        let mut manager = SoftwareManager::new();
        manager.load_package_cache();
        let games = manager.get_packages_by_category(SoftwareCategory::Games);
        assert!(games.len() > 0);
    }
}
