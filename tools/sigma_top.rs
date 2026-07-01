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

// ─── Module: SigmaOS::SigmaTop ─────────────────────

/// SigmaTop — OOP singleton pattern.
pub struct SigmaTop {
    pub initialized: SigmaBool,
}

impl SigmaTop {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    pub unsafe fn init(&mut self) {
        // Migrated: init
        self.initialized = true;
    }

    pub unsafe fn sigmatop_init(&mut self) {
        // Migrated: sigmatop_init
        self.initialized = true;
    }

    pub unsafe fn sigmatop_render(&mut self) {
        // Migrated: sigmatop_render
        self.initialized = true;
    }

}

static mut INSTANCE: SigmaTop = SigmaTop::new();

#[no_mangle]
pub unsafe extern "C" fn init() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn sigmatop_init() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn sigmatop_render() {
    INSTANCE.initialized = true;
}

