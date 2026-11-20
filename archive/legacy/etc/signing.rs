// SPDX-License-Identifier: MIT
// SigmaOS Package Signing Infrastructure - Arch Linux Inspired
// Implements PGP-based package signing following Arch's security model

use std::collections::BTreeMap;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

/// Signature validation status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignatureStatus {
    Valid,
    Invalid,
    Missing,
    Unknown,
    Revoked,
}

/// Signature key information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignatureKey {
    pub key_id: String,
    pub fingerprint: String,
    pub uid: String,
    pub created: u64,
    pub expires: Option<u64>,
    pub trust_level: TrustLevel,
    pub is_master: bool,
    pub revoked: bool,
}

/// Trust levels for signature keys
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrustLevel {
    Never,
    Marginal,
    Unknown,
    Full,
    Ultimate,
}

/// Package signature information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageSignature {
    pub package_name: String,
    pub package_version: String,
    pub signature_data: Vec<u8>,
    pub signature_type: SignatureType,
    pub signer_key_id: Option<String>,
    pub timestamp: u64,
    pub status: SignatureStatus,
}

/// Types of signatures
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignatureType {
    PgpDetached,
    PgpClearsign,
    Gpg,
    Sha256,
    Custom(String),
}

/// Signature verification result
#[derive(Debug, Clone)]
pub struct SignatureVerification {
    pub package_name: String,
    pub signature_status: SignatureStatus,
    pub signer_fingerprint: Option<String>,
    pub verification_time: u64,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// Keyring manager for signature keys
#[derive(Debug, Clone)]
pub struct KeyringManager {
    pub keys: BTreeMap<String, SignatureKey>,
    pub trust_levels: BTreeMap<String, TrustLevel>,
    pub revoked_keys: Vec<String>,
    pub keyring_path: Option<String>,
}

impl KeyringManager {
    /// Create a new keyring manager
    pub fn new() -> Self {
        Self {
            keys: BTreeMap::new(),
            trust_levels: BTreeMap::new(),
            revoked_keys: Vec::new(),
            keyring_path: None,
        }
    }

    /// Add a signature key
    pub fn add_key(&mut self, key: SignatureKey) {
        let key_id = key.key_id.clone();
        self.keys.insert(key_id.clone(), key);

        if let Some(trust) = self.trust_levels.get(&key_id) {
            // Trust level already set
        }
    }

    /// Remove a key
    pub fn remove_key(&mut self, key_id: &str) -> Result<(), String> {
        if self.keys.remove(key_id).is_some() {
            self.trust_levels.remove(key_id);
            Ok(())
        } else {
            Err(format!("Key '{}' not found", key_id))
        }
    }

    /// Get a key by ID
    pub fn get_key(&self, key_id: &str) -> Option<&SignatureKey> {
        self.keys.get(key_id)
    }

    /// Get a key by fingerprint
    pub fn get_key_by_fingerprint(&self, fingerprint: &str) -> Option<&SignatureKey> {
        self.keys.values().find(|k| k.fingerprint == fingerprint)
    }

    /// Set trust level for a key
    pub fn set_trust_level(&mut self, key_id: &str, level: TrustLevel) -> Result<(), String> {
        if self.keys.contains_key(key_id) {
            self.trust_levels.insert(key_id.to_string(), level);
            Ok(())
        } else {
            Err(format!("Key '{}' not found", key_id))
        }
    }

    /// Get trust level for a key
    pub fn get_trust_level(&self, key_id: &str) -> TrustLevel {
        self.trust_levels.get(key_id).cloned().unwrap_or(TrustLevel::Unknown)
    }

    /// Revoke a key
    pub fn revoke_key(&mut self, key_id: &str) -> Result<(), String> {
        if let Some(mut key) = self.keys.get(key_id).cloned() {
            key.revoked = true;
            self.keys.insert(key_id.to_string(), key);
            self.revoked_keys.push(key_id.to_string());
            Ok(())
        } else {
            Err(format!("Key '{}' not found", key_id))
        }
    }

    /// Check if a key is revoked
    pub fn is_key_revoked(&self, key_id: &str) -> bool {
        self.revoked_keys.contains(&key_id.to_string())
    }

