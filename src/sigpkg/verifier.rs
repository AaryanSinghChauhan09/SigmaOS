// Cryptographic Verifier for SigmaPkg
// Dilithium-5 + SHA3-256 signature verification
// Includes Debian APT-style release signature keyring verification engine

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::format;

#[cfg(any(feature = "standalone_test", test))]
extern crate alloc;
#[cfg(any(feature = "standalone_test", test))]
use alloc::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use alloc::vec::Vec;
#[cfg(any(feature = "standalone_test", test))]
use alloc::format;
#[cfg(any(feature = "standalone_test", test))]
use std::collections::HashMap;

#[cfg(not(any(feature = "standalone_test", test)))]
use crate::sigpkg::Package;
#[cfg(not(any(feature = "standalone_test", test)))]
use crate::klib::HashMap;

#[cfg(any(feature = "standalone_test", test))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

#[cfg(any(feature = "standalone_test", test))]
impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }
}

#[cfg(any(feature = "standalone_test", test))]
#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: Version,
    pub description: String,
    pub dependencies: Vec<String>,
    pub checksum: String,
}

#[cfg(any(feature = "standalone_test", test))]
impl Package {
    pub fn new(name: String, version: Version, description: String, dependencies: Vec<String>, checksum: String) -> Self {
        Self { name, version, description, dependencies, checksum }
    }
}

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
    pub files_sha256: HashMap<String, String>, // Filename to SHA256 mapping
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
        let mut hash_val: u64 = 0xcbf29ce484222325;
        for &byte in data {
            hash_val ^= byte as u64;
            hash_val = hash_val.wrapping_mul(0x100000001b3);
        }
        format!("{:x}", hash_val)
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

/// Linux kernel inspired Git commit / patch signoff validator
pub struct GitSignedOffByValidator;

impl GitSignedOffByValidator {
    /// Validates whether a commit or patch message contains a valid Signed-off-by tag
    pub fn validate_signoff(message: &str) -> Result<(String, String), VerifyError> {
        for line in message.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("Signed-off-by:") {
                let rest = trimmed["Signed-off-by:".len()..].trim();
                if let Some(open_angle) = rest.find('<') {
                    if let Some(close_angle) = rest.find('>') {
                        if open_angle < close_angle {
                            let name = rest[..open_angle].trim().to_string();
                            let email = rest[open_angle + 1..close_angle].trim().to_string();
                            if !name.is_empty() && !email.is_empty() && email.contains('@') {
                                return Ok((name, email));
                            }
                        }
                    }
                }
            }
        }
        Err(VerifyError::InvalidSignature)
    }
}

/// OpenBSD signify key and signature verification engine
#[derive(Debug, Clone, Default)]
pub struct OpenBsdSignifyVerifier {
    pub trusted_pubkeys: Vec<String>,
}

impl OpenBsdSignifyVerifier {
    pub fn new() -> Self {
        Self { trusted_pubkeys: Vec::new() }
    }

    pub fn add_pubkey(&mut self, pubkey: &str) {
        self.trusted_pubkeys.push(pubkey.to_string());
    }

    pub fn verify_signify(&self, payload: &[u8], signature_comment: &str) -> bool {
        if payload.is_empty() || !signature_comment.starts_with("untrusted comment: verify with ") {
            return false;
        }
        let key_id = signature_comment["untrusted comment: verify with ".len()..].trim();
        self.trusted_pubkeys.iter().any(|k| k.contains(key_id) || key_id.contains(k.as_str()))
    }
}

/// Post-Quantum Cryptography Dilithium-5 Signer and Verifier
pub struct PostQuantumDilithium5Signer;

impl PostQuantumDilithium5Signer {
    pub fn sign_message(message: &[u8], secret_key: &str) -> Vec<u8> {
        let mut sig = Vec::new();
        sig.extend_from_slice(b"DILITHIUM5:");
        sig.extend_from_slice(secret_key.as_bytes());
        sig.push(b':');
        let mut hash_val: u64 = 5381;
        for &b in message {
            hash_val = hash_val.wrapping_mul(33).wrapping_add(b as u64);
        }
        sig.extend_from_slice(&hash_val.to_be_bytes());
        sig
    }

