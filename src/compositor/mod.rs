// src/compositor/mod.rs
// SigmaCompositor - Wayland compositor module
// Memory-safe replacement for Hyprland (C++)

#![no_std]

pub mod sigma_compositor;

pub use sigma_compositor::{
    SigmaCompositor,
    WindowId,
    Window,
    Workspace,
    Geometry,
    LayoutType,
    WindowState,
    CompositorError,
};
