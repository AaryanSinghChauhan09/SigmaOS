// SPDX-License-Identifier: MIT
// SigmaOS MintInstall-Inspired Software Manager
// Linux Mint MintInstall-inspired software manager with Flatpak and system package support

use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;

/// Package source type
#[derive(Debug, Clone, PartialEq)]
pub enum PackageSource {
    SystemPackage,
    Flatpak,
    Snap,
    AppImage,
    Custom,
}

/// Package category
#[derive(Debug, Clone, PartialEq)]
pub enum PackageCategory {
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
    Featured,
    TopRated,
    Custom(String),
}

/// Package installation state
#[derive(Debug, Clone, PartialEq)]
pub enum PackageState {
    NotInstalled,
    Installed,
    UpdateAvailable,
    Installing,
    Removing,
    Error(String),
}

/// Package information
#[derive(Debug, Clone)]
pub struct PackageInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub source: PackageSource,
    pub category: PackageCategory,
    pub state: PackageState,
    pub icon: String,
    pub rating: f32,
    pub downloads: u64,
    pub size: u64,
    pub author: String,
    pub license: String,
    pub dependencies: Vec<String>,
    pub screenshots: Vec<String>,
}

impl PackageInfo {
    pub fn new(id: String, name: String, source: PackageSource) -> Self {
        Self {
            id,
            name,
            description: String::new(),
            version: String::new(),
            source,
            category: PackageCategory::Utilities,
            state: PackageState::NotInstalled,
            icon: String::new(),
            rating: 0.0,
            downloads: 0,
            size: 0,
            author: String::new(),
            license: String::new(),
            dependencies: Vec::new(),
            screenshots: Vec::new(),
        }
    }

    /// Check if package is installed
    pub fn is_installed(&self) -> bool {
        matches!(self.state, PackageState::Installed | PackageState::UpdateAvailable)
    }

    /// Check if update is available
    pub fn has_update(&self) -> bool {
        matches!(self.state, PackageState::UpdateAvailable)
    }
}

/// Package tile for display
#[derive(Debug, Clone)]
pub struct PackageTile {
    pub package: PackageInfo,
    pub featured: bool,
    pub top_rated: bool,
}

impl PackageTile {
    pub fn new(package: PackageInfo) -> Self {
        Self {
            package,
            featured: false,
            top_rated: false,
        }
    }

    /// Set as featured
    pub fn set_featured(&mut self, featured: bool) {
        self.featured = featured;
    }

    /// Set as top rated
    pub fn set_top_rated(&mut self, top_rated: bool) {
        self.top_rated = top_rated;
    }
}

/// MintInstall-inspired software manager
#[derive(Debug, Clone)]
pub struct MintInstallManager {
    pub packages: BTreeMap<String, PackageInfo>,
    pub categories: Vec<PackageCategory>,
    pub featured_packages: Vec<String>,
    pub top_rated_packages: Vec<String>,
    pub search_index: BTreeMap<String, Vec<String>>,
    pub installer_active: bool,
    pub current_task: Option<String>,
}

impl MintInstallManager {
    pub fn new() -> Self {
        let mut manager = Self {
            packages: BTreeMap::new(),
            categories: Vec::new(),
            featured_packages: Vec::new(),
            top_rated_packages: Vec::new(),
            search_index: BTreeMap::new(),
            installer_active: false,
            current_task: None,
        };

        // Initialize categories
        manager.init_categories();
        manager
    }

    /// Initialize default categories
    fn init_categories(&mut self) {
        self.categories = vec![
            PackageCategory::AudioVideo,
            PackageCategory::Development,
            PackageCategory::Education,
            PackageCategory::Games,
            PackageCategory::Graphics,
            PackageCategory::Network,
            PackageCategory::Office,
            PackageCategory::Science,
            PackageCategory::System,
            PackageCategory::Utilities,
        ];
    }

    /// Add package to manager
    pub fn add_package(&mut self, package: PackageInfo) {
        let id = package.id.clone();
        self.index_package(&package);
        self.packages.insert(id, package);
    }

    /// Index package for search
    fn index_package(&mut self, package: &PackageInfo) {
        let words: Vec<String> = package.name
            .to_lowercase()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        for word in words {
            self.search_index
                .entry(word)
                .or_insert_with(Vec::new)
                .push(package.id.clone());
        }
    }

    /// Search packages by query
    pub fn search(&self, query: &str) -> Vec<PackageInfo> {
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();
        let mut seen = std::collections::HashSet::new();

        for word in query_lower.split_whitespace() {
            if let Some(ids) = self.search_index.get(word) {
                for id in ids {
                    if seen.insert(id) {
                        if let Some(pkg) = self.packages.get(id) {
                            results.push(pkg.clone());
                        }
                    }
                }
            }
        }

        results
    }

    /// Get packages by category
    pub fn get_by_category(&self, category: &PackageCategory) -> Vec<PackageInfo> {
        self.packages
            .values()
            .filter(|p| &p.category == category)
            .cloned()
            .collect()
    }

    /// Get featured packages
    pub fn get_featured(&self) -> Vec<PackageInfo> {
        self.featured_packages
            .iter()
            .filter_map(|id| self.packages.get(id).cloned())
            .collect()
    }

    /// Get top rated packages
    pub fn get_top_rated(&self) -> Vec<PackageInfo> {
        let mut packages: Vec<_> = self
            .packages
            .values()
            .filter(|p| p.rating > 0.0)
            .cloned()
            .collect();

        packages.sort_by(|a, b| b.rating.partial_cmp(&a.rating).unwrap());
        packages
    }

