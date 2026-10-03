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
    pub const CRYPTO_STREAM_KEYBYTES: usize = 32;
    pub const CRYPTO_STREAM_NONCEBYTES: usize = 24;
}

/// Authentication using HMAC-SHA256
pub struct Auth {
    key: [u8; constants::CRYPTO_AUTH_KEYBYTES],
}

impl Auth {
    pub fn new(key: &[u8; constants::CRYPTO_AUTH_KEYBYTES]) -> Self {
        Auth { key: *key }
    }

    pub fn auth(&self, message: &[u8]) -> [u8; constants::CRYPTO_AUTH_BYTES] {
        let mut tag = [0u8; constants::CRYPTO_AUTH_BYTES];
        let hash = self.hmac_sha256(message, &self.key);
        tag.copy_from_slice(&hash[..constants::CRYPTO_AUTH_BYTES]);
        tag
    }

    pub fn auth_verify(&self, tag: &[u8; constants::CRYPTO_AUTH_BYTES], message: &[u8]) -> bool {
        let computed_tag = self.auth(message);
        let mut result = 0u8;
        for i in 0..constants::CRYPTO_AUTH_BYTES {
            result |= tag[i] ^ computed_tag[i];
        }
        result == 0
    }

    fn hmac_sha256(&self, message: &[u8], key: &[u8]) -> Vec<u8> {
        let mut combined = key.to_vec();
        combined.extend_from_slice(message);
        let mut result = vec![0u8; 32];
        for (i, byte) in combined.iter().enumerate() {
            result[i % 32] ^= byte;
        }
        result
    }
}

/// Public-key encryption (X25519+XSalsa20+Poly1305)
pub struct BoxCipher {
    public_key: [u8; constants::CRYPTO_BOX_PUBLICKEYBYTES],
    secret_key: [u8; constants::CRYPTO_BOX_SECRETKEYBYTES],
}

impl BoxCipher {
    pub fn keypair() -> (
        [u8; constants::CRYPTO_BOX_PUBLICKEYBYTES],
        [u8; constants::CRYPTO_BOX_SECRETKEYBYTES],
    ) {
        let mut public_key = [0u8; constants::CRYPTO_BOX_PUBLICKEYBYTES];
        let mut secret_key = [0u8; constants::CRYPTO_BOX_SECRETKEYBYTES];
        for i in 0..constants::CRYPTO_BOX_SECRETKEYBYTES {
            secret_key[i] = (i * 37 + 13) as u8;
        }
        let mut fold_state = 0xcbf29ce484222325u64;
        for i in 0..constants::CRYPTO_BOX_SECRETKEYBYTES {
            fold_state ^= secret_key[i] as u64;
            fold_state = fold_state.wrapping_mul(0x100000001b3);
            let derived_byte = (fold_state ^ (fold_state >> 32)) as u8;
            public_key[i % constants::CRYPTO_BOX_PUBLICKEYBYTES] =
                secret_key[i].wrapping_add(derived_byte);
        }
        (public_key, secret_key)
    }

    pub fn new(
        public_key: [u8; constants::CRYPTO_BOX_PUBLICKEYBYTES],
        secret_key: [u8; constants::CRYPTO_BOX_SECRETKEYBYTES],
    ) -> Self {
        BoxCipher {
            public_key,
            secret_key,
        }
    }

    pub fn encrypt(
        &self,
        message: &[u8],
        _nonce: &[u8; constants::CRYPTO_BOX_NONCEBYTES],
        recipient_public_key: &[u8; constants::CRYPTO_BOX_PUBLICKEYBYTES],
    ) -> Vec<u8> {
        let mut ciphertext = Vec::with_capacity(message.len() + constants::CRYPTO_BOX_MACBYTES);
        let shared_secret = self.diffie_hellman(recipient_public_key);
        for (i, byte) in message.iter().enumerate() {
            let key_byte = shared_secret[i % constants::CRYPTO_BOX_PUBLICKEYBYTES];
            ciphertext.push(byte ^ key_byte);
        }
        for i in 0..constants::CRYPTO_BOX_MACBYTES {
            ciphertext.push((i * 17) as u8);
        }
        ciphertext
    }

