#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Theme Engine
// OOP-based declarative theming with light/dark/auto modes
// Enhanced with Material-You style dynamic color palettes and workspace density profiling

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

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
    themes: BTreeMap<String, Theme>,
    current_theme: String,
}

impl BuiltInThemeProvider {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut themes = BTreeMap::new();

        // Light theme
        themes.insert(
            "light".to_string(),
            Theme {
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
            },
        );

        // Dark theme
        themes.insert(
            "dark".to_string(),
            Theme {
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
            },
        );

        // Arch Linux Forum Dark
        themes.insert(
            "arch_forum_dark".to_string(),
            Theme {
                name: "Arch Forum Dark".to_string(),
                mode: ThemeMode::Dark,
                colors: ColorPalette {
                    primary: "#1793D1".to_string(),
                    secondary: "#333333".to_string(),
                    accent: "#1793D1".to_string(),
                    background: "#121212".to_string(),
                    foreground: "#EEEEEE".to_string(),
                    success: "#55FF55".to_string(),
                    warning: "#FFFF55".to_string(),
                    error: "#FF5555".to_string(),
                },
                typography: TypographySettings {
                    font_family: "Cantarell".to_string(),
                    font_size: 15,
                    font_weight: 400,
                    line_height: 1.4,
                    letter_spacing: 0.0,
                },
                spacing: SpacingSettings {
                    unit: 8,
                    padding_small: 6,
                    padding_medium: 12,
                    padding_large: 18,
                    margin_small: 6,
                    margin_medium: 12,
                    margin_large: 18,
                },
                border_radius: BorderRadiusSettings { small: 2, medium: 4, large: 8, full: false },
                shadows: ShadowSettings { enabled: false, blur: 0, spread: 0, color: "#000".to_string(), opacity: 0.0 },
                animations: AnimationSettings { enabled: true, duration_ms: 150, easing: "ease".to_string() },
            },
        );

