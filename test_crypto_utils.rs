// Standalone test for cryptographic utilities
// This can be run with: rustc --test test_crypto_utils.rs && ./test_crypto_utils

#[cfg(test)]
mod tests {
    // Import the crypto_utils module directly
    mod crypto_utils {
        include!("src/security/crypto_utils.rs");
    }

    use crypto_utils::*;

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
