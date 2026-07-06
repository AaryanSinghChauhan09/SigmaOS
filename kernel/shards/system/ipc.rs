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

// â”€â”€â”€ Module: Sigma::ipc â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

/// SigmaPipe â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SigmaPipe {
    pub head: SigmaU32,
    pub tail: SigmaU32,
    pub count: SigmaU32,
    pub valid: SigmaBool,
    pub write_closed: SigmaBool,
    pub read_closed: SigmaBool,
}

/// SigmaMsg â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SigmaMsg {
    pub mtype: SigmaU32,
    pub len: SigmaU32,
}

/// SigmaMQ â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SigmaMQ {
    pub head: SigmaU32,
    pub count: SigmaU32,
    pub valid: SigmaBool,
    pub name: [u8; 32],
}

/// SigmaSHM â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SigmaSHM {
    pub paddr: SigmaU64,
    pub size: SigmaU64,
    pub key: SigmaU32,
    pub refs: SigmaU32,
    pub valid: SigmaBool,
}

/// SigmaFutex â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SigmaFutex {
    pub waiter_tid: SigmaU64,
    pub valid: SigmaBool,
}

#[no_mangle]
pub unsafe extern "C" fn ipc_init() {
}



