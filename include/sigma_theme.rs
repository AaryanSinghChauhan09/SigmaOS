/// SigmaOS: @file sigma_theme.h
/// Migrated from C/C++ to Rust â€” no_std, no alloc, no external crates.
/// All types hand-defined. OOP via struct + impl + trait patterns.

#![no_std]
#![allow(dead_code)]

// â”€â”€â”€ Kernel Primitive Types â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;
type SigmaUsize = usize;

// â”€â”€â”€ Module: sigma::sigma_theme â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

/// ColorPalette â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ColorPalette {
    pub accent: SigmaU32,
    pub background: SigmaU32,
    pub surface: SigmaU32,
    pub on_background: SigmaU32,
    pub on_surface: SigmaU32,
    pub error: SigmaU32,
    pub success: SigmaU32,
    pub warning: SigmaU32,
}

/// FontConfig â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FontConfig {
    pub family: [u8; 64],
    pub size_body: SigmaU32,
    pub size_heading: SigmaU32,
    pub size_mono: SigmaU32,
    pub antialiased: SigmaBool,
    pub hinting: SigmaBool,
}

/// WindowStyle â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WindowStyle {
    pub corner_radius: SigmaU32,
    pub shadow_blur: SigmaU32,
    pub border_width: SigmaU32,
    pub blur_behind: SigmaBool,
}

/// AnimationConfig â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AnimationConfig {
    pub duration_fast: SigmaU32,
    pub duration_normal: SigmaU32,
    pub duration_slow: SigmaU32,
    pub reduced_motion: SigmaBool,
}

/// SigmaTheme â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SigmaTheme {
    pub name: [u8; 64],
    pub dark_mode: SigmaBool,
    pub colors: SigmaU64,
    pub fonts: SigmaU64,
    pub windows: SigmaU64,
    pub animations: SigmaU64,
}



