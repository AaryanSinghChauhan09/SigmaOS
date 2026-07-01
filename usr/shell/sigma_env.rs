// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: Sigma Environment Manager (Rust, no_std)
//! =========================================================================
//! Replaces: usr/sigma_env.c
//!
//! OOP Design:
//!   - SigmaEnv struct: static key-value store (64 vars max).
//!   - Hand-rolled FNV-1a key hashing.
//! =========================================================================

pub type SigmaStatus = i32;
pub const SIGMA_OK:    SigmaStatus = 0;
pub const SIGMA_ERROR: SigmaStatus = -1;
type U32 = u32;
type U64 = u64;

const ENV_KEY_LEN: usize  = 64;
const ENV_VAL_LEN: usize  = 256;
const ENV_MAX:     usize  = 64;

// ── Entry ──────────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
struct EnvEntry {
    key_hash: U64,
    key:  [u8; ENV_KEY_LEN],
    val:  [u8; ENV_VAL_LEN],
    klen: usize,
    vlen: usize,
    used: bool,
}

impl EnvEntry {
    const fn empty() -> Self {
        EnvEntry {
            key_hash: 0,
            key:  [0u8; ENV_KEY_LEN],
            val:  [0u8; ENV_VAL_LEN],
            klen: 0,
            vlen: 0,
            used: false,
        }
    }
}

// ── SigmaEnv Struct ────────────────────────────────────────────────────────

pub struct SigmaEnv {
    table: [EnvEntry; ENV_MAX],
    count: usize,
}

impl SigmaEnv {
    pub const fn new() -> Self {
        const E: EnvEntry = EnvEntry::empty();
        SigmaEnv { table: [E; ENV_MAX], count: 0 }
    }

    fn fnv1a(b: &[u8]) -> U64 {
        let mut h: U64 = 0xcbf29ce484222325;
        let mut i = 0;
        while i < b.len() {
            h ^= b[i] as U64;
            h = h.wrapping_mul(0x100000001b3);
            i += 1;
        }
        h
    }

    fn copy(dst: &mut [u8], src: &[u8]) -> usize {
        let n = if src.len() < dst.len() { src.len() } else { dst.len() - 1 };
        let mut i = 0;
        while i < n { dst[i] = src[i]; i += 1; }
        n
    }

    pub fn set(&mut self, key: &[u8], val: &[u8]) -> SigmaStatus {
        let h = Self::fnv1a(key);
        // Update existing
        let mut i = 0;
        while i < ENV_MAX {
            if self.table[i].used && self.table[i].key_hash == h {
                self.table[i].vlen = Self::copy(&mut self.table[i].val, val);
                return SIGMA_OK;
            }
            i += 1;
        }
        // Insert new
        if self.count >= ENV_MAX { return SIGMA_ERROR; }
        let slot = self.count;
        self.table[slot].used     = true;
        self.table[slot].key_hash = h;
        self.table[slot].klen     = Self::copy(&mut self.table[slot].key, key);
        self.table[slot].vlen     = Self::copy(&mut self.table[slot].val, val);
        self.count += 1;
        SIGMA_OK
    }

    pub fn get<'a>(&'a self, key: &[u8]) -> Option<&'a [u8]> {
        let h = Self::fnv1a(key);
        let mut i = 0;
        while i < ENV_MAX {
            if self.table[i].used && self.table[i].key_hash == h {
                return Some(&self.table[i].val[..self.table[i].vlen]);
            }
            i += 1;
        }
        None
    }

    pub fn unset(&mut self, key: &[u8]) -> SigmaStatus {
        let h = Self::fnv1a(key);
        let mut i = 0;
        while i < ENV_MAX {
            if self.table[i].used && self.table[i].key_hash == h {
                self.table[i] = EnvEntry::empty();
                return SIGMA_OK;
            }
            i += 1;
        }
        SIGMA_ERROR
    }
}

// ── Global Singleton ───────────────────────────────────────────────────────
static mut G_ENV: SigmaEnv = SigmaEnv::new();

// ── C-ABI Exports ──────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_env_set(
    key: *const u8, klen: U32,
    val: *const u8, vlen: U32,
) -> SigmaStatus {
    let ks = core::slice::from_raw_parts(key, klen as usize);
    let vs = core::slice::from_raw_parts(val, vlen as usize);
    G_ENV.set(ks, vs)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_env_get(key: *const u8, klen: U32) -> *const u8 {
    let ks = core::slice::from_raw_parts(key, klen as usize);
    match G_ENV.get(ks) {
        Some(v) => v.as_ptr(),
        None    => core::ptr::null(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_env_unset(key: *const u8, klen: U32) -> SigmaStatus {
    let ks = core::slice::from_raw_parts(key, klen as usize);
    G_ENV.unset(ks)
}
