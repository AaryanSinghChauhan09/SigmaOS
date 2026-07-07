//! SigmaOS — ChaCha20 Native Implementation
//! Pure no_std, zero-dependency stream cipher for kernel crypto (storage/network).

#![no_std]
#![allow(dead_code)]

type U8  = u8;
type U32 = u32;
type Usize = usize;

#[inline(always)]
fn rotl32(x: U32, n: U32) -> U32 {
    (x << n) | (x >> (32 - n))
}

#[inline(always)]
fn quarter_round(a: &mut U32, b: &mut U32, c: &mut U32, d: &mut U32) {
    *a = a.wrapping_add(*b); *d ^= *a; *d = rotl32(*d, 16);
    *c = c.wrapping_add(*d); *b ^= *c; *b = rotl32(*b, 12);
    *a = a.wrapping_add(*b); *d ^= *a; *d = rotl32(*d, 8);
    *c = c.wrapping_add(*d); *b ^= *c; *b = rotl32(*b, 7);
}

/// ChaCha20 block function.
/// Takes a 16-word state and generates a 64-byte keystream block.
pub fn chacha20_block(state: &[U32; 16], out: &mut [U8; 64]) {
    let mut x = *state;

    for _ in 0..10 {
        // Odd rounds
        quarter_round(&mut x[0], &mut x[4], &mut x[8],  &mut x[12]);
        quarter_round(&mut x[1], &mut x[5], &mut x[9],  &mut x[13]);
        quarter_round(&mut x[2], &mut x[6], &mut x[10], &mut x[14]);
        quarter_round(&mut x[3], &mut x[7], &mut x[11], &mut x[15]);
        // Even rounds
        quarter_round(&mut x[0], &mut x[5], &mut x[10], &mut x[15]);
        quarter_round(&mut x[1], &mut x[6], &mut x[11], &mut x[12]);
        quarter_round(&mut x[2], &mut x[7], &mut x[8],  &mut x[13]);
        quarter_round(&mut x[3], &mut x[4], &mut x[9],  &mut x[14]);
    }

    for i in 0..16 {
        x[i] = x[i].wrapping_add(state[i]);
        let bytes = x[i].to_le_bytes();
        out[i * 4]     = bytes[0];
        out[i * 4 + 1] = bytes[1];
        out[i * 4 + 2] = bytes[2];
        out[i * 4 + 3] = bytes[3];
    }
}

/// Initialize ChaCha20 state with 256-bit key and 96-bit nonce.
pub fn chacha20_init(key: &[U8; 32], nonce: &[U8; 12], counter: U32) -> [U32; 16] {
    let mut state = [0u32; 16];
    
    // "expand 32-byte k"
    state[0] = 0x61707865;
    state[1] = 0x3320646e;
    state[2] = 0x79622d32;
    state[3] = 0x6b206574;

    for i in 0..8 {
        state[4 + i] = U32::from_le_bytes([
            key[i * 4], key[i * 4 + 1], key[i * 4 + 2], key[i * 4 + 3]
        ]);
    }

    state[12] = counter;
    for i in 0..3 {
        state[13 + i] = U32::from_le_bytes([
            nonce[i * 4], nonce[i * 4 + 1], nonce[i * 4 + 2], nonce[i * 4 + 3]
        ]);
    }
    state
}

/// Encrypt or decrypt data in place using ChaCha20.
#[no_mangle]
pub unsafe extern "C" fn chacha20_crypt(
    key: *const U8, nonce: *const U8, counter: U32,
    data: *mut U8, len: Usize
) {
    if key.is_null() || nonce.is_null() || data.is_null() || len == 0 { return; }
    
    let key_slice = core::slice::from_raw_parts(key, 32);
    let mut key_arr = [0u8; 32];
    key_arr.copy_from_slice(key_slice);

    let nonce_slice = core::slice::from_raw_parts(nonce, 12);
    let mut nonce_arr = [0u8; 12];
    nonce_arr.copy_from_slice(nonce_slice);

    let data_slice = core::slice::from_raw_parts_mut(data, len);
    
    let mut state = chacha20_init(&key_arr, &nonce_arr, counter);
    let mut block = [0u8; 64];

    let mut i = 0;
    while i < len {
        chacha20_block(&state, &mut block);
        state[12] = state[12].wrapping_add(1);

        let take = core::cmp::min(64, len - i);
        for j in 0..take {
            data_slice[i + j] ^= block[j];
        }
        i += take;
    }
}
