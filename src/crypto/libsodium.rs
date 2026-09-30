#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(dead_code)]
//! libsodium Compatibility Layer for SigmaOS
//!
//! This module is an API-shape prototype only. Its primitives are not
//! cryptographically secure and must not be used for confidentiality,
//! authentication, signatures, key generation, or random-number generation.
//! Do not use this module in production. Production callers need a separately
//! integrated, audited cryptographic provider.
use std::vec::Vec;

pub type c_int = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CryptoUnavailable {
    ProviderNotIntegrated,
}

/// libsodium is not integrated. Return a negative status rather than claiming
/// that the compatibility API initialized a cryptographic provider.
pub fn sodium_init() -> c_int {
    -1
}

/// Constants for cryptographic operations
pub mod constants {
    /// Size of crypto_auth_keybytes
    pub const CRYPTO_AUTH_KEYBYTES: usize = 32;
    /// Size of crypto_auth_bytes
    pub const CRYPTO_AUTH_BYTES: usize = 16;

    /// Size of crypto_box_publickeybytes
    pub const CRYPTO_BOX_PUBLICKEYBYTES: usize = 32;
    /// Size of crypto_box_secretkeybytes
    pub const CRYPTO_BOX_SECRETKEYBYTES: usize = 32;
    /// Size of crypto_box_noncebytes
    pub const CRYPTO_BOX_NONCEBYTES: usize = 24;
    /// Size of crypto_box_macbytes
    pub const CRYPTO_BOX_MACBYTES: usize = 16;

    /// Size of crypto_secretbox_keybytes
    pub const CRYPTO_SECRETBOX_KEYBYTES: usize = 32;
    /// Size of crypto_secretbox_noncebytes
    pub const CRYPTO_SECRETBOX_NONCEBYTES: usize = 24;
    /// Size of crypto_secretbox_macbytes
    pub const CRYPTO_SECRETBOX_MACBYTES: usize = 16;

    /// Size of crypto_sign_publickeybytes
    pub const CRYPTO_SIGN_PUBLICKEYBYTES: usize = 32;
    /// Size of crypto_sign_secretkeybytes
    pub const CRYPTO_SIGN_SECRETKEYBYTES: usize = 64;
    /// Size of crypto_sign_bytes
    pub const CRYPTO_SIGN_BYTES: usize = 64;

    /// Size of crypto_hash_sha256_bytes
    pub const CRYPTO_HASH_SHA256_BYTES: usize = 32;
    /// Size of crypto_hash_sha512_bytes
    pub const CRYPTO_HASH_SHA512_BYTES: usize = 64;

    /// Size of crypto_scalarmult_bytes
    pub const CRYPTO_SCALARMULT_BYTES: usize = 32;
    /// Size of crypto_scalarmult_scalarbytes
    pub const CRYPTO_SCALARMULT_SCALARBYTES: usize = 32;

    /// Size of crypto_stream_keybytes
    pub const CRYPTO_STREAM_KEYBYTES: usize = 32;
    /// Size of crypto_stream_noncebytes
    pub const CRYPTO_STREAM_NONCEBYTES: usize = 24;
}

/// Authentication using HMAC-SHA256
pub struct Auth;

impl Auth {
    /// Create a new authentication context with the given key
    pub fn new(_key: &[u8; constants::CRYPTO_AUTH_KEYBYTES]) -> Self {
        Auth
    }

    /// Generate authentication tag for a message
    pub fn auth(
        &self,
        _message: &[u8],
    ) -> Result<[u8; constants::CRYPTO_AUTH_BYTES], CryptoUnavailable> {
        Err(CryptoUnavailable::ProviderNotIntegrated)
    }

    /// Verify authentication tag for a message
    pub fn auth_verify(
        &self,
        _tag: &[u8; constants::CRYPTO_AUTH_BYTES],
        _message: &[u8],
    ) -> Result<bool, CryptoUnavailable> {
        Err(CryptoUnavailable::ProviderNotIntegrated)
    }
}

/// Public-key encryption (X25519+XSalsa20+Poly1305)
pub struct BoxCipher;

impl BoxCipher {
    /// Generate a new keypair
    pub fn keypair() -> Result<
        (
            [u8; constants::CRYPTO_BOX_PUBLICKEYBYTES],
            [u8; constants::CRYPTO_BOX_SECRETKEYBYTES],
        ),
        CryptoUnavailable,
    > {
        Err(CryptoUnavailable::ProviderNotIntegrated)
    }