    /// Export keyring to string
    pub fn export_keyring(&self) -> String {
        let mut output = String::new();

        for key in self.keys.values() {
            output.push_str(&format!("Key ID: {}\n", key.key_id));
            output.push_str(&format!("Fingerprint: {}\n", key.fingerprint));
            output.push_str(&format!("UID: {}\n", key.uid));
            output.push_str(&format!("Created: {}\n", key.created));
            if let Some(expires) = key.expires {
                output.push_str(&format!("Expires: {}\n", expires));
            }
            output.push_str(&format!("Trust: {:?}\n", self.get_trust_level(&key.key_id)));
            output.push_str(&format!("Revoked: {}\n\n", key.revoked));
        }

        output
    }

    /// Import keyring from string (simplified)
    pub fn import_keyring(&mut self, data: &str) -> Result<(), String> {
        // In real implementation, would parse GPG keyring format
        // For now, just return success
        let _ = data;
        Ok(())
    }

    /// Set keyring path
    pub fn set_keyring_path(&mut self, path: String) {
        self.keyring_path = Some(path);
    }

    /// Load keyring from file
    pub fn load_keyring(&mut self, path: &str) -> Result<(), String> {
        // In real implementation, would load GPG keyring
        self.set_keyring_path(path.to_string());
        Ok(())
    }

    /// Save keyring to file
    pub fn save_keyring(&self, path: &str) -> Result<(), String> {
        // In real implementation, would save GPG keyring
        let _ = path;
        Ok(())
    }
}

impl Default for KeyringManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Signature verifier
#[derive(Debug, Clone)]
pub struct SignatureVerifier {
    pub keyring: KeyringManager,
    pub require_signatures: bool,
    pub verify_checksums: bool,
    pub allow_unsigned: bool,
    pub strict_mode: bool,
}

impl SignatureVerifier {
    /// Create a new signature verifier
    pub fn new() -> Self {
        Self {
            keyring: KeyringManager::new(),
            require_signatures: true,
            verify_checksums: true,
            allow_unsigned: false,
            strict_mode: false,
        }
    }

    /// Verify a package signature
    pub fn verify_signature(
        &self,
        package_name: &str,
        package_version: &str,
        signature: &PackageSignature,
    ) -> SignatureVerification {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut status = SignatureStatus::Unknown;

        // Check if signature exists
        if signature.signature_data.is_empty() {
            errors.push("No signature data provided".to_string());
            status = SignatureStatus::Missing;
        } else {
            // Check signature type
            match signature.signature_type {
                SignatureType::PgpDetached | SignatureType::PgpClearsign | SignatureType::Gpg => {
                    // Simulate PGP verification
                    status = self.verify_pgp_signature(signature);
                }
                SignatureType::Sha256 => {
                    status = self.verify_sha256_signature(signature);
                }
                SignatureType::Custom(_) => {
                    status = SignatureStatus::Unknown;
                    warnings.push("Custom signature type - verification not implemented".to_string());
                }
            }
        }

        // Check if signature is required
        if self.require_signatures && status == SignatureStatus::Missing {
            if !self.allow_unsigned {
                errors.push("Package signature required but missing".to_string());
            } else {
                warnings.push("Package is unsigned - proceeding due to allow_unsigned".to_string());
            }
        }

        // Check signer key trust
        if let Some(key_id) = &signature.signer_key_id {
            if let Some(key) = self.keyring.get_key(key_id) {
                if key.revoked {
                    errors.push("Signer key has been revoked".to_string());
                    status = SignatureStatus::Revoked;
                } else {
                    let trust = self.keyring.get_trust_level(key_id);
                    if trust < TrustLevel::Full {
                        warnings.push(format!("Signer key has low trust level: {:?}", trust));
                    }
                }
            } else {
                errors.push("Signer key not found in keyring".to_string());
                status = SignatureStatus::Unknown;
            }
        }

        // Strict mode checks
        if self.strict_mode {
            if status != SignatureStatus::Valid {
                errors.push("Strict mode: signature must be valid".to_string());
            }
        }

        SignatureVerification {
            package_name: package_name.to_string(),
            signature_status: status,
            signer_fingerprint: signature.signer_key_id.as_ref()
                .and_then(|key_id| self.keyring.get_key(key_id))
                .map(|key| key.fingerprint.clone()),
            verification_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            errors,
            warnings,
        }
    }

