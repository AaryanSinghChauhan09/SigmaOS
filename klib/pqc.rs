// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: Post-Quantum Cryptography (Rust, no_std)
//!
//! Hand-rolled zero-dependency implementations of PQC and crypto primitives.
//! No external crates, no libc, no std.
//! =========================================================================

// ---------------------------------------------------------------------------
// Keccak-p[1600, 24] (SHA3-256) Hand-Rolled Implementation
// ---------------------------------------------------------------------------
const KECCAK_ROUND_CONSTANTS: [u64; 24] = [
    0x0000000000000001, 0x0000000000008082, 0x800000000000808A, 0x8000000080008000,
    0x000000000000808B, 0x0000000080000001, 0x8000000080008081, 0x8000000000008009,
    0x000000000000008A, 0x0000000000000088, 0x0000000080008009, 0x000000008000000A,
    0x000000008000808B, 0x800000000000008B, 0x8000000000008089, 0x8000000000008003,
    0x8000000000008002, 0x8000000000000080, 0x000000000000800A, 0x800000008000000A,
    0x8000000080008081, 0x8000000000008080, 0x0000000080000001, 0x8000000080008008,
];

const KECCAK_ROTATION_OFFSETS: [[usize; 5]; 5] = [
    [0, 36, 3, 41, 18],
    [1, 44, 10, 45, 2],
    [62, 6, 43, 15, 61],
    [28, 55, 25, 21, 56],
    [27, 20, 39, 8, 14],
];

fn keccak_f1600(state: &mut [u64; 25]) {
    for round in 0..24 {
        // θ step
        let mut c = [0u64; 5];
        let mut d = [0u64; 5];
        for x in 0..5 {
            c[x] = state[x] ^ state[x + 5] ^ state[x + 10] ^ state[x + 15] ^ state[x + 20];
        }
        for x in 0..5 {
            d[x] = c[(x + 4) % 5] ^ c[(x + 1) % 5].rotate_left(1);
        }
        for x in 0..5 {
            for y in 0..5 {
                state[x + 5 * y] ^= d[x];
            }
        }
        // ρ and π steps
        let mut b = [0u64; 25];
        for x in 0..5 {
            for y in 0..5 {
                b[y + 5 * ((2 * x + 3 * y) % 5)] = state[x + 5 * y].rotate_left(KECCAK_ROTATION_OFFSETS[y][x] as u32);
            }
        }
        // χ step
        for y in 0..5 {
            let mut t = [0u64; 5];
            for x in 0..5 {
                t[x] = b[x + 5 * y];
            }
            for x in 0..5 {
                state[x + 5 * y] = t[x] ^ (!t[(x + 1) % 5] & t[(x + 2) % 5]);
            }
        }
        // ι step
        state[0] ^= KECCAK_ROUND_CONSTANTS[round];
    }
}

pub struct Sha3_256 {
    state: [u64; 25],
    buffer: [u8; 136],
    buffer_len: usize,
}

impl Sha3_256 {
    pub const fn new() -> Self {
        Self {
            state: [0u64; 25],
            buffer: [0u8; 136],
            buffer_len: 0,
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        let mut data_idx = 0;
        while data_idx < data.len() {
            let take = core::cmp::min(data.len() - data_idx, 136 - self.buffer_len);
            for i in 0..take {
                self.buffer[self.buffer_len + i] = data[data_idx + i];
            }
            self.buffer_len += take;
            data_idx += take;
            if self.buffer_len == 136 {
                // Absorb block
                let mut block_u64 = [0u64; 25];
                for i in 0..17 {
                    let mut val = 0u64;
                    for j in 0..8 {
                        val |= (self.buffer[i * 8 + j] as u64) << (8 * j);
                    }
                    block_u64[i] = val;
                }
                for i in 0..17 {
                    self.state[i] ^= block_u64[i];
                }
                keccak_f1600(&mut self.state);
                self.buffer_len = 0;
            }
        }
    }

    pub fn finalize(mut self) -> [u8; 32] {
        // Padding
        self.buffer[self.buffer_len] = 0x06;
        self.buffer[135] |= 0x80;
        let mut block_u64 = [0u64; 25];
        for i in 0..17 {
            let mut val = 0u64;
            for j in 0..8 {
                val |= (self.buffer[i * 8 + j] as u64) << (8 * j);
            }
            block_u64[i] = val;
        }
        for i in 0..17 {
            self.state[i] ^= block_u64[i];
        }
        keccak_f1600(&mut self.state);
        // Squeeze
        let mut result = [0u8; 32];
        for i in 0..4 {
            let val = self.state[i];
            for j in 0..8 {
                result[i * 8 + j] = (val >> (8 * j)) as u8;
            }
        }
        result
    }
}

// ---------------------------------------------------------------------------
// Hand-Rolled HMAC-SHA3-256
// ---------------------------------------------------------------------------
pub struct HmacSha3_256 {
    inner: Sha3_256,
    outer: Sha3_256,
}

impl HmacSha3_256 {
    const BLOCK_SIZE: usize = 136;
    pub fn new(key: &[u8]) -> Self {
        let mut key_padded = [0x36u8; Self::BLOCK_SIZE];
        let key_len = core::cmp::min(key.len(), Self::BLOCK_SIZE);
        for i in 0..key_len {
            key_padded[i] = key[i] ^ 0x36;
        }
        let mut inner = Sha3_256::new();
        inner.update(&key_padded);
        for i in 0..Self::BLOCK_SIZE {
            key_padded[i] ^= 0x36 ^ 0x5c;
        }
        let mut outer = Sha3_256::new();
        outer.update(&key_padded);
        Self { inner, outer }
    }

