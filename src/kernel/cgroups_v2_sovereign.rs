#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unexpected_cfgs)]
#![allow(clippy::new_without_default)]

#[cfg(not(any(feature = "standalone_test", test)))]


// SigmaOS Sovereign cgroups v2 Resource Accounting
// Implements Linux cgroups v2 unified hierarchy resource controller
// in 100% safe Rust with no external dependencies.
//
// Inspired by Linux kernel cgroups v2 (Linux 4.5+) unified hierarchy.


#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;
#[cfg(not(any(feature = "standalone_test", test)))]
use std::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use std::vec::Vec;

// ─── Resource controller types (mirrors Linux cgroup v2 controllers) ──────────

#[derive(Debug, Clone, PartialEq)]
pub enum CgroupController {
    Cpu,
    CpuSet,
    Io,
    Memory,
    Pids,
    Rdma,
    HugeTlb,
    Misc,
}

impl CgroupController {
    pub fn name(&self) -> &'static str {
        match self {
            CgroupController::Cpu     => "cpu",
            CgroupController::CpuSet  => "cpuset",
            CgroupController::Io      => "io",
            CgroupController::Memory  => "memory",
            CgroupController::Pids    => "pids",
            CgroupController::Rdma    => "rdma",
            CgroupController::HugeTlb => "hugetlb",
            CgroupController::Misc    => "misc",
        }
    }
}

// ─── CPU accounting (mirrors cpu.stat / cpu.weight) ──────────────────────────

#[derive(Debug, Clone)]
pub struct CpuAccounting {
    /// cpu.weight (1–10000), Linux default 100
    pub weight: u32,
    /// cpu.max: bandwidth quota in µs per period
    pub quota_us: u64,
    /// cpu.max: period in µs (default 100000)
    pub period_us: u64,
    /// cumulative usage_usec
    pub usage_usec: u64,
    /// cumulative user_usec
    pub user_usec: u64,
    /// cumulative system_usec
    pub system_usec: u64,
    /// nr_periods — total scheduler periods elapsed
    pub nr_periods: u64,
    /// nr_throttled — periods where cgroup was throttled
    pub nr_throttled: u64,
    /// throttled_usec — total time throttled
    pub throttled_usec: u64,
}

impl CpuAccounting {
    pub fn new() -> Self {
        CpuAccounting {
            weight: 100,
            quota_us: u64::MAX, // unlimited
            period_us: 100_000,
            usage_usec: 0,
            user_usec: 0,
            system_usec: 0,
            nr_periods: 0,
            nr_throttled: 0,
            throttled_usec: 0,
        }
    }

    /// Record a scheduling quantum of `delta_us` µs in user mode.
    pub fn record_user(&mut self, delta_us: u64) {
        self.usage_usec = self.usage_usec.saturating_add(delta_us);
        self.user_usec  = self.user_usec.saturating_add(delta_us);
        self.nr_periods  = self.nr_periods.saturating_add(1);
        // Throttle check: if quota exceeded, accumulate throttled time
        if self.quota_us != u64::MAX && delta_us > self.quota_us {
            let excess = delta_us - self.quota_us;
            self.throttled_usec = self.throttled_usec.saturating_add(excess);
            self.nr_throttled   = self.nr_throttled.saturating_add(1);
        }
    }

    /// Record a scheduling quantum of `delta_us` µs in kernel mode.
    pub fn record_system(&mut self, delta_us: u64) {
        self.usage_usec  = self.usage_usec.saturating_add(delta_us);
        self.system_usec = self.system_usec.saturating_add(delta_us);
    }

    pub fn throttle_ratio(&self) -> f64 {
        if self.nr_periods == 0 { return 0.0; }
        self.nr_throttled as f64 / self.nr_periods as f64
    }
}

// ─── Memory accounting (mirrors memory.stat) ─────────────────────────────────

#[derive(Debug, Clone)]
pub struct MemoryAccounting {
    /// memory.max: hard limit in bytes (u64::MAX = unlimited)
    pub limit_bytes: u64,
    /// memory.high: soft throttle limit (u64::MAX = unlimited)
    pub high_bytes: u64,
    /// current anon usage in bytes
    pub anon_bytes: u64,
    /// current file-backed usage in bytes
    pub file_bytes: u64,
    /// kernel stack bytes
    pub kernel_stack_bytes: u64,
    /// slab bytes (reclaimable + unreclaimable)
    pub slab_bytes: u64,
    /// number of OOM kills in this cgroup
    pub oom_kills: u64,
    /// number of memory.high throttle events
    pub high_events: u64,
    /// number of memory.max limit events
    pub max_events: u64,
}

impl MemoryAccounting {
    pub fn new() -> Self {
        MemoryAccounting {
            limit_bytes: u64::MAX,
            high_bytes: u64::MAX,
            anon_bytes: 0,
            file_bytes: 0,
            kernel_stack_bytes: 0,
            slab_bytes: 0,
            oom_kills: 0,
            high_events: 0,
            max_events: 0,
        }
    }

