// SigmaOS — CryptFS Key Derivation (Issue #1009)
// Replaces the dummy `derive_key()` that returned 32 zero bytes.
// Implements PBKDF2-SHA256 + HKDF-SHA256 — no external crate dependencies.

#![allow(dead_code)]

// ─── SHA-256 (sovereign, no external deps) ───────────────────────────────────

const K: [u32; 64] = [
    0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,0x3956c25b,0x59f111f1,0x923f82a4,0xab1c5ed5,
    0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,
    0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc,0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,
    0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,0xc6e00bf3,0xd5a79147,0x06ca6351,0x14292967,
    0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,
    0xa2bfe8a1,0xa81a664b,0xc24b8b70,0xc76c51a3,0xd192e819,0xd6990624,0xf40e3585,0x106aa070,
    0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,0x682e6ff3,
    0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2,
];

#[inline(always)] fn rotr32(x: u32, n: u32) -> u32 { (x >> n) | (x << (32 - n)) }
#[inline(always)] fn ch(x: u32, y: u32, z: u32) -> u32 { (x & y) ^ (!x & z) }
#[inline(always)] fn maj(x: u32, y: u32, z: u32) -> u32 { (x & y) ^ (x & z) ^ (y & z) }
#[inline(always)] fn sigma0(x: u32) -> u32 { rotr32(x,2)  ^ rotr32(x,13) ^ rotr32(x,22) }
#[inline(always)] fn sigma1(x: u32) -> u32 { rotr32(x,6)  ^ rotr32(x,11) ^ rotr32(x,25) }
#[inline(always)] fn gamma0(x: u32) -> u32 { rotr32(x,7)  ^ rotr32(x,18) ^ (x >> 3) }
#[inline(always)] fn gamma1(x: u32) -> u32 { rotr32(x,17) ^ rotr32(x,19) ^ (x >> 10) }

pub fn sha256(msg: &[u8]) -> [u8; 32] {
    let mut h: [u32; 8] = [
        0x6a09e667,0xbb67ae85,0x3c6ef372,0xa54ff53a,
        0x510e527f,0x9b05688c,0x1f83d9ab,0x5be0cd19,
    ];

    // Padding
    let bit_len = (msg.len() as u64) * 8;
    let mut padded: [u8; 128] = [0u8; 128];
    let msg_len = msg.len();
    for (i, &b) in msg.iter().enumerate() { padded[i] = b; }
    padded[msg_len] = 0x80;
    // append 64-bit big-endian bit length at end of 64-byte block
    let pad_len = if msg_len < 56 { 64 } else { 128 };
    let len_off = pad_len - 8;
    let bits = bit_len.to_be_bytes();
    for (i, b) in bits.iter().enumerate() { padded[len_off + i] = *b; }

    let blocks = pad_len / 64;
    for blk in 0..blocks {
        let off = blk * 64;
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([padded[off+i*4], padded[off+i*4+1],
                                       padded[off+i*4+2], padded[off+i*4+3]]);
        }
        for i in 16..64 {
            w[i] = gamma1(w[i-2]).wrapping_add(w[i-7])
                   .wrapping_add(gamma0(w[i-15])).wrapping_add(w[i-16]);
        }
        let (mut a,mut b,mut c,mut d,mut e,mut f,mut g,mut hh) =
            (h[0],h[1],h[2],h[3],h[4],h[5],h[6],h[7]);

        for i in 0..64 {
            let t1 = hh.wrapping_add(sigma1(e)).wrapping_add(ch(e,f,g))
                       .wrapping_add(K[i]).wrapping_add(w[i]);
            let t2 = sigma0(a).wrapping_add(maj(a,b,c));
            hh = g; g = f; f = e; e = d.wrapping_add(t1);
            d = c; c = b; b = a; a = t1.wrapping_add(t2);
        }
        h[0]=h[0].wrapping_add(a); h[1]=h[1].wrapping_add(b);
        h[2]=h[2].wrapping_add(c); h[3]=h[3].wrapping_add(d);
        h[4]=h[4].wrapping_add(e); h[5]=h[5].wrapping_add(f);
        h[6]=h[6].wrapping_add(g); h[7]=h[7].wrapping_add(hh);
    }

    let mut out = [0u8; 32];
    for (i, &v) in h.iter().enumerate() {
        let b = v.to_be_bytes();
        out[i*4..i*4+4].copy_from_slice(&b);
    }
    out
}

// ─── HMAC-SHA256 ─────────────────────────────────────────────────────────────

pub fn hmac_sha256(key: &[u8], msg: &[u8]) -> [u8; 32] {
    let mut k_padded = [0u8; 64];
    if key.len() <= 64 {
        k_padded[..key.len()].copy_from_slice(key);
    } else {
        let hk = sha256(key);
        k_padded[..32].copy_from_slice(&hk);
    }
    let mut i_key = [0u8; 64];
    let mut o_key = [0u8; 64];
    for i in 0..64 {
        i_key[i] = k_padded[i] ^ 0x36;
        o_key[i] = k_padded[i] ^ 0x5c;
    }
    // inner = SHA256(i_key || msg)
    let mut inner_input = [0u8; 64 + 128];
    inner_input[..64].copy_from_slice(&i_key);
    let msg_copy_len = msg.len().min(128);
    inner_input[64..64 + msg_copy_len].copy_from_slice(&msg[..msg_copy_len]);
    let inner_hash = sha256(&inner_input[..64 + msg_copy_len]);

    // outer = SHA256(o_key || inner_hash)
    let mut outer_input = [0u8; 96];
    outer_input[..64].copy_from_slice(&o_key);
    outer_input[64..].copy_from_slice(&inner_hash);
    sha256(&outer_input)
}

