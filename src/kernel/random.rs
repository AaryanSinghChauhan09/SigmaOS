// Linux-inspired random number generator interface
// getrandom, urandom, and /dev/random for SigmaOS

use std::sync::{Arc, Mutex};

/// Random source type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RandomSource {
    Urandom,
    Random,
}

/// Random quality flags (Linux getrandom.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RandomFlags {
    pub non_blocking: bool,
    pub no_warn: bool,
}

impl RandomFlags {
    pub fn new() -> Self {
        RandomFlags {
            non_blocking: false,
            no_warn: false,
        }
    }

    pub fn with_non_blocking(mut self) -> Self {
        self.non_blocking = true;
        self
    }

    pub fn with_no_warn(mut self) -> Self {
        self.no_warn = true;
        self
    }
}

impl Default for RandomFlags {
    fn default() -> Self {
        Self::new()
    }
}

/// Random bytes result
#[derive(Debug, Clone)]
pub struct RandomBytes {
    data: Vec<u8>,
    source: RandomSource,
}

impl RandomBytes {
    pub fn new(data: Vec<u8>, source: RandomSource) -> Self {
        RandomBytes {
            data,
            source,
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn source(&self) -> RandomSource {
        self.source
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

/// Random generator state
#[derive(Debug)]
struct RandomState {
    seed: u64,
}

impl RandomState {
    pub fn new(seed: u64) -> Self {
        RandomState {
            seed,
        }
    }

    /// Simple XOR-shift random number generator
    pub fn next(&mut self) -> u64 {
        self.seed ^= self.seed >> 12;
        self.seed ^= self.seed << 25;
        self.seed ^= self.seed >> 27;
        self.seed
    }

    /// Generate random bytes
    pub fn random_bytes(&mut self, len: usize) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(len);
        for _ in 0..len {
            let random = self.next();
            bytes.push((random & 0xFF) as u8);
        }
        bytes
    }
}

/// Random manager for the system
pub struct RandomManager {
    urandom_state: Arc<Mutex<RandomState>>,
    random_state: Arc<Mutex<RandomState>>,
}

impl RandomManager {
    pub fn new() -> Self {
        // Initialize with time-based seeds (simplified)
        let seed1 = 1234567890;
        let seed2 = 9876543210;
        
        RandomManager {
            urandom_state: Arc::new(Mutex::new(RandomState::new(seed1))),
            random_state: Arc::new(Mutex::new(RandomState::new(seed2))),
        }
    }

    /// Get random bytes from urandom (non-blocking)
    pub fn getrandom_urandom(&self, len: usize, _flags: RandomFlags) -> Result<RandomBytes, String> {
        let mut state = self.urandom_state.lock().unwrap();
        let data = state.random_bytes(len);
        
        Ok(RandomBytes::new(data, RandomSource::Urandom))
    }

    /// Get random bytes from random (blocking for entropy)
    pub fn getrandom_random(&self, len: usize, _flags: RandomFlags) -> Result<RandomBytes, String> {
        let mut state = self.random_state.lock().unwrap();
        let data = state.random_bytes(len);
        
        Ok(RandomBytes::new(data, RandomSource::Random))
    }

    /// Get random bytes from specified source
    pub fn getrandom(&self, source: RandomSource, len: usize, flags: RandomFlags) -> Result<RandomBytes, String> {
        match source {
            RandomSource::Urandom => self.getrandom_urandom(len, flags),
            RandomSource::Random => self.getrandom_random(len, flags),
        }
    }

    /// Reseed the random generators
    pub fn reseed(&self, urandom_seed: u64, random_seed: u64) {
        let mut urandom_state = self.urandom_state.lock().unwrap();
        *urandom_state = RandomState::new(urandom_seed);
        
        let mut random_state = self.random_state.lock().unwrap();
        *random_state = RandomState::new(random_seed);
    }
}

impl Default for RandomManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_flags_creation() {
        let flags = RandomFlags::new();
        assert!(!flags.non_blocking);
        assert!(!flags.no_warn);
    }

    #[test]
    fn test_random_flags_with_non_blocking() {
        let flags = RandomFlags::new().with_non_blocking();
        assert!(flags.non_blocking);
    }

    #[test]
    fn test_random_state_creation() {
        let mut state = RandomState::new(12345);
        let val1 = state.next();
        let val2 = state.next();
        assert_ne!(val1, val2); // Next should be different
    }

    #[test]
    fn test_random_state_random_bytes() {
        let mut state = RandomState::new(12345);
        let bytes = state.random_bytes(16);
        assert_eq!(bytes.len(), 16);
    }

    #[test]
    fn test_random_bytes_creation() {
        let bytes = RandomBytes::new(vec![1, 2, 3, 4], RandomSource::Urandom);
        assert_eq!(bytes.len(), 4);
        assert_eq!(bytes.source(), RandomSource::Urandom);
    }

    #[test]
    fn test_random_manager_creation() {
        let _manager = RandomManager::new();
        // Just verify it doesn't panic
        assert!(true);
    }

    #[test]
    fn test_random_manager_getrandom_urandom() {
        let manager = RandomManager::new();
        let flags = RandomFlags::new();
        
        let result = manager.getrandom_urandom(16, flags);
        assert!(result.is_ok());
        
        let bytes = result.unwrap();
        assert_eq!(bytes.len(), 16);
    }

    #[test]
    fn test_random_manager_getrandom_random() {
        let manager = RandomManager::new();
        let flags = RandomFlags::new();
        
        let result = manager.getrandom_random(16, flags);
        assert!(result.is_ok());
        
        let bytes = result.unwrap();
        assert_eq!(bytes.len(), 16);
    }

    #[test]
    fn test_random_manager_getrandom() {
        let manager = RandomManager::new();
        let flags = RandomFlags::new();
        
        let result = manager.getrandom(RandomSource::Urandom, 16, flags);
        assert!(result.is_ok());
        
        let bytes = result.unwrap();
        assert_eq!(bytes.len(), 16);
    }

    #[test]
    fn test_random_manager_reseed() {
        let manager = RandomManager::new();
        manager.reseed(9999, 8888);
        // Just verify it doesn't panic
        assert!(true);
    }
}
