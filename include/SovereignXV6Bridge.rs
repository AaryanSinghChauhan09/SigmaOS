/// SigmaOS: SovereignXV6Bridge module
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

// ─── Module: SigmaOS::SovereignTrapHandler ─────────────────────

/// SovereignTrapHandler — OOP singleton pattern.
pub struct SovereignTrapHandler {
    pub initialized: SigmaBool,
}

impl SovereignTrapHandler {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

}

static mut INSTANCE: SovereignTrapHandler = SovereignTrapHandler::new();

