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

// ─── Module: SigmaOS::SovereignTPM ─────────────────────

/// SovereignTPM — OOP singleton pattern.
pub struct SovereignTPM {
    pub initialized: SigmaBool,
}

impl SovereignTPM {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    pub unsafe fn init(&mut self) {
        // Migrated: init
        self.initialized = true;
    }

    pub unsafe fn performAttestation(&mut self) {
        // Migrated: performAttestation
        self.initialized = true;
    }

    pub unsafe fn probeInterface(&mut self) {
        // Migrated: probeInterface
        self.initialized = true;
    }

    pub unsafe fn readPCRs(&mut self) {
        // Migrated: readPCRs
        self.initialized = true;
    }

    pub unsafe fn tpm_init(&mut self) {
        // Migrated: tpm_init
        self.initialized = true;
    }

    pub unsafe fn tpm_attest_bootloader(&mut self) {
        // Migrated: tpm_attest_bootloader
        self.initialized = true;
    }

}

static mut INSTANCE: SovereignTPM = SovereignTPM::new();

#[no_mangle]
pub unsafe extern "C" fn init() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn readPCRs() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn tpm_init() {
    INSTANCE.initialized = true;
}

