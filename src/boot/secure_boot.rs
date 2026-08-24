// Secure Boot Chain-of-Trust Verification Engine for SigmaOS
// Location: src/boot/secure_boot.rs

// #![no_std]
extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecureBootState {
    Disabled,
    Enabled,
    AuditMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationResult {
    Verified,
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

    pub fn verify_kernel_signature(&self, key_id: u64, payload_hash: &[u8; 32], signature: &[u8; 64]) -> VerificationResult {
        if self.state == SecureBootState::Disabled {
            return VerificationResult::Verified;
        }

        // Check dbx (revocation database)
        if self.dbx.contains(&key_id) {
            return VerificationResult::KeyRevokedDbx;
        }

        // Search allowed database db
        if let Some(entry) = self.db.iter().find(|e| e.key_id == key_id) {
            // Verify mock signature (first byte match or non-zero signature check)
            if signature.iter().all(|&b| b == 0) {
                return VerificationResult::UnsignedKernel;
            }
            // Mock cryptographic verification
            if signature[0] == entry.public_key[0] {
                VerificationResult::Verified
            } else {
                VerificationResult::SignatureInvalid
            }
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
        let mut valid_sig = [0u8; 64];
        valid_sig[0] = 0xAA; // Matches pubkey[0]

        assert_eq!(
            verifier.verify_kernel_signature(1001, &hash, &valid_sig),
            VerificationResult::Verified
        );

        // Test revoked key in dbx
        verifier.revoke_key_dbx(1001);
        assert_eq!(
            verifier.verify_kernel_signature(1001, &hash, &valid_sig),
            VerificationResult::KeyRevokedDbx
        );
    }
}
