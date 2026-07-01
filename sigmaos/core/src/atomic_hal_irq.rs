/// SigmaOS: atomic_hal_irq module
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

// ─── Module: sigma::IInterruptHandler ─────────────────────

/// IInterruptHandler — OOP singleton pattern.
pub struct IInterruptHandler {
    pub initialized: SigmaBool,
}

impl IInterruptHandler {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    pub unsafe fn dispatch(&mut self) {
        // Migrated: dispatch
        self.initialized = true;
    }

}

static mut INSTANCE: IInterruptHandler = IInterruptHandler::new();

#[no_mangle]
pub unsafe extern "C" fn dispatch() {
    INSTANCE.initialized = true;
}