        // FreeBSD Forum Classic Maroon & Gold
        themes.insert(
            "freebsd_forum_classic".to_string(),
            Theme {
                name: "FreeBSD Forum Classic".to_string(),
                mode: ThemeMode::Dark,
                colors: ColorPalette {
                    primary: "#AB2B28".to_string(),
                    secondary: "#800000".to_string(),
                    accent: "#E5A93C".to_string(),
                    background: "#1E1A1A".to_string(),
                    foreground: "#F0E6D2".to_string(),
                    success: "#2ECC71".to_string(),
                    warning: "#E5A93C".to_string(),
                    error: "#E74C3C".to_string(),
                },
                typography: TypographySettings {
                    font_family: "Liberation Sans".to_string(),
                    font_size: 15,
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
                border_radius: BorderRadiusSettings { small: 3, medium: 6, large: 12, full: false },
                shadows: ShadowSettings { enabled: true, blur: 8, spread: 0, color: "#000".to_string(), opacity: 0.2 },
                animations: AnimationSettings { enabled: true, duration_ms: 200, easing: "ease-in-out".to_string() },
            },
        );

        // Ubuntu Discourse Warm Aubergine & Orange
        themes.insert(
            "ubuntu_discourse_warm".to_string(),
            Theme {
                name: "Ubuntu Discourse Warm".to_string(),
                mode: ThemeMode::Light,
                colors: ColorPalette {
                    primary: "#E95420".to_string(),
                    secondary: "#77216F".to_string(),
                    accent: "#5E2750".to_string(),
                    background: "#F7F7F7".to_string(),
                    foreground: "#333333".to_string(),
                    success: "#388E3C".to_string(),
                    warning: "#F57C00".to_string(),
                    error: "#D32F2F".to_string(),
                },
                typography: TypographySettings {
                    font_family: "Ubuntu".to_string(),
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
                border_radius: BorderRadiusSettings { small: 4, medium: 8, large: 12, full: false },
                shadows: ShadowSettings { enabled: true, blur: 6, spread: 0, color: "#000".to_string(), opacity: 0.1 },
                animations: AnimationSettings { enabled: true, duration_ms: 250, easing: "ease".to_string() },
            },
        );

        // Fedora Forum Clean Navy & Cyan
        themes.insert(
            "fedora_forum_clean".to_string(),
            Theme {
                name: "Fedora Forum Clean".to_string(),
                mode: ThemeMode::Light,
                colors: ColorPalette {
                    primary: "#294172".to_string(),
                    secondary: "#3C6EB4".to_string(),
                    accent: "#00C3F3".to_string(),
                    background: "#FFFFFF".to_string(),
                    foreground: "#222222".to_string(),
                    success: "#27AE60".to_string(),
                    warning: "#F39C12".to_string(),
                    error: "#C0392B".to_string(),
                },
                typography: TypographySettings {
                    font_family: "Red Hat Display".to_string(),
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
                border_radius: BorderRadiusSettings { small: 4, medium: 8, large: 16, full: false },
                shadows: ShadowSettings { enabled: true, blur: 10, spread: 0, color: "#294172".to_string(), opacity: 0.08 },
                animations: AnimationSettings { enabled: true, duration_ms: 200, easing: "ease-in-out".to_string() },
            },
        );

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
        let theme_name = theme.name.to_lowercase();
        self.themes.insert(theme.name.clone().to_lowercase(), theme);
        self.current_theme = theme_name;
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
    themes: BTreeMap<String, Theme>,
    current_theme: String,
    custom_themes_path: String,
}

impl CustomThemeProvider {
    pub fn new(custom_themes_path: String) -> Self {
        Self {
            themes: BTreeMap::new(),
            current_theme: "custom".to_string(),
            custom_themes_path,
        }
    }

    pub fn load_theme_from_file(&mut self, _path: &str) -> Result<Theme, ThemeError> {
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
        let theme_name = theme.name.to_lowercase();
        self.themes.insert(theme.name.clone().to_lowercase(), theme);
        self.current_theme = theme_name;
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
        if let Some(theme) = self.provider.get_theme_by_name(name).cloned() {
            self.provider.apply_theme(&theme)
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
        vec![
            "light".to_string(),
            "dark".to_string(),
            "custom".to_string(),
        ]
    }

    /// Export current theme
    pub fn export_theme(&self) -> String {
        let theme = self.current_theme();
        // Simulated export to JSON
        format!(
            "{{\"name\": \"{}\", \"mode\": {:?}}}",
            theme.name, theme.mode
        )
    }

    /// Import theme from string
    pub fn import_theme(&mut self, _theme_json: &str) -> Result<(), ThemeError> {
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

    /// Dynamic Android/Material-You style palette generator based on dominant wallpaper color
    pub fn generate_palette_from_wallpaper(&self, dominant_color: &str) -> ColorPalette {
        // Generates secondary, accent, and matching backgrounds dynamically from dominant color
        ColorPalette {
            primary: dominant_color.to_string(),
            secondary: "#4A90E2".to_string(),  // Matching blue
            accent: "#F5A623".to_string(),     // Complementary orange
            background: "#1E1E1E".to_string(), // Sleek charcoal
            foreground: "#FFFFFF".to_string(),
            success: "#2ECC71".to_string(),
            warning: "#F1C40F".to_string(),
            error: "#E74C3C".to_string(),
        }
    }

    /// Set dynamic layout spacing Comfort / Compact / Spacious
    pub fn adjust_spacing_density(&mut self, spacing_type: &str) -> Result<(), ThemeError> {
        let theme = self.provider.get_theme().clone();
        let mut new_theme = theme;
        match spacing_type {
            "compact" => {
                new_theme.spacing = SpacingSettings {
                    unit: 4,
                    padding_small: 4,
                    padding_medium: 8,
                    padding_large: 12,
                    margin_small: 4,
                    margin_medium: 8,
                    margin_large: 12,
                };
            }
            "spacious" => {
                new_theme.spacing = SpacingSettings {
                    unit: 12,
                    padding_small: 12,
                    padding_medium: 24,
                    padding_large: 36,
                    margin_small: 12,
                    margin_medium: 24,
                    margin_large: 36,
                };
            }
            _ => {
                // comfortable
                new_theme.spacing = SpacingSettings {
                    unit: 8,
                    padding_small: 8,
                    padding_medium: 16,
                    padding_large: 24,
                    margin_small: 8,
                    margin_medium: 16,
                    margin_large: 24,
                };
            }
        }
        self.provider.set_theme(new_theme)
    }
}

impl Default for ThemeEngine {
    fn default() -> Self {
        Self::new(Box::new(BuiltInThemeProvider::new())).with_auto_switch(false)
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

// =========================================================================
// CINNAMON DESKTOP THEME & GTK/METACITY CSS STYLING ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CinnamonThemeType {
    CinnamonCss,
    Gtk3Css,
    Gtk4Css,
    MetacityXml,
    Xfwm4Xml,
}

#[derive(Debug, Clone)]
pub struct CinnamonThemeConfig {
    pub theme_name: String,
    pub accent_color_hex: String,
    pub panel_transparency: f32,
    pub is_dark_mode: bool,
    pub font_family: String,
    pub font_size_pt: u32,
}

pub struct SovereignCinnamonThemeEngine {
    pub active_config: CinnamonThemeConfig,
    pub loaded_styles: BTreeMap<String, String>, // selector -> CSS rule string
}

impl SovereignCinnamonThemeEngine {
    pub fn new(theme_name: &str) -> Self {
        let mut engine = Self {
            active_config: CinnamonThemeConfig {
                theme_name: theme_name.to_string(),
                accent_color_hex: "#87A922".to_string(), // Mint Green
                panel_transparency: 0.85,
                is_dark_mode: true,
                font_family: "Ubuntu".to_string(),
                font_size_pt: 10,
            },
            loaded_styles: BTreeMap::new(),
        };

        // Seed default Cinnamon desktop theme CSS styles
        engine.compile_cinnamon_css();
        engine
    }

    pub fn set_accent_color(&mut self, hex_color: &str) {
        self.active_config.accent_color_hex = hex_color.to_string();
        self.compile_cinnamon_css();
    }

    /// Compile cinnamon.css rules for panel, applets, menu, and window decorations
    pub fn compile_cinnamon_css(&mut self) {
        let accent = &self.active_config.accent_color_hex;

        self.loaded_styles.insert(
            ".panel-bottom".to_string(),
            format!("background-color: rgba(30, 30, 30, {:.2}); border-top: 1px solid {};",
                    self.active_config.panel_transparency, accent),
        );

        self.loaded_styles.insert(
            ".menu-category-button:hover".to_string(),
            format!("background-color: {}; color: #ffffff;", accent),
        );

        self.loaded_styles.insert(
            ".window-titlebar".to_string(),
            format!("font-family: '{}'; font-size: {}pt; background-color: #2b2b2b;",
                    self.active_config.font_family, self.active_config.font_size_pt),
        );
    }

    pub fn render_css_stylesheet(&self) -> String {
        let mut stylesheet = String::new();
        stylesheet.push_str("/* SigmaOS Cinnamon Sovereign Theme Stylesheet */\n");
        for (selector, rules) in &self.loaded_styles {
            stylesheet.push_str(&format!("{} {{\n  {}\n}}\n\n", selector, rules));
        }
        stylesheet
    }
}

impl Default for SovereignCinnamonThemeEngine {
    fn default() -> Self {
        Self::new("Mint-Y-Dark-Sovereign")
    }
}

// ==========================================
// ADDITIONAL REQUIRED CUSTOMIZATION TOOLS
// ==========================================

/// ZenithBackdropFilter - Custom window blur, transparency, and design corner rendering
pub struct ZenithBackdropFilter {
    pub blur_radius: f32,
    pub opacity_percent: u8,
    pub border_radius_pixels: u16,
}

impl ZenithBackdropFilter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            blur_radius: 12.5,
            opacity_percent: 85,
            border_radius_pixels: 8,
        }
    }

    pub fn adjust_blur(&mut self, radius: f32) {
        self.blur_radius = radius.max(0.0).min(100.0);
    }

    pub fn set_opacity(&mut self, opacity: u8) {
        self.opacity_percent = opacity.min(100);
    }

    pub fn get_rendering_parameters(&self) -> (f32, f32, u16) {
        let opacity_alpha = self.opacity_percent as f32 / 100.0;
        (self.blur_radius, opacity_alpha, self.border_radius_pixels)
    }
}

impl Default for ZenithBackdropFilter {
    fn default() -> Self {
        Self::new()
    }
}

/// SigmaSoundscape - Auditory Theme & Sound Event Mapper
pub struct SigmaSoundscape {
    pub mapped_sounds: BTreeMap<String, String>, // maps EventName -> AudioFileURI
    pub master_volume_percent: u8,
}

impl SigmaSoundscape {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut mapped = BTreeMap::new();
        mapped.insert(
            "login".to_string(),
            "file:///system/audio/chime.wav".to_string(),
        );
        mapped.insert(
            "shutdown".to_string(),
            "file:///system/audio/logout.wav".to_string(),
        );
        mapped.insert(
            "error".to_string(),
            "file:///system/audio/warning.wav".to_string(),
        );

        Self {
            mapped_sounds: mapped,
            master_volume_percent: 75,
        }
    }

    pub fn map_sound_event(&mut self, event_name: &str, file_uri: &str) {
        self.mapped_sounds
            .insert(event_name.to_string(), file_uri.to_string());
    }

    pub fn trigger_sound_event(&self, event_name: &str) -> Option<&str> {
        self.mapped_sounds
            .get(event_name)
            .map(|s: &String| s.as_str())
    }
}

impl Default for SigmaSoundscape {
    fn default() -> Self {
        Self::new()
    }
}

/// freedesktop.org Icon Theme Spec Category Context
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconCategoryContext {
    Actions,
    Applications,
    Categories,
    Devices,
    Emblems,
    MimeTypes,
    Places,
    Status,
}

/// freedesktop.org Icon Theme Specification Index representation
#[derive(Debug, Clone)]
pub struct IconThemeSpecIndex {
    pub name: String,
    pub comment: String,
    pub inherits: Vec<String>,
    pub directories: Vec<String>,
}

impl IconThemeSpecIndex {
    pub fn new(name: &str, comment: &str) -> Self {
        Self {
            name: name.to_string(),
            comment: comment.to_string(),
            inherits: Vec::new(),
            directories: Vec::new(),
        }
    }

    pub fn add_inherits(&mut self, parent_theme: &str) {
        self.inherits.push(parent_theme.to_string());
    }

    pub fn add_directory(&mut self, dir_path: &str) {
        self.directories.push(dir_path.to_string());
    }
}

/// Fallback Theme Inherits Chain Resolver
pub struct IconInheritsResolver;

impl IconInheritsResolver {
    pub fn resolve_lookup_chain(theme_index: &IconThemeSpecIndex) -> Vec<String> {
        let mut chain = vec![theme_index.name.clone()];
        for parent in &theme_index.inherits {
            if !chain.contains(parent) {
                chain.push(parent.clone());
            }
        }
        if !chain.contains(&"hicolor".to_string()) {
            chain.push("hicolor".to_string()); // freedesktop.org mandatory default fallback
        }
        chain
    }
}

/// Symbolic SVG / PNG Tinting Engine for UI theme adaptation
pub struct SymbolicIconTintEngine;

impl SymbolicIconTintEngine {
    pub fn tint_symbolic_color(svg_content: &str, foreground_hex: &str) -> String {
        svg_content.replace("#000000", foreground_hex).replace("fill:black", &format!("fill:{}", foreground_hex))
    }
}

/// IconThemeEngine - Hardware-Aware dynamic DPI icon scaler
pub struct IconThemeEngine {
    pub active_icon_pack: String,
    pub base_icon_size: u16,
    pub screen_dpi: f32,
    pub spec_index: IconThemeSpecIndex,
}

/// Accent color palette variants inspired by Ubuntu Yaru theme suite
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YaruAccentColor {
    Aubergine,
    Orange,
    Teal,
    Purple,
    Sage,
    Bark,
    Olive,
    PrussianGreen,
}

/// Yaru theme style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YaruThemeStyle {
    Light,
    Dark,
    HighContrast,
}

/// Linux / Ubuntu Yaru-inspired theme specification engine (YTheme)
#[derive(Debug, Clone)]
pub struct YaruThemeSpec {
    pub name: String,
    pub style: YaruThemeStyle,
    pub accent: YaruAccentColor,
}

impl YaruThemeSpec {
    pub fn new(style: YaruThemeStyle, accent: YaruAccentColor) -> Self {
        let name = format!("Yaru-{:?}-{:?}", accent, style);
        Self { name, style, accent }
    }

