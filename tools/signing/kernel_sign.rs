// SPDX-License-Identifier: GPL-3.0-or-later
// Kernel and Module Signing Tool for SigmaOS
// Location: tools/signing/kernel_sign.rs

#![no_std]
extern crate alloc;
use alloc::vec::Vec;

pub struct KernelSigningUtility {
    pub key_id: u64,
    pub secret_key: [u8; 32],
}

impl KernelSigningUtility {
    pub fn new(key_id: u64, secret_key: [u8; 32]) -> Self {
        KernelSigningUtility { key_id, secret_key }
    }

    pub fn sign_payload(&self, payload: &[u8]) -> [u8; 64] {
        let mut sig = [0u8; 64];
        // Mock cryptographic signature: derive signature bytes from secret key and payload length
        sig[0] = self.secret_key[0];
        sig[1] = (payload.len() & 0xFF) as u8;
        sig[2] = ((payload.len() >> 8) & 0xFF) as u8;
        for i in 3..64 {
            sig[i] = self.secret_key[i % 32];
        }
        sig
    }

    pub fn append_signature_header(&self, payload: &[u8]) -> Vec<u8> {
        let sig = self.sign_payload(payload);
        let mut signed_binary = Vec::with_capacity(payload.len() + 64 + 8);
        signed_binary.extend_from_slice(&self.key_id.to_le_bytes());
        signed_binary.extend_from_slice(&sig);
        signed_binary.extend_from_slice(payload);
        signed_binary
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_signing_utility() {
        let signer = KernelSigningUtility::new(2001, [0xAA; 32]);
        let payload = b"SIGMAOS_KERNEL_BINARY_TEST_IMAGE";
        let signed = signer.append_signature_header(payload);

        assert_eq!(signed.len(), payload.len() + 64 + 8);
        assert_eq!(&signed[0..8], &2001u64.to_le_bytes());
        assert_eq!(signed[8], 0xAA);
    }
}