    pub fn total_usage(&self) -> u64 {
        self.anon_bytes
            .saturating_add(self.file_bytes)
            .saturating_add(self.kernel_stack_bytes)
            .saturating_add(self.slab_bytes)
    }

    /// Try to allocate `bytes`. Returns false if over hard limit.
    pub fn try_alloc(&mut self, bytes: u64) -> bool {
        let new_total = self.total_usage().saturating_add(bytes);
        if new_total > self.limit_bytes {
            self.max_events = self.max_events.saturating_add(1);
            return false;
        }
        if new_total > self.high_bytes {
            self.high_events = self.high_events.saturating_add(1);
            // Would throttle but still permit in soft limit model
        }
        self.anon_bytes = self.anon_bytes.saturating_add(bytes);
        true
    }

    pub fn free(&mut self, bytes: u64) {
        self.anon_bytes = self.anon_bytes.saturating_sub(bytes);
    }

    pub fn trigger_oom(&mut self) {
        self.oom_kills = self.oom_kills.saturating_add(1);
    }
}

// ─── PID accounting (mirrors pids.max / pids.current) ────────────────────────

#[derive(Debug, Clone)]
pub struct PidAccounting {
    pub max_pids: u64,
    pub current_pids: u64,
    pub fork_events: u64,
    pub reject_events: u64,
}

impl PidAccounting {
    pub fn new(max_pids: u64) -> Self {
        PidAccounting { max_pids, current_pids: 0, fork_events: 0, reject_events: 0 }
    }

    pub fn try_fork(&mut self) -> bool {
        if self.current_pids >= self.max_pids {
            self.reject_events = self.reject_events.saturating_add(1);
            return false;
        }
        self.current_pids = self.current_pids.saturating_add(1);
        self.fork_events  = self.fork_events.saturating_add(1);
        true
    }

    pub fn task_exit(&mut self) {
        self.current_pids = self.current_pids.saturating_sub(1);
    }
}

// ─── Cgroup node (unified hierarchy) ─────────────────────────────────────────

#[derive(Debug)]
pub struct CgroupNode {
    pub name: String,
    pub path: String,
    pub enabled_controllers: Vec<CgroupController>,
    pub cpu: CpuAccounting,
    pub memory: MemoryAccounting,
    pub pids: PidAccounting,
    /// Children cgroup names (tree structure)
    pub children: Vec<String>,
    pub frozen: bool,
}

impl CgroupNode {
    pub fn new(name: &str, path: &str, max_pids: u64) -> Self {
        CgroupNode {
            name: name.to_string(),
            path: path.to_string(),
            enabled_controllers: Vec::new(),
            cpu: CpuAccounting::new(),
            memory: MemoryAccounting::new(),
            pids: PidAccounting::new(max_pids),
            children: Vec::new(),
            frozen: false,
        }
    }

    pub fn enable_controller(&mut self, ctrl: CgroupController) {
        if !self.enabled_controllers.contains(&ctrl) {
            self.enabled_controllers.push(ctrl);
        }
    }

    pub fn set_cpu_weight(&mut self, weight: u32) -> bool {
        if weight == 0 || weight > 10_000 { return false; }
        self.cpu.weight = weight;
        true
    }

    pub fn set_memory_limit(&mut self, bytes: u64) {
        self.memory.limit_bytes = bytes;
    }

    pub fn freeze(&mut self) { self.frozen = true; }
    pub fn thaw(&mut self)   { self.frozen = false; }

    pub fn stat_summary(&self) -> String {
        let mut s = String::new();
        s.push_str("cgroup: ");
        s.push_str(&self.path);
        s.push('\n');
        s.push_str("  cpu.weight=");
        s.push_str(&self.cpu.weight.to_string());
        s.push_str(" usage_usec=");
        s.push_str(&self.cpu.usage_usec.to_string());
        s.push_str(" throttled=");
        s.push_str(&self.cpu.nr_throttled.to_string());
        s.push('\n');
        s.push_str("  memory.current=");
        s.push_str(&self.memory.total_usage().to_string());
        s.push_str(" oom_kills=");
        s.push_str(&self.memory.oom_kills.to_string());
        s.push('\n');
        s.push_str("  pids.current=");
        s.push_str(&self.pids.current_pids.to_string());
        s.push_str("/");
        s.push_str(&self.pids.max_pids.to_string());
        s.push('\n');
        s
    }
}

// ─── Sovereign cgroups v2 Manager ────────────────────────────────────────────

pub struct SovereignCgroupsV2Manager {
    pub cgroups: Vec<CgroupNode>,
    pub total_fork_count: u64,
}

impl SovereignCgroupsV2Manager {
    pub fn new() -> Self {
        let mut mgr = SovereignCgroupsV2Manager {
            cgroups: Vec::new(),
            total_fork_count: 0,
        };
        // Create root cgroup "/" like Linux
        let mut root = CgroupNode::new("root", "/", u64::MAX);
        root.enable_controller(CgroupController::Cpu);
        root.enable_controller(CgroupController::Memory);
        root.enable_controller(CgroupController::Pids);
        mgr.cgroups.push(root);
        mgr
    }

