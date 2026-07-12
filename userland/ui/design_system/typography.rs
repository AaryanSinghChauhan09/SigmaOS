// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS Design System - Typography System

use serde::{Deserialize, Serialize};

/// Typography system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypographySystem {
    pub font_families: FontFamilies,
    pub font_scale: FontScale,
}

impl Default for TypographySystem {
    fn default() -> Self {
        Self {
            font_families: FontFamilies::default(),
            font_scale: FontScale::default(),
        }
    }
}

/// Font families
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontFamilies {
    pub sans: String,
    pub mono: String,
    pub display: String,
}

impl Default for FontFamilies {
    fn default() -> Self {
        Self {
            sans: "Inter, system-ui, sans-serif".to_string(),
            mono: "JetBrains Mono, monospace".to_string(),
            display: "Inter, system-ui, sans-serif".to_string(),
        }
    }
}

/// Font scale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontScale {
    pub xs: FontSize,
    pub sm: FontSize,
    pub base: FontSize,
    pub lg: FontSize,
    pub xl: FontSize,
    pub xl2: FontSize,
    pub xl3: FontSize,
    pub xl4: FontSize,
    pub xl5: FontSize,
    pub xl6: FontSize,
}

impl Default for FontScale {
    fn default() -> Self {
        Self {
            xs: FontSize {
                size: 12,
                line_height: 1.5,
                letter_spacing: 0.0,
            },
            sm: FontSize {
                size: 14,
                line_height: 1.5,
                letter_spacing: 0.0,
            },
            base: FontSize {
                size: 16,
                line_height: 1.5,
                letter_spacing: 0.0,
            },
            lg: FontSize {
                size: 18,
                line_height: 1.5,
                letter_spacing: 0.0,
            },
            xl: FontSize {
                size: 20,
                line_height: 1.5,
                letter_spacing: 0.0,
            },
            xl2: FontSize {
                size: 24,
                line_height: 1.5,
                letter_spacing: 0.0,
            },
            xl3: FontSize {
                size: 30,
                line_height: 1.5,
                letter_spacing: 0.0,
            },
            xl4: FontSize {
                size: 36,
                line_height: 1.5,
                letter_spacing: 0.0,
            },
            xl5: FontSize {
                size: 48,
                line_height: 1.5,
                letter_spacing: 0.0,
            },
            xl6: FontSize {
                size: 60,
                line_height: 1.5,
                letter_spacing: 0.0,
            },
        }
    }
}

/// Font size
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontSize {
    pub size: u32,
    pub line_height: f32,
    pub letter_spacing: f32,
}

/// Font weight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FontWeight {
    Light = 300,
    Regular = 400,
    Medium = 500,
    Semibold = 600,
    Bold = 700,
}
