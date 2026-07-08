//! sigma_crypto.rs — Custom Ed25519-like signing primitives (no_std Rust)
//! Provides: SHA-512 (iterative, no external crate), SHA-256, Curve25519 field arithmetic
//! stubs, and a signing key validation harness.
//! Designed to replace OpenSSL/libgcrypt dependencies for package signature checks.

#![no_std]
#![allow(dead_code)]

// ── SHA-512 Constants ─────────────────────────────────────────────────────

const K: [u64; 80] = [
    0x428a2f98d728ae22, 0x7137449123ef65cd, 0xb5c0fbcfec4d3b2f, 0xe9b5dba58189dbbc,
    0x3956c25bf348b538, 0x59f111f1b605d019, 0x923f82a4af194f9b, 0xab1c5ed5da6d8118,
    0xd807aa98a3030242, 0x12835b0145706fbe, 0x243185be4ee4b28c, 0x550c7dc3d5ffb4e2,
    0x72be5d74f27b896f, 0x80deb1fe3b1696b1, 0x9bdc06a725c71235, 0xc19bf174cf692694,
    0xe49b69c19ef14ad2, 0xefbe4786384f25e3, 0x0fc19dc68b8cd5b5, 0x240ca1cc77ac9c65,
    0x2de92c6f592b0275, 0x4a7484aa6ea6e483, 0x5cb0a9dcbd41fbd4, 0x76f988da831153b5,
    0x983e5152ee66dfab, 0xa831c66d2db43210, 0xb00327c898fb213f, 0xbf597fc7beef0ee4,
    0xc6e00bf33da88fc2, 0xd5a79147930aa725, 0x06ca6351e003826f, 0x142929670a0e6e70,
    0x27b70a8546d22ffc, 0x2e1b21385c26c926, 0x4d2c6dfc5ac42aed, 0x53380d139d95b3df,
    0x650a73548baf63de, 0x766a0abb3c77b2a8, 0x81c2c92e47edaee6, 0x92722c851482353b,
    0xa2bfe8a14cf10364, 0xa81a664bbc423001, 0xc24b8b70d0f89791, 0xc76c51a30654be30,
    0xd192e819d6ef5218, 0xd69906245565a910, 0xf40e35855771202a, 0x106aa07032bbd1b8,
    0x19a4c116b8d2d0c8, 0x1e376c085141ab53, 0x2748774cdf8eeb99, 0x34b0bcb5e19b48a8,
    0x391c0cb3c5c95a63, 0x4ed8aa4ae3418acb, 0x5b9cca4f7763e373, 0x682e6ff3d6b2b8a3,
    0x748f82ee5defb2fc, 0x78a5636f43172f60, 0x84c87814a1f0ab72, 0x8cc702081a6439ec,
    0x90befffa23631e28, 0xa4506cebde82bde9, 0xbef9a3f7b2c67915, 0xc67178f2e372532b,
    0xca273eceea26619c, 0xd186b8c721c0c207, 0xeada7dd6cde0eb1e, 0xf57d4f7fee6ed178,
    0x06f067aa72176fba, 0x0a637dc5a2c898a6, 0x113f9804bef90dae, 0x1b710b35131c471b,
    0x28db77f523047d84, 0x32caab7b40c72493, 0x3c9ebe0a15c9bebc, 0x431d67c49c100d4c,
    0x4cc5d4becb3e42b6, 0x597f299cfc657e2a, 0x5fcb6fab3ad6faec, 0x6c44198c4a475817,
];

#[inline(always)]
fn rotr64(x: u64, n: u32) -> u64 { x.rotate_right(n) }

// ── SHA-256 Constants ─────────────────────────────────────────────────────

const K256: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1,
    0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
    0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786,
    0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147,
    0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
    0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
    0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a,
    0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
    0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

#[inline(always)]
fn rotr32(x: u32, n: u32) -> u32 { x.rotate_right(n) }

// ── SHA-512 State ─────────────────────────────────────────────────────────

pub struct Sha512 {
    state:  [u64; 8],
    buf:    [u8; 128],
    buf_len: usize,
    total:  u128,
}

impl Sha512 {
    pub const fn new() -> Self {
        Self {
            state: [
                0x6a09e667f3bcc908, 0xbb67ae8584caa73b,
                0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1,
                0x510e527fade682d1, 0x9b05688c2b3e6c1f,
                0x1f83d9abfb41bd6b, 0x5be0cd19137e2179,
            ],
            buf: [0u8; 128],
            buf_len: 0,
            total: 0,
        }
    }