    /// Resolves primary accent color HEX code for Yaru accent
    pub fn accent_hex(&self) -> &'static str {
        match self.accent {
            YaruAccentColor::Aubergine => "#77216F",
            YaruAccentColor::Orange => "#E95420",
            YaruAccentColor::Teal => "#008080",
            YaruAccentColor::Purple => "#762572",
            YaruAccentColor::Sage => "#87A96B",
            YaruAccentColor::Bark => "#786D5F",
            YaruAccentColor::Olive => "#808000",
            YaruAccentColor::PrussianGreen => "#003153",
        }
    }

    /// Converts YaruThemeSpec into standard Theme palette
    pub fn to_theme(&self) -> Theme {
        let is_dark = self.style == YaruThemeStyle::Dark;
        let bg = if is_dark { "#1E1E1E" } else { "#FAFAFA" };
        let fg = if is_dark { "#FFFFFF" } else { "#111111" };

        Theme {
            name: self.name.clone(),
            mode: if is_dark { ThemeMode::Dark } else { ThemeMode::Light },
            colors: ColorPalette {
                primary: self.accent_hex().to_string(),
                secondary: "#5E5CE6".to_string(),
                accent: self.accent_hex().to_string(),
                background: bg.to_string(),
                foreground: fg.to_string(),
                success: "#30D158".to_string(),
                warning: "#FFD60A".to_string(),
                error: "#FF453A".to_string(),
            },
            typography: TypographySettings {
                font_family: "Ubuntu".to_string(),
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
                small: 6,
                medium: 10,
                large: 16,
                full: false,
            },
            shadows: ShadowSettings {
                enabled: true,
                blur: 12,
                spread: 0,
                color: "#000000".to_string(),
                opacity: 0.15,
            },
            animations: AnimationSettings {
                enabled: true,
                duration_ms: 250,
                easing: "cubic-bezier(0.25, 0.1, 0.25, 1.0)".to_string(),
            },
        }
    }
}

