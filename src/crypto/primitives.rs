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

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

/// SHA-256 hash
#[repr(C)]
pub struct SHA256Hash {
    pub data: [u8; 32],
}

impl SHA256Hash {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SHA256Hash { data: [0; 32] }
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.data
    }
}

/// SHA-256 implementation
pub struct SHA256 {
    state: [u32; 8],
    buffer: [u8; 64],
    buffer_len: usize,
    total_len: u64,
}

impl SHA256 {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SHA256 {
            state: [
                0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
                0x5be0cd19,
            ],
            buffer: [0; 64],
            buffer_len: 0,
            total_len: 0,
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        let mut offset = 0;
        while offset < data.len() {
            let remaining = data.len() - offset;
            let space = 64 - self.buffer_len;

            if remaining >= space {
                // Fill buffer and process
                self.buffer[self.buffer_len..64].copy_from_slice(&data[offset..offset + space]);
                self.process_block();
                self.buffer_len = 0;
                offset += space;
            } else {
                // Copy remaining to buffer
                self.buffer[self.buffer_len..self.buffer_len + remaining]
                    .copy_from_slice(&data[offset..]);
                self.buffer_len += remaining;
                offset += remaining;
            }
        }
        self.total_len = self.total_len.wrapping_add(data.len() as u64);
    }

    pub fn finalize(mut self) -> SHA256Hash {
        // Append padding
        let bit_len = self.total_len.wrapping_mul(8);

        self.buffer[self.buffer_len] = 0x80;
        self.buffer_len += 1;

        if self.buffer_len > 56 {
            self.buffer[self.buffer_len..].fill(0);
            self.process_block();
            self.buffer_len = 0;
        }

        self.buffer[self.buffer_len..56].fill(0);
        let len_bytes = bit_len.to_be_bytes();
        self.buffer[56..64].copy_from_slice(&len_bytes);

        self.process_block();

        // Convert state to hash
        let mut hash = SHA256Hash::new();
        for i in 0..8 {
            let bytes = self.state[i].to_be_bytes();
            hash.data[i * 4..(i + 1) * 4].copy_from_slice(&bytes);
        }

        hash
    }

    fn process_block(&mut self) {
        let mut w = [0u32; 64];

        // Prepare message schedule
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                self.buffer[i * 4],
                self.buffer[i * 4 + 1],
                self.buffer[i * 4 + 2],
                self.buffer[i * 4 + 3],
            ]);
        }

        for i in 16..64 {
            let s0 = sigma1(w[i - 2]);
            let s1 = sigma0(w[i - 15]);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }

        // Initialize working variables
        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];
        let mut e = self.state[4];
        let mut f = self.state[5];
        let mut g = self.state[6];
        let mut h = self.state[7];

        // Compression function
        for i in 0..64 {
            let t1 = h
                .wrapping_add(big_sigma1(e))
                .wrapping_add(ch(e, f, g))
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let t2 = big_sigma0(a).wrapping_add(maj(a, b, c));

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }

        // Update state
        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
        self.state[4] = self.state[4].wrapping_add(e);
        self.state[5] = self.state[5].wrapping_add(f);
        self.state[6] = self.state[6].wrapping_add(g);
        self.state[7] = self.state[7].wrapping_add(h);
    }
}

/// SHA-256 constants
const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

/// SHA-256 helper functions
fn ch(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (!x & z)
}

fn maj(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}

fn big_sigma0(x: u32) -> u32 {
    x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}

fn big_sigma1(x: u32) -> u32 {
    x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}

fn sigma0(x: u32) -> u32 {
    x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
}

fn sigma1(x: u32) -> u32 {
    x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
}

/// AES-256 encryption key
#[repr(C)]
pub struct AES256Key {
    pub data: [u8; 32],
}

impl AES256Key {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        AES256Key { data: [0; 32] }
    }

    pub fn from_bytes(bytes: &[u8; 32]) -> Self {
        AES256Key { data: *bytes }
    }
}

/// AES-256 block (128 bits)
#[repr(C)]
pub struct AES256Block {
    pub data: [u8; 16],
}

impl AES256Block {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        AES256Block { data: [0; 16] }
    }

    pub fn from_bytes(bytes: &[u8; 16]) -> Self {
        AES256Block { data: *bytes }
    }
}

/// AES-256 encryption
pub struct AES256 {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveError {
    ProviderNotIntegrated,
    BufferLengthMismatch,
}

impl AES256 {
    pub fn new(_key: &AES256Key) -> Self {
        AES256 {}
    }

    pub fn encrypt_block(&self, _block: &mut AES256Block) -> Result<(), PrimitiveError> {
        Err(PrimitiveError::ProviderNotIntegrated)
    }

