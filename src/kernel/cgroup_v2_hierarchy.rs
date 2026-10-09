//! # Cgroups v2 Hierarchy Manager
//!
//! Linux cgroups v2 (unified hierarchy) resource controller for SigmaOS.
//! Inspired by Linux `kernel/cgroup/cgroup.c`, `kernel/cgroup/cgroup-v2.c`,
//! `include/linux/cgroup.h`, and the cgroups v2 man7 documentation.
//!
//! Implements: cgroup tree, memory controller, CPU controller, PID controller.

#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicI64, AtomicU32, AtomicU64, Ordering};

// ============================================================================
// 1. CGROUP CONTROLLER TYPES (from Linux include/linux/cgroup_subsys.h)
// ============================================================================

/// Available cgroup subsystem controllers
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CgroupController {
    Cpu,     // cpu.weight, cpu.max
    Memory,  // memory.max, memory.high, memory.min
    Pids,    // pids.max
    Io,      // io.max, io.weight
    Cpuset,  // cpuset.cpus, cpuset.mems
    Hugetlb, // hugetlb.<size>.max
}

// ============================================================================
// 2. RESOURCE LIMITS & STATISTICS
// ============================================================================

/// CPU controller settings (cpu.weight = 1..10000, cpu.max = quota/period)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CpuLimits {
    pub weight: u32,       // 1–10000, default 100 (relative shares)
    pub max_quota_us: i64, // Microseconds per period; -1 = unlimited
    pub period_us: u64,    // Period in microseconds (default 100ms)
    pub burst_us: u64,     // cpu.stat burst allowance
}

impl Default for CpuLimits {
    fn default() -> Self {
        Self {
            weight: 100,
            max_quota_us: -1,   // unlimited
            period_us: 100_000, // 100ms
            burst_us: 0,
        }
    }
}

/// Memory controller settings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryLimits {
    pub max_bytes: i64,  // memory.max — hard limit; -1 = unlimited
    pub high_bytes: i64, // memory.high — soft throttle limit; -1 = unlimited
    pub min_bytes: i64,  // memory.min — guaranteed minimum; 0 = none
    pub swap_max: i64,   // memory.swap.max; -1 = unlimited
}

impl Default for MemoryLimits {
    fn default() -> Self {
        Self {
            max_bytes: -1,
            high_bytes: -1,
            min_bytes: 0,
            swap_max: -1,
        }
    }
}

/// PID controller settings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PidLimits {
    pub max_pids: i64, // pids.max; -1 = unlimited
}

impl Default for PidLimits {
    fn default() -> Self {
        Self { max_pids: -1 }
    }
}

/// Live resource usage statistics for a cgroup
#[derive(Debug)]
pub struct CgroupStats {
    pub memory_current_bytes: AtomicU64,
    pub cpu_usage_us: AtomicU64,
    pub cpu_throttled_us: AtomicU64,
    pub cpu_nr_throttled: AtomicU64,
    pub pids_current: AtomicU32,
    pub io_read_bytes: AtomicU64,
    pub io_write_bytes: AtomicU64,
    pub oom_kills: AtomicU64,
}

impl Default for CgroupStats {
    fn default() -> Self {
        Self {
            memory_current_bytes: AtomicU64::new(0),
            cpu_usage_us: AtomicU64::new(0),
            cpu_throttled_us: AtomicU64::new(0),
            cpu_nr_throttled: AtomicU64::new(0),
            pids_current: AtomicU32::new(0),
            io_read_bytes: AtomicU64::new(0),
            io_write_bytes: AtomicU64::new(0),
            oom_kills: AtomicU64::new(0),
        }
    }
}

impl Clone for CgroupStats {
    fn clone(&self) -> Self {
        Self {
            memory_current_bytes: AtomicU64::new(self.memory_current_bytes.load(Ordering::SeqCst)),
            cpu_usage_us: AtomicU64::new(self.cpu_usage_us.load(Ordering::SeqCst)),
            cpu_throttled_us: AtomicU64::new(self.cpu_throttled_us.load(Ordering::SeqCst)),
            cpu_nr_throttled: AtomicU64::new(self.cpu_nr_throttled.load(Ordering::SeqCst)),
            pids_current: AtomicU32::new(self.pids_current.load(Ordering::SeqCst)),
            io_read_bytes: AtomicU64::new(self.io_read_bytes.load(Ordering::SeqCst)),
            io_write_bytes: AtomicU64::new(self.io_write_bytes.load(Ordering::SeqCst)),
            oom_kills: AtomicU64::new(self.oom_kills.load(Ordering::SeqCst)),
        }
    }
}

// ============================================================================
// 3. CGROUP NODE (single cgroup in the hierarchy)
//    Inspired by Linux struct cgroup (include/linux/cgroup.h)
// ============================================================================