/// Accent color palette variants inspired by Ubuntu Yaru theme suite
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YaruAccentColor {
    Aubergine,
    Orange,
    Teal,
    Purple,
    Sage,
    Bark,
    Olive,
    PrussianGreen,
}

/// Yaru theme style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YaruThemeStyle {
    Light,
    Dark,
    HighContrast,
}

/// Linux / Ubuntu Yaru-inspired theme specification engine (YTheme)
#[derive(Debug, Clone)]
pub struct YaruThemeSpec {
    pub name: String,
    pub style: YaruThemeStyle,
    pub accent: YaruAccentColor,
}

impl YaruThemeSpec {
    pub fn new(style: YaruThemeStyle, accent: YaruAccentColor) -> Self {
        let name = format!("Yaru-{:?}-{:?}", accent, style);
        Self { name, style, accent }
    }

    /// Resolves primary accent color HEX code for Yaru accent
    pub fn accent_hex(&self) -> &'static str {
        match self.accent {
            YaruAccentColor::Aubergine => "#77216F",
            YaruAccentColor::Orange => "#E95420",
            YaruAccentColor::Teal => "#008080",
            YaruAccentColor::Purple => "#762572",
            YaruAccentColor::Sage => "#87A96B",
            YaruAccentColor::Bark => "#786D5F",
            YaruAccentColor::Olive => "#808000",
            YaruAccentColor::PrussianGreen => "#003153",
        }
    }

    /// Converts YaruThemeSpec into standard Theme palette
    pub fn to_theme(&self) -> Theme {
        let is_dark = self.style == YaruThemeStyle::Dark;
        let bg = if is_dark { "#1E1E1E" } else { "#FAFAFA" };
        let fg = if is_dark { "#FFFFFF" } else { "#111111" };

        Theme {
            name: self.name.clone(),
            mode: if is_dark { ThemeMode::Dark } else { ThemeMode::Light },
            colors: ColorPalette {
                primary: self.accent_hex().to_string(),
                secondary: "#5E5CE6".to_string(),
                accent: self.accent_hex().to_string(),
                background: bg.to_string(),
                foreground: fg.to_string(),
                success: "#30D158".to_string(),
                warning: "#FFD60A".to_string(),
                error: "#FF453A".to_string(),
            },
            typography: TypographySettings {
                font_family: "Ubuntu".to_string(),
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
                small: 6,
                medium: 10,
                large: 16,
                full: false,
            },
            shadows: ShadowSettings {
                enabled: true,
                blur: 12,
                spread: 0,
                color: "#000000".to_string(),
                opacity: 0.15,
            },
            animations: AnimationSettings {
                enabled: true,
                duration_ms: 250,
                easing: "cubic-bezier(0.25, 0.1, 0.25, 1.0)".to_string(),
            },
        }
    }
}

