//! Applications Manager
//!
//! Application management system inspired by Linux Mint's mintinstall and Omarchy's
//! applications system, supporting installation, removal, and management of desktop applications.

use std::collections::HashMap;
use std::path::PathBuf;

/// Application category
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DesktopAppCategory {
    Accessories,
    AudioVideo,
    Development,
    Education,
    Games,
    Graphics,
    Internet,
    Office,
    Science,
    System,
    Utility,
    Other,
}

impl DesktopAppCategory {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "accessories" => Some(DesktopAppCategory::Accessories),
            "audiovideo" | "audio" | "video" => Some(DesktopAppCategory::AudioVideo),
            "development" | "dev" => Some(DesktopAppCategory::Development),
            "education" => Some(DesktopAppCategory::Education),
            "games" => Some(DesktopAppCategory::Games),
            "graphics" => Some(DesktopAppCategory::Graphics),
            "internet" => Some(DesktopAppCategory::Internet),
            "office" => Some(DesktopAppCategory::Office),
            "science" => Some(DesktopAppCategory::Science),
            "system" => Some(DesktopAppCategory::System),
            "utility" => Some(DesktopAppCategory::Utility),
            _ => Some(DesktopAppCategory::Other),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            DesktopAppCategory::Accessories => "Accessories",
            DesktopAppCategory::AudioVideo => "AudioVideo",
            DesktopAppCategory::Development => "Development",
            DesktopAppCategory::Education => "Education",
            DesktopAppCategory::Games => "Games",
            DesktopAppCategory::Graphics => "Graphics",
            DesktopAppCategory::Internet => "Internet",
            DesktopAppCategory::Office => "Office",
            DesktopAppCategory::Science => "Science",
            DesktopAppCategory::System => "System",
            DesktopAppCategory::Utility => "Utility",
            DesktopAppCategory::Other => "Other",
        }
    }
}

/// Application status
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppStatus {
    NotInstalled,
    Installed,
    UpdateAvailable,
    Installing,
    Removing,
}

/// Application information
#[derive(Debug, Clone)]
pub struct Application {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub category: DesktopAppCategory,
    pub icon: Option<String>,
    pub status: AppStatus,
    pub size: u64,
    pub installed_size: u64,
    pub dependencies: Vec<String>,
    pub license: String,
    pub repository: String,
}

impl Application {
    pub fn new(
        id: String,
        name: String,
        version: String,
        description: String,
        category: DesktopAppCategory,
    ) -> Self {
        Self {
            id,
            name,
            version,
            description,
            category,
            icon: None,
            status: AppStatus::NotInstalled,
            size: 0,
            installed_size: 0,
            dependencies: Vec::new(),
            license: "Unknown".to_string(),
            repository: "unknown".to_string(),
        }
    }

    pub fn set_status(&mut self, status: AppStatus) {
        self.status = status;
    }

    pub fn set_icon(&mut self, icon: String) {
        self.icon = Some(icon);
    }

    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }

    pub fn set_installed_size(&mut self, size: u64) {
        self.installed_size = size;
    }

    pub fn add_dependency(&mut self, dep: String) {
        self.dependencies.push(dep);
    }

    pub fn set_license(&mut self, license: String) {
        self.license = license;
    }

    pub fn set_repository(&mut self, repo: String) {
        self.repository = repo;
    }

    pub fn is_installed(&self) -> bool {
        self.status == AppStatus::Installed || self.status == AppStatus::UpdateAvailable
    }

    pub fn has_update(&self) -> bool {
        self.status == AppStatus::UpdateAvailable
    }
}

/// Application manager
#[derive(Debug)]
pub struct AppManager {
    applications: HashMap<String, Application>,
    repositories: Vec<String>,
    install_count: u32,
    remove_count: u32,
}

impl AppManager {
    pub fn new() -> Self {
        Self {
            applications: HashMap::new(),
            repositories: Vec::new(),
            install_count: 0,
            remove_count: 0,
        }
    }

    /// Add a repository
    pub fn add_repository(&mut self, repo: String) {
        if !self.repositories.contains(&repo) {
            self.repositories.push(repo);
        }
    }

