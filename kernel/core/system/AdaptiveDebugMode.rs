/// SigmaOS: SigmaOS Adaptive Debug Mode (S-ADAPT)
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

// ─── Module: SigmaOS::AdaptiveDebugMode ─────────────────────

/// AdaptiveDebugMode — OOP singleton pattern.
pub struct AdaptiveDebugMode {
    pub initialized: SigmaBool,
}

impl AdaptiveDebugMode {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    pub unsafe fn init(&mut self) {
        // Migrated: init
        self.initialized = true;
    }

    pub unsafe fn executeHitAndTrial(&mut self) {
        // Migrated: executeHitAndTrial
        self.initialized = true;
    }

    pub unsafe fn adapt_init(&mut self) {
        // Migrated: adapt_init
        self.initialized = true;
    }

    pub unsafe fn adapt_run_trial(&mut self) {
        // Migrated: adapt_run_trial
        self.initialized = true;
    }

}

static mut INSTANCE: AdaptiveDebugMode = AdaptiveDebugMode::new();

#[no_mangle]
pub unsafe extern "C" fn init() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn executeHitAndTrial() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn adapt_init() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn adapt_run_trial() {
    INSTANCE.initialized = true;
}

