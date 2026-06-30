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

// ─── Module: SigmaOS::SovereignDXVK ─────────────────────

/// SovereignDXVK — OOP singleton pattern.
pub struct SovereignDXVK {
    pub initialized: SigmaBool,
}

impl SovereignDXVK {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    pub unsafe fn initializeDXVK(&mut self) {
        // Migrated: initializeDXVK
        self.initialized = true;
    }

    pub unsafe fn proton_dxvk_init(&mut self) {
        // Migrated: proton_dxvk_init
        self.initialized = true;
    }

}

static mut INSTANCE: SovereignDXVK = SovereignDXVK::new();

#[no_mangle]
pub unsafe extern "C" fn initializeDXVK() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn proton_dxvk_init() {
    INSTANCE.initialized = true;
}

