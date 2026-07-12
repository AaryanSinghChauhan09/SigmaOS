// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS Design System - Color System

use super::Theme;
use serde::{Deserialize, Serialize};

/// Color palette
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    pub primary: PrimaryColors,
    pub semantic: SemanticColors,
    pub neutral: NeutralColors,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            primary: PrimaryColors::default(),
            semantic: SemanticColors::default(),
            neutral: NeutralColors::default(),
        }
    }
}

/// Primary colors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimaryColors {
    pub primary: String,
    pub primary_dark: String,
    pub primary_light: String,
}

impl Default for PrimaryColors {
    fn default() -> Self {
        Self {
            primary: "#3B82F6".to_string(),
            primary_dark: "#2563EB".to_string(),
            primary_light: "#60A5FA".to_string(),
        }
    }
}

/// Semantic colors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticColors {
    pub success: String,
    pub warning: String,
    pub error: String,
    pub info: String,
}

impl Default for SemanticColors {
    fn default() -> Self {
        Self {
            success: "#10B981".to_string(),
            warning: "#F59E0B".to_string(),
            error: "#EF4444".to_string(),
            info: "#3B82F6".to_string(),
        }
    }
}

/// Neutral colors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeutralColors {
    pub gray_50: String,
    pub gray_100: String,
    pub gray_200: String,
    pub gray_300: String,
    pub gray_400: String,
    pub gray_500: String,
    pub gray_600: String,
    pub gray_700: String,
    pub gray_800: String,
    pub gray_900: String,
}

impl Default for NeutralColors {
    fn default() -> Self {
        Self {
            gray_50: "#F9FAFB".to_string(),
            gray_100: "#F3F4F6".to_string(),
            gray_200: "#E5E7EB".to_string(),
            gray_300: "#D1D5DB".to_string(),
            gray_400: "#9CA3AF".to_string(),
            gray_500: "#6B7280".to_string(),
            gray_600: "#4B5563".to_string(),
            gray_700: "#374151".to_string(),
            gray_800: "#1F2937".to_string(),
            gray_900: "#111827".to_string(),
        }
    }
}

/// Theme colors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    pub background: String,
    pub surface: String,
    pub text_primary: String,
    pub text_secondary: String,
    pub border: String,
}

impl ColorPalette {
    /// Get theme colors based on theme
    pub fn theme_colors(&self, theme: &Theme) -> ThemeColors {
        match theme {
            Theme::Light => ThemeColors {
                background: self.neutral.gray_50.clone(),
                surface: "#FFFFFF".to_string(),
                text_primary: self.neutral.gray_900.clone(),
                text_secondary: self.neutral.gray_600.clone(),
                border: self.neutral.gray_200.clone(),
            },
            Theme::Dark => ThemeColors {
                background: self.neutral.gray_900.clone(),
                surface: self.neutral.gray_800.clone(),
                text_primary: self.neutral.gray_50.clone(),
                text_secondary: self.neutral.gray_400.clone(),
                border: self.neutral.gray_700.clone(),
            },
            Theme::Auto => {
                // Auto would detect system preference
                // For now, default to dark
                self.theme_colors(&Theme::Dark)
            }
        }
    }
}