    /// Create a new box cipher with existing keys
    pub fn new(
        _public_key: [u8; constants::CRYPTO_BOX_PUBLICKEYBYTES],
        _secret_key: [u8; constants::CRYPTO_BOX_SECRETKEYBYTES],
    ) -> Self {
        BoxCipher
    }

    /// Encrypt a message
    pub fn encrypt(
        &self,
        message: &[u8],
        nonce: &[u8; constants::CRYPTO_BOX_NONCEBYTES],
        recipient_public_key: &[u8; constants::CRYPTO_BOX_PUBLICKEYBYTES],
    ) -> Result<Vec<u8>, CryptoUnavailable> {
        let _ = (message, nonce, recipient_public_key);
        Err(CryptoUnavailable::ProviderNotIntegrated)
    }

    /// Decrypt a message
    pub fn decrypt(
        &self,
        ciphertext: &[u8],
        nonce: &[u8; constants::CRYPTO_BOX_NONCEBYTES],
        sender_public_key: &[u8; constants::CRYPTO_BOX_PUBLICKEYBYTES],
    ) -> Result<Vec<u8>, CryptoUnavailable> {
        let _ = (ciphertext, nonce, sender_public_key);
        Err(CryptoUnavailable::ProviderNotIntegrated)
    }
}

/// Secret-key encryption (XSalsa20+Poly1305)
pub struct SecretBox;

impl SecretBox {
    /// Create a new secret box with the given key
    pub fn new(_key: &[u8; constants::CRYPTO_SECRETBOX_KEYBYTES]) -> Self {
        SecretBox
    }

    /// Encrypt a message
    pub fn encrypt(
        &self,
        message: &[u8],
        nonce: &[u8; constants::CRYPTO_SECRETBOX_NONCEBYTES],
    ) -> Result<Vec<u8>, CryptoUnavailable> {
        let _ = (message, nonce);
        Err(CryptoUnavailable::ProviderNotIntegrated)
    }

    /// Decrypt a message
    pub fn decrypt(
        &self,
        ciphertext: &[u8],
        nonce: &[u8; constants::CRYPTO_SECRETBOX_NONCEBYTES],
    ) -> Result<Vec<u8>, CryptoUnavailable> {
        let _ = (ciphertext, nonce);
        Err(CryptoUnavailable::ProviderNotIntegrated)
    }
}

/// Digital signatures (Ed25519)
pub struct Sign;

impl Sign {
    /// Generate a new signing keypair
    pub fn keypair() -> Result<
        (
            [u8; constants::CRYPTO_SIGN_PUBLICKEYBYTES],
            [u8; constants::CRYPTO_SIGN_SECRETKEYBYTES],
        ),
        CryptoUnavailable,
    > {
        Err(CryptoUnavailable::ProviderNotIntegrated)
    }

    /// Create a new signer with existing keys
    pub fn new(
        _public_key: [u8; constants::CRYPTO_SIGN_PUBLICKEYBYTES],
        _secret_key: [u8; constants::CRYPTO_SIGN_SECRETKEYBYTES],
    ) -> Self {
        Sign
    }

    /// Sign a message
    pub fn sign(
        &self,
        _message: &[u8],
    ) -> Result<[u8; constants::CRYPTO_SIGN_BYTES], CryptoUnavailable> {
        Err(CryptoUnavailable::ProviderNotIntegrated)
    }

    /// Verify a signature
    pub fn verify(
        &self,
        _signature: &[u8; constants::CRYPTO_SIGN_BYTES],
        _message: &[u8],
    ) -> Result<bool, CryptoUnavailable> {
        Err(CryptoUnavailable::ProviderNotIntegrated)
    }
}

/// Hash functions
pub struct Hash;

impl Hash {
    /// SHA256 hash
    pub fn sha256(
        _message: &[u8],
    ) -> Result<[u8; constants::CRYPTO_HASH_SHA256_BYTES], CryptoUnavailable> {
        Err(CryptoUnavailable::ProviderNotIntegrated)
    }

    /// SHA512 hash
    pub fn sha512(
        _message: &[u8],
    ) -> Result<[u8; constants::CRYPTO_HASH_SHA512_BYTES], CryptoUnavailable> {
        Err(CryptoUnavailable::ProviderNotIntegrated)
    }
}