    /// Remove a repository
    pub fn remove_repository(&mut self, repo: &str) -> Result<(), String> {
        let pos = self.repositories.iter()
            .position(|r| r == repo)
            .ok_or_else(|| format!("Repository {} not found", repo))?;
        self.repositories.remove(pos);
        Ok(())
    }

    /// List repositories
    pub fn list_repositories(&self) -> Vec<&str> {
        self.repositories.iter().map(|s| s.as_str()).collect()
    }

    /// Add an application
    pub fn add_application(&mut self, app: Application) {
        self.applications.insert(app.id.clone(), app);
    }

    /// Get an application
    pub fn get_application(&self, id: &str) -> Option<&Application> {
        self.applications.get(id)
    }

    /// Get an application mutably
    pub fn get_application_mut(&mut self, id: &str) -> Option<&mut Application> {
        self.applications.get_mut(id)
    }

    /// Install an application
    pub fn install(&mut self, id: &str) -> Result<(), String> {
        let app = self.get_application_mut(id)
            .ok_or_else(|| format!("Application {} not found", id))?;

        if app.is_installed() {
            return Err(format!("Application {} is already installed", id));
        }

        app.set_status(AppStatus::Installing);

        // Simulate installation
        app.set_status(AppStatus::Installed);
        app.set_installed_size(app.size);
        self.install_count += 1;

        Ok(())
    }

    /// Remove an application
    pub fn remove(&mut self, id: &str) -> Result<(), String> {
        let app = self.get_application_mut(id)
            .ok_or_else(|| format!("Application {} not found", id))?;

        if !app.is_installed() {
            return Err(format!("Application {} is not installed", id));
        }

        app.set_status(AppStatus::Removing);

        // Simulate removal
        app.set_status(AppStatus::NotInstalled);
        app.set_installed_size(0);
        self.remove_count += 1;

        Ok(())
    }

    /// Update an application
    pub fn update(&mut self, id: &str) -> Result<(), String> {
        let app = self.get_application_mut(id)
            .ok_or_else(|| format!("Application {} not found", id))?;

        if !app.is_installed() {
            return Err(format!("Application {} is not installed", id));
        }

        if !app.has_update() {
            return Err(format!("Application {} has no updates available", id));
        }

        app.set_status(AppStatus::Installing);

        // Simulate update
        app.set_status(AppStatus::Installed);
        self.install_count += 1;

        Ok(())
    }

    /// Check for updates
    pub fn check_updates(&mut self) -> Vec<String> {
        let mut updates = Vec::new();

        for (id, app) in self.applications.iter_mut() {
            if app.is_installed() {
                // Simulate update check
                if id.len() % 3 == 0 {
                    app.set_status(AppStatus::UpdateAvailable);
                    updates.push(id.clone());
                }
            }
        }

        updates
    }

    /// List all applications
    pub fn list_applications(&self) -> Vec<&Application> {
        self.applications.values().collect()
    }

    /// List applications by category
    pub fn list_by_category(&self, category: DesktopAppCategory) -> Vec<&Application> {
        self.applications.values()
            .filter(|app| app.category == category)
            .collect()
    }

    /// List installed applications
    pub fn list_installed(&self) -> Vec<&Application> {
        self.applications.values()
            .filter(|app| app.is_installed())
            .collect()
    }

