//! SigmaOS Theme Engine — theme rendering and color synthesis.
//!
//! Upstream inspiration:
//! - Cinnamon themes: <https://github.com/linuxmint/cinnamon>

pub mod css;
pub mod icons;
pub mod palette;

use std::{collections::HashMap, fmt};

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub enum ThemeError {
    ThemeNotFound(String),
    InvalidToken(String),
    ParseError(String),
}

impl fmt::Display for ThemeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThemeError::ThemeNotFound(n) => write!(f, "theme not found: {n}"),
            ThemeError::InvalidToken(t) => write!(f, "invalid token: {t}"),
            ThemeError::ParseError(msg) => write!(f, "parse error: {msg}"),
        }
    }
}

impl std::error::Error for ThemeError {}

// ---------------------------------------------------------------------------
// Color
// ---------------------------------------------------------------------------

/// An sRGB colour with full alpha.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    /// Construct an opaque colour.
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Construct a colour with alpha.
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Blend this colour 50/50 with `other` (simple average).
    pub fn blend(&self, other: &Color) -> Color {
        Color {
            r: ((self.r as u16 + other.r as u16) / 2) as u8,
            g: ((self.g as u16 + other.g as u16) / 2) as u8,
            b: ((self.b as u16 + other.b as u16) / 2) as u8,
            a: ((self.a as u16 + other.a as u16) / 2) as u8,
        }
    }
}

// ---------------------------------------------------------------------------
// Theme token
// ---------------------------------------------------------------------------

/// A semantic design token — maps a role name to a colour.
#[derive(Debug, Clone)]
pub struct ThemeToken {
    pub name: String,
    pub value: Color,
}

impl ThemeToken {
    pub fn new(name: impl Into<String>, value: Color) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }
}

// ---------------------------------------------------------------------------
// Color palette
// ---------------------------------------------------------------------------

/// A named collection of [`ThemeToken`]s.
#[derive(Debug, Default)]
pub struct ColorPalette {
    tokens: HashMap<String, Color>,
}

impl ColorPalette {
    /// Insert or overwrite a token.
    pub fn set(&mut self, name: impl Into<String>, color: Color) {
        self.tokens.insert(name.into(), color);
    }

    /// Look up a token by name.
    pub fn get(&self, name: &str) -> Option<Color> {
        self.tokens.get(name).copied()
    }

    /// Returns the number of tokens.
    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Theme renderer
// ---------------------------------------------------------------------------

/// Renders theme tokens to CSS-like output.
#[derive(Debug, Default)]
pub struct ThemeRenderer;

impl ThemeRenderer {
    /// Render a palette to a `HashMap<name, css_hex_colour>`.
    pub fn render_css(&self, palette: &ColorPalette) -> HashMap<String, String> {
        palette
            .tokens
            .iter()
            .map(|(name, color)| {
                let hex = format!(
                    "#{:02X}{:02X}{:02X}{:02X}",
                    color.r, color.g, color.b, color.a
                );
                (name.clone(), hex)
            })
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Theme engine
// ---------------------------------------------------------------------------

/// Top-level theme engine.
#[derive(Debug)]
pub struct ThemeEngine {
    pub name: String,
    pub palette: ColorPalette,
    pub renderer: ThemeRenderer,
}

impl ThemeEngine {
    fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            palette: ColorPalette::default(),
            renderer: ThemeRenderer::default(),
        }
    }

    fn seed_defaults(&mut self) {
        self.palette.set("background", Color::rgb(30, 30, 30));
        self.palette.set("foreground", Color::rgb(220, 220, 220));
        self.palette.set("accent", Color::rgb(100, 149, 237));
        self.palette.set("border", Color::rgb(60, 60, 60));
    }
}

// ---------------------------------------------------------------------------
// Public entry-point
// ---------------------------------------------------------------------------

/// Load a theme by `name`.
pub fn load_theme(name: &str) -> Result<ThemeEngine, ThemeError> {
    match name {
        "default" | "dark" => {
            let mut engine = ThemeEngine::new(name);
            engine.seed_defaults();
            Ok(engine)
        }
        "light" => {
            let mut engine = ThemeEngine::new("light");
            engine.palette.set("background", Color::rgb(255, 255, 255));
            engine.palette.set("foreground", Color::rgb(30, 30, 30));
            engine.palette.set("accent", Color::rgb(0, 120, 215));
            engine.palette.set("border", Color::rgb(200, 200, 200));
            Ok(engine)
        }
        other => Err(ThemeError::ThemeNotFound(other.to_owned())),
    }
}

// ---------------------------------------------------------------------------
// Inline tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_default_theme_succeeds() {
        let engine = load_theme("default").expect("default theme must load");
        assert!(!engine.palette.is_empty());
    }

    #[test]
    fn unknown_theme_returns_error() {
        let result = load_theme("nonexistent-theme-xyz");
        assert!(result.is_err());
    }

    #[test]
    fn color_blend_midpoint() {
        let black = Color::rgb(0, 0, 0);
        let white = Color::rgb(255, 255, 255);
        let mid = black.blend(&white);
        assert_eq!(mid.r, 127);
    }
}
