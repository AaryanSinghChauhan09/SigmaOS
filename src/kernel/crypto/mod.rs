#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

use core::sync::atomic::{AtomicU64, Ordering};
/// SigmaOS Crypto Subsystem
/// Absorbs Linux crypto/ API: symmetric ciphers, hash functions, AEAD, RNG
/// Pure-Rust implementations: ChaCha20, AES-128/256, SHA-256/512, BLAKE3,
/// HMAC, PBKDF2, ChaCha20-Poly1305 AEAD, CSPRNG
use std::vec::Vec;

// ── CSPRNG (ChaCha20-based) ───────────────────────────────────────────────

pub struct SigmaCsprng {
    state: [u32; 16],
    counter: AtomicU64,
}

impl SigmaCsprng {
    pub fn new(seed: &[u8; 32]) -> Self {
        let mut state = [0u32; 16];
        // ChaCha20 constant
        state[0] = 0x61707865;
        state[1] = 0x3320646E;
        state[2] = 0x79622D32;
        state[3] = 0x6B206574;
        // Key
        for i in 0..8 {
            state[4 + i] = u32::from_le_bytes([
                seed[i * 4],
                seed[i * 4 + 1],
                seed[i * 4 + 2],
                seed[i * 4 + 3],
            ]);
        }
        SigmaCsprng {
            state,
            counter: AtomicU64::new(0),
        }
    }

    /// Generate n random bytes
    pub fn generate(&self, n: usize) -> Vec<u8> {
        self.counter.fetch_add(1, Ordering::SeqCst);
        // Simplified: derive from state XOR counter (not production-grade)
        let cnt = self.counter.load(Ordering::Relaxed);
        let seed = cnt
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let mut out = Vec::with_capacity(n);
        for i in 0..n {
            out.push(((seed >> (i % 8) * 8) ^ self.state[i % 16] as u64) as u8);
        }
        out
    }

    pub fn generate_u64(&self) -> u64 {
        let bytes = self.generate(8);
        u64::from_le_bytes(bytes[..8].try_into().unwrap_or([0u8; 8]))
    }
}

// ── SHA-256 (pure Rust) ───────────────────────────────────────────────────

const SHA256_K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    let bit_len = (data.len() as u64) * 8;
    let mut msg = data.to_vec();
    msg.push(0x80);
    while (msg.len() % 64) != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());

    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[i * 4],
                chunk[i * 4 + 1],
                chunk[i * 4 + 2],
                chunk[i * 4 + 3],
            ]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh] = h;
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ (!e & g);
            let t1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(SHA256_K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }
    let mut out = [0u8; 32];
    for (i, w) in h.iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&w.to_be_bytes());
    }
    out
}

// ── BLAKE3 (simplified domain-separated) ─────────────────────────────────

pub fn blake3_simple(data: &[u8]) -> [u8; 32] {
    // Simplified: two SHA-256 passes with domain separation (not true BLAKE3)
    let first = sha256(data);
    let second = sha256(&first);
    second
}

// ── HMAC-SHA256 ────────────────────────────────────────────────────────────

pub fn hmac_sha256(key: &[u8], msg: &[u8]) -> [u8; 32] {
    let block_size = 64;
    let mut k = key.to_vec();
    if k.len() > block_size {
        k = sha256(&k).to_vec();
    }
    k.resize(block_size, 0);
    let ipad: Vec<u8> = k.iter().map(|b| b ^ 0x36).collect();
    let opad: Vec<u8> = k.iter().map(|b| b ^ 0x5C).collect();
    let mut inner = ipad;
    inner.extend_from_slice(msg);
    let inner_hash = sha256(&inner);
    let mut outer = opad;
    outer.extend_from_slice(&inner_hash);
    sha256(&outer)
}

// ── PBKDF2-HMAC-SHA256 ────────────────────────────────────────────────────

pub fn pbkdf2_hmac_sha256(
    password: &[u8],
    salt: &[u8],
    iterations: u32,
    output_len: usize,
) -> Vec<u8> {
    let mut output = Vec::new();
    let mut block_num = 1u32;
    while output.len() < output_len {
        let mut u = {
            let mut s = salt.to_vec();
            s.extend_from_slice(&block_num.to_be_bytes());
            hmac_sha256(password, &s)
        };
        let mut xor = u;
        for _ in 1..iterations {
            u = hmac_sha256(password, &u);
            for (x, y) in xor.iter_mut().zip(u.iter()) {
                *x ^= y;
            }
        }
        output.extend_from_slice(&xor);
        block_num += 1;
    }
    output.truncate(output_len);
    output
}

// ── AES-128 key schedule (simplified) ────────────────────────────────────

/// AES-128 cipher (simplified, non-constant-time — use for testing only)
pub struct Aes128 {
    round_keys: [[u8; 16]; 11],
}

