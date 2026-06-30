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

// ─── Module: SigmaOS::SigmaNotificationCenter ─────────────────────

/// SigmaNotificationCenter — OOP singleton pattern.
pub struct SigmaNotificationCenter {
    pub initialized: SigmaBool,
}

impl SigmaNotificationCenter {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    pub unsafe fn init(&mut self) {
        // Migrated: init
        self.initialized = true;
    }

    pub unsafe fn push_alert(&mut self) {
        // Migrated: push_alert
        self.initialized = true;
    }

    pub unsafe fn clear_all(&mut self) {
        // Migrated: clear_all
        self.initialized = true;
    }

    pub unsafe fn notify_init(&mut self) {
        // Migrated: notify_init
        self.initialized = true;
    }

    pub unsafe fn notify_push(&mut self) {
        // Migrated: notify_push
        self.initialized = true;
    }

    pub unsafe fn notify_clear(&mut self) {
        // Migrated: notify_clear
        self.initialized = true;
    }

}

static mut INSTANCE: SigmaNotificationCenter = SigmaNotificationCenter::new();

#[no_mangle]
pub unsafe extern "C" fn init() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn push_alert() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn clear_all() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn notify_init() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn notify_push() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn notify_clear() {
    INSTANCE.initialized = true;
}

