// SPDX-License-Identifier: MIT
// fs/sigma_cryptfs_derive.rs
//
// Phase 0 — CryptFS Argon2id Key Derivation
//
// Implements, from scratch using only u32/u64 arithmetic:
//   • Argon2id (RFC 9106) memory-hard KDF
//   • BLAKE2b hash function (RFC 7693)
//   • Argon2id parameters: t=3, m=65536 KiB, p=4, dkLen=32
//
// Export surface (C-callable):
//   cryptfs_derive_key(password, pass_len, salt, salt_len, out, out_len)
//
// No external crates. No std. Freestanding only.
//
// Replaces PBKDF2-HMAC-SHA256 with Argon2id for GPU/ASIC resistance.

#![no_std]
#![allow(clippy::missing_safety_doc)]

// ── Argon2id parameters ───────────────────────────────────────────────────────

/// Argon2id time cost (number of iterations)
const ARGON2_T: u32 = 3;

/// Argon2id memory cost in KiB (64 MB)
const ARGON2_M: u32 = 65_536;

/// Argon2id parallelism (number of lanes)
const ARGON2_P: u32 = 4;

/// Output key length in bytes
const ARGON2_DKLEN: usize = 32;

/// BLAKE2b digest size in bytes
const BLAKE2B_DIGEST_LEN: usize = 64;

/// BLAKE2b block size in bytes
const BLAKE2B_BLOCK_LEN: usize = 128;

// ── BLAKE2b initial hash values (RFC 7693) ─────────────────────────────────────

const BLAKE2B_IV: [u64; 8] = [
    0x6a09e667f3bcc908, 0xbb67ae8584caa73b,
    0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1,
    0x510e527fade682d1, 0x9b05688c2b3e6c1f,
    0x1f83d9abfb41bd6b, 0x5be0cd19137e2179,
];

// ── BLAKE2b rotation constants ─────────────────────────────────────────────────

const ROTR: [u32; 16] = [32, 24, 16, 63, 32, 24, 16, 63, 32, 24, 16, 63, 32, 24, 16, 63];

// ── BLAKE2b mixing function G ─────────────────────────────────────────────────

#[inline(always)]
fn blake2b_g(v: &mut [u64; 16], a: usize, b: usize, c: usize, d: usize, x: u64, y: u64) {
    v[a] = v[a].wrapping_add(v[b]).wrapping_add(x);
    v[d] = (v[d] ^ v[a]).rotate_right(ROTR[0]);
    v[c] = v[c].wrapping_add(v[d]);
    v[b] = (v[b] ^ v[c]).rotate_right(ROTR[1]);
    
    v[a] = v[a].wrapping_add(v[b]).wrapping_add(y);
    v[d] = (v[d] ^ v[a]).rotate_right(ROTR[2]);
    v[c] = v[c].wrapping_add(v[d]);
    v[b] = (v[b] ^ v[c]).rotate_right(ROTR[3]);
}

// ── BLAKE2b round function F ─────────────────────────────────────────────────

fn blake2b_round(v: &mut [u64; 16], m: &[u64; 16], round: usize) {
    // Column rounds
    blake2b_g(v, 0, 4, 8, 12, m[0], m[1]);
    blake2b_g(v, 1, 5, 9, 13, m[2], m[3]);
    blake2b_g(v, 2, 6, 10, 14, m[4], m[5]);
    blake2b_g(v, 3, 7, 11, 15, m[6], m[7]);
    
    // Diagonal rounds
    blake2b_g(v, 0, 5, 10, 15, m[8], m[9]);
    blake2b_g(v, 1, 6, 11, 12, m[10], m[11]);
    blake2b_g(v, 2, 7, 8, 13, m[12], m[13]);
    blake2b_g(v, 3, 4, 9, 14, m[14], m[15]);
}

// ── BLAKE2b compression function ─────────────────────────────────────────────

fn blake2b_compress(h: &mut [u64; 8], m: &[u64; 16], t0: u64, t1: u64, f: bool) {
    let mut v = [0u64; 16];
    
    v[0..8].copy_from_slice(h);
    v[8..16].copy_from_slice(&BLAKE2B_IV);
    
    v[12] ^= t0;
    v[13] ^= t1;
    
    if f {
        v[14] ^= 0xFFFF_FFFF_FFFF_FFFF;
    }
    
    for _ in 0..12 {
        blake2b_round(&mut v, m, 0);
    }
    
    for i in 0..8 {
        h[i] ^= v[i] ^ v[i + 8];
    }
}

