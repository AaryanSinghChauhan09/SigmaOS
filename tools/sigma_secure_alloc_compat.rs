//! SigmaOS Secure Zero-on-Free Allocator Helper
//! Implements strict memory scrubbing and zeroing guarantees on deallocation,
//! inspired by OpenBSD's hardened security-first memory allocators (omalloc).
//! Zero external dependencies

#![no_std]
#![allow(dead_code)]

type SigmaU8 = u8;
type SigmaU32 = u32;
type SigmaI32 = i32;
type SigmaBool = bool;
type SigmaSize = usize;

/// Statistics tracker for memory scrubbing operations
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AllocScrubStats {
    pub total_pages_scrubbed: SigmaSize,
    pub total_bytes_zeroed: u64,
}

static mut SCRUBBER_INITIALIZED: SigmaBool = false;
static mut SCRUB_STATS: AllocScrubStats = AllocScrubStats {
    total_pages_scrubbed: 0,
    total_bytes_zeroed: 0,
};

/// Initialize the secure memory scrubbing subsystem
#[no_mangle]
pub unsafe extern "C" fn secure_alloc_init() -> SigmaI32 {
    SCRUBBER_INITIALIZED = true;
    SCRUB_STATS.total_pages_scrubbed = 0;
    SCRUB_STATS.total_bytes_zeroed = 0;
    0 // Success
}

/// Securely zero out a memory segment before returning it to the kernel page pool
#[no_mangle]
pub unsafe extern "C" fn secure_alloc_free_scrub(ptr: *mut u8, len: SigmaSize) -> SigmaI32 {
    if !SCRUBBER_INITIALIZED || ptr.is_null() || len == 0 {
        return -1;
    }

    // Force volatile writes to ensure the compiler does not optimize away the zeroing operation (dead code elimination)
    for i in 0..len {
        core::ptr::write_volatile(ptr.add(i), 0);
    }

    // Update global tracking telemetry
    SCRUB_STATS.total_bytes_zeroed += len as u64;
    if len >= 4096 {
        SCRUB_STATS.total_pages_scrubbed += len / 4096;
    }

    0 // Success
}

/// Get secure memory scrubbing performance statistics
#[no_mangle]
pub unsafe extern "C" fn secure_alloc_get_stats(stats: *mut AllocScrubStats) -> SigmaI32 {
    if !SCRUBBER_INITIALIZED || stats.is_null() {
        return -1;
    }

    *stats = SCRUB_STATS;
    0 // Success
}
