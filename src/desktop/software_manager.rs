// Software Manager
// Linux Mint Software Manager (mintinstall) inspired graphical software browsing and managing utility

use std::collections::HashMap;
use std::path::PathBuf;

/// Package source type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageSource {
    System,  // System packages (APT)
    Flatpak, // Flatpak packages
}

/// Package category
#[derive(Debug, Clone, PartialEq, Eq)]
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
    Accessories,
    Internet,
    Settings,
    Other(String),
}

/// Package state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageState {
    Installed,
    NotInstalled,
    UpdateAvailable,
}

/// Package
#[derive(Debug, Clone)]
pub struct Package {
    pub id: String,
    pub name: String,
    pub summary: String,
    pub description: String,
    pub version: String,
    pub category: PackageCategory,
    pub source: PackageSource,
    pub state: PackageState,
    pub rating: f64,
    pub downloads: u64,
    pub icon: String,
    pub screenshots: Vec<String>,
    pub verified: bool,
}

/// Featured application
#[derive(Debug, Clone)]
pub struct FeaturedApp {
    pub package: Package,
    pub banner_url: String,
    pub position: usize,
}

/// Search result
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub package: Package,
    pub relevance_score: f64,
}

/// Installation progress
#[derive(Debug, Clone)]
pub struct InstallationProgress {
    pub package_id: String,
    pub status: String,
    pub percentage: f64,
    pub download_speed: String,
    pub estimated_time: String,
}

/// Software manager configuration
#[derive(Debug, Clone)]
pub struct SoftwareManagerConfig {
    pub show_system_packages: bool,
    pub show_flatpaks: bool,
    pub auto_update_checks: bool,
    pub search_by_summary: bool,
    pub screenshot_dir: PathBuf,
    pub cache_enabled: bool,
    pub featured_count: usize,
    pub top_rated_count: usize,
}

impl Default for SoftwareManagerConfig {
    fn default() -> Self {
        SoftwareManagerConfig {
            show_system_packages: true,
            show_flatpaks: true,
            auto_update_checks: true,
            search_by_summary: true,
            screenshot_dir: PathBuf::from("/home/user/Pictures/Screenshots"),
            cache_enabled: true,
            featured_count: 5,
            top_rated_count: 30,
        }
    }
}

/// Software manager
#[derive(Debug, Clone)]
pub struct SoftwareManager {
    pub config: SoftwareManagerConfig,
    pub packages: Vec<Package>,
    pub featured_apps: Vec<FeaturedApp>,
    pub top_rated: Vec<Package>,
    pub installed_packages: Vec<String>,
    pub installation_queue: Vec<InstallationProgress>,
    pub search_history: Vec<String>,
    pub flatpak_remotes: Vec<String>,
}

impl Default for SoftwareManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SoftwareManager {
    pub fn new() -> Self {
        SoftwareManager {
            config: SoftwareManagerConfig::default(),
            packages: Vec::new(),
            featured_apps: Vec::new(),
            top_rated: Vec::new(),
            installed_packages: Vec::new(),
            installation_queue: Vec::new(),
            search_history: Vec::new(),
            flatpak_remotes: vec!["flathub".to_string()],
        }
    }

    /// Add package
    pub fn add_package(&mut self, package: Package) {
        self.packages.push(package);
    }

