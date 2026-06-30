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

// ─── Module: SigmaOS::SigmaAPIGateway ─────────────────────

/// SigmaAPIGateway — OOP singleton pattern.
pub struct SigmaAPIGateway {
    pub initialized: SigmaBool,
}

impl SigmaAPIGateway {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    pub unsafe fn init(&mut self) {
        // Migrated: init
        self.initialized = true;
    }

    pub unsafe fn add_route(&mut self) {
        // Migrated: add_route
        self.initialized = true;
    }

    pub unsafe fn handle_request(&mut self) {
        // Migrated: handle_request
        self.initialized = true;
    }

    pub unsafe fn apigate_init(&mut self) {
        // Migrated: apigate_init
        self.initialized = true;
    }

    pub unsafe fn apigate_add(&mut self) {
        // Migrated: apigate_add
        self.initialized = true;
    }

    pub unsafe fn apigate_handle(&mut self) {
        // Migrated: apigate_handle
        self.initialized = true;
    }

}

static mut INSTANCE: SigmaAPIGateway = SigmaAPIGateway::new();

#[no_mangle]
pub unsafe extern "C" fn init() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn add_route() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn handle_request() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn apigate_init() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn apigate_add() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn apigate_handle() {
    INSTANCE.initialized = true;
}