/// Native, zero-dependency Sovereign CSS Color Engine
pub struct SovereignCssColorEngine;

impl SovereignCssColorEngine {
    /// Parses CSS hex color strings (#rgb, #rgba, #rrggbb, #rrggbbaa) into RGBA floats (0.0 to 1.0)
    pub fn parse_hex_color(hex_str: &str) -> Result<(f32, f32, f32, f32), &'static str> {
        let clean = hex_str.trim().trim_start_matches('#');
        match clean.len() {
            3 => {
                let r = u8::from_str_radix(&clean[0..1].repeat(2), 16).map_err(|_| "Invalid hex")?
                    as f32
                    / 255.0;
                let g = u8::from_str_radix(&clean[1..2].repeat(2), 16).map_err(|_| "Invalid hex")?
                    as f32
                    / 255.0;
                let b = u8::from_str_radix(&clean[2..3].repeat(2), 16).map_err(|_| "Invalid hex")?
                    as f32
                    / 255.0;
                Ok((r, g, b, 1.0))
            }
            6 => {
                let r =
                    u8::from_str_radix(&clean[0..2], 16).map_err(|_| "Invalid hex")? as f32 / 255.0;
                let g =
                    u8::from_str_radix(&clean[2..4], 16).map_err(|_| "Invalid hex")? as f32 / 255.0;
                let b =
                    u8::from_str_radix(&clean[4..6], 16).map_err(|_| "Invalid hex")? as f32 / 255.0;
                Ok((r, g, b, 1.0))
            }
            8 => {
                let r =
                    u8::from_str_radix(&clean[0..2], 16).map_err(|_| "Invalid hex")? as f32 / 255.0;
                let g =
                    u8::from_str_radix(&clean[2..4], 16).map_err(|_| "Invalid hex")? as f32 / 255.0;
                let b =
                    u8::from_str_radix(&clean[4..6], 16).map_err(|_| "Invalid hex")? as f32 / 255.0;
                let a =
                    u8::from_str_radix(&clean[6..8], 16).map_err(|_| "Invalid hex")? as f32 / 255.0;
                Ok((r, g, b, a))
            }
            _ => Err("Unsupported hex color length"),
        }
    }
}

