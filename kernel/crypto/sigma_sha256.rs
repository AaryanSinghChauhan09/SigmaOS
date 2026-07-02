// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/crypto/sigma_sha256.rs — SHA-256 (cleanroom, no_std)
// Language: Rust #![no_std] — no libc, no third-party crates
// Pattern: OOP via Sha256 struct (streaming hasher)

#![no_std]

const K: [u32; 64] = [
    0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,
    0x3956c25b,0x59f111f1,0x923f82a4,0xab1c5ed5,
    0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,
    0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,
    0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc,
    0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,
    0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,
    0xc6e00bf3,0xd5a79147,0x06ca6351,0x14292967,
    0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,
    0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,
    0xa2bfe8a1,0xa81a664b,0xc24b8b70,0xc76c51a3,
    0xd192e819,0xd6990624,0xf40e3585,0x106aa070,
    0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,
    0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,0x682e6ff3,
    0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,
    0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2,
];

const H0: [u32; 8] = [
    0x6a09e667,0xbb67ae85,0x3c6ef372,0xa54ff53a,
    0x510e527f,0x9b05688c,0x1f83d9ab,0x5be0cd19,
];

#[inline(always)] fn rotr(x: u32, n: u32) -> u32 { x.rotate_right(n) }
#[inline(always)] fn ch (e:u32,f:u32,g:u32) -> u32 { (e&f)^((!e)&g) }
#[inline(always)] fn maj(a:u32,b:u32,c:u32) -> u32 { (a&b)^(a&c)^(b&c) }
#[inline(always)] fn ep0(a:u32) -> u32 { rotr(a,2)^rotr(a,13)^rotr(a,22) }
#[inline(always)] fn ep1(e:u32) -> u32 { rotr(e,6)^rotr(e,11)^rotr(e,25) }
#[inline(always)] fn sg0(x:u32) -> u32 { rotr(x,7)^rotr(x,18)^(x>>3) }
#[inline(always)] fn sg1(x:u32) -> u32 { rotr(x,17)^rotr(x,19)^(x>>10) }

pub struct Sha256 {
    state:  [u32; 8],
    buf:    [u8; 64],
    buf_len: usize,
    total:  u64,
}

impl Sha256 {
    pub const fn new() -> Self {
        Self { state: H0, buf: [0u8; 64], buf_len: 0, total: 0 }
    }

    pub fn update(&mut self, data: &[u8]) {
        let mut i = 0;
        while i < data.len() {
            let space = 64 - self.buf_len;
            let take  = space.min(data.len() - i);
            self.buf[self.buf_len..self.buf_len+take].copy_from_slice(&data[i..i+take]);
            self.buf_len += take;
            self.total   += take as u64;
            i            += take;
            if self.buf_len == 64 {
                let block = self.buf;
                Self::compress(&mut self.state, &block);
                self.buf_len = 0;
            }
        }
    }

    pub fn finalize(mut self) -> [u8; 32] {
        let bit_len = self.total * 8;
        self.buf[self.buf_len] = 0x80;
        self.buf_len += 1;
        if self.buf_len > 56 {
            for b in &mut self.buf[self.buf_len..64] { *b = 0; }
            let block = self.buf;
            Self::compress(&mut self.state, &block);
            self.buf_len = 0;
        }
        for b in &mut self.buf[self.buf_len..56] { *b = 0; }
        let bl = bit_len.to_be_bytes();
        self.buf[56..64].copy_from_slice(&bl);
        let block = self.buf;
        Self::compress(&mut self.state, &block);

        let mut out = [0u8; 32];
        for (i, w) in self.state.iter().enumerate() {
            out[i*4..(i+1)*4].copy_from_slice(&w.to_be_bytes());
        }
        out
    }

    fn compress(state: &mut [u32; 8], block: &[u8; 64]) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([block[i*4],block[i*4+1],block[i*4+2],block[i*4+3]]);
        }
        for i in 16..64 { w[i] = sg1(w[i-2]).wrapping_add(w[i-7]).wrapping_add(sg0(w[i-15])).wrapping_add(w[i-16]); }

        let [mut a,mut b,mut c,mut d,mut e,mut f,mut g,mut h] = *state;
        for i in 0..64 {
            let t1 = h.wrapping_add(ep1(e)).wrapping_add(ch(e,f,g)).wrapping_add(K[i]).wrapping_add(w[i]);
            let t2 = ep0(a).wrapping_add(maj(a,b,c));
            h=g; g=f; f=e; e=d.wrapping_add(t1);
            d=c; c=b; b=a; a=t1.wrapping_add(t2);
        }
        state[0]=state[0].wrapping_add(a); state[1]=state[1].wrapping_add(b);
        state[2]=state[2].wrapping_add(c); state[3]=state[3].wrapping_add(d);
        state[4]=state[4].wrapping_add(e); state[5]=state[5].wrapping_add(f);
        state[6]=state[6].wrapping_add(g); state[7]=state[7].wrapping_add(h);
    }
}

/// One-shot hash
pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(data);
    h.finalize()
}

/// HMAC-SHA256
pub fn hmac_sha256(key: &[u8], data: &[u8]) -> [u8; 32] {
    let mut k = [0u8; 64];
    if key.len() > 64 { let h = sha256(key); k[..32].copy_from_slice(&h); }
    else { k[..key.len()].copy_from_slice(key); }
    let mut ipad = [0u8; 64]; let mut opad = [0u8; 64];
    for i in 0..64 { ipad[i] = k[i] ^ 0x36; opad[i] = k[i] ^ 0x5C; }
    let mut inner = Sha256::new();
    inner.update(&ipad); inner.update(data);
    let inner_hash = inner.finalize();
    let mut outer = Sha256::new();
    outer.update(&opad); outer.update(&inner_hash);
    outer.finalize()
}
