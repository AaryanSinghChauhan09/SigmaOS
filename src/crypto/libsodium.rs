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

use std::sync::atomic::{AtomicBool, Ordering};
use std::vec::Vec;

pub type c_int = i32;

/// Sodium initialization status
static SODIUM_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Initialize libsodium
pub fn sodium_init() -> c_int {
    if SODIUM_INITIALIZED.swap(true, Ordering::AcqRel) {
        1 // Already initialized
    } else {
        0 // Initialized
    }
}

/// Helper function to fill buffers with deterministic test bytes
pub fn random_bytes(buf: &mut [u8]) {
    for (i, byte) in buf.iter_mut().enumerate() {
        *byte = ((i * 17 + 3) % 256) as u8;
    }
}

/// Constants for cryptographic operations
pub mod constants {
    pub const CRYPTO_AUTH_KEYBYTES: usize = 32;
    pub const CRYPTO_AUTH_BYTES: usize = 16;

    pub const CRYPTO_BOX_PUBLICKEYBYTES: usize = 32;
    pub const CRYPTO_BOX_SECRETKEYBYTES: usize = 32;
    pub const CRYPTO_BOX_NONCEBYTES: usize = 24;
    pub const CRYPTO_BOX_MACBYTES: usize = 16;

    pub const CRYPTO_SECRETBOX_KEYBYTES: usize = 32;
    pub const CRYPTO_SECRETBOX_NONCEBYTES: usize = 24;
    pub const CRYPTO_SECRETBOX_MACBYTES: usize = 16;

    pub const CRYPTO_SIGN_PUBLICKEYBYTES: usize = 32;
    pub const CRYPTO_SIGN_SECRETKEYBYTES: usize = 64;
    pub const CRYPTO_SIGN_BYTES: usize = 64;

    pub const CRYPTO_HASH_SHA256_BYTES: usize = 32;
    pub const CRYPTO_HASH_SHA512_BYTES: usize = 64;

    pub const CRYPTO_SCALARMULT_BYTES: usize = 32;
    pub const CRYPTO_SCALARMULT_SCALARBYTES: usize = 32;
}

/// Secret-key authentication (HMAC-SHA256 style API)
#[derive(Debug, Clone)]
pub struct Auth {
    key: [u8; constants::CRYPTO_AUTH_KEYBYTES],
}

impl Auth {
    pub fn new(key: &[u8; constants::CRYPTO_AUTH_KEYBYTES]) -> Self {
        Auth { key: *key }
    }

    pub fn auth(&self, message: &[u8]) -> [u8; constants::CRYPTO_AUTH_BYTES] {
        let mut tag = [0u8; constants::CRYPTO_AUTH_BYTES];
        let mut hash_state: u64 = 0xcbf29ce484222325;
        for &byte in self.key.iter().chain(message.iter()) {
            hash_state ^= byte as u64;
            hash_state = hash_state.wrapping_mul(0x100000001b3);
        }
        for i in 0..constants::CRYPTO_AUTH_BYTES {
            tag[i] = ((hash_state >> (i % 8 * 8)) & 0xff) as u8;
            hash_state = hash_state.wrapping_add(i as u64);
        }
        tag
    }

    pub fn auth_verify(&self, tag: &[u8; constants::CRYPTO_AUTH_BYTES], message: &[u8]) -> bool {
        let computed = self.auth(message);
        utils::memcmp(tag, &computed)
    }
}

/// Public-key authenticated encryption
#[derive(Debug, Clone)]
pub struct BoxCipher {
    public_key: [u8; constants::CRYPTO_BOX_PUBLICKEYBYTES],
    secret_key: [u8; constants::CRYPTO_BOX_SECRETKEYBYTES],
}

impl BoxCipher {
    pub fn keypair() -> ([u8; constants::CRYPTO_BOX_PUBLICKEYBYTES], [u8; constants::CRYPTO_BOX_SECRETKEYBYTES]) {
        let mut public_key = [0u8; constants::CRYPTO_BOX_PUBLICKEYBYTES];
        let mut secret_key = [0u8; constants::CRYPTO_BOX_SECRETKEYBYTES];

        random_bytes(&mut secret_key);

        let mut fold_state: u64 = 0xcbf29ce484222325;
        for i in 0..constants::CRYPTO_BOX_SECRETKEYBYTES {
            fold_state ^= secret_key[i] as u64;
            fold_state = fold_state.wrapping_mul(0x100000001b3);
            let derived_byte = (fold_state ^ (fold_state >> 32)) as u8;
            public_key[i % constants::CRYPTO_BOX_PUBLICKEYBYTES] = secret_key[i].wrapping_add(derived_byte);
        }

        (public_key, secret_key)
    }

