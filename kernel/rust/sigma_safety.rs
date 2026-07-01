// =============================================================================
// Σ SIGMAOS: SOVEREIGN RUST NO_STD SHARD (v1.0 - 0.3% of codebase)
// =============================================================================
// Purpose: Safety-critical validation shard — memory-safe kernel boundary checks.
// Rules:
//   - no_std (zero Rust std runtime, zero alloc crate)
//   - no_main (called as C-compatible extern "C" functions)
//   - panic_handler: calls sigma_kernel_panic (C symbol)
//   - Uses only core:: primitives (no alloc, no std)
// Target: x86_64-unknown-none (bare metal)
// =============================================================================
#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU64, Ordering};
use core::ptr;

// =============================================================================
// Panic handler — required for no_std
// =============================================================================
#[panic_handler]
fn sigma_rust_panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        sigma_kernel_panic(b"RUST_SHARD_PANIC\0".as_ptr() as *const u8);
    }
    loop {}
}

// =============================================================================
// External C kernel functions callable from Rust
// =============================================================================
extern "C" {
    fn kprintf(fmt: *const u8, ...);
    fn sigma_kernel_panic(msg: *const u8) -> !;
}

// =============================================================================
// Atomic counters (lock-free via x86_64 hardware)
// =============================================================================
static VALIDATION_OPS:   AtomicU64 = AtomicU64::new(0);
static BOUNDS_VIOLATIONS: AtomicU64 = AtomicU64::new(0);
static CANARY_CHECKS:    AtomicU64 = AtomicU64::new(0);

// =============================================================================
// Σ RUST SHARD 1: Bounds-Checked Kernel Buffer Validator
// Verifies that a pointer + length does not escape a given region.
// =============================================================================
#[no_mangle]
pub extern "C" fn rust_validate_range(
    ptr:        *const u8,
    len:        u64,
    region_base: u64,
    region_size: u64,
) -> i32 {
    VALIDATION_OPS.fetch_add(1, Ordering::Relaxed);

    let addr = ptr as u64;
    let end  = addr.checked_add(len);

    match end {
        None => {
            // Overflow → definitive out-of-bounds
            BOUNDS_VIOLATIONS.fetch_add(1, Ordering::Relaxed);
            -1
        }
        Some(e) => {
            if addr >= region_base && e <= (region_base + region_size) {
                0  // OK
            } else {
                BOUNDS_VIOLATIONS.fetch_add(1, Ordering::Relaxed);
                -2
            }
        }
    }
}

// =============================================================================
// Σ RUST SHARD 2: Stack Canary Verifier
// Checks that a canary value at a given stack address has not been corrupted.
// =============================================================================
#[no_mangle]
pub extern "C" fn rust_check_stack_canary(
    canary_ptr: *const u64,
    expected:   u64,
) -> i32 {
    CANARY_CHECKS.fetch_add(1, Ordering::Relaxed);

    // Safety: caller guarantees canary_ptr is valid and aligned
    let actual = unsafe { ptr::read_volatile(canary_ptr) };

    if actual == expected {
        0   // canary intact
    } else {
        BOUNDS_VIOLATIONS.fetch_add(1, Ordering::Relaxed);
        unsafe {
            kprintf(
                b"[RUST-SHARD]: STACK SMASH DETECTED @ %p (exp=%llu got=%llu)\n\0"
                    .as_ptr() as *const u8,
                canary_ptr, expected, actual,
            );
            sigma_kernel_panic(b"STACK_SMASH\0".as_ptr() as *const u8);
        }
    }
}

// =============================================================================
// Σ RUST SHARD 3: Memory Region Zeroing (volatile, prevents elision)
// =============================================================================
#[no_mangle]
pub extern "C" fn rust_secure_zero(ptr: *mut u8, len: u64) {
    let mut p = ptr;
    let mut n = len;
    // Volatile write — compiler cannot elide this (critical for key material)
    while n > 0 {
        unsafe { ptr::write_volatile(p, 0u8); }
        p = unsafe { p.add(1) };
        n -= 1;
    }
    VALIDATION_OPS.fetch_add(1, Ordering::Relaxed);
}

// =============================================================================
// Σ RUST SHARD 4: Integer Arithmetic with Overflow Detection
// =============================================================================
#[no_mangle]
pub extern "C" fn rust_safe_add_u64(a: u64, b: u64, result: *mut u64) -> i32 {
    match a.checked_add(b) {
        Some(v) => {
            unsafe { ptr::write(result, v); }
            VALIDATION_OPS.fetch_add(1, Ordering::Relaxed);
            0
        }
        None => {
            BOUNDS_VIOLATIONS.fetch_add(1, Ordering::Relaxed);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn rust_safe_mul_u64(a: u64, b: u64, result: *mut u64) -> i32 {
    match a.checked_mul(b) {
        Some(v) => {
            unsafe { ptr::write(result, v); }
            VALIDATION_OPS.fetch_add(1, Ordering::Relaxed);
            0
        }
        None => {
            BOUNDS_VIOLATIONS.fetch_add(1, Ordering::Relaxed);
            -1
        }
    }
}

// =============================================================================
// Σ RUST SHARD 5: Report statistics back to C kernel logger
// =============================================================================
#[no_mangle]
pub extern "C" fn rust_shard_audit() {
    let ops  = VALIDATION_OPS.load(Ordering::Relaxed);
    let viols = BOUNDS_VIOLATIONS.load(Ordering::Relaxed);
    let can  = CANARY_CHECKS.load(Ordering::Relaxed);

    unsafe {
        kprintf(
            b"[RUST-SHARD]: ops=%llu violations=%llu canary_checks=%llu\n\0"
                .as_ptr() as *const u8,
            ops, viols, can,
        );
    }
}
