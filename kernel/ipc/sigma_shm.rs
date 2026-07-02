// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// kernel/ipc/sigma_shm.rs — Shared Memory (no_std)
// Language: Rust #![no_std]
// Pattern: OOP via ShmManager + ShmRegion

#![no_std]

pub const MAX_SHM_REGIONS: usize = 32;
pub const MAX_SHM_SIZE:     usize = 64 * 1024 * 1024; // 64 MB cap per region

#[derive(Clone, Copy)]
pub struct ShmRegion {
    pub key:       u32,
    pub phys_base: usize,
    pub size:      usize,
    pub owner_pid: u32,
    pub ref_count: u32,
    pub flags:     ShmFlags,
}

#[derive(Clone, Copy, Default)]
pub struct ShmFlags {
    pub read_only:  bool,
    pub exec:       bool,
    pub persistent: bool, // survives last detach
}

pub struct ShmManager {
    regions: [Option<ShmRegion>; MAX_SHM_REGIONS],
    count:   usize,
}

impl ShmManager {
    pub const fn new() -> Self {
        Self { regions: [const { None }; MAX_SHM_REGIONS], count: 0 }
    }

    /// Create a new shared memory region backed by physical memory `phys`
    pub fn create(&mut self, key: u32, phys: usize, size: usize,
                  owner: u32, flags: ShmFlags) -> Option<usize> {
        if size == 0 || size > MAX_SHM_SIZE { return None; }
        // Check key not already in use
        if self.find_by_key(key).is_some() { return None; }
        for (i, slot) in self.regions.iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some(ShmRegion {
                    key, phys_base: phys, size,
                    owner_pid: owner, ref_count: 1, flags,
                });
                self.count += 1;
                return Some(i);
            }
        }
        None
    }

    pub fn find_by_key(&self, key: u32) -> Option<usize> {
        self.regions.iter().position(|s| matches!(s, Some(r) if r.key == key))
    }

    pub fn attach(&mut self, idx: usize) -> Option<usize> {
        if let Some(Some(r)) = self.regions.get_mut(idx) {
            r.ref_count += 1;
            return Some(r.phys_base);
        }
        None
    }

    pub fn detach(&mut self, idx: usize) {
        if let Some(Some(r)) = self.regions.get_mut(idx) {
            r.ref_count = r.ref_count.saturating_sub(1);
            if r.ref_count == 0 && !r.flags.persistent {
                self.regions[idx] = None;
                self.count -= 1;
            }
        }
    }

    pub fn destroy(&mut self, key: u32, pid: u32) -> bool {
        if let Some(idx) = self.find_by_key(key) {
            if let Some(Some(r)) = self.regions.get(idx) {
                if r.owner_pid == pid || pid == 0 {
                    self.regions[idx] = None;
                    self.count -= 1;
                    return true;
                }
            }
        }
        false
    }

    pub fn region(&self, idx: usize) -> Option<&ShmRegion> {
        self.regions.get(idx)?.as_ref()
    }
}
