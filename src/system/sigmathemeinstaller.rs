//! SigmaThemeInstaller - Theme Installation Manager
//!
//! Inspired by Linux Mint's theme management, this module provides comprehensive
//! theme installation including icon themes, cursor themes, and GTK themes.

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, Ordering};

/// Theme type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeType {
    GTK,
    Icon,
    Cursor,
    WindowBorder,
    Metacity,
    Unknown,
}

/// Theme state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeState {
    Installed,
    NotInstalled,
    PartiallyInstalled,
    Unknown,
}

/// Theme data structure
#[derive(Debug, Clone)]
pub struct Theme {
    pub theme_id: String,
    pub theme_name: String,
    pub theme_type: ThemeType,
    pub version: String,
    pub author: String,
    pub description: String,
    pub state: ThemeState,
    pub path: String,
    pub preview_image: String,
}

impl Theme {
    /// Create a new theme
    pub fn new(
        theme_id: String,
        theme_name: String,
        theme_type: ThemeType,
        version: String,
        author: String,
        description: String,
    ) -> Self {
        Self {
            theme_id,
            theme_name,
            theme_type,
            version,
            author,
            description,
            state: ThemeState::NotInstalled,
            path: String::new(),
            preview_image: String::new(),
        }
    }

    /// Set state
    pub fn set_state(&mut self, state: ThemeState) {
        self.state = state;
    }

    /// Set path
    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }

    /// Set preview image
    pub fn set_preview_image(&mut self, preview_image: String) {
        self.preview_image = preview_image;
    }

    /// Check if theme is installed
    pub fn is_installed(&self) -> bool {
        self.state == ThemeState::Installed
    }

    /// Check if theme is partially installed
    pub fn is_partially_installed(&self) -> bool {
        self.state == ThemeState::PartiallyInstalled
    }
}

/// Theme manager
pub struct SigmaThemeInstaller {
    themes: BTreeMap<String, Theme>,
    running: AtomicBool,
}

impl SigmaThemeInstaller {
    /// Create a new theme installer manager
    pub fn new() -> Self {
        Self {
            themes: BTreeMap::new(),
            running: AtomicBool::new(false),
        }
    }

    /// Start the theme installer manager
    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
    }

    /// Stop the theme installer manager
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Add a theme
    pub fn add_theme(&mut self, theme: Theme) -> bool {
        let theme_id = theme.theme_id.clone();
        self.themes.insert(theme_id, theme);
        true
    }

    /// Remove a theme
    pub fn remove_theme(&mut self, theme_id: &str) -> Option<Theme> {
        self.themes.remove(theme_id)
    }

    /// Get a theme by ID
    pub fn get_theme(&self, theme_id: &str) -> Option<&Theme> {
        self.themes.get(theme_id)
    }

    /// List all themes
    pub fn list_themes(&self) -> Vec<&Theme> {
        self.themes.values().collect()
    }

    /// List themes by type
    pub fn list_themes_by_type(&self, theme_type: ThemeType) -> Vec<&Theme> {
        self.themes
            .values()
            .filter(|t| t.theme_type == theme_type)
            .collect()
    }

    /// List installed themes
    pub fn list_installed_themes(&self) -> Vec<&Theme> {
        self.themes.values().filter(|t| t.is_installed()).collect()
    }

    /// List themes by type and state
    pub fn list_themes_by_type_and_state(
        &self,
        theme_type: ThemeType,
        state: ThemeState,
    ) -> Vec<&Theme> {
        self.themes
            .values()
            .filter(|t| t.theme_type == theme_type && t.state == state)
            .collect()
    }

    /// Install a theme
    pub fn install_theme(&mut self, theme_id: &str) -> bool {
        if let Some(theme) = self.themes.get_mut(theme_id) {
            theme.set_state(ThemeState::Installed);
            true
        } else {
            false
        }
    }

    /// Uninstall a theme
    pub fn uninstall_theme(&mut self, theme_id: &str) -> bool {
        if let Some(theme) = self.themes.get_mut(theme_id) {
            theme.set_state(ThemeState::NotInstalled);
            true
        } else {
            false
        }
    }

    /// Set theme as partially installed
    pub fn set_partially_installed(&mut self, theme_id: &str) -> bool {
        if let Some(theme) = self.themes.get_mut(theme_id) {
            theme.set_state(ThemeState::PartiallyInstalled);
            true
        } else {
            false
        }
    }

    /// Search themes by name
    pub fn search_themes(&self, query: &str) -> Vec<&Theme> {
        let query_lower = query.to_lowercase();
        self.themes
            .values()
            .filter(|t| {
                t.theme_name.to_lowercase().contains(&query_lower)
                    || t.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Get theme count
    pub fn theme_count(&self) -> usize {
        self.themes.len()
    }

    /// Get installed theme count
    pub fn installed_count(&self) -> usize {
        self.list_installed_themes().len()
    }
}

impl Default for SigmaThemeInstaller {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaThemeInstaller instance (placeholder for actual global state)
static mut SIGMATHEMEINSTALLER_INSTANCE: Option<SigmaThemeInstaller> = None;

/// Initialize the global SigmaThemeInstaller instance
pub fn init_sigmathemeinstaller() {
    unsafe {
        SIGMATHEMEINSTALLER_INSTANCE = Some(SigmaThemeInstaller::new());
    }
}

/// Get the global SigmaThemeInstaller instance
pub fn get_sigmathemeinstaller() -> Option<&'static SigmaThemeInstaller> {
    unsafe { SIGMATHEMEINSTALLER_INSTANCE.as_ref() }
}

/// Get mutable global SigmaThemeInstaller instance
pub fn get_sigmathemeinstaller_mut() -> Option<&'static mut SigmaThemeInstaller> {
    unsafe { SIGMATHEMEINSTALLER_INSTANCE.as_mut() }
}

/// Convenience function to add a theme
pub fn add_theme(theme: Theme) -> bool {
    if let Some(theme_mgr) = get_sigmathemeinstaller_mut() {
        theme_mgr.add_theme(theme)
    } else {
        false
    }
}

/// Convenience function to install a theme
pub fn install_theme(theme_id: &str) -> bool {
    if let Some(theme_mgr) = get_sigmathemeinstaller_mut() {
        theme_mgr.install_theme(theme_id)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_creation() {
        let theme = Theme::new(
            "theme1".to_string(),
            "Mint-Y".to_string(),
            ThemeType::GTK,
            "1.0".to_string(),
            "Linux Mint".to_string(),
            "Default Mint theme".to_string(),
        );

        assert_eq!(theme.theme_id, "theme1");
        assert_eq!(theme.theme_type, ThemeType::GTK);
        assert_eq!(theme.state, ThemeState::NotInstalled);
    }

    #[test]
    fn test_theme_state() {
        let mut theme = Theme::new(
            "theme1".to_string(),
            "Mint-Y".to_string(),
            ThemeType::Icon,
            "1.0".to_string(),
            "Linux Mint".to_string(),
            "Mint icon theme".to_string(),
        );

        assert!(!theme.is_installed());
        theme.set_state(ThemeState::Installed);
        assert!(theme.is_installed());
    }

    #[test]
    fn test_sigmathemeinstaller_creation() {
        let theme_mgr = SigmaThemeInstaller::new();
        assert!(!theme_mgr.is_running());
        assert_eq!(theme_mgr.theme_count(), 0);
    }

    #[test]
    fn test_sigmathemeinstaller_start_stop() {
        let mut theme_mgr = SigmaThemeInstaller::new();
        theme_mgr.start();
        assert!(theme_mgr.is_running());
        theme_mgr.stop();
        assert!(!theme_mgr.is_running());
    }

    #[test]
    fn test_sigmathemeinstaller_add_theme() {
        let mut theme_mgr = SigmaThemeInstaller::new();
        let theme = Theme::new(
            "theme1".to_string(),
            "Mint-Y".to_string(),
            ThemeType::GTK,
            "1.0".to_string(),
            "Linux Mint".to_string(),
            "Default Mint theme".to_string(),
        );

        let added = theme_mgr.add_theme(theme);
        assert!(added);
        assert_eq!(theme_mgr.theme_count(), 1);
    }

    #[test]
    fn test_sigmathemeinstaller_list_by_type() {
        let mut theme_mgr = SigmaThemeInstaller::new();

        theme_mgr.add_theme(Theme::new(
            "gtk1".to_string(),
            "Mint-Y".to_string(),
            ThemeType::GTK,
            "1.0".to_string(),
            "Linux Mint".to_string(),
            "GTK theme".to_string(),
        ));

        theme_mgr.add_theme(Theme::new(
            "icon1".to_string(),
            "Mint-X".to_string(),
            ThemeType::Icon,
            "1.0".to_string(),
            "Linux Mint".to_string(),
            "Icon theme".to_string(),
        ));

        let gtk_themes = theme_mgr.list_themes_by_type(ThemeType::GTK);
        assert_eq!(gtk_themes.len(), 1);

        let icon_themes = theme_mgr.list_themes_by_type(ThemeType::Icon);
        assert_eq!(icon_themes.len(), 1);
    }

    #[test]
    fn test_sigmathemeinstaller_install_theme() {
        let mut theme_mgr = SigmaThemeInstaller::new();
        let theme = Theme::new(
            "theme1".to_string(),
            "Mint-Y".to_string(),
            ThemeType::GTK,
            "1.0".to_string(),
            "Linux Mint".to_string(),
            "Default Mint theme".to_string(),
        );

        theme_mgr.add_theme(theme);
        let installed = theme_mgr.install_theme("theme1");
        assert!(installed);

        let theme = theme_mgr.get_theme("theme1").unwrap();
        assert!(theme.is_installed());
    }

    #[test]
    fn test_sigmathemeinstaller_uninstall_theme() {
        let mut theme_mgr = SigmaThemeInstaller::new();
        let theme = Theme::new(
            "theme1".to_string(),
            "Mint-Y".to_string(),
            ThemeType::GTK,
            "1.0".to_string(),
            "Linux Mint".to_string(),
            "Default Mint theme".to_string(),
        );

        theme_mgr.add_theme(theme);
        theme_mgr.install_theme("theme1");
        let uninstalled = theme_mgr.uninstall_theme("theme1");
        assert!(uninstalled);

        let theme = theme_mgr.get_theme("theme1").unwrap();
        assert!(!theme.is_installed());
    }

    #[test]
    fn test_sigmathemeinstaller_search_themes() {
        let mut theme_mgr = SigmaThemeInstaller::new();

        theme_mgr.add_theme(Theme::new(
            "gtk1".to_string(),
            "Mint-Y".to_string(),
            ThemeType::GTK,
            "1.0".to_string(),
            "Linux Mint".to_string(),
            "Mint green theme".to_string(),
        ));

        theme_mgr.add_theme(Theme::new(
            "gtk2".to_string(),
            "Mint-X".to_string(),
            ThemeType::GTK,
            "1.0".to_string(),
            "Linux Mint".to_string(),
            "Mint dark theme".to_string(),
        ));

        let results = theme_mgr.search_themes("mint");
        assert_eq!(results.len(), 2);

        let results = theme_mgr.search_themes("green");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_global_instance() {
        init_sigmathemeinstaller();

        let theme_mgr = get_sigmathemeinstaller();
        assert!(theme_mgr.is_some());

        let mut theme_mgr_mut = get_sigmathemeinstaller_mut();
        assert!(theme_mgr_mut.is_some());

        if let Some(theme_mgr) = theme_mgr_mut {
            theme_mgr.start();
            assert!(theme_mgr.is_running());
        }
    }

    #[test]
    fn test_convenience_add_theme() {
        init_sigmathemeinstaller();

        let theme = Theme::new(
            "theme1".to_string(),
            "Mint-Y".to_string(),
            ThemeType::GTK,
            "1.0".to_string(),
            "Linux Mint".to_string(),
            "Default Mint theme".to_string(),
        );

        let added = add_theme(theme);
        assert!(added);

        let theme_mgr = get_sigmathemeinstaller().unwrap();
        assert_eq!(theme_mgr.theme_count(), 1);
    }

    #[test]
    fn test_convenience_install_theme() {
        init_sigmathemeinstaller();

        let theme = Theme::new(
            "theme1".to_string(),
            "Mint-Y".to_string(),
            ThemeType::GTK,
            "1.0".to_string(),
            "Linux Mint".to_string(),
            "Default Mint theme".to_string(),
        );

        add_theme(theme);
        let installed = install_theme("theme1");
        assert!(installed);

        let theme_mgr = get_sigmathemeinstaller().unwrap();
        let theme = theme_mgr.get_theme("theme1").unwrap();
        assert!(theme.is_installed());
    }
}
