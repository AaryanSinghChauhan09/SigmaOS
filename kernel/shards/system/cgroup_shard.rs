#![no_std]
#![allow(dead_code)]

/// SigmaOS cgroup v2 Resource Controller Shard
/// Provides hierarchical resource limits (CPU, memory) inspired by Linux cgroups.

use core::sync::atomic::{AtomicU64, AtomicU32, Ordering};

const MAX_CGROUPS: usize = 64;
const MAX_PIDS_PER_CGROUP: usize = 256;

#[derive(Debug)]
pub struct CgroupState {
    pub id: u32,
    pub parent_id: u32,
    
    // Resource limits
    pub memory_limit_bytes: u64,
    pub memory_usage_bytes: AtomicU64,
    
    pub cpu_quota_us: u64,
    pub cpu_period_us: u64,
    pub cpu_usage_us: AtomicU64,
    
    pub pids: [u32; MAX_PIDS_PER_CGROUP],
    pub pid_count: usize,
    
    pub in_use: bool,
}

impl CgroupState {
    pub const fn new() -> Self {
        Self {
            id: 0,
            parent_id: 0,
            memory_limit_bytes: u64::MAX,
            memory_usage_bytes: AtomicU64::new(0),
            cpu_quota_us: u64::MAX,
            cpu_period_us: 100_000,
            cpu_usage_us: AtomicU64::new(0),
            pids: [0; MAX_PIDS_PER_CGROUP],
            pid_count: 0,
            in_use: false,
        }
    }
    
    pub fn attach_pid(&mut self, pid: u32) -> Result<(), &'static str> {
        if self.pid_count >= MAX_PIDS_PER_CGROUP {
            return Err("Cgroup PID limit reached");
        }
        
        // Prevent duplicates
        for i in 0..self.pid_count {
            if self.pids[i] == pid {
                return Ok(()); // Already attached
            }
        }
        
        self.pids[self.pid_count] = pid;
        self.pid_count += 1;
        Ok(())
    }
    
    pub fn check_memory(&self, alloc_bytes: u64) -> bool {
        let current = self.memory_usage_bytes.load(Ordering::Relaxed);
        if current.saturating_add(alloc_bytes) > self.memory_limit_bytes {
            return false; // OOM in this cgroup
        }
        true
    }
    
    pub fn charge_memory(&self, bytes: u64) {
        self.memory_usage_bytes.fetch_add(bytes, Ordering::Relaxed);
    }
    
    pub fn uncharge_memory(&self, bytes: u64) {
        let current = self.memory_usage_bytes.load(Ordering::Relaxed);
        let new_val = current.saturating_sub(bytes);
        self.memory_usage_bytes.store(new_val, Ordering::Relaxed);
    }
}

pub struct CgroupController {
    cgroups: [CgroupState; MAX_CGROUPS],
    cgroup_count: AtomicU32,
}

impl CgroupController {
    pub const fn new() -> Self {
        const INIT_CGROUP: CgroupState = CgroupState::new();
        Self {
            cgroups: [INIT_CGROUP; MAX_CGROUPS],
            cgroup_count: AtomicU32::new(1), // Root cgroup exists
        }
    }
    
    pub fn init(&mut self) {
        self.cgroups[0].in_use = true;
        self.cgroups[0].id = 0; // root
        self.cgroups[0].memory_limit_bytes = u64::MAX;
    }
    
    pub fn create_cgroup(&mut self, parent_id: u32, mem_limit: u64, cpu_quota: u64) -> i32 {
        for i in 1..MAX_CGROUPS {
            if !self.cgroups[i].in_use {
                self.cgroups[i].in_use = true;
                self.cgroups[i].id = i as u32;
                self.cgroups[i].parent_id = parent_id;
                self.cgroups[i].memory_limit_bytes = mem_limit;
                self.cgroups[i].cpu_quota_us = cpu_quota;
                self.cgroup_count.fetch_add(1, Ordering::Relaxed);
                return i as i32;
            }
        }
        -1 // No free cgroups
    }
    
    pub fn attach(&mut self, cgroup_id: u32, pid: u32) -> i32 {
        if cgroup_id as usize >= MAX_CGROUPS || !self.cgroups[cgroup_id as usize].in_use {
            return -1;
        }
        match self.cgroups[cgroup_id as usize].attach_pid(pid) {
            Ok(_) => 0,
            Err(_) => -2,
        }
    }
}

static mut G_CGROUP_CTRL: CgroupController = CgroupController::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_cgroup_init() {
    G_CGROUP_CTRL.init();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_cgroup_create(parent_id: u32, mem_limit: u64, cpu_quota: u64) -> i32 {
    G_CGROUP_CTRL.create_cgroup(parent_id, mem_limit, cpu_quota)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_cgroup_attach(cgroup_id: u32, pid: u32) -> i32 {
    G_CGROUP_CTRL.attach(cgroup_id, pid)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_cgroup_charge_mem(cgroup_id: u32, bytes: u64) -> i32 {
    if cgroup_id as usize >= MAX_CGROUPS || !G_CGROUP_CTRL.cgroups[cgroup_id as usize].in_use {
        return -1;
    }
    let cg = &G_CGROUP_CTRL.cgroups[cgroup_id as usize];
    if cg.check_memory(bytes) {
        cg.charge_memory(bytes);
        return 0; // Success
    }
    -12 // ENOMEM
}

#[no_mangle]
pub unsafe extern "C" fn sigma_cgroup_uncharge_mem(cgroup_id: u32, bytes: u64) {
    if cgroup_id as usize < MAX_CGROUPS && G_CGROUP_CTRL.cgroups[cgroup_id as usize].in_use {
        G_CGROUP_CTRL.cgroups[cgroup_id as usize].uncharge_memory(bytes);
    }
}
