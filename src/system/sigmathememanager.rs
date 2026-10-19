//! SigmaThemeManager - Theme and Appearance Management System
//!
//! Provides comprehensive theme and appearance management inspired by Linux Mint and Omarchy.
//! Supports themes, icons, cursors, fonts, and appearance customization.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Theme type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeType {
    GTK,
    Qt,
    WindowManager,
    Icon,
    Cursor,
    Font,
}

/// Color scheme
#[derive(Debug, Clone)]
pub struct ColorScheme {
    pub id: String,
    pub name: String,
    pub background: String, // hex color
    pub foreground: String,
    pub accent: String,
    pub selection: String,
    pub window_bg: String,
    pub window_fg: String,
}

impl ColorScheme {
    pub fn new(id: String, name: String, background: String, foreground: String, accent: String) -> Self {
        Self {
            id,
            name,
            background,
            foreground,
            accent,
            selection: accent.clone(),
            window_bg: background.clone(),
            window_fg: foreground.clone(),
        }
    }

    pub fn set_selection(&mut self, color: String) {
        self.selection = color;
    }

    pub fn set_window_bg(&mut self, color: String) {
        self.window_bg = color;
    }

    pub fn set_window_fg(&mut self, color: String) {
        self.window_fg = color;
    }
}

/// Theme
#[derive(Debug, Clone)]
pub struct Theme {
    pub id: String,
    pub name: String,
    pub theme_type: ThemeType,
    pub path: String,
    pub is_dark: bool,
    pub color_scheme: Option<ColorScheme>,
    pub enabled: bool,
}

impl Theme {
    pub fn new(id: String, name: String, theme_type: ThemeType, path: String, is_dark: bool) -> Self {
        Self {
            id,
            name,
            theme_type,
            path,
            is_dark,
            color_scheme: None,
            enabled: true,
        }
    }

    pub fn set_color_scheme(&mut self, scheme: ColorScheme) {
        self.color_scheme = Some(scheme);
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

/// Font configuration
#[derive(Debug, Clone)]
pub struct FontConfig {
    pub id: String,
    pub name: String,
    pub font_family: String,
    pub size: u32, // in points
    pub weight: String, // normal, bold, light
    pub style: String, // normal, italic
    pub antialiasing: bool,
    pub hinting: bool,
}

impl FontConfig {
    pub fn new(id: String, name: String, font_family: String, size: u32) -> Self {
        Self {
            id,
            name,
            font_family,
            size,
            weight: "normal".to_string(),
            style: "normal".to_string(),
            antialiasing: true,
            hinting: true,
        }
    }

    pub fn set_weight(&mut self, weight: String) {
        self.weight = weight;
    }

    pub fn set_style(&mut self, style: String) {
        self.style = style;
    }

    pub fn set_antialiasing(&mut self, enabled: bool) {
        self.antialiasing = enabled;
    }

    pub fn set_hinting(&mut self, enabled: bool) {
        self.hinting = enabled;
    }
}

/// Appearance settings
#[derive(Debug, Clone)]
pub struct AppearanceSettings {
    pub id: String,
    pub gtk_theme: Option<String>,
    pub qt_theme: Option<String>,
    pub icon_theme: Option<String>,
    pub cursor_theme: Option<String>,
    pub font_config: Option<FontConfig>,
    pub dark_mode: bool,
    pub accent_color: String,
}

impl AppearanceSettings {
    pub fn new(id: String) -> Self {
        Self {
            id,
            gtk_theme: None,
            qt_theme: None,
            icon_theme: None,
            cursor_theme: None,
            font_config: None,
            dark_mode: false,
            accent_color: "#3daee9".to_string(),
        }
    }

    pub fn set_gtk_theme(&mut self, theme: Option<String>) {
        self.gtk_theme = theme;
    }

    pub fn set_qt_theme(&mut self, theme: Option<String>) {
        self.qt_theme = theme;
    }

    pub fn set_icon_theme(&mut self, theme: Option<String>) {
        self.icon_theme = theme;
    }

    pub fn set_cursor_theme(&mut self, theme: Option<String>) {
        self.cursor_theme = theme;
    }

    pub fn set_font_config(&mut self, config: Option<FontConfig>) {
        self.font_config = config;
    }

    pub fn set_dark_mode(&mut self, dark: bool) {
        self.dark_mode = dark;
    }

    pub fn set_accent_color(&mut self, color: String) {
        self.accent_color = color;
    }
}

/// SigmaThemeManager - Theme and appearance management
#[derive(Debug)]
pub struct SigmaThemeManager {
    themes: BTreeMap<String, Theme>,
    appearance: AppearanceSettings,
}

impl SigmaThemeManager {
    pub fn new() -> Self {
        Self {
            themes: BTreeMap::new(),
            appearance: AppearanceSettings::new("default".to_string()),
        }
    }

    pub fn add_theme(&mut self, theme: Theme) {
        self.themes.insert(theme.id.clone(), theme);
    }

    pub fn remove_theme(&mut self, id: &str) -> bool {
        self.themes.remove(id).is_some()
    }

    pub fn get_theme(&self, id: &str) -> Option<&Theme> {
        self.themes.get(id)
    }

    pub fn get_theme_mut(&mut self, id: &str) -> Option<&mut Theme> {
        self.themes.get_mut(id)
    }

    pub fn list_by_type(&self, theme_type: ThemeType) -> Vec<&Theme> {
        self.themes.values()
            .filter(|theme| theme.theme_type == theme_type)
            .collect()
    }

    pub fn list_dark(&self) -> Vec<&Theme> {
        self.themes.values()
            .filter(|theme| theme.is_dark)
            .collect()
    }

    pub fn list_light(&self) -> Vec<&Theme> {
        self.themes.values()
            .filter(|theme| !theme.is_dark)
            .collect()
    }

    pub fn get_appearance(&self) -> &AppearanceSettings {
        &self.appearance
    }

    pub fn get_appearance_mut(&mut self) -> &mut AppearanceSettings {
        &mut self.appearance
    }

    pub fn theme_count(&self) -> usize {
        self.themes.len()
    }
}

impl Default for SigmaThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaThemeManager instance
static mut SIGMATHEMEMANAGER_INSTANCE: Option<SigmaThemeManager> = None;

/// Initialize the global SigmaThemeManager instance
pub fn init_sigmathememanager() {
    unsafe {
        SIGMATHEMEMANAGER_INSTANCE = Some(SigmaThemeManager::new());
    }
}

/// Get the global SigmaThemeManager instance
pub fn get_sigmathememanager() -> Option<&'static SigmaThemeManager> {
    unsafe {
        SIGMATHEMEMANAGER_INSTANCE.as_ref()
    }
}

/// Get the global SigmaThemeManager instance (mutable)
pub fn get_sigmathememanager_mut() -> Option<&'static mut SigmaThemeManager> {
    unsafe {
        SIGMATHEMEMANAGER_INSTANCE.as_mut()
    }
}

