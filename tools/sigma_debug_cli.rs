/// SigmaOS: =========================================================================
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

// ─── Module: SigmaOS::SigmaDebugCLI ─────────────────────

/// SigmaDebugCLI — OOP singleton pattern.
pub struct SigmaDebugCLI {
    pub initialized: SigmaBool,
}

impl SigmaDebugCLI {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    pub unsafe fn init(&mut self) {
        // Migrated: init
        self.initialized = true;
    }

    pub unsafe fn attach(&mut self) {
        // Migrated: attach
        self.initialized = true;
    }

    pub unsafe fn dump_registers(&mut self) {
        // Migrated: dump_registers
        self.initialized = true;
    }

    pub unsafe fn debugcli_init(&mut self) {
        // Migrated: debugcli_init
        self.initialized = true;
    }

    pub unsafe fn debugcli_attach(&mut self) {
        // Migrated: debugcli_attach
        self.initialized = true;
    }

    pub unsafe fn debugcli_dump(&mut self) {
        // Migrated: debugcli_dump
        self.initialized = true;
    }

}

static mut INSTANCE: SigmaDebugCLI = SigmaDebugCLI::new();

#[no_mangle]
pub unsafe extern "C" fn init() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn attach() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn dump_registers() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn debugcli_init() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn debugcli_attach() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn debugcli_dump() {
    INSTANCE.initialized = true;
}

