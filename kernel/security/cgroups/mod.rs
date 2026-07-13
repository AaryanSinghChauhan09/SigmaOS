// =============================================================================
// SIGMAOS: CGROUP-AWARE NAMESPACE ACCOUNTING (sigma-cgroups)
// =============================================================================
// Sovereign cgroup subsystem providing resource accounting, namespace-based
// isolation, and hierarchical process group management. Implements a subset
// of the cgroup v2 interface without POSIX libc dependencies.
//
// Subsystems: cpu, memory, io, network, pids, devices, freezer.
// =============================================================================

#![no_std]

extern crate alloc;
use alloc::{collections::BTreeMap, string::String, vec::Vec};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Maximum cgroup nesting depth.
pub const MAX_CGROUP_DEPTH: usize = 16;
/// Unlimited value sentinel.
pub const CGROUP_UNLIMITED: i64 = -1;

// ---------------------------------------------------------------------------
// Resource Limits
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct CpuLimits {
    /// CPU quota in microseconds per period (CGROUP_UNLIMITED = unlimited).
    pub quota_us: i64,
    /// Period in microseconds (default 100_000 = 100 ms).
    pub period_us: u64,
    /// CPU shares (relative weight, default 1024).
    pub shares: u64,
    /// Cpuset mask (bitmask of allowed CPUs).
    pub cpuset_mask: u64,
}

