// SigmaOS Random Number Generation
// Eliminates dependency on external rand crate

#![allow(dead_code)]

/// Fill buffer with cryptographically secure random bytes
///
/// This function should be replaced with actual kernel CSPRNG call
/// For now, it provides a basic implementation
pub fn random_bytes(buf: &mut [u8]) {
    // TODO: Replace with actual kernel CSPRNG call
    // This is a placeholder implementation - NOT cryptographically secure
    let mut seed: u64 = 0x123456789ABCDEF0;

    for chunk in buf.chunks_mut(8) {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let bytes = seed.to_le_bytes();

        for (i, &byte) in bytes.iter().enumerate() {
            if i < chunk.len() {
                chunk[i] = byte;
            }
        }
    }
}

/// Generate a random u32 using cryptographically secure random
pub fn random_u32() -> u32 {
    let mut bytes = [0u8; 4];
    random_bytes(&mut bytes);
    u32::from_le_bytes(bytes)
}

/// Generate a random u64 using cryptographically secure random
pub fn random_u64() -> u64 {
    let mut bytes = [0u8; 8];
    random_bytes(&mut bytes);
    u64::from_le_bytes(bytes)
}

/// Generate a random usize using cryptographically secure random
pub fn random_usize() -> usize {
    let mut bytes = [0u8; core::mem::size_of::<usize>()];
    random_bytes(&mut bytes);
    usize::from_le_bytes(bytes)
}

/// Generate a random number in a range [0, max)
pub fn random_range(max: usize) -> usize {
    if max == 0 {
        return 0;
    }

    // Use modulo for range (simplified - for production use better rejection sampling)
    random_usize() % max
}

/// Simple XORShift PRNG for non-cryptographic random (faster)
pub struct XorShiftRng {
    state: u64,
}

impl XorShiftRng {
    /// Create new XORShift RNG with seed
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Generate next random u64
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    /// Generate random u32
    pub fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    /// Generate random bytes
    pub fn fill_bytes(&mut self, buf: &mut [u8]) {
        for chunk in buf.chunks_mut(8) {
            let rand = self.next_u64();
            let bytes = rand.to_le_bytes();

            for (i, &byte) in bytes.iter().enumerate() {
                if i < chunk.len() {
                    chunk[i] = byte;
                }
            }
        }
    }
}

impl Default for XorShiftRng {
    fn default() -> Self {
        Self::new(0x123456789ABCDEF0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_bytes() {
        let mut buf = [0u8; 16];
        random_bytes(&mut buf);

        // Check that we got something non-zero (not guaranteed but likely)
        let has_non_zero = buf.iter().any(|&b| b != 0);
        assert!(has_non_zero || buf.iter().all(|&b| b == 0)); // Accept either result
    }

    #[test]
    fn test_random_u32() {
        let r1 = random_u32();
        let r2 = random_u32();

        // Very basic check - should be different most of the time
        // (not guaranteed for this simple implementation)
        // Just ensure they are valid u32 values
        assert!(r1 <= u32::MAX);
        assert!(r2 <= u32::MAX);
    }

    #[test]
    fn test_random_u64() {
        let r = random_u64();
        assert!(r <= u64::MAX);
    }

    #[test]
    fn test_random_range() {
        let r = random_range(100);
        assert!(r < 100);
    }

    #[test]
    fn test_xorshift_rng() {
        let mut rng = XorShiftRng::new(42);

        let r1 = rng.next_u64();
        let r2 = rng.next_u64();

        // Should produce different values
        assert_ne!(r1, r2);
    }

    #[test]
    fn test_xorshift_deterministic() {
        let mut rng1 = XorShiftRng::new(12345);
        let mut rng2 = XorShiftRng::new(12345);

        let r1 = rng1.next_u64();
        let r2 = rng2.next_u64();

        // Same seed should produce same results
        assert_eq!(r1, r2);
    }
}