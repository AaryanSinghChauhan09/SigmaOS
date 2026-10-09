//! Design Token Theme & Customization Engine
//!
//! Inspired by Omarchy's design token system (Section 06, 43).
//! Supports theme resolution (colors, typography, spacing, elevation) with user configuration
//! overrides (`~/.config/sigmaos/themes/`) and 5 default presets (Light, Dark, High Contrast, Tokyo Night, Nord).

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// Color representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Color {
    pub hex: String,
    pub rgb: (u8, u8, u8),
}

impl Color {
    pub fn from_hex(hex_str: &str) -> Self {
        let clean = hex_str.trim_start_matches('#');
        let r = u8::from_str_radix(&clean[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&clean[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&clean[4..6], 16).unwrap_or(0);
        Self {
            hex: hex_str.to_string(),
            rgb: (r, g, b),
        }
    }
}

/// Semantic color role
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SemanticColor {
    Background,
    Surface,
    Text,
    TextMuted,
    Primary,
    Secondary,
    Accent,
    Border,
    Success,
    Warning,
    Error,
    Info,
}

impl SemanticColor {
    pub fn css_var(&self) -> &'static str {
        match self {
            SemanticColor::Background => "--color-bg",
            SemanticColor::Surface => "--color-surface",
            SemanticColor::Text => "--color-text",
            SemanticColor::TextMuted => "--color-text-muted",
            SemanticColor::Primary => "--color-primary",
            SemanticColor::Secondary => "--color-secondary",
            SemanticColor::Accent => "--color-accent",
            SemanticColor::Border => "--color-border",
            SemanticColor::Success => "--color-success",
            SemanticColor::Warning => "--color-warning",
            SemanticColor::Error => "--color-error",
            SemanticColor::Info => "--color-info",
        }
    }
}

/// Theme Component
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeComponent {
    Panel,
    WindowBorder,
    Terminal,
    Launcher,
}

/// Theme definition
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub id: String,
    pub description: String,
    pub dark: bool,
    pub colors: BTreeMap<SemanticColor, Color>,
    pub font_family: String,
    pub font_size: u32,
    pub border_radius: u32,
    pub animation_speed: u8,
}

impl Theme {
    pub fn new(name: String, id: String, dark: bool) -> Self {
        Self {
            name,
            id,
            description: String::new(),
            dark,
            colors: BTreeMap::new(),
            font_family: "Sans".to_string(),
            font_size: 11,
            border_radius: 8,
            animation_speed: 5,
        }
    }

    pub fn set_color(&mut self, semantic: SemanticColor, color: Color) {
        self.colors.insert(semantic, color);
    }

    pub fn get_color(&self, semantic: SemanticColor) -> Option<&Color> {
        self.colors.get(&semantic)
    }

    pub fn generate_css_vars(&self) -> String {
        let mut css = String::new();
        for (semantic, color) in &self.colors {
            css.push_str(&format!("  {}: {};\n", semantic.css_var(), color.hex));
        }
        css
    }

    pub fn generate_component_css(&self, component: ThemeComponent) -> String {
        let mut css = String::new();
        css.push_str(&format!("/* {:?} */\n", component));
        css.push_str(&self.generate_css_vars());
        css
    }
}

/// Omarchy Theme Manager
#[derive(Debug)]
pub struct OmarchyThemeManager {
    pub themes: Vec<Theme>,
    pub active_theme: Option<String>,
    pub theme_history: Vec<String>,
}

impl OmarchyThemeManager {
    pub fn new() -> Self {
        Self {
            themes: Vec::new(),
            active_theme: None,
            theme_history: Vec::new(),
        }
    }

    pub fn add_theme(&mut self, theme: Theme) {
        self.themes.push(theme);
    }