impl Default for CpuLimits {
    fn default() -> Self {
        CpuLimits {
            quota_us: CGROUP_UNLIMITED,
            period_us: 100_000,
            shares: 1024,
            cpuset_mask: u64::MAX,  // all CPUs
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemoryLimits {
    /// Hard memory limit in bytes (CGROUP_UNLIMITED = unlimited).
    pub limit_bytes: i64,
    /// Soft memory limit (reclaim target).
    pub soft_limit_bytes: i64,
    /// Swap limit in bytes.
    pub swap_limit_bytes: i64,
    /// OOM kill disable flag.
    pub oom_kill_disable: bool,
}

impl Default for MemoryLimits {
    fn default() -> Self {
        MemoryLimits {
            limit_bytes: CGROUP_UNLIMITED,
            soft_limit_bytes: CGROUP_UNLIMITED,
            swap_limit_bytes: CGROUP_UNLIMITED,
            oom_kill_disable: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IoLimits {
    /// Max read bandwidth in bytes/s per device (device_id -> limit).
    pub read_bps: BTreeMap<u32, u64>,
    /// Max write bandwidth in bytes/s per device.
    pub write_bps: BTreeMap<u32, u64>,
    /// Max read IOPS per device.
    pub read_iops: BTreeMap<u32, u64>,
    /// Max write IOPS per device.
    pub write_iops: BTreeMap<u32, u64>,
}

impl Default for IoLimits {
    fn default() -> Self {
        IoLimits {
            read_bps: BTreeMap::new(),
            write_bps: BTreeMap::new(),
            read_iops: BTreeMap::new(),
            write_iops: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NetworkLimits {
    /// Ingress bandwidth limit in bytes/s.
    pub ingress_bps: Option<u64>,
    /// Egress bandwidth limit in bytes/s.
    pub egress_bps: Option<u64>,
    /// Network namespace ID (None = host namespace).
    pub netns_id: Option<u32>,
}

impl Default for NetworkLimits {
    fn default() -> Self {
        NetworkLimits { ingress_bps: None, egress_bps: None, netns_id: None }
    }
}

#[derive(Debug, Clone)]
pub struct PidLimits {
    /// Maximum number of processes/threads (CGROUP_UNLIMITED = unlimited).
    pub max_pids: i64,
}

impl Default for PidLimits {
    fn default() -> Self {
        PidLimits { max_pids: CGROUP_UNLIMITED }
    }
}

// ---------------------------------------------------------------------------
// Resource Accounting (per-cgroup statistics)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default)]
pub struct CpuStats {
    pub usage_us:        u64,   // total CPU time used (µs)
    pub user_us:         u64,
    pub system_us:       u64,
    pub throttled_count: u64,
    pub throttled_time_us: u64,
    pub nr_periods:      u64,
}

#[derive(Debug, Clone, Default)]
pub struct MemoryStats {
    pub usage_bytes:     u64,
    pub rss_bytes:       u64,
    pub cache_bytes:     u64,
    pub swap_bytes:      u64,
    pub oom_kill_count:  u64,
    pub peak_usage_bytes: u64,
}

#[derive(Debug, Clone, Default)]
pub struct IoStats {
    pub read_bytes:  u64,
    pub write_bytes: u64,
    pub read_ops:    u64,
    pub write_ops:   u64,
}

#[derive(Debug, Clone, Default)]
pub struct NetworkStats {
    pub rx_bytes:   u64,
    pub tx_bytes:   u64,
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_dropped: u64,
    pub tx_dropped: u64,
}

#[derive(Debug, Clone, Default)]
pub struct CgroupStats {
    pub cpu:     CpuStats,
    pub memory:  MemoryStats,
    pub io:      IoStats,
    pub network: NetworkStats,
    pub pid_count: u32,
}

// ---------------------------------------------------------------------------
// Cgroup State Machine
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum CgroupState {
    Running,
    Frozen,
    Dying,
}

// ---------------------------------------------------------------------------
// Cgroup Node
// ---------------------------------------------------------------------------

pub struct Cgroup {
    pub id:        u32,
    pub name:      String,
    pub depth:     usize,
    pub state:     CgroupState,
    pub parent_id: Option<u32>,
    pub pids:      Vec<u32>,

    // Limits
    pub cpu_limits:  CpuLimits,
    pub mem_limits:  MemoryLimits,
    pub io_limits:   IoLimits,
    pub net_limits:  NetworkLimits,
    pub pid_limits:  PidLimits,

    // Accounting
    pub stats: CgroupStats,
}

impl Cgroup {
    pub fn new(id: u32, name: &str, parent_id: Option<u32>, depth: usize) -> Self {
        Cgroup {
            id,
            name: String::from(name),
            depth,
            state: CgroupState::Running,
            parent_id,
            pids: Vec::new(),
            cpu_limits: CpuLimits::default(),
            mem_limits: MemoryLimits::default(),
            io_limits: IoLimits::default(),
            net_limits: NetworkLimits::default(),
            pid_limits: PidLimits::default(),
            stats: CgroupStats::default(),
        }
    }

    pub fn attach_pid(&mut self, pid: u32) -> bool {
        if self.pid_limits.max_pids != CGROUP_UNLIMITED
            && self.pids.len() as i64 >= self.pid_limits.max_pids
        {
            return false;  // PID limit exceeded
        }
        if !self.pids.contains(&pid) {
            self.pids.push(pid);
        }
        true
    }

    pub fn detach_pid(&mut self, pid: u32) {
        self.pids.retain(|p| *p != pid);
    }

    /// Freeze all processes in this cgroup (send SIGSTOP semantics).
    pub fn freeze(&mut self) {
        self.state = CgroupState::Frozen;
    }

    /// Thaw (resume) all processes.
    pub fn thaw(&mut self) {
        self.state = CgroupState::Running;
    }

    /// Accumulate CPU usage stats.
    pub fn record_cpu_usage(&mut self, user_us: u64, system_us: u64) {
        self.stats.cpu.user_us += user_us;
        self.stats.cpu.system_us += system_us;
        self.stats.cpu.usage_us += user_us + system_us;
        self.stats.cpu.nr_periods += 1;

        // Check throttling.
        if self.cpu_limits.quota_us != CGROUP_UNLIMITED {
            let quota = self.cpu_limits.quota_us as u64;
            let period = self.cpu_limits.period_us;
            let period_usage = user_us + system_us;
            if period_usage > quota {
                self.stats.cpu.throttled_count += 1;
                self.stats.cpu.throttled_time_us += period_usage - quota;
            }
        }
    }

    /// Accumulate memory accounting.
    pub fn record_memory_usage(&mut self, usage: u64, rss: u64, cache: u64) {
        self.stats.memory.usage_bytes = usage;
        self.stats.memory.rss_bytes = rss;
        self.stats.memory.cache_bytes = cache;
        if usage > self.stats.memory.peak_usage_bytes {
            self.stats.memory.peak_usage_bytes = usage;
        }
    }

    /// Check if this cgroup is within its memory limit.
    pub fn memory_ok(&self) -> bool {
        if self.mem_limits.limit_bytes == CGROUP_UNLIMITED {
            return true;
        }
        self.stats.memory.usage_bytes <= self.mem_limits.limit_bytes as u64
    }

    /// Check if a new process can be added (within PID limit).
    pub fn can_add_pid(&self) -> bool {
        self.pid_limits.max_pids == CGROUP_UNLIMITED
            || (self.pids.len() as i64) < self.pid_limits.max_pids
    }

    pub fn pid_count(&self) -> usize {
        self.pids.len()
    }
}

// ---------------------------------------------------------------------------
// Cgroup Controller (the sovereign cgroup manager)
// ---------------------------------------------------------------------------

pub struct CgroupController {
    cgroups:    BTreeMap<u32, Cgroup>,
    next_id:    u32,
    /// Maps PID -> cgroup ID for fast lookup.
    pid_map:    BTreeMap<u32, u32>,
    /// Root cgroup ID.
    root_id:    u32,
}

impl CgroupController {
    pub fn new() -> Self {
        let mut ctrl = CgroupController {
            cgroups: BTreeMap::new(),
            next_id: 1,
            pid_map: BTreeMap::new(),
            root_id: 1,
        };
        // Create root cgroup.
        let root = Cgroup::new(1, "/", None, 0);
        ctrl.cgroups.insert(1, root);
        ctrl.next_id = 2;
        ctrl
    }

    /// Create a new cgroup under a parent.
    pub fn create(&mut self, name: &str, parent_id: u32) -> Result<u32, &'static str> {
        let depth = self.cgroups.get(&parent_id)
            .map(|p| p.depth + 1)
            .ok_or("Parent cgroup not found")?;
        if depth >= MAX_CGROUP_DEPTH {
            return Err("Maximum cgroup depth exceeded");
        }
        let id = self.next_id;
        self.next_id += 1;
        let cg = Cgroup::new(id, name, Some(parent_id), depth);
        self.cgroups.insert(id, cg);
        Ok(id)
    }

    /// Delete a cgroup (must be empty).
    pub fn delete(&mut self, id: u32) -> Result<(), &'static str> {
        if id == self.root_id {
            return Err("Cannot delete root cgroup");
        }
        let cg = self.cgroups.get(&id).ok_or("Cgroup not found")?;
        if !cg.pids.is_empty() {
            return Err("Cgroup still has attached PIDs");
        }
        // Check for children.
        let has_children = self.cgroups.values().any(|c| c.parent_id == Some(id));
        if has_children {
            return Err("Cgroup has child cgroups");
        }
        self.cgroups.remove(&id);
        Ok(())
    }

    /// Move a PID into a cgroup.
    pub fn attach_pid(&mut self, cgroup_id: u32, pid: u32) -> Result<(), &'static str> {
        // Detach from current cgroup if any.
        if let Some(&old_cg_id) = self.pid_map.get(&pid) {
            if let Some(old_cg) = self.cgroups.get_mut(&old_cg_id) {
                old_cg.detach_pid(pid);
            }
        }
        let cg = self.cgroups.get_mut(&cgroup_id).ok_or("Cgroup not found")?;
        if !cg.attach_pid(pid) {
            return Err("PID limit exceeded for cgroup");
        }
        self.pid_map.insert(pid, cgroup_id);
        Ok(())
    }

    /// Release a PID from its cgroup (process exited).
    pub fn release_pid(&mut self, pid: u32) {
        if let Some(cg_id) = self.pid_map.remove(&pid) {
            if let Some(cg) = self.cgroups.get_mut(&cg_id) {
                cg.detach_pid(pid);
            }
        }
    }

    /// Look up which cgroup a PID belongs to.
    pub fn find_cgroup_for_pid(&self, pid: u32) -> Option<u32> {
        self.pid_map.get(&pid).copied()
    }

    pub fn get(&self, id: u32) -> Option<&Cgroup> {
        self.cgroups.get(&id)
    }

    pub fn get_mut(&mut self, id: u32) -> Option<&mut Cgroup> {
        self.cgroups.get_mut(&id)
    }

    /// Freeze a cgroup and all descendants.
    pub fn freeze_subtree(&mut self, id: u32) {
        let ids: Vec<u32> = self.cgroups.keys().copied().collect();
        for cg_id in ids {
            let is_descendant = {
                let cg = &self.cgroups[&cg_id];
                cg.id == id || self.is_descendant(cg.id, id)
            };
            if is_descendant {
                if let Some(cg) = self.cgroups.get_mut(&cg_id) {
                    cg.freeze();
                }
            }
        }
    }

    fn is_descendant(&self, id: u32, ancestor_id: u32) -> bool {
        let mut current = id;
        loop {
            if let Some(cg) = self.cgroups.get(&current) {
                match cg.parent_id {
                    Some(parent) if parent == ancestor_id => return true,
                    Some(parent) => current = parent,
                    None => return false,
                }
            } else {
                return false;
            }
        }
    }

    pub fn cgroup_count(&self) -> usize {
        self.cgroups.len()
    }

    pub fn root_id(&self) -> u32 {
        self.root_id
    }
}

impl Default for CgroupController {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root_cgroup_exists() {
        let ctrl = CgroupController::new();
        assert!(ctrl.get(ctrl.root_id()).is_some());
    }

    #[test]
    fn test_create_and_delete_cgroup() {
        let mut ctrl = CgroupController::new();
        let id = ctrl.create("test", ctrl.root_id()).unwrap();
        assert!(ctrl.get(id).is_some());
        ctrl.delete(id).unwrap();
        assert!(ctrl.get(id).is_none());
    }

    #[test]
    fn test_pid_attach_detach() {
        let mut ctrl = CgroupController::new();
        let cg_id = ctrl.create("webserver", ctrl.root_id()).unwrap();
        ctrl.attach_pid(cg_id, 1001).unwrap();
        assert_eq!(ctrl.find_cgroup_for_pid(1001), Some(cg_id));
        ctrl.release_pid(1001);
        assert_eq!(ctrl.find_cgroup_for_pid(1001), None);
    }

    #[test]
    fn test_pid_limit() {
        let mut ctrl = CgroupController::new();
        let cg_id = ctrl.create("limited", ctrl.root_id()).unwrap();
        ctrl.get_mut(cg_id).unwrap().pid_limits.max_pids = 2;
        ctrl.attach_pid(cg_id, 100).unwrap();
        ctrl.attach_pid(cg_id, 101).unwrap();
        assert!(ctrl.attach_pid(cg_id, 102).is_err());
    }

    #[test]
    fn test_memory_limit_check() {
        let mut cg = Cgroup::new(1, "test", None, 0);
        cg.mem_limits.limit_bytes = 1024 * 1024;  // 1 MiB
        cg.record_memory_usage(512 * 1024, 400 * 1024, 112 * 1024);
        assert!(cg.memory_ok());
        cg.record_memory_usage(2 * 1024 * 1024, 1_800_000, 200_000);
        assert!(!cg.memory_ok());
    }

    #[test]
    fn test_cpu_throttle_accounting() {
        let mut cg = Cgroup::new(1, "cpu-limited", None, 0);
        cg.cpu_limits.quota_us = 50_000;  // 50 ms per 100 ms period
        cg.record_cpu_usage(60_000, 0);   // used 60 ms → throttled
        assert_eq!(cg.stats.cpu.throttled_count, 1);
        assert_eq!(cg.stats.cpu.throttled_time_us, 10_000);
    }

    #[test]
    fn test_freeze_thaw() {
        let mut ctrl = CgroupController::new();
        let cg_id = ctrl.create("container", ctrl.root_id()).unwrap();
        ctrl.freeze_subtree(cg_id);
        assert_eq!(ctrl.get(cg_id).unwrap().state, CgroupState::Frozen);
        ctrl.get_mut(cg_id).unwrap().thaw();
        assert_eq!(ctrl.get(cg_id).unwrap().state, CgroupState::Running);
    }

    #[test]
    fn test_max_depth() {
        let mut ctrl = CgroupController::new();
        let mut parent = ctrl.root_id();
        for i in 0..MAX_CGROUP_DEPTH {
            parent = ctrl.create(&alloc::format!("d{i}"), parent).unwrap();
        }
        assert!(ctrl.create("too_deep", parent).is_err());
    }
}