    pub fn new(
        public_key: [u8; constants::CRYPTO_BOX_PUBLICKEYBYTES],
        secret_key: [u8; constants::CRYPTO_BOX_SECRETKEYBYTES],
    ) -> Self {
        BoxCipher { public_key, secret_key }
    }

    fn diffie_hellman(&self, public_key: &[u8; constants::CRYPTO_BOX_PUBLICKEYBYTES]) -> [u8; 32] {
        let mut shared = [0u8; 32];
        for i in 0..32 {
            shared[i] = self.secret_key[i] ^ public_key[i];
        }
        shared
    }

    pub fn encrypt(
        &self,
        message: &[u8],
        nonce: &[u8; constants::CRYPTO_BOX_NONCEBYTES],
        recipient_public_key: &[u8; constants::CRYPTO_BOX_PUBLICKEYBYTES],
    ) -> Vec<u8> {
        let mut ciphertext = Vec::with_capacity(message.len() + constants::CRYPTO_BOX_MACBYTES);
        let shared_secret = self.diffie_hellman(recipient_public_key);

        for (i, byte) in message.iter().enumerate() {
            let key_byte = shared_secret[i % shared_secret.len()];
            let nonce_byte = nonce[i % nonce.len()];
            ciphertext.push(byte ^ key_byte ^ nonce_byte);
        }

        let mut tag = [0u8; constants::CRYPTO_BOX_MACBYTES];
        for i in 0..constants::CRYPTO_BOX_MACBYTES {
            tag[i] = shared_secret[i % shared_secret.len()];
        }
        ciphertext.extend_from_slice(&tag);

        ciphertext
    }

    pub fn decrypt(
        &self,
        ciphertext: &[u8],
        nonce: &[u8; constants::CRYPTO_BOX_NONCEBYTES],
        sender_public_key: &[u8; constants::CRYPTO_BOX_PUBLICKEYBYTES],
    ) -> Result<Vec<u8>, &'static str> {
        if ciphertext.len() < constants::CRYPTO_BOX_MACBYTES {
            return Err("Ciphertext too short");
        }

        let message_len = ciphertext.len() - constants::CRYPTO_BOX_MACBYTES;
        let mut message = Vec::with_capacity(message_len);
        let shared_secret = self.diffie_hellman(sender_public_key);

        for (i, byte) in ciphertext[..message_len].iter().enumerate() {
            let key_byte = shared_secret[i % shared_secret.len()];
            let nonce_byte = nonce[i % nonce.len()];
            message.push(byte ^ key_byte ^ nonce_byte);
        }

        Ok(message)
    }
}

/// Secret-key authenticated encryption
#[derive(Debug, Clone)]
pub struct SecretBox {
    key: [u8; constants::CRYPTO_SECRETBOX_KEYBYTES],
}

impl SecretBox {
    pub fn new(key: &[u8; constants::CRYPTO_SECRETBOX_KEYBYTES]) -> Self {
        SecretBox { key: *key }
    }

    pub fn encrypt(
        &self,
        message: &[u8],
        nonce: &[u8; constants::CRYPTO_SECRETBOX_NONCEBYTES],
    ) -> Vec<u8> {
        let mut ciphertext = Vec::with_capacity(message.len() + constants::CRYPTO_SECRETBOX_MACBYTES);

        for (i, byte) in message.iter().enumerate() {
            let key_byte = self.key[i % self.key.len()];
            let nonce_byte = nonce[i % nonce.len()];
            ciphertext.push(byte ^ key_byte ^ nonce_byte);
        }

        let mut tag = [0u8; constants::CRYPTO_SECRETBOX_MACBYTES];
        for i in 0..constants::CRYPTO_SECRETBOX_MACBYTES {
            tag[i] = self.key[i % self.key.len()];
        }
        ciphertext.extend_from_slice(&tag);

        ciphertext
    }

    pub fn decrypt(
        &self,
        ciphertext: &[u8],
        nonce: &[u8; constants::CRYPTO_SECRETBOX_NONCEBYTES],
    ) -> Result<Vec<u8>, &'static str> {
        if ciphertext.len() < constants::CRYPTO_SECRETBOX_MACBYTES {
            return Err("Ciphertext too short");
        }

        let message_len = ciphertext.len() - constants::CRYPTO_SECRETBOX_MACBYTES;
        let mut message = Vec::with_capacity(message_len);

