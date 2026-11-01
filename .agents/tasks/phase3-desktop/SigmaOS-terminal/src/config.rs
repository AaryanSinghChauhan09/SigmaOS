//! Terminal configuration for SigmaOS-terminal.

use crate::{Color, Result, TermError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    pub name: String,
    pub foreground: [u8; 3],
    pub background: [u8; 3],
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            name: "sigma-dark".to_string(),
            foreground: [0xd8, 0xd8, 0xd8],
            background: [0x18, 0x18, 0x18],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TermConfig {
    pub font_family: String,
    pub font_size: f32,
    pub color_scheme: ColorScheme,
    pub scrollback_lines: usize,
    pub enable_sixel: bool,
    pub enable_kitty_graphics: bool,
    pub shell: String,
}

impl Default for TermConfig {
    fn default() -> Self {
        Self {
            font_family: "monospace".to_string(),
            font_size: 13.0,
            color_scheme: ColorScheme::default(),
            scrollback_lines: 10_000,
            enable_sixel: true,
            enable_kitty_graphics: true,
            shell: "/bin/bash".to_string(),
        }
    }
}

impl TermConfig {
    pub fn load_or_default() -> Result<Self> {
        Ok(Self::default())
    }

    pub fn foreground_color(&self) -> Color {
        let [r, g, b] = self.color_scheme.foreground;
        Color { r, g, b }
    }

    pub fn background_color(&self) -> Color {
        let [r, g, b] = self.color_scheme.background;
        Color { r, g, b }
    }
}
