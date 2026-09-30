//! Random-number APIs. No OS entropy provider is integrated; secure requests
//! return an error and leave caller buffers unchanged.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RandomError {
    EntropyUnavailable,
    InvalidRange,
}

/// API placeholder for a future vetted CSPRNG provider.
#[derive(Default)]
pub struct SovereignCsprng;

impl SovereignCsprng {
    pub const fn new() -> Self {
        Self
    }

    pub fn generate_block(&mut self) -> Result<[u8; 64], RandomError> {
        Err(RandomError::EntropyUnavailable)
    }

    pub fn fill_bytes(&mut self, _buf: &mut [u8]) -> Result<(), RandomError> {
        Err(RandomError::EntropyUnavailable)
    }
}

/// Request secure bytes. This fails closed until an OS/hardware entropy
/// provider with reviewed seeding and synchronization is integrated.
pub fn random_bytes(_buf: &mut [u8]) -> Result<(), RandomError> {
    Err(RandomError::EntropyUnavailable)
}

pub fn random_u32() -> Result<u32, RandomError> {
    Err(RandomError::EntropyUnavailable)
}

pub fn random_u64() -> Result<u64, RandomError> {
    Err(RandomError::EntropyUnavailable)
}

pub fn random_usize() -> Result<usize, RandomError> {
    Err(RandomError::EntropyUnavailable)
}

pub fn random_range(max: usize) -> Result<usize, RandomError> {
    if max == 0 {
        return Err(RandomError::InvalidRange);
    }
    Err(RandomError::EntropyUnavailable)
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
    use super::{random_bytes, random_range, RandomError, XorShiftRng};

    #[test]
    fn secure_random_fails_without_modifying_the_buffer() {
        let mut buf = [0xA5; 16];
        assert_eq!(random_bytes(&mut buf), Err(RandomError::EntropyUnavailable));
        assert_eq!(buf, [0xA5; 16]);
        assert_eq!(random_range(100), Err(RandomError::EntropyUnavailable));
        assert_eq!(random_range(0), Err(RandomError::InvalidRange));
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
