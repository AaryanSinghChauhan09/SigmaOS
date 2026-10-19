//! SigmaOS Web Apps Manager
//!
//! Inspired by: Omarchy web apps (omancy-webapp-install, omancy-webapp-remove, omancy-webapp-remove-all)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING,
//! SECURE_DEFAULTS, OPEN_STANDARD_AND_INTEROPERABILITY, USER_FREEDOM_AND_CONTROL.

/// Web app
#[derive(Debug, Clone)]
pub struct WebApp {
    pub id: String,
    pub name: String,
    pub url: String,
    pub icon: Option<String>,
    pub category: String,
    pub install_date: u64,
}

/// Web apps manager
pub struct WebAppsManager {
    pub apps: HashMap<String, WebApp>,
    pub next_id: u64,
}

impl WebAppsManager {
    pub fn new() -> Self {
        Self {
            apps: HashMap::new(),
            next_id: 1,
        }
    }

    /// Install a web app
    pub fn install(
        &mut self,
        name: String,
        url: String,
        icon: Option<String>,
        category: String,
    ) -> Result<String, String> {
        if name.is_empty() {
            return Err("Web app name cannot be empty".to_string());
        }
        if url.is_empty() {
            return Err("Web app URL cannot be empty".to_string());
        }

        // Check for duplicate name
        if self.apps.values().any(|a| a.name == name) {
            return Err(format!("Web app '{}' already installed", name));
        }

        let id = self.next_id.to_string();
        self.next_id += 1;

        let app = WebApp {
            id: id.clone(),
            name,
            url,
            icon,
            category,
            install_date: now_secs(),
        };

        self.apps.insert(id.clone(), app);
        Ok(id)
    }

    /// Remove a web app
    pub fn remove(&mut self, id: &str) -> Result<(), String> {
        self.apps
            .remove(id)
            .ok_or_else(|| format!("Web app {} not found", id))?;
        Ok(())
    }

    /// Remove all web apps
    pub fn remove_all(&mut self) -> usize {
        let count = self.apps.len();
        self.apps.clear();
        count
    }

    /// List all web apps
    pub fn list(&self) -> Vec<&WebApp> {
        self.apps.values().collect()
    }

    /// Get a web app by ID
    pub fn get(&self, id: &str) -> Option<&WebApp> {
        self.apps.get(id)
    }

    /// Get a web app by name
    pub fn get_by_name(&self, name: &str) -> Option<&WebApp> {
        self.apps.values().find(|a| a.name == name)
    }

    /// App count
    pub fn app_count(&self) -> usize {
        self.apps.len()
    }
}

impl Default for WebAppsManager {
    fn default() -> Self {
        Self::new()
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_install_web_app() {
        let mut mgr = WebAppsManager::new();
        let id = mgr.install(
            "GitHub".to_string(),
            "https://github.com".to_string(),
            Some("https://github.com/icon.png".to_string()),
            "Development".to_string(),
        )
        .unwrap();
        assert_eq!(id, "1");
        assert_eq!(mgr.app_count(), 1);
    }

    #[test]
    fn test_install_duplicate_name_fails() {
        let mut mgr = WebAppsManager::new();
        mgr.install(
            "GitHub".to_string(),
            "https://github.com".to_string(),
            None,
            "Development".to_string(),
        )
        .ok();
        let result = mgr.install(
            "GitHub".to_string(),
            "https://github.com".to_string(),
            None,
            "Development".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_install_empty_name_fails() {
        let mut mgr = WebAppsManager::new();
        let result = mgr.install(String::new(), "https://example.com".to_string(), None, "Test".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_web_app() {
        let mut mgr = WebAppsManager::new();
        mgr.install("Test".to_string(), "https://example.com".to_string(), None, "Test".to_string())
            .unwrap();
        assert!(mgr.remove("1").is_ok());
        assert_eq!(mgr.app_count(), 0);
    }

    #[test]
    fn test_remove_all() {
        let mut mgr = WebAppsManager::new();
        mgr.install("App1".to_string(), "https://app1.com".to_string(), None, "Test".to_string())
            .unwrap();
        mgr.install("App2".to_string(), "https://app2.com".to_string(), None, "Test".to_string())
            .unwrap();
        assert_eq!(mgr.remove_all(), 2);
        assert_eq!(mgr.app_count(), 0);
    }

    #[test]
    fn test_get_by_name() {
        let mut mgr = WebAppsManager::new();
        mgr.install("GitHub".to_string(), "https://github.com".to_string(), None, "Development".to_string())
            .unwrap();
        let app = mgr.get_by_name("GitHub").unwrap();
        assert_eq!(app.url, "https://github.com");
    }
}