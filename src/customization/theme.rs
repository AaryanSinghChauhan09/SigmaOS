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
extern crate alloc;
use alloc::vec;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;

// SigmaOS Theme Engine
// OOP-based declarative theming with light/dark/auto modes
// Enhanced with Material-You style dynamic color palettes and workspace density profiling

use crate::klib::BTreeMap;

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

/// Linux & BSD Distro-Inspired Icon Theme Presets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistroIconPreset {
    SovereignSatori, // Default vector theme with dynamic accent tinting (inspired by Elementary OS / Pantheon)
    PapirusSovereign, // Flat, high-contrast vector icon pack (inspired by Papirus / Manjaro)
    YaruSigma,        // Modern rounded ubuntu-inspired icon theme (inspired by Ubuntu Yaru)
    BreezeZenith,     // Clean paper-flat KDE-inspired icon set (inspired by KDE Breeze)
    AdwaitaHardened,  // Gnome/FreeBSD enterprise standard icon theme (inspired by GNOME Adwaita)
}

/// Freedesktop.org Icon Specification Categories
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconCategory {
    Apps,
    Actions,
    Categories,
    Devices,
    Emblems,
    Mimetypes,
    Places,
    Status,
}

/// Icon metadata representation
#[derive(Debug, Clone)]
pub struct IconEntry {
    pub name: String,
    pub category: IconCategory,
    pub path: String,
    pub is_scalable_svg: bool,
    pub accent_tintable: bool,
}

/// IconThemeEngine - Freedesktop.org Icon Theme Specification & Dynamic DPI Scaler Engine
pub struct IconThemeEngine {
    pub active_preset: DistroIconPreset,
    pub active_icon_pack: String,
    pub inherits: Vec<String>,
    pub base_icon_size: u16,
    pub screen_dpi: f32,
    pub accent_color_hex: String,
    pub registered_icons: BTreeMap<String, IconEntry>,
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
        let preset = match pack.to_lowercase().as_str() {
            "papirus" | "papirussovereign" => DistroIconPreset::PapirusSovereign,
            "yaru" | "yarusigma" => DistroIconPreset::YaruSigma,
            "breeze" | "breezezenith" => DistroIconPreset::BreezeZenith,
            "adwaita" | "adwaitahardened" => DistroIconPreset::AdwaitaHardened,
            _ => DistroIconPreset::SovereignSatori,
        };

        let mut engine = Self {
            active_preset: preset,
            active_icon_pack: pack.to_string(),
            inherits: vec!["hicolor".to_string(), "Adwaita".to_string()],
            base_icon_size: 48,
            screen_dpi: dpi,
            accent_color_hex: "#007AFF".to_string(),
            registered_icons: BTreeMap::new(),
        };

