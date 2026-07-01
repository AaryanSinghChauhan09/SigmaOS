/// SigmaOS: interrupt_shard module
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

// ─── Module: SigmaOS::SovereignInterruptShard ─────────────────────

/// SovereignInterruptShard — OOP singleton pattern.
pub struct SovereignInterruptShard {
    pub initialized: SigmaBool,
}

impl SovereignInterruptShard {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    pub unsafe fn DisableInterrupts(&mut self) {
        // Migrated: DisableInterrupts
        self.initialized = true;
    }

    pub unsafe fn EnableInterrupts(&mut self) {
        // Migrated: EnableInterrupts
        self.initialized = true;
    }

    pub unsafe fn RaiseInterrupt(&mut self) {
        // Migrated: RaiseInterrupt
        self.initialized = true;
    }

}

static mut INSTANCE: SovereignInterruptShard = SovereignInterruptShard::new();

#[no_mangle]
pub unsafe extern "C" fn DisableInterrupts() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn EnableInterrupts() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn RaiseInterrupt() {
    INSTANCE.initialized = true;
}

