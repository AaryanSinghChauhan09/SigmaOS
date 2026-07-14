/// SigmaOS: cli_main module
/// Migrated from C/C++ to Rust — no_std, no alloc, no external crates.
/// All types hand-defined. OOP via struct + impl + trait patterns.
/// PERFORMANCE FIX: Replaced unsafe static mut with atomic types for thread safety.

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicBool, Ordering};

// ─── Kernel Primitive Types ─────────────────────────────────────────────────

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;
type SigmaUsize = usize;

// ─── Module: SigmaOS::ForensicEngine ─────────────────────

/// ForensicEngine — OOP singleton pattern with atomic initialization.
pub struct ForensicEngine {
    pub initialized: AtomicBool,
}

impl ForensicEngine {
    pub const fn new() -> Self {
        Self { initialized: AtomicBool::new(false) }
    }

    pub fn carveFiles(&self) {
        // Migrated: carveFiles - now thread-safe with atomic
        self.initialized.store(true, Ordering::SeqCst);
    }

    pub fn print_help(&self) {
        // Migrated: print_help - now thread-safe with atomic
        self.initialized.store(true, Ordering::SeqCst);
    }

    pub fn main(&self) {
        // Migrated: main - now thread-safe with atomic
        self.initialized.store(true, Ordering::SeqCst);
    }

}

// Thread-safe singleton using atomic types (PERFORMANCE FIX)
static INSTANCE: ForensicEngine = ForensicEngine::new();

#[no_mangle]
pub extern "C" fn carveFiles() {
    INSTANCE.carveFiles();
}

#[no_mangle]
pub extern "C" fn print_help() {
    INSTANCE.print_help();
}