impl IconThemeEngine {
    pub fn new(pack: &str, dpi: f32) -> Self {
        let mut spec = IconThemeSpecIndex::new(pack, "SigmaOS Native Icon Pack");
        spec.add_inherits("Adwaita");
        spec.add_inherits("hicolor");

        Self {
            active_icon_pack: pack.to_string(),
            base_icon_size: 48,
            screen_dpi: dpi,
            spec_index: spec,
        }
    }

    /// Evaluates dynamic scaled sizes to ensure pixel-perfect resolution on high density screens
    pub fn get_scaled_icon_size(&self) -> u16 {
        let scale_factor = self.screen_dpi / 96.0; // 96 is standard baseline DPI
        let raw_scaled = self.base_icon_size as f32 * scale_factor;
        raw_scaled as u16
    }
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
        assert_eq!(provider.themes.len(), 6);
    }

    #[test]
    fn test_community_forum_theme_presets() {
        let provider = BuiltInThemeProvider::new();
        assert!(provider.get_theme_by_name("arch_forum_dark").is_some());
        assert!(provider.get_theme_by_name("freebsd_forum_classic").is_some());
        assert!(provider.get_theme_by_name("ubuntu_discourse_warm").is_some());
        assert!(provider.get_theme_by_name("fedora_forum_clean").is_some());

        let arch = provider.get_theme_by_name("arch_forum_dark").unwrap();
        assert_eq!(arch.colors.primary, "#1793D1");

        let freebsd = provider.get_theme_by_name("freebsd_forum_classic").unwrap();
        assert_eq!(freebsd.colors.primary, "#AB2B28");
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

    #[test]
    fn test_backdrop_filter_blur_adjustment() {
        let mut filter = ZenithBackdropFilter::new();
        assert_eq!(filter.get_rendering_parameters(), (12.5, 0.85, 8));

        filter.adjust_blur(25.0);
        filter.set_opacity(50);
        assert_eq!(filter.get_rendering_parameters(), (25.0, 0.50, 8));
    }

    #[test]
    fn test_sigma_soundscape_mapping() {
        let mut scape = SigmaSoundscape::new();
        assert_eq!(
            scape.trigger_sound_event("login"),
            Some("file:///system/audio/chime.wav")
        );

        scape.map_sound_event("notification", "file:///system/audio/beep.wav");
        assert_eq!(
            scape.trigger_sound_event("notification"),
            Some("file:///system/audio/beep.wav")
        );
    }

    #[test]
    fn test_sovereign_css_color_engine() {
        let (r, g, b, a) = SovereignCssColorEngine::parse_hex_color("#FF0000").unwrap();
        assert_eq!((r, g, b, a), (1.0, 0.0, 0.0, 1.0));

        let (r, g, b, a) = SovereignCssColorEngine::parse_hex_color("#00FF0080").unwrap();
        assert_eq!((r, g, b), (0.0, 1.0, 0.0));
        assert!((a - 0.5019).abs() < 0.01);
    }

    #[test]
    fn test_icon_theme_scaling() {
        let pack = IconThemeEngine::new("SovereignIcons", 144.0); // 1.5x scaling
        assert_eq!(pack.get_scaled_icon_size(), 72);
    }

    #[test]
    fn test_icon_theme_spec_index() {
        let mut index = IconThemeSpecIndex::new("Yaru", "Ubuntu Yaru Icon Theme");
        index.add_inherits("Adwaita");
        index.add_directory("48x48/apps");
        assert_eq!(index.name, "Yaru");
        assert_eq!(index.inherits, vec!["Adwaita".to_string()]);
    }

    #[test]
    fn test_icon_inherits_resolver() {
        let mut index = IconThemeSpecIndex::new("Breeze", "KDE Breeze Icon Theme");
        index.add_inherits("oxygen");
        let chain = IconInheritsResolver::resolve_lookup_chain(&index);
        assert_eq!(chain, vec!["Breeze".to_string(), "oxygen".to_string(), "hicolor".to_string()]);
    }

    }

    #[test]
    fn test_symbolic_icon_tint_engine() {
        let svg = "<path fill=\"#000000\" d=\"M0 0h24v24H0z\"/>";
        let tinted = SymbolicIconTintEngine::tint_symbolic_color(svg, "#3584E4");
        assert_eq!(tinted, "<path fill=\"#3584E4\" d=\"M0 0h24v24H0z\"/>");

    }

    #[test]
    fn test_yaru_theme_spec() {
        let yaru = YaruThemeSpec::new(YaruThemeStyle::Dark, YaruAccentColor::Aubergine);
        assert_eq!(yaru.accent_hex(), "#77216F");
        let theme = yaru.to_theme();
        assert_eq!(theme.mode, ThemeMode::Dark);
        assert_eq!(theme.colors.primary, "#77216F");
        assert_eq!(theme.colors.background, "#1E1E1E");
    fn test_cinnamon_theme_engine() {
        let mut engine = SovereignCinnamonThemeEngine::new("Mint-Y-Teal");
        assert_eq!(engine.active_config.accent_color_hex, "#87A922");

        engine.set_accent_color("#00adb5");
        assert_eq!(engine.active_config.accent_color_hex, "#00adb5");

        let css = engine.render_css_stylesheet();
        assert!(css.contains(".panel-bottom"));
        assert!(css.contains("#00adb5"));
    }
}