/// A single cgroup node in the v2 unified hierarchy
#[derive(Debug)]
pub struct Cgroup {
    pub id: u64,
    pub name: String,
    pub parent_id: Option<u64>,
    pub children: Vec<u64>,
    pub pids: Vec<u32>,
    pub enabled_controllers: Vec<CgroupController>,
    pub cpu_limits: CpuLimits,
    pub memory_limits: MemoryLimits,
    pub pid_limits: PidLimits,
    pub stats: CgroupStats,
    pub is_root: bool,
}

impl Cgroup {
    pub fn new_root() -> Self {
        Self {
            id: 1,
            name: String::from("/"),
            parent_id: None,
            children: Vec::new(),
            pids: Vec::new(),
            enabled_controllers: vec![
                CgroupController::Cpu,
                CgroupController::Memory,
                CgroupController::Pids,
            ],
            cpu_limits: CpuLimits::default(),
            memory_limits: MemoryLimits::default(),
            pid_limits: PidLimits::default(),
            stats: CgroupStats::default(),
            is_root: true,
        }
    }

    pub fn new_child(id: u64, name: &str, parent_id: u64) -> Self {
        Self {
            id,
            name: String::from(name),
            parent_id: Some(parent_id),
            children: Vec::new(),
            pids: Vec::new(),
            enabled_controllers: Vec::new(),
            cpu_limits: CpuLimits::default(),
            memory_limits: MemoryLimits::default(),
            pid_limits: PidLimits::default(),
            stats: CgroupStats::default(),
            is_root: false,
        }
    }

    /// Attach a process to this cgroup (cgroup.procs write)
    pub fn attach_pid(&mut self, pid: u32) -> Result<(), &'static str> {
        let current = self.stats.pids_current.load(Ordering::SeqCst);
        if self.pid_limits.max_pids > 0 && (current as i64) >= self.pid_limits.max_pids {
            return Err("pids.max limit exceeded");
        }
        if !self.pids.contains(&pid) {
            self.pids.push(pid);
            self.stats.pids_current.fetch_add(1, Ordering::SeqCst);
        }
        Ok(())
    }

    /// Remove process from cgroup (on process exit or migration)
    pub fn detach_pid(&mut self, pid: u32) {
        if let Some(pos) = self.pids.iter().position(|&p| p == pid) {
            self.pids.remove(pos);
            self.stats.pids_current.fetch_sub(1, Ordering::SeqCst);
        }
    }

    /// Check if memory allocation is within limits
    pub fn check_memory_limit(&self, request_bytes: u64) -> bool {
        let current = self.stats.memory_current_bytes.load(Ordering::SeqCst);
        if self.memory_limits.max_bytes > 0 {
            (current + request_bytes) as i64 <= self.memory_limits.max_bytes
        } else {
            true // unlimited
        }
    }

    /// Record memory charge (memory.current)
    pub fn charge_memory(&self, bytes: u64) -> bool {
        if !self.check_memory_limit(bytes) {
            return false;
        }
        self.stats
            .memory_current_bytes
            .fetch_add(bytes, Ordering::SeqCst);
        true
    }

    /// Record memory uncharge
    pub fn uncharge_memory(&self, bytes: u64) {
        let cur = self.stats.memory_current_bytes.load(Ordering::SeqCst);
        self.stats
            .memory_current_bytes
            .store(cur.saturating_sub(bytes), Ordering::SeqCst);
    }

    /// Record CPU time usage
    pub fn account_cpu_time(&self, us: u64) {
        self.stats.cpu_usage_us.fetch_add(us, Ordering::SeqCst);
    }

    pub fn set_memory_max(&mut self, max_bytes: i64) {
        self.memory_limits.max_bytes = max_bytes;
    }

    pub fn set_pid_max(&mut self, max_pids: i64) {
        self.pid_limits.max_pids = max_pids;
    }

    pub fn set_cpu_weight(&mut self, weight: u32) {
        self.cpu_limits.weight = weight.max(1).min(10000);
    }

    pub fn enable_controller(&mut self, ctrl: CgroupController) {
        if !self.enabled_controllers.contains(&ctrl) {
            self.enabled_controllers.push(ctrl);
        }
    }
}

// ============================================================================
// 4. CGROUP HIERARCHY MANAGER
//    Inspired by Linux cgroup_create(), cgroup_attach_task()
// ============================================================================

pub struct CgroupHierarchy {
    pub cgroups: BTreeMap<u64, Cgroup>,
    pub next_id: AtomicU64,
    pub total_pids_tracked: AtomicU32,
}

impl CgroupHierarchy {
    pub fn new() -> Self {
        let mut h = Self {
            cgroups: BTreeMap::new(),
            next_id: AtomicU64::new(2),
            total_pids_tracked: AtomicU32::new(0),
        };
        h.cgroups.insert(1, Cgroup::new_root());
        h
    }

