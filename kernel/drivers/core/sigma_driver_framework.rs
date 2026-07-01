/// SigmaOS: sigma_driver_framework.h — Sigma Driver Framework (SDF)
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

// ─── Module: Sigma::sigma_driver_framework ─────────────────────

/// sigma_device — hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct {s_name} {{
    pub name: [u8; 64],
    pub class_: SigmaU64,
    pub bus: SigmaU64,
    pub power_state: SigmaU64,
    pub device_id: SigmaU64,
    pub irq: SigmaU32,
    pub mmio_base: SigmaU64,
    pub mmio_size: SigmaU64,
    pub bound: SigmaBool,
}

/// sigma_driver — hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct {s_name} {{
    pub dilithium_sig: [SigmaU8; 4595],
    pub trusted: SigmaBool,
}

/// sigma_irp — hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct {s_name} {{
    pub type: SigmaU64,
    pub offset: SigmaU64,
    pub length: SigmaU64,
    pub status: SigmaI32,
}

