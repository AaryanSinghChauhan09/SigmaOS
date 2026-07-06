// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS dm-verity Block Verifier
//! Verifies block integrity against a Merkle tree of SHA-256 hashes.
//! no_std, no alloc. Hand-rolled SHA-256 (sovereign, no external crates).

#![no_std]
#![allow(dead_code)]

type SigmaU32  = u32;
type SigmaU64  = u64;
type SigmaI32  = i32;
type SigmaUsize= usize;

pub const DM_BLOCK_SIZE:    usize = 4096;
pub const DM_HASH_SIZE:     usize = 32;     // SHA-256
pub const DM_MAX_BLOCKS:    usize = 1024;

/// SHA-256 K constants
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

fn rotr32(x: u32, n: u32) -> u32 { x.rotate_right(n) }

/// Sovereign SHA-256 implementation over exactly one 64-byte block (padded by caller).
fn sha256_compress(state: &mut [u32; 8], block: &[u8; 64]) {
    let mut w = [0u32; 64];
    for i in 0..16 {
        w[i] = (block[4*i] as u32) << 24
             | (block[4*i+1] as u32) << 16
             | (block[4*i+2] as u32) << 8
             | (block[4*i+3] as u32);
    }
    for i in 16..64 {
        let s0 = rotr32(w[i-15], 7) ^ rotr32(w[i-15], 18) ^ (w[i-15] >> 3);
        let s1 = rotr32(w[i-2], 17) ^ rotr32(w[i-2], 19)  ^ (w[i-2]  >> 10);
        w[i] = w[i-16].wrapping_add(s0).wrapping_add(w[i-7]).wrapping_add(s1);
    }
    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = *state;
    for i in 0..64 {
        let s1 = rotr32(e, 6) ^ rotr32(e, 11) ^ rotr32(e, 25);
        let ch = (e & f) ^ (!e & g);
        let t1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
        let s0 = rotr32(a, 2) ^ rotr32(a, 13) ^ rotr32(a, 22);
        let maj = (a & b) ^ (a & c) ^ (b & c);
        let t2 = s0.wrapping_add(maj);
        h = g; g = f; f = e; e = d.wrapping_add(t1);
        d = c; c = b; b = a; a = t1.wrapping_add(t2);
    }
    state[0] = state[0].wrapping_add(a); state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c); state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e); state[5] = state[5].wrapping_add(f);
    state[6] = state[6].wrapping_add(g); state[7] = state[7].wrapping_add(h);
}

/// Hash 4096-byte block into 32-byte output.
pub fn hash_block(data: &[u8; DM_BLOCK_SIZE], out: &mut [u8; DM_HASH_SIZE]) {
    let mut state: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
        0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];
    // Process 64 full chunks from 4096 bytes
    let mut chunk = [0u8; 64];
    for c in 0..64 {
        chunk.copy_from_slice(&data[c*64..(c+1)*64]);
        sha256_compress(&mut state, &chunk);
    }
    // Final padding block (length: 4096 bytes = 32768 bits)
    let mut pad = [0u8; 64];
    pad[0] = 0x80;
    pad[56] = 0; pad[57] = 0; pad[58] = 0; pad[59] = 0;
    pad[60] = 0; pad[61] = 0; pad[62] = 0x80; pad[63] = 0x00; // 32768 in big-endian u64
    sha256_compress(&mut state, &pad);
    for i in 0..8 {
        out[4*i  ] = (state[i] >> 24) as u8;
        out[4*i+1] = (state[i] >> 16) as u8;
        out[4*i+2] = (state[i] >>  8) as u8;
        out[4*i+3] =  state[i]        as u8;
    }
}

/// Stored expected hashes (root Merkle tree leaf hashes)
static mut DM_HASH_TABLE: [[u8; DM_HASH_SIZE]; DM_MAX_BLOCKS] = [[0u8; DM_HASH_SIZE]; DM_MAX_BLOCKS];
static mut DM_BLOCK_COUNT: SigmaU32 = 0;

#[no_mangle]
pub unsafe extern "C" fn sigma_dmverity_init(block_count: SigmaU32) {
    DM_BLOCK_COUNT = block_count.min(DM_MAX_BLOCKS as SigmaU32);
}

/// Store the expected hash for a block index (set during image signing).
#[no_mangle]
pub unsafe extern "C" fn sigma_dmverity_set_hash(block_idx: SigmaU32, hash: *const u8) {
    if block_idx >= DM_BLOCK_COUNT || hash.is_null() { return; }
    for i in 0..DM_HASH_SIZE {
        DM_HASH_TABLE[block_idx as usize][i] = *hash.add(i);
    }
}

/// Verify a block. Returns 0 if hash matches, -1 on tampering/error.
#[no_mangle]
pub unsafe extern "C" fn sigma_dmverity_verify(block_idx: SigmaU32, data: *const u8) -> SigmaI32 {
    if block_idx >= DM_BLOCK_COUNT || data.is_null() { return -1; }
    let block = &*(data as *const [u8; DM_BLOCK_SIZE]);
    let mut computed = [0u8; DM_HASH_SIZE];
    hash_block(block, &mut computed);
    let expected = &DM_HASH_TABLE[block_idx as usize];
    for i in 0..DM_HASH_SIZE {
        if computed[i] != expected[i] { return -1; }
    }
    0
}
