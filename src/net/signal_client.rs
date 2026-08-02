// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::vec::Vec;

/// Signal Parity: Secure Messaging Core
/// Implementation of a basic Double Ratchet cryptographic state machine.

pub struct DoubleRatchetState {
    pub root_key: [u8; 32],
    pub send_chain_key: [u8; 32],
    pub receive_chain_key: [u8; 32],
    pub message_number: u32,
}

impl DoubleRatchetState {
    pub fn new(shared_secret: [u8; 32]) -> Self {
        Self {
            root_key: shared_secret,
            send_chain_key: [0; 32],
            receive_chain_key: [0; 32],
            message_number: 0,
        }
    }

    /// Simulates a ratchet step for sending a message
    pub fn ratchet_encrypt(&mut self, plaintext: &[u8]) -> Vec<u8> {
        self.message_number += 1;
        // In a real implementation, KDF(chain_key) would produce a message key.
        // For this #![no_std] placeholder, we simply XOR with a fixed byte for demonstration.
        let mut ciphertext = Vec::with_capacity(plaintext.len());
        for &byte in plaintext {
            ciphertext.push(byte ^ 0x42);
        }
        ciphertext
    }

    /// Simulates a ratchet step for receiving a message
    pub fn ratchet_decrypt(&mut self, ciphertext: &[u8]) -> Vec<u8> {
        // Real implementation would try to derive the message key from receive_chain_key.
        let mut plaintext = Vec::with_capacity(ciphertext.len());
        for &byte in ciphertext {
            plaintext.push(byte ^ 0x42);
        }
        plaintext
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_ratchet_encryption() {
        let mut state = DoubleRatchetState::new([1; 32]);
        let plaintext = b"Hello Signal";
        
        let ciphertext = state.ratchet_encrypt(plaintext);
        assert_ne!(ciphertext, plaintext);
        
        let decrypted = state.ratchet_decrypt(&ciphertext);
        assert_eq!(decrypted, plaintext);
        assert_eq!(state.message_number, 1);
    }
}
