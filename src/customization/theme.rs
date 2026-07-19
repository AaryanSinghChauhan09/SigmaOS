// SigmaOS Theme Engine
// OOP-based declarative theming with light/dark/auto modes

use std::collections::HashMap;

/// Color palette
#[derive(Debug, Clone)]
pub struct ColorPalette {
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub background: String,
    pub foreground: String,
    pub success: String,
    pub warning: String,
    pub error: String,
}

/// Typography settings
#[derive(Debug, Clone)]
pub struct TypographySettings {
    pub font_family: String,
    pub font_size: u16,
    pub font_weight: u16,
    pub line_height: f32,
    pub letter_spacing: f32,
}

/// Spacing settings
#[derive(Debug, Clone)]
pub struct SpacingSettings {
    pub unit: u16,
    pub padding_small: u16,
    pub padding_medium: u16,
    pub padding_large: u16,
    pub margin_small: u16,
    pub margin_medium: u16,
    pub margin_large: u16,
}

/// Border radius settings
#[derive(Debug, Clone)]
pub struct BorderRadiusSettings {
    pub small: u16,
    pub medium: u16,
    pub large: u16,
    pub full: bool,
}

/// Shadow settings
#[derive(Debug, Clone)]
pub struct ShadowSettings {
    pub enabled: bool,
    pub blur: u16,
    pub spread: u16,
    pub color: String,
    pub opacity: f32,
}

/// Animation settings
#[derive(Debug, Clone)]
pub struct AnimationSettings {
    pub enabled: bool,
    pub duration_ms: u32,
    pub easing: String,
}

/// Theme mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
    Auto,
}

/// Theme
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub mode: ThemeMode,
    pub colors: ColorPalette,
    pub typography: TypographySettings,
    pub spacing: SpacingSettings,
    pub border_radius: BorderRadiusSettings,
    pub shadows: ShadowSettings,
    pub animations: AnimationSettings,
}

/// OOP trait for theme providers
pub trait ThemeProvider {
    /// Get current theme
    fn get_theme(&self) -> &Theme;
    /// Set theme
    fn set_theme(&mut self, theme: Theme) -> Result<(), ThemeError>;
    /// Get theme by name
    fn get_theme_by_name(&self, name: &str) -> Option<&Theme>;
    /// Apply theme
    fn apply_theme(&mut self, theme: &Theme) -> Result<(), ThemeError>;
    /// Get provider name
    fn name(&self) -> &str;
}

/// Built-in theme provider
pub struct BuiltInThemeProvider {
    themes: HashMap<String, Theme>,
    current_theme: String,
}

impl BuiltInThemeProvider {
    pub fn new() -> Self {
        let mut themes = HashMap::new();

        // Light theme
        themes.insert("light".to_string(), Theme {
            name: "Light".to_string(),
            mode: ThemeMode::Light,
            colors: ColorPalette {
                primary: "#007AFF".to_string(),
                secondary: "#5856D6".to_string(),
                accent: "#FF9500".to_string(),
                background: "#FFFFFF".to_string(),
                foreground: "#000000".to_string(),
                success: "#34C759".to_string(),
                warning: "#FFCC00".to_string(),
                error: "#FF3B30".to_string(),
            },
            typography: TypographySettings {
                font_family: "System UI".to_string(),
                font_size: 16,
                font_weight: 400,
                line_height: 1.5,
                letter_spacing: 0.0,
            },
            spacing: SpacingSettings {
                unit: 8,
                padding_small: 8,
                padding_medium: 16,
                padding_large: 24,
                margin_small: 8,
                margin_medium: 16,
                margin_large: 24,
            },
            border_radius: BorderRadiusSettings {
                small: 4,
                medium: 8,
                large: 16,
                full: false,
            },
            shadows: ShadowSettings {
                enabled: true,
                blur: 10,
                spread: 0,
                color: "#000000".to_string(),
                opacity: 0.1,
            },
            animations: AnimationSettings {
                enabled: true,
                duration_ms: 300,
                easing: "ease-in-out".to_string(),
            },
        });

        // Dark theme
        themes.insert("dark".to_string(), Theme {
            name: "Dark".to_string(),
            mode: ThemeMode::Dark,
            colors: ColorPalette {
                primary: "#0A84FF".to_string(),
                secondary: "#5E5CE6".to_string(),
                accent: "#FFD60A".to_string(),
                background: "#000000".to_string(),
                foreground: "#FFFFFF".to_string(),
                success: "#30D158".to_string(),
                warning: "#FFD60A".to_string(),
                error: "#FF453A".to_string(),
            },
            typography: TypographySettings {
                font_family: "System UI".to_string(),
                font_size: 16,
                font_weight: 400,
                line_height: 1.5,
                letter_spacing: 0.0,
            },
            spacing: SpacingSettings {
                unit: 8,
                padding_small: 8,
                padding_medium: 16,
                padding_large: 24,
                margin_small: 8,
                margin_medium: 16,
                margin_large: 24,
            },
            border_radius: BorderRadiusSettings {
                small: 4,
                medium: 8,
                large: 16,
                full: false,
            },
            shadows: ShadowSettings {
                enabled: true,
                blur: 10,
                spread: 0,
                color: "#FFFFFF".to_string(),
                opacity: 0.1,
            },
            animations: AnimationSettings {
                enabled: true,
                duration_ms: 300,
                easing: "ease-in-out".to_string(),
            },
        });

        Self {
            themes,
            current_theme: "light".to_string(),
        }
    }

