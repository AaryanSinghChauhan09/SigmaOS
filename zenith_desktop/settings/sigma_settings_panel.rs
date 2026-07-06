/// SigmaOS: @file sigma_settings_panel.cpp
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

// â”€â”€â”€ Module: sigma::sigma_settings_panel â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

/// SettingsPlug â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SettingsPlug {
    pub category: SigmaU64,
    pub name: [u8; 64],
    pub icon: [u8; 128],
    pub description: [u8; 256],
    pub visible: SigmaBool,
}

/// SettingEntry â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SettingEntry {
    pub key: [u8; 128],
    pub label: [u8; 128],
    pub description: [u8; 256],
    pub type: SigmaU64,
    pub value_int: SigmaU32,
    pub value_str: [u8; 256],
    pub min_int: SigmaU32,
    pub max_int: SigmaU32,
    pub category: SigmaU64,
}

/// SettingsRegistry â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SettingsRegistry {
    pub count: SigmaU32,
    pub plug_count: SigmaU32,
}



