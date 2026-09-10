// src/theming/mod.rs
// Theme system for SigmaOS

#![no_std]

pub mod theme_engine;

pub use theme_engine::{
    Color,
    Theme,
    ColorScheme,
    Typography,
    Spacing,
    Borders,
    Shadows,
    Shadow,
    Animations,
    ThemeEngine,
    ThemeError,
};
