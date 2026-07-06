/// SigmaOS: @file sigma_spins.h
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

// â”€â”€â”€ Module: sigma::SpinId â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

/// SpinManifest â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SpinManifest {
    pub id: SigmaU64,
    pub net_policy: SigmaU64,
    pub sandbox_ring_level: SigmaU64,
    pub read_only_root: SigmaBool,
    pub gaming_optimized: SigmaBool,
    pub audio_low_latency: SigmaBool,
}

/// SpinId â€” OOP singleton pattern.
pub struct SpinId {
    pub initialized: SigmaBool,
}

impl SpinId {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

}

static mut INSTANCE: SpinId = SpinId::new();



