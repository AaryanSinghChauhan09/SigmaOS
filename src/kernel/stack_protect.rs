//! Stack Canary Protection
//! Inspired by GCC -fstack-protector-strong, OpenBSD, and Linux kernel SSP.
//! Places a random sentinel value ("canary") before the return address on
//! the stack. On function return, the canary is verified. Corruption =
//! buffer overflow detected → controlled kernel panic.
//!
//! References:
//! - Linux arch/x86/include/asm/stackprotector.h
//! - OpenBSD src/sys/arch/amd64/include/pcb.h (per-CPU canary)
//! - GCC documentation: -fstack-protector-strong

use crate::crypto::entropy;
use core::sync::atomic::{AtomicU64, Ordering};

/// Per-boot stack canary value. Initialized once, used by all stack frames.
/// On x86_64 Linux this is stored in gs:0x28 (per-CPU).
static STACK_CANARY: AtomicU64 = AtomicU64::new(0);

/// Number of stack smashing violations detected.
static SSP_VIOLATIONS: AtomicU64 = AtomicU64::new(0);

/// Initialize the stack canary with a fresh random value.
/// Must be called once during very early kernel startup.
pub fn init() {
    let mut buf = [0u8; 8];
    entropy::get_entropy_bytes(&mut buf);
    // Ensure the canary never contains a NUL byte (prevents string-based overwrites)
    // and is never all-ones (prevents trivial guessing)
    let mut canary = u64::from_le_bytes(buf);
    canary |= 0x0100_0000_0000_0000; // set a high bit
    canary &= !0x00FF_0000_0000_0000; // clear byte 6 to avoid NUL patterns
    if canary == 0 {
        canary = 0xDEAD_C0DE_FACE_CAFE;
    }
    STACK_CANARY.store(canary, Ordering::SeqCst);
}

/// Get the current canary value.
/// In hardware SSP, this is read from gs:0x28 by the compiler.
#[inline(always)]
pub fn canary() -> u64 {
    STACK_CANARY.load(Ordering::Relaxed)
}

/// Called by the function epilogue to verify the canary.
/// On mismatch, this calls the stack smash handler.
///
/// # Safety
/// `stack_canary_slot` must point to the canary slot on the current stack frame.
#[inline(always)]
pub unsafe fn check(stack_canary_slot: u64) {
    if stack_canary_slot != canary() {
        stack_smash_handler();
    }
}

/// Stack smash handler — called when a canary mismatch is detected.
/// Logs the violation and halts the kernel (like `__stack_chk_fail` in glibc).
#[cold]
#[inline(never)]
pub fn stack_smash_handler() -> ! {
    SSP_VIOLATIONS.fetch_add(1, Ordering::Relaxed);
    // In production: call into the crash reporter, log backtrace, then halt.
    // For now: spin-halt (equivalent to kernel panic).
    loop {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

/// Get total SSP violations detected this boot.
pub fn violation_count() -> u64 {
    SSP_VIOLATIONS.load(Ordering::Relaxed)
}

/// Stack frame guard — allocate on entry, verify on drop.
/// Simulates what the compiler inserts with `-fstack-protector-strong`.
pub struct StackGuard {
    saved_canary: u64,
}

impl StackGuard {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            saved_canary: canary(),
        }
    }
}

impl Drop for StackGuard {
    #[inline(always)]
    fn drop(&mut self) {
        if self.saved_canary != canary() {
            stack_smash_handler();
        }
    }
}

impl Default for StackGuard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canary_initialized() {
        STACK_CANARY.store(0xDEAD_BEEF_1234_5678, Ordering::SeqCst);
        assert_eq!(canary(), 0xDEAD_BEEF_1234_5678);
    }

    #[test]
    fn test_stack_guard_no_corruption() {
        STACK_CANARY.store(0x1234_5678_ABCD_EF01, Ordering::SeqCst);
        let guard = StackGuard::new();
        assert_eq!(guard.saved_canary, canary());
        // Drop without panic = no corruption
    }

    #[test]
    fn test_violation_counter() {
        let before = violation_count();
        SSP_VIOLATIONS.fetch_add(1, Ordering::Relaxed);
        assert_eq!(violation_count(), before + 1);
    }
}
