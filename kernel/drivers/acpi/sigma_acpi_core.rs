/// SigmaOS: @file sigma_acpi_core.cpp
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

// â”€â”€â”€ Module: sigma::sigma_acpi_core â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

/// RSDPDescriptor â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RSDPDescriptor {
    pub Signature: [u8; 8],
    pub Checksum: SigmaU8,
    pub OEMID: [u8; 6],
    pub Revision: SigmaU8,
    pub RsdtAddress: SigmaU32,
}

/// ACPISDTHeader â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACPISDTHeader {
    pub Signature: [u8; 4],
    pub Length: SigmaU32,
    pub Revision: SigmaU8,
    pub Checksum: SigmaU8,
    pub OEMID: [u8; 6],
    pub OEMTableID: [u8; 8],
    pub OEMRevision: SigmaU32,
    pub CreatorID: SigmaU32,
    pub CreatorRevision: SigmaU32,
}



