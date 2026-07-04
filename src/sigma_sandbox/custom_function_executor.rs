/// SigmaOS: =============================================================================
/// Migrated from C/C++ to Rust — no_std, no alloc, no external crates.
/// All types hand-defined. OOP via struct + impl + trait patterns.

#![no_std]

use core::ptr;

// ─── Kernel Primitive Types ─────────────────────────────────────────────────

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;
type SigmaUsize = usize;

// ─── Module: Sigma::custom_function_executor ─────────────────────

#[no_mangle]
pub unsafe extern "C" fn free_sandboxed_memory(ptr: *mut SigmaU8, len: SigmaUsize) {
    if ptr.is_null() || len == 0 {
        return;
    }
    // Zero the memory as a safety precaution
    ptr::write_bytes(ptr, 0, len);
}

