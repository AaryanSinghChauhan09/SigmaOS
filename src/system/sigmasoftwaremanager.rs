//! SigmaSoftwareManager - Software Management System
//!
//! Provides comprehensive software management inspired by Linux Mint and Omarchy.
//! Supports application discovery, installation, updates, and categorization.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Application category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppCategory {
    System,
    Development,
    Graphics,
    Multimedia,
    Office,
    Internet,
    Games,
    Education,
    Science,
    Utilities,
    Accessories,
    Other,
}

/// Application status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppStatus {
    Installed,
    NotInstalled,
    UpdateAvailable,
    Broken,
}

/// Rating score
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rating {
    pub score: u8, // 0-5
    pub count: u32,
}

impl Rating {
    pub fn new(score: u8, count: u32) -> Self {
        Self {
            score: score.min(5),
            count,
        }
    }
}

/// Application
#[derive(Debug, Clone)]
pub struct Application {
    pub id: String,
    pub name: String,
    pub summary: String,
    pub description: String,
    pub category: AppCategory,
    pub version: String,
    pub status: AppStatus,
    pub installed_version: Option<String>,
    pub latest_version: String,
    pub size: u64, // in bytes
    pub rating: Rating,
    pub icon: String,
    pub screenshots: Vec<String>,
}

impl Application {
    pub fn new(id: String, name: String, summary: String, description: String, category: AppCategory, version: String) -> Self {
        Self {
            id,
            name,
            summary,
            description,
            category,
            version: version.clone(),
            status: AppStatus::NotInstalled,
            installed_version: None,
            latest_version: version,
            size: 0,
            rating: Rating::new(0, 0),
            icon: String::new(),
            screenshots: Vec::new(),
        }
    }

    pub fn set_status(&mut self, status: AppStatus) {
        self.status = status;
    }

    pub fn set_installed_version(&mut self, version: Option<String>) {
        self.installed_version = version;
    }

    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }

    pub fn set_rating(&mut self, rating: Rating) {
        self.rating = rating;
    }

    pub fn set_icon(&mut self, icon: String) {
        self.icon = icon;
    }

    pub fn add_screenshot(&mut self, url: String) {
        self.screenshots.push(url);
    }

    pub fn needs_update(&self) -> bool {
        self.status == AppStatus::UpdateAvailable
    }

    pub fn is_installed(&self) -> bool {
        self.status == AppStatus::Installed || self.status == AppStatus::UpdateAvailable
    }
}

/// SigmaSoftwareManager - Software management system
#[derive(Debug)]
pub struct SigmaSoftwareManager {
    applications: BTreeMap<String, Application>,
    categories: BTreeMap<AppCategory, Vec<String>>, // category -> app IDs
}

impl SigmaSoftwareManager {
    pub fn new() -> Self {
        Self {
            applications: BTreeMap::new(),
            categories: BTreeMap::new(),
        }
    }

    pub fn add_application(&mut self, app: Application) {
        let category = app.category;
        let id = app.id.clone();
        self.applications.insert(id.clone(), app);
        self.categories.entry(category).or_insert_with(Vec::new).push(id);
    }

    pub fn remove_application(&mut self, id: &str) -> bool {
        if let Some(app) = self.applications.remove(id) {
            if let Some(apps) = self.categories.get_mut(&app.category) {
                apps.retain(|x| x != id);
            }
            true
        } else {
            false
        }
    }

    pub fn get_application(&self, id: &str) -> Option<&Application> {
        self.applications.get(id)
    }

    pub fn get_application_mut(&mut self, id: &str) -> Option<&mut Application> {
        self.applications.get_mut(id)
    }

