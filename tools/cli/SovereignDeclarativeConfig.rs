// SPDX-License-Identifier: GPL-2.0-or-later
//! SovereignDeclarativeConfig — no_std declarative configuration engine
//! Migrated from C/C++. Supports typed entries, generation snapshots, rollback.

#![no_std]
#![allow(dead_code)]
#![allow(non_snake_case)]

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
const KEY_LEN:      usize = 64;
const VAL_LEN:      usize = 128;
const MAX_ENTRIES:  usize = 128;
const MAX_GENS:     usize = 8;

// ─── Entry type discriminant ─────────────────────────────────────────────────
#[repr(u64)]
#[derive(Copy, Clone, PartialEq)]
pub enum EntryKind {
    String  = 0,
    Int     = 1,
    Bool    = 2,
    Float   = 3,
    Array   = 4,
    Table   = 5,
}

impl EntryKind {
    pub fn from_u64(v: u64) -> Self {
        match v {
            1 => Self::Int,
            2 => Self::Bool,
            3 => Self::Float,
            4 => Self::Array,
            5 => Self::Table,
            _ => Self::String,
        }
    }
}

/// ConfigEntry — a single typed key=value configuration entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ConfigEntry {
    pub id:     SigmaU32,
    pub kind:   SigmaU64,       // EntryKind as u64 (C-compatible)
    pub key:    [u8; KEY_LEN],
    pub value:  [u8; VAL_LEN],
    pub active: SigmaBool,
}

impl ConfigEntry {
    pub const fn empty() -> Self {
        Self {
            id: 0, kind: 0,
            key:   [0u8; KEY_LEN],
            value: [0u8; VAL_LEN],
            active: false,
        }
    }
}

/// Generation — immutable snapshot of a complete config state
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Generation {
    pub gen_id:      SigmaU32,
    pub entry_count: SigmaU32,
    pub timestamp:   SigmaU64,
    pub current:     SigmaBool,
    pub entries:     [ConfigEntry; MAX_ENTRIES],
}

impl Generation {
    pub const fn empty() -> Self {
        Self {
            gen_id: 0, entry_count: 0, timestamp: 0, current: false,
            entries: [ConfigEntry { id: 0, kind: 0, key: [0u8; KEY_LEN], value: [0u8; VAL_LEN], active: false }; MAX_ENTRIES],
        }
    }
}

/// DeclarativeConfig — the main engine
#[repr(C)]
pub struct DeclarativeConfig {
    pub generations:  [Generation; MAX_GENS],
    pub gen_count:    SigmaU32,
    pub active_gen:   SigmaU32,
    pub next_id:      SigmaU32,
    pub initialized:  SigmaBool,
}

impl DeclarativeConfig {
    pub const fn new() -> Self {
        Self {
            generations: [Generation {
                gen_id: 0, entry_count: 0, timestamp: 0, current: false,
                entries: [ConfigEntry { id: 0, kind: 0, key: [0u8; KEY_LEN], value: [0u8; VAL_LEN], active: false }; MAX_ENTRIES],
            }; MAX_GENS],
            gen_count:   0,
            active_gen:  0,
            next_id:     1,
            initialized: false,
        }
    }

    unsafe fn strcopy(dst: *mut u8, src: *const u8, n: usize) {
        for i in 0..n {
            let b = *src.add(i);
            *dst.add(i) = b;
            if b == 0 { return; }
        }
    }

    unsafe fn streq(a: *const u8, b: *const u8, n: usize) -> bool {
        for i in 0..n {
            if *a.add(i) != *b.add(i) { return false; }
            if *a.add(i) == 0 { return true; }
        }
        true
    }

    pub unsafe fn dconfig_init(&mut self) {
        self.gen_count   = 1;          // start at generation 1
        self.active_gen  = 0;
        self.next_id     = 1;
        self.generations[0].gen_id  = 1;
        self.generations[0].current = true;
        self.initialized = true;
    }

    pub unsafe fn dconfig_set(&mut self, key: *const u8, val: *const u8, kind: SigmaU64) -> SigmaBool {
        if !self.initialized { return false; }
        let gen = &mut self.generations[self.active_gen as usize];
        // Update existing
        for i in 0..gen.entry_count as usize {
            if Self::streq(gen.entries[i].key.as_ptr(), key, KEY_LEN) {
                Self::strcopy(gen.entries[i].value.as_mut_ptr(), val, VAL_LEN);
                gen.entries[i].kind   = kind;
                gen.entries[i].active = true;
                return true;
            }
        }
        // Insert new
        if gen.entry_count as usize >= MAX_ENTRIES { return false; }
        let idx = gen.entry_count as usize;
        gen.entries[idx].id   = self.next_id;
        gen.entries[idx].kind = kind;
        Self::strcopy(gen.entries[idx].key.as_mut_ptr(), key, KEY_LEN);
        Self::strcopy(gen.entries[idx].value.as_mut_ptr(), val, VAL_LEN);
        gen.entries[idx].active = true;
        gen.entry_count  += 1;
        self.next_id     += 1;
        true
    }

