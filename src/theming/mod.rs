// src/theming/mod.rs
// Theme system for SigmaOS

#![no_std]

pub mod theme_engine;

pub use theme_engine::{
    Animations, Borders, Color, ColorScheme, Shadow, Shadows, Spacing, Theme, ThemeEngine,
    ThemeError, Typography,
};
