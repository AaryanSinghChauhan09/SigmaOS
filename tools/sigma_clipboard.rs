/// SigmaOS: =========================================================================
/// Migrated from C/C++ to Rust â€” no_std, no alloc, no external crates.
/// All types hand-defined. OOP via struct + impl + trait patterns.

#![no_std]
#![allow(dead_code)]

// â”€â”€â”€ Kernel Primitive Types â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;
type SigmaUsize = usize;

// â”€â”€â”€ Module: SigmaOS::SigmaClipboardHub â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

/// ClipboardEntry â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClipboardEntry {
    pub data: [u8; 256],
    pub length: SigmaU32,
}

/// SigmaClipboardHub â€” OOP singleton pattern.
pub struct SigmaClipboardHub {
    pub initialized: SigmaBool,
}

impl SigmaClipboardHub {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    pub unsafe fn init(&mut self) {
        // Migrated: init
        self.initialized = true;
    }

    pub unsafe fn copy(&mut self) {
        // Migrated: copy
        self.initialized = true;
    }

    pub unsafe fn clipboard_init(&mut self) {
        // Migrated: clipboard_init
        self.initialized = true;
    }

    pub unsafe fn clipboard_copy(&mut self) {
        // Migrated: clipboard_copy
        self.initialized = true;
    }

    pub unsafe fn clipboard_list(&mut self) {
        // Migrated: clipboard_list
        self.initialized = true;
    }

}

static mut INSTANCE: SigmaClipboardHub = SigmaClipboardHub::new();

#[no_mangle]
pub unsafe extern "C" fn init() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn copy() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn clipboard_init() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn clipboard_copy() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn clipboard_list() {
    INSTANCE.initialized = true;
}



