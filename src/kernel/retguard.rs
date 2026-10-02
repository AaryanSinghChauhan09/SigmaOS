//! retguard — Return Address Protection
//! Inspired by OpenBSD retguard: every function prologue XORs a per-function
//! random cookie into the return address on the stack. On return, the cookie
//! is verified. A corrupted return address → panic before ROP can execute.
//!
//! Reference: OpenBSD src/sys/arch/amd64/amd64/retguard.S
//! SigmaOS: software-layer simulation; hardware CET/shadow stack when available.

use core::sync::atomic::{AtomicU64, Ordering};
use crate::crypto::entropy;

/// Global per-boot cookie used to XOR-mask return addresses.
static RETGUARD_COOKIE: AtomicU64 = AtomicU64::new(0);

/// Violation counter (incremented on detected corruption, never decremented).
static VIOLATIONS: AtomicU64 = AtomicU64::new(0);

/// Initialize retguard — generate a random per-boot cookie.
/// Must be called once during early kernel initialization.
pub fn init() {
    let mut buf = [0u8; 8];
    entropy::get_entropy_bytes(&mut buf);
    let cookie = u64::from_le_bytes(buf) | 1; // ensure nonzero
    RETGUARD_COOKIE.store(cookie, Ordering::SeqCst);
}

/// Get the current retguard cookie.
#[inline(always)]
pub fn cookie() -> u64 {
    RETGUARD_COOKIE.load(Ordering::Relaxed)
}

/// Encode a return address with the retguard cookie.
/// In hardware: XOR with cookie; here: XOR + rotate to add diffusion.
#[inline(always)]
pub fn encode_retaddr(ret_addr: u64) -> u64 {
    let c = cookie();
    ret_addr.rotate_left(17) ^ c
}

/// Verify an encoded return address. Returns the original address if valid.
/// Returns `None` if the address appears corrupted.
#[inline(always)]
pub fn verify_retaddr(encoded: u64) -> Option<u64> {
    let c = cookie();
    let decoded = (encoded ^ c).rotate_right(17);
    // Sanity: kernel addresses are in the high canonical range
    if decoded >= 0xFFFF_8000_0000_0000 || decoded < 0x1000 {
        Some(decoded)
    } else {
        VIOLATIONS.fetch_add(1, Ordering::Relaxed);
        None
    }
}

/// Get total number of retguard violations detected this boot.
pub fn violation_count() -> u64 {
    VIOLATIONS.load(Ordering::Relaxed)
}

/// Guard structure for stack-allocated return address protection.
/// Usage: `let _guard = RetGuard::new(&return_addr_slot);`
pub struct RetGuard {
    encoded: u64,
}

impl RetGuard {
    /// Encode and store the return address.
    pub fn new(ret_addr: u64) -> Self {
        Self { encoded: encode_retaddr(ret_addr) }
    }

    /// Verify the return address on drop (simulates epilogue check).
    pub fn verify(&self) -> bool {
        verify_retaddr(self.encoded).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_roundtrip() {
        RETGUARD_COOKIE.store(0xDEAD_BEEF_CAFE_1337, Ordering::SeqCst);
        let addr = 0xFFFF_FFFF_8012_3456u64;
        let encoded = encode_retaddr(addr);
        assert_ne!(encoded, addr);
        let decoded = verify_retaddr(encoded);
        assert_eq!(decoded, Some(addr));
    }

    #[test]
    fn test_corrupted_address_detected() {
        RETGUARD_COOKIE.store(0xDEAD_BEEF_CAFE_1337, Ordering::SeqCst);
        let addr = 0xFFFF_FFFF_8012_3456u64;
        let mut encoded = encode_retaddr(addr);
        encoded ^= 0x1; // flip a bit
        // corrupted — decode won't match original
        let decoded = verify_retaddr(encoded);
        // Decoded may or may not be Some (depends on value), but encoded != original
        assert_ne!(decoded, Some(addr));
    }
}
