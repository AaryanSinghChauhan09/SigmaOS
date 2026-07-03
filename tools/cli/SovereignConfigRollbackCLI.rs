// SPDX-License-Identifier: GPL-2.0-or-later
//! SovereignConfigRollbackCLI — declarative config rollback manager
//! Migrated from C/C++ to Rust, no_std, no alloc, no external crates.
//! Manages generations of sigma.toml config and allows atomic rollback.

#![no_std]
#![allow(dead_code)]

// ─── Primitive Types ─────────────────────────────────────────────────────────
type SigmaU8   = u8;
type SigmaU16  = u16;
type SigmaU32  = u32;
type SigmaU64  = u64;
type SigmaI32  = i32;
type SigmaI64  = i64;
type SigmaBool = bool;
type SigmaUsize = usize;

// ─── Constants ────────────────────────────────────────────────────────────────
const CONFIG_KEY_LEN:   usize = 64;
const CONFIG_VAL_LEN:   usize = 128;
const MAX_ENTRIES:      usize = 64;
const MAX_GENERATIONS:  usize = 8;
const GEN_LABEL_LEN:    usize = 64;

// ─── Structs ─────────────────────────────────────────────────────────────────

/// ConfigEntry — a single key=value pair in a generation snapshot
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ConfigEntry {
    pub key:    [u8; CONFIG_KEY_LEN],
    pub value:  [u8; CONFIG_VAL_LEN],
    pub active: SigmaBool,
}

impl ConfigEntry {
    pub const fn empty() -> Self {
        Self {
            key:    [0u8; CONFIG_KEY_LEN],
            value:  [0u8; CONFIG_VAL_LEN],
            active: false,
        }
    }
}

/// Generation — one immutable snapshot of the configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Generation {
    pub gen_id:      SigmaU32,
    pub timestamp:   SigmaU64,
    pub entry_count: SigmaU32,
    pub label:       [u8; GEN_LABEL_LEN],
    pub entries:     [ConfigEntry; MAX_ENTRIES],
    pub valid:       SigmaBool,
}

impl Generation {
    pub const fn empty() -> Self {
        Self {
            gen_id:      0,
            timestamp:   0,
            entry_count: 0,
            label:       [0u8; GEN_LABEL_LEN],
            entries:     [ConfigEntry {
                key:    [0u8; CONFIG_KEY_LEN],
                value:  [0u8; CONFIG_VAL_LEN],
                active: false,
            }; MAX_ENTRIES],
            valid: false,
        }
    }
}

/// SovereignConfigRollback — manages generation ring of config snapshots
#[repr(C)]
pub struct SovereignConfigRollback {
    pub generations:    [Generation; MAX_GENERATIONS],
    pub gen_count:      SigmaU32,
    pub active_gen:     SigmaU32,   // index of current active generation
    pub initialized:    SigmaBool,
}

impl SovereignConfigRollback {
    pub const fn new() -> Self {
        Self {
            generations: [Generation {
                gen_id: 0, timestamp: 0, entry_count: 0,
                label: [0u8; GEN_LABEL_LEN],
                entries: [ConfigEntry { key: [0u8; CONFIG_KEY_LEN], value: [0u8; CONFIG_VAL_LEN], active: false }; MAX_ENTRIES],
                valid: false,
            }; MAX_GENERATIONS],
            gen_count:   0,
            active_gen:  0,
            initialized: false,
        }
    }

    /// strncpy helper (no_std, no alloc)
    pub unsafe fn safe_copy(dst: *mut u8, src: *const u8, n: usize) {
        let mut i = 0;
        while i < n {
            let b = *src.add(i);
            *dst.add(i) = b;
            if b == 0 { break; }
            i += 1;
        }
    }

    /// strcmp helper
    pub unsafe fn key_eq(a: *const u8, b: *const u8, n: usize) -> SigmaBool {
        for i in 0..n {
            if *a.add(i) != *b.add(i) { return false; }
            if *a.add(i) == 0 { return true; }
        }
        true
    }

    /// Initialise the rollback engine
    pub unsafe fn init(&mut self) {
        self.gen_count   = 0;
        self.active_gen  = 0;
        self.initialized = true;
    }

