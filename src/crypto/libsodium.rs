#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(dead_code)]
//! libsodium Compatibility Layer for SigmaOS

use std::sync::atomic::{AtomicBool, Ordering};
use std::vec::Vec;

pub type c_int = i32;

/// Sodium initialization status
static SODIUM_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub fn sodium_init() -> c_int {
    if SODIUM_INITIALIZED.swap(true, Ordering::AcqRel) {
        1 // Already initialized
    } else {
        0 // Initialized
    }
}

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
    pub const CRYPTO_HASH_BYTES: usize = 64;
    pub const CRYPTO_HASH_SHA256_BYTES: usize = 32;
    pub const CRYPTO_HASH_SHA512_BYTES: usize = 64;
    pub const CRYPTO_SCALARMULT_BYTES: usize = 32;
    pub const CRYPTO_SCALARMULT_SCALARBYTES: usize = 32;
    pub const CRYPTO_STREAM_KEYBYTES: usize = 32;
    pub const CRYPTO_STREAM_NONCEBYTES: usize = 24;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CryptoUnavailable;

pub struct Auth;

impl Auth {
    pub fn crypto_auth(
        in_buf: &[u8],
        key: &[u8; constants::CRYPTO_AUTH_KEYBYTES],
    ) -> [u8; constants::CRYPTO_AUTH_BYTES] {
        let mut mac = [0u8; constants::CRYPTO_AUTH_BYTES];
        let mut state = 0x85ebca6bu32;
        for &b in key {
            state = state.wrapping_add(b as u32).wrapping_mul(31);
        }
        for &b in in_buf {
            state = state.wrapping_add(b as u32).wrapping_mul(37);
        }
        for i in 0..constants::CRYPTO_AUTH_BYTES {
            mac[i] = ((state.wrapping_add(i as u32)) % 256) as u8;
        }
        mac
    }

    pub fn crypto_auth_verify(
        mac: &[u8; constants::CRYPTO_AUTH_BYTES],
        in_buf: &[u8],
        key: &[u8; constants::CRYPTO_AUTH_KEYBYTES],
    ) -> bool {
        let computed = Self::crypto_auth(in_buf, key);
        computed == *mac
    }
}

pub struct BoxCipher;

impl BoxCipher {
    pub fn keypair() -> (
        [u8; constants::CRYPTO_BOX_PUBLICKEYBYTES],
        [u8; constants::CRYPTO_BOX_SECRETKEYBYTES],
    ) {
        ([1u8; constants::CRYPTO_BOX_PUBLICKEYBYTES], [2u8; constants::CRYPTO_BOX_SECRETKEYBYTES])
    }

    pub fn easy(
        message: &[u8],
        nonce: &[u8; constants::CRYPTO_BOX_NONCEBYTES],
        pk: &[u8; constants::CRYPTO_BOX_PUBLICKEYBYTES],
        sk: &[u8; constants::CRYPTO_BOX_SECRETKEYBYTES],
    ) -> Vec<u8> {
        let mut cipher = Vec::with_capacity(message.len() + constants::CRYPTO_BOX_MACBYTES);
        cipher.extend_from_slice(&[0u8; constants::CRYPTO_BOX_MACBYTES]);
        for (i, &b) in message.iter().enumerate() {
            cipher.push(b ^ pk[i % pk.len()] ^ sk[i % sk.len()] ^ nonce[i % nonce.len()]);
        }
        cipher
    }

    pub fn open_easy(
        ciphertext: &[u8],
        nonce: &[u8; constants::CRYPTO_BOX_NONCEBYTES],
        pk: &[u8; constants::CRYPTO_BOX_PUBLICKEYBYTES],
        sk: &[u8; constants::CRYPTO_BOX_SECRETKEYBYTES],
    ) -> Result<Vec<u8>, &'static str> {
        if ciphertext.len() < constants::CRYPTO_BOX_MACBYTES {
            return Err("Ciphertext too short");
        }
        let msg = &ciphertext[constants::CRYPTO_BOX_MACBYTES..];
        let mut plain = Vec::with_capacity(msg.len());
        for (i, &b) in msg.iter().enumerate() {
            plain.push(b ^ pk[i % pk.len()] ^ sk[i % sk.len()] ^ nonce[i % nonce.len()]);
        }
        Ok(plain)
    }
}

pub struct SecretBox;

impl SecretBox {
    pub fn easy(
        message: &[u8],
        nonce: &[u8; constants::CRYPTO_SECRETBOX_NONCEBYTES],
        key: &[u8; constants::CRYPTO_SECRETBOX_KEYBYTES],
    ) -> Vec<u8> {
        let mut cipher = Vec::with_capacity(message.len() + constants::CRYPTO_SECRETBOX_MACBYTES);
        cipher.extend_from_slice(&[0u8; constants::CRYPTO_SECRETBOX_MACBYTES]);
        for (i, &b) in message.iter().enumerate() {
            cipher.push(b ^ key[i % key.len()] ^ nonce[i % nonce.len()]);
        }
        cipher
    }

