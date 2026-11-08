// Secure Boot Chain-of-Trust verification model for SigmaOS.
// This module has no cryptographic verifier and never authenticates signatures.
// Location: src/boot/secure_boot.rs

use std::string::String;
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecureBootState {
    Disabled,
    Enabled,
    AuditMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationResult {
    Verified,
    VerificationDisabled,
    VerifierUnavailable,
    UnsignedKernel,
    SignatureInvalid,
    KeyRevokedDbx,
    KeyNotFound,
}

pub struct SecureBootDbEntry {
    pub key_id: u64,
    pub public_key: [u8; 64],
    pub is_revoked: bool,
}

pub struct SecureBootVerifier {
    pub state: SecureBootState,
    pub db: Vec<SecureBootDbEntry>,
    pub dbx: Vec<u64>, // Revoked key IDs
    /// Real Ed25519 (RFC 8032) kernel-signing keys: key_id -> 32-byte pubkey.
    /// Entries here get actual cryptographic verification; the legacy
    /// `db` path remains VerifierUnavailable until a PQ backend lands.
    pub ed25519_db: Vec<(u64, [u8; 32])>,
}

impl SecureBootVerifier {
    pub fn new(state: SecureBootState) -> Self {
        SecureBootVerifier {
            state,
            db: Vec::new(),
            dbx: Vec::new(),
            ed25519_db: Vec::new(),
        }
    }

    /// Enroll a real Ed25519 kernel-signing key
    pub fn enroll_ed25519_key(&mut self, key_id: u64, public_key: [u8; 32]) {
        self.ed25519_db.push((key_id, public_key));
    }

    /// Verify a kernel payload signed with Ed25519.
    /// `signature` is the 64-byte RFC 8032 signature over `payload`
    /// (the raw kernel image bytes — callers hash externally if needed).
    pub fn verify_kernel_signature_ed25519(
        &self,
        key_id: u64,
        payload: &[u8],
        signature: &[u8],
    ) -> VerificationResult {
        if self.state == SecureBootState::Disabled {
            return VerificationResult::VerificationDisabled;
        }
        if self.dbx.contains(&key_id) {
            return VerificationResult::KeyRevokedDbx;
        }
        let Some((_, pubkey)) = self.ed25519_db.iter().find(|(id, _)| *id == key_id) else {
            return VerificationResult::KeyNotFound;
        };
        let Ok(sig) = <[u8; 64]>::try_from(signature) else {
            return VerificationResult::SignatureInvalid;
        };
        if crate::sigpkg::ed25519::verify(pubkey, payload, &sig) {
            VerificationResult::Verified
        } else {
            VerificationResult::SignatureInvalid
        }
    }

    pub fn enroll_allowed_key(&mut self, key_id: u64, public_key: [u8; 64]) {
        self.db.push(SecureBootDbEntry {
            key_id,
            public_key,
            is_revoked: false,
        });
    }

    pub fn revoke_key_dbx(&mut self, key_id: u64) {
        self.dbx.push(key_id);
    }

    pub fn verify_kernel_signature(
        &self,
        key_id: u64,
        _payload_hash: &[u8; 32],
        signature: &[u8; 64],
    ) -> VerificationResult {
        if self.state == SecureBootState::Disabled {
            return VerificationResult::VerificationDisabled;
        }

        // Check dbx (revocation database)
        if self.dbx.contains(&key_id) {
            return VerificationResult::KeyRevokedDbx;
        }

        // Search allowed database db
        if self.db.iter().any(|entry| entry.key_id == key_id) {
            if signature.iter().all(|&b| b == 0) {
                return VerificationResult::UnsignedKernel;
            }
            // No cryptographic verifier is configured. Never report a
            // non-empty signature as verified based on a byte comparison.
            VerificationResult::VerifierUnavailable
        } else {
            VerificationResult::KeyNotFound
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_boot_verification() {
        let mut verifier = SecureBootVerifier::new(SecureBootState::Enabled);
        let mut pubkey = [0u8; 64];
        pubkey[0] = 0xAA;
        verifier.enroll_allowed_key(1001, pubkey);

        let hash = [0u8; 32];
        let mut unverified_sig = [0u8; 64];
        unverified_sig[0] = 0xAA; // Matching one byte is not verification.

        assert_eq!(
            verifier.verify_kernel_signature(1001, &hash, &unverified_sig),
            VerificationResult::VerifierUnavailable
        );

        // Test revoked key in dbx
        verifier.revoke_key_dbx(1001);
        assert_eq!(
            verifier.verify_kernel_signature(1001, &hash, &unverified_sig),
            VerificationResult::KeyRevokedDbx
        );
    }

    #[test]
    fn test_secure_boot_ed25519() {
        use crate::sigpkg::ed25519::{sign_with_seed, Ed25519Signer};

        let mut verifier = SecureBootVerifier::new(SecureBootState::Enabled);
        let (sig, pubkey) = sign_with_seed(&[11u8; 32], b"kernel-image");
        verifier.enroll_ed25519_key(2002, pubkey);

        // valid signature verifies for real
        assert_eq!(
            verifier.verify_kernel_signature_ed25519(2002, b"kernel-image", &sig),
            VerificationResult::Verified
        );
        // tampered payload rejected
        assert_eq!(
            verifier.verify_kernel_signature_ed25519(2002, b"evil-kernel", &sig),
            VerificationResult::SignatureInvalid
        );
        // unknown key id rejected
        assert_eq!(
            verifier.verify_kernel_signature_ed25519(9999, b"kernel-image", &sig),
            VerificationResult::KeyNotFound
        );
        // revoked key rejected even with valid sig
        verifier.revoke_key_dbx(2002);
        assert_eq!(
            verifier.verify_kernel_signature_ed25519(2002, b"kernel-image", &sig),
            VerificationResult::KeyRevokedDbx
        );
        // forged signature from another key fails
        let other = Ed25519Signer::from_seed(&[12u8; 32]);
        let forged = other.sign(b"kernel-image");
        let mut v2 = SecureBootVerifier::new(SecureBootState::Enabled);
        v2.enroll_ed25519_key(2002, pubkey);
        assert_eq!(
            v2.verify_kernel_signature_ed25519(2002, b"kernel-image", &forged),
            VerificationResult::SignatureInvalid
        );
    }
}
