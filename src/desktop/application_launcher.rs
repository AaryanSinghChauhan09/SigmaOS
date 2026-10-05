//! Application Launcher
//!
//! Application launcher inspired by Linux Mint's menu and Omarchy's
//! launcher utilities, supporting application search, favorites, and categories.

use std::collections::HashMap;

/// Application category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LauncherCategory {
    All,
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

impl LauncherCategory {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "all" => Some(LauncherCategory::All),
            "system" => Some(LauncherCategory::System),
            "development" => Some(LauncherCategory::Development),
            "multimedia" => Some(LauncherCategory::Multimedia),
            "network" => Some(LauncherCategory::Network),
            "graphics" => Some(LauncherCategory::Graphics),
            "office" => Some(LauncherCategory::Office),
            "games" => Some(LauncherCategory::Games),
            "education" => Some(LauncherCategory::Education),
            "utility" => Some(LauncherCategory::Utility),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            LauncherCategory::All => "All",
            LauncherCategory::System => "System",
            LauncherCategory::Development => "Development",
            LauncherCategory::Multimedia => "Multimedia",
            LauncherCategory::Network => "Network",
            LauncherCategory::Graphics => "Graphics",
            LauncherCategory::Office => "Office",
            LauncherCategory::Games => "Games",
            LauncherCategory::Education => "Education",
            LauncherCategory::Utility => "Utility",
        }
    }
}

/// Launcher application
#[derive(Debug, Clone)]
pub struct LauncherApp {
    pub id: String,
    pub name: String,
    pub exec: String,
    pub icon: String,
    pub category: LauncherCategory,
    pub description: String,
    pub is_favorite: bool,
}

impl LauncherApp {
    pub fn new(
        id: String,
        name: String,
        exec: String,
        icon: String,
        category: LauncherCategory,
        description: String,
    ) -> Self {
        Self {
            id,
            name,
            exec,
            icon,
            category,
            description,
            is_favorite: false,
        }
    }

    pub fn set_favorite(&mut self, favorite: bool) {
        self.is_favorite = favorite;
    }
}

/// Application launcher
#[derive(Debug)]
pub struct ApplicationLauncher {
    apps: HashMap<String, LauncherApp>,
    recent_apps: Vec<String>,
}

impl ApplicationLauncher {
    pub fn new() -> Self {
        let mut launcher = Self {
            apps: HashMap::new(),
            recent_apps: Vec::new(),
        };

        // Add default applications
        launcher.add_default_apps();

        launcher
    }

    /// Add default applications
    fn add_default_apps(&mut self) {
        let apps = vec![
            ("terminal", "Terminal", "sigma-terminal", "terminal", LauncherCategory::System, "Terminal emulator"),
            ("file-manager", "Files", "sigma-file-manager", "folder", LauncherCategory::System, "File manager"),
            ("text-editor", "Text Editor", "sigma-text-editor", "text-editor", LauncherCategory::Development, "Text editor"),
            ("web-browser", "Web Browser", "sigma-web-browser", "web-browser", LauncherCategory::Network, "Web browser"),
            ("media-player", "Media Player", "sigma-media-player", "media-player", LauncherCategory::Multimedia, "Media player"),
            ("settings", "Settings", "sigma-settings", "settings", LauncherCategory::System, "System settings"),
            ("calculator", "Calculator", "sigma-calculator", "calculator", LauncherCategory::Utility, "Calculator"),
        ];

        for (id, name, exec, icon, category, description) in apps {
            let app = LauncherApp::new(
                id.to_string(),
                name.to_string(),
                exec.to_string(),
                icon.to_string(),
                category,
                description.to_string(),
            );
            self.apps.insert(id.to_string(), app);
        }
    }

    /// Add an application
    pub fn add_app(&mut self, app: LauncherApp) {
        self.apps.insert(app.id.clone(), app);
    }

    /// Get an application
    pub fn get_app(&self, id: &str) -> Option<&LauncherApp> {
        self.apps.get(id)
    }

    /// List all applications
    pub fn list_apps(&self) -> Vec<&LauncherApp> {
        self.apps.values().collect()
    }

    /// List by category
    pub fn list_by_category(&self, category: LauncherCategory) -> Vec<&LauncherApp> {
        if category == LauncherCategory::All {
            self.list_apps()
        } else {
            self.apps.values()
                .filter(|a| a.category == category)
                .collect()
        }
    }

