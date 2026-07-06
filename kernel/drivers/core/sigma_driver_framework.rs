/// SigmaOS: sigma_driver_framework.h â€” Sigma Driver Framework (SDF)
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

// â”€â”€â”€ Module: Sigma::sigma_driver_framework â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

/// sigma_device â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigma_device {
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

/// sigma_driver â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigma_driver {
    pub dilithium_sig: [SigmaU8; 4595],
    pub trusted: SigmaBool,
}

/// sigma_irp â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigma_irp {
    pub type: SigmaU64,
    pub offset: SigmaU64,
    pub length: SigmaU64,
    pub status: SigmaI32,
}