    pub fn create_cgroup(&mut self, name: &str, parent_path: &str, max_pids: u64) -> bool {
        // Verify parent exists
        let parent_exists = self.cgroups.iter().any(|c| c.path == parent_path);
        if !parent_exists { return false; }

        let path = if parent_path == "/" {
            let mut p = String::from("/");
            p.push_str(name);
            p
        } else {
            let mut p = parent_path.to_string();
            p.push('/');
            p.push_str(name);
            p
        };

        // Prevent duplicate
        if self.cgroups.iter().any(|c| c.path == path) { return false; }

        // Add child reference to parent
        if let Some(parent) = self.cgroups.iter_mut().find(|c| c.path == parent_path) {
            parent.children.push(path.clone());
        }

        let node = CgroupNode::new(name, &path, max_pids);
        self.cgroups.push(node);
        true
    }

    pub fn get_cgroup_mut(&mut self, path: &str) -> Option<&mut CgroupNode> {
        self.cgroups.iter_mut().find(|c| c.path == path)
    }

    pub fn record_cpu_usage(&mut self, path: &str, user_us: u64, sys_us: u64) -> bool {
        if let Some(cg) = self.get_cgroup_mut(path) {
            cg.cpu.record_user(user_us);
            cg.cpu.record_system(sys_us);
            true
        } else { false }
    }

    pub fn try_alloc_memory(&mut self, path: &str, bytes: u64) -> bool {
        if let Some(cg) = self.get_cgroup_mut(path) {
            cg.memory.try_alloc(bytes)
        } else { false }
    }

    pub fn try_fork(&mut self, path: &str) -> bool {
        if let Some(cg) = self.get_cgroup_mut(path) {
            if cg.frozen { return false; }
            let ok = cg.pids.try_fork();
            if ok { self.total_fork_count = self.total_fork_count.saturating_add(1); }
            ok
        } else { false }
    }

    pub fn cgroup_count(&self) -> usize { self.cgroups.len() }

    pub fn full_dump(&self) -> String {
        let mut out = String::from("=== SigmaOS cgroups v2 Hierarchy ===\n");
        for cg in &self.cgroups {
            out.push_str(&cg.stat_summary());
        }
        out
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cgroups_v2_hierarchy_creation() {
        let mut mgr = SovereignCgroupsV2Manager::new();
        assert!(mgr.create_cgroup("system", "/", 1024));
        assert!(mgr.create_cgroup("user.slice", "/", 65536));
        assert!(mgr.create_cgroup("init.scope", "/system", 64));
        assert_eq!(mgr.cgroup_count(), 4); // root + 3
    }

    #[test]
    fn test_cpu_accounting_throttle() {
        let mut cpu = CpuAccounting::new();
        cpu.quota_us = 10_000; // 10ms quota
        cpu.record_user(15_000); // 15ms — exceeds quota
        assert_eq!(cpu.nr_throttled, 1);
        assert!(cpu.throttle_ratio() > 0.0);
    }

    #[test]
    fn test_memory_limit_enforcement() {
        let mut mgr = SovereignCgroupsV2Manager::new();
        mgr.create_cgroup("limited", "/", 100);
        // Set 1MB memory limit
        if let Some(cg) = mgr.get_cgroup_mut("/limited") {
            cg.set_memory_limit(1024 * 1024);
        }
        assert!(mgr.try_alloc_memory("/limited", 512 * 1024));  // 512KB — OK
        assert!(!mgr.try_alloc_memory("/limited", 600 * 1024)); // 600KB — exceeds 1MB total
    }

    #[test]
    fn test_pid_accounting_fork_limit() {
        let mut mgr = SovereignCgroupsV2Manager::new();
        mgr.create_cgroup("fork_test", "/", 3);
        assert!(mgr.try_fork("/fork_test"));
        assert!(mgr.try_fork("/fork_test"));
        assert!(mgr.try_fork("/fork_test"));
        assert!(!mgr.try_fork("/fork_test")); // Over limit
        assert_eq!(mgr.total_fork_count, 3);
    }

    #[test]
    fn test_cgroup_freeze_thaw() {
        let mut mgr = SovereignCgroupsV2Manager::new();
        mgr.create_cgroup("freeze_test", "/", 10);
        if let Some(cg) = mgr.get_cgroup_mut("/freeze_test") {
            cg.freeze();
        }
        assert!(!mgr.try_fork("/freeze_test")); // Frozen — reject
        if let Some(cg) = mgr.get_cgroup_mut("/freeze_test") {
            cg.thaw();
        }
        assert!(mgr.try_fork("/freeze_test"));  // Thawed — permit
    }

    #[test]
    fn test_cpu_weight_validation() {
        let mut cg = CgroupNode::new("test", "/test", 100);
        assert!(cg.set_cpu_weight(500));
        assert!(!cg.set_cpu_weight(0));       // Invalid
        assert!(!cg.set_cpu_weight(10_001));  // Invalid
        assert_eq!(cg.cpu.weight, 500);
    }
}