    pub fn decrypt_block(&self, _block: &mut AES256Block) -> Result<(), PrimitiveError> {
        Err(PrimitiveError::ProviderNotIntegrated)
    }
}

/// Deterministic xorshift generator for simulation only. Never use it for keys,
/// nonces, authentication tokens, or other secrets.
pub struct XorshiftRNG {
    state: [u64; 4],
}

impl XorshiftRNG {
    pub fn new(seed: u64) -> Self {
        XorshiftRNG {
            state: [
                seed,
                seed.wrapping_mul(0x5851f42d4c957f2d),
                seed.wrapping_mul(0x14057b7ef767814f),
                seed.wrapping_mul(0xc4ceb9fe1a85ec53),
            ],
        }
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut t = self.state[0];
        let s = self.state[3];

        self.state[0] = s;
        t ^= t << 23;
        t ^= t >> 17;
        t ^= s ^ (s >> 26);
        self.state[3] = t;
        self.state[1] = self.state[1].wrapping_add(t);
        self.state[2] = self.state[2].wrapping_add(s);

        t
    }

    pub fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    pub fn next_bytes(&mut self, buf: &mut [u8]) {
        for chunk in buf.chunks_mut(8) {
            let val = self.next_u64();
            let bytes = val.to_le_bytes();
            if chunk.len() >= 8 {
                chunk.copy_from_slice(&bytes);
            } else {
                chunk.copy_from_slice(&bytes[..chunk.len()]);
            }
        }
    }

    pub fn fill_random(&mut self, buf: &mut [u8]) {
        self.next_bytes(buf);
    }
}

/// Hash data using SHA-256
pub fn sha256_hash(data: &[u8]) -> SHA256Hash {
    let mut hasher = SHA256::new();
    hasher.update(data);
    hasher.finalize()
}

/// Secure randomness is unavailable until a real entropy provider is wired in.
pub fn random_bytes(_buf: &mut [u8]) -> Result<(), PrimitiveError> {
    Err(PrimitiveError::ProviderNotIntegrated)
}

/// Generate random 256-bit key
pub fn random_key() -> Result<AES256Key, PrimitiveError> {
    Err(PrimitiveError::ProviderNotIntegrated)
}

/// XOR two byte arrays
pub fn xor_bytes(a: &[u8], b: &[u8], out: &mut [u8]) -> Result<(), PrimitiveError> {
    if a.len() != b.len() || a.len() != out.len() {
        return Err(PrimitiveError::BufferLengthMismatch);
    }
    for ((out_byte, a_byte), b_byte) in out.iter_mut().zip(a).zip(b) {
        *out_byte = *a_byte ^ *b_byte;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        random_bytes, random_key, sha256_hash, AES256Block, AES256Key, PrimitiveError, AES256,
    };

    #[test]
    fn sha256_matches_standard_vectors_and_padding_boundaries() {
        assert_eq!(
            sha256_hash(b"").data,
            [
                0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f,
                0xb9, 0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b,
                0x78, 0x52, 0xb8, 0x55,
            ]
        );
        assert_eq!(
            sha256_hash(b"abc").data,
            [
                0xba, 0x78, 0x16, 0xbf, 0x8f, 0x01, 0xcf, 0xea, 0x41, 0x41, 0x40, 0xde, 0x5d, 0xae,
                0x22, 0x23, 0xb0, 0x03, 0x61, 0xa3, 0x96, 0x17, 0x7a, 0x9c, 0xb4, 0x10, 0xff, 0x61,
                0xf2, 0x00, 0x15, 0xad,
            ]
        );
        assert_eq!(
            sha256_hash(&[b'a'; 56]).data,
            [
                0xb3, 0x54, 0x39, 0xa4, 0xac, 0x6f, 0x09, 0x48, 0xb6, 0xd6, 0xf9, 0xe3, 0xc6, 0xaf,
                0x0f, 0x5f, 0x59, 0x0c, 0xe2, 0x0f, 0x1b, 0xde, 0x70, 0x90, 0xef, 0x79, 0x70, 0x68,
                0x6e, 0xc6, 0x73, 0x8a,
            ]
        );
        assert_eq!(
            sha256_hash(&[b'a'; 64]).data,
            [
                0xff, 0xe0, 0x54, 0xfe, 0x7a, 0xe0, 0xcb, 0x6d, 0xc6, 0x5c, 0x3a, 0xf9, 0xb6, 0x1d,
                0x52, 0x09, 0xf4, 0x39, 0x85, 0x1d, 0xb4, 0x3d, 0x0b, 0xa5, 0x99, 0x73, 0x37, 0xdf,
                0x15, 0x46, 0x68, 0xeb,
            ]
        );
    }

    #[test]
    fn aes_and_random_key_apis_fail_without_provider() {
        let cipher = AES256::new(&AES256Key::new());
        let mut block = AES256Block::new();
        let before = block.data;
        assert_eq!(
            cipher.encrypt_block(&mut block),
            Err(PrimitiveError::ProviderNotIntegrated)
        );
        assert_eq!(
            cipher.decrypt_block(&mut block),
            Err(PrimitiveError::ProviderNotIntegrated)
        );
        assert_eq!(block.data, before);

        let mut bytes = [0xA5; 32];
        assert_eq!(
            random_bytes(&mut bytes),
            Err(PrimitiveError::ProviderNotIntegrated)
        );
        assert_eq!(bytes, [0xA5; 32]);
        assert!(matches!(
            random_key(),
            Err(PrimitiveError::ProviderNotIntegrated)
        ));
    }
}