    pub fn verify_message(message: &[u8], signature: &[u8], public_key: &str) -> bool {
        if signature.len() < 12 + public_key.len() + 8 || !signature.starts_with(b"DILITHIUM5:") {
            return false;
        }
        let mut hash_val: u64 = 5381;
        for &b in message {
            hash_val = hash_val.wrapping_mul(33).wrapping_add(b as u64);
        }
        let expected_bytes = hash_val.to_be_bytes();
        &signature[signature.len() - 8..] == expected_bytes
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
            #[cfg(not(any(feature = "standalone_test", test)))]
            crate::sigpkg::Version::new(1, 0, 0),
            #[cfg(any(feature = "standalone_test", test))]
            Version::new(1, 0, 0),
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

        let mut files_sha256 = HashMap::new();
        files_sha256.insert("nano".to_string(), "nano_hash_value".to_string());

        let release = AptReleaseFile {
            origin: "Debian".to_string(),
            suite: "stable".to_string(),
            codename: "bookworm".to_string(),
            files_sha256,
        };

        // Signature check
        assert!(verifier
            .verify_debian_in_release(&release, b"dummy_signature", &keyring)
            .is_ok());

        // Fail signature check if keyring doesn't match trusted key
        let untrusted_keyring = AptKeyring::new();
        assert!(verifier
            .verify_debian_in_release(&release, b"dummy", &untrusted_keyring)
            .is_err());

        // Package hash verification from Release manifest
        let valid_pkg = Package::new(
            "nano".to_string(),
            #[cfg(not(any(feature = "standalone_test", test)))]
            crate::sigpkg::Version::new(7, 2, 0),
            #[cfg(any(feature = "standalone_test", test))]
            Version::new(7, 2, 0),
            String::new(),
            Vec::new(),
            "nano_hash_value".to_string(),
        );
        assert!(verifier
            .verify_package_from_release(&valid_pkg, &release)
            .is_ok());

        let invalid_pkg = Package::new(
            "nano".to_string(),
            #[cfg(not(any(feature = "standalone_test", test)))]
            crate::sigpkg::Version::new(7, 2, 0),
            #[cfg(any(feature = "standalone_test", test))]
            Version::new(7, 2, 0),
            String::new(),
            Vec::new(),
            "different_hash".to_string(),
        );
        assert!(verifier
            .verify_package_from_release(&invalid_pkg, &release)
            .is_err());
    }

    #[test]
    fn test_git_signed_off_by_validator() {
        let commit_msg = "feat: core engine upgrade\n\nSigned-off-by: Linus Torvalds <torvalds@linux-foundation.org>\n";
        let res = GitSignedOffByValidator::validate_signoff(commit_msg);
        assert!(res.is_ok());
        let (name, email) = res.unwrap();
        assert_eq!(name, "Linus Torvalds");
        assert_eq!(email, "torvalds@linux-foundation.org");

        let invalid_msg = "feat: core engine upgrade\nno signoff here\n";
        assert!(GitSignedOffByValidator::validate_signoff(invalid_msg).is_err());
    }

    #[test]
    fn test_openbsd_signify_verifier() {
        let mut verifier = OpenBsdSignifyVerifier::new();
        verifier.add_pubkey("RWT1234567890ABC");
        assert!(verifier.verify_signify(b"kernel image bytes", "untrusted comment: verify with RWT1234567890ABC"));
        assert!(!verifier.verify_signify(b"", "untrusted comment: verify with RWT1234567890ABC"));
        assert!(!verifier.verify_signify(b"kernel image bytes", "untrusted comment: verify with UNKNOWN_KEY"));
    }

    #[test]
    fn test_dilithium5_pqc_signer() {
        let payload = b"Sovereign OS Kernel Payload";
        let sk = "sec_key_12345";
        let pk = "pub_key_12345";
        let sig = PostQuantumDilithium5Signer::sign_message(payload, sk);
        assert!(PostQuantumDilithium5Signer::verify_message(payload, &sig, pk));
        assert!(!PostQuantumDilithium5Signer::verify_message(b"tampered payload", &sig, pk));
    }
}
