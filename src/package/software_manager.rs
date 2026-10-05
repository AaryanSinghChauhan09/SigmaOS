//! Software Manager
//!
//! Software management inspired by Linux Mint's Software Manager and Omarchy's
//! software utilities, supporting package search, installation, and repository management.

use std::collections::HashMap;

/// AppPackage status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppPackageStatus {
    Installed,
    NotInstalled,
    UpdateAvailable,
    Pinned,
}

impl AppPackageStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "installed" => Some(AppPackageStatus::Installed),
            "notinstalled" | "not_installed" => Some(AppPackageStatus::NotInstalled),
            "updateavailable" | "update_available" => Some(AppPackageStatus::UpdateAvailable),
            "pinned" => Some(AppPackageStatus::Pinned),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            AppPackageStatus::Installed => "Installed",
            AppPackageStatus::NotInstalled => "Not Installed",
            AppPackageStatus::UpdateAvailable => "Update Available",
            AppPackageStatus::Pinned => "Pinned",
        }
    }
}

/// AppPackage category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppPackageCategory {
    System,
    Development,
    Multimedia,
    Network,
    Graphics,
    Office,
    Games,
    Education,
    Utility,
}

impl AppPackageCategory {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "system" => Some(AppPackageCategory::System),
            "development" => Some(AppPackageCategory::Development),
            "multimedia" => Some(AppPackageCategory::Multimedia),
            "network" => Some(AppPackageCategory::Network),
            "graphics" => Some(AppPackageCategory::Graphics),
            "office" => Some(AppPackageCategory::Office),
            "games" => Some(AppPackageCategory::Games),
            "education" => Some(AppPackageCategory::Education),
            "utility" => Some(AppPackageCategory::Utility),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            AppPackageCategory::System => "System",
            AppPackageCategory::Development => "Development",
            AppPackageCategory::Multimedia => "Multimedia",
            AppPackageCategory::Network => "Network",
            AppPackageCategory::Graphics => "Graphics",
            AppPackageCategory::Office => "Office",
            AppPackageCategory::Games => "Games",
            AppPackageCategory::Education => "Education",
            AppPackageCategory::Utility => "Utility",
        }
    }
}

/// AppPackage
#[derive(Debug, Clone)]
pub struct AppPackage {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub category: AppPackageCategory,
    pub status: AppPackageStatus,
    pub size: u64,
    pub rating: f32,
}

impl AppPackage {
    pub fn new(
        id: String,
        name: String,
        version: String,
        description: String,
        category: AppPackageCategory,
    ) -> Self {
        Self {
            id,
            name,
            version,
            description,
            category,
            status: AppPackageStatus::NotInstalled,
            size: 0,
            rating: 0.0,
        }
    }

    pub fn set_status(&mut self, status: AppPackageStatus) {
        self.status = status;
    }

    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }

    pub fn set_rating(&mut self, rating: f32) {
        self.rating = rating;
    }
}

/// Software repository
#[derive(Debug, Clone)]
pub struct SoftwarePackageRepository {
    pub id: String,
    pub name: String,
    pub url: String,
    pub is_enabled: bool,
}

impl SoftwarePackageRepository {
    pub fn new(id: String, name: String, url: String) -> Self {
        Self {
            id,
            name,
            url,
            is_enabled: true,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.is_enabled = enabled;
    }
}

/// Software manager
#[derive(Debug)]
pub struct SoftwarePackageManager {
    packages: HashMap<String, AppPackage>,
    repositories: HashMap<String, SoftwarePackageRepository>,
}

impl SoftwarePackageManager {
    pub fn new() -> Self {
        let mut manager = Self {
            packages: HashMap::new(),
            repositories: HashMap::new(),
        };

        // Add default repositories
        let main_repo = SoftwarePackageRepository::new(
            "main".to_string(),
            "Main Repository".to_string(),
            "https://packages.sigmaos.org/main".to_string(),
        );
        manager.repositories.insert("main".to_string(), main_repo);

        let community_repo = SoftwarePackageRepository::new(
            "community".to_string(),
            "Community Repository".to_string(),
            "https://packages.sigmaos.org/community".to_string(),
        );
        manager.repositories.insert("community".to_string(), community_repo);

        // Add default packages
        manager.add_default_packages();

        manager
    }