// ─── HKDF-SHA256 (RFC 5869) ──────────────────────────────────────────────────

pub fn hkdf_extract(salt: &[u8], ikm: &[u8]) -> [u8; 32] {
    hmac_sha256(salt, ikm)
}

pub fn hkdf_expand(prk: &[u8; 32], info: &[u8], out: &mut [u8]) {
    let len = out.len();
    assert!(len <= 32 * 255, "HKDF output too long");
    let mut t = [0u8; 32];
    let mut pos = 0usize;
    let mut counter = 1u8;
    let mut prev_len = 0usize;

    while pos < len {
        // input = T(i-1) || info || counter
        let mut buf = [0u8; 32 + 64 + 1];
        buf[..prev_len].copy_from_slice(&t[..prev_len]);
        let info_copy = info.len().min(64);
        buf[prev_len..prev_len + info_copy].copy_from_slice(&info[..info_copy]);
        buf[prev_len + info_copy] = counter;
        t = hmac_sha256(prk, &buf[..prev_len + info_copy + 1]);
        prev_len = 32;
        counter += 1;

        let copy = (len - pos).min(32);
        out[pos..pos + copy].copy_from_slice(&t[..copy]);
        pos += copy;
    }
}

// ─── PBKDF2-SHA256 (RFC 2898) ────────────────────────────────────────────────

pub fn pbkdf2_sha256(password: &[u8], salt: &[u8], iterations: u32, out: &mut [u8]) {
    let dk_len = out.len();
    let blocks = (dk_len + 31) / 32;
    let mut pos = 0usize;

    for block_idx in 1u32..=blocks as u32 {
        // U1 = HMAC(password, salt || INT(i))
        let mut s = [0u8; 128];
        let salt_len = salt.len().min(124);
        s[..salt_len].copy_from_slice(&salt[..salt_len]);
        let i_be = block_idx.to_be_bytes();
        s[salt_len..salt_len+4].copy_from_slice(&i_be);
        let mut u = hmac_sha256(password, &s[..salt_len + 4]);
        let mut xor = u;

        for _ in 1..iterations {
            u = hmac_sha256(password, &u);
            for j in 0..32 { xor[j] ^= u[j]; }
        }

        let copy = (dk_len - pos).min(32);
        out[pos..pos + copy].copy_from_slice(&xor[..copy]);
        pos += copy;
    }
}

// ─── CryptFS Key Derivation Entry Point ──────────────────────────────────────

/// Derive a 32-byte AES-256 key from passphrase + salt.
/// Replaces the previous stub that returned [0u8; 32].
/// Uses PBKDF2-SHA256 with 100,000 iterations as per NIST SP 800-132.
pub fn derive_key(passphrase: &[u8], salt: &[u8; 32]) -> [u8; 32] {
    let mut key = [0u8; 32];
    pbkdf2_sha256(passphrase, salt, 100_000, &mut key);
    key
}

/// Derive a volume encryption key using HKDF for sub-key derivation.
/// Used for per-file and per-directory keys inside CryptFS.
pub fn derive_subkey(master_key: &[u8; 32], context: &[u8]) -> [u8; 32] {
    let salt = [0u8; 32]; // context-specific salt can be added
    let prk = hkdf_extract(&salt, master_key);
    let mut subkey = [0u8; 32];
    hkdf_expand(&prk, context, &mut subkey);
    subkey
}

/// Verify a stored key derivation (for unlock validation).
pub fn verify_key(passphrase: &[u8], salt: &[u8; 32], expected: &[u8; 32]) -> bool {
    let derived = derive_key(passphrase, salt);
    // Constant-time comparison to prevent timing attacks
    let mut diff = 0u8;
    for i in 0..32 {
        diff |= derived[i] ^ expected[i];
    }
    diff == 0
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_empty() {
        // SHA256("") = e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
        let h = sha256(b"");
        assert_eq!(h[0], 0xe3);
        assert_eq!(h[1], 0xb0);
    }

    #[test]
    fn derive_key_nonzero() {
        let pw = b"mysecretpassword";
        let salt = [0x42u8; 32];
        let k = derive_key(pw, &salt);
        // Must not be all zeros
        assert!(k.iter().any(|&b| b != 0));
    }

    #[test]
    fn verify_key_correct() {
        let pw = b"test";
        let salt = [0x01u8; 32];
        let k = derive_key(pw, &salt);
        assert!(verify_key(pw, &salt, &k));
    }

    #[test]
    fn verify_key_wrong() {
        let pw = b"test";
        let salt = [0x01u8; 32];
        let k = derive_key(pw, &salt);
        assert!(!verify_key(b"wrong", &salt, &k));
    }
}