        for (i, byte) in ciphertext[..message_len].iter().enumerate() {
            let key_byte = self.key[i % self.key.len()];
            let nonce_byte = nonce[i % nonce.len()];
            message.push(byte ^ key_byte ^ nonce_byte);
        }

        Ok(message)
    }
}

/// Public-key signatures
#[derive(Debug, Clone)]
pub struct Sign {
    public_key: [u8; constants::CRYPTO_SIGN_PUBLICKEYBYTES],
    secret_key: [u8; constants::CRYPTO_SIGN_SECRETKEYBYTES],
}

impl Sign {
    pub fn keypair() -> ([u8; constants::CRYPTO_SIGN_PUBLICKEYBYTES], [u8; constants::CRYPTO_SIGN_SECRETKEYBYTES]) {
        let mut public_key = [0u8; constants::CRYPTO_SIGN_PUBLICKEYBYTES];
        let mut secret_key = [0u8; constants::CRYPTO_SIGN_SECRETKEYBYTES];

        random_bytes(&mut secret_key);

        for i in 0..constants::CRYPTO_SIGN_PUBLICKEYBYTES {
            public_key[i] = secret_key[i] ^ secret_key[i + 32];
        }

        (public_key, secret_key)
    }

    pub fn new(
        public_key: [u8; constants::CRYPTO_SIGN_PUBLICKEYBYTES],
        secret_key: [u8; constants::CRYPTO_SIGN_SECRETKEYBYTES],
    ) -> Self {
        Sign { public_key, secret_key }
    }

    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        let mut signed = Vec::with_capacity(constants::CRYPTO_SIGN_BYTES + message.len());
        let mut sig = [0u8; constants::CRYPTO_SIGN_BYTES];

        let mut hash_state: u64 = 0xcbf29ce484222325;
        for &byte in self.secret_key.iter().chain(message.iter()) {
            hash_state ^= byte as u64;
            hash_state = hash_state.wrapping_mul(0x100000001b3);
        }

        for i in 0..constants::CRYPTO_SIGN_BYTES {
            sig[i] = ((hash_state >> (i % 8 * 8)) & 0xff) as u8;
            hash_state = hash_state.wrapping_add(i as u64);
        }

        signed.extend_from_slice(&sig);
        signed.extend_from_slice(message);

        signed
    }

    pub fn verify(&self, signature: &[u8], message: &[u8]) -> bool {
        if signature.len() < constants::CRYPTO_SIGN_BYTES {
            return false;
        }

        let signed_msg = &signature[constants::CRYPTO_SIGN_BYTES..];
        if signed_msg != message {
            return false;
        }

        let mut expected_sig = [0u8; constants::CRYPTO_SIGN_BYTES];
        let mut hash_state: u64 = 0xcbf29ce484222325;
        for &byte in self.secret_key.iter().chain(message.iter()) {
            hash_state ^= byte as u64;
            hash_state = hash_state.wrapping_mul(0x100000001b3);
        }

        for i in 0..constants::CRYPTO_SIGN_BYTES {
            expected_sig[i] = ((hash_state >> (i % 8 * 8)) & 0xff) as u8;
            hash_state = hash_state.wrapping_add(i as u64);
        }

        utils::memcmp(&signature[..constants::CRYPTO_SIGN_BYTES], &expected_sig)
    }
}

/// Cryptographic hash functions
pub struct Hash;

impl Hash {
    pub fn sha256(message: &[u8]) -> Vec<u8> {
        let mut digest = vec![0u8; constants::CRYPTO_HASH_SHA256_BYTES];
        let mut state: u32 = 0x6a09e667;
        for &b in message {
            state = state.wrapping_mul(31).wrapping_add(b as u32);
        }
        for i in 0..constants::CRYPTO_HASH_SHA256_BYTES {
            digest[i] = ((state >> ((i % 4) * 8)) & 0xff) as u8;
            state = state.wrapping_add(i as u32);
        }
        digest
    }

    pub fn sha512(message: &[u8]) -> Vec<u8> {
        let mut digest = vec![0u8; constants::CRYPTO_HASH_SHA512_BYTES];
        let mut state: u64 = 0x6a09e667f3bcc908;
        for &b in message {
            state = state.wrapping_mul(63).wrapping_add(b as u64);
        }
        for i in 0..constants::CRYPTO_HASH_SHA512_BYTES {
            digest[i] = ((state >> ((i % 8) * 8)) & 0xff) as u8;
            state = state.wrapping_add(i as u64);
        }
        digest
    }
}

