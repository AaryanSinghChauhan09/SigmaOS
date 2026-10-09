//! Theme Manager
//!
//! Theme management inspired by Omarchy's theme system and Linux Mint's
//! theme preferences, supporting dark/light modes, accent colors, and
//! custom theme creation.

use std::collections::HashMap;

/// Theme mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
    Auto,
}

impl ThemeMode {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "light" => Some(ThemeMode::Light),
            "dark" => Some(ThemeMode::Dark),
            "auto" => Some(ThemeMode::Auto),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ThemeMode::Light => "Light",
            ThemeMode::Dark => "Dark",
            ThemeMode::Auto => "Auto",
        }
    }
}

/// Accent color
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccentColor {
    Blue,
    Green,
    Red,
    Orange,
    Purple,
    Pink,
    Teal,
    Yellow,
    Custom { r: u8, g: u8, b: u8 },
}

impl AccentColor {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        AccentColor::Custom { r, g, b }
    }

    pub fn to_rgb(&self) -> (u8, u8, u8) {
        match self {
            AccentColor::Blue => (59, 130, 246),
            AccentColor::Green => (34, 197, 94),
            AccentColor::Red => (239, 68, 68),
            AccentColor::Orange => (249, 115, 22),
            AccentColor::Purple => (168, 85, 247),
            AccentColor::Pink => (236, 72, 153),
            AccentColor::Teal => (20, 184, 166),
            AccentColor::Yellow => (234, 179, 8),
            AccentColor::Custom { r, g, b } => (*r, *g, *b),
        }
    }
}

/// Window style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowStyle {
    Traditional,
    Modern,
    Compact,
    Floating,
}

impl WindowStyle {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "traditional" => Some(WindowStyle::Traditional),
            "modern" => Some(WindowStyle::Modern),
            "compact" => Some(WindowStyle::Compact),
            "floating" => Some(WindowStyle::Floating),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            WindowStyle::Traditional => "Traditional",
            WindowStyle::Modern => "Modern",
            WindowStyle::Compact => "Compact",
            WindowStyle::Floating => "Floating",
        }
    }
}

/// Font family
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontFamily {
    pub name: String,
    pub is_system: bool,
}

impl FontFamily {
    pub fn new(name: String) -> Self {
        Self {
            name,
            is_system: false,
        }
    }

    pub fn system() -> Self {
        Self {
            name: "System UI".to_string(),
            is_system: true,
        }
    }
}

/// Color palette
#[derive(Debug, Clone, Copy)]
pub struct ColorPalette {
    pub background: (u8, u8, u8),
    pub foreground: (u8, u8, u8),
    pub surface: (u8, u8, u8),
    pub border: (u8, u8, u8),
    pub accent: (u8, u8, u8),
}

impl ColorPalette {
    pub fn dark() -> Self {
        Self {
            background: (30, 30, 30),
            foreground: (255, 255, 255),
            surface: (50, 50, 50),
            border: (100, 100, 100),
            accent: (59, 130, 246),
        }
    }

    pub fn light() -> Self {
        Self {
            background: (255, 255, 255),
            foreground: (0, 0, 0),
            surface: (240, 240, 240),
            border: (200, 200, 200),
            accent: (59, 130, 246),
        }
    }

    pub fn with_accent(mut self, accent: AccentColor) -> Self {
        self.accent = accent.to_rgb();
        self
    }
}

/// Theme configuration
#[derive(Debug, Clone)]
pub struct ThemeConfig {
    pub name: String,
    pub mode: ThemeMode,
    pub accent: AccentColor,
    pub window_style: WindowStyle,
    pub font_family: FontFamily,
    pub font_size: u32,
    pub palette: ColorPalette,
    pub is_custom: bool,
}

impl ThemeConfig {
    pub fn new(name: String, mode: ThemeMode) -> Self {
        let palette = match mode {
            ThemeMode::Light => ColorPalette::light(),
            ThemeMode::Dark => ColorPalette::dark(),
            ThemeMode::Auto => ColorPalette::dark(),
        };

        Self {
            name,
            mode,
            accent: AccentColor::Blue,
            window_style: WindowStyle::Modern,
            font_family: FontFamily::system(),
            font_size: 14,
            palette,
            is_custom: false,
        }
    }

    pub fn with_accent(mut self, accent: AccentColor) -> Self {
        self.accent = accent;
        self.palette = self.palette.with_accent(accent);
        self
    }

    pub fn with_window_style(mut self, style: WindowStyle) -> Self {
        self.window_style = style;
        self
    }

    pub fn with_font_size(mut self, size: u32) -> Self {
        self.font_size = size;
        self
    }

    pub fn mark_custom(&mut self) {
        self.is_custom = true;
    }
}

/// Theme manager
#[derive(Debug)]
pub struct ThemeManager {
    themes: HashMap<String, ThemeConfig>,
    current_theme: String,
    default_theme: String,
}

impl ThemeManager {
    pub fn new() -> Self {
        let mut manager = Self {
            themes: HashMap::new(),
            current_theme: "Sigma Dark".to_string(),
            default_theme: "Sigma Dark".to_string(),
        };

        // Add default themes
        manager.add_default_themes();

        manager
    }

    /// Add default themes
    fn add_default_themes(&mut self) {
        let dark_theme = ThemeConfig::new("Sigma Dark".to_string(), ThemeMode::Dark);
        self.themes.insert("Sigma Dark".to_string(), dark_theme);

        let light_theme = ThemeConfig::new("Sigma Light".to_string(), ThemeMode::Light);
        self.themes.insert("Sigma Light".to_string(), light_theme);

        let auto_theme = ThemeConfig::new("Sigma Auto".to_string(), ThemeMode::Auto);
        self.themes.insert("Sigma Auto".to_string(), auto_theme);
    }

