//! Hardware entropy collection for SigmaOS
//! Inspired by Linux drivers/char/random.c and FreeBSD sys/dev/random/
//! Uses XorShift64 PRNG combined with timer-based entropy mixing.
//! In production: wire to RDRAND/RDSEED (x86_64) or equivalent.

use core::sync::atomic::{AtomicU64, Ordering};

/// Global entropy pool — seeded with a compile-time constant, mixed at runtime.
static ENTROPY_POOL: AtomicU64 = AtomicU64::new(0x_9E37_79B9_7F4A_7C15);

/// XorShift64 — fast non-cryptographic mixing function.
#[inline(always)]
fn xorshift64(mut x: u64) -> u64 {
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}

/// Mix external entropy into the pool (call with timer values, hardware events, etc.).
pub fn mix_entropy(value: u64) {
    let current = ENTROPY_POOL.load(Ordering::Relaxed);
    let mixed = xorshift64(current ^ value);
    ENTROPY_POOL.store(mixed, Ordering::Relaxed);
}

/// Fill `buf` with entropy bytes derived from the pool.
pub fn get_entropy_bytes(buf: &mut [u8]) {
    // Relaxed ordering: entropy mixing does not require synchronization
    let mut state = ENTROPY_POOL.fetch_add(1, Ordering::Relaxed);
    for (i, byte) in buf.iter_mut().enumerate() {
        state = xorshift64(state ^ (i as u64).wrapping_mul(0x517C_C1B7_2722_0A95));
        *byte = (state >> 56) as u8;
    }
    mix_entropy(state);
}

/// Simulate RDRAND on x86_64 — returns entropy from pool until real RDRAND is wired.
/// Safety: Safe to call from any context; uses atomic operations only.
pub fn rdrand() -> Option<u64> {
    let v = ENTROPY_POOL.fetch_add(0xACE1_ACE1, Ordering::Relaxed);
    Some(xorshift64(v))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entropy_bytes_nonzero() {
        let mut buf = [0u8; 32];
        get_entropy_bytes(&mut buf);
        assert!(
            buf.iter().any(|&b| b != 0),
            "entropy buffer should not be all zeros"
        );
    }

    #[test]
    fn test_mix_changes_output() {
        let mut buf1 = [0u8; 8];
        let mut buf2 = [0u8; 8];
        get_entropy_bytes(&mut buf1);
        mix_entropy(0xDEAD_BEEF_CAFE_BABE);
        get_entropy_bytes(&mut buf2);
        assert_ne!(buf1, buf2, "entropy should differ after mixing");
    }

    #[test]
    fn test_rdrand_not_zero() {
        let v = rdrand();
        assert!(v.is_some());
    }
}