/// Scalar multiplication (Curve25519)
pub struct ScalarMult;

impl ScalarMult {
    /// Scalar multiplication
    pub fn scalar_mult(
        _scalar: &[u8; constants::CRYPTO_SCALARMULT_SCALARBYTES],
        _point: &[u8; constants::CRYPTO_SCALARMULT_BYTES],
    ) -> Result<[u8; constants::CRYPTO_SCALARMULT_BYTES], CryptoUnavailable> {
        Err(CryptoUnavailable::ProviderNotIntegrated)
    }

    /// Scalar multiplication base
    pub fn scalar_mult_base(
        _scalar: &[u8; constants::CRYPTO_SCALARMULT_SCALARBYTES],
    ) -> Result<[u8; constants::CRYPTO_SCALARMULT_BYTES], CryptoUnavailable> {
        Err(CryptoUnavailable::ProviderNotIntegrated)
    }
}

/// Stream cipher (XSalsa20)
pub struct Stream;

impl Stream {
    /// Generate stream cipher output
    pub fn stream_xor(
        _output: &mut [u8],
        _input: &[u8],
        _nonce: &[u8; constants::CRYPTO_STREAM_NONCEBYTES],
        _key: &[u8; constants::CRYPTO_STREAM_KEYBYTES],
    ) -> Result<(), CryptoUnavailable> {
        Err(CryptoUnavailable::ProviderNotIntegrated)
    }

    /// Generate stream cipher output (in-place)
    pub fn stream_xor_inplace(
        _data: &mut [u8],
        _nonce: &[u8; constants::CRYPTO_STREAM_NONCEBYTES],
        _key: &[u8; constants::CRYPTO_STREAM_KEYBYTES],
    ) -> Result<(), CryptoUnavailable> {
        Err(CryptoUnavailable::ProviderNotIntegrated)
    }
}

/// Utility functions
pub mod utils {
    /// Compare two byte arrays in constant time
    pub fn memcmp(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }

        let mut result = 0u8;
        for i in 0..a.len() {
            result |= a[i] ^ b[i];
        }

        result == 0
    }

    /// Zero memory securely
    pub fn memzero(data: &mut [u8]) {
        for byte in data.iter_mut() {
            *byte = 0;
        }
    }

    /// Generate random bytes
    pub fn randombytes(_buf: &mut [u8]) -> Result<(), super::CryptoUnavailable> {
        Err(super::CryptoUnavailable::ProviderNotIntegrated)
    }
}

#[cfg(test)]
mod fail_closed_tests {
    use super::{
        constants, utils, Auth, BoxCipher, CryptoUnavailable, Hash, ScalarMult, SecretBox, Sign,
        Stream,
    };

    #[test]
    fn unaudited_cryptographic_operations_never_return_placeholder_results() {
        assert_eq!(super::sodium_init(), -1);
        let key = [1; constants::CRYPTO_AUTH_KEYBYTES];
        let auth = Auth::new(&key);
        assert_eq!(
            auth.auth(b"data"),
            Err(CryptoUnavailable::ProviderNotIntegrated)
        );
        assert_eq!(
            auth.auth_verify(&[0; constants::CRYPTO_AUTH_BYTES], b"data"),
            Err(CryptoUnavailable::ProviderNotIntegrated)
        );
        assert!(BoxCipher::keypair().is_err());
        assert!(Sign::keypair().is_err());
        assert!(Hash::sha256(b"data").is_err());
        assert!(Hash::sha512(b"data").is_err());
        assert!(ScalarMult::scalar_mult(&[1; 32], &[2; 32]).is_err());
        assert!(ScalarMult::scalar_mult_base(&[1; 32]).is_err());

        let nonce = [3; constants::CRYPTO_SECRETBOX_NONCEBYTES];
        let secretbox = SecretBox::new(&[4; constants::CRYPTO_SECRETBOX_KEYBYTES]);
        assert!(secretbox.encrypt(b"data", &nonce).is_err());
        assert!(secretbox.decrypt(b"ciphertext", &nonce).is_err());

        let mut data = [0xA5; 4];
        assert!(Stream::stream_xor_inplace(&mut data, &[0; 24], &[0; 32]).is_err());
        assert_eq!(data, [0xA5; 4]);
        assert!(utils::randombytes(&mut data).is_err());
        assert_eq!(data, [0xA5; 4]);
    }
}