    /// Set a key=value in the current in-progress generation
    pub unsafe fn set(&mut self, key: *const u8, val: *const u8) -> SigmaBool {
        if !self.initialized || self.active_gen as usize >= MAX_GENERATIONS { return false; }
        let gen = &mut self.generations[self.active_gen as usize];
        // Search for existing key
        for i in 0..gen.entry_count as usize {
            if Self::key_eq(gen.entries[i].key.as_ptr(), key, CONFIG_KEY_LEN) {
                Self::safe_copy(gen.entries[i].value.as_mut_ptr(), val, CONFIG_VAL_LEN);
                return true;
            }
        }
        // New entry
        if gen.entry_count as usize >= MAX_ENTRIES { return false; }
        let idx = gen.entry_count as usize;
        Self::safe_copy(gen.entries[idx].key.as_mut_ptr(), key, CONFIG_KEY_LEN);
        Self::safe_copy(gen.entries[idx].value.as_mut_ptr(), val, CONFIG_VAL_LEN);
        gen.entries[idx].active = true;
        gen.entry_count += 1;
        true
    }

    /// Get a value by key; writes to out_val; returns false if not found
    pub unsafe fn get(&self, key: *const u8, out_val: *mut u8, out_len: usize) -> SigmaBool {
        if !self.initialized { return false; }
        let gen = &self.generations[self.active_gen as usize];
        for i in 0..gen.entry_count as usize {
            if Self::key_eq(gen.entries[i].key.as_ptr(), key, CONFIG_KEY_LEN) {
                Self::safe_copy(out_val, gen.entries[i].value.as_ptr(), out_len);
                return true;
            }
        }
        false
    }

    /// Snapshot the current state into a new generation
    pub unsafe fn snapshot_current(&mut self, label: *const u8, timestamp: SigmaU64) -> SigmaU32 {
        if !self.initialized { return 0; }
        let next_idx = self.gen_count as usize % MAX_GENERATIONS;
        let src_gen  = self.active_gen as usize;
        // Copy current generation to new slot
        self.generations[next_idx] = self.generations[src_gen];
        self.generations[next_idx].gen_id    = self.gen_count + 1;
        self.generations[next_idx].timestamp = timestamp;
        self.generations[next_idx].valid     = true;
        Self::safe_copy(self.generations[next_idx].label.as_mut_ptr(), label, GEN_LABEL_LEN);
        self.gen_count  += 1;
        self.active_gen  = next_idx as SigmaU32;
        self.gen_count
    }

    /// Roll back to a specific generation ID
    pub unsafe fn rollback_to(&mut self, gen_id: SigmaU32) -> SigmaBool {
        for i in 0..MAX_GENERATIONS {
            if self.generations[i].valid && self.generations[i].gen_id == gen_id {
                self.active_gen = i as SigmaU32;
                return true;
            }
        }
        false
    }

    /// Roll back to the previous generation
    pub unsafe fn rollback_previous(&mut self) -> SigmaBool {
        if self.active_gen == 0 || !self.initialized { return false; }
        let prev = (self.active_gen as usize).wrapping_sub(1) % MAX_GENERATIONS;
        if self.generations[prev].valid {
            self.active_gen = prev as SigmaU32;
            return true;
        }
        false
    }

    /// List all available generations (writes gen_id list to out array)
    pub unsafe fn list_generations(&self, out: *mut SigmaU32, out_count: *mut SigmaU32) {
        let mut n = 0usize;
        for i in 0..MAX_GENERATIONS {
            if self.generations[i].valid {
                *out.add(n) = self.generations[i].gen_id;
                n += 1;
            }
        }
        *out_count = n as SigmaU32;
    }
}

// ─── Static singleton ─────────────────────────────────────────────────────────
static mut INSTANCE: SovereignConfigRollback = SovereignConfigRollback::new();

// ─── C-ABI Exports ────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn rollback_init() {
    INSTANCE.init();
}

#[no_mangle]
pub unsafe extern "C" fn rollback_set(key: *const u8, val: *const u8) -> SigmaBool {
    INSTANCE.set(key, val)
}

#[no_mangle]
pub unsafe extern "C" fn rollback_get(key: *const u8, out: *mut u8, len: SigmaU32) -> SigmaBool {
    INSTANCE.get(key, out, len as usize)
}

#[no_mangle]
pub unsafe extern "C" fn rollback_snapshot(label: *const u8, ts: SigmaU64) -> SigmaU32 {
    INSTANCE.snapshot_current(label, ts)
}

#[no_mangle]
pub unsafe extern "C" fn rollback_to(gen_id: SigmaU32) -> SigmaBool {
    INSTANCE.rollback_to(gen_id)
}

#[no_mangle]
pub unsafe extern "C" fn rollback_previous() -> SigmaBool {
    INSTANCE.rollback_previous()
}

#[no_mangle]
pub unsafe extern "C" fn rollback_list(out: *mut SigmaU32, out_count: *mut SigmaU32) {
    INSTANCE.list_generations(out, out_count);
}
