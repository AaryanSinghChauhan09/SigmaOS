// SPDX-License-Identifier: MIT
// SigmaOS - Software Manager (inspired by Linux Mint Software Manager)
// Inspired by Linux Mint Software Manager (https://github.com/linuxmint/mintinstall)
// Provides software installation and management

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::AtomicBool;

//// Software package
#[derive(Debug)]
pub struct SoftwarePackage {
    pub name: String,
    pub version: String,
    pub description: String,
    pub category: SoftwareCategory,
    pub installed: bool,
    pub size: u64,                     // bytes
    pub rating: u32,                   // 0-5 stars
    pub release_notes: Option<String>, // New: release notes
    pub homepage: Option<String>,      // New: project homepage
}

/// Software category - enhanced with more options inspired by Linux Mint and Omarchy OS
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoftwareCategory {
    // Original categories
    AudioVideo,
    Development,
    Education,
    Games,
    Graphics,
    Network,
    Office,
    System,
    // New categories inspired by Linux Mint and Omarchy OS
    Accessibility,  // New: accessibility tools
    GraphicsDesign, // New: graphics design software
    Internet,       // New: internet utilities
    SystemTools,    // New: system administration tools
    Virtualization, // New: virtualization software
    Kernel,         // New: kernel modules and tools
    Drivers,        // New: device drivers
    Security,       // New: security software
    Utilities,      // New: general utilities
}

/// Software manager - enhanced with new features
pub struct SoftwareManager {
    packages: BTreeMap<String, SoftwarePackage>,
    /// Track installed status atomically
    installed_tracker: AtomicBool,
}

impl SoftwareManager {
    pub fn new() -> Self {
        Self {
            packages: BTreeMap::new(),
            installed_tracker: AtomicBool::new(false),
        }
    }

    /// Install a software package
    pub fn install_package(
        &mut self,
        name: String,
        version: String,
        category: SoftwareCategory,
        size: u64,
    ) -> () {
        let package = SoftwarePackage {
            name: name.clone(),
            version,
            description: format!("{} - installed", name),
            category,
            installed: true,
            size,
            rating: 0,
            release_notes: None,
            homepage: None,
        };
        self.packages.insert(name, package);
    }

    /// Update package rating inspired by user feedback (like Linux Mint's rating system)
    pub fn update_package_rating(&mut self, name: &str, rating: u32) -> Option<&SoftwarePackage> {
        let package = self.packages.get_mut(name)?;
        package.rating = rating.max(0).min(5); // Clamp to 0-5 scale
        Some(package)
    }

    /// Add release notes inspired by Omarchy OS update system
    pub fn set_release_notes(
        &mut self,
        name: &str,
        release_notes: String,
    ) -> Option<&SoftwarePackage> {
        let package = self.packages.get_mut(name)?;
        package.release_notes = Some(release_notes);
        Some(package)
    }

    /// Set homepage inspired by project websites
    pub fn set_homepage(&mut self, name: &str, homepage: String) -> Option<&SoftwarePackage> {
        let package = self.packages.get_mut(name)?;
        package.homepage = Some(homepage);
        Some(package)
    }

    /// Get packages by category - enhanced with filtering
    pub fn get_packages_by_category(&self, category: SoftwareCategory) -> Vec<&SoftwarePackage> {
        self.packages
            .values()
            .filter(|pkg| pkg.category == category)
            .collect()
    }

    /// Search packages by name or description
    pub fn search_packages(&self, query: &str) -> Vec<&SoftwarePackage> {
        let query_lower = query.to_lowercase();
        self.packages
            .values()
            .filter(|pkg| {
                pkg.name.to_lowercase().contains(&query_lower)
                    || pkg.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Get all packages
    pub fn get_all_packages(&self) -> Vec<&SoftwarePackage> {
        self.packages.values().collect()
    }

    /// Check if package is installed
    pub fn is_installed(&self, name: &str) -> bool {
        self.packages.get(name).map_or(false, |pkg| pkg.installed)
    }
}