    /// Add a custom theme
    pub fn add_theme(&mut self, theme: ThemeConfig) {
        let mut theme = theme;
        theme.mark_custom();
        self.themes.insert(theme.name.clone(), theme);
    }

    /// Get a theme
    pub fn get_theme(&self, name: &str) -> Option<&ThemeConfig> {
        self.themes.get(name)
    }

    /// Get current theme
    pub fn get_current_theme(&self) -> &ThemeConfig {
        self.themes
            .get(&self.current_theme)
            .unwrap_or_else(|| self.themes.get(&self.default_theme).unwrap())
    }

    /// Set current theme
    pub fn set_current_theme(&mut self, name: &str) -> Result<(), String> {
        if !self.themes.contains_key(name) {
            return Err(format!("Theme {} not found", name));
        }
        self.current_theme = name.to_string();
        Ok(())
    }

    /// List all themes
    pub fn list_themes(&self) -> Vec<&ThemeConfig> {
        self.themes.values().collect()
    }

    /// List custom themes
    pub fn list_custom_themes(&self) -> Vec<&ThemeConfig> {
        self.themes.values().filter(|t| t.is_custom).collect()
    }

    /// Remove a custom theme
    pub fn remove_theme(&mut self, name: &str) -> Result<(), String> {
        let theme = self
            .themes
            .get(name)
            .ok_or_else(|| format!("Theme {} not found", name))?;

        if !theme.is_custom {
            return Err(format!("Cannot remove default theme {}", name));
        }

        if name == self.current_theme {
            self.current_theme = self.default_theme.clone();
        }

        self.themes.remove(name);
        Ok(())
    }

    /// Create a variant of an existing theme
    pub fn create_variant(
        &mut self,
        base_name: &str,
        variant_name: String,
        accent: AccentColor,
    ) -> Result<(), String> {
        let base = self
            .themes
            .get(base_name)
            .ok_or_else(|| format!("Base theme {} not found", base_name))?;

        let mut variant = base.clone();
        variant.name = variant_name.clone();
        variant.accent = accent;
        variant.palette = variant.palette.with_accent(accent);
        variant.mark_custom();

        self.themes.insert(variant_name, variant);
        Ok(())
    }

    /// Get statistics
    pub fn get_statistics(&self) -> ThemeStatistics {
        let total_themes = self.themes.len();
        let custom_themes = self.themes.values().filter(|t| t.is_custom).count();
        let default_themes = total_themes - custom_themes;

        ThemeStatistics {
            total_themes,
            custom_themes,
            default_themes,
            current_theme: self.current_theme.clone(),
        }
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Theme statistics
#[derive(Debug, Clone)]
pub struct ThemeStatistics {
    pub total_themes: usize,
    pub custom_themes: usize,
    pub default_themes: usize,
    pub current_theme: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_mode_from_str() {
        assert_eq!(ThemeMode::from_str("dark"), Some(ThemeMode::Dark));
        assert_eq!(ThemeMode::from_str("light"), Some(ThemeMode::Light));
    }

    #[test]
    fn test_accent_color() {
        let (r, g, b) = AccentColor::Blue.to_rgb();
        assert_eq!(r, 59);
        assert_eq!(g, 130);
        assert_eq!(b, 246);
    }

    #[test]
    fn test_color_palette() {
        let dark = ColorPalette::dark();
        assert_eq!(dark.background, (30, 30, 30));

        let light = ColorPalette::light();
        assert_eq!(light.background, (255, 255, 255));
    }

    #[test]
    fn test_theme_config_creation() {
        let theme = ThemeConfig::new("Test".to_string(), ThemeMode::Dark);
        assert_eq!(theme.name, "Test");
        assert_eq!(theme.mode, ThemeMode::Dark);
    }

    #[test]
    fn test_theme_manager_creation() {
        let manager = ThemeManager::new();
        assert!(manager.list_themes().len() >= 3);
    }

    #[test]
    fn test_add_custom_theme() {
        let mut manager = ThemeManager::new();
        let theme = ThemeConfig::new("Custom".to_string(), ThemeMode::Dark);
        manager.add_theme(theme);
        assert!(manager.get_theme("Custom").is_some());
    }

    #[test]
    fn test_set_current_theme() {
        let mut manager = ThemeManager::new();
        assert!(manager.set_current_theme("Sigma Light").is_ok());
        assert_eq!(manager.get_current_theme().name, "Sigma Light");
    }

    #[test]
    fn test_create_variant() {
        let mut manager = ThemeManager::new();
        assert!(manager
            .create_variant(
                "Sigma Dark",
                "Sigma Dark Green".to_string(),
                AccentColor::Green
            )
            .is_ok());
        assert!(manager.get_theme("Sigma Dark Green").is_some());
    }

    #[test]
    fn test_remove_custom_theme() {
        let mut manager = ThemeManager::new();
        let theme = ThemeConfig::new("Custom".to_string(), ThemeMode::Dark);
        manager.add_theme(theme);
        assert!(manager.remove_theme("Custom").is_ok());
        assert!(manager.get_theme("Custom").is_none());
    }

    #[test]
    fn test_statistics() {
        let manager = ThemeManager::new();
        let stats = manager.get_statistics();
        assert!(stats.total_themes >= 3);
    }
}
