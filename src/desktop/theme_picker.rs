#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(dead_code)]

// SigmaOS Theme Picker - Omarchy-inspired Theme Management System
// Coordinated theme system for terminal, editor, browser, wallpaper, and bar

use std::collections::HashMap;

/// Theme color scheme
#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub background: String,
    pub foreground: String,
    pub cursor: String,
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub error: String,
    pub warning: String,
    pub success: String,
    pub info: String,
}

/// Theme metadata
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub id: String,
    pub description: String,
    pub author: String,
    pub variant: String,
    pub colors: ThemeColors,
    pub wallpaper: Option<String>,
    pub supports_terminal: bool,
    pub supports_editor: bool,
    pub supports_browser: bool,
    pub supports_bar: bool,
}

/// Theme application target
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeTarget {
    Terminal,
    Editor,
    Browser,
    Bar,
    Wallpaper,
    All,
}

/// Theme application status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeStatus {
    Idle,
    Applying,
    Complete,
    Failed,
}

/// Theme picker configuration
#[derive(Debug, Clone)]
pub struct ThemePickerConfig {
    pub auto_apply: bool,
    pub save_history: bool,
    pub max_history: usize,
    pub confirm_apply: bool,
}

impl Default for ThemePickerConfig {
    fn default() -> Self {
        Self {
            auto_apply: true,
            save_history: true,
            max_history: 10,
            confirm_apply: false,
        }
    }
}

/// Theme picker manager
#[derive(Debug, Clone)]
pub struct ThemePickerManager {
    config: ThemePickerConfig,
    themes: HashMap<String, Theme>,
    current_theme: Option<String>,
    history: Vec<String>,
    status: ThemeStatus,
}

impl ThemePickerManager {
    pub fn new(config: ThemePickerConfig) -> Self {
        Self {
            config,
            themes: HashMap::new(),
            current_theme: None,
            history: Vec::new(),
            status: ThemeStatus::Idle,
        }
    }

    pub fn with_default_config() -> Self {
        Self::new(ThemePickerConfig::default())
    }

    /// Add theme
    pub fn add_theme(&mut self, theme: Theme) -> Result<(), String> {
        self.themes.insert(theme.id.clone(), theme);
        Ok(())
    }

    /// Remove theme
    pub fn remove_theme(&mut self, id: &str) -> Result<(), String> {
        self.themes.remove(id);
        Ok(())
    }

    /// Get theme
    pub fn get_theme(&self, id: &str) -> Option<&Theme> {
        self.themes.get(id)
    }

    /// Get all themes
    pub fn get_themes(&self) -> Vec<&Theme> {
        self.themes.values().collect()
    }

    /// Search themes
    pub fn search_themes(&self, query: &str) -> Vec<&Theme> {
        self.themes
            .values()
            .filter(|t| {
                t.name.to_lowercase().contains(&query.to_lowercase())
                    || t.description.to_lowercase().contains(&query.to_lowercase())
                    || t.author.to_lowercase().contains(&query.to_lowercase())
            })
            .collect()
    }

    /// Apply theme
    pub fn apply_theme(&mut self, id: &str, target: ThemeTarget) -> Result<(), String> {
        if !self.themes.contains_key(id) {
            return Err(format!("Theme {} not found", id));
        }

        self.status = ThemeStatus::Applying;

        let theme = self.themes.get(id).unwrap();

        // Apply to specified targets
        match target {
            ThemeTarget::Terminal => {
                if !theme.supports_terminal {
                    return Err("Theme does not support terminal".to_string());
                }
            }
            ThemeTarget::Editor => {
                if !theme.supports_editor {
                    return Err("Theme does not support editor".to_string());
                }
            }
            ThemeTarget::Browser => {
                if !theme.supports_browser {
                    return Err("Theme does not support browser".to_string());
                }
            }
            ThemeTarget::Bar => {
                if !theme.supports_bar {
                    return Err("Theme does not support bar".to_string());
                }
            }
            ThemeTarget::Wallpaper => {
                if theme.wallpaper.is_none() {
                    return Err("Theme does not have wallpaper".to_string());
                }
            }
            ThemeTarget::All => {
                // Apply to all supported targets
            }
        }

        // Update history
        if self.config.save_history {
            if let Some(current) = &self.current_theme {
                if !self.history.contains(current) {
                    self.history.push(current.clone());
                    if self.history.len() > self.config.max_history {
                        self.history.remove(0);
                    }
                }
            }
        }

        self.current_theme = Some(id.to_string());
        self.status = ThemeStatus::Complete;

        Ok(())
    }

    /// Get current theme
    pub fn get_current_theme(&self) -> Option<&Theme> {
        self.current_theme
            .as_ref()
            .and_then(|id| self.themes.get(id))
    }