    /// Create a new cgroup under a parent (mkdir in cgroupfs)
    pub fn create(&mut self, name: &str, parent_id: u64) -> Result<u64, &'static str> {
        if !self.cgroups.contains_key(&parent_id) {
            return Err("Parent cgroup not found");
        }
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let cg = Cgroup::new_child(id, name, parent_id);
        self.cgroups.get_mut(&parent_id).unwrap().children.push(id);
        self.cgroups.insert(id, cg);
        Ok(id)
    }

    /// Attach PID to a cgroup
    pub fn attach(&mut self, cgroup_id: u64, pid: u32) -> Result<(), &'static str> {
        // Detach from any other cgroup
        let old_cg_id: Option<u64> = self
            .cgroups
            .values()
            .find(|cg| cg.pids.contains(&pid))
            .map(|cg| cg.id);

        if let Some(old_id) = old_cg_id {
            if let Some(old_cg) = self.cgroups.get_mut(&old_id) {
                old_cg.detach_pid(pid);
            }
        } else {
            self.total_pids_tracked.fetch_add(1, Ordering::SeqCst);
        }

        let cg = self.cgroups.get_mut(&cgroup_id).ok_or("Cgroup not found")?;
        cg.attach_pid(pid)
    }

    /// Remove a cgroup (rmdir in cgroupfs — must be empty)
    pub fn remove(&mut self, cgroup_id: u64) -> Result<(), &'static str> {
        if cgroup_id == 1 {
            return Err("Cannot remove root cgroup");
        }
        let cg = self.cgroups.get(&cgroup_id).ok_or("Cgroup not found")?;
        if !cg.pids.is_empty() {
            return Err("Cgroup has live processes");
        }
        if !cg.children.is_empty() {
            return Err("Cgroup has children");
        }
        let parent_id = cg.parent_id;
        self.cgroups.remove(&cgroup_id);
        if let Some(pid) = parent_id {
            if let Some(parent) = self.cgroups.get_mut(&pid) {
                parent.children.retain(|&c| c != cgroup_id);
            }
        }
        Ok(())
    }

    pub fn get(&self, id: u64) -> Option<&Cgroup> {
        self.cgroups.get(&id)
    }

    pub fn get_mut(&mut self, id: u64) -> Option<&mut Cgroup> {
        self.cgroups.get_mut(&id)
    }

    /// Find cgroup owning a given PID
    pub fn find_by_pid(&self, pid: u32) -> Option<u64> {
        self.cgroups
            .values()
            .find(|cg| cg.pids.contains(&pid))
            .map(|cg| cg.id)
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(any(test, feature = "standalone_test"))]
mod tests {
    use super::*;

    #[test]
    fn test_cgroup_create_and_attach() {
        let mut h = CgroupHierarchy::new();
        let sys_id = h.create("system.slice", 1).unwrap();
        let svc_id = h.create("nginx.service", sys_id).unwrap();

        h.attach(svc_id, 1001).unwrap();
        h.attach(svc_id, 1002).unwrap();

        let svc = h.get(svc_id).unwrap();
        assert_eq!(svc.pids.len(), 2);
        assert_eq!(svc.stats.pids_current.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_memory_limit_enforcement() {
        let mut h = CgroupHierarchy::new();
        let id = h.create("test.slice", 1).unwrap();
        h.get_mut(id).unwrap().set_memory_max(4 * 1024 * 1024); // 4MB

        let cg = h.get(id).unwrap();
        assert!(cg.charge_memory(1024 * 1024)); // 1MB OK
        assert!(cg.charge_memory(2 * 1024 * 1024)); // 3MB total OK
        assert!(!cg.charge_memory(2 * 1024 * 1024)); // 5MB exceeds 4MB limit
    }

    #[test]
    fn test_pid_limit_enforcement() {
        let mut h = CgroupHierarchy::new();
        let id = h.create("limited.slice", 1).unwrap();
        h.get_mut(id).unwrap().set_pid_max(2);

        h.attach(id, 100).unwrap();
        h.attach(id, 101).unwrap();
        assert!(h.attach(id, 102).is_err()); // Should fail
    }

    #[test]
    fn test_remove_empty_cgroup() {
        let mut h = CgroupHierarchy::new();
        let id = h.create("empty.slice", 1).unwrap();
        assert!(h.remove(id).is_ok());
        assert!(h.get(id).is_none());
        assert!(h.remove(1).is_err()); // Cannot remove root
    }

    #[test]
    fn test_find_by_pid() {
        let mut h = CgroupHierarchy::new();
        let id = h.create("search.slice", 1).unwrap();
        h.attach(id, 9999).unwrap();
        assert_eq!(h.find_by_pid(9999), Some(id));
        assert_eq!(h.find_by_pid(1111), None);
    }
}