    pub fn add_theme(&mut self, theme: Theme) {
        self.themes.insert(theme.name.clone().to_lowercase(), theme);
    }
}

impl ThemeProvider for BuiltInThemeProvider {
    fn get_theme(&self) -> &Theme {
        self.themes.get(&self.current_theme).unwrap()
    }

    fn set_theme(&mut self, theme: Theme) -> Result<(), ThemeError> {
        self.themes.insert(theme.name.clone().to_lowercase(), theme);
        self.current_theme = theme.name.to_lowercase();
        Ok(())
    }

    fn get_theme_by_name(&self, name: &str) -> Option<&Theme> {
        self.themes.get(&name.to_lowercase())
    }

    fn apply_theme(&mut self, theme: &Theme) -> Result<(), ThemeError> {
        self.current_theme = theme.name.to_lowercase();
        // In real implementation, this would apply the theme to the UI
        Ok(())
    }

    fn name(&self) -> &str {
        "BuiltInThemeProvider"
    }
}

/// Custom theme provider
pub struct CustomThemeProvider {
    themes: HashMap<String, Theme>,
    current_theme: String,
    custom_themes_path: String,
}

impl CustomThemeProvider {
    pub fn new(custom_themes_path: String) -> Self {
        Self {
            themes: HashMap::new(),
            current_theme: "custom".to_string(),
            custom_themes_path,
        }
    }

    pub fn load_theme_from_file(&mut self, path: &str) -> Result<Theme, ThemeError> {
        // Simulated theme loading from file
        // In real implementation, would parse JSON/YAML theme file
        Ok(Theme {
            name: "Custom".to_string(),
            mode: ThemeMode::Light,
            colors: ColorPalette {
                primary: "#000000".to_string(),
                secondary: "#333333".to_string(),
                accent: "#666666".to_string(),
                background: "#FFFFFF".to_string(),
                foreground: "#000000".to_string(),
                success: "#00FF00".to_string(),
                warning: "#FFFF00".to_string(),
                error: "#FF0000".to_string(),
            },
            typography: TypographySettings {
                font_family: "Custom".to_string(),
                font_size: 16,
                font_weight: 400,
                line_height: 1.5,
                letter_spacing: 0.0,
            },
            spacing: SpacingSettings {
                unit: 8,
                padding_small: 8,
                padding_medium: 16,
                padding_large: 24,
                margin_small: 8,
                margin_medium: 16,
                margin_large: 24,
            },
            border_radius: BorderRadiusSettings {
                small: 4,
                medium: 8,
                large: 16,
                full: false,
            },
            shadows: ShadowSettings {
                enabled: true,
                blur: 10,
                spread: 0,
                color: "#000000".to_string(),
                opacity: 0.1,
            },
            animations: AnimationSettings {
                enabled: true,
                duration_ms: 300,
                easing: "ease-in-out".to_string(),
            },
        })
    }
}

impl ThemeProvider for CustomThemeProvider {
    fn get_theme(&self) -> &Theme {
        self.themes.get(&self.current_theme).unwrap()
    }

    fn set_theme(&mut self, theme: Theme) -> Result<(), ThemeError> {
        self.themes.insert(theme.name.clone().to_lowercase(), theme);
        self.current_theme = theme.name.to_lowercase();
        Ok(())
    }

    fn get_theme_by_name(&self, name: &str) -> Option<&Theme> {
        self.themes.get(&name.to_lowercase())
    }

