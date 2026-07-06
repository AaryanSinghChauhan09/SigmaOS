/// SigmaOS: =============================================================================
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

// â”€â”€â”€ Module: Sigma::console â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

/// VGAConsole â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VGAConsole {
    pub col: SigmaU32,
    pub row: SigmaU32,
    pub attr: SigmaU8,
}

#[no_mangle]
pub unsafe extern "C" fn vga_scroll() {
}

#[no_mangle]
pub unsafe extern "C" fn vga_putc() {
}

#[no_mangle]
pub unsafe extern "C" fn vga_init() {
}

#[no_mangle]
pub unsafe extern "C" fn serial_init() {
}

#[no_mangle]
pub unsafe extern "C" fn serial_putc() {
}

#[no_mangle]
pub unsafe extern "C" fn serial_puts() {
}

#[no_mangle]
pub unsafe extern "C" fn kprint_u64() {
}

#[no_mangle]
pub unsafe extern "C" fn kprint_str() {
}

#[no_mangle]
pub unsafe extern "C" fn kprint_char() {
}

#[no_mangle]
pub unsafe extern "C" fn kprintf() {
}

#[no_mangle]
pub unsafe extern "C" fn console_init() {
}