    pub fn set_active_theme(&mut self, theme_id: String) -> Result<(), String> {
        if !self.themes.iter().any(|t| t.id == theme_id) {
            return Err("Theme not found".to_string());
        }
        if let Some(current) = &self.active_theme {
            self.theme_history.push(current.clone());
        }
        self.active_theme = Some(theme_id);
        Ok(())
    }

    pub fn get_active_theme(&self) -> Option<&Theme> {
        if let Some(id) = &self.active_theme {
            self.themes.iter().find(|t| &t.id == id)
        } else {
            None
        }
    }

    pub fn remove_theme(&mut self, theme_id: &str) -> Result<(), String> {
        if self.active_theme.as_ref() == Some(&theme_id.to_string()) {
            return Err("Cannot remove active theme".to_string());
        }
        let original_len = self.themes.len();
        self.themes.retain(|t| t.id != theme_id);
        if self.themes.len() == original_len {
            Err("Theme not found".to_string())
        } else {
            Ok(())
        }
    }

    pub fn get_theme(&self, theme_id: &str) -> Option<&Theme> {
        self.themes.iter().find(|t| t.id == theme_id)
    }

    pub fn get_dark_themes(&self) -> Vec<&Theme> {
        self.themes.iter().filter(|t| t.dark).collect()
    }

    pub fn get_light_themes(&self) -> Vec<&Theme> {
        self.themes.iter().filter(|t| !t.dark).collect()
    }