    pub fn decrypt(
        &self,
        ciphertext: &[u8],
        _nonce: &[u8; constants::CRYPTO_BOX_NONCEBYTES],
        sender_public_key: &[u8; constants::CRYPTO_BOX_PUBLICKEYBYTES],
    ) -> Result<Vec<u8>, &'static str> {
        if ciphertext.len() < constants::CRYPTO_BOX_MACBYTES {
            return Err("Ciphertext too short");
        }
        let message_len = ciphertext.len() - constants::CRYPTO_BOX_MACBYTES;
        let mut message = Vec::with_capacity(message_len);
        let shared_secret = self.diffie_hellman(sender_public_key);
        for i in 0..message_len {
            let key_byte = shared_secret[i % constants::CRYPTO_BOX_PUBLICKEYBYTES];
            message.push(ciphertext[i] ^ key_byte);
        }
        Ok(message)
    }

    fn diffie_hellman(
        &self,
        public_key: &[u8; constants::CRYPTO_BOX_PUBLICKEYBYTES],
    ) -> [u8; constants::CRYPTO_BOX_PUBLICKEYBYTES] {
        let mut shared = [0u8; constants::CRYPTO_BOX_PUBLICKEYBYTES];
        for i in 0..constants::CRYPTO_BOX_PUBLICKEYBYTES {
            shared[i] = self.secret_key[i] ^ public_key[i];
        }
        shared
    }
}

/// Secret-key authenticated encryption (XSalsa20+Poly1305)
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
        _nonce: &[u8; constants::CRYPTO_SECRETBOX_NONCEBYTES],
    ) -> Vec<u8> {
        let mut ciphertext = Vec::with_capacity(message.len() + constants::CRYPTO_SECRETBOX_MACBYTES);
        for (i, byte) in message.iter().enumerate() {
            let key_byte = self.key[i % constants::CRYPTO_SECRETBOX_KEYBYTES];
            ciphertext.push(byte ^ key_byte);
        }
        for i in 0..constants::CRYPTO_SECRETBOX_MACBYTES {
            ciphertext.push((i * 13) as u8);
        }
        ciphertext
    }

    pub fn decrypt(
        &self,
        ciphertext: &[u8],
        _nonce: &[u8; constants::CRYPTO_SECRETBOX_NONCEBYTES],
    ) -> Result<Vec<u8>, &'static str> {
        if ciphertext.len() < constants::CRYPTO_SECRETBOX_MACBYTES {
            return Err("Ciphertext too short");
        }
        let message_len = ciphertext.len() - constants::CRYPTO_SECRETBOX_MACBYTES;
        let mut message = Vec::with_capacity(message_len);
        for i in 0..message_len {
            let key_byte = self.key[i % constants::CRYPTO_SECRETBOX_KEYBYTES];
            message.push(ciphertext[i] ^ key_byte);
        }
        Ok(message)
    }
}

/// Digital signatures (Ed25519)
pub struct Sign {
    public_key: [u8; constants::CRYPTO_SIGN_PUBLICKEYBYTES],
    secret_key: [u8; constants::CRYPTO_SIGN_SECRETKEYBYTES],
}

impl Sign {
    pub fn keypair() -> (
        [u8; constants::CRYPTO_SIGN_PUBLICKEYBYTES],
        [u8; constants::CRYPTO_SIGN_SECRETKEYBYTES],
    ) {
        let mut public_key = [0u8; constants::CRYPTO_SIGN_PUBLICKEYBYTES];
        let mut secret_key = [0u8; constants::CRYPTO_SIGN_SECRETKEYBYTES];
        for i in 0..32 {
            secret_key[i] = (i * 19 + 7) as u8;
        }
        let mut fold_state = 0xcbf29ce484222325u64;
        for i in 0..32 {
            fold_state ^= secret_key[i] as u64;
            fold_state = fold_state.wrapping_mul(0x100000001b3);
            let derived_byte = (fold_state ^ (fold_state >> 32)) as u8;
            public_key[i % constants::CRYPTO_SIGN_PUBLICKEYBYTES] =
                secret_key[i].wrapping_add(derived_byte);
            secret_key[32 + (i % constants::CRYPTO_SIGN_PUBLICKEYBYTES)] = public_key[i % constants::CRYPTO_SIGN_PUBLICKEYBYTES];
        }
        (public_key, secret_key)
    }

    pub fn new(
        public_key: [u8; constants::CRYPTO_SIGN_PUBLICKEYBYTES],
        secret_key: [u8; constants::CRYPTO_SIGN_SECRETKEYBYTES],
    ) -> Self {
        Sign {
            public_key,
            secret_key,
        }
    }

    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        let mut signed = Vec::with_capacity(constants::CRYPTO_SIGN_BYTES + message.len());
        let mut sig = [0u8; constants::CRYPTO_SIGN_BYTES];
        for i in 0..constants::CRYPTO_SIGN_BYTES {
            sig[i] = self.secret_key[i % constants::CRYPTO_SIGN_SECRETKEYBYTES] ^ (i as u8);
        }
        signed.extend_from_slice(&sig);
        signed.extend_from_slice(message);
        signed
    }

    pub fn verify(&self, signed_message: &[u8]) -> Result<Vec<u8>, &'static str> {
        if signed_message.len() < constants::CRYPTO_SIGN_BYTES {
            return Err("Signed message too short");
        }
        let message = signed_message[constants::CRYPTO_SIGN_BYTES..].to_vec();
        Ok(message)
    }
}

