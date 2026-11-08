// SPDX-License-Identifier: MIT
// SigmaOS Ed25519 package signatures (RFC 8032).
// Thin wrapper over the audited ed25519-dalek crate so package signing
// uses real asymmetric cryptography rather than the symmetric HMAC path.

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

/// Ed25519 signing key (32-byte seed)
#[derive(Debug)]
pub struct Ed25519Signer {
    key: SigningKey,
}

impl Ed25519Signer {
    /// Deterministic key from a 32-byte seed (RFC 8032 keygen).
    /// Callers can source seeds from `crypto::primitives::random_bytes`.
    pub fn from_seed(seed: &[u8; 32]) -> Self {
        Self {
            key: SigningKey::from_bytes(seed),
        }
    }

    /// Public key bytes (32 bytes, compressed point)
    pub fn public_key(&self) -> [u8; 32] {
        *self.key.verifying_key().as_bytes()
    }

    /// Sign a message, returning the 64-byte RFC 8032 signature
    pub fn sign(&self, message: &[u8]) -> [u8; 64] {
        let sig: Signature = self.key.sign(message);
        sig.to_bytes()
    }
}

/// Verify an Ed25519 signature against a 32-byte public key.
/// Returns false on malformed keys, signatures, or verification failure.
pub fn verify(public_key: &[u8; 32], message: &[u8], signature: &[u8; 64]) -> bool {
    let Ok(vk) = VerifyingKey::from_bytes(public_key) else {
        return false;
    };
    let Ok(sig) = Signature::from_slice(signature) else {
        return false;
    };
    vk.verify(message, &sig).is_ok()
}

/// Convenience: sign with seed bytes directly
pub fn sign_with_seed(seed: &[u8; 32], message: &[u8]) -> ([u8; 64], [u8; 32]) {
    let signer = Ed25519Signer::from_seed(seed);
    (signer.sign(message), signer.public_key())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rfc8032_test1() {
        // RFC 8032 section 7.1 TEST 1
        let seed: [u8; 32] = [
            0x9d, 0x61, 0xb1, 0x9d, 0xef, 0xfd, 0x5a, 0x60, 0xba, 0x84, 0x4a, 0xf4, 0x92, 0xec,
            0x2c, 0xc4, 0x44, 0x49, 0xc5, 0x69, 0x7b, 0x32, 0x69, 0x19, 0x70, 0x3b, 0xac, 0x03,
            0x1c, 0xae, 0x7f, 0x60,
        ];
        fn hex(b: &[u8]) -> String {
            b.iter().map(|x| format!("{:02x}", x)).collect()
        }
        let signer = Ed25519Signer::from_seed(&seed);
        assert_eq!(
            hex(&signer.public_key()),
            "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"
        );
        let sig = signer.sign(b"");
        assert_eq!(
            hex(&sig),
            "e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e065224901555fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b"
        );
        assert!(verify(&signer.public_key(), b"", &sig));
        assert!(!verify(&signer.public_key(), b"tampered", &sig));
    }
}
