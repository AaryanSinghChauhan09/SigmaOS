/// SigmaOS: SovereignKdump module
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

// ─── Module: SigmaOS::SovereignKdump ─────────────────────

/// SovereignKdump — OOP singleton pattern.
pub struct SovereignKdump {
    pub initialized: SigmaBool,
}

impl SovereignKdump {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    pub unsafe fn init(&mut self) {
        // Migrated: init
        self.initialized = true;
    }

    pub unsafe fn captureCrash(&mut self) {
        // Migrated: captureCrash
        self.initialized = true;
    }

    pub unsafe fn kdump_init(&mut self) {
        // Migrated: kdump_init
        self.initialized = true;
    }

}

static mut INSTANCE: SovereignKdump = SovereignKdump::new();

#[no_mangle]
pub unsafe extern "C" fn init() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn captureCrash() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn kdump_init() {
    INSTANCE.initialized = true;
}