    /// Search packages
    pub fn search(&mut self, query: &str) -> Vec<SearchResult> {
        let query_lower = query.to_lowercase();

        // Add to search history
        self.search_history.retain(|q| q != query);
        self.search_history.insert(0, query.to_string());
        if self.search_history.len() > 50 {
            self.search_history.pop();
        }

        let mut results: Vec<SearchResult> = self
            .packages
            .iter()
            .filter(|pkg| {
                if !self.config.show_system_packages && pkg.source == PackageSource::System {
                    return false;
                }
                if !self.config.show_flatpaks && pkg.source == PackageSource::Flatpak {
                    return false;
                }

                let name_match = pkg.name.to_lowercase().contains(&query_lower);
                let summary_match = if self.config.search_by_summary {
                    pkg.summary.to_lowercase().contains(&query_lower)
                } else {
                    false
                };

                name_match || summary_match
            })
            .map(|pkg| {
                let mut score = 0.0;

                // Name match score
                if pkg.name.to_lowercase() == query_lower {
                    score += 100.0;
                } else if pkg.name.to_lowercase().starts_with(&query_lower) {
                    score += 50.0;
                } else if pkg.name.to_lowercase().contains(&query_lower) {
                    score += 25.0;
                }

                // Summary match score
                if self.config.search_by_summary && pkg.summary.to_lowercase().contains(&query_lower) {
                    score += 10.0;
                }

                // Rating boost
                score += pkg.rating * 5.0;

                // Verified flatpak boost
                if pkg.source == PackageSource::Flatpak && pkg.verified {
                    score += 20.0;
                }

                // Unverified flatpak penalty
                if pkg.source == PackageSource::Flatpak && !pkg.verified {
                    score -= 30.0;
                }

                SearchResult {
                    package: pkg.clone(),
                    relevance_score: score,
                }
            })
            .collect();

        results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());
        results
    }

    /// Get packages by category
    pub fn get_packages_by_category(&self, category: PackageCategory) -> Vec<&Package> {
        self.packages
            .iter()
            .filter(|pkg| pkg.category == category)
            .collect()
    }

    /// Get featured apps
    pub fn get_featured_apps(&self) -> &[FeaturedApp] {
        &self.featured_apps
    }

    /// Get top rated packages
    pub fn get_top_rated(&self) -> &[Package] {
        &self.top_rated
    }

    /// Install package
    pub fn install_package(&mut self, package_id: &str) -> Result<(), String> {
        if self.installed_packages.contains(&package_id.to_string()) {
            return Err("Package already installed".to_string());
        }

        let progress = InstallationProgress {
            package_id: package_id.to_string(),
            status: "Downloading".to_string(),
            percentage: 0.0,
            download_speed: "0 MB/s".to_string(),
            estimated_time: "Calculating...".to_string(),
        };

        self.installation_queue.push(progress);
        self.installed_packages.push(package_id.to_string());

        Ok(())
    }

    /// Remove package
    pub fn remove_package(&mut self, package_id: &str) -> Result<(), String> {
        if !self.installed_packages.contains(&package_id.to_string()) {
            return Err("Package not installed".to_string());
        }

        self.installed_packages.retain(|id| id != package_id);
        Ok(())
    }

    /// Update package
    pub fn update_package(&mut self, package_id: &str) -> Result<(), String> {
        if !self.installed_packages.contains(&package_id.to_string()) {
            return Err("Package not installed".to_string());
        }

        let progress = InstallationProgress {
            package_id: package_id.to_string(),
            status: "Updating".to_string(),
            percentage: 0.0,
            download_speed: "0 MB/s".to_string(),
            estimated_time: "Calculating...".to_string(),
        };

        self.installation_queue.push(progress);
        Ok(())
    }

    /// Get installed packages
    pub fn get_installed_packages(&self) -> Vec<&Package> {
        self.installed_packages
            .iter()
            .filter_map(|id| self.packages.iter().find(|p| &p.id == id))
            .collect()
    }

    /// Get package by id
    pub fn get_package(&self, id: &str) -> Option<&Package> {
        self.packages.iter().find(|p| p.id == id)
    }

    /// Add featured app
    pub fn add_featured_app(&mut self, featured: FeaturedApp) {
        self.featured_apps.push(featured);
    }

    /// Set top rated packages
    pub fn set_top_rated(&mut self, packages: Vec<Package>) {
        self.top_rated = packages;
    }

    /// Add flatpak remote
    pub fn add_flatpak_remote(&mut self, remote: String) {
        if !self.flatpak_remotes.contains(&remote) {
            self.flatpak_remotes.push(remote);
        }
    }

    /// Get flatpak remotes
    pub fn get_flatpak_remotes(&self) -> &[String] {
        &self.flatpak_remotes
    }

    /// Set configuration
    pub fn set_config(&mut self, config: SoftwareManagerConfig) {
        self.config = config;
    }

    /// Get search history
    pub fn get_search_history(&self) -> &[String] {
        &self.search_history
    }

    /// Clear search history
    pub fn clear_search_history(&mut self) {
        self.search_history.clear();
    }

    /// Get installation queue
    pub fn get_installation_queue(&self) -> &[InstallationProgress] {
        &self.installation_queue
    }

    /// Get statistics
    pub fn get_statistics(&self) -> (usize, usize, usize, usize) {
        (
            self.packages.len(),
            self.installed_packages.len(),
            self.featured_apps.len(),
            self.flatpak_remotes.len(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_software_manager_creation() {
        let manager = SoftwareManager::new();
        assert_eq!(manager.packages.len(), 0);
        assert_eq!(manager.config.show_system_packages, true);
    }

    #[test]
    fn test_add_package() {
        let mut manager = SoftwareManager::new();
        let pkg = Package {
            id: "test-pkg".to_string(),
            name: "Test Package".to_string(),
            summary: "Test summary".to_string(),
            description: "Test description".to_string(),
            version: "1.0.0".to_string(),
            category: PackageCategory::Utilities,
            source: PackageSource::System,
            state: PackageState::NotInstalled,
            rating: 4.5,
            downloads: 1000,
            icon: "test".to_string(),
            screenshots: vec![],
            verified: true,
        };
        manager.add_package(pkg);
        assert_eq!(manager.packages.len(), 1);
    }

    #[test]
    fn test_search() {
        let mut manager = SoftwareManager::new();
        let pkg = Package {
            id: "test-pkg".to_string(),
            name: "Firefox".to_string(),
            summary: "Web Browser".to_string(),
            description: "Test".to_string(),
            version: "1.0".to_string(),
            category: PackageCategory::Network,
            source: PackageSource::System,
            state: PackageState::NotInstalled,
            rating: 4.5,
            downloads: 1000,
            icon: "firefox".to_string(),
            screenshots: vec![],
            verified: true,
        };
        manager.add_package(pkg);
        let results = manager.search("firefox");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_install_package() {
        let mut manager = SoftwareManager::new();
        manager.install_package("test-pkg").unwrap();
        assert!(manager.installed_packages.contains(&"test-pkg".to_string()));
    }

    #[test]
    fn test_remove_package() {
        let mut manager = SoftwareManager::new();
        manager.install_package("test-pkg").unwrap();
        manager.remove_package("test-pkg").unwrap();
        assert!(!manager.installed_packages.contains(&"test-pkg".to_string()));
    }

    #[test]
    fn test_update_package() {
        let mut manager = SoftwareManager::new();
        manager.install_package("test-pkg").unwrap();
        manager.update_package("test-pkg").unwrap();
        assert_eq!(manager.installation_queue.len(), 2);
        assert_eq!(manager.installation_queue[1].status, "Updating");
    }

    #[test]
    fn test_get_installed_packages() {
        let mut manager = SoftwareManager::new();
        let pkg = Package {
            id: "test-pkg".to_string(),
            name: "Test".to_string(),
            summary: "Test".to_string(),
            description: "Test".to_string(),
            version: "1.0".to_string(),
            category: PackageCategory::Utilities,
            source: PackageSource::System,
            state: PackageState::Installed,
            rating: 4.0,
            downloads: 100,
            icon: "test".to_string(),
            screenshots: vec![],
            verified: true,
        };
        manager.add_package(pkg);
        manager.install_package("test-pkg").unwrap();
        let installed = manager.get_installed_packages();
        assert_eq!(installed.len(), 1);
    }

    #[test]
    fn test_flatpak_remotes() {
        let mut manager = SoftwareManager::new();
        manager.add_flatpak_remote("flathub-beta".to_string());
        assert!(manager.flatpak_remotes.contains(&"flathub-beta".to_string()));
    }

    #[test]
    fn test_search_history() {
        let mut manager = SoftwareManager::new();
        manager.search("firefox");
        manager.search("chrome");
        assert_eq!(manager.search_history.len(), 2);
        manager.clear_search_history();
        assert_eq!(manager.search_history.len(), 0);
    }

    #[test]
    fn test_featured_apps() {
        let mut manager = SoftwareManager::new();
        let pkg = Package {
            id: "test".to_string(),
            name: "Test".to_string(),
            summary: "Test".to_string(),
            description: "Test".to_string(),
            version: "1.0".to_string(),
            category: PackageCategory::Utilities,
            source: PackageSource::System,
            state: PackageState::NotInstalled,
            rating: 4.0,
            downloads: 100,
            icon: "test".to_string(),
            screenshots: vec![],
            verified: true,
        };
        let featured = FeaturedApp {
            package: pkg,
            banner_url: "banner.png".to_string(),
            position: 0,
        };
        manager.add_featured_app(featured);
        assert_eq!(manager.featured_apps.len(), 1);
    }

    #[test]
    fn test_statistics() {
        let mut manager = SoftwareManager::new();
        let pkg = Package {
            id: "test".to_string(),
            name: "Test".to_string(),
            summary: "Test".to_string(),
            description: "Test".to_string(),
            version: "1.0".to_string(),
            category: PackageCategory::Utilities,
            source: PackageSource::System,
            state: PackageState::NotInstalled,
            rating: 4.0,
            downloads: 100,
            icon: "test".to_string(),
            screenshots: vec![],
            verified: true,
        };
        manager.add_package(pkg);
        manager.install_package("test").unwrap();
        let (total, installed, featured, remotes) = manager.get_statistics();
        assert_eq!(total, 1);
        assert_eq!(installed, 1);
        assert_eq!(featured, 0);
        assert_eq!(remotes, 1);
    }
}
