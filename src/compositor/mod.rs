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
pub mod wayland_surface_engine;
pub mod zenith_core;

pub use wayland_surface_engine::*;
