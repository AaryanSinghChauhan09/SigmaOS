//! WebApp Manager for SigmaOS
//!
//! Inspired by Linux Mint's `webapp-manager` (which allows running websites as standalone desktop applications).
//! Provides:
//! - Web app configuration with isolated browser profiles (isolated cookies & cache)
//! - Multi-backend browser selection (Chromium, Firefox, WebKitGTK, SigmaBrowser)
//! - XDG desktop entry generator (.desktop files with custom StartupWMClass & icons)
//! - Navigation bar toggle, window geometry persistence, and custom user-agent spoofing

#![allow(dead_code)]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Supported browser backends for isolated web application rendering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrowserBackend {
    Chromium,
    Firefox,
    WebKitGtk,
    SigmaBrowser,
}

impl BrowserBackend {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Chromium => "chromium",
            Self::Firefox => "firefox",
            Self::WebKitGtk => "webkitgtk",
            Self::SigmaBrowser => "sigma-browser",
        }
    }

    pub fn binary_name(&self) -> &'static str {
        match self {
            Self::Chromium => "chromium",
            Self::Firefox => "firefox",
            Self::WebKitGtk => "sigma-web-runtime",
            Self::SigmaBrowser => "sigmabrowser",
        }
    }
}

/// Category classification for XDG desktop entries
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebAppCategory {
    Network,
    Office,
    AudioVideo,
    Utility,
    Development,
    Social,
}

impl WebAppCategory {
    pub fn xdg_category(&self) -> &'static str {
        match self {
            Self::Network => "Network;WebBrowser;",
            Self::Office => "Office;",
            Self::AudioVideo => "AudioVideo;",
            Self::Utility => "Utility;",
            Self::Development => "Development;",
            Self::Social => "Network;Chat;InstantMessaging;",
        }
    }
}

/// Configuration descriptor for an isolated web application
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebAppConfig {
    pub id: String,
    pub name: String,
    pub url: String,
    pub icon_path: String,
    pub category: WebAppCategory,
    pub backend: BrowserBackend,
    pub profile_dir: String,
    pub isolated_cookies: bool,
    pub custom_user_agent: Option<String>,
    pub show_nav_bar: bool,
    pub window_width: u32,
    pub window_height: u32,
}

impl WebAppConfig {
    pub fn new(id: &str, name: &str, url: &str) -> Self {
        let clean_id = id.trim().to_lowercase().replace(' ', "-");
        Self {
            profile_dir: format!("~/.local/share/sigma-webapps/{}", clean_id),
            id: clean_id,
            name: name.to_string(),
            url: url.to_string(),
            icon_path: "applications-internet".to_string(),
            category: WebAppCategory::Network,
            backend: BrowserBackend::Chromium,
            isolated_cookies: true,
            custom_user_agent: None,
            show_nav_bar: false,
            window_width: 1200,
            window_height: 800,
        }
    }

    /// Build the executable command line for the configured browser backend
    pub fn build_exec_command(&self) -> String {
        match self.backend {
            BrowserBackend::Chromium => {
                let mut cmd = format!(
                    "{} --app=\"{}\" --user-data-dir=\"{}\"",
                    self.backend.binary_name(),
                    self.url,
                    self.profile_dir
                );
                cmd.push_str(&format!(" --class=\"webapp-{}\"", self.id));
                if let Some(ref ua) = self.custom_user_agent {
                    cmd.push_str(&format!(" --user-agent=\"{}\"", ua));
                }
                cmd
            }
            BrowserBackend::Firefox => {
                format!(
                    "{} --profile \"{}\" --new-window \"{}\"",
                    self.backend.binary_name(),
                    self.profile_dir,
                    self.url
                )
            }
            BrowserBackend::WebKitGtk | BrowserBackend::SigmaBrowser => {
                format!(
                    "{} --app-id=\"{}\" --url=\"{}\" --data-dir=\"{}\"",
                    self.backend.binary_name(),
                    self.id,
                    self.url,
                    self.profile_dir
                )
            }
        }
    }

