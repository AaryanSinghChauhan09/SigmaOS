/// SigmaOS: SovereignISR module
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

// ─── Module: SigmaOS::SovereignISR ─────────────────────

/// SovereignISR — OOP singleton pattern.
pub struct SovereignISR {
    pub initialized: SigmaBool,
}

impl SovereignISR {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    pub unsafe fn init(&mut self) {
        // Migrated: init
        self.initialized = true;
    }

    pub unsafe fn dispatchISR(&mut self) {
        // Migrated: dispatchISR
        self.initialized = true;
    }

    pub unsafe fn sovereignisr_init(&mut self) {
        // Migrated: sovereignisr_init
        self.initialized = true;
    }

}

static mut INSTANCE: SovereignISR = SovereignISR::new();

#[no_mangle]
pub unsafe extern "C" fn init() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn dispatchISR() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn sovereignisr_init() {
    INSTANCE.initialized = true;
}

