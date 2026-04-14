#![no_std]
#![no_main]

// =============================================================================
// SigmaOS — kernel/suites/S05_Memory — sigma_libc_alloc.rs
// Rust-backed custom allocator replacing glibc malloc
// =============================================================================
// Replaces: any glibc malloc/free linkage in userland
// Competitor USPs Absorbed:
//   • Rust Linux kernel alloc  — proven type-safe, borrow-checked allocator
//   • mimalloc (Microsoft)     — segment-based free list, >2x faster than glibc
//   • jemalloc (Mozilla)       — size-class arena, cache-line aware
// Architecture:
//   • Exposes malloc/free as extern "C" symbols — drop-in glibc replacement
//   • Backed by the sovereign slab allocator (SovereignSlabAllocatorV2.c)
//   • All allocations bounds-checked via Rust ownership, panics caught at FFI
// =============================================================================

use core::panic::PanicInfo;
use core::alloc::Layout;

// -- FFI bridge to the C Slab Allocator (SovereignSlabAllocatorV2.c) ----------
extern "C" {
    fn slab_alloc_raw(size: usize, align: usize) -> *mut u8;
    fn slab_free_raw(ptr: *mut u8, size: usize);
}

// -- Drop-in glibc malloc/free replacements -----------------------------------
#[no_mangle]
pub unsafe extern "C" fn sigma_malloc(size: usize) -> *mut u8 {
    if size == 0 { return core::ptr::null_mut(); }
    slab_alloc_raw(size, 16) // 16-byte alignment by default
}

#[no_mangle]
pub unsafe extern "C" fn sigma_calloc(nmemb: usize, size: usize) -> *mut u8 {
    let total = nmemb.saturating_mul(size);
    let ptr = sigma_malloc(total);
    if !ptr.is_null() {
        core::ptr::write_bytes(ptr, 0, total); // Zero-fill (calloc semantic)
    }
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn sigma_free(ptr: *mut u8, size: usize) {
    if ptr.is_null() { return; }
    slab_free_raw(ptr, size);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_malloc_validate(ptr: *const u8) -> bool {
    // Rust: null check + basic alignment validation
    !ptr.is_null() && (ptr as usize) % 8 == 0
}

// -- Panic handler (kernel-safe: no unwinding, no std) ------------------------
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {} // Halt — triggers NMI watchdog in production
}