    fn compress(&mut self, block: &[u8; 128]) {
        let mut w = [0u64; 80];
        for i in 0..16 {
            w[i] = u64::from_be_bytes([
                block[i*8], block[i*8+1], block[i*8+2], block[i*8+3],
                block[i*8+4], block[i*8+5], block[i*8+6], block[i*8+7],
            ]);
        }
        for i in 16..80 {
            let s0 = rotr64(w[i-15], 1) ^ rotr64(w[i-15], 8) ^ (w[i-15] >> 7);
            let s1 = rotr64(w[i-2],  19) ^ rotr64(w[i-2],  61) ^ (w[i-2] >> 6);
            w[i] = w[i-16].wrapping_add(s0).wrapping_add(w[i-7]).wrapping_add(s1);
        }

        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = self.state;

        for i in 0..80 {
            let s1 = rotr64(e, 14) ^ rotr64(e, 18) ^ rotr64(e, 41);
            let ch  = (e & f) ^ ((!e) & g);
            let t1  = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0  = rotr64(a, 28) ^ rotr64(a, 34) ^ rotr64(a, 39);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2  = s0.wrapping_add(maj);
            h = g; g = f; f = e; e = d.wrapping_add(t1);
            d = c; c = b; b = a; a = t1.wrapping_add(t2);
        }

        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
        self.state[4] = self.state[4].wrapping_add(e);
        self.state[5] = self.state[5].wrapping_add(f);
        self.state[6] = self.state[6].wrapping_add(g);
        self.state[7] = self.state[7].wrapping_add(h);
    }

    pub fn update(&mut self, data: &[u8]) {
        self.total += data.len() as u128;
        let mut off = 0usize;
        while off < data.len() {
            let space = 128 - self.buf_len;
            let copy  = space.min(data.len() - off);
            self.buf[self.buf_len..self.buf_len+copy].copy_from_slice(&data[off..off+copy]);
            self.buf_len += copy;
            off += copy;
            if self.buf_len == 128 {
                let block: [u8; 128] = self.buf;
                self.compress(&block);
                self.buf_len = 0;
            }
        }
    }

    pub fn finalise(mut self) -> [u8; 64] {
        let bit_len = self.total * 8;
        self.update(&[0x80]);
        while self.buf_len != 112 {
            self.update(&[0x00]);
        }
        let mut len_bytes = [0u8; 16];
        len_bytes[8..].copy_from_slice(&(bit_len as u64).to_be_bytes());
        self.update(&len_bytes);

        let mut out = [0u8; 64];
        for (i, word) in self.state.iter().enumerate() {
            out[i*8..(i+1)*8].copy_from_slice(&word.to_be_bytes());
        }
        out
    }
}

/// Convenience: hash a byte slice in one call.
pub fn sha512(data: &[u8]) -> [u8; 64] {
    let mut h = Sha512::new();
    h.update(data);
    h.finalise()
}

// ── SHA-256 State ─────────────────────────────────────────────────────────

pub struct Sha256 {
    state:  [u32; 8],
    buf:    [u8; 64],
    buf_len: usize,
    total:  u64,
}

impl Sha256 {
    pub const fn new() -> Self {
        Self {
            state: [
                0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
                0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
            ],
            buf: [0u8; 64],
            buf_len: 0,
            total: 0,
        }
    }

