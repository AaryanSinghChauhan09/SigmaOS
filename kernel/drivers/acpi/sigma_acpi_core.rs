/// SigmaOS: @file sigma_acpi_core.cpp
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

// ─── Module: sigma::sigma_acpi_core ─────────────────────

/// RSDPDescriptor — hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct {s_name} {{
    pub Signature: [u8; 8],
    pub Checksum: SigmaU8,
    pub OEMID: [u8; 6],
    pub Revision: SigmaU8,
    pub RsdtAddress: SigmaU32,
}

/// ACPISDTHeader — hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct {s_name} {{
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

