// SPDX-License-Identifier: MIT
// SigmaOS - Themes Manager
// Inspired by GNOME Tweaks and Unity Tweak Tool
// Provides theme and appearance management

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Theme type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeType {
    GTK,
    Icon,
    Cursor,
    Shell,
    Sound,
}

/// Color scheme
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorScheme {
    Light,
    Dark,
    Adaptable,
}

/// Accent color
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccentColor {
    Blue,
    Teal,
    Green,
    Yellow,
    Orange,
    Red,
    Pink,
    Purple,
    Slate,
}

/// Theme
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub theme_type: ThemeType,
    pub path: String,
    pub is_installed: bool,
    pub is_default: bool,
}

/// Appearance settings
#[derive(Debug, Clone)]
pub struct AppearanceSettings {
    pub gtk_theme: String,
    pub icon_theme: String,
    pub cursor_theme: String,
    pub shell_theme: String,
    pub color_scheme: ColorScheme,
    pub accent_color: AccentColor,
    pub font_name: String,
    pub document_font_name: String,
    pub monospace_font_name: String,
    pub font_size: u32,
}

/// Themes manager
pub struct ThemesManager {
    pub themes: BTreeMap<String, Vec<Theme>>,
    pub appearance: AppearanceSettings,
    pub available_gtk_themes: Vec<String>,
    pub available_icon_themes: Vec<String>,
    pub available_cursor_themes: Vec<String>,
}

impl ThemesManager {
    pub fn new() -> Self {
        Self {
            themes: BTreeMap::new(),
            appearance: AppearanceSettings {
                gtk_theme: "Adwaita".to_string(),
                icon_theme: "Adwaita".to_string(),
                cursor_theme: "Adwaita".to_string(),
                shell_theme: "Adwaita".to_string(),
                color_scheme: ColorScheme::Adaptable,
                accent_color: AccentColor::Blue,
                font_name: "Sans 11".to_string(),
                document_font_name: "Sans 11".to_string(),
                monospace_font_name: "Monospace 11".to_string(),
                font_size: 11,
            },
            available_gtk_themes: vec![
                "Adwaita".to_string(),
                "Yaru".to_string(),
                "Pop".to_string(),
            ],
            available_icon_themes: vec![
                "Adwaita".to_string(),
                "Yaru".to_string(),
                "Pop".to_string(),
            ],
            available_cursor_themes: vec!["Adwaita".to_string(), "Yaru".to_string()],
        }
    }

    /// Add theme
    pub fn add_theme(&mut self, name: String, theme_type: ThemeType, path: String) {
        let theme = Theme {
            name: name.clone(),
            theme_type,
            path,
            is_installed: true,
            is_default: false,
        };
        self.themes
            .entry(name.clone())
            .or_insert_with(Vec::new)
            .push(theme);
    }

    /// Remove theme
    pub fn remove_theme(&mut self, name: String) -> Result<(), &'static str> {
        if !self.themes.contains_key(&name) {
            return Err("Theme not found");
        }
        self.themes.remove(&name);
        Ok(())
    }

    /// Set GTK theme
    pub fn set_gtk_theme(&mut self, theme: String) -> Result<(), &'static str> {
        if !self.available_gtk_themes.contains(&theme) {
            return Err("Theme not available");
        }
        self.appearance.gtk_theme = theme;
        Ok(())
    }

    /// Set icon theme
    pub fn set_icon_theme(&mut self, theme: String) -> Result<(), &'static str> {
        if !self.available_icon_themes.contains(&theme) {
            return Err("Theme not available");
        }
        self.appearance.icon_theme = theme;
        Ok(())
    }

    /// Set cursor theme
    pub fn set_cursor_theme(&mut self, theme: String) -> Result<(), &'static str> {
        if !self.available_cursor_themes.contains(&theme) {
            return Err("Theme not available");
        }
        self.appearance.cursor_theme = theme;
        Ok(())
    }

    /// Set color scheme
    pub fn set_color_scheme(&mut self, scheme: ColorScheme) {
        self.appearance.color_scheme = scheme;
    }

    /// Set accent color
    pub fn set_accent_color(&mut self, color: AccentColor) {
        self.appearance.accent_color = color;
    }

    /// Set font
    pub fn set_font(&mut self, name: String, size: u32) {
        self.appearance.font_name = name.clone();
        self.appearance.document_font_name = name.clone();
        self.appearance.font_size = size;
    }

    /// Set monospace font
    pub fn set_monospace_font(&mut self, name: String) {
        self.appearance.monospace_font_name = name;
    }

    /// Get appearance settings
    pub fn get_appearance(&self) -> &AppearanceSettings {
        &self.appearance
    }

    /// Get themes by type
    pub fn get_themes_by_type(&self, theme_type: ThemeType) -> Vec<&Theme> {
        self.themes
            .values()
            .flatten()
            .filter(|t| t.theme_type == theme_type)
            .collect()
    }

    /// Get available GTK themes
    pub fn get_available_gtk_themes(&self) -> &[String] {
        &self.available_gtk_themes
    }

    /// Get available icon themes
    pub fn get_available_icon_themes(&self) -> &[String] {
        &self.available_icon_themes
    }

    /// Get available cursor themes
    pub fn get_available_cursor_themes(&self) -> &[String] {
        &self.available_cursor_themes
    }
}

/// Global themes manager
static mut THEMES_MANAGER: Option<ThemesManager> = None;

/// Initialize themes manager
pub fn init_themes_manager() {
    unsafe {
        THEMES_MANAGER = Some(ThemesManager::new());
    }
}

/// Get themes manager
pub fn get_themes_manager() -> &'static mut ThemesManager {
    unsafe {
        THEMES_MANAGER
            .as_mut()
            .expect("Themes manager not initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_themes_manager_creation() {
        let tm = ThemesManager::new();
        assert_eq!(tm.available_gtk_themes.len(), 3);
    }

    #[test]
    fn test_set_gtk_theme() {
        let mut tm = ThemesManager::new();
        tm.set_gtk_theme("Yaru".to_string()).unwrap();
        assert_eq!(tm.appearance.gtk_theme, "Yaru");
    }

    #[test]
    fn test_set_color_scheme() {
        let mut tm = ThemesManager::new();
        tm.set_color_scheme(ColorScheme::Dark);
        assert_eq!(tm.appearance.color_scheme, ColorScheme::Dark);
    }
}
