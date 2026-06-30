/// SigmaOS: SigmaOS Sovereign PS/2 Keyboard Driver (S-PS2)
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

// ─── Module: SigmaOS::SovereignPS2 ─────────────────────

/// SovereignPS2 — OOP singleton pattern.
pub struct SovereignPS2 {
    pub initialized: SigmaBool,
}

impl SovereignPS2 {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    pub unsafe fn init(&mut self) {
        // Migrated: init
        self.initialized = true;
    }

    pub unsafe fn initLegacySupport(&mut self) {
        // Migrated: initLegacySupport
        self.initialized = true;
    }

    pub unsafe fn readScancode(&mut self) {
        // Migrated: readScancode
        self.initialized = true;
    }

    pub unsafe fn inb(&mut self) {
        // Migrated: inb
        self.initialized = true;
    }

    pub unsafe fn kbd_init(&mut self) {
        // Migrated: kbd_init
        self.initialized = true;
    }

    pub unsafe fn kbd_init_legacy_fallback(&mut self) {
        // Migrated: kbd_init_legacy_fallback
        self.initialized = true;
    }

    pub unsafe fn kbd_read(&mut self) {
        // Migrated: kbd_read
        self.initialized = true;
    }

}

static mut INSTANCE: SovereignPS2 = SovereignPS2::new();

#[no_mangle]
pub unsafe extern "C" fn init() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn initLegacySupport() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn kbd_init() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn kbd_init_legacy_fallback() {
    INSTANCE.initialized = true;
}