    /// Add default packages
    fn add_default_packages(&mut self) {
        let packages = vec![
            ("sigma-terminal", "Sigma Terminal", "1.0.0", "Terminal emulator", AppPackageCategory::System),
            ("sigma-file-manager", "Sigma File Manager", "1.0.0", "File manager", AppPackageCategory::System),
            ("sigma-text-editor", "Sigma Text Editor", "1.0.0", "Text editor", AppPackageCategory::Development),
            ("sigma-web-browser", "Sigma Web Browser", "1.0.0", "Web browser", AppPackageCategory::Network),
            ("sigma-media-player", "Sigma Media Player", "1.0.0", "Media player", AppPackageCategory::Multimedia),
        ];

        for (id, name, version, description, category) in packages {
            let mut pkg = AppPackage::new(
                id.to_string(),
                name.to_string(),
                version.to_string(),
                description.to_string(),
                category,
            );
            pkg.set_size(10 * 1024 * 1024); // 10 MB
            pkg.set_rating(4.5);
            self.packages.insert(id.to_string(), pkg);
        }
    }

    /// Add a package
    pub fn add_package(&mut self, package: AppPackage) {
        self.packages.insert(package.id.clone(), package);
    }

    /// Get a package
    pub fn get_package(&self, id: &str) -> Option<&AppPackage> {
        self.packages.get(id)
    }

    /// List all packages
    pub fn list_packages(&self) -> Vec<&AppPackage> {
        self.packages.values().collect()
    }

    /// List packages by category
    pub fn list_by_category(&self, category: AppPackageCategory) -> Vec<&AppPackage> {
        self.packages.values()
            .filter(|p| p.category == category)
            .collect()
    }

    /// List packages by status
    pub fn list_by_status(&self, status: AppPackageStatus) -> Vec<&AppPackage> {
        self.packages.values()
            .filter(|p| p.status == status)
            .collect()
    }

