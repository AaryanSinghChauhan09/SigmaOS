// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS Design System - Spacing System

use serde::{Deserialize, Serialize};

/// Spacing scale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacingScale {
    pub px: u32,
    pub xxs: u32,
    pub xs: u32,
    pub sm: u32,
    pub md: u32,
    pub lg: u32,
    pub xl: u32,
    pub xl2: u32,
    pub xl3: u32,
    pub xl4: u32,
    pub xl5: u32,
}

impl Default for SpacingScale {
    fn default() -> Self {
        Self {
            px: 0,
            xxs: 4,
            xs: 8,
            sm: 12,
            md: 16,
            lg: 24,
            xl: 32,
            xl2: 48,
            xl3: 64,
            xl4: 96,
            xl5: 128,
        }
    }
}