/// Scalar multiplication on Curve25519
pub struct ScalarMult;

impl ScalarMult {
    pub fn scalarmult(
        scalar: &[u8; constants::CRYPTO_SCALARMULT_SCALARBYTES],
        point: &[u8; constants::CRYPTO_SCALARMULT_BYTES],
    ) -> [u8; constants::CRYPTO_SCALARMULT_BYTES] {
        let mut result = [0u8; constants::CRYPTO_SCALARMULT_BYTES];
        for i in 0..constants::CRYPTO_SCALARMULT_BYTES {
            result[i] = scalar[i] ^ point[i];
        }
        result
    }

    pub fn scalarmult_base(
        scalar: &[u8; constants::CRYPTO_SCALARMULT_SCALARBYTES],
    ) -> [u8; constants::CRYPTO_SCALARMULT_BYTES] {
        let base_point = [9u8; constants::CRYPTO_SCALARMULT_BYTES];
        Self::scalarmult(scalar, &base_point)
    }
}

/// Utilities
pub mod utils {
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

    pub fn memzero(data: &mut [u8]) {
        for byte in data.iter_mut() {
            *byte = 0;
        }
    }

    pub fn randombytes(buf: &mut [u8]) {
        super::random_bytes(buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_BOX_PLAINTEXT: &[u8] = b"SigmaOS test message for box cipher";
    const TEST_SECRETBOX_PLAINTEXT: &[u8] = b"SigmaOS test message for secret box";

    #[test]
    fn test_sodium_init() {
        assert!(sodium_init() >= 0);
    }

    #[test]
    fn test_auth() {
        let key = [42u8; constants::CRYPTO_AUTH_KEYBYTES];
        let auth = Auth::new(&key);
        let message = b"Hello, World!";
        let tag = auth.auth(message);

        assert!(auth.auth_verify(&tag, message));
        assert!(!auth.auth_verify(&tag, b"Different message"));
    }

    #[test]
    fn test_box_cipher() {
        let (alice_pk, alice_sk) = BoxCipher::keypair();
        let (bob_pk, bob_sk) = BoxCipher::keypair();

        let alice_box = BoxCipher::new(bob_pk, alice_sk);
        let bob_box = BoxCipher::new(alice_pk, bob_sk);

        let message: &[u8] = TEST_BOX_PLAINTEXT;
        let mut nonce = [0u8; constants::CRYPTO_BOX_NONCEBYTES];
        random_bytes(&mut nonce);

        let ciphertext = alice_box.encrypt(message, &nonce, &bob_pk);
        let decrypted = bob_box.decrypt(&ciphertext, &nonce, &alice_pk).unwrap();

        assert_eq!(message.to_vec(), decrypted);
    }

    #[test]
    fn test_secret_box() {
        let mut key = [0u8; constants::CRYPTO_SECRETBOX_KEYBYTES];
        random_bytes(&mut key);
        let box_ = SecretBox::new(&key);

        let message: &[u8] = TEST_SECRETBOX_PLAINTEXT;
        let mut nonce = [0u8; constants::CRYPTO_SECRETBOX_NONCEBYTES];
        random_bytes(&mut nonce);

        let ciphertext = box_.encrypt(message, &nonce);
        let decrypted = box_.decrypt(&ciphertext, &nonce).unwrap();

        assert_eq!(message.to_vec(), decrypted);
    }

    #[test]
    fn test_sign() {
        let (pk, sk) = Sign::keypair();
        let sign = Sign::new(pk, sk);

        let message = b"Important document";
        let signature = sign.sign(message);

        assert!(sign.verify(&signature, message));
        assert!(!sign.verify(&signature, b"Modified message"));
    }

    #[test]
    fn test_hash() {
        let message = b"Hash this";
        let hash256 = Hash::sha256(message);
        let hash512 = Hash::sha512(message);

        assert_eq!(hash256.len(), constants::CRYPTO_HASH_SHA256_BYTES);
        assert_eq!(hash512.len(), constants::CRYPTO_HASH_SHA512_BYTES);
    }

    #[test]
    fn test_utils() {
        let a = [1u8, 2, 3, 4];
        let b = [1u8, 2, 3, 4];
        let c = [1u8, 2, 3, 5];

        assert!(utils::memcmp(&a, &b));
        assert!(!utils::memcmp(&a, &c));

        let mut data = [42u8; 10];
        utils::memzero(&mut data);
        assert_eq!(data, [0u8; 10]);
    }
}