    /// Get history
    pub fn get_history(&self) -> Vec<&Theme> {
        self.history
            .iter()
            .filter_map(|id| self.themes.get(id))
            .collect()
    }

    /// Apply previous theme
    pub fn apply_previous(&mut self) -> Result<(), String> {
        if let Some(prev_id) = self.history.pop() {
            self.apply_theme(&prev_id, ThemeTarget::All)
        } else {
            Err("No previous theme in history".to_string())
        }
    }

    /// Get theme status
    pub fn get_status(&self) -> ThemeStatus {
        self.status
    }

    /// Get theme by name
    pub fn get_theme_by_name(&self, name: &str) -> Option<&Theme> {
        self.themes.values().find(|t| t.name == name)
    }

    /// Get themes by author
    pub fn get_themes_by_author(&self, author: &str) -> Vec<&Theme> {
        self.themes
            .values()
            .filter(|t| t.author == author)
            .collect()
    }

    /// Get themes supporting specific target
    pub fn get_themes_supporting(&self, target: ThemeTarget) -> Vec<&Theme> {
        self.themes
            .values()
            .filter(|t| match target {
                ThemeTarget::Terminal => t.supports_terminal,
                ThemeTarget::Editor => t.supports_editor,
                ThemeTarget::Browser => t.supports_browser,
                ThemeTarget::Bar => t.supports_bar,
                ThemeTarget::Wallpaper => t.wallpaper.is_some(),
                ThemeTarget::All => true,
            })
            .collect()
    }

    /// Create custom theme
    pub fn create_custom_theme(
        &mut self,
        name: String,
        colors: ThemeColors,
        targets: Vec<ThemeTarget>,
    ) -> Result<(), String> {
        let id = format!("custom-{}", name.to_lowercase().replace(' ', "-"));

        let theme = Theme {
            name: name.clone(),
            id: id.clone(),
            description: format!("Custom theme: {}", name),
            author: "User".to_string(),
            variant: "custom".to_string(),
            colors,
            wallpaper: None,
            supports_terminal: targets.contains(&ThemeTarget::Terminal)
                || targets.contains(&ThemeTarget::All),
            supports_editor: targets.contains(&ThemeTarget::Editor)
                || targets.contains(&ThemeTarget::All),
            supports_browser: targets.contains(&ThemeTarget::Browser)
                || targets.contains(&ThemeTarget::All),
            supports_bar: targets.contains(&ThemeTarget::Bar)
                || targets.contains(&ThemeTarget::All),
        };

        self.add_theme(theme)?;
        Ok(())
    }

    /// Export theme configuration
    pub fn export_theme(&self, id: &str) -> Result<String, String> {
        if let Some(theme) = self.themes.get(id) {
            Ok(format!(
                "Name: {}\nVariant: {}\nAuthor: {}\nBackground: {}\nForeground: {}\nPrimary: {}",
                theme.name,
                theme.variant,
                theme.author,
                theme.colors.background,
                theme.colors.foreground,
                theme.colors.primary
            ))
        } else {
            Err(format!("Theme {} not found", id))
        }
    }

