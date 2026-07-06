/// SigmaOS: =========================================================================
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

// â”€â”€â”€ Module: SigmaOS::SovereignHypervisor â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

/// VirtualMachineConfig â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VirtualMachineConfig {
    pub name: [u8; 64],
    pub memory_mb: SigmaU64,
    pub vcpu_count: SigmaU32,
    pub pqc_isolation: SigmaBool,
}

/// GuestVMState â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GuestVMState {
    pub id: SigmaU32,
    pub config: SigmaU64,
    pub is_running: SigmaBool,
    pub cr3_guest: SigmaU64,
}

/// SovereignHypervisor â€” OOP singleton pattern.
pub struct SovereignHypervisor {
    pub initialized: SigmaBool,
}

impl SovereignHypervisor {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

}

static mut INSTANCE: SovereignHypervisor = SovereignHypervisor::new();



