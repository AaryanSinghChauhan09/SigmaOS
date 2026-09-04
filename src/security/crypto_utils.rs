//! Cryptographic Utilities for SigmaOS
//! 
//! This module provides secure random number generation and cryptographic utilities.
//! In production, these should use hardware RNG or properly vetted cryptographic libraries.
use std::vec;

use std::vec::Vec;

/// Error types for cryptographic operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CryptoError {
    RandomGenerationFailed,
    InvalidKeyLength,
    InvalidNonce,
}

/// Simple cryptographic random number generator
/// 
/// WARNING: This is a basic implementation for development/testing purposes.
/// In production, use:
/// - Hardware RNG (RDRAND on x86, RNG on ARM)
/// - Or a vetted cryptographic library like RustCrypto/rand
pub struct SecureRandom {
    // In a real implementation, this would maintain internal state
    // for a proper CSPRNG (ChaCha20, AES-CTR, etc.)
}

impl SecureRandom {
    pub fn new() -> Self {
        Self {}
    }

    /// Fill a buffer with cryptographically secure random bytes
    /// 
    /// # Arguments
    /// * `buffer` - Mutable slice to fill with random bytes
    /// 
    /// # Returns
    /// * `Result<(), CryptoError>` - Success or error
    pub fn fill_bytes(&mut self, buffer: &mut [u8]) -> Result<(), CryptoError> {
        // WARNING: This is a mock implementation using a simple LCG
        // Never use this in production! Use proper CSPRNG.
        
        // In production, this would call:
        // - Hardware RNG instructions
        // - Or a cryptographic PRNG seeded from hardware entropy
        
        const DEFAULT_PRNG_SEED: u64 = 0x5a5a5a5a5a5a5a5a;

        #[cfg(not(target_os = "none"))]
        let mut seed: u64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(DEFAULT_PRNG_SEED as u128) as u64;

        #[cfg(target_os = "none")]
        let mut seed: u64 = DEFAULT_PRNG_SEED;
        for byte in buffer.iter_mut() {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            *byte = (seed >> 32) as u8;
        }
        
        Ok(())
    }

    /// Generate a random key of specified length
    /// 
    /// # Arguments
    /// * `length` - Desired key length in bytes
    /// 
    /// # Returns
    /// * `Result<Vec<u8>, CryptoError>` - Random key or error
    pub fn generate_key(&mut self, length: usize) -> Result<Vec<u8>, CryptoError> {
        if length == 0 {
            return Err(CryptoError::InvalidKeyLength);
        }
        
        let mut key = vec![0u8; length];
        self.fill_bytes(&mut key)?;
        Ok(key)
    }

    /// Generate a random 256-bit (32-byte) key suitable for AES-256
    pub fn generate_aes256_key(&mut self) -> Result<[u8; 32], CryptoError> {
        let mut key = [0u8; 32];
        self.fill_bytes(&mut key)?;
        Ok(key)
    }

    /// Generate a random 128-bit (16-byte) key suitable for AES-128
    pub fn generate_aes128_key(&mut self) -> Result<[u8; 16], CryptoError> {
        let mut key = [0u8; 16];
        self.fill_bytes(&mut key)?;
        Ok(key)
    }

    /// Generate a random nonce for cryptographic operations
    pub fn generate_nonce(&mut self, length: usize) -> Result<Vec<u8>, CryptoError> {
        if length == 0 || length > 64 {
            return Err(CryptoError::InvalidNonce);
        }
        self.generate_key(length)
    }
}

impl Default for SecureRandom {
    fn default() -> Self {
        Self::new()
    }
}

/// Constant-time comparison for cryptographic values
/// 
/// This prevents timing attacks that could leak information about
/// secret values (like MACs or passwords).
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    
    let mut result = 0u8;
    for (byte_a, byte_b) in a.iter().zip(b.iter()) {
        result |= byte_a ^ byte_b;
    }
    
    result == 0
}

/// Simple password hashing (placeholder)
/// 
/// WARNING: This is a placeholder for development only.
/// In production, use Argon2, bcrypt, or scrypt with proper parameters.
pub fn hash_password_placeholder(password: &str, salt: &[u8; 16]) -> [u8; 32] {
    // This is NOT secure - just a placeholder for testing
    // In production, use:
    // - argon2 crate for password hashing
    // - Or bcrypt/scrypt with proper parameters
    
    let mut hash = [0u8; 32];
    let password_bytes = password.as_bytes();
    
    // Performance optimization: Replace the index-modulo loop (R1-Bolt-optimization)
    // with a single-pass iterator chain using `.iter().cycle()`.
    // This completely eliminates:
    // 1. Division/modulo instructions (`% password_bytes.len()`, `% 16`), which cost 10-40 cycles.
    // 2. Bounds checking insertions, allowing compiler auto-vectorization and clean unrolling.
    let mut pwd_cycle = password_bytes.iter().cycle();
    let mut salt_cycle = salt.iter().cycle();

    for h_byte in hash.iter_mut() {
        if let (Some(&p_b), Some(&s_b)) = (pwd_cycle.next(), salt_cycle.next()) {
            *h_byte = p_b ^ s_b;
        }
    }
    
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_random_generation() {
        let mut rng = SecureRandom::new();
        
        let mut buffer = [0u8; 32];
        assert!(rng.fill_bytes(&mut buffer).is_ok());
        
        // Ensure we don't get all zeros (statistically unlikely)
        let all_zeros = buffer.iter().all(|&b| b == 0);
        assert!(!all_zeros);
    }

    #[test]
    fn test_key_generation() {
        let mut rng = SecureRandom::new();
        
        let key = rng.generate_aes256_key();
        assert!(key.is_ok());
        
        let key = key.unwrap();
        // Ensure key is not all zeros
        let all_zeros = key.iter().all(|&b| b == 0);
        assert!(!all_zeros);
    }

    #[test]
    fn test_constant_time_comparison() {
        let a = [1u8, 2, 3, 4];
        let b = [1u8, 2, 3, 4];
        let c = [1u8, 2, 3, 5];
        
        assert!(constant_time_eq(&a, &b));
        assert!(!constant_time_eq(&a, &c));
    }

    #[test]
    fn test_constant_time_different_lengths() {
        let a = [1u8, 2, 3];
        let b = [1u8, 2, 3, 4];
        
        assert!(!constant_time_eq(&a, &b));
    }

    #[test]
    fn test_nonce_generation() {
        let mut rng = SecureRandom::new();
        
        let nonce = rng.generate_nonce(12);
        assert!(nonce.is_ok());
        
        let nonce = nonce.unwrap();
        assert_eq!(nonce.len(), 12);
    }

    #[test]
    fn test_invalid_nonce_length() {
        let mut rng = SecureRandom::new();
        
        let nonce = rng.generate_nonce(0);
        assert!(nonce.is_err());
        
        let nonce = rng.generate_nonce(65);
        assert!(nonce.is_err());
    }
}
