// src/theming/theme_engine.rs
// Theme engine for SigmaOS
// Agent-powered theme generation and management
//
// Features:
// - Color scheme extraction from wallpapers
// - Agent-generated themes
// - Hot-reload support
// - Theme marketplace integration

#![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use core::fmt;

use crate::ai::agent_runtime::{AgentId, SovereignAgentRuntime};

/// RGBA color representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
    
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    
    pub fn to_hex(&self) -> String {
        if self.a == 255 {
            alloc::format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
        } else {
            alloc::format!("#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a)
        }
    }
    
    pub fn from_hex(hex: &str) -> Result<Self, ThemeError> {
        let hex = hex.trim_start_matches('#');
        
        match hex.len() {
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| ThemeError::InvalidColor)?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| ThemeError::InvalidColor)?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| ThemeError::InvalidColor)?;
                Ok(Self::rgb(r, g, b))
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| ThemeError::InvalidColor)?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| ThemeError::InvalidColor)?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| ThemeError::InvalidColor)?;
                let a = u8::from_str_radix(&hex[6..8], 16).map_err(|_| ThemeError::InvalidColor)?;
                Ok(Self::rgba(r, g, b, a))
            }
            _ => Err(ThemeError::InvalidColor),
        }
    }
    
    /// Blend two colors together
    pub fn blend(&self, other: &Color, ratio: f32) -> Self {
        let ratio = ratio.clamp(0.0, 1.0);
        let inv = 1.0 - ratio;
        
        Self::rgba(
            ((self.r as f32 * inv) + (other.r as f32 * ratio)) as u8,
            ((self.g as f32 * inv) + (other.g as f32 * ratio)) as u8,
            ((self.b as f32 * inv) + (other.b as f32 * ratio)) as u8,
            ((self.a as f32 * inv) + (other.a as f32 * ratio)) as u8,
        )
    }
    
    /// Calculate luminance (0.0 - 1.0)
    pub fn luminance(&self) -> f32 {
        let r = (self.r as f32 / 255.0).powf(2.2);
        let g = (self.g as f32 / 255.0).powf(2.2);
        let b = (self.b as f32 / 255.0).powf(2.2);
        
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }
    
    /// Check if color is light or dark
    pub fn is_light(&self) -> bool {
        self.luminance() > 0.5
    }
}

/// Complete theme specification
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub colors: ColorScheme,
    pub typography: Typography,
    pub spacing: Spacing,
    pub borders: Borders,
    pub shadows: Shadows,
    pub animations: Animations,
}

/// Color scheme for the theme
#[derive(Debug, Clone)]
pub struct ColorScheme {
    pub background: Color,
    pub foreground: Color,
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,
    pub surface: Color,
    pub border: Color,
}

impl ColorScheme {
    pub fn dark_default() -> Self {
        Self {
            background: Color::rgb(30, 30, 30),
            foreground: Color::rgb(255, 255, 255),
            primary: Color::rgb(100, 149, 237),
            secondary: Color::rgb(108, 99, 255),
            accent: Color::rgb(255, 110, 180),
            success: Color::rgb(76, 175, 80),
            warning: Color::rgb(255, 152, 0),
            error: Color::rgb(244, 67, 54),
            info: Color::rgb(33, 150, 243),
            surface: Color::rgb(40, 40, 40),
            border: Color::rgb(60, 60, 60),
        }
    }
    
    pub fn light_default() -> Self {
        Self {
            background: Color::rgb(255, 255, 255),
            foreground: Color::rgb(0, 0, 0),
            primary: Color::rgb(25, 118, 210),
            secondary: Color::rgb(156, 39, 176),
            accent: Color::rgb(255, 64, 129),
            success: Color::rgb(56, 142, 60),
            warning: Color::rgb(251, 140, 0),
            error: Color::rgb(211, 47, 47),
            info: Color::rgb(2, 136, 209),
            surface: Color::rgb(245, 245, 245),
            border: Color::rgb(224, 224, 224),
        }
    }
}

/// Typography settings
#[derive(Debug, Clone)]
pub struct Typography {
    pub font_family: String,
    pub font_size_base: u32,
    pub font_size_small: u32,
    pub font_size_large: u32,
    pub line_height: f32,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            font_family: "Inter".into(),
            font_size_base: 14,
            font_size_small: 12,
            font_size_large: 18,
            line_height: 1.5,
        }
    }
}

