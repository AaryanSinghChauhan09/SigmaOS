// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// kernel/core/sigma_cgroup.rs — cgroup v2 Resource Enforcement (no_std)
// Language: Rust #![no_std]
// Pattern: OOP via CgroupManager + Cgroup struct

#![no_std]

pub const MAX_CGROUPS:  usize = 64;
pub const MAX_MEMBERS:  usize = 32;
pub const CGROUP_NAME:  usize = 64;

// ── Resource Limits ───────────────────────────────────────────────────────────
#[derive(Clone, Copy, Default)]
pub struct ResourceLimits {
    pub cpu_shares:       u32,    // relative CPU weight (1024 = default)
    pub cpu_quota_us:     u64,    // µs of CPU per period (0 = unlimited)
    pub cpu_period_us:    u64,    // period length in µs (100_000 = 100ms)
    pub memory_limit:     u64,    // bytes (0 = unlimited)
    pub memory_soft:      u64,    // soft limit in bytes
    pub io_read_bps:      u64,    // bytes/sec read limit (0 = unlimited)
    pub io_write_bps:     u64,    // bytes/sec write limit
    pub pids_max:         u32,    // max PIDs in group (0 = unlimited)
}

impl ResourceLimits {
    pub const fn default() -> Self {
        Self {
            cpu_shares:    1024,
            cpu_quota_us:  0,
            cpu_period_us: 100_000,
            memory_limit:  0,
            memory_soft:   0,
            io_read_bps:   0,
            io_write_bps:  0,
            pids_max:      0,
        }
    }
}

// ── Usage Accounting ──────────────────────────────────────────────────────────
#[derive(Clone, Copy, Default)]
pub struct ResourceUsage {
    pub cpu_time_us:    u64,
    pub memory_bytes:   u64,
    pub io_read_bytes:  u64,
    pub io_write_bytes: u64,
    pub n_pids:         u32,
    pub cpu_throttle_us: u64, // time spent throttled
}

// ── Cgroup ────────────────────────────────────────────────────────────────────
#[derive(Clone, Copy)]
pub struct Cgroup {
    pub id:       u32,
    pub parent:   Option<u32>,
    pub name:     [u8; CGROUP_NAME],
    pub name_len: usize,
    pub limits:   ResourceLimits,
    pub usage:    ResourceUsage,
    pub members:  [u32; MAX_MEMBERS], // PIDs
    pub n_members: usize,
    pub enabled:  bool,
}

impl Cgroup {
    pub fn new(id: u32, name: &[u8], parent: Option<u32>) -> Self {
        let mut c = Cgroup {
            id, parent,
            name: [0u8; CGROUP_NAME],
            name_len: name.len().min(CGROUP_NAME),
            limits: ResourceLimits::default(),
            usage: ResourceUsage::default(),
            members: [0u32; MAX_MEMBERS],
            n_members: 0,
            enabled: true,
        };
        c.name[..c.name_len].copy_from_slice(&name[..c.name_len]);
        c
    }

    pub fn add_pid(&mut self, pid: u32) -> bool {
        if self.n_members >= MAX_MEMBERS { return false; }
        if self.members[..self.n_members].contains(&pid) { return true; }
        self.members[self.n_members] = pid; self.n_members += 1; true
    }

    pub fn remove_pid(&mut self, pid: u32) {
        if let Some(i) = self.members[..self.n_members].iter().position(|&p| p == pid) {
            self.n_members -= 1;
            self.members[i] = self.members[self.n_members];
        }
    }

    pub fn has_pid(&self, pid: u32) -> bool {
        self.members[..self.n_members].contains(&pid)
    }

    /// Returns true if this process may consume more CPU (quota check)
    pub fn cpu_allowed(&self) -> bool {
        if self.limits.cpu_quota_us == 0 { return true; }
        let used_in_period = self.usage.cpu_time_us % self.limits.cpu_period_us;
        used_in_period < self.limits.cpu_quota_us
    }

    /// Returns true if memory allocation is within limits
    pub fn memory_allowed(&self, bytes: u64) -> bool {
        if self.limits.memory_limit == 0 { return true; }
        self.usage.memory_bytes + bytes <= self.limits.memory_limit
    }

    /// Charge CPU time (called from scheduler tick)
    pub fn charge_cpu(&mut self, us: u64) {
        self.usage.cpu_time_us += us;
        if !self.cpu_allowed() { self.usage.cpu_throttle_us += us; }
    }

    /// Charge memory
    pub fn charge_memory(&mut self, bytes: u64) {
        self.usage.memory_bytes += bytes;
    }

    pub fn uncharge_memory(&mut self, bytes: u64) {
        self.usage.memory_bytes = self.usage.memory_bytes.saturating_sub(bytes);
    }
}

// ── Cgroup Manager ────────────────────────────────────────────────────────────
pub struct CgroupManager {
    groups:    [Option<Cgroup>; MAX_CGROUPS],
    count:     usize,
    next_id:   u32,
}

impl CgroupManager {
    pub const fn new() -> Self {
        Self { groups: [const { None }; MAX_CGROUPS], count: 0, next_id: 1 }
    }

    /// Create root cgroup
    pub fn create_root(&mut self) -> u32 {
        let id = self.next_id; self.next_id += 1;
        self.groups[0] = Some(Cgroup::new(id, b"root", None));
        self.count += 1;
        id
    }

    pub fn create(&mut self, name: &[u8], parent: Option<u32>) -> Option<u32> {
        if self.count >= MAX_CGROUPS { return None; }
        let id = self.next_id; self.next_id += 1;
        for slot in &mut self.groups {
            if slot.is_none() {
                *slot = Some(Cgroup::new(id, name, parent));
                self.count += 1;
                return Some(id);
            }
        }
        None
    }

    pub fn delete(&mut self, id: u32) -> bool {
        for slot in &mut self.groups {
            if matches!(slot, Some(g) if g.id == id && g.n_members == 0) {
                *slot = None; self.count -= 1; return true;
            }
        }
        false
    }

    pub fn get_mut(&mut self, id: u32) -> Option<&mut Cgroup> {
        self.groups.iter_mut().flatten().find(|g| g.id == id)
    }

    pub fn get(&self, id: u32) -> Option<&Cgroup> {
        self.groups.iter().flatten().find(|g| g.id == id)
    }

    /// Find the cgroup a PID belongs to
    pub fn cgroup_of(&self, pid: u32) -> Option<u32> {
        self.groups.iter().flatten()
            .find(|g| g.has_pid(pid)).map(|g| g.id)
    }

    pub fn move_pid(&mut self, pid: u32, from: u32, to: u32) -> bool {
        if let Some(g) = self.get_mut(from) { g.remove_pid(pid); }
        if let Some(g) = self.get_mut(to)   { return g.add_pid(pid); }
        false
    }
}