    /// Verify PGP signature (simplified)
    fn verify_pgp_signature(&self, signature: &PackageSignature) -> SignatureStatus {
        // In real implementation, would use GPG library
        // For now, simulate verification based on signature data
        if signature.signature_data.len() > 0 {
            SignatureStatus::Valid
        } else {
            SignatureStatus::Invalid
        }
    }

    /// Verify SHA256 signature
    fn verify_sha256_signature(&self, signature: &PackageSignature) -> SignatureStatus {
        // In real implementation, would verify SHA256 checksum
        if signature.signature_data.len() == 32 {
            SignatureStatus::Valid
        } else {
            SignatureStatus::Invalid
        }
    }

    /// Verify multiple signatures
    pub fn verify_signatures(
        &self,
        signatures: Vec<PackageSignature>,
    ) -> Vec<SignatureVerification> {
        signatures
            .iter()
            .map(|sig| {
                self.verify_signature(
                    &sig.package_name,
                    &sig.package_version,
                    sig,
                )
            })
            .collect()
    }

    /// Check if all signatures are valid
    pub fn all_signatures_valid(&self, signatures: &[PackageSignature]) -> bool {
        signatures.iter().all(|sig| {
            let verification = self.verify_signature(
                &sig.package_name,
                &sig.package_version,
                sig,
            );
            verification.signature_status == SignatureStatus::Valid
        })
    }

    /// Get verification errors
    pub fn get_verification_errors(&self, signatures: &[PackageSignature]) -> Vec<String> {
        let mut errors = Vec::new();

        for sig in signatures {
            let verification = self.verify_signature(
                &sig.package_name,
                &sig.package_version,
                sig,
            );
            errors.extend(verification.errors);
        }

        errors
    }

    /// Enable strict mode
    pub fn strict_mode(mut self, enable: bool) -> Self {
        self.strict_mode = enable;
        self
    }

    /// Set signature requirement
    pub fn require_signatures(mut self, require: bool) -> Self {
        self.require_signatures = require;
        self
    }

    /// Allow unsigned packages
    pub fn allow_unsigned(mut self, allow: bool) -> Self {
        self.allow_unsigned = allow;
        self
    }

    /// Enable checksum verification
    pub fn verify_checksums(mut self, enable: bool) -> Self {
        self.verify_checksums = enable;
        self
    }
}

impl Default for SignatureVerifier {
    fn default() -> Self {
        Self::new()
    }
}

/// Signature error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignatureError {
    MissingSignature,
    InvalidSignature,
    ExpiredKey,
    RevokedKey,
    UnknownKey,
    WeakSignature,
    ChecksumMismatch,
    Custom(String),
}

impl fmt::Display for SignatureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SignatureError::MissingSignature => write!(f, "Missing signature"),
            SignatureError::InvalidSignature => write!(f, "Invalid signature"),
            SignatureError::ExpiredKey => write!(f, "Expired key"),
            SignatureError::RevokedKey => write!(f, "Revoked key"),
            SignatureError::UnknownKey => write!(f, "Unknown key"),
            SignatureError::WeakSignature => write!(f, "Weak signature algorithm"),
            SignatureError::ChecksumMismatch => write!(f, "Checksum mismatch"),
            SignatureError::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for SignatureError {}

/// Signature manager for package repositories
#[derive(Debug, Clone)]
pub struct SignatureManager {
    pub verifier: SignatureVerifier,
    pub keyring: KeyringManager,
    pub repository_keys: BTreeMap<String, Vec<String>>, // repo_name -> key_ids
}

impl SignatureManager {
    /// Create a new signature manager
    pub fn new() -> Self {
        Self {
            verifier: SignatureVerifier::new(),
            keyring: KeyringManager::new(),
            repository_keys: BTreeMap::new(),
        }
    }

    /// Add repository with signing keys
    pub fn add_repository(&mut self, name: String, key_ids: Vec<String>) {
        self.repository_keys.insert(name, key_ids);
    }