/// Spacing system
#[derive(Debug, Clone)]
pub struct Spacing {
    pub unit: u32,
    pub xs: u32,
    pub sm: u32,
    pub md: u32,
    pub lg: u32,
    pub xl: u32,
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            unit: 4,
            xs: 4,
            sm: 8,
            md: 16,
            lg: 24,
            xl: 32,
        }
    }
}

/// Border settings
#[derive(Debug, Clone)]
pub struct Borders {
    pub width: u32,
    pub radius_small: u32,
    pub radius_medium: u32,
    pub radius_large: u32,
}

impl Default for Borders {
    fn default() -> Self {
        Self {
            width: 1,
            radius_small: 4,
            radius_medium: 8,
            radius_large: 16,
        }
    }
}

/// Shadow settings
#[derive(Debug, Clone)]
pub struct Shadows {
    pub small: Shadow,
    pub medium: Shadow,
    pub large: Shadow,
}

#[derive(Debug, Clone)]
pub struct Shadow {
    pub offset_x: i32,
    pub offset_y: i32,
    pub blur: u32,
    pub spread: i32,
    pub color: Color,
}

impl Default for Shadows {
    fn default() -> Self {
        Self {
            small: Shadow {
                offset_x: 0,
                offset_y: 1,
                blur: 3,
                spread: 0,
                color: Color::rgba(0, 0, 0, 50),
            },
            medium: Shadow {
                offset_x: 0,
                offset_y: 4,
                blur: 6,
                spread: -1,
                color: Color::rgba(0, 0, 0, 70),
            },
            large: Shadow {
                offset_x: 0,
                offset_y: 10,
                blur: 15,
                spread: -3,
                color: Color::rgba(0, 0, 0, 90),
            },
        }
    }
}

/// Animation settings
#[derive(Debug, Clone)]
pub struct Animations {
    pub duration_fast: u32,
    pub duration_normal: u32,
    pub duration_slow: u32,
    pub easing: String,
}

impl Default for Animations {
    fn default() -> Self {
        Self {
            duration_fast: 150,
            duration_normal: 300,
            duration_slow: 500,
            easing: "ease-in-out".into(),
        }
    }
}

/// Theme engine for managing themes
pub struct ThemeEngine {
    active_theme: Theme,
    themes: BTreeMap<String, Theme>,
    agent_runtime: Option<AgentId>,
}

impl ThemeEngine {
    pub fn new() -> Self {
        let default_theme = Theme {
            name: "SigmaDefault".into(),
            colors: ColorScheme::dark_default(),
            typography: Typography::default(),
            spacing: Spacing::default(),
            borders: Borders::default(),
            shadows: Shadows::default(),
            animations: Animations::default(),
        };
        
        let mut themes = BTreeMap::new();
        themes.insert("SigmaDefault".into(), default_theme.clone());
        
        Self {
            active_theme: default_theme,
            themes,
            agent_runtime: None,
        }
    }
    
    pub fn set_active_theme(&mut self, name: &str) -> Result<(), ThemeError> {
        let theme = self.themes
            .get(name)
            .ok_or(ThemeError::ThemeNotFound)?
            .clone();
        
        self.active_theme = theme;
        Ok(())
    }
    
    pub fn get_active_theme(&self) -> &Theme {
        &self.active_theme
    }
    
    pub fn add_theme(&mut self, theme: Theme) {
        self.themes.insert(theme.name.clone(), theme);
    }
    
    pub fn remove_theme(&mut self, name: &str) -> Result<(), ThemeError> {
        if name == "SigmaDefault" {
            return Err(ThemeError::CannotRemoveDefault);
        }
        
        self.themes.remove(name).ok_or(ThemeError::ThemeNotFound)?;
        Ok(())
    }
    
    pub fn list_themes(&self) -> Vec<String> {
        self.themes.keys().cloned().collect()
    }
    