    /// Get theme statistics
    pub fn get_statistics(&self) -> (usize, usize, usize) {
        let total = self.themes.len();
        let with_wallpaper = self
            .themes
            .values()
            .filter(|t| t.wallpaper.is_some())
            .count();
        let full_support = self
            .themes
            .values()
            .filter(|t| {
                t.supports_terminal && t.supports_editor && t.supports_browser && t.supports_bar
            })
            .count();
        (total, with_wallpaper, full_support)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_picker_creation() {
        let manager = ThemePickerManager::with_default_config();
        assert_eq!(manager.themes.len(), 0);
        assert!(manager.current_theme.is_none());
    }

    #[test]
    fn test_config_default() {
        let config = ThemePickerConfig::default();
        assert!(config.auto_apply);
        assert!(config.save_history);
        assert_eq!(config.max_history, 10);
        assert!(!config.confirm_apply);
    }

    #[test]
    fn test_add_theme() {
        let mut manager = ThemePickerManager::with_default_config();
        let theme = Theme {
            name: "Test Theme".to_string(),
            id: "test-theme".to_string(),
            description: "Test description".to_string(),
            author: "Test Author".to_string(),
            variant: "dark".to_string(),
            colors: ThemeColors {
                background: "#1e1e1e".to_string(),
                foreground: "#d4d4d4".to_string(),
                cursor: "#ffffff".to_string(),
                primary: "#007acc".to_string(),
                secondary: "#4ec9b0".to_string(),
                accent: "#ce9178".to_string(),
                error: "#f14c4c".to_string(),
                warning: "#cca700".to_string(),
                success: "#4ec9b0".to_string(),
                info: "#3794ff".to_string(),
            },
            wallpaper: None,
            supports_terminal: true,
            supports_editor: true,
            supports_browser: true,
            supports_bar: true,
        };

        manager.add_theme(theme).unwrap();
        assert_eq!(manager.themes.len(), 1);
    }

    #[test]
    fn test_apply_theme() {
        let mut manager = ThemePickerManager::with_default_config();
        let theme = Theme {
            name: "Test Theme".to_string(),
            id: "test-theme".to_string(),
            description: "Test description".to_string(),
            author: "Test Author".to_string(),
            variant: "dark".to_string(),
            colors: ThemeColors {
                background: "#1e1e1e".to_string(),
                foreground: "#d4d4d4".to_string(),
                cursor: "#ffffff".to_string(),
                primary: "#007acc".to_string(),
                secondary: "#4ec9b0".to_string(),
                accent: "#ce9178".to_string(),
                error: "#f14c4c".to_string(),
                warning: "#cca700".to_string(),
                success: "#4ec9b0".to_string(),
                info: "#3794ff".to_string(),
            },
            wallpaper: None,
            supports_terminal: true,
            supports_editor: true,
            supports_browser: true,
            supports_bar: true,
        };

        manager.add_theme(theme).unwrap();
        manager.apply_theme("test-theme", ThemeTarget::All).unwrap();

        assert_eq!(manager.current_theme, Some("test-theme".to_string()));
        assert_eq!(manager.status, ThemeStatus::Complete);
    }

    #[test]
    fn test_search_themes() {
        let mut manager = ThemePickerManager::with_default_config();
        let theme = Theme {
            name: "Tokyo Night".to_string(),
            id: "tokyo-night".to_string(),
            description: "Beautiful dark theme".to_string(),
            author: "Test Author".to_string(),
            variant: "dark".to_string(),
            colors: ThemeColors {
                background: "#1a1b26".to_string(),
                foreground: "#a9b1d6".to_string(),
                cursor: "#c0caf5".to_string(),
                primary: "#7aa2f7".to_string(),
                secondary: "#bb9af7".to_string(),
                accent: "#f7768e".to_string(),
                error: "#f7768e".to_string(),
                warning: "#e0af68".to_string(),
                success: "#9ece6a".to_string(),
                info: "#7dcfff".to_string(),
            },
            wallpaper: None,
            supports_terminal: true,
            supports_editor: true,
            supports_browser: true,
            supports_bar: true,
        };

        manager.add_theme(theme).unwrap();
        let results = manager.search_themes("tokyo");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Tokyo Night");
    }

    #[test]
    fn test_get_themes_supporting() {
        let mut manager = ThemePickerManager::with_default_config();
        let theme = Theme {
            name: "Test Theme".to_string(),
            id: "test-theme".to_string(),
            description: "Test description".to_string(),
            author: "Test Author".to_string(),
            variant: "dark".to_string(),
            colors: ThemeColors {
                background: "#1e1e1e".to_string(),
                foreground: "#d4d4d4".to_string(),
                cursor: "#ffffff".to_string(),
                primary: "#007acc".to_string(),
                secondary: "#4ec9b0".to_string(),
                accent: "#ce9178".to_string(),
                error: "#f14c4c".to_string(),
                warning: "#cca700".to_string(),
                success: "#4ec9b0".to_string(),
                info: "#3794ff".to_string(),
            },
            wallpaper: None,
            supports_terminal: true,
            supports_editor: true,
            supports_browser: true,
            supports_bar: true,
        };

        manager.add_theme(theme).unwrap();
        let terminal_themes = manager.get_themes_supporting(ThemeTarget::Terminal);

        assert_eq!(terminal_themes.len(), 1);
    }

    #[test]
    fn test_get_statistics() {
        let mut manager = ThemePickerManager::with_default_config();
        let theme = Theme {
            name: "Test Theme".to_string(),
            id: "test-theme".to_string(),
            description: "Test description".to_string(),
            author: "Test Author".to_string(),
            variant: "dark".to_string(),
            colors: ThemeColors {
                background: "#1e1e1e".to_string(),
                foreground: "#d4d4d4".to_string(),
                cursor: "#ffffff".to_string(),
                primary: "#007acc".to_string(),
                secondary: "#4ec9b0".to_string(),
                accent: "#ce9178".to_string(),
                error: "#f14c4c".to_string(),
                warning: "#cca700".to_string(),
                success: "#4ec9b0".to_string(),
                info: "#3794ff".to_string(),
            },
            wallpaper: Some("/path/to/wallpaper.jpg".to_string()),
            supports_terminal: true,
            supports_editor: true,
            supports_browser: true,
            supports_bar: true,
        };

        manager.add_theme(theme).unwrap();
        let (total, with_wallpaper, full_support) = manager.get_statistics();

        assert_eq!(total, 1);
        assert_eq!(with_wallpaper, 1);
        assert_eq!(full_support, 1);
    }
}