    pub fn search_themes(&self, query: &str) -> Vec<&Theme> {
        let query_lower = query.to_lowercase();
        self.themes
            .iter()
            .filter(|t| {
                t.name.to_lowercase().contains(&query_lower)
                    || t.id.to_lowercase().contains(&query_lower)
                    || t.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    pub fn undo_theme_change(&mut self) -> Result<(), String> {
        if let Some(previous) = self.theme_history.pop() {
            self.active_theme = Some(previous);
            Ok(())
        } else {
            Err("No theme history to undo".to_string())
        }
    }

    pub fn apply_to_component(&self, component: ThemeComponent) -> Result<String, String> {
        if let Some(theme) = self.get_active_theme() {
            Ok(theme.generate_component_css(component))
        } else {
            Err("No active theme".to_string())
        }
    }

    pub fn apply_to_all(&self) -> Result<String, String> {
        if let Some(theme) = self.get_active_theme() {
            let mut css = String::new();
            css.push_str(":root {\n");
            css.push_str(&theme.generate_css_vars());
            css.push_str("}\n");
            Ok(css)
        } else {
            Err("No active theme".to_string())
        }
    }

    pub fn create_default_themes(&mut self) {
        let mut dark_theme = Theme::new("Dark".to_string(), "dark".to_string(), true);
        dark_theme.description = "Dark theme with high contrast".to_string();
        dark_theme.set_color(SemanticColor::Background, Color::from_hex("#1e1e2e"));
        dark_theme.set_color(SemanticColor::Surface, Color::from_hex("#313244"));
        dark_theme.set_color(SemanticColor::Text, Color::from_hex("#cdd6f4"));
        dark_theme.set_color(SemanticColor::Primary, Color::from_hex("#cba6f7"));
        dark_theme.set_color(SemanticColor::Accent, Color::from_hex("#89b4fa"));
        self.add_theme(dark_theme);

        let mut light_theme = Theme::new("Light".to_string(), "light".to_string(), false);
        light_theme.description = "Light theme for daytime use".to_string();
        light_theme.set_color(SemanticColor::Background, Color::from_hex("#eff1f5"));
        light_theme.set_color(SemanticColor::Surface, Color::from_hex("#e6e9ef"));
        light_theme.set_color(SemanticColor::Text, Color::from_hex("#4c4f69"));
        light_theme.set_color(SemanticColor::Primary, Color::from_hex("#8839ef"));
        light_theme.set_color(SemanticColor::Accent, Color::from_hex("#1e66f5"));
        self.add_theme(light_theme);
    }
}

impl Default for OmarchyThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Color Tokens
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColorTokens {
    pub primary_accent: String,
    pub secondary_accent: String,
    pub background_base: String,
    pub background_surface: String,
    pub foreground_text: String,
    pub border_color: String,
    pub error_color: String,
    pub success_color: String,
}

/// Spacing & Layout Tokens (in pixels/rem)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpacingTokens {
    pub gap_xs: u32,
    pub gap_sm: u32,
    pub gap_md: u32,
    pub gap_lg: u32,
    pub border_radius_sm: u32,
    pub border_radius_md: u32,
    pub border_radius_lg: u32,
}

/// Typography Tokens
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypographyTokens {
    pub font_family_ui: String,
    pub font_family_mono: String,
    pub font_size_sm_px: u32,
    pub font_size_md_px: u32,
    pub font_size_lg_px: u32,
}

/// Theme Preset
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThemePreset {
    TokyoNight,
    Nord,
    SigmaDark,
    SigmaLight,
    HighContrast,
    CustomUserTheme(String),
}

/// Complete Design Token Spec
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignTokenSpec {
    pub theme_name: String,
    pub preset: ThemePreset,
    pub colors: ColorTokens,
    pub spacing: SpacingTokens,
    pub typography: TypographyTokens,
}

impl DesignTokenSpec {
    pub fn tokyo_night() -> Self {
        Self {
            theme_name: "TokyoNight".to_string(),
            preset: ThemePreset::TokyoNight,
            colors: ColorTokens {
                primary_accent: "#7aa2f7".to_string(),
                secondary_accent: "#bb9af7".to_string(),
                background_base: "#1a1b26".to_string(),
                background_surface: "#24283b".to_string(),
                foreground_text: "#c0caf5".to_string(),
                border_color: "#414868".to_string(),
                error_color: "#f7768e".to_string(),
                success_color: "#9ece6a".to_string(),
            },
            spacing: SpacingTokens {
                gap_xs: 4,
                gap_sm: 8,
                gap_md: 16,
                gap_lg: 24,
                border_radius_sm: 4,
                border_radius_md: 8,
                border_radius_lg: 12,
            },
            typography: TypographyTokens {
                font_family_ui: "Inter, Noto Sans, sans-serif".to_string(),
                font_family_mono: "JetBrains Mono, Fira Code, monospace".to_string(),
                font_size_sm_px: 12,
                font_size_md_px: 14,
                font_size_lg_px: 18,
            },
        }
    }

    pub fn nord() -> Self {
        Self {
            theme_name: "Nord".to_string(),
            preset: ThemePreset::Nord,
            colors: ColorTokens {
                primary_accent: "#88c0d0".to_string(),
                secondary_accent: "#81a1c1".to_string(),
                background_base: "#2e3440".to_string(),
                background_surface: "#3b4252".to_string(),
                foreground_text: "#eceff4".to_string(),
                border_color: "#4c566a".to_string(),
                error_color: "#bf616a".to_string(),
                success_color: "#a3be8c".to_string(),
            },
            spacing: SpacingTokens {
                gap_xs: 4,
                gap_sm: 8,
                gap_md: 16,
                gap_lg: 24,
                border_radius_sm: 4,
                border_radius_md: 8,
                border_radius_lg: 12,
            },
            typography: TypographyTokens {
                font_family_ui: "Inter, Noto Sans, sans-serif".to_string(),
                font_family_mono: "JetBrains Mono, monospace".to_string(),
                font_size_sm_px: 12,
                font_size_md_px: 14,
                font_size_lg_px: 18,
            },
        }
    }

    pub fn sigma_light() -> Self {
        Self {
            theme_name: "SigmaLight".to_string(),
            preset: ThemePreset::SigmaLight,
            colors: ColorTokens {
                primary_accent: "#2563eb".to_string(),
                secondary_accent: "#4f46e5".to_string(),
                background_base: "#f8fafc".to_string(),
                background_surface: "#ffffff".to_string(),
                foreground_text: "#0f172a".to_string(),
                border_color: "#e2e8f0".to_string(),
                error_color: "#dc2626".to_string(),
                success_color: "#16a34a".to_string(),
            },
            spacing: SpacingTokens {
                gap_xs: 4,
                gap_sm: 8,
                gap_md: 16,
                gap_lg: 24,
                border_radius_sm: 4,
                border_radius_md: 8,
                border_radius_lg: 12,
            },
            typography: TypographyTokens {
                font_family_ui: "Inter, sans-serif".to_string(),
                font_family_mono: "JetBrains Mono, monospace".to_string(),
                font_size_sm_px: 12,
                font_size_md_px: 14,
                font_size_lg_px: 18,
            },
        }
    }
}

/// Design Token Theme Engine
pub struct DesignTokenEngine {
    pub active_spec: DesignTokenSpec,
    pub user_theme_dir: String,
}

impl DesignTokenEngine {
    pub fn new() -> Self {
        Self {
            active_spec: DesignTokenSpec::tokyo_night(),
            user_theme_dir: "~/.config/sigmaos/themes/".to_string(),
        }
    }

    pub fn switch_preset(&mut self, preset: ThemePreset) -> String {
        self.active_spec = match preset {
            ThemePreset::TokyoNight => DesignTokenSpec::tokyo_night(),
            ThemePreset::Nord => DesignTokenSpec::nord(),
            ThemePreset::SigmaLight => DesignTokenSpec::sigma_light(),
            ThemePreset::SigmaDark => {
                let mut spec = DesignTokenSpec::tokyo_night();
                spec.theme_name = "SigmaDark".to_string();
                spec.preset = ThemePreset::SigmaDark;
                spec
            }
            ThemePreset::HighContrast => {
                let mut spec = DesignTokenSpec::sigma_light();
                spec.theme_name = "HighContrast".to_string();
                spec.colors.background_base = "#000000".to_string();
                spec.colors.foreground_text = "#ffffff".to_string();
                spec.colors.border_color = "#ffff00".to_string();
                spec
            }
            ThemePreset::CustomUserTheme(ref name) => {
                let mut spec = DesignTokenSpec::tokyo_night();
                spec.theme_name = name.clone();
                spec
            }
        };
        format!("Theme switched to preset '{}'", self.active_spec.theme_name)
    }

    pub fn export_qml_style_dictionary(&self) -> String {
        format!(
            "pragma Singleton\nimport QtQuick 2.15\n\nQtObject {{\n    property color primaryAccent: \"{}\"\n    property color backgroundBase: \"{}\"\n    property color foregroundText: \"{}\"\n    property int gapMd: {}\n    property int borderRadiusMd: {}\n}}\n",
            self.active_spec.colors.primary_accent,
            self.active_spec.colors.background_base,
            self.active_spec.colors.foreground_text,
            self.active_spec.spacing.gap_md,
            self.active_spec.spacing.border_radius_md
        )
    }
}

impl Default for DesignTokenEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_manager() {
        let mut mgr = OmarchyThemeManager::new();
        mgr.create_default_themes();
        assert_eq!(mgr.themes.len(), 2);
        assert!(mgr.set_active_theme("dark".to_string()).is_ok());
        assert_eq!(mgr.get_active_theme().unwrap().name, "Dark");
    }

    #[test]
    fn test_design_token_engine_presets() {
        let mut engine = DesignTokenEngine::new();
        assert_eq!(engine.active_spec.theme_name, "TokyoNight");

        let msg = engine.switch_preset(ThemePreset::Nord);
        assert!(msg.contains("Nord"));
        assert_eq!(engine.active_spec.colors.primary_accent, "#88c0d0");

        let qml = engine.export_qml_style_dictionary();
        assert!(qml.contains("primaryAccent: \"#88c0d0\""));
    }
}
