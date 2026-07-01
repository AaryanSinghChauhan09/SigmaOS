/// SigmaOS: @file sigma_dual_boot.cpp
/// Migrated from C/C++ to Rust — no_std, no alloc, no external crates.
/// All types hand-defined. OOP via struct + impl + trait patterns.

#![no_std]
#![allow(dead_code)]

// ─── Kernel Primitive Types ─────────────────────────────────────────────────

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;
type SigmaUsize = usize;

// ─── Module: sigma::sigma_dual_boot ─────────────────────

/// BootEntry — hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct {s_name} {{
    pub index: SigmaU32,
    pub os_name: [u8; 32],
    pub kernel_path: [u8; 128],
    pub boot_args: [u8; 256],
    pub is_default: SigmaBool,
}

