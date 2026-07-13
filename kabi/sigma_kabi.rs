// sigma_kabi.rs
// SigmaOS Kernel ABI Checker — replaces sigma_kabi.c
// Zero-dependency Rust implementation
// Checks that exported kernel symbols match an approved ABI manifest

#![no_std]
#![allow(dead_code)]

use core::ffi::c_char;
use core::slice;

// ── Approved ABI Symbol Table ────────────────────────────────────────────
// Populated from the KABI manifest. In a full implementation this is
// loaded from a signed binary manifest file.
const APPROVED_SYMBOLS: [&str; 12] = [
    "sigma_kmalloc",
    "sigma_kfree",
    "sigma_printk",
    "sigma_schedule",
    "sigma_alloc_pages",
    "sigma_free_pages",
    "sigma_mmap_region",
    "sigma_ipc_send",
    "sigma_ipc_recv",
    "sigma_sandbox_enter",
    "sigma_sandbox_exit",
    "",
];

// ── Symbol Check ──────────────────────────────────────────────────────────
fn is_approved(sym: &str) -> bool {
    for approved in APPROVED_SYMBOLS.iter() {
        if approved.is_empty() {
            break;
        }
        if sym == *approved {
            return true;
        }
    }
    false
}

// ── C string to Rust string conversion ─────────────────────────────────────
unsafe fn c_str_to_str(ptr: *const c_char) -> Option<&'static str> {
    if ptr.is_null() {
        return None;
    }
    
    let mut len = 0;
    while *ptr.add(len) != 0 {
        len += 1;
    }
    
    let slice = slice::from_raw_parts(ptr as *const u8, len);
    core::str::from_utf8(slice).ok()
}

// ── Check KABI from symbol list ─────────────────────────────────────────────
pub fn check_kabi_symbols(symbols: &[&str]) -> (usize, usize) {
    let mut violations = 0;
    let mut checked = 0;
    
    for sym in symbols.iter() {
        if sym.starts_with('#') || sym.is_empty() {
            continue;
        }
        
        checked += 1;
        if !is_approved(sym) {
            violations += 1;
        }
    }
    
    (checked, violations)
}

// ── Main entry point for no_std context ────────────────────────────────────
#[no_mangle]
pub extern "C" fn sigma_kabi_check(symbol_ptr: *const c_char, symbol_count: usize) -> i32 {
    if symbol_ptr.is_null() {
        return 1; // Error: null pointer
    }
    
    let symbols_slice = unsafe { slice::from_raw_parts(symbol_ptr, symbol_count) };
    let mut rust_symbols = Vec::new();
    
    for &ptr in symbols_slice.iter() {
        if let Some(sym) = unsafe { c_str_to_str(ptr) } {
            rust_symbols.push(sym);
        }
    }
    
    let (checked, violations) = check_kabi_symbols(&rust_symbols);
    
    if violations > 0 {
        2 // Error: violations found
    } else {
        0 // Success
    }
}

// ── Simple Vec implementation for no_std ───────────────────────────────────
struct Vec<T> {
    ptr: *mut T,
    capacity: usize,
    len: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Self {
            ptr: core::ptr::null_mut(),
            capacity: 0,
            len: 0,
        }
    }
    
    fn push(&mut self, item: T) {
        // Simplified - in real implementation would need allocation
        self.len += 1;
    }
    
    fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}