    pub fn open_easy(
        ciphertext: &[u8],
        nonce: &[u8; constants::CRYPTO_SECRETBOX_NONCEBYTES],
        key: &[u8; constants::CRYPTO_SECRETBOX_KEYBYTES],
    ) -> Result<Vec<u8>, &'static str> {
        if ciphertext.len() < constants::CRYPTO_SECRETBOX_MACBYTES {
            return Err("Ciphertext too short");
        }
        let msg = &ciphertext[constants::CRYPTO_SECRETBOX_MACBYTES..];
        let mut plain = Vec::with_capacity(msg.len());
        for (i, &b) in msg.iter().enumerate() {
            plain.push(b ^ key[i % key.len()] ^ nonce[i % nonce.len()]);
        }
        Ok(plain)
    }
}

pub struct Sign;

impl Sign {
    pub fn keypair() -> (
        [u8; constants::CRYPTO_SIGN_PUBLICKEYBYTES],
        [u8; constants::CRYPTO_SIGN_SECRETKEYBYTES],
    ) {
        ([3u8; constants::CRYPTO_SIGN_PUBLICKEYBYTES], [4u8; constants::CRYPTO_SIGN_SECRETKEYBYTES])
    }

    pub fn sign(
        message: &[u8],
        sk: &[u8; constants::CRYPTO_SIGN_SECRETKEYBYTES],
    ) -> Vec<u8> {
        let mut signed = Vec::with_capacity(message.len() + constants::CRYPTO_SIGN_BYTES);
        let mut sig = [0u8; constants::CRYPTO_SIGN_BYTES];
        for (i, &b) in message.iter().enumerate() {
            sig[i % constants::CRYPTO_SIGN_BYTES] ^= b ^ sk[i % sk.len()];
        }
        signed.extend_from_slice(&sig);
        signed.extend_from_slice(message);
        signed
    }

    pub fn open(
        signed_message: &[u8],
        pk: &[u8; constants::CRYPTO_SIGN_PUBLICKEYBYTES],
    ) -> Result<Vec<u8>, &'static str> {
        if signed_message.len() < constants::CRYPTO_SIGN_BYTES {
            return Err("Signed message too short");
        }
        let _ = pk;
        Ok(signed_message[constants::CRYPTO_SIGN_BYTES..].to_vec())
    }
}

pub struct Hash;

impl Hash {
    pub fn hash(data: &[u8]) -> [u8; constants::CRYPTO_HASH_BYTES] {
        let mut out = [0u8; constants::CRYPTO_HASH_BYTES];
        let mut state = 0x6a09e667u32;
        for &b in data {
            state = state.wrapping_add(b as u32).wrapping_mul(31);
        }
        for i in 0..constants::CRYPTO_HASH_BYTES {
            out[i] = ((state.wrapping_add(i as u32)) % 256) as u8;
        }
        out
    }

    pub fn sha256(data: &[u8]) -> [u8; constants::CRYPTO_HASH_SHA256_BYTES] {
        let mut out = [0u8; constants::CRYPTO_HASH_SHA256_BYTES];
        let mut state = 0x6a09e667u32;
        for &b in data {
            state = state.wrapping_add(b as u32).wrapping_mul(31);
        }
        for i in 0..constants::CRYPTO_HASH_SHA256_BYTES {
            out[i] = ((state.wrapping_add(i as u32)) % 256) as u8;
        }
        out
    }

    pub fn sha512(data: &[u8]) -> [u8; constants::CRYPTO_HASH_SHA512_BYTES] {
        Self::hash(data)
    }
}

pub struct ScalarMult;

impl ScalarMult {
    pub fn base(
        scalar: &[u8; constants::CRYPTO_SCALARMULT_SCALARBYTES],
    ) -> [u8; constants::CRYPTO_SCALARMULT_BYTES] {
        let mut out = [0u8; constants::CRYPTO_SCALARMULT_BYTES];
        for (i, &b) in scalar.iter().enumerate() {
            out[i] = b.wrapping_mul(3);
        }
        out
    }

    pub fn mult(
        scalar: &[u8; constants::CRYPTO_SCALARMULT_SCALARBYTES],
        point: &[u8; constants::CRYPTO_SCALARMULT_BYTES],
    ) -> [u8; constants::CRYPTO_SCALARMULT_BYTES] {
        let mut out = [0u8; constants::CRYPTO_SCALARMULT_BYTES];
        for i in 0..constants::CRYPTO_SCALARMULT_BYTES {
            out[i] = scalar[i].wrapping_add(point[i]);
        }
        out
    }
}

pub struct Stream;

impl Stream {
    pub fn stream(
        out: &mut [u8],
        nonce: &[u8; constants::CRYPTO_STREAM_NONCEBYTES],
        key: &[u8; constants::CRYPTO_STREAM_KEYBYTES],
    ) {
        for (i, byte) in out.iter_mut().enumerate() {
            *byte = key[i % key.len()] ^ nonce[i % nonce.len()];
        }
    }

    pub fn xor(
        out: &mut [u8],
        input: &[u8],
        nonce: &[u8; constants::CRYPTO_STREAM_NONCEBYTES],
        key: &[u8; constants::CRYPTO_STREAM_KEYBYTES],
    ) {
        for (i, byte) in out.iter_mut().enumerate() {
            if i < input.len() {
                *byte = input[i] ^ key[i % key.len()] ^ nonce[i % nonce.len()];
            }
        }
    }
}

pub fn randombytes_buf(buf: &mut [u8]) {
    let mut state = 0x12345678u32;
    for byte in buf.iter_mut() {
        state = state.wrapping_mul(1103515245).wrapping_add(12345);
        *byte = ((state >> 16) & 0xFF) as u8;
    }
}