    /// Search applications
    pub fn search(&self, query: &str) -> Vec<&LauncherApp> {
        let query_lower = query.to_lowercase();
        self.apps.values()
            .filter(|a| {
                a.name.to_lowercase().contains(&query_lower) ||
                a.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Launch an application
    pub fn launch(&mut self, id: &str) -> Result<(), String> {
        let app = self.apps.get(id)
            .ok_or_else(|| format!("Application {} not found", id))?;

        // Add to recent
        if let Some(pos) = self.recent_apps.iter().position(|x| x == id) {
            self.recent_apps.remove(pos);
        }
        self.recent_apps.insert(0, id.to_string());

        // Limit recent to 10
        if self.recent_apps.len() > 10 {
            self.recent_apps.pop();
        }

        // In a real implementation, this would execute the application
        Ok(())
    }

    /// Add to favorites
    pub fn add_favorite(&mut self, id: &str) -> Result<(), String> {
        let app = self.apps.get_mut(id)
            .ok_or_else(|| format!("Application {} not found", id))?;

        app.set_favorite(true);
        Ok(())
    }

    /// Remove from favorites
    pub fn remove_favorite(&mut self, id: &str) -> Result<(), String> {
        let app = self.apps.get_mut(id)
            .ok_or_else(|| format!("Application {} not found", id))?;

        app.set_favorite(false);
        Ok(())
    }

    /// List favorites
    pub fn list_favorites(&self) -> Vec<&LauncherApp> {
        self.apps.values()
            .filter(|a| a.is_favorite)
            .collect()
    }

    /// List recent applications
    pub fn list_recent(&self) -> Vec<&LauncherApp> {
        self.recent_apps.iter()
            .filter_map(|id| self.apps.get(id))
            .collect()
    }

    /// Get statistics
    pub fn get_statistics(&self) -> LauncherStatistics {
        let total_apps = self.apps.len();
        let favorite_count = self.apps.values()
            .filter(|a| a.is_favorite)
            .count();
        let recent_count = self.recent_apps.len();

        LauncherStatistics {
            total_apps,
            favorite_count,
            recent_count,
        }
    }
}

impl Default for ApplicationLauncher {
    fn default() -> Self {
        Self::new()
    }
}

/// Launcher statistics
#[derive(Debug, Clone)]
pub struct LauncherStatistics {
    pub total_apps: usize,
    pub favorite_count: usize,
    pub recent_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_launcher_category_from_str() {
        assert_eq!(LauncherCategory::from_str("system"), Some(LauncherCategory::System));
        assert_eq!(LauncherCategory::from_str("development"), Some(LauncherCategory::Development));
    }

    #[test]
    fn test_launcher_app_creation() {
        let app = LauncherApp::new(
            "test".to_string(),
            "Test".to_string(),
            "test".to_string(),
            "icon".to_string(),
            LauncherCategory::Utility,
            "Test app".to_string(),
        );
        assert_eq!(app.name, "Test");
    }

    #[test]
    fn test_application_launcher_creation() {
        let launcher = ApplicationLauncher::new();
        assert!(launcher.get_app("terminal").is_some());
    }

    #[test]
    fn test_add_app() {
        let mut launcher = ApplicationLauncher::new();
        let app = LauncherApp::new(
            "test".to_string(),
            "Test".to_string(),
            "test".to_string(),
            "icon".to_string(),
            LauncherCategory::Utility,
            "Test".to_string(),
        );
        launcher.add_app(app);
        assert!(launcher.get_app("test").is_some());
    }

    #[test]
    fn test_search() {
        let launcher = ApplicationLauncher::new();
        let results = launcher.search("terminal");
        assert!(results.len() > 0);
    }

    #[test]
    fn test_launch() {
        let mut launcher = ApplicationLauncher::new();
        assert!(launcher.launch("terminal").is_ok());
        assert!(launcher.list_recent().len() > 0);
    }

    #[test]
    fn test_favorites() {
        let mut launcher = ApplicationLauncher::new();
        launcher.add_favorite("terminal").ok();
        assert!(launcher.get_app("terminal").unwrap().is_favorite);
        launcher.remove_favorite("terminal").ok();
        assert!(!launcher.get_app("terminal").unwrap().is_favorite);
    }

    #[test]
    fn test_list_by_category() {
        let launcher = ApplicationLauncher::new();
        let system = launcher.list_by_category(LauncherCategory::System);
        assert!(system.len() > 0);
    }

    #[test]
    fn test_statistics() {
        let launcher = ApplicationLauncher::new();
        let stats = launcher.get_statistics();
        assert!(stats.total_apps >= 7);
    }
}