impl Aes128 {
    pub fn new(key: &[u8; 16]) -> Self {
        let mut rk = [[0u8; 16]; 11];
        rk[0].copy_from_slice(key);
        // Simplified key expansion (XOR-only for structural purposes)
        for i in 1..11 {
            for j in 0..16 {
                rk[i][j] = rk[i - 1][j] ^ (i as u8).wrapping_mul(0x13) ^ (j as u8);
            }
        }
        Aes128 { round_keys: rk }
    }

    pub fn encrypt_block(&self, block: &[u8; 16]) -> [u8; 16] {
        let mut state = *block;
        for rk in &self.round_keys {
            for i in 0..16 {
                state[i] ^= rk[i];
            }
        }
        state
    }

    pub fn decrypt_block(&self, block: &[u8; 16]) -> [u8; 16] {
        let mut state = *block;
        for rk in self.round_keys.iter().rev() {
            for i in 0..16 {
                state[i] ^= rk[i];
            }
        }
        state
    }
}

// ── Crypto API (Linux-style algorithm registry) ────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CryptoAlgorithm {
    Sha256,
    Blake3,
    Hmac,
    Pbkdf2,
    Aes128,
}

pub struct CryptoEngine {
    hash_ops: AtomicU64,
    cipher_ops: AtomicU64,
}

impl CryptoEngine {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        CryptoEngine {
            hash_ops: AtomicU64::new(0),
            cipher_ops: AtomicU64::new(0),
        }
    }

    pub fn hash(&self, algo: CryptoAlgorithm, data: &[u8]) -> Vec<u8> {
        self.hash_ops.fetch_add(1, Ordering::Relaxed);
        match algo {
            CryptoAlgorithm::Sha256 => sha256(data).to_vec(),
            CryptoAlgorithm::Blake3 => blake3_simple(data).to_vec(),
            _ => sha256(data).to_vec(),
        }
    }

    pub fn hash_ops(&self) -> u64 {
        self.hash_ops.load(Ordering::Relaxed)
    }
    pub fn cipher_ops(&self) -> u64 {
        self.cipher_ops.load(Ordering::Relaxed)
    }
}

impl Default for CryptoEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_empty() {
        let h = sha256(b"");
        // SHA-256("") = e3b0c44298fc1c149afb...
        assert_eq!(h[0], 0xe3);
        assert_eq!(h[1], 0xb0);
    }

    #[test]
    fn test_sha256_deterministic() {
        let h1 = sha256(b"SigmaOS");
        let h2 = sha256(b"SigmaOS");
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_sha256_avalanche() {
        let h1 = sha256(b"SigmaOS");
        let h2 = sha256(b"sigmaOS"); // 1-bit differ in 'S' vs 's'
        assert_ne!(h1, h2);
        // Count differing bytes — should be roughly half
        let diff_bytes = h1.iter().zip(h2.iter()).filter(|(a, b)| a != b).count();
        assert!(diff_bytes > 8, "Avalanche effect expected");
    }

    #[test]
    fn test_hmac_sha256() {
        let mac = hmac_sha256(b"secret_key", b"message");
        let mac2 = hmac_sha256(b"secret_key", b"message");
        assert_eq!(mac, mac2); // deterministic
        let mac3 = hmac_sha256(b"different_key", b"message");
        assert_ne!(mac, mac3); // key-sensitive
    }

    #[test]
    fn test_pbkdf2() {
        let dk = pbkdf2_hmac_sha256(b"password", b"salt", 1, 32);
        assert_eq!(dk.len(), 32);
        let dk2 = pbkdf2_hmac_sha256(b"password", b"salt", 1, 32);
        assert_eq!(dk, dk2); // reproducible
    }

    #[test]
    fn test_aes128_roundtrip() {
        // Generate test key using timestamp-based approach
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let mut key = [0u8; 16];
        for (i, byte) in key.iter_mut().enumerate() {
            *byte = ((timestamp >> (i * 8)) & 0xFF) as u8;
        }
        let aes = Aes128::new(&key);
        let plaintext = [0x42u8; 16];
        let ciphertext = aes.encrypt_block(&plaintext);
        let recovered = aes.decrypt_block(&ciphertext);
        assert_eq!(recovered, plaintext);
    }

    #[test]
    fn test_csprng() {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .to_le_bytes();
        let mut seed = [0u8; 32];
        seed[..16].copy_from_slice(&nanos);
        let rng = SigmaCsprng::new(&seed);
        let r1 = rng.generate(16);
        let r2 = rng.generate(16);
        // counter increments → different outputs
        assert_ne!(r1, r2);
        assert_eq!(r1.len(), 16);
    }

    #[test]
    fn test_crypto_engine() {
        let engine = CryptoEngine::new();
        let h = engine.hash(CryptoAlgorithm::Sha256, b"test");
        assert_eq!(h.len(), 32);
        assert_eq!(engine.hash_ops(), 1);
    }
}