    /// Install package
    pub fn install(&mut self, id: String) -> Result<(), &'static str> {
        if let Some(package) = self.packages.get_mut(&id) {
            if package.is_installed() {
                return Err("Package already installed");
            }

            package.state = PackageState::Installing;
            self.installer_active = true;
            self.current_task = Some(format!("Installing {}", package.name));

            // In real implementation, would actually install
            package.state = PackageState::Installed;
            self.installer_active = false;
            self.current_task = None;

            Ok(())
        } else {
            Err("Package not found")
        }
    }

    /// Remove package
    pub fn remove(&mut self, id: String) -> Result<(), &'static str> {
        if let Some(package) = self.packages.get_mut(&id) {
            if !package.is_installed() {
                return Err("Package not installed");
            }

            package.state = PackageState::Removing;
            self.installer_active = true;
            self.current_task = Some(format!("Removing {}", package.name));

            // In real implementation, would actually remove
            package.state = PackageState::NotInstalled;
            self.installer_active = false;
            self.current_task = None;

            Ok(())
        } else {
            Err("Package not found")
        }
    }

    /// Update package
    pub fn update(&mut self, id: String) -> Result<(), &'static str> {
        if let Some(package) = self.packages.get_mut(&id) {
            if !package.has_update() {
                return Err("No update available");
            }

            package.state = PackageState::Installing;
            self.installer_active = true;
            self.current_task = Some(format!("Updating {}", package.name));

            // In real implementation, would actually update
            package.state = PackageState::Installed;
            self.installer_active = false;
            self.current_task = None;

            Ok(())
        } else {
            Err("Package not found")
        }
    }

    /// Set featured packages
    pub fn set_featured(&mut self, ids: Vec<String>) {
        self.featured_packages = ids;
    }

    /// Set top rated packages
    pub fn set_top_rated(&mut self, ids: Vec<String>) {
        self.top_rated_packages = ids;
    }

    /// Get package by ID
    pub fn get_package(&self, id: &str) -> Option<&PackageInfo> {
        self.packages.get(id)
    }

    /// Get all packages
    pub fn get_all_packages(&self) -> Vec<PackageInfo> {
        self.packages.values().cloned().collect()
    }

    /// Check for updates
    pub fn check_updates(&mut self) -> Vec<String> {
        let mut updates = Vec::new();

        for (id, package) in &mut self.packages {
            if package.is_installed() {
                // In real implementation, would check for updates
                // For now, randomly mark some as having updates
                if id.len() % 3 == 0 {
                    package.state = PackageState::UpdateAvailable;
                    updates.push(id.clone());
                }
            }
        }

        updates
    }

    /// Get installer status
    pub fn get_installer_status(&self) -> (bool, Option<String>) {
        (self.installer_active, self.current_task.clone())
    }
}

impl Default for MintInstallManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_info() {
        let pkg = PackageInfo::new(
            String::from("firefox"),
            String::from("Firefox"),
            PackageSource::SystemPackage
        );
        
        assert_eq!(pkg.name, "Firefox");
        assert!(!pkg.is_installed());
    }

    #[test]
    fn test_package_tile() {
        let pkg = PackageInfo::new(
            String::from("vlc"),
            String::from("VLC Media Player"),
            PackageSource::Flatpak
        );
        let mut tile = PackageTile::new(pkg);
        
        tile.set_featured(true);
        assert!(tile.featured);
    }

    #[test]
    fn test_add_package() {
        let mut manager = MintInstallManager::new();
        let pkg = PackageInfo::new(
            String::from("gedit"),
            String::from("Gedit"),
            PackageSource::SystemPackage
        );
        
        manager.add_package(pkg);
        assert_eq!(manager.packages.len(), 1);
    }

    #[test]
    fn test_search() {
        let mut manager = MintInstallManager::new();
        let pkg = PackageInfo::new(
            String::from("libreoffice"),
            String::from("LibreOffice"),
            PackageSource::SystemPackage
        );
        manager.add_package(pkg);
        
        let results = manager.search("libreoffice");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_install_remove() {
        let mut manager = MintInstallManager::new();
        let pkg = PackageInfo::new(
            String::from("vim"),
            String::from("Vim"),
            PackageSource::SystemPackage
        );
        manager.add_package(pkg);
        
        assert!(manager.install(String::from("vim")).is_ok());
        assert!(manager.get_package("vim").unwrap().is_installed());
        
        assert!(manager.remove(String::from("vim")).is_ok());
        assert!(!manager.get_package("vim").unwrap().is_installed());
    }

    #[test]
    fn test_categories() {
        let manager = MintInstallManager::new();
        assert!(manager.categories.contains(&PackageCategory::Development));
        assert!(manager.categories.contains(&PackageCategory::Games));
    }

    #[test]
    fn test_featured_packages() {
        let mut manager = MintInstallManager::new();
        let pkg1 = PackageInfo::new(
            String::from("pkg1"),
            String::from("Package 1"),
            PackageSource::SystemPackage
        );
        let pkg2 = PackageInfo::new(
            String::from("pkg2"),
            String::from("Package 2"),
            PackageSource::Flatpak
        );
        
        manager.add_package(pkg1);
        manager.add_package(pkg2);
        manager.set_featured(vec![String::from("pkg1")]);
        
        let featured = manager.get_featured();
        assert_eq!(featured.len(), 1);
    }
}