/// Convenience function to add a theme
pub fn add_theme(theme: Theme) -> bool {
    if let Some(manager) = get_sigmathememanager_mut() {
        manager.add_theme(theme);
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_scheme_creation() {
        let scheme = ColorScheme::new(
            "scheme1".to_string(),
            "Dark".to_string(),
            "#1e1e1e".to_string(),
            "#ffffff".to_string(),
            "#3daee9".to_string(),
        );
        assert_eq!(scheme.id, "scheme1");
        assert_eq!(scheme.background, "#1e1e1e");
    }

    #[test]
    fn test_theme_creation() {
        let theme = Theme::new(
            "theme1".to_string(),
            "Adwaita Dark".to_string(),
            ThemeType::GTK,
            "/usr/share/themes".to_string(),
            true,
        );
        assert_eq!(theme.id, "theme1");
        assert!(theme.is_dark);
    }

    #[test]
    fn test_font_config_creation() {
        let font = FontConfig::new(
            "font1".to_string(),
            "Default".to_string(),
            "Noto Sans".to_string(),
            11,
        );
        assert_eq!(font.font_family, "Noto Sans");
        assert_eq!(font.size, 11);
    }

    #[test]
    fn test_appearance_settings_creation() {
        let settings = AppearanceSettings::new("default".to_string());
        assert!(!settings.dark_mode);
        assert_eq!(settings.accent_color, "#3daee9");
    }

    #[test]
    fn test_appearance_settings_modification() {
        let mut settings = AppearanceSettings::new("default".to_string());
        settings.set_dark_mode(true);
        settings.set_gtk_theme(Some("Adwaita-dark".to_string()));
        settings.set_accent_color("#ff6b6b".to_string());
        assert!(settings.dark_mode);
        assert_eq!(settings.gtk_theme, Some("Adwaita-dark".to_string()));
    }

    #[test]
    fn test_sigmathememanager_creation() {
        let manager = SigmaThemeManager::new();
        assert_eq!(manager.theme_count(), 0);
    }

    #[test]
    fn test_sigmathememanager_management() {
        let mut manager = SigmaThemeManager::new();
        let theme = Theme::new(
            "theme1".to_string(),
            "Test".to_string(),
            ThemeType::GTK,
            "/path".to_string(),
            false,
        );
        manager.add_theme(theme);
        assert_eq!(manager.theme_count(), 1);
    }

    #[test]
    fn test_dark_theme_filtering() {
        let mut manager = SigmaThemeManager::new();
        let dark_theme = Theme::new(
            "dark".to_string(),
            "Dark".to_string(),
            ThemeType::GTK,
            "/path".to_string(),
            true,
        );
        let light_theme = Theme::new(
            "light".to_string(),
            "Light".to_string(),
            ThemeType::GTK,
            "/path".to_string(),
            false,
        );
        manager.add_theme(dark_theme);
        manager.add_theme(light_theme);
        let dark = manager.list_dark();
        assert_eq!(dark.len(), 1);
    }
}