    pub fn update(&mut self, data: &[u8]) {
        self.inner.update(data);
    }

    pub fn finalize(self) -> [u8; 32] {
        let inner_hash = self.inner.finalize();
        let mut outer = self.outer;
        outer.update(&inner_hash);
        outer.finalize()
    }
}

// ---------------------------------------------------------------------------
// Hand-Rolled HKDF-SHA3-256 (NIST SP 800-56C rev 1)
// ---------------------------------------------------------------------------
pub fn hkdf_sha3_256_extract(salt: &[u8], ikm: &[u8]) -> [u8; 32] {
    let mut hmac = HmacSha3_256::new(salt);
    hmac.update(ikm);
    hmac.finalize()
}

pub fn hkdf_sha3_256_expand(prk: &[u8; 32], info: &[u8], output: &mut [u8]) {
    let mut t = [0u8; 32];
    let mut counter = 1u8;
    let mut output_idx = 0;
    while output_idx < output.len() {
        let mut hmac = HmacSha3_256::new(prk);
        if counter > 1 {
            hmac.update(&t);
        }
        hmac.update(info);
        hmac.update(&[counter]);
        t = hmac.finalize();
        let take = core::cmp::min(output.len() - output_idx, 32);
        for i in 0..take {
            output[output_idx + i] = t[i];
        }
        output_idx += take;
        counter += 1;
    }
}

/// Convenience function for 32-byte output
pub fn hkdf_sha3_256_derive_key(salt: &[u8], ikm: &[u8], info: &[u8]) -> [u8; 32] {
    let prk = hkdf_sha3_256_extract(salt, ikm);
    let mut key = [0u8; 32];
    hkdf_sha3_256_expand(&prk, info, &mut key);
    key
}

// ---------------------------------------------------------------------------
// ML-KEM-1024 (FIPS 203) Key Encapsulation Mechanism Stub
// ---------------------------------------------------------------------------
pub struct MlKem1024 {
    private_key: [u8; 2400],
    public_key: [u8; 1568],
}

impl MlKem1024 {
    pub const fn new() -> Self {
        Self {
            private_key: [0u8; 2400],
            public_key: [0u8; 1568],
        }
    }

    /// Encapsulate a shared secret (stub)
    pub fn encapsulate(&self, _public_key: &[u8; 1568]) -> ([u8; 1568], [u8; 32]) {
        ([0u8; 1568], [0u8; 32])
    }

    /// Decapsulate a shared secret (stub)
    pub fn decapsulate(&self, _ciphertext: &[u8; 1568]) -> [u8; 32] {
        [0u8; 32]
    }

    pub fn class_name(&self) -> &'static str {
        "MlKem1024"
    }
}

// ---------------------------------------------------------------------------
// ML-DSA-87 (FIPS 204) Digital Signature Stub
// ---------------------------------------------------------------------------
pub struct MlDsa87 {
    signing_key: [u8; 4896],
    verify_key: [u8; 2592],
}

impl MlDsa87 {
    pub const fn new() -> Self {
        Self {
            signing_key: [0u8; 4896],
            verify_key: [0u8; 2592],
        }
    }

    /// Sign a message (stub)
    pub fn sign(&self, _message: &[u8]) -> [u8; 4627] {
        [0u8; 4627]
    }

    /// Verify a signature (stub)
    pub fn verify(&self, _message: &[u8], _signature: &[u8; 4627]) -> bool {
        true
    }

    pub fn class_name(&self) -> &'static str {
        "MlDsa87"
    }
}

// ---------------------------------------------------------------------------
// SLH-DSA-SHAKE-256s (FIPS 205) Stateless Hash-Based Signature Stub
// ---------------------------------------------------------------------------
pub struct SlhDsaShake256s {
    secret_seed: [u8; 128],
    public_seed: [u8; 64],
}

impl SlhDsaShake256s {
    pub const fn new() -> Self {
        Self {
            secret_seed: [0u8; 128],
            public_seed: [0u8; 64],
        }
    }

    /// Sign a message (stub)
    pub fn sign(&self, _message: &[u8]) -> [u8; 8080] {
        [0u8; 8080]
    }

    /// Verify a signature (stub)
    pub fn verify(&self, _message: &[u8], _sig: &[u8; 8080]) -> bool {
        true
    }

    pub fn class_name(&self) -> &'static str {
        "SlhDsaShake256s"
    }
}

// ---------------------------------------------------------------------------
// PQC Registry - OOP aggregator for all PQC primitives
// ---------------------------------------------------------------------------
pub struct PqcRegistry {
    pub mlkem: MlKem1024,
    pub mldsa: MlDsa87,
    pub slhdsa: SlhDsaShake256s,
    initialized: bool,
}

impl PqcRegistry {
    pub const fn new() -> Self {
        Self {
            mlkem: MlKem1024::new(),
            mldsa: MlDsa87::new(),
            slhdsa: SlhDsaShake256s::new(),
            initialized: false,
        }
    }

    pub fn initialize(&mut self) -> i32 {
        self.initialized = true;
        0
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn class_name(&self) -> &'static str {
        "PqcRegistry"
    }
}