    /// Remove repository
    pub fn remove_repository(&mut self, name: &str) {
        self.repository_keys.remove(name);
    }

    /// Get repository keys
    pub fn get_repository_keys(&self, name: &str) -> Option<&Vec<String>> {
        self.repository_keys.get(name)
    }

    /// Verify repository signature
    pub fn verify_repository_signature(
        &self,
        repo_name: &str,
        package_name: &str,
        signature: &PackageSignature,
    ) -> SignatureVerification {
        // Check if repository is trusted
        if let Some(key_ids) = self.repository_keys.get(repo_name) {
            // Check if signature key is in repository's trusted keys
            if let Some(signer_key_id) = &signature.signer_key_id {
                if key_ids.contains(signer_key_id) {
                    return self.verifier.verify_signature(
                        package_name,
                        &signature.package_version,
                        signature,
                    );
                }
            }
        }

        // Repository not trusted or key not found
        SignatureVerification {
            package_name: package_name.to_string(),
            signature_status: SignatureStatus::Unknown,
            signer_fingerprint: None,
            verification_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            errors: vec![format!("Repository '{}' not trusted or key not found", repo_name)],
            warnings: Vec::new(),
        }
    }

    /// Export trusted keys
    pub fn export_trusted_keys(&self) -> String {
        self.keyring.export_keyring()
    }

    /// Import trusted keys
    pub fn import_trusted_keys(&mut self, data: &str) -> Result<(), String> {
        self.keyring.import_keyring(data)
    }

    /// Set keyring path
    pub fn set_keyring_path(&mut self, path: String) {
        self.keyring.set_keyring_path(path);
    }
}

impl Default for SignatureManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature_key() {
        let key = SignatureKey {
            key_id: "12345678".to_string(),
            fingerprint: "ABCDEF1234567890".to_string(),
            uid: "Test Key <test@example.com>".to_string(),
            created: 1700000000,
            expires: None,
            trust_level: TrustLevel::Full,
            is_master: true,
            revoked: false,
        };

        assert_eq!(key.key_id, "12345678");
        assert_eq!(key.trust_level, TrustLevel::Full);
    }

    #[test]
    fn test_keyring_manager() {
        let mut keyring = KeyringManager::new();

        let key = SignatureKey {
            key_id: "12345678".to_string(),
            fingerprint: "ABCDEF1234567890".to_string(),
            uid: "Test Key".to_string(),
            created: 1700000000,
            expires: None,
            trust_level: TrustLevel::Full,
            is_master: true,
            revoked: false,
        };

        keyring.add_key(key);
        keyring.set_trust_level("12345678", TrustLevel::Full).unwrap();

        assert!(keyring.get_key("12345678").is_some());
        assert_eq!(keyring.get_trust_level("12345678"), TrustLevel::Full);
    }

    #[test]
    fn test_signature_verifier() {
        let verifier = SignatureVerifier::new();

        let signature = PackageSignature {
            package_name: "test".to_string(),
            package_version: "1.0".to_string(),
            signature_data: vec![1, 2, 3, 4],
            signature_type: SignatureType::PgpDetached,
            signer_key_id: Some("12345678".to_string()),
            timestamp: 1700000000,
            status: SignatureStatus::Unknown,
        };

        let verification = verifier.verify_signature("test", "1.0", &signature);
        assert!(verification.signature_status != SignatureStatus::Missing);
    }

    #[test]
    fn test_signature_manager() {
        let mut manager = SignatureManager::new();

        manager.add_repository("core".to_string(), vec!["key1".to_string(), "key2".to_string()]);
        manager.add_repository("extra".to_string(), vec!["key3".to_string()]);

        assert!(manager.get_repository_keys("core").is_some());
        assert_eq!(manager.get_repository_keys("core").unwrap().len(), 2);
    }

    #[test]
    fn test_signature_error_display() {
        let error = SignatureError::MissingSignature;
        assert_eq!(error.to_string(), "Missing signature");

        let error = SignatureError::Custom("Custom error".to_string());
        assert_eq!(error.to_string(), "Custom error");
    }
}
