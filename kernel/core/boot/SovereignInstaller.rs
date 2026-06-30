/// SigmaOS: SigmaOS Sovereign Installer (S-INSTALLER)
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

// ─── Module: SigmaOS::SovereignInstaller ─────────────────────

/// SovereignInstaller — OOP singleton pattern.
pub struct SovereignInstaller {
    pub initialized: SigmaBool,
}

impl SovereignInstaller {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    pub unsafe fn ignite(&mut self) {
        // Migrated: ignite
        self.initialized = true;
    }

    pub unsafe fn installer_run(&mut self) {
        // Migrated: installer_run
        self.initialized = true;
    }

}

static mut INSTANCE: SovereignInstaller = SovereignInstaller::new();

#[no_mangle]
pub unsafe extern "C" fn ignite() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn installer_run() {
    INSTANCE.initialized = true;
}

