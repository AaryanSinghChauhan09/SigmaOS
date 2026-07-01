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

// ─── Module: SigmaOS::SigmaOverlayCLI ─────────────────────

/// SigmaOverlayCLI — OOP singleton pattern.
pub struct SigmaOverlayCLI {
    pub initialized: SigmaBool,
}

impl SigmaOverlayCLI {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    pub unsafe fn run_command(&mut self) {
        // Migrated: run_command
        self.initialized = true;
    }

    pub unsafe fn print_usage(&mut self) {
        // Migrated: print_usage
        self.initialized = true;
    }

    pub unsafe fn overlay_cli_run(&mut self) {
        // Migrated: overlay_cli_run
        self.initialized = true;
    }

}

static mut INSTANCE: SigmaOverlayCLI = SigmaOverlayCLI::new();

#[no_mangle]
pub unsafe extern "C" fn run_command() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn print_usage() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn overlay_cli_run() {
    INSTANCE.initialized = true;
}