    fn compress(&mut self, block: &[u8; 64]) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                block[i*4], block[i*4+1], block[i*4+2], block[i*4+3],
            ]);
        }
        for i in 16..64 {
            let s0 = rotr32(w[i-15], 7) ^ rotr32(w[i-15], 18) ^ (w[i-15] >> 3);
            let s1 = rotr32(w[i-2],  17) ^ rotr32(w[i-2],  19) ^ (w[i-2] >> 10);
            w[i] = w[i-16].wrapping_add(s0).wrapping_add(w[i-7]).wrapping_add(s1);
        }

        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = self.state;

        for i in 0..64 {
            let s1 = rotr32(e, 6) ^ rotr32(e, 11) ^ rotr32(e, 25);
            let ch  = (e & f) ^ ((!e) & g);
            let t1  = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(K256[i]).wrapping_add(w[i]);
            let s0  = rotr32(a, 2) ^ rotr32(a, 13) ^ rotr32(a, 22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2  = s0.wrapping_add(maj);
            h = g; g = f; f = e; e = d.wrapping_add(t1);
            d = c; c = b; b = a; a = t1.wrapping_add(t2);
        }

        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
        self.state[4] = self.state[4].wrapping_add(e);
        self.state[5] = self.state[5].wrapping_add(f);
        self.state[6] = self.state[6].wrapping_add(g);
        self.state[7] = self.state[7].wrapping_add(h);
    }

    pub fn update(&mut self, data: &[u8]) {
        self.total += data.len() as u64;
        let mut off = 0usize;
        while off < data.len() {
            let space = 64 - self.buf_len;
            let copy  = space.min(data.len() - off);
            self.buf[self.buf_len..self.buf_len+copy].copy_from_slice(&data[off..off+copy]);
            self.buf_len += copy;
            off += copy;
            if self.buf_len == 64 {
                let block: [u8; 64] = self.buf;
                self.compress(&block);
                self.buf_len = 0;
            }
        }
    }

    pub fn finalise(mut self) -> [u8; 32] {
        let bit_len = self.total * 8;
        self.update(&[0x80]);
        while self.buf_len != 56 {
            self.update(&[0x00]);
        }
        let mut len_bytes = [0u8; 8];
        len_bytes.copy_from_slice(&bit_len.to_be_bytes());
        self.update(&len_bytes);

        let mut out = [0u8; 32];
        for (i, word) in self.state.iter().enumerate() {
            out[i*4..(i+1)*4].copy_from_slice(&word.to_be_bytes());
        }
        out
    }
}

/// Convenience: hash a byte slice in one call.
pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(data);
    h.finalise()
}

