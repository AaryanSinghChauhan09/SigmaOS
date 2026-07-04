// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// security/sigma_avc.rs — Access Vector Cache (SELinux-inspired O(1) MAC)
// Language: Rust #![no_std]
// Pattern: OOP via Avc struct with hash-table cache

#![no_std]

pub const AVC_CACHE_SLOTS: usize = 256;

/// Object class identifiers (what resource is being accessed)
#[repr(u16)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ObjClass {
    File    = 1,
    Dir     = 2,
    Process = 3,
    Socket  = 4,
    Shard   = 5,
    Ipc     = 6,
}

/// Permission bits (what operation is being attempted)
pub struct Perm(pub u32);
impl Perm {
    pub const READ:    u32 = 1 << 0;
    pub const WRITE:   u32 = 1 << 1;
    pub const EXEC:    u32 = 1 << 2;
    pub const CREATE:  u32 = 1 << 3;
    pub const UNLINK:  u32 = 1 << 4;
    pub const CONNECT: u32 = 1 << 5;
    pub const SEND:    u32 = 1 << 6;
    pub const RECV:    u32 = 1 << 7;
    pub const SIGNAL:  u32 = 1 << 8;
    pub const FORK:    u32 = 1 << 9;
}

/// AVC cache entry
#[derive(Clone, Copy, Default)]
struct AvcEntry {
    ssid:     u32,   // source security identifier (process label)
    tsid:     u32,   // target security identifier (resource label)
    tclass:   u16,   // object class
    allowed:  u32,   // allowed permission bitmask
    denied:   u32,   // explicitly denied bitmask
    valid:    bool,
}

/// Access Vector Cache — O(1) lookup for MAC decisions
pub struct Avc {
    cache:   [AvcEntry; AVC_CACHE_SLOTS],
    hits:    u64,
    misses:  u64,
}

impl Avc {
    pub const fn new() -> Self {
        Self {
            cache:  [AvcEntry { ssid: 0, tsid: 0, tclass: 0,
                                allowed: 0, denied: 0, valid: false };
                     AVC_CACHE_SLOTS],
            hits:   0,
            misses: 0,
        }
    }

    fn hash(ssid: u32, tsid: u32, tclass: u16) -> usize {
        let h = ssid.wrapping_mul(0x9e3779b9)
                    .wrapping_add(tsid.wrapping_mul(0x517cc1b7))
                    .wrapping_add(tclass as u32 * 0x27d4eb2d);
        (h as usize) % AVC_CACHE_SLOTS
    }

    /// Check if (ssid, tsid, tclass, perm) is allowed.
    /// Returns `Some(true/false)` if cached, `None` on cache miss.
    pub fn lookup(&mut self, ssid: u32, tsid: u32, tclass: ObjClass, perm: u32)
                  -> Option<bool> {
        let slot = Self::hash(ssid, tsid, tclass as u16);
        let e = &self.cache[slot];
        if e.valid && e.ssid == ssid && e.tsid == tsid && e.tclass == tclass as u16 {
            self.hits += 1;
            if e.denied & perm != 0 { return Some(false); }
            if e.allowed & perm == perm { return Some(true); }
        }
        self.misses += 1;
        None
    }

    /// Insert a policy decision into the cache
    pub fn insert(&mut self, ssid: u32, tsid: u32, tclass: ObjClass,
                  allowed: u32, denied: u32) {
        let slot = Self::hash(ssid, tsid, tclass as u16);
        self.cache[slot] = AvcEntry {
            ssid, tsid, tclass: tclass as u16, allowed, denied, valid: true,
        };
    }

    /// Invalidate all cache entries (e.g. after policy reload)
    pub fn flush(&mut self) {
        for e in &mut self.cache { e.valid = false; }
    }

    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 { return 0.0; }
        self.hits as f64 / total as f64
    }
}