    /// Search applications
    pub fn search(&self, query: &str) -> Vec<&Application> {
        let query_lower = query.to_lowercase();
        self.applications.values()
            .filter(|app| {
                app.name.to_lowercase().contains(&query_lower)
                    || app.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Get statistics
    pub fn get_statistics(&self) -> AppStatistics {
        let installed_count = self.applications.values()
            .filter(|app| app.is_installed())
            .count();

        let update_count = self.applications.values()
            .filter(|app| app.has_update())
            .count();

        let total_size: u64 = self.applications.values()
            .map(|app| app.installed_size)
            .sum();

        AppStatistics {
            total_apps: self.applications.len(),
            installed_count,
            update_count,
            total_size,
            repository_count: self.repositories.len(),
            install_count: self.install_count,
            remove_count: self.remove_count,
        }
    }
}

impl Default for AppManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Application statistics
#[derive(Debug, Clone)]
pub struct AppStatistics {
    pub total_apps: usize,
    pub installed_count: usize,
    pub update_count: usize,
    pub total_size: u64,
    pub repository_count: usize,
    pub install_count: u32,
    pub remove_count: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_category_from_str() {
        assert_eq!(DesktopAppCategory::from_str("development"), Some(DesktopAppCategory::Development));
        assert_eq!(DesktopAppCategory::from_str("games"), Some(DesktopAppCategory::Games));
    }

    #[test]
    fn test_application_creation() {
        let app = Application::new(
            "test".to_string(),
            "Test App".to_string(),
            "1.0.0".to_string(),
            "Test description".to_string(),
            DesktopAppCategory::Utility,
        );
        assert_eq!(app.id, "test");
        assert_eq!(app.name, "Test App");
        assert_eq!(app.status, AppStatus::NotInstalled);
    }

    #[test]
    fn test_application_status() {
        let mut app = Application::new(
            "test".to_string(),
            "Test App".to_string(),
            "1.0.0".to_string(),
            "Test description".to_string(),
            DesktopAppCategory::Utility,
        );
        assert!(!app.is_installed());
        app.set_status(AppStatus::Installed);
        assert!(app.is_installed());
    }

    #[test]
    fn test_app_manager_creation() {
        let manager = AppManager::new();
        assert_eq!(manager.list_applications().len(), 0);
    }

    #[test]
    fn test_app_manager_add_repository() {
        let mut manager = AppManager::new();
        manager.add_repository("test-repo".to_string());
        assert_eq!(manager.list_repositories().len(), 1);
    }

    #[test]
    fn test_app_manager_add_application() {
        let mut manager = AppManager::new();
        let app = Application::new(
            "test".to_string(),
            "Test App".to_string(),
            "1.0.0".to_string(),
            "Test description".to_string(),
            DesktopAppCategory::Utility,
        );
        manager.add_application(app);
        assert_eq!(manager.list_applications().len(), 1);
    }

    #[test]
    fn test_app_manager_install() {
        let mut manager = AppManager::new();
        let app = Application::new(
            "test".to_string(),
            "Test App".to_string(),
            "1.0.0".to_string(),
            "Test description".to_string(),
            DesktopAppCategory::Utility,
        );
        app.set_size(1024);
        manager.add_application(app);
        assert!(manager.install("test").is_ok());
        assert!(manager.get_application("test").unwrap().is_installed());
    }

    #[test]
    fn test_app_manager_remove() {
        let mut manager = AppManager::new();
        let mut app = Application::new(
            "test".to_string(),
            "Test App".to_string(),
            "1.0.0".to_string(),
            "Test description".to_string(),
            DesktopAppCategory::Utility,
        );
        app.set_status(AppStatus::Installed);
        manager.add_application(app);
        assert!(manager.remove("test").is_ok());
        assert!(!manager.get_application("test").unwrap().is_installed());
    }

    #[test]
    fn test_app_manager_search() {
        let mut manager = AppManager::new();
        let app = Application::new(
            "test".to_string(),
            "Test Application".to_string(),
            "1.0.0".to_string(),
            "A test application for testing".to_string(),
            DesktopAppCategory::Utility,
        );
        manager.add_application(app);
        let results = manager.search("test");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_app_manager_statistics() {
        let mut manager = AppManager::new();
        let mut app = Application::new(
            "test".to_string(),
            "Test App".to_string(),
            "1.0.0".to_string(),
            "Test description".to_string(),
            DesktopAppCategory::Utility,
        );
        app.set_size(1024);
        manager.add_application(app);
        manager.install("test").ok();

        let stats = manager.get_statistics();
        assert_eq!(stats.total_apps, 1);
        assert_eq!(stats.installed_count, 1);
        assert_eq!(stats.total_size, 1024);
    }
}
