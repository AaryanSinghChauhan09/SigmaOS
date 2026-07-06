/// SigmaOS: =============================================================================
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

// â”€â”€â”€ Module: Sigma::hal_contract â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

/// SigmaDisplayOps â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SigmaDisplayOps {
}

/// SigmaInputOps â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SigmaInputOps {
}

/// SigmaStorageOps â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SigmaStorageOps {
}

/// SigmaNetOps â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SigmaNetOps {
}

/// SigmaTimerOps â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SigmaTimerOps {
}

/// SigmaSerialOps â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SigmaSerialOps {
}



