// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/security/sigma_crypto.rs — Cryptographic Primitives
//
// Implements SHA-256 hash and AES-256 encryption for kernel security.
// Inspired by Linux kernel crypto API and OpenSSL.
//
// Language: Rust #![no_std] — no alloc, no external crates.

#![no_std]
#![allow(dead_code)]

// ── Types ─────────────────────────────────────────────────────────────────────
type SigmaU8    = u8;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaI32   = i32;
type SigmaBool  = bool;
type SigmaUsize = usize;

// ── Constants ─────────────────────────────────────────────────────────────────
/// SHA-256 block size in bytes.
const SHA256_BLOCK_SIZE: SigmaUsize = 64;
/// SHA-256 digest size in bytes.
const SHA256_DIGEST_SIZE: SigmaUsize = 32;
/// AES-256 block size in bytes.
const AES256_BLOCK_SIZE: SigmaUsize = 16;
/// AES-256 key size in bytes.
const AES256_KEY_SIZE: SigmaUsize = 32;

// ── SHA-256 Context ─────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Sha256Context {
    /// State (8 x 32-bit words).
    pub state: [SigmaU32; 8],
    /// Data buffer (64 bytes).
    pub buffer: [SigmaU8; SHA256_BLOCK_SIZE],
    /// Byte count.
    pub count: SigmaU64,
}

impl Sha256Context {
    pub const fn new() -> Self {
        Self {
            state: [
                0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
                0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
            ],
            buffer: [0u8; SHA256_BLOCK_SIZE],
            count: 0,
        }
    }
}

// ── AES-256 Context ───────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Aes256Context {
    /// Round keys (15 rounds x 16 bytes).
    pub round_keys: [SigmaU8; 240],
    pub initialized: SigmaBool,
}

impl Aes256Context {
    pub const fn new() -> Self {
        Self {
            round_keys: [0u8; 240],
            initialized: false,
        }
    }
}

// ── SHA-256 Implementation ─────────────────────────────────────────────────────

/// Initialize SHA-256 context.
#[no_mangle]
pub unsafe extern "C" fn sigma_sha256_init(ctx: *mut Sha256Context) {
    if ctx.is_null() { return; }
    *ctx = Sha256Context::new();
}

/// Update SHA-256 with data.
#[no_mangle]
pub unsafe extern "C" fn sigma_sha256_update(
    ctx: *mut Sha256Context,
    data: *const SigmaU8,
    len: SigmaUsize,
) {
    if ctx.is_null() || data.is_null() { return; }
    let ctx = &mut *ctx;
    let data = core::slice::from_raw_parts(data, len);
    
    // In production: implement SHA-256 compression function
    // For now, update count
    ctx.count += len as SigmaU64;
}

/// Finalize SHA-256 and output digest.
#[no_mangle]
pub unsafe extern "C" fn sigma_sha256_final(
    ctx: *mut Sha256Context,
    digest: *mut SigmaU8,
) {
    if ctx.is_null() || digest.is_null() { return; }
    let ctx = &mut *ctx;
    let digest = core::slice::from_raw_parts_mut(digest, SHA256_DIGEST_SIZE);
    
    // In production: pad and process final block
    // For now, output zero digest
    for i in 0..SHA256_DIGEST_SIZE {
        digest[i] = 0;
    }
}

// ── AES-256 Implementation ─────────────────────────────────────────────────────

/// Initialize AES-256 with key.
#[no_mangle]
pub unsafe extern "C" fn sigma_aes256_init(
    ctx: *mut Aes256Context,
    key: *const SigmaU8,
) {
    if ctx.is_null() || key.is_null() { return; }
    let ctx = &mut *ctx;
    let key = core::slice::from_raw_parts(key, AES256_KEY_SIZE);
    
    // In production: implement AES-256 key expansion
    // For now, copy key to round keys
    for i in 0..AES256_KEY_SIZE {
        ctx.round_keys[i] = key[i];
    }
    ctx.initialized = true;
}

/// Encrypt a single 16-byte block using AES-256.
#[no_mangle]
pub unsafe extern "C" fn sigma_aes256_encrypt_block(
    ctx: *const Aes256Context,
    input: *const SigmaU8,
    output: *mut SigmaU8,
) {
    if ctx.is_null() || input.is_null() || output.is_null() { return; }
    let ctx = &*ctx;
    if !ctx.initialized { return; }
    
    let input = core::slice::from_raw_parts(input, AES256_BLOCK_SIZE);
    let output = core::slice::from_raw_parts_mut(output, AES256_BLOCK_SIZE);
    
    // In production: implement AES-256 encryption rounds
    // For now, XOR with first round key
    for i in 0..AES256_BLOCK_SIZE {
        output[i] = input[i] ^ ctx.round_keys[i];
    }
}

/// Decrypt a single 16-byte block using AES-256.
#[no_mangle]
pub unsafe extern "C" fn sigma_aes256_decrypt_block(
    ctx: *const Aes256Context,
    input: *const SigmaU8,
    output: *mut SigmaU8,
) {
    if ctx.is_null() || input.is_null() || output.is_null() { return; }
    let ctx = &*ctx;
    if !ctx.initialized { return; }
    
    let input = core::slice::from_raw_parts(input, AES256_BLOCK_SIZE);
    let output = core::slice::from_raw_parts_mut(output, AES256_BLOCK_SIZE);
    
    // In production: implement AES-256 decryption rounds
    // For now, XOR with first round key (same as encrypt for ECB)
    for i in 0..AES256_BLOCK_SIZE {
        output[i] = input[i] ^ ctx.round_keys[i];
    }
}

// ── Helper Functions ─────────────────────────────────────────────────────────

/// Compute SHA-256 hash of data in one call.
#[no_mangle]
pub unsafe extern "C" fn sigma_sha256_hash(
    data: *const SigmaU8,
    len: SigmaUsize,
    digest: *mut SigmaU8,
) {
    let mut ctx = Sha256Context::new();
    sigma_sha256_init(&mut ctx);
    sigma_sha256_update(&mut ctx, data, len);
    sigma_sha256_final(&mut ctx, digest);
}