    pub fn search_applications(&self, query: &str) -> Vec<&Application> {
        let query_lower = query.to_lowercase();
        self.applications.values()
            .filter(|app| {
                app.name.to_lowercase().contains(&query_lower)
                    || app.summary.to_lowercase().contains(&query_lower)
                    || app.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    pub fn list_by_category(&self, category: AppCategory) -> Vec<&Application> {
        self.categories.get(&category)
            .map(|ids| ids.iter()
                .filter_map(|id| self.applications.get(id))
                .collect())
            .unwrap_or_default()
    }

    pub fn list_installed(&self) -> Vec<&Application> {
        self.applications.values()
            .filter(|app| app.is_installed())
            .collect()
    }

    pub fn list_updates(&self) -> Vec<&Application> {
        self.applications.values()
            .filter(|app| app.needs_update())
            .collect()
    }

    pub fn list_by_rating(&self, min_score: u8) -> Vec<&Application> {
        self.applications.values()
            .filter(|app| app.rating.score >= min_score)
            .collect()
    }

    pub fn application_count(&self) -> usize {
        self.applications.len()
    }

    pub fn installed_count(&self) -> usize {
        self.applications.values()
            .filter(|app| app.is_installed())
            .count()
    }

    pub fn update_count(&self) -> usize {
        self.applications.values()
            .filter(|app| app.needs_update())
            .count()
    }
}

impl Default for SigmaSoftwareManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaSoftwareManager instance
static mut SIGMASOFTWAREMANAGER_INSTANCE: Option<SigmaSoftwareManager> = None;

/// Initialize the global SigmaSoftwareManager instance
pub fn init_sigmasoftwaremanager() {
    unsafe {
        SIGMASOFTWAREMANAGER_INSTANCE = Some(SigmaSoftwareManager::new());
    }
}

/// Get the global SigmaSoftwareManager instance
pub fn get_sigmasoftwaremanager() -> Option<&'static SigmaSoftwareManager> {
    unsafe {
        SIGMASOFTWAREMANAGER_INSTANCE.as_ref()
    }
}

/// Get the global SigmaSoftwareManager instance (mutable)
pub fn get_sigmasoftwaremanager_mut() -> Option<&'static mut SigmaSoftwareManager> {
    unsafe {
        SIGMASOFTWAREMANAGER_INSTANCE.as_mut()
    }
}

/// Convenience function to add an application
pub fn add_application(app: Application) -> bool {
    if let Some(manager) = get_sigmasoftwaremanager_mut() {
        manager.add_application(app);
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rating_creation() {
        let rating = Rating::new(4, 100);
        assert_eq!(rating.score, 4);
        assert_eq!(rating.count, 100);
    }

    #[test]
    fn test_rating_clamping() {
        let rating = Rating::new(7, 10);
        assert_eq!(rating.score, 5); // clamped to 5
    }

    #[test]
    fn test_application_creation() {
        let app = Application::new(
            "app1".to_string(),
            "Firefox".to_string(),
            "Web Browser".to_string(),
            "A web browser".to_string(),
            AppCategory::Internet,
            "100.0".to_string(),
        );
        assert_eq!(app.id, "app1");
        assert_eq!(app.status, AppStatus::NotInstalled);
    }

    #[test]
    fn test_application_modification() {
        let mut app = Application::new(
            "app1".to_string(),
            "Test".to_string(),
            "Test app".to_string(),
            "Test".to_string(),
            AppCategory::System,
            "1.0".to_string(),
        );
        app.set_status(AppStatus::Installed);
        app.set_installed_version(Some("1.0".to_string()));
        app.set_size(1000000);
        app.set_rating(Rating::new(5, 100));
        assert!(app.is_installed());
        assert_eq!(app.size, 1000000);
    }

    #[test]
    fn test_sigmasoftwaremanager_creation() {
        let manager = SigmaSoftwareManager::new();
        assert_eq!(manager.application_count(), 0);
    }

    #[test]
    fn test_sigmasoftwaremanager_management() {
        let mut manager = SigmaSoftwareManager::new();
        let app = Application::new(
            "app1".to_string(),
            "Test".to_string(),
            "Test".to_string(),
            "Test".to_string(),
            AppCategory::System,
            "1.0".to_string(),
        );
        manager.add_application(app);
        assert_eq!(manager.application_count(), 1);
    }

    #[test]
    fn test_application_search() {
        let mut manager = SigmaSoftwareManager::new();
        let app = Application::new(
            "app1".to_string(),
            "Firefox".to_string(),
            "Web Browser".to_string(),
            "A web browser".to_string(),
            AppCategory::Internet,
            "1.0".to_string(),
        );
        manager.add_application(app);
        let results = manager.search_applications("browser");
        assert_eq!(results.len(), 1);
    }
}
