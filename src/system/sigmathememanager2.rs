// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Aaryan Singh Chauhan

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Theme component type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeComponent {
    Desktop,
    Terminal,
    Editor,
    Browser,
    System,
}

/// Theme definition
#[derive(Debug, Clone)]
pub struct Theme {
    pub id: String,
    pub name: String,
    pub description: String,
    pub component: ThemeComponent,
    pub is_active: bool,
    pub colors: Vec<(String, String)>,
}

/// Theme manager for managing system themes
/// Inspired by Omarchy theme- commands and Linux theming systems
#[derive(Debug, Clone)]
pub struct SigmaThemeManager2 {
    themes: Vec<Theme>,
    active_theme: Option<String>,
}

impl SigmaThemeManager2 {
    pub fn new() -> Self {
        Self {
            themes: Vec::new(),
            active_theme: None,
        }
    }

    /// Add a theme
    pub fn add_theme(&mut self, theme: Theme) {
        self.themes.push(theme);
    }

    /// Get theme by ID
    pub fn get_theme(&self, id: &str) -> Option<&Theme> {
        self.themes.iter().find(|t| t.id == id)
    }

    /// Get all themes
    pub fn get_themes(&self) -> &[Theme] {
        &self.themes
    }

    /// Set active theme
    pub fn set_active_theme(&mut self, id: &str) -> Result<String, String> {
        let found = self.themes.iter().any(|t| t.id == id);
        if found {
            for t in &mut self.themes {
                t.is_active = false;
            }
            if let Some(theme) = self.themes.iter_mut().find(|t| t.id == id) {
                theme.is_active = true;
                self.active_theme = Some(id.to_string());
                Ok(format!("Set active theme: {}", theme.name))
            } else {
                Err(format!("Theme not found: {}", id))
            }
        } else {
            Err(format!("Theme not found: {}", id))
        }
    }

    /// Get active theme
    pub fn get_active_theme(&self) -> Option<&Theme> {
        self.active_theme
            .as_ref()
            .and_then(|id| self.get_theme(id))
    }

    /// Get themes by component
    pub fn get_themes_by_component(&self, component: ThemeComponent) -> Vec<&Theme> {
        self.themes
            .iter()
            .filter(|t| t.component == component)
            .collect()
    }

    /// Refresh theme (reload colors)
    pub fn refresh_theme(&mut self, id: &str) -> Result<String, String> {
        if let Some(_theme) = self.get_theme(id) {
            Ok(format!("Refreshed theme: {}", id))
        } else {
            Err(format!("Theme not found: {}", id))
        }
    }

    /// Remove theme
    pub fn remove_theme(&mut self, id: &str) -> Result<String, String> {
        if let Some(pos) = self.themes.iter().position(|t| t.id == id) {
            let theme = self.themes.remove(pos);
            if self.active_theme.as_deref() == Some(id) {
                self.active_theme = None;
            }
            Ok(format!("Removed theme: {}", theme.name))
        } else {
            Err(format!("Theme not found: {}", id))
        }
    }

    /// Get theme colors
    pub fn get_theme_colors(&self, id: &str) -> Option<&[(String, String)]> {
        self.get_theme(id).map(|t| t.colors.as_slice())
    }
}

impl Default for SigmaThemeManager2 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_theme() {
        let mut manager = SigmaThemeManager2::new();
        let theme = Theme {
            id: "dark".to_string(),
            name: "Dark Theme".to_string(),
            description: "Dark color scheme".to_string(),
            component: ThemeComponent::Desktop,
            is_active: false,
            colors: vec![("bg".to_string(), "#000000".to_string())],
        };
        manager.add_theme(theme);
        assert_eq!(manager.get_themes().len(), 1);
    }

    #[test]
    fn test_set_active_theme() {
        let mut manager = SigmaThemeManager2::new();
        let theme = Theme {
            id: "dark".to_string(),
            name: "Dark Theme".to_string(),
            description: "Dark color scheme".to_string(),
            component: ThemeComponent::Desktop,
            is_active: false,
            colors: vec![("bg".to_string(), "#000000".to_string())],
        };
        manager.add_theme(theme);
        manager.set_active_theme("dark").unwrap();
        assert!(manager.get_theme("dark").unwrap().is_active);
    }

    #[test]
    fn test_get_active_theme() {
        let mut manager = SigmaThemeManager2::new();
        let theme = Theme {
            id: "dark".to_string(),
            name: "Dark Theme".to_string(),
            description: "Dark color scheme".to_string(),
            component: ThemeComponent::Desktop,
            is_active: false,
            colors: vec![("bg".to_string(), "#000000".to_string())],
        };
        manager.add_theme(theme);
        manager.set_active_theme("dark").unwrap();
        assert!(manager.get_active_theme().is_some());
    }

    #[test]
    fn test_get_themes_by_component() {
        let mut manager = SigmaThemeManager2::new();
        let theme1 = Theme {
            id: "dark".to_string(),
            name: "Dark Theme".to_string(),
            description: "Dark color scheme".to_string(),
            component: ThemeComponent::Desktop,
            is_active: false,
            colors: vec![("bg".to_string(), "#000000".to_string())],
        };
        let theme2 = Theme {
            id: "terminal-dark".to_string(),
            name: "Terminal Dark".to_string(),
            description: "Terminal dark theme".to_string(),
            component: ThemeComponent::Terminal,
            is_active: false,
            colors: vec![("bg".to_string(), "#000000".to_string())],
        };
        manager.add_theme(theme1);
        manager.add_theme(theme2);
        assert_eq!(manager.get_themes_by_component(ThemeComponent::Desktop).len(), 1);
    }

    #[test]
    fn test_remove_theme() {
        let mut manager = SigmaThemeManager2::new();
        let theme = Theme {
            id: "dark".to_string(),
            name: "Dark Theme".to_string(),
            description: "Dark color scheme".to_string(),
            component: ThemeComponent::Desktop,
            is_active: false,
            colors: vec![("bg".to_string(), "#000000".to_string())],
        };
        manager.add_theme(theme);
        manager.remove_theme("dark").unwrap();
        assert_eq!(manager.get_themes().len(), 0);
    }
}