    fn apply_theme(&mut self, theme: &Theme) -> Result<(), ThemeError> {
        self.current_theme = theme.name.to_lowercase();
        Ok(())
    }

    fn name(&self) -> &str {
        "CustomThemeProvider"
    }
}

/// OOP-based Theme Engine
pub struct ThemeEngine {
    provider: Box<dyn ThemeProvider>,
    auto_switch_enabled: bool,
    current_mode: ThemeMode,
}

impl ThemeEngine {
    pub fn new(provider: Box<dyn ThemeProvider>) -> Self {
        Self {
            provider,
            auto_switch_enabled: false,
            current_mode: ThemeMode::Light,
        }
    }

    /// Enable auto-switch based on system theme
    pub fn with_auto_switch(mut self, enabled: bool) -> Self {
        self.auto_switch_enabled = enabled;
        self
    }

    /// Get current theme
    pub fn current_theme(&self) -> &Theme {
        self.provider.get_theme()
    }

    /// Set theme by name
    pub fn set_theme_by_name(&mut self, name: &str) -> Result<(), ThemeError> {
        if let Some(theme) = self.provider.get_theme_by_name(name) {
            self.provider.apply_theme(theme)
        } else {
            Err(ThemeError::ThemeNotFound(name.to_string()))
        }
    }

    /// Add custom theme
    pub fn add_theme(&mut self, theme: Theme) -> Result<(), ThemeError> {
        self.provider.set_theme(theme)
    }

    /// Get available themes
    pub fn available_themes(&self) -> Vec<String> {
        // In real implementation, would return list of available theme names
        vec!["light".to_string(), "dark".to_string(), "custom".to_string()]
    }

    /// Export current theme
    pub fn export_theme(&self) -> String {
        let theme = self.current_theme();
        // Simulated export to JSON
        format!("{{\"name\": \"{}\", \"mode\": {:?}}}", theme.name, theme.mode)
    }

    /// Import theme from string
    pub fn import_theme(&mut self, theme_json: &str) -> Result<(), ThemeError> {
        // Simulated import from JSON
        // In real implementation, would parse JSON and create theme
        Ok(())
    }

    /// Set theme mode
    pub fn set_mode(&mut self, mode: ThemeMode) -> Result<(), ThemeError> {
        self.current_mode = mode;

        if self.auto_switch_enabled {
            let theme_name = match mode {
                ThemeMode::Light => "light",
                ThemeMode::Dark => "dark",
                ThemeMode::Auto => "light", // Default to light for auto
            };
            self.set_theme_by_name(theme_name)?;
        }

        Ok(())
    }

    /// Get current mode
    pub fn current_mode(&self) -> ThemeMode {
        self.current_mode
    }
}

impl Default for ThemeEngine {
    fn default() -> Self {
        Self::new(Box::new(BuiltInThemeProvider::new()))
            .with_auto_switch(false)
    }
}

/// Theme errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThemeError {
    ThemeNotFound(String),
    InvalidThemeFormat(String),
    ApplyError(String),
    LoadError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_palette() {
        let palette = ColorPalette {
            primary: "#007AFF".to_string(),
            secondary: "#5856D6".to_string(),
            accent: "#FF9500".to_string(),
            background: "#FFFFFF".to_string(),
            foreground: "#000000".to_string(),
            success: "#34C759".to_string(),
            warning: "#FFCC00".to_string(),
            error: "#FF3B30".to_string(),
        };
        assert_eq!(palette.primary, "#007AFF");
    }

    #[test]
    fn test_built_in_theme_provider() {
        let provider = BuiltInThemeProvider::new();
        assert_eq!(provider.name(), "BuiltInThemeProvider");
        assert_eq!(provider.themes.len(), 2);
    }

    #[test]
    fn test_custom_theme_provider() {
        let provider = CustomThemeProvider::new("/themes".to_string());
        assert_eq!(provider.name(), "CustomThemeProvider");
    }

    #[test]
    fn test_theme_engine() {
        let engine = ThemeEngine::default();
        let theme = engine.current_theme();
        assert_eq!(theme.name, "Light");
    }

    #[test]
    fn test_set_theme_by_name() {
        let mut engine = ThemeEngine::default();
        engine.set_theme_by_name("dark").unwrap();
        let theme = engine.current_theme();
        assert_eq!(theme.name, "Dark");
    }

    #[test]
    fn test_set_mode() {
        let mut engine = ThemeEngine::default();
        engine.set_mode(ThemeMode::Dark).unwrap();
        assert_eq!(engine.current_mode(), ThemeMode::Dark);
    }
}
