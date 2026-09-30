// src/compositor/mod.rs
// SigmaCompositor - Wayland compositor module
// Memory-safe replacement for Hyprland (C++)

#![no_std]

pub mod sigma_compositor;

pub use sigma_compositor::{
    CompositorError, Geometry, LayoutType, SigmaCompositor, Window, WindowId, WindowState,
    Workspace,
};

// ─── Zenith Wayland Compositor Core ──────────────────────────────────────────
pub mod zenith_core;