    /// Generate an XDG `.desktop` specification string
    pub fn generate_desktop_entry(&self) -> String {
        format!(
            "[Desktop Entry]\n\
             Version=1.0\n\
             Type=Application\n\
             Name={}\n\
             Comment=Web application for {}\n\
             Exec={}\n\
             Icon={}\n\
             Terminal=false\n\
             StartupNotify=true\n\
             StartupWMClass=webapp-{}\n\
             Categories={}\n",
            self.name,
            self.url,
            self.build_exec_command(),
            self.icon_path,
            self.id,
            self.category.xdg_category()
        )
    }
}

/// Registry and management engine for WebApps in SigmaOS
#[derive(Debug, Default)]
pub struct WebAppManager {
    apps: BTreeMap<String, WebAppConfig>,
}

impl WebAppManager {
    pub fn new() -> Self {
        Self {
            apps: BTreeMap::new(),
        }
    }

    /// Add a new web app
    pub fn add_app(&mut self, app: WebAppConfig) -> Result<(), &'static str> {
        if app.id.is_empty() {
            return Err("WebApp id cannot be empty");
        }
        if !app.url.starts_with("http://") && !app.url.starts_with("https://") {
            return Err("WebApp URL must start with http:// or https://");
        }
        self.apps.insert(app.id.clone(), app);
        Ok(())
    }

    /// Remove a web app by id
    pub fn remove_app(&mut self, id: &str) -> Option<WebAppConfig> {
        self.apps.remove(id)
    }

    /// Retrieve an app reference
    pub fn get_app(&self, id: &str) -> Option<&WebAppConfig> {
        self.apps.get(id)
    }

    /// List all registered apps
    pub fn list_apps(&self) -> Vec<&WebAppConfig> {
        self.apps.values().collect()
    }

    /// Count registered apps
    pub fn count(&self) -> usize {
        self.apps.len()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webapp_config_creation() {
        let app = WebAppConfig::new("github-web", "GitHub", "https://github.com");
        assert_eq!(app.id, "github-web");
        assert_eq!(app.name, "GitHub");
        assert_eq!(app.url, "https://github.com");
        assert_eq!(app.backend, BrowserBackend::Chromium);
        assert!(app.isolated_cookies);
    }

    #[test]
    fn test_exec_command_chromium() {
        let app = WebAppConfig::new("slack", "Slack", "https://app.slack.com");
        let cmd = app.build_exec_command();
        assert!(cmd.contains("chromium --app=\"https://app.slack.com\""));
        assert!(cmd.contains("--class=\"webapp-slack\""));
    }

    #[test]
    fn test_desktop_entry_generation() {
        let mut app = WebAppConfig::new("youtube", "YouTube", "https://youtube.com");
        app.category = WebAppCategory::AudioVideo;
        let entry = app.generate_desktop_entry();
        assert!(entry.contains("Name=YouTube"));
        assert!(entry.contains("StartupWMClass=webapp-youtube"));
        assert!(entry.contains("Categories=AudioVideo;"));
    }

    #[test]
    fn test_manager_crud() {
        let mut manager = WebAppManager::new();
        assert_eq!(manager.count(), 0);

        let app1 = WebAppConfig::new("notion", "Notion", "https://notion.so");
        let app2 = WebAppConfig::new("linear", "Linear", "https://linear.app");

        assert!(manager.add_app(app1).is_ok());
        assert!(manager.add_app(app2).is_ok());
        assert_eq!(manager.count(), 2);

        assert!(manager.get_app("notion").is_some());
        assert_eq!(manager.get_app("notion").unwrap().name, "Notion");

        let removed = manager.remove_app("notion");
        assert!(removed.is_some());
        assert_eq!(manager.count(), 1);
        assert!(manager.get_app("notion").is_none());
    }

    #[test]
    fn test_url_validation() {
        let mut manager = WebAppManager::new();
        let invalid_app = WebAppConfig::new("bad", "Bad", "ftp://invalid-url.com");
        assert!(manager.add_app(invalid_app).is_err());
    }
}
