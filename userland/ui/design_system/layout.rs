// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS Design System - UI Layouts

use serde::{Deserialize, Serialize};

/// Flexbox-style layout engine primitive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flex {
    pub direction: FlexDirection,
    pub justify: FlexJustify,
    pub align: FlexAlign,
    pub wrap: bool,
    pub gap: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlexDirection {
    Row,
    Column,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlexJustify {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlexAlign {
    Start,
    Center,
    End,
    Stretch,
}

impl Default for Flex {
    fn default() -> Self {
        Self {
            direction: FlexDirection::Row,
            justify: FlexJustify::Start,
            align: FlexAlign::Start,
            wrap: false,
            gap: 0,
        }
    }
}

/// Grid layout engine primitive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grid {
    pub columns: u32,
    pub rows: u32,
    pub gap: u32,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            columns: 1,
            rows: 1,
            gap: 0,
        }
    }
}

/// Native Tiling Node for Window Management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TilingNode {
    Vertical(Vec<TilingNode>),
    Horizontal(Vec<TilingNode>),
    Window(u64), // Window ID
}