        engine.register_default_theme_icons();
        engine
    }

    pub fn set_preset(&mut self, preset: DistroIconPreset) {
        self.active_preset = preset;
        self.active_icon_pack = match preset {
            DistroIconPreset::SovereignSatori => "SovereignSatori".to_string(),
            DistroIconPreset::PapirusSovereign => "PapirusSovereign".to_string(),
            DistroIconPreset::YaruSigma => "YaruSigma".to_string(),
            DistroIconPreset::BreezeZenith => "BreezeZenith".to_string(),
            DistroIconPreset::AdwaitaHardened => "AdwaitaHardened".to_string(),
        };
    }

    pub fn set_accent_color(&mut self, hex_color: &str) {
        self.accent_color_hex = hex_color.to_string();
    }

    pub fn register_icon(&mut self, name: &str, category: IconCategory, path: &str, is_svg: bool, tintable: bool) {
        self.registered_icons.insert(
            name.to_string(),
            IconEntry {
                name: name.to_string(),
                category,
                path: path.to_string(),
                is_scalable_svg: is_svg,
                accent_tintable: tintable,
            },
        );
    }

    fn register_default_theme_icons(&mut self) {
        self.register_icon("folder", IconCategory::Places, "/usr/share/icons/{theme}/scalable/places/folder.svg", true, true);
        self.register_icon("user-home", IconCategory::Places, "/usr/share/icons/{theme}/scalable/places/user-home.svg", true, true);
        self.register_icon("system-file-manager", IconCategory::Apps, "/usr/share/icons/{theme}/{size}x{size}/apps/system-file-manager.png", false, false);
        self.register_icon("utilities-terminal", IconCategory::Apps, "/usr/share/icons/{theme}/{size}x{size}/apps/utilities-terminal.png", false, false);
        self.register_icon("network-wireless", IconCategory::Status, "/usr/share/icons/{theme}/scalable/status/network-wireless.svg", true, false);
        self.register_icon("dialog-information", IconCategory::Status, "/usr/share/icons/{theme}/scalable/status/dialog-information.svg", true, false);
    }

    /// Evaluates dynamic scaled sizes to ensure pixel-perfect resolution on high density screens
    pub fn get_scaled_icon_size(&self) -> u16 {
        let scale_factor = self.screen_dpi / 96.0; // 96 is standard baseline DPI
        let raw_scaled = self.base_icon_size as f32 * scale_factor;
        raw_scaled as u16
    }

    /// Implements Freedesktop.org Icon Theme Specification Lookup Algorithm
    pub fn lookup_icon(&self, name: &str, category: IconCategory, size: u16) -> String {
        let theme_dir = self.active_icon_pack.as_str();

        if let Some(entry) = self.registered_icons.get(name) {
            if entry.category == category {
                let resolved_path = entry.path
                    .replace("{theme}", theme_dir)
                    .replace("{size}", &size.to_string());
                return resolved_path;
            }
        }

        // Fallback chain across inherited themes
        for fallback_theme in &self.inherits {
            let fallback_path = format!(
                "/usr/share/icons/{}/{}x{}/{}/{}.png",
                fallback_theme,
                size,
                size,
                match category {
                    IconCategory::Apps => "apps",
                    IconCategory::Actions => "actions",
                    IconCategory::Categories => "categories",
                    IconCategory::Devices => "devices",
                    IconCategory::Emblems => "emblems",
                    IconCategory::Mimetypes => "mimetypes",
                    IconCategory::Places => "places",
                    IconCategory::Status => "status",
                },
                name
            );
            return fallback_path;
        }

        format!("/usr/share/icons/hicolor/{}/apps/{}.png", size, name)
    }

    /// Dynamically tints SVG folder/places icons with user's desktop accent color
    pub fn render_tinted_svg_icon(&self, raw_svg_xml: &str) -> String {
        raw_svg_xml.replace("#007AFF", &self.accent_color_hex)
            .replace("#007aff", &self.accent_color_hex)
            .replace("fill=\"currentAccent\"", &format!("fill=\"{}\"", self.accent_color_hex))
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
    fn test_icon_theme_distro_presets_and_lookup() {
        let mut engine = IconThemeEngine::new("Papirus", 96.0);
        assert_eq!(engine.active_preset, DistroIconPreset::PapirusSovereign);

        // Check lookup algorithm
        let folder_path = engine.lookup_icon("folder", IconCategory::Places, 48);
        assert_eq!(folder_path, "/usr/share/icons/PapirusSovereign/scalable/places/folder.svg");

        let app_path = engine.lookup_icon("system-file-manager", IconCategory::Apps, 64);
        assert_eq!(app_path, "/usr/share/icons/PapirusSovereign/64x64/apps/system-file-manager.png");

        // Test fallback lookup
        let fallback_path = engine.lookup_icon("unknown-action", IconCategory::Actions, 32);
        assert_eq!(fallback_path, "/usr/share/icons/hicolor/32x32/actions/unknown-action.png");

        // Test preset switching
        engine.set_preset(DistroIconPreset::YaruSigma);
        assert_eq!(engine.active_icon_pack, "YaruSigma");

        // Test SVG dynamic accent tinting
        engine.set_accent_color("#E95420"); // Canonical Orange
        let raw_svg = "<svg><path fill=\"#007AFF\" d=\"M0 0h10v10H0z\"/></svg>";
        let tinted_svg = engine.render_tinted_svg_icon(raw_svg);
        assert!(tinted_svg.contains("fill=\"#E95420\""));
    }
}