    pub unsafe fn dconfig_get(&self, key: *const u8, out: *mut u8, out_kind: *mut SigmaU64, len: usize) -> SigmaBool {
        if !self.initialized { return false; }
        let gen = &self.generations[self.active_gen as usize];
        for i in 0..gen.entry_count as usize {
            if Self::streq(gen.entries[i].key.as_ptr(), key, KEY_LEN) {
                Self::strcopy(out, gen.entries[i].value.as_ptr(), len);
                *out_kind = gen.entries[i].kind;
                return true;
            }
        }
        false
    }

    /// Snapshot current state as a new immutable generation
    pub unsafe fn dconfig_snapshot(&mut self, timestamp: SigmaU64) -> SigmaU32 {
        if !self.initialized { return 0; }
        let next_slot = self.gen_count as usize % MAX_GENS;
        let cur = self.active_gen as usize;
        // Copy entries into new generation
        self.generations[next_slot] = self.generations[cur];
        self.generations[cur].current = false;
        self.gen_count += 1;
        self.generations[next_slot].gen_id    = self.gen_count;
        self.generations[next_slot].timestamp = timestamp;
        self.generations[next_slot].current   = true;
        self.active_gen = next_slot as SigmaU32;
        self.gen_count
    }

    /// Roll back to a specific generation
    pub unsafe fn dconfig_rollback(&mut self, gen_id: SigmaU32) -> SigmaBool {
        for i in 0..MAX_GENS {
            if self.generations[i].gen_id == gen_id {
                self.generations[self.active_gen as usize].current = false;
                self.active_gen = i as SigmaU32;
                self.generations[i].current = true;
                return true;
            }
        }
        false
    }

    /// Write status summary to a buffer (returns bytes written)
    pub unsafe fn dconfig_status(&self, out: *mut u8, out_len: usize) -> usize {
        if !self.initialized || out_len < 4 { return 0; }
        // Write "GEN:N ENTRIES:M" as ASCII
        let gen   = self.gen_count;
        let count = self.generations[self.active_gen as usize].entry_count;
        // Simple ASCII number write
        let mut pos = 0usize;
        let prefix = b"GEN:";
        for &b in prefix { if pos < out_len { *out.add(pos) = b; pos += 1; } }
        // gen_count digits
        let s = u32_to_ascii(gen);
        for &b in s.iter().take_while(|&&b| b != 0) { if pos < out_len { *out.add(pos) = b; pos += 1; } }
        let sep = b" ENTRIES:";
        for &b in sep { if pos < out_len { *out.add(pos) = b; pos += 1; } }
        let e = u32_to_ascii(count);
        for &b in e.iter().take_while(|&&b| b != 0) { if pos < out_len { *out.add(pos) = b; pos += 1; } }
        if pos < out_len { *out.add(pos) = 0; }
        pos
    }
}

fn u32_to_ascii(mut n: SigmaU32) -> [u8; 12] {
    let mut buf = [0u8; 12];
    if n == 0 { buf[0] = b'0'; return buf; }
    let mut i = 11usize;
    while n > 0 && i > 0 {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    // Shift left
    let start = i;
    let mut out = [0u8; 12];
    let mut j = 0;
    for k in start..12 { if buf[k] != 0 { out[j] = buf[k]; j += 1; } }
    out
}

// ─── Static singleton ─────────────────────────────────────────────────────────
static mut INSTANCE: DeclarativeConfig = DeclarativeConfig::new();

// ─── C-ABI Exports ────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn dconfig_init() {
    INSTANCE.dconfig_init();
}

#[no_mangle]
pub unsafe extern "C" fn dconfig_set(key: *const u8, val: *const u8, kind: SigmaU64) -> SigmaBool {
    INSTANCE.dconfig_set(key, val, kind)
}

#[no_mangle]
pub unsafe extern "C" fn dconfig_get(key: *const u8, out: *mut u8, out_kind: *mut SigmaU64, len: SigmaU32) -> SigmaBool {
    INSTANCE.dconfig_get(key, out, out_kind, len as usize)
}

#[no_mangle]
pub unsafe extern "C" fn dconfig_snapshot(ts: SigmaU64) -> SigmaU32 {
    INSTANCE.dconfig_snapshot(ts)
}

#[no_mangle]
pub unsafe extern "C" fn dconfig_rollback(gen_id: SigmaU32) -> SigmaBool {
    INSTANCE.dconfig_rollback(gen_id)
}

#[no_mangle]
pub unsafe extern "C" fn dconfig_status(out: *mut u8, len: SigmaU32) -> SigmaU32 {
    INSTANCE.dconfig_status(out, len as usize) as SigmaU32
}

#[no_mangle]
pub unsafe extern "C" fn printStatus() {
    // kernel console write would go here; stub keeps ABI compatibility
    let _ = &INSTANCE;
}