    /// Search packages
    pub fn search(&self, query: &str) -> Vec<&AppPackage> {
        let query_lower = query.to_lowercase();
        self.packages.values()
            .filter(|p| {
                p.name.to_lowercase().contains(&query_lower) ||
                p.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Install a package
    pub fn install(&mut self, id: &str) -> Result<(), String> {
        let package = self.packages.get_mut(id)
            .ok_or_else(|| format!("AppPackage {} not found", id))?;

        if package.status == AppPackageStatus::Installed {
            return Err(format!("AppPackage {} is already installed", id));
        }

        package.set_status(AppPackageStatus::Installed);
        Ok(())
    }

    /// Remove a package
    pub fn remove(&mut self, id: &str) -> Result<(), String> {
        let package = self.packages.get_mut(id)
            .ok_or_else(|| format!("AppPackage {} not found", id))?;

        if package.status == AppPackageStatus::NotInstalled {
            return Err(format!("AppPackage {} is not installed", id));
        }

        package.set_status(AppPackageStatus::NotInstalled);
        Ok(())
    }

    /// Update a package
    pub fn update(&mut self, id: &str) -> Result<(), String> {
        let package = self.packages.get_mut(id)
            .ok_or_else(|| format!("AppPackage {} not found", id))?;

        if package.status != AppPackageStatus::UpdateAvailable {
            return Err(format!("AppPackage {} has no update available", id));
        }

        package.set_status(AppPackageStatus::Installed);
        Ok(())
    }

    /// Pin a package
    pub fn pin(&mut self, id: &str) -> Result<(), String> {
        let package = self.packages.get_mut(id)
            .ok_or_else(|| format!("AppPackage {} not found", id))?;

        package.set_status(AppPackageStatus::Pinned);
        Ok(())
    }

    /// Add a repository
    pub fn add_repository(&mut self, repository: SoftwarePackageRepository) {
        self.repositories.insert(repository.id.clone(), repository);
    }

    /// Get a repository
    pub fn get_repository(&self, id: &str) -> Option<&SoftwarePackageRepository> {
        self.repositories.get(id)
    }

    /// List all repositories
    pub fn list_repositories(&self) -> Vec<&SoftwarePackageRepository> {
        self.repositories.values().collect()
    }

    /// Enable a repository
    pub fn enable_repository(&mut self, id: &str) -> Result<(), String> {
        let repo = self.repositories.get_mut(id)
            .ok_or_else(|| format!("Repository {} not found", id))?;

        repo.set_enabled(true);
        Ok(())
    }

    /// Disable a repository
    pub fn disable_repository(&mut self, id: &str) -> Result<(), String> {
        let repo = self.repositories.get_mut(id)
            .ok_or_else(|| format!("Repository {} not found", id))?;

        repo.set_enabled(false);
        Ok(())
    }

    /// Get statistics
    pub fn get_statistics(&self) -> SoftwarePackageStatistics {
        let total_packages = self.packages.len();
        let installed_count = self.packages.values()
            .filter(|p| p.status == AppPackageStatus::Installed)
            .count();
        let update_available_count = self.packages.values()
            .filter(|p| p.status == AppPackageStatus::UpdateAvailable)
            .count();
        let total_repositories = self.repositories.len();
        let enabled_repositories = self.repositories.values()
            .filter(|r| r.is_enabled)
            .count();

        SoftwarePackageStatistics {
            total_packages,
            installed_count,
            update_available_count,
            total_repositories,
            enabled_repositories,
        }
    }
}

impl Default for SoftwarePackageManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Software statistics
#[derive(Debug, Clone)]
pub struct SoftwarePackageStatistics {
    pub total_packages: usize,
    pub installed_count: usize,
    pub update_available_count: usize,
    pub total_repositories: usize,
    pub enabled_repositories: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_status_from_str() {
        assert_eq!(AppPackageStatus::from_str("installed"), Some(AppPackageStatus::Installed));
        assert_eq!(AppPackageStatus::from_str("notinstalled"), Some(AppPackageStatus::NotInstalled));
    }

    #[test]
    fn test_package_category_from_str() {
        assert_eq!(AppPackageCategory::from_str("system"), Some(AppPackageCategory::System));
        assert_eq!(AppPackageCategory::from_str("development"), Some(AppPackageCategory::Development));
    }

    #[test]
    fn test_package_creation() {
        let pkg = AppPackage::new(
            "test".to_string(),
            "Test".to_string(),
            "1.0".to_string(),
            "Test package".to_string(),
            AppPackageCategory::Utility,
        );
        assert_eq!(pkg.name, "Test");
    }

    #[test]
    fn test_software_manager_creation() {
        let manager = SoftwarePackageManager::new();
        assert!(manager.get_package("sigma-terminal").is_some());
    }

    #[test]
    fn test_install_remove() {
        let mut manager = SoftwarePackageManager::new();
        manager.install("sigma-terminal").ok();
        assert_eq!(manager.get_package("sigma-terminal").unwrap().status, AppPackageStatus::Installed);
        manager.remove("sigma-terminal").ok();
        assert_eq!(manager.get_package("sigma-terminal").unwrap().status, AppPackageStatus::NotInstalled);
    }

    #[test]
    fn test_search() {
        let manager = SoftwarePackageManager::new();
        let results = manager.search("terminal");
        assert!(results.len() > 0);
    }

    #[test]
    fn test_list_by_category() {
        let manager = SoftwarePackageManager::new();
        let system = manager.list_by_category(AppPackageCategory::System);
        assert!(system.len() > 0);
    }

    #[test]
    fn test_repository_management() {
        let mut manager = SoftwarePackageManager::new();
        manager.disable_repository("main").ok();
        assert!(!manager.get_repository("main").unwrap().is_enabled);
        manager.enable_repository("main").ok();
        assert!(manager.get_repository("main").unwrap().is_enabled);
    }

    #[test]
    fn test_statistics() {
        let manager = SoftwarePackageManager::new();
        let stats = manager.get_statistics();
        assert!(stats.total_packages >= 5);
    }
}
