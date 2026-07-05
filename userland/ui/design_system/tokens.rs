// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS Design System - Design Tokens

use super::{colors::ColorPalette, typography::TypographySystem, spacing::SpacingScale, DesignSystemConfig};

/// Design tokens
#[derive(Debug, Clone)]
pub struct DesignTokens {
    pub colors: ColorPalette,
    pub typography: TypographySystem,
    pub spacing: SpacingScale,
}

impl DesignTokens {
    pub fn new(config: &DesignSystemConfig) -> Self {
        Self {
            colors: ColorPalette::default(),
            typography: config.font_scale.clone(),
            spacing: config.spacing_scale.clone(),
        }
    }

    /// Get a color token
    pub fn color(&self, name: &str) -> String {
        match name {
            "primary" => self.colors.primary.primary.clone(),
            "primary-dark" => self.colors.primary.primary_dark.clone(),
            "success" => self.colors.semantic.success.clone(),
            "error" => self.colors.semantic.error.clone(),
            "background" => self.colors.neutral.gray_50.clone(),
            _ => "#000000".to_string(),
        }
    }

    /// Get a spacing token
    pub fn spacing(&self, name: &str) -> u32 {
        match name {
            "xs" => self.spacing.xs,
            "sm" => self.spacing.sm,
            "md" => self.spacing.md,
            "lg" => self.spacing.lg,
            "xl" => self.spacing.xl,
            _ => self.spacing.md,
        }
    }

    /// Get a typography token
    pub fn font_size(&self, name: &str) -> u32 {
        match name {
            "xs" => self.typography.xs.size,
            "sm" => self.typography.sm.size,
            "base" => self.typography.base.size,
            "lg" => self.typography.lg.size,
            "xl" => self.typography.xl.size,
            _ => self.typography.base.size,
        }
    }
}
