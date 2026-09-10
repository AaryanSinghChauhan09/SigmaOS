//! Omarchy-inspired Theme System
//! 
//! This module implements a theme system inspired by Omarchy Linux, which features
//! a visual theme switcher with live previews, semantic color systems, and
//! coordinated theming across desktop, terminal, editor, and applications.

#![allow(dead_code)]



use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// Semantic color names for consistent theming
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SemanticColor {
    /// Primary brand color
    Primary,
    /// Secondary brand color
    Secondary,
    /// Success/positive color
    Success,
    /// Warning color
    Warning,
    /// Error/danger color
    Error,
    /// Info color
    Info,
    /// Background color
    Background,
    /// Surface/panel color
    Surface,
    /// Text color
    Text,
    /// Muted/subtle text color
    TextMuted,
    /// Border color
    Border,
    /// Accent color
    Accent,
}

impl SemanticColor {
    /// Get CSS variable name for the color
    pub fn css_var(&self) -> &'static str {
        match self {
            SemanticColor::Primary => "--color-primary",
            SemanticColor::Secondary => "--color-secondary",
            SemanticColor::Success => "--color-success",
            SemanticColor::Warning => "--color-warning",
            SemanticColor::Error => "--color-error",
            SemanticColor::Info => "--color-info",
            SemanticColor::Background => "--color-background",
            SemanticColor::Surface => "--color-surface",
            SemanticColor::Text => "--color-text",
            SemanticColor::TextMuted => "--color-text-muted",
            SemanticColor::Border => "--color-border",
            SemanticColor::Accent => "--color-accent",
        }
    }
}

/// Theme component
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThemeComponent {
    /// Desktop environment
    Desktop,
    /// Terminal
    Terminal,
    /// Editor (Neovim)
    Editor,
    /// Activity monitor (btop)
    ActivityMonitor,
    /// Notifications (mako)
    Notifications,
    /// Top bar (waybar)
    TopBar,
    /// Application launcher
    Launcher,
    /// Lock screen
    LockScreen,
    /// All components
    All,
}

/// Color definition
#[derive(Debug, Clone)]
pub struct Color {
    /// Hex color code
    pub hex: String,
    /// RGB values
    pub rgb: (u8, u8, u8),
    /// HSL values
    pub hsl: (f32, f32, f32),
}

impl Color {
    /// Create color from hex code
    pub fn from_hex(hex: &str) -> Self {
        let hex_clean = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex_clean[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex_clean[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex_clean[4..6], 16).unwrap_or(0);
        
        let (h, s, l) = Self::rgb_to_hsl(r, g, b);
        
        Self {
            hex: hex.to_string(),
            rgb: (r, g, b),
            hsl: (h, s, l),
        }
    }
    
    /// Convert RGB to HSL
    fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
        let r_f = r as f32 / 255.0;
        let g_f = g as f32 / 255.0;
        let b_f = b as f32 / 255.0;
        
        let max = r_f.max(g_f).max(b_f);
        let min = r_f.min(g_f).min(b_f);
        let delta = max - min;
        
        let l = (max + min) / 2.0;
        
        let h = if delta == 0.0 {
            0.0
        } else if max == r_f {
            60.0 * (((g_f - b_f) / delta) % 6.0)
        } else if max == g_f {
            60.0 * (((b_f - r_f) / delta) + 2.0)
        } else {
            60.0 * (((r_f - g_f) / delta) + 4.0)
        };
        
        let s = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * l - 1.0).abs())
        };
        
        (h, s, l)
    }
}

/// Theme definition
#[derive(Debug, Clone)]
pub struct Theme {
    /// Theme name
    pub name: String,
    /// Theme ID
    pub id: String,
    /// Theme description
    pub description: String,
    /// Whether theme is dark
    pub dark: bool,
    /// Semantic color palette
    pub colors: BTreeMap<SemanticColor, Color>,
    /// Font family
    pub font_family: String,
    /// Font size
    pub font_size: u32,
    /// Border radius
    pub border_radius: u32,
    /// Animation speed (1-10)
    pub animation_speed: u8,
}

impl Theme {
    /// Create a new theme
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
    
    /// Set a semantic color
    pub fn set_color(&mut self, semantic: SemanticColor, color: Color) {
        self.colors.insert(semantic, color);
    }
    
    /// Get a semantic color
    pub fn get_color(&self, semantic: SemanticColor) -> Option<&Color> {
        self.colors.get(&semantic)
    }
    
    /// Generate CSS variables
    pub fn generate_css_vars(&self) -> String {
        let mut css = String::new();
        
        for (semantic, color) in &self.colors {
            css.push_str(&format!("  {}: {};\n", semantic.css_var(), color.hex));
        }
        
        css
    }
    