    /// Extract colors from an image (wallpaper)
    pub fn extract_from_image(&self, image_data: &[u8]) -> Result<ColorScheme, ThemeError> {
        // Simple color extraction algorithm
        // In production, would use k-means clustering or similar
        
        if image_data.len() < 12 {
            return Err(ThemeError::InvalidImage);
        }
        
        // Sample dominant colors (simplified)
        let mut colors: Vec<Color> = Vec::new();
        
        for chunk in image_data.chunks(4) {
            if chunk.len() >= 3 {
                colors.push(Color::rgb(chunk[0], chunk[1], chunk[2]));
            }
        }
        
        // Sort by luminance
        colors.sort_by(|a, b| {
            a.luminance().partial_cmp(&b.luminance()).unwrap()
        });
        
        // Pick dark/light and accent colors
        let background = colors.first().copied().unwrap_or(Color::rgb(30, 30, 30));
        let foreground = colors.last().copied().unwrap_or(Color::rgb(255, 255, 255));
        let accent = colors.get(colors.len() / 2).copied().unwrap_or(Color::rgb(100, 149, 237));
        
        Ok(ColorScheme {
            background,
            foreground,
            primary: accent,
            secondary: accent.blend(&foreground, 0.3),
            accent,
            success: Color::rgb(76, 175, 80),
            warning: Color::rgb(255, 152, 0),
            error: Color::rgb(244, 67, 54),
            info: Color::rgb(33, 150, 243),
            surface: background.blend(&foreground, 0.05),
            border: background.blend(&foreground, 0.1),
        })
    }
    
    /// Generate theme from description using AI agent
    pub fn generate_from_description(
        &mut self,
        description: &str,
        agent: &mut SovereignAgentRuntime,
    ) -> Result<Theme, ThemeError> {
        // TODO: Integrate with agent runtime for AI generation
        // For now, return a variant based on keywords
        
        let is_dark = description.to_lowercase().contains("dark");
        let is_light = description.to_lowercase().contains("light");
        
        let colors = if is_dark {
            ColorScheme::dark_default()
        } else if is_light {
            ColorScheme::light_default()
        } else {
            ColorScheme::dark_default()
        };
        
        Ok(Theme {
            name: "Generated".into(),
            colors,
            typography: Typography::default(),
            spacing: Spacing::default(),
            borders: Borders::default(),
            shadows: Shadows::default(),
            animations: Animations::default(),
        })
    }
}

/// Theme errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeError {
    ThemeNotFound,
    InvalidColor,
    InvalidImage,
    CannotRemoveDefault,
    AgentError,
}

impl fmt::Display for ThemeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ThemeNotFound => write!(f, "Theme not found"),
            Self::InvalidColor => write!(f, "Invalid color format"),
            Self::InvalidImage => write!(f, "Invalid image data"),
            Self::CannotRemoveDefault => write!(f, "Cannot remove default theme"),
            Self::AgentError => write!(f, "Agent generation error"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_color_hex() {
        let color = Color::rgb(255, 128, 64);
        assert_eq!(color.to_hex(), "#ff8040");
        
        let parsed = Color::from_hex("#ff8040").unwrap();
        assert_eq!(parsed, color);
    }
    
    #[test]
    fn test_color_blend() {
        let black = Color::rgb(0, 0, 0);
        let white = Color::rgb(255, 255, 255);
        let gray = black.blend(&white, 0.5);
        
        assert!(gray.r > 100 && gray.r < 155);
    }
    
    #[test]
    fn test_color_luminance() {
        let black = Color::rgb(0, 0, 0);
        let white = Color::rgb(255, 255, 255);
        
        assert!(black.luminance() < 0.1);
        assert!(white.luminance() > 0.9);
        assert!(!black.is_light());
        assert!(white.is_light());
    }
    
    #[test]
    fn test_theme_engine() {
        let mut engine = ThemeEngine::new();
        
        assert_eq!(engine.list_themes().len(), 1);
        assert!(engine.list_themes().contains(&"SigmaDefault".into()));
        
        let theme = Theme {
            name: "CustomTheme".into(),
            colors: ColorScheme::light_default(),
            typography: Typography::default(),
            spacing: Spacing::default(),
            borders: Borders::default(),
            shadows: Shadows::default(),
            animations: Animations::default(),
        };
        
        engine.add_theme(theme);
        assert_eq!(engine.list_themes().len(), 2);
        
        engine.set_active_theme("CustomTheme").unwrap();
        assert_eq!(engine.get_active_theme().name, "CustomTheme");
    }
}