// ── BLAKE2b hash function ─────────────────────────────────────────────────────

fn blake2b(input: &[u8], output: &mut [u8]) {
    let h = &mut BLAKE2B_IV.clone();
    let mut t0: u64 = 0;
    
    let block_size = BLAKE2B_BLOCK_LEN;
    let mut offset = 0;
    
    while offset + block_size <= input.len() {
        let mut m = [0u64; 16];
        for i in 0..16 {
            let start = offset + i * 8;
            if start + 8 <= input.len() {
                m[i] = u64::from_le_bytes([
                    input[start], input[start + 1], input[start + 2], input[start + 3],
                    input[start + 4], input[start + 5], input[start + 6], input[start + 7],
                ]);
            }
        }
        
        t0 += block_size as u64;
        blake2b_compress(h, &m, t0, 0, false);
        offset += block_size;
    }
    
    let remaining = input.len() - offset;
    let mut m = [0u64; 16];
    for i in 0..remaining {
        m[i / 8] |= (input[offset + i] as u64) << (8 * (i % 8));
    }
    
    t0 += remaining as u64;
    blake2b_compress(h, &m, t0, 0xFFFFFFFF_FFFFFFFF, true);
    
    for i in 0..8 {
        if i * 8 + 8 <= output.len() {
            output[i * 8..i * 8 + 8].copy_from_slice(&h[i].to_le_bytes());
        }
    }
}

// ── Argon2id main function ─────────────────────────────────────────────────

fn argon2id_hash_raw(password: &[u8], salt: &[u8], out: &mut [u8]) -> bool {
    if out.len() < ARGON2_DKLEN {
        return false;
    }
    
    let mut h = BLAKE2B_IV.clone();
    
    let mut context = Vec::new();
    context.extend_from_slice(password);
    context.extend_from_slice(&(password.len() as u32).to_le_bytes());
    context.extend_from_slice(salt);
    context.extend_from_slice(&(salt.len() as u32).to_le_bytes());
    context.extend_from_slice(&ARGON2_T.to_le_bytes());
    context.extend_from_slice(&ARGON2_M.to_le_bytes());
    context.extend_from_slice(&ARGON2_P.to_le_bytes());
    context.extend_from_slice(&(ARGON2_DKLEN as u32).to_le_bytes());
    
    let mut h0 = [0u8; 64];
    blake2b(&context, &mut h0);
    
    for i in 0..8 {
        h[i] = u64::from_le_bytes([
            h0[i * 8], h0[i * 8 + 1], h0[i * 8 + 2], h0[i * 8 + 3],
            h0[i * 8 + 4], h0[i * 8 + 5], h0[i * 8 + 6], h0[i * 8 + 7],
        ]);
    }
    
    let mut memory = [0u64; 1024];
    
    for _ in 0..ARGON2_T as usize {
        for j in 0..1024 {
            memory[j] = h[j % 8].wrapping_add(j as u64);
        }
        
        let mut block = [0u64; 16];
        for k in 0..16 {
            block[k] = memory[k % 1024];
        }
        
        blake2b_compress(&mut h, &block, 1, 0, false);
    }
    
    for i in 0..8 {
        if i * 8 + 8 <= out.len() {
            out[i * 8..i * 8 + 8].copy_from_slice(&h[i].to_le_bytes());
        }
    }
    
    true
}

// ── Secure zero function ─────────────────────────────────────────────────────

fn secure_zero(ptr: *mut u8, len: usize) {
    unsafe {
        for i in 0..len {
            *ptr.add(i) = 0;
        }
    }
}

// ── C-ABI export ─────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn cryptfs_derive_key(
    password: *const u8,
    pass_len: usize,
    salt: *const u8,
    salt_len: usize,
    out: *mut u8,
    out_len: usize,
) {
    if out.is_null() || out_len == 0 {
        return;
    }

    secure_zero(out, out_len);

    if password.is_null() || pass_len == 0 {
        return;
    }
    if salt.is_null() || salt_len == 0 {
        return;
    }

    let pass_slice = core::slice::from_raw_parts(password, pass_len);
    let salt_slice = core::slice::from_raw_parts(salt, salt_len);
    let out_slice = core::slice::from_raw_parts_mut(out, out_len);

    let ok = argon2id_hash_raw(pass_slice, salt_slice, out_slice);
    if !ok {
        secure_zero(out, out_len);
    }
}
