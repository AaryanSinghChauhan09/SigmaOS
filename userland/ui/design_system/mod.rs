// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS Design System - UI Component Library

mod colors;
mod typography;
mod spacing;
mod components;
mod tokens;

pub use colors::{ColorPalette, ThemeColors};
pub use typography::{TypographySystem, FontScale, FontWeight};
pub use spacing::{SpacingScale};
pub use components::{Button, Input, Card, Modal};
pub use tokens::{DesignTokens};

use serde::{Deserialize, Serialize};

/// Design system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignSystemConfig {
    pub theme: Theme,
    pub font_scale: FontScale,
    pub spacing_scale: SpacingScale,
}

impl Default for DesignSystemConfig {
    fn default() -> Self {
        Self {
            theme: Theme::Dark,
            font_scale: FontScale::default(),
            spacing_scale: SpacingScale::default(),
        }
    }
}

/// Theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    Auto,
}

/// Main Design System structure
pub struct DesignSystem {
    config: DesignSystemConfig,
    tokens: DesignTokens,
}

impl DesignSystem {
    /// Create a new Design System instance
    pub fn new(config: DesignSystemConfig) -> Self {
        let tokens = DesignTokens::new(&config);
        
        Self {
            config,
            tokens,
        }
    }

    /// Get design tokens
    pub fn tokens(&self) -> &DesignTokens {
        &self.tokens
    }

    /// Get theme colors
    pub fn theme_colors(&self) -> ThemeColors {
        self.tokens.colors.theme_colors(&self.config.theme)
    }

    /// Update configuration
    pub fn update_config(&mut self, config: DesignSystemConfig) {
        self.tokens = DesignTokens::new(&config);
        self.config = config;
    }
}
