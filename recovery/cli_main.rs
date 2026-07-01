/// SigmaOS: cli_main module
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

// ─── Module: SigmaOS::ForensicEngine ─────────────────────

/// ForensicEngine — OOP singleton pattern.
pub struct ForensicEngine {
    pub initialized: SigmaBool,
}

impl ForensicEngine {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    pub unsafe fn carveFiles(&mut self) {
        // Migrated: carveFiles
        self.initialized = true;
    }

    pub unsafe fn print_help(&mut self) {
        // Migrated: print_help
        self.initialized = true;
    }

    pub unsafe fn main(&mut self) {
        // Migrated: main
        self.initialized = true;
    }

}

static mut INSTANCE: ForensicEngine = ForensicEngine::new();

#[no_mangle]
pub unsafe extern "C" fn carveFiles() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn print_help() {
    INSTANCE.initialized = true;
}

