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
}

impl SecureBootVerifier {
    pub fn new(state: SecureBootState) -> Self {
        SecureBootVerifier {
            state,
            db: Vec::new(),
            dbx: Vec::new(),
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
}
