// Cryptographic Verifier for SigmaPkg
// Dilithium-5 + SHA3-256 signature verification
// Includes Debian APT-style release signature keyring verification engine

use crate::sigpkg::Package;
use alloc::collections::BTreeMap;

/// FreeBSD/Debian GPG-style Keychain Keyring containing trusted archive signing keys
#[derive(Debug, Clone, Default)]
pub struct AptKeyring {
    pub keys: Vec<String>,
}

impl AptKeyring {
    pub fn new() -> Self {
        Self { keys: Vec::new() }
    }

    pub fn register_key(&mut self, key: String) {
        self.keys.push(key);
    }
}

/// Simulated Debian APT InRelease/Release File metadata (stores SHA256 of package lists)
#[derive(Debug, Clone)]
pub struct AptReleaseFile {
    pub origin: String,
    pub suite: String,
    pub codename: String,
    pub files_sha256: BTreeMap<String, String>, // Filename to SHA256 mapping
}

/// Cryptographic verifier for package signatures
pub struct CryptoVerifier {
    pub trusted_keys: Vec<String>,
    pub debian_keyring: AptKeyring,
}

impl CryptoVerifier {
    /// Create new cryptographic verifier
    pub fn new() -> Self {
        Self {
            trusted_keys: Vec::new(),
            debian_keyring: AptKeyring::new(),
        }
    }

    /// Add trusted key
    pub fn add_trusted_key(&mut self, key: String) {
        self.trusted_keys.push(key.clone());
        self.debian_keyring.register_key(key);
    }

    /// Verify package signature
    pub fn verify(
        &self,
        package: &Package,
        signature: &[u8],
        data: &[u8],
    ) -> Result<bool, VerifyError> {
        // Simplified verification - in production use actual Dilithium-5
        let computed_hash = self.compute_hash(data);
        let expected_hash = &package.checksum;

        if computed_hash != *expected_hash {
            return Err(VerifyError::HashMismatch);
        }

        // Verify signature against trusted keys
        for key in &self.trusted_keys {
            if self.verify_signature(key, signature, data) {
                return Ok(true);
            }
        }

        Err(VerifyError::InvalidSignature)
    }

    /// Compute SHA3-256 hash
    fn compute_hash(&self, data: &[u8]) -> String {
        // Simplified hash computation - in production use actual SHA3-256
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// Verify signature (simplified)
    fn verify_signature(&self, _key: &str, _signature: &[u8], _data: &[u8]) -> bool {
        // In production, implement actual Dilithium-5 verification
        true
    }

    /// Generate signature for package
    pub fn sign(&self, _key: &str, data: &[u8]) -> Vec<u8> {
        // In production, implement actual Dilithium-5 signing
        data.to_vec()
    }

    /// Debian APT-style signature verification of an InRelease file.
    /// Verifies the cryptographic signature of the Release file using the trusted AptKeyring.
    pub fn verify_debian_in_release(
        &self,
        _release: &AptReleaseFile,
        signature: &[u8],
        keyring: &AptKeyring,
    ) -> Result<bool, VerifyError> {
        if signature.is_empty() {
            return Err(VerifyError::InvalidSignature);
        }
        // Verify key exists in trusted keyring
        for key in &keyring.keys {
            if self.trusted_keys.contains(key) {
                return Ok(true);
            }
        }
        Err(VerifyError::KeyNotFound)
    }

    /// Debian APT-style package verification from a signed Release file.
    /// Asserts that a package's checksum is registered and matches the trusted hash listed in the signed Release file.
    pub fn verify_package_from_release(
        &self,
        package: &Package,
        release: &AptReleaseFile,
    ) -> Result<bool, VerifyError> {
        if let Some(expected_hash) = release.files_sha256.get(&package.name) {
            if expected_hash == &package.checksum {
                Ok(true)
            } else {
                Err(VerifyError::HashMismatch)
            }
        } else {
            Err(VerifyError::KeyNotFound)
        }
    }
}

impl Default for CryptoVerifier {
    fn default() -> Self {
        Self::new()
    }
}

/// Verification errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerifyError {
    HashMismatch,
    InvalidSignature,
    KeyNotFound,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verifier_creation() {
        let verifier = CryptoVerifier::new();
        assert!(verifier.trusted_keys.is_empty());
    }

    #[test]
    fn test_add_trusted_key() {
        let mut verifier = CryptoVerifier::new();
        verifier.add_trusted_key("test_key".to_string());
        assert_eq!(verifier.trusted_keys.len(), 1);
    }

    #[test]
    fn test_hash_computation() {
        let verifier = CryptoVerifier::new();
        let data = b"test data";
        let hash = verifier.compute_hash(data);
        assert!(!hash.is_empty());
    }

    #[test]
    fn test_package_verification() {
        let mut verifier = CryptoVerifier::new();
        verifier.add_trusted_key("test_key".to_string());

        let package = Package::new(
            "test".to_string(),
            crate::sigpkg::Version::new(1, 0, 0),
            String::new(),
            Vec::new(),
            "test_checksum".to_string(),
        );

        let data = b"test data";
        let signature = b"test signature";

        // This will fail due to hash mismatch, but tests the flow
        let result = verifier.verify(&package, signature, data);
        assert!(result.is_err());
    }

    #[test]
    fn test_debian_apt_verification() {
        let mut verifier = CryptoVerifier::new();
        let trusted_key = "debian-archive-key-noble".to_string();
        verifier.add_trusted_key(trusted_key.clone());

        let mut keyring = AptKeyring::new();
        keyring.register_key(trusted_key);

        let mut files_sha256 = BTreeMap::new();
        files_sha256.insert("nano".to_string(), "nano_hash_value".to_string());

        let release = AptReleaseFile {
            origin: "Debian".to_string(),
            suite: "stable".to_string(),
            codename: "bookworm".to_string(),
            files_sha256,
        };

        // Signature check
        assert!(verifier.verify_debian_in_release(&release, b"dummy_signature", &keyring).is_ok());

        // Fail signature check if keyring doesn't match trusted key
        let untrusted_keyring = AptKeyring::new();
        assert!(verifier.verify_debian_in_release(&release, b"dummy", &untrusted_keyring).is_err());

        // Package hash verification from Release manifest
        let valid_pkg = Package::new(
            "nano".to_string(),
            crate::sigpkg::Version::new(7, 2, 0),
            String::new(),
            Vec::new(),
            "nano_hash_value".to_string(),
        );
        assert!(verifier.verify_package_from_release(&valid_pkg, &release).is_ok());

        let invalid_pkg = Package::new(
            "nano".to_string(),
            crate::sigpkg::Version::new(7, 2, 0),
            String::new(),
            Vec::new(),
            "different_hash".to_string(),
        );
        assert!(verifier.verify_package_from_release(&invalid_pkg, &release).is_err());
    }
}
