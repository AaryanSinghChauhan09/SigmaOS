// SPDX-License-Identifier: MIT
// SigmaOS - Theme Manager
// Theme management inspired by Omarchy theme structure and Linux theme systems

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Theme color
#[derive(Debug, Clone)]
pub struct ThemeColor {
    pub name: String,
    pub value: String,
}

/// Theme
#[derive(Debug, Clone)]
pub struct Theme {
    pub id: String,
    pub name: String,
    pub colors: Vec<ThemeColor>,
    pub accent: String,
    pub background: String,
    pub foreground: String,
}

/// Theme manager
pub struct SigmaThemeManager {
    themes: Vec<Theme>,
    active_theme: Option<String>,
}

impl SigmaThemeManager {
    pub fn new() -> Self {
        Self {
            themes: Vec::new(),
            active_theme: None,
        }
    }

    pub fn add_theme(&mut self, theme: Theme) {
        self.themes.push(theme);
    }

    pub fn get_themes(&self) -> &[Theme] {
        &self.themes
    }

    pub fn get_theme(&self, id: &str) -> Option<&Theme> {
        self.themes.iter().find(|t| t.id == id)
    }

    pub fn set_active_theme(&mut self, id: String) -> Result<(), String> {
        if self.get_theme(&id).is_some() {
            self.active_theme = Some(id);
            Ok(())
        } else {
            Err(format!("Theme {} not found", id))
        }
    }

    pub fn get_active_theme(&self) -> Option<&Theme> {
        if let Some(ref id) = self.active_theme {
            self.get_theme(id)
        } else {
            None
        }
    }

    pub fn remove_theme(&mut self, id: String) -> Result<(), String> {
        if let Some(idx) = self.themes.iter().position(|t| t.id == id) {
            if self.active_theme.as_ref() == Some(&id) {
                self.active_theme = None;
            }
            self.themes.remove(idx);
            Ok(())
        } else {
            Err(format!("Theme {} not found", id))
        }
    }

    pub fn get_theme_color(&self, theme_id: &str, color_name: &str) -> Option<&String> {
        self.get_theme(theme_id)
            .and_then(|t| t.colors.iter().find(|c| c.name == color_name))
            .map(|c| &c.value)
    }
}

impl Default for SigmaThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_manager_creation() {
        let manager = SigmaThemeManager::new();
        assert_eq!(manager.get_themes().len(), 0);
        assert!(manager.get_active_theme().is_none());
    }

    #[test]
    fn test_add_theme() {
        let mut manager = SigmaThemeManager::new();
        let theme = Theme {
            id: "dark".to_string(),
            name: "Dark".to_string(),
            colors: Vec::new(),
            accent: "#007acc".to_string(),
            background: "#1e1e1e".to_string(),
            foreground: "#d4d4d4".to_string(),
        };
        manager.add_theme(theme);
        assert_eq!(manager.get_themes().len(), 1);
    }

    #[test]
    fn test_set_active_theme() {
        let mut manager = SigmaThemeManager::new();
        let theme = Theme {
            id: "dark".to_string(),
            name: "Dark".to_string(),
            colors: Vec::new(),
            accent: "#007acc".to_string(),
            background: "#1e1e1e".to_string(),
            foreground: "#d4d4d4".to_string(),
        };
        manager.add_theme(theme);
        manager.set_active_theme("dark".to_string()).unwrap();
        assert!(manager.get_active_theme().is_some());
    }

    #[test]
    fn test_remove_theme() {
        let mut manager = SigmaThemeManager::new();
        let theme = Theme {
            id: "dark".to_string(),
            name: "Dark".to_string(),
            colors: Vec::new(),
            accent: "#007acc".to_string(),
            background: "#1e1e1e".to_string(),
            foreground: "#d4d4d4".to_string(),
        };
        manager.add_theme(theme);
        manager.remove_theme("dark".to_string()).unwrap();
        assert_eq!(manager.get_themes().len(), 0);
    }

    #[test]
    fn test_get_theme_color() {
        let mut manager = SigmaThemeManager::new();
        let mut colors = Vec::new();
        colors.push(ThemeColor {
            name: "red".to_string(),
            value: "#ff0000".to_string(),
        });
        let theme = Theme {
            id: "dark".to_string(),
            name: "Dark".to_string(),
            colors,
            accent: "#007acc".to_string(),
            background: "#1e1e1e".to_string(),
            foreground: "#d4d4d4".to_string(),
        };
        manager.add_theme(theme);
        assert_eq!(
            manager.get_theme_color("dark", "red"),
            Some(&"#ff0000".to_string())
        );
    }
}