    /// Generate CSS for specific component
    pub fn generate_component_css(&self, component: ThemeComponent) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("/* {} */\n", format!("{:?}", component)));
        css.push_str(&self.generate_css_vars());
        
        css
    }
}

/// Theme manager - manages theme switching and application
#[derive(Debug)]
pub struct OmarchyThemeManager {
    /// Available themes
    pub themes: Vec<Theme>,
    /// Currently active theme
    pub active_theme: Option<String>,
    /// Theme history for undo
    pub theme_history: Vec<String>,
}

impl OmarchyThemeManager {
    /// Create a new Theme Manager
    pub fn new() -> Self {
        Self {
            themes: Vec::new(),
            active_theme: None,
            theme_history: Vec::new(),
        }
    }
    
    /// Add a theme
    pub fn add_theme(&mut self, theme: Theme) {
        self.themes.push(theme);
    }
    
    /// Remove a theme
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
    
    /// Set active theme
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
    
    /// Get active theme
    pub fn get_active_theme(&self) -> Option<&Theme> {
        if let Some(id) = &self.active_theme {
            self.themes.iter().find(|t| &t.id == id)
        } else {
            None
        }
    }
    
    /// Get theme by ID
    pub fn get_theme(&self, theme_id: &str) -> Option<&Theme> {
        self.themes.iter().find(|t| t.id == theme_id)
    }
    
    /// Get dark themes
    pub fn get_dark_themes(&self) -> Vec<&Theme> {
        self.themes.iter().filter(|t| t.dark).collect()
    }
    
    /// Get light themes
    pub fn get_light_themes(&self) -> Vec<&Theme> {
        self.themes.iter().filter(|t| !t.dark).collect()
    }
    
    /// Search themes by name
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
    
    /// Undo last theme change
    pub fn undo_theme_change(&mut self) -> Result<(), String> {
        if let Some(previous) = self.theme_history.pop() {
            self.active_theme = Some(previous);
            Ok(())
        } else {
            Err("No theme history to undo".to_string())
        }
    }
    
    /// Apply theme to specific component
    pub fn apply_to_component(&self, component: ThemeComponent) -> Result<String, String> {
        if let Some(theme) = self.get_active_theme() {
            Ok(theme.generate_component_css(component))
        } else {
            Err("No active theme".to_string())
        }
    }
    
    /// Apply theme to all components
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
    
    /// Create default themes
    pub fn create_default_themes(&mut self) {
        // Dark theme
        let mut dark_theme = Theme::new("Dark".to_string(), "dark".to_string(), true);
        dark_theme.description = "Dark theme with high contrast".to_string();
        dark_theme.set_color(SemanticColor::Background, Color::from_hex("#1e1e2e"));
        dark_theme.set_color(SemanticColor::Surface, Color::from_hex("#313244"));
        dark_theme.set_color(SemanticColor::Text, Color::from_hex("#cdd6f4"));
        dark_theme.set_color(SemanticColor::Primary, Color::from_hex("#cba6f7"));
        dark_theme.set_color(SemanticColor::Accent, Color::from_hex("#89b4fa"));
        self.add_theme(dark_theme);
        
        // Light theme
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_theme_manager_creation() {
        let manager = OmarchyThemeManager::new();
        assert_eq!(manager.themes.len(), 0);
    }
    
    #[test]
    fn test_add_theme() {
        let mut manager = OmarchyThemeManager::new();
        let theme = Theme::new("Test".to_string(), "test".to_string(), true);
        manager.add_theme(theme);
        assert_eq!(manager.themes.len(), 1);
    }
    
    #[test]
    fn test_set_active_theme() {
        let mut manager = OmarchyThemeManager::new();
        let theme = Theme::new("Test".to_string(), "test".to_string(), true);
        manager.add_theme(theme);
        
        let result = manager.set_active_theme("test".to_string());
        assert!(result.is_ok());
        assert_eq!(manager.active_theme, Some("test".to_string()));
    }
    
    #[test]
    fn test_color_from_hex() {
        let color = Color::from_hex("#ff0000");
        assert_eq!(color.hex, "#ff0000");
        assert_eq!(color.rgb, (255, 0, 0));
    }
    
    #[test]
    fn test_generate_css_vars() {
        let mut theme = Theme::new("Test".to_string(), "test".to_string(), true);
        theme.set_color(SemanticColor::Primary, Color::from_hex("#ff0000"));
        
        let css = theme.generate_css_vars();
        assert!(css.contains("--color-primary"));
        assert!(css.contains("#ff0000"));
    }
    
    #[test]
    fn test_default_themes() {
        let mut manager = OmarchyThemeManager::new();
        manager.create_default_themes();
        
        assert_eq!(manager.themes.len(), 2);
        assert!(manager.get_dark_themes().len() > 0);
        assert!(manager.get_light_themes().len() > 0);
    }
}
