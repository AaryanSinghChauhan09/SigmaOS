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

// ─── Module: SigmaOS::for ─────────────────────

/// for — OOP singleton pattern.
pub struct for {
    pub initialized: SigmaBool,
}

impl for {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    pub unsafe fn on_shard_fault(&mut self) {
        // Migrated: on_shard_fault
        self.initialized = true;
    }

}

static mut INSTANCE: for = for::new();

#[no_mangle]
pub unsafe extern "C" fn on_shard_fault() {
    INSTANCE.initialized = true;
}

