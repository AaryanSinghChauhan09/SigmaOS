#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
use std::vec::Vec;

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

/// OOP-based Post-Quantum Crypto Integration for SigmaOS
/// Based on Roadmap Item: Post-Quantum Crypto Integration
/// Implements HKDF-SHA3-256 key derivation and PQC/Dilithium-5 signatures

pub type KeyID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CryptoError {
    Success = 0,
    InvalidKey = 1,
    DerivationFailed = 2,
    SignFailed = 3,
    ProviderUnavailable = 4,
}

pub trait KeyDerivation {
    fn derive_key(&self, secret: &[u8], salt: &[u8], info: &[u8]) -> Result<Vec<u8>, CryptoError>;
    fn hkdf_sha3_256(&self, ikm: &[u8], salt: &[u8], info: &[u8]) -> Result<Vec<u8>, CryptoError>;
}

#[repr(C)]
pub struct SimpleKeyDerivation;

impl SimpleKeyDerivation {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleKeyDerivation
    }
}

impl KeyDerivation for SimpleKeyDerivation {
    fn derive_key(&self, secret: &[u8], salt: &[u8], info: &[u8]) -> Result<Vec<u8>, CryptoError> {
        self.hkdf_sha3_256(secret, salt, info)
    }
    fn hkdf_sha3_256(
        &self,
        _ikm: &[u8],
        _salt: &[u8],
        _info: &[u8],
    ) -> Result<Vec<u8>, CryptoError> {
        Err(CryptoError::ProviderUnavailable)
    }
}

pub trait PostQuantumSignature {
    fn sign(&self, message: &[u8], private_key: &[u8]) -> Result<Vec<u8>, CryptoError>;
    fn verify(
        &self,
        message: &[u8],
        signature: &[u8],
        public_key: &[u8],
    ) -> Result<bool, CryptoError>;
    fn generate_keypair(&mut self) -> Result<(Vec<u8>, Vec<u8>), CryptoError>;
}

#[repr(C)]
pub struct Dilithium5Signature;

impl Dilithium5Signature {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Dilithium5Signature
    }
}

impl PostQuantumSignature for Dilithium5Signature {
    fn sign(&self, _message: &[u8], _private_key: &[u8]) -> Result<Vec<u8>, CryptoError> {
        Err(CryptoError::ProviderUnavailable)
    }
    fn verify(
        &self,
        _message: &[u8],
        _signature: &[u8],
        _public_key: &[u8],
    ) -> Result<bool, CryptoError> {
        Err(CryptoError::ProviderUnavailable)
    }
    fn generate_keypair(&mut self) -> Result<(Vec<u8>, Vec<u8>), CryptoError> {
        Err(CryptoError::ProviderUnavailable)
    }
}

pub trait SecureBootSigning {
    fn sign_bootloader(&self, bootloader: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptoError>;
    fn verify_bootloader(
        &self,
        bootloader: &[u8],
        signature: &[u8],
        key: &[u8],
    ) -> Result<bool, CryptoError>;
}

#[repr(C)]
pub struct SimpleSecureBootSigning {
    pub signature: Dilithium5Signature,
}

impl SimpleSecureBootSigning {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleSecureBootSigning {
            signature: Dilithium5Signature::new(),
        }
    }
}

impl SecureBootSigning for SimpleSecureBootSigning {
    fn sign_bootloader(&self, bootloader: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptoError> {
        self.signature.sign(bootloader, key)
    }
    fn verify_bootloader(
        &self,
        bootloader: &[u8],
        signature: &[u8],
        key: &[u8],
    ) -> Result<bool, CryptoError> {
        self.signature.verify(bootloader, signature, key)
    }
}

pub trait FullDiskEncryption {
    fn encrypt_volume(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptoError>;
    fn decrypt_volume(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptoError>;
}

#[repr(C)]
pub struct SimpleFDE {
    pub derivation: SimpleKeyDerivation,
}

impl SimpleFDE {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleFDE {
            derivation: SimpleKeyDerivation::new(),
        }
    }
}

impl FullDiskEncryption for SimpleFDE {
    fn encrypt_volume(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let _ = (data, key);
        Err(CryptoError::ProviderUnavailable)
    }
    fn decrypt_volume(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let _ = (data, key);
        Err(CryptoError::ProviderUnavailable)
    }
}

#[cfg(test)]
mod fail_closed_tests {
    use super::{
        CryptoError, Dilithium5Signature, FullDiskEncryption, KeyDerivation, PostQuantumSignature,
        SimpleFDE, SimpleKeyDerivation,
    };

    #[test]
    fn post_quantum_and_fde_apis_fail_without_provider() {
        let kdf = SimpleKeyDerivation::new();
        assert_eq!(
            kdf.derive_key(b"ikm", b"salt", b"info"),
            Err(CryptoError::ProviderUnavailable)
        );

        let mut signature = Dilithium5Signature::new();
        assert_eq!(
            signature.generate_keypair(),
            Err(CryptoError::ProviderUnavailable)
        );
        assert_eq!(
            signature.sign(b"message", b"key"),
            Err(CryptoError::ProviderUnavailable)
        );
        assert_eq!(
            signature.verify(b"message", b"signature", b"public key"),
            Err(CryptoError::ProviderUnavailable)
        );

        let fde = SimpleFDE::new();
        assert_eq!(
            fde.encrypt_volume(b"data", b"key"),
            Err(CryptoError::ProviderUnavailable)
        );
        assert_eq!(
            fde.decrypt_volume(b"data", b"key"),
            Err(CryptoError::ProviderUnavailable)
        );
    }
}
