// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/sigma_cgroups.rs — cgroups v2 resource management
// Language: Rust #![no_std]
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, Ordering};

const MAX_CGROUPS: usize = 64;
const MAX_PIDS_PER_CGROUP: usize = 32;

#[derive(Copy, Clone)]
pub struct CgroupLimits {
    pub memory_max_bytes: u64,   // 0 = unlimited
    pub cpu_weight:       u32,   // 1-10000, default 100
    pub cpu_quota_us:     u64,   // microseconds per period, 0=unlimited
    pub cpu_period_us:    u64,   // period in microseconds
    pub io_rbps_max:      u64,   // read bytes/sec, 0=unlimited
    pub io_wbps_max:      u64,   // write bytes/sec, 0=unlimited
}

impl CgroupLimits {
    pub const fn unlimited() -> Self {
        Self {
            memory_max_bytes: 0,
            cpu_weight: 100,
            cpu_quota_us: 0,
            cpu_period_us: 100_000,
            io_rbps_max: 0,
            io_wbps_max: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct CgroupStats {
    pub memory_current:  u64,
    pub cpu_usage_us:    u64,
    pub io_rbytes:       u64,
    pub io_wbytes:       u64,
    pub nr_periods:      u64,
    pub nr_throttled:    u64,
}

pub struct Cgroup {
    pub id:       u32,
    pub name:     [u8; 64],
    pub limits:   CgroupLimits,
    pub stats:    CgroupStats,
    pub pids:     [u32; MAX_PIDS_PER_CGROUP],
    pub pid_count:usize,
    pub parent:   u32,   // 0 = root
    pub active:   bool,
}

impl Cgroup {
    pub const fn empty() -> Self {
        Self {
            id: 0, name: [0u8; 64],
            limits: CgroupLimits::unlimited(),
            stats: CgroupStats { memory_current:0, cpu_usage_us:0,
                                 io_rbytes:0, io_wbytes:0, nr_periods:0, nr_throttled:0 },
            pids: [0u32; MAX_PIDS_PER_CGROUP],
            pid_count: 0, parent: 0, active: false,
        }
    }
}

pub struct CgroupManager {
    groups: [Cgroup; MAX_CGROUPS],
    next_id: u32,
}

impl CgroupManager {
    pub const fn new() -> Self {
        Self {
            groups: [const { Cgroup::empty() }; MAX_CGROUPS],
            next_id: 1,
        }
    }

    pub fn init(&mut self) {
        // Create root cgroup (id=1)
        self.groups[0].id = 1;
        self.groups[0].name[0] = b'/';
        self.groups[0].active = true;
        self.next_id = 2;
    }

    pub fn create(&mut self, name: &[u8], parent: u32) -> u32 {
        for g in &mut self.groups {
            if !g.active {
                let id = self.next_id;
                self.next_id += 1;
                g.id = id;
                g.parent = parent;
                g.active = true;
                let len = name.len().min(63);
                g.name[..len].copy_from_slice(&name[..len]);
                return id;
            }
        }
        0
    }

    pub fn set_memory_max(&mut self, id: u32, bytes: u64) -> bool {
        for g in &mut self.groups {
            if g.active && g.id == id { g.limits.memory_max_bytes = bytes; return true; }
        }
        false
    }

    pub fn set_cpu_weight(&mut self, id: u32, weight: u32) -> bool {
        let w = weight.clamp(1, 10000);
        for g in &mut self.groups { if g.active && g.id == id { g.limits.cpu_weight = w; return true; } }
        false
    }

    pub fn attach_pid(&mut self, id: u32, pid: u32) -> bool {
        for g in &mut self.groups {
            if g.active && g.id == id {
                if g.pid_count < MAX_PIDS_PER_CGROUP {
                    g.pids[g.pid_count] = pid;
                    g.pid_count += 1;
                    return true;
                }
                return false;
            }
        }
        false
    }

    pub fn check_memory(&self, id: u32, alloc_bytes: u64) -> bool {
        for g in &self.groups {
            if g.active && g.id == id {
                let max = g.limits.memory_max_bytes;
                return max == 0 || g.stats.memory_current + alloc_bytes <= max;
            }
        }
        true
    }

    pub fn account_memory(&mut self, id: u32, bytes: i64) {
        for g in &mut self.groups {
            if g.active && g.id == id {
                if bytes >= 0 {
                    g.stats.memory_current += bytes as u64;
                } else {
                    g.stats.memory_current = g.stats.memory_current.saturating_sub((-bytes) as u64);
                }
                return;
            }
        }
    }

    pub fn get_stats(&self, id: u32) -> Option<CgroupStats> {
        self.groups.iter().find(|g| g.active && g.id == id).map(|g| g.stats)
    }
}

static mut G_CGROUPS: CgroupManager = CgroupManager::new();

#[no_mangle] pub unsafe extern "C" fn sigma_cgroup_init() { G_CGROUPS.init(); }
#[no_mangle] pub unsafe extern "C" fn sigma_cgroup_create(name: *const u8, nlen: usize, parent: u32) -> u32 {
    if name.is_null() { return 0; }
    G_CGROUPS.create(core::slice::from_raw_parts(name, nlen), parent)
}
#[no_mangle] pub unsafe extern "C" fn sigma_cgroup_set_memory(id: u32, bytes: u64) -> i32 {
    if G_CGROUPS.set_memory_max(id, bytes) { 0 } else { -1 }
}
#[no_mangle] pub unsafe extern "C" fn sigma_cgroup_set_cpu(id: u32, weight: u32) -> i32 {
    if G_CGROUPS.set_cpu_weight(id, weight) { 0 } else { -1 }
}
#[no_mangle] pub unsafe extern "C" fn sigma_cgroup_attach(id: u32, pid: u32) -> i32 {
    if G_CGROUPS.attach_pid(id, pid) { 0 } else { -1 }
}
#[no_mangle] pub unsafe extern "C" fn sigma_cgroup_check_memory(id: u32, bytes: u64) -> bool {
    G_CGROUPS.check_memory(id, bytes)
}
#[no_mangle] pub unsafe extern "C" fn sigma_cgroup_account_memory(id: u32, delta: i64) {
    G_CGROUPS.account_memory(id, delta);
}
