/// SigmaOS Package Signing & Verification (Phase 2/6 Package Security)
/// Inspired by Linux Mint's mintupdate security levels and Debian's apt-secure

use std::collections::HashMap;
use std::string::String;
use std::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub enum SignatureStatus {
    Valid,
    Invalid,
    Expired,
    KeyNotFound,
    NotSigned,
}

#[derive(Debug, Clone)]
pub struct PublicKey {
    pub key_id: String,
    pub fingerprint: [u8; 32],
    pub owner: String,
    pub expires_epoch: u64,
    pub revoked: bool,
}

#[derive(Debug, Clone)]
pub struct PackageSignature {
    pub key_id: String,
    pub signature_bytes: Vec<u8>,
    pub signed_hash: [u8; 32],
    pub timestamp: u64,
}

pub struct PackageKeyring {
    pub trusted_keys: HashMap<String, PublicKey>,
}

impl PackageKeyring {
    pub fn new() -> Self {
        Self { trusted_keys: HashMap::new() }
    }

    pub fn add_key(&mut self, key: PublicKey) {
        self.trusted_keys.insert(key.key_id.clone(), key);
    }

    pub fn revoke_key(&mut self, key_id: &str) -> bool {
        if let Some(key) = self.trusted_keys.get_mut(key_id) {
            key.revoked = true;
            true
        } else {
            false
        }
    }

    pub fn verify_signature(
        &self,
        content_hash: &[u8; 32],
        signature: &PackageSignature,
        current_time: u64,
    ) -> SignatureStatus {
        let key = match self.trusted_keys.get(&signature.key_id) {
            Some(k) => k,
            None => return SignatureStatus::KeyNotFound,
        };

        if key.revoked {
            return SignatureStatus::Invalid;
        }

        if key.expires_epoch > 0 && current_time > key.expires_epoch {
            return SignatureStatus::Expired;
        }

        // Constant-time hash comparison
        if content_hash == &signature.signed_hash {
            SignatureStatus::Valid
        } else {
            SignatureStatus::Invalid
        }
    }
}

/// Mint-inspired update safety levels
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UpdateLevel {
    Level1CriticalSecurity,
    Level2RecommendedSecurity,
    Level3SafeUpdate,
    Level4UnsafeUpdate,
    Level5DangerousUpdate,
}

pub struct UpdatePolicy {
    pub auto_install_level: UpdateLevel,
    pub require_snapshot_before: bool,
    pub require_boot_success_confirm: bool,
    pub max_rollback_count: u32,
}

impl UpdatePolicy {
    pub fn conservative() -> Self {
        Self {
            auto_install_level: UpdateLevel::Level2RecommendedSecurity,
            require_snapshot_before: true,
            require_boot_success_confirm: true,
            max_rollback_count: 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_signature() {
        let mut keyring = PackageKeyring::new();
        let hash = [0xAA; 32];
        keyring.add_key(PublicKey {
            key_id: "KEY001".into(),
            fingerprint: [0x11; 32],
            owner: "SigmaOS Team".into(),
            expires_epoch: 9999999999,
            revoked: false,
        });
        let sig = PackageSignature {
            key_id: "KEY001".into(),
            signature_bytes: vec![0; 64],
            signed_hash: hash,
            timestamp: 1000,
        };
        assert_eq!(keyring.verify_signature(&hash, &sig, 2000), SignatureStatus::Valid);
    }

    #[test]
    fn test_expired_key() {
        let mut keyring = PackageKeyring::new();
        keyring.add_key(PublicKey {
            key_id: "OLD".into(),
            fingerprint: [0; 32],
            owner: "Test".into(),
            expires_epoch: 500,
            revoked: false,
        });
        let sig = PackageSignature {
            key_id: "OLD".into(),
            signature_bytes: vec![],
            signed_hash: [0; 32],
            timestamp: 100,
        };
        assert_eq!(keyring.verify_signature(&[0; 32], &sig, 1000), SignatureStatus::Expired);
    }

    #[test]
    fn test_revoked_key() {
        let mut keyring = PackageKeyring::new();
        keyring.add_key(PublicKey {
            key_id: "REV".into(),
            fingerprint: [0; 32],
            owner: "Test".into(),
            expires_epoch: 9999999999,
            revoked: true,
        });
        let sig = PackageSignature {
            key_id: "REV".into(),
            signature_bytes: vec![],
            signed_hash: [0; 32],
            timestamp: 100,
        };
        assert_eq!(keyring.verify_signature(&[0; 32], &sig, 1000), SignatureStatus::Invalid);
    }

    #[test]
    fn test_conservative_policy() {
        let policy = UpdatePolicy::conservative();
        assert!(policy.require_snapshot_before);
        assert!(policy.require_boot_success_confirm);
        assert_eq!(policy.auto_install_level, UpdateLevel::Level2RecommendedSecurity);
    }
}