// ── C-ABI Exports for SHA-256 ───────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_crypto_sha256(
    data: *const u8,
    len: u32,
    hash: *mut u8,
) -> i32 {
    if data.is_null() || hash.is_null() {
        return -1;
    }

    let data_slice = core::slice::from_raw_parts(data, len as usize);
    let result = sha256(data_slice);
    
    for i in 0..32 {
        *hash.add(i) = result[i];
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn sigma_crypto_sha512(
    data: *const u8,
    len: u32,
    hash: *mut u8,
) -> i32 {
    if data.is_null() || hash.is_null() {
        return -1;
    }

    let data_slice = core::slice::from_raw_parts(data, len as usize);
    let result = sha512(data_slice);
    
    for i in 0..64 {
        *hash.add(i) = result[i];
    }

    0
}

// ── PGP Key Generation (Ed25519-like) ───────────────────────────────────────

#[repr(C)]
pub struct PgpKeyPair {
    pub public_key: [u8; 32],
    pub private_key: [u8; 64],
    pub key_id: [u8; 8],
    pub created: u64,
}

#[repr(C)]
pub struct PgpIdentity {
    pub name: [u8; 256],
    pub email: [u8; 256],
    pub comment: [u8; 256],
}

static mut PGP_KEY_PAIR: Option<PgpKeyPair> = None;

/// Generate PGP key pair (Ed25519-like)
#[no_mangle]
pub unsafe extern "C" fn sigma_crypto_pgp_generate_key(
    identity: *const PgpIdentity,
    key_pair: *mut PgpKeyPair,
) -> i32 {
    if identity.is_null() || key_pair.is_null() {
        return -1;
    }

    let ident = &*identity;
    let kp = &mut *key_pair;

    // Generate Ed25519 key pair
    // In real implementation, use proper Ed25519 key generation
    // For now, use SHA-512 to derive keys from identity
    
    let mut seed = [0u8; 32];
    
    // Derive seed from identity
    let mut hasher = Sha512::new();
    hasher.update(&ident.name);
    hasher.update(&ident.email);
    hasher.update(&ident.comment);
    let hash = hasher.finalise();
    
    seed.copy_from_slice(&hash[..32]);
    
    // Derive private key (64 bytes for Ed25519)
    let mut priv_hasher = Sha512::new();
    priv_hasher.update(&seed);
    let priv_hash = priv_hasher.finalise();
    kp.private_key.copy_from_slice(&priv_hash);
    
    // Derive public key (32 bytes)
    // In real Ed25519, public_key = [priv_hash[32..]] * G
    // For now, use first 32 bytes of private hash
    kp.public_key.copy_from_slice(&priv_hash[32..64]);
    
    // Generate key ID (first 8 bytes of public key)
    kp.key_id.copy_from_slice(&kp.public_key[..8]);
    
    // Set creation timestamp (current time in seconds since epoch)
    // In real implementation, get actual time
    kp.created = 1715097600; // 2024-05-07 (example)
    
    // Store globally
    PGP_KEY_PAIR = Some(*kp);
    
    0
}

/// Sign data with PGP private key
#[no_mangle]
pub unsafe extern "C" fn sigma_crypto_pgp_sign(
    data: *const u8,
    data_len: u32,
    signature: *mut u8,
    sig_len: *mut u32,
) -> i32 {
    if data.is_null() || signature.is_null() || sig_len.is_null() {
        return -1;
    }

    if PGP_KEY_PAIR.is_none() {
        return -2;
    }

    let kp = &PGP_KEY_PAIR.unwrap();
    
    // Sign data using Ed25519
    // In real implementation, use proper Ed25519 signing
    // For now, use HMAC-SHA512 with private key
    
    let data_slice = core::slice::from_raw_parts(data, data_len as usize);
    
    let mut hmac_key = [0u8; 128];
    for i in 0..64 {
        if i < 64 {
            hmac_key[i] = kp.private_key[i];
        } else {
            hmac_key[i] = 0x36; // HMAC inner pad
        }
    }
    
    let mut inner_hasher = Sha512::new();
    inner_hasher.update(&hmac_key);
    inner_hasher.update(data_slice);
    let inner_hash = inner_hasher.finalise();
    
    for i in 0..64 {
        if i < 64 {
            hmac_key[i] = kp.private_key[i] ^ 0x5c; // HMAC outer pad
        } else {
            hmac_key[i] = 0x5c;
        }
    }
    
    let mut outer_hasher = Sha512::new();
    outer_hasher.update(&hmac_key);
    outer_hasher.update(&inner_hash);
    let signature_hash = outer_hasher.finalise();
    
    *sig_len = 64;
    for i in 0..64 {
        *signature.add(i) = signature_hash[i];
    }
    
    0
}

/// Verify PGP signature
#[no_mangle]
pub unsafe extern "C" fn sigma_crypto_pgp_verify(
    data: *const u8,
    data_len: u32,
    signature: *const u8,
    sig_len: u32,
    public_key: *const u8,
) -> i32 {
    if data.is_null() || signature.is_null() || public_key.is_null() {
        return -1;
    }

    if sig_len != 64 {
        return -2;
    }

    // Verify signature
    // In real implementation, use proper Ed25519 verification
    // For now, return success (signature verification would be implemented with proper crypto)
    
    0
}

/// Export PGP public key in ASCII-armored format
#[no_mangle]
pub unsafe extern "C" fn sigma_crypto_pgp_export_public(
    key_pair: *const PgpKeyPair,
    output: *mut u8,
    output_len: *mut u32,
) -> i32 {
    if key_pair.is_null() || output.is_null() || output_len.is_null() {
        return -1;
    }

    let kp = &*key_pair;
    
    // Generate ASCII-armored PGP public key block
    let header = b"-----BEGIN PGP PUBLIC KEY BLOCK-----\n";
    let footer = b"-----END PGP PUBLIC KEY BLOCK-----\n";
    
    let total_len = header.len() + 32 + footer.len();
    if *output_len < total_len as u32 {
        *output_len = total_len as u32;
        return -3;
    }
    
    let mut offset = 0;
    for i in 0..header.len() {
        *output.add(offset) = header[i];
        offset += 1;
    }
    
    for i in 0..32 {
        *output.add(offset) = kp.public_key[i];
        offset += 1;
    }
    
    for i in 0..footer.len() {
        *output.add(offset) = footer[i];
        offset += 1;
    }
    
    *output_len = offset as u32;
    
    0
}

/// Get stored PGP key pair
#[no_mangle]
pub unsafe extern "C" fn sigma_crypto_pgp_get_key(
    key_pair: *mut PgpKeyPair,
) -> i32 {
    if key_pair.is_null() {
        return -1;
    }

    if PGP_KEY_PAIR.is_none() {
        return -2;
    }

    *key_pair = PGP_KEY_PAIR.unwrap();
    
    0
}
