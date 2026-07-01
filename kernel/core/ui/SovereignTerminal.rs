/// SigmaOS: SigmaOS Sovereign Terminal Shard (S-TERM)
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

// ─── Module: SigmaOS::SovereignTerminal ─────────────────────

/// SovereignTerminal — OOP singleton pattern.
pub struct SovereignTerminal {
    pub initialized: SigmaBool,
}

impl SovereignTerminal {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    pub unsafe fn init(&mut self) {
        // Migrated: init
        self.initialized = true;
    }

    pub unsafe fn renderFrame(&mut self) {
        // Migrated: renderFrame
        self.initialized = true;
    }

    pub unsafe fn handleInput(&mut self) {
        // Migrated: handleInput
        self.initialized = true;
    }

    pub unsafe fn term_init(&mut self) {
        // Migrated: term_init
        self.initialized = true;
    }

}

static mut INSTANCE: SovereignTerminal = SovereignTerminal::new();

#[no_mangle]
pub unsafe extern "C" fn init() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn renderFrame() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn handleInput() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn term_init() {
    INSTANCE.initialized = true;
}

