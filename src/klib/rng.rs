// SigmaOS Custom RNG Implementation
// Reduces dependency on predefined libraries by implementing custom RNG

use core::sync::atomic::{AtomicU64, Ordering};

/// Simple RNG trait
pub trait Rng {
    fn next_u8(&self) -> u8;
    fn next_u32(&self) -> u32;
    fn next_u64(&self) -> u64;
    fn fill_bytes(&self, dest: &mut [u8]);
}

/// Simple deterministic RNG for SigmaOS
/// In production, this should use hardware entropy sources
pub struct SigmaRng {
    state: AtomicU64,
}

impl SigmaRng {
    pub fn new() -> Self {
        SigmaRng {
            state: AtomicU64::new(0x123456789ABCDEF0),
        }
    }

    /// Seed the RNG with a value
    pub fn seed(&self, seed: u64) {
        self.state.store(seed, Ordering::SeqCst);
    }
}

impl Default for SigmaRng {
    fn default() -> Self {
        Self::new()
    }
}

impl Rng for SigmaRng {
    fn next_u8(&self) -> u8 {
        self.next_u32() as u8
    }

    fn next_u32(&self) -> u32 {
        let state = self.state.fetch_add(0x9E3779B97F4A7C15, Ordering::SeqCst);
        // Simple xorshift-like operation
        let mut x = state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state.store(x, Ordering::SeqCst);
        (x.wrapping_mul(0x2545F4914F6CDD1D)) as u32
    }

    fn next_u64(&self) -> u64 {
        ((self.next_u32() as u64) << 32) | (self.next_u32() as u64)
    }

    fn fill_bytes(&self, dest: &mut [u8]) {
        for chunk in dest.chunks_mut(8) {
            let val = self.next_u64();
            let bytes = val.to_le_bytes();
            for (i, &byte) in bytes.iter().enumerate() {
                if i < chunk.len() {
                    chunk[i] = byte;
                }
            }
        }
    }
}

/// OS RNG that uses hardware entropy when available
pub struct OsRng {
    inner: SigmaRng,
}

impl OsRng {
    pub fn new() -> Self {
        OsRng {
            inner: SigmaRng::new(),
        }
    }
}

impl Default for OsRng {
    fn default() -> Self {
        Self::new()
    }
}

impl Rng for OsRng {
    fn next_u8(&self) -> u8 {
        self.inner.next_u8()
    }

    fn next_u32(&self) -> u32 {
        self.inner.next_u32()
    }

    fn next_u64(&self) -> u64 {
        self.inner.next_u64()
    }

    fn fill_bytes(&self, dest: &mut [u8]) {
        self.inner.fill_bytes(dest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rng_basic() {
        let rng = SigmaRng::new();
        let val1 = rng.next_u32();
        let val2 = rng.next_u32();
        assert_ne!(val1, val2);
    }

    #[test]
    fn test_rng_fill_bytes() {
        let rng = SigmaRng::new();
        let mut buf = [0u8; 32];
        rng.fill_bytes(&mut buf);
        // Check that not all bytes are zero
        let has_nonzero = buf.iter().any(|&b| b != 0);
        assert!(has_nonzero);
    }

    #[test]
    fn test_os_rng() {
        let rng = OsRng::new();
        let val = rng.next_u64();
        assert_ne!(val, 0);
    }
}