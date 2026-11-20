// Simple random number generator for SigmaOS
// Reduces dependency on external rand crate

use core::sync::atomic::{AtomicU64, Ordering};

/// Simple XORShift PRNG implementation
pub struct XorShiftRng {
    state: AtomicU64,
}

impl XorShiftRng {
    /// Create a new RNG with a seed
    pub fn new(seed: u64) -> Self {
        Self {
            state: AtomicU64::new(seed),
        }
    }

    /// Generate a random u64
    pub fn gen_u64(&self) -> u64 {
        let mut state = self.state.load(Ordering::SeqCst);
        state ^= state >> 12;
        state ^= state << 25;
        state ^= state >> 27;
        self.state.store(state, Ordering::SeqCst);
        state
    }

    /// Generate a random u32
    pub fn gen_u32(&self) -> u32 {
        (self.gen_u64() & 0xFFFFFFFF) as u32
    }

    /// Generate a random u16
    pub fn gen_u16(&self) -> u16 {
        (self.gen_u64() & 0xFFFF) as u16
    }

    /// Generate a random u8
    pub fn gen_u8(&self) -> u8 {
        (self.gen_u64() & 0xFF) as u8
    }

    /// Generate a random number in range [min, max)
    pub fn gen_range(&self, min: u64, max: u64) -> u64 {
        let range = max - min;
        if range == 0 {
            return min;
        }
        min + (self.gen_u64() % range)
    }

    /// Generate a random boolean
    pub fn gen_bool(&self) -> bool {
        self.gen_u64() & 1 == 1
    }
}

impl Default for XorShiftRng {
    fn default() -> Self {
        Self::new(0x123456789ABCDEF0)
    }
}

/// Global RNG instance (thread-safe with OnceLock)
static GLOBAL_RNG: std::sync::OnceLock<std::sync::Mutex<XorShiftRng>> = std::sync::OnceLock::new();

/// Initialize the global RNG
pub fn init_global_rng(seed: u64) {
    GLOBAL_RNG.get_or_init(|| std::sync::Mutex::new(XorShiftRng::new(seed)));
}

/// Get a random u64 using the global RNG
pub fn random_u64() -> u64 {
    if let Some(rng_mutex) = GLOBAL_RNG.get() {
        if let Ok(rng) = rng_mutex.lock() {
            return rng.gen_u64();
        }
    }
    // Fallback if not initialized - use a hardware-based or better source in production
    // WARNING: This fallback is NOT cryptographically secure
    // In production, always initialize with init_global_rng() using a proper entropy source
    0x123456789ABCDEF0
}

/// Get a random u32 using the global RNG
pub fn random_u32() -> u32 {
    (random_u64() & 0xFFFFFFFF) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xorshift_rng() {
        let rng = XorShiftRng::new(42);
        let val1 = rng.gen_u64();
        let val2 = rng.gen_u64();
        assert_ne!(val1, val2); // Should produce different values
    }

    #[test]
    fn test_gen_range() {
        let rng = XorShiftRng::new(42);
        let val = rng.gen_range(10, 20);
        assert!(val >= 10 && val < 20);
    }

    #[test]
    fn test_gen_bool() {
        let rng = XorShiftRng::new(42);
        let _ = rng.gen_bool(); // Just ensure it doesn't panic
    }
}
