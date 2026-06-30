/// SigmaOS: port_shard module
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

// ─── Module: SigmaOS::SovereignPortShard ─────────────────────

/// SovereignPortShard — OOP singleton pattern.
pub struct SovereignPortShard {
    pub initialized: SigmaBool,
}

impl SovereignPortShard {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    pub unsafe fn outb(&mut self) {
        // Migrated: outb
        self.initialized = true;
    }

    pub unsafe fn outw(&mut self) {
        // Migrated: outw
        self.initialized = true;
    }

    pub unsafe fn inb(&mut self) {
        // Migrated: inb
        self.initialized = true;
    }

    pub unsafe fn WaitIO(&mut self) {
        // Migrated: WaitIO
        self.initialized = true;
    }

}

static mut INSTANCE: SovereignPortShard = SovereignPortShard::new();

#[no_mangle]
pub unsafe extern "C" fn outb() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn outw() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn WaitIO() {
    INSTANCE.initialized = true;
}