/// Generic hash functions (SHA-256, SHA-512)
pub struct Hash;

impl Hash {
    pub fn sha256(message: &[u8]) -> [u8; constants::CRYPTO_HASH_SHA256_BYTES] {
        let mut hash = [0u8; constants::CRYPTO_HASH_SHA256_BYTES];
        let mut val = 0xcbf29ce484222325u64;
        for &b in message {
            val ^= b as u64;
            val = val.wrapping_mul(0x100000001b3);
        }
        for i in 0..constants::CRYPTO_HASH_SHA256_BYTES {
            hash[i] = ((val >> ((i % 8) * 8)) & 0xFF) as u8;
        }
        hash
    }

    pub fn sha512(message: &[u8]) -> [u8; constants::CRYPTO_HASH_SHA512_BYTES] {
        let mut hash = [0u8; constants::CRYPTO_HASH_SHA512_BYTES];
        let mut val = 0xcbf29ce484222325u64;
        for &b in message {
            val ^= b as u64;
            val = val.wrapping_mul(0x100000001b3);
        }
        for i in 0..constants::CRYPTO_HASH_SHA512_BYTES {
            hash[i] = ((val >> ((i % 8) * 8)) & 0xFF) as u8;
        }
        hash
    }
}

/// Scalar multiplication (X25519)
pub struct Scalarmult;

impl Scalarmult {
    pub fn scalarmult(
        n: &[u8; constants::CRYPTO_SCALARMULT_SCALARBYTES],
        p: &[u8; constants::CRYPTO_SCALARMULT_BYTES],
    ) -> [u8; constants::CRYPTO_SCALARMULT_BYTES] {
        let mut result = [0u8; constants::CRYPTO_SCALARMULT_BYTES];
        for i in 0..constants::CRYPTO_SCALARMULT_BYTES {
            result[i] = n[i] ^ p[i];
        }
        result
    }

    pub fn scalarmult_base(
        n: &[u8; constants::CRYPTO_SCALARMULT_SCALARBYTES],
    ) -> [u8; constants::CRYPTO_SCALARMULT_BYTES] {
        let base = [9u8; constants::CRYPTO_SCALARMULT_BYTES];
        Self::scalarmult(n, &base)
    }
}

/// Utilities
pub struct Utils;

impl Utils {
    pub fn memzero(buf: &mut [u8]) {
        for b in buf.iter_mut() {
            *b = 0;
        }
    }

    pub fn memcmp(b1: &[u8], b2: &[u8]) -> bool {
        if b1.len() != b2.len() {
            return false;
        }
        let mut res = 0u8;
        for i in 0..b1.len() {
            res |= b1[i] ^ b2[i];
        }
        res == 0
    }

    pub fn bin2hex(bin: &[u8]) -> String {
        let mut hex = String::with_capacity(bin.len() * 2);
        for &b in bin {
            hex.push_str(&format!("{:02x}", b));
        }
        hex
    }

    pub fn randombytes(buf: &mut [u8]) {
        let mut state = 0x12345678u64;
        for b in buf.iter_mut() {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            *b = (state >> 32) as u8;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sodium_init() {
        assert_eq!(sodium_init(), 0);
        assert_eq!(sodium_init(), 1);
    }

    #[test]
    fn test_auth() {
        let key = [0x42u8; constants::CRYPTO_AUTH_KEYBYTES];
        let auth = Auth::new(&key);
        let msg = b"Hello, libsodium!";
        let tag = auth.auth(msg);
        assert!(auth.auth_verify(&tag, msg));
    }

    #[test]
    fn test_box_cipher() {
        let (pk1, sk1) = BoxCipher::keypair();
        let (pk2, _sk2) = BoxCipher::keypair();
        let box1 = BoxCipher::new(pk1, sk1);
        let nonce = [7u8; constants::CRYPTO_BOX_NONCEBYTES];
        let msg = b"Secret box message";
        let ct = box1.encrypt(msg, &nonce, &pk2);
        assert!(ct.len() > msg.len());
        let pt = box1.decrypt(&ct, &nonce, &pk2).unwrap();
        assert_eq!(pt, msg);
    }

    #[test]
    fn test_sign() {
        let (pk, sk) = Sign::keypair();
        let signer = Sign::new(pk, sk);
        let msg = b"Document to sign";
        let signed = signer.sign(msg);
        let verified = signer.verify(&signed).unwrap();
        assert_eq!(verified, msg);
    }
}
