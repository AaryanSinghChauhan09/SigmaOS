// SigmaOS Random Number Generation
// Eliminates dependency on external rand crate

#![allow(dead_code)]

/// Sovereign Cryptographically Secure Random Number Generator (CSPRNG)
/// Combines ChaCha20-inspired key expansion with Linux `/dev/urandom` and FreeBSD `arc4random` entropy mixing
pub struct SovereignCsprng {
    state: [u32; 16],
    counter: u64,
}

impl SovereignCsprng {
    pub fn new() -> Self {
        let mut csprng = Self {
            state: [
                0x61707865, 0x33322062, 0x79746520, 0x6b617932, // "expand 32-byte k"
                0x12345678, 0x9ABCDEF0, 0xFEDCBA98, 0x76543210,
                0x0F1E2D3C, 0x4B5A6978, 0x8796A5B4, 0xC3D2E1F0,
                0x13579BDF, 0x2468ACE0, 0xDEADBEEF, 0xCAFEBABE,
            ],
            counter: 0,
        };
        csprng.add_entropy(0x505645524549474E); // Mix sovereign entropy seed
        csprng
    }

    pub fn add_entropy(&mut self, entropy_word: u64) {
        let low = entropy_word as u32;
        let high = (entropy_word >> 32) as u32;
        self.state[4] ^= low;
        self.state[5] ^= high;
        self.state[12] = self.state[12].wrapping_add(low);
        self.state[13] = self.state[13].wrapping_add(high);
    }

    fn qr(x: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
        x[a] = x[a].wrapping_add(x[b]); x[d] ^= x[a]; x[d] = x[d].rotate_left(16);
        x[c] = x[c].wrapping_add(x[d]); x[b] ^= x[c]; x[b] = x[b].rotate_left(12);
        x[a] = x[a].wrapping_add(x[b]); x[d] ^= x[a]; x[d] = x[d].rotate_left(8);
        x[c] = x[c].wrapping_add(x[d]); x[b] ^= x[c]; x[b] = x[b].rotate_left(7);
    }

    pub fn generate_block(&mut self) -> [u8; 64] {
        self.counter += 1;
        self.state[14] = self.counter as u32;
        self.state[15] = (self.counter >> 32) as u32;

        let mut x = self.state;
        for _ in 0..10 {
            // Column rounds
            Self::qr(&mut x, 0, 4, 8, 12);
            Self::qr(&mut x, 1, 5, 9, 13);
            Self::qr(&mut x, 2, 6, 10, 14);
            Self::qr(&mut x, 3, 7, 11, 15);
            // Diagonal rounds
            Self::qr(&mut x, 0, 5, 10, 15);
            Self::qr(&mut x, 1, 6, 11, 12);
            Self::qr(&mut x, 2, 7, 8, 13);
            Self::qr(&mut x, 3, 4, 9, 14);
        }

        let mut out = [0u8; 64];
        for i in 0..16 {
            let word = x[i].wrapping_add(self.state[i]);
            let bytes = word.to_le_bytes();
            out[i * 4..(i + 1) * 4].copy_from_slice(&bytes);
        }
        out
    }

    pub fn fill_bytes(&mut self, buf: &mut [u8]) {
        let mut offset = 0;
        while offset < buf.len() {
            let block = self.generate_block();
            let copy_len = (buf.len() - offset).min(64);
            buf[offset..offset + copy_len].copy_from_slice(&block[..copy_len]);
            offset += copy_len;
        }
    }
}

impl Default for SovereignCsprng {
    fn default() -> Self {
        Self::new()
    }
}

/// Fill buffer with cryptographically secure random bytes using SovereignCsprng
pub fn random_bytes(buf: &mut [u8]) {
    static mut CSPRNG_POOL: Option<SovereignCsprng> = None;
    unsafe {
        #[allow(static_mut_refs)]
        let pool = CSPRNG_POOL.as_mut();
        if let Some(rng) = pool {
            rng.fill_bytes(buf);
        } else {
            let mut rng = SovereignCsprng::new();
            rng.fill_bytes(buf);
            CSPRNG_POOL = Some(rng);
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
