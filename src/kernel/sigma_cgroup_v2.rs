//! SigmaOS cgroup v2 (Unified Hierarchy) Resource Controllers
//!
//! Sovereign implementation of Linux cgroup v2 unified hierarchy.
//! Provides CPU, Memory, I/O, PID, and Freezer controllers.
//!
//! # Design Principles
//! - Single unified hierarchy (no v1 per-subsystem mount)
//! - Pull model: processes attach to leaf cgroups
//! - Controllers enabled by writing to cgroup.controllers
//! - Delegation model: parent grants controllers to children
//!
//! # References
//! - Linux Documentation/admin-guide/cgroup-v2.rst
//! - FreeBSD RCTL/RACCT resource controls

#![allow(dead_code)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

// ============================================================
// Controller Types
// ============================================================

/// Available cgroup v2 resource controllers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CgroupController {
    /// CPU scheduling weight and quota
    Cpu,
    /// Memory limits and accounting
    Memory,
    /// Block I/O weight and rate limits
    Io,
    /// Process count limits
    Pids,
    /// Freeze/thaw all tasks in cgroup
    Freezer,
    /// Network I/O classification (tc integration)
    Net,
    /// Hardware performance counters delegation
    Perf,
    /// Huge TLB page limits
    HugeTlb,
    /// RDMA device limits
    Rdma,
}

impl CgroupController {
    /// Returns the controller name as it appears in cgroup.controllers.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cpu => "cpu",
            Self::Memory => "memory",
            Self::Io => "io",
            Self::Pids => "pids",
            Self::Freezer => "freezer",
            Self::Net => "net",
            Self::Perf => "perf_event",
            Self::HugeTlb => "hugetlb",
            Self::Rdma => "rdma",
        }
    }
}

// ============================================================
// CPU Controller
// ============================================================

/// CPU controller configuration (cpu.* files).
///
/// # Comparison with Linux cgroup v2 cpu controller:
/// - cpu.weight: 1–10000 (default 100), replaces cpu.shares
/// - cpu.max: "$QUOTA $PERIOD" — bandwidth limiting
/// - cpu.stat: usage, throttled periods, etc.
#[derive(Debug, Clone)]
pub struct CgroupCpuController {
    /// Scheduling weight 1–10000 (default 100)
    pub weight: u32,
    /// CPU quota in microseconds per period (u64::MAX = unlimited)
    pub quota_us: u64,
    /// Period in microseconds (default 100000 = 100ms)
    pub period_us: u64,
    /// Total CPU usage in nanoseconds (accounting)
    pub usage_ns: u64,
    /// Number of periods in which the cgroup was throttled
    pub throttled_periods: u64,
    /// Total time throttled in nanoseconds
    pub throttled_ns: u64,
}

impl Default for CgroupCpuController {
    fn default() -> Self {
        Self {
            weight: 100,
            quota_us: u64::MAX,
            period_us: 100_000,
            usage_ns: 0,
            throttled_periods: 0,
            throttled_ns: 0,
        }
    }
}

impl CgroupCpuController {
    /// Check whether this cgroup is currently throttled.
    pub fn is_throttled(&self) -> bool {
        self.quota_us != u64::MAX && self.usage_ns >= self.quota_us * 1000
    }

    /// Record CPU usage (called by scheduler on context switch).
    pub fn charge_cpu_ns(&mut self, ns: u64) {
        self.usage_ns = self.usage_ns.saturating_add(ns);
    }

    /// Reset period counters (called at start of new period).
    pub fn new_period(&mut self) {
        if self.is_throttled() {
            self.throttled_periods += 1;
        }
        self.usage_ns = 0;
    }
}

// ============================================================
// Memory Controller
// ============================================================

/// Memory controller configuration (memory.* files).
#[derive(Debug, Clone)]
pub struct CgroupMemController {
    /// Memory usage limit in bytes (u64::MAX = unlimited)
    pub memory_max: u64,
    /// Memory + swap limit in bytes
    pub memory_swap_max: u64,
    /// Soft limit — reclaim starts here
    pub memory_high: u64,
    /// Current memory usage in bytes
    pub memory_current: u64,
    /// Current swap usage in bytes
    pub swap_current: u64,
    /// OOM kill count
    pub oom_kill_count: u64,
}

impl Default for CgroupMemController {
    fn default() -> Self {
        Self {
            memory_max: u64::MAX,
            memory_swap_max: u64::MAX,
            memory_high: u64::MAX,
            memory_current: 0,
            swap_current: 0,
            oom_kill_count: 0,
        }
    }
}

impl CgroupMemController {
    /// Charge memory allocation. Returns Err if over limit.
    pub fn charge(&mut self, bytes: u64) -> Result<(), &'static str> {
        let new_usage = self.memory_current.saturating_add(bytes);
        if new_usage > self.memory_max {
            return Err("memory.max exceeded");
        }
        self.memory_current = new_usage;
        Ok(())
    }

    /// Uncharge memory (on free).
    pub fn uncharge(&mut self, bytes: u64) {
        self.memory_current = self.memory_current.saturating_sub(bytes);
    }

    /// Returns true if usage exceeds the high watermark.
    pub fn is_over_high(&self) -> bool {
        self.memory_current > self.memory_high
    }
}

// ============================================================
// PID Controller
// ============================================================

/// PID controller — limits number of processes/threads.
#[derive(Debug, Clone)]
pub struct CgroupPidsController {
    /// Maximum PIDs allowed (u64::MAX = unlimited)
    pub pids_max: u64,
    /// Current PID count
    pub pids_current: u64,
}

impl Default for CgroupPidsController {
    fn default() -> Self {
        Self { pids_max: u64::MAX, pids_current: 0 }
    }
}

impl CgroupPidsController {
    /// Try to fork — returns Err if at limit.
    pub fn try_fork(&mut self) -> Result<(), &'static str> {
        if self.pids_current >= self.pids_max {
            return Err("pids.max exceeded");
        }
        self.pids_current += 1;
        Ok(())
    }

    /// Called when a process exits.
    pub fn on_exit(&mut self) {
        self.pids_current = self.pids_current.saturating_sub(1);
    }
}

// ============================================================
// Freezer
// ============================================================

/// Freezer state for a cgroup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FreezerState {
    /// All tasks are running normally
    Thawed,
    /// Freeze in progress (tasks being stopped)
    Freezing,
    /// All tasks frozen (SIGSTOP sent)
    Frozen,
}

// ============================================================
// CgroupNode — Tree Node
// ============================================================

/// A single node in the cgroup v2 unified hierarchy.
///
/// # Abstraction
/// Each node represents one directory in the cgroupfs virtual
/// filesystem. The root node is `/sys/fs/cgroup/`.
pub struct CgroupNode {
    /// Node name (last component of path)
    name: String,
    /// Full path from root
    path: String,
    /// Parent node ID (None for root)
    parent_id: Option<u64>,
    /// Child node IDs
    children: Vec<u64>,
    /// Tasks (PIDs) attached to this cgroup
    tasks: Vec<u32>,
    /// Enabled controllers (subset of parent's subtree_control)
    enabled_controllers: Vec<CgroupController>,
    /// CPU controller settings
    pub cpu: CgroupCpuController,
    /// Memory controller settings
    pub mem: CgroupMemController,
    /// PID controller settings
    pub pids: CgroupPidsController,
    /// Freezer state
    pub freezer: FreezerState,
    /// Node ID
    id: u64,
}

impl CgroupNode {
    /// Create a new cgroup node.
    pub fn new(id: u64, name: &str, path: &str, parent_id: Option<u64>) -> Self {
        Self {
            id,
            name: name.into(),
            path: path.into(),
            parent_id,
            children: Vec::new(),
            tasks: Vec::new(),
            enabled_controllers: Vec::new(),
            cpu: CgroupCpuController::default(),
            mem: CgroupMemController::default(),
            pids: CgroupPidsController::default(),
            freezer: FreezerState::Thawed,
        }
    }

    /// Attach a task (PID) to this cgroup.
    pub fn attach_task(&mut self, pid: u32) {
        if !self.tasks.contains(&pid) {
            self.tasks.push(pid);
        }
    }

    /// Detach a task from this cgroup.
    pub fn detach_task(&mut self, pid: u32) {
        self.tasks.retain(|&p| p != pid);
    }

    /// Enable a controller on this node.
    pub fn enable_controller(&mut self, ctrl: CgroupController) {
        if !self.enabled_controllers.contains(&ctrl) {
            self.enabled_controllers.push(ctrl);
        }
    }

    /// Returns true if this node is a leaf (no children).
    pub fn is_leaf(&self) -> bool { self.children.is_empty() }

    /// Returns the node name.
    pub fn name(&self) -> &str { &self.name }
    /// Returns the full path.
    pub fn path(&self) -> &str { &self.path }
    /// Returns the node ID.
    pub fn id(&self) -> u64 { self.id }
    /// Returns task list.
    pub fn tasks(&self) -> &[u32] { &self.tasks }
    /// Returns enabled controllers.
    pub fn enabled_controllers(&self) -> &[CgroupController] { &self.enabled_controllers }
}

// ============================================================
// SigmaCgroupV2 — Unified Cgroup Hierarchy
// ============================================================

/// The root cgroup v2 hierarchy manager.
///
/// Manages the full tree of cgroup nodes, enforces limits,
/// and provides Linux-compatible cgroup v2 semantics.
pub struct SigmaCgroupV2 {
    /// All nodes indexed by ID
    nodes: BTreeMap<u64, CgroupNode>,
    /// Root node ID
    root_id: u64,
    /// Next node ID counter
    next_id: u64,
    /// Map from PID to cgroup node ID (task → cgroup)
    task_map: BTreeMap<u32, u64>,
}

impl SigmaCgroupV2 {
    /// Create a new cgroup v2 hierarchy with a root node.
    pub fn new() -> Self {
        let mut nodes = BTreeMap::new();
        let root = CgroupNode::new(1, "/", "/sys/fs/cgroup", None);
        nodes.insert(1, root);
        Self {
            nodes,
            root_id: 1,
            next_id: 2,
            task_map: BTreeMap::new(),
        }
    }

    /// Create a child cgroup under `parent_id`.
    ///
    /// # Returns
    /// The new node's ID, or Err if parent doesn't exist.
    pub fn mkdir(&mut self, parent_id: u64, name: &str) -> Result<u64, &'static str> {
        let parent_path = self.nodes.get(&parent_id)
            .map(|n| n.path.clone())
            .ok_or("parent cgroup not found")?;

        let new_id = self.next_id;
        self.next_id += 1;

        let path = if parent_path == "/sys/fs/cgroup" {
            alloc::format!("/sys/fs/cgroup/{}", name)
        } else {
            alloc::format!("{}/{}", parent_path, name)
        };

        let node = CgroupNode::new(new_id, name, &path, Some(parent_id));
        self.nodes.insert(new_id, node);

        // Register as child of parent
        if let Some(parent) = self.nodes.get_mut(&parent_id) {
            parent.children.push(new_id);
        }
        Ok(new_id)
    }

    /// Remove a leaf cgroup (must have no tasks, no children).
    pub fn rmdir(&mut self, id: u64) -> Result<(), &'static str> {
        let (parent_id, is_leaf, has_tasks) = {
            let node = self.nodes.get(&id).ok_or("cgroup not found")?;
            if id == self.root_id { return Err("cannot remove root cgroup"); }
            (node.parent_id, node.is_leaf(), !node.tasks.is_empty())
        };
        if !is_leaf { return Err("cgroup has children"); }
        if has_tasks { return Err("cgroup has active tasks"); }
        self.nodes.remove(&id);
        if let Some(pid) = parent_id {
            if let Some(parent) = self.nodes.get_mut(&pid) {
                parent.children.retain(|&c| c != id);
            }
        }
        Ok(())
    }

    /// Attach a task (PID) to a cgroup.
    pub fn attach_task(&mut self, pid: u32, cgroup_id: u64) -> Result<(), &'static str> {
        // Detach from current cgroup if any
        if let Some(&old_id) = self.task_map.get(&pid) {
            if let Some(old_node) = self.nodes.get_mut(&old_id) {
                old_node.detach_task(pid);
            }
        }
        // Check PID limit before attaching
        let node = self.nodes.get_mut(&cgroup_id).ok_or("cgroup not found")?;
        node.pids.try_fork()?;
        node.attach_task(pid);
        self.task_map.insert(pid, cgroup_id);
        Ok(())
    }

    /// Called when a task exits — remove from cgroup.
    pub fn on_task_exit(&mut self, pid: u32) {
        if let Some(cgroup_id) = self.task_map.remove(&pid) {
            if let Some(node) = self.nodes.get_mut(&cgroup_id) {
                node.detach_task(pid);
                node.pids.on_exit();
            }
        }
    }

    /// Charge memory allocation to a task's cgroup.
    pub fn charge_memory(&mut self, pid: u32, bytes: u64) -> Result<(), &'static str> {
        let cgroup_id = *self.task_map.get(&pid).ok_or("task not in any cgroup")?;
        let node = self.nodes.get_mut(&cgroup_id).ok_or("cgroup not found")?;
        node.mem.charge(bytes)
    }

    /// Uncharge memory from a task's cgroup.
    pub fn uncharge_memory(&mut self, pid: u32, bytes: u64) {
        if let Some(&cgroup_id) = self.task_map.get(&pid) {
            if let Some(node) = self.nodes.get_mut(&cgroup_id) {
                node.mem.uncharge(bytes);
            }
        }
    }

    /// Freeze all tasks in a cgroup.
    pub fn freeze(&mut self, cgroup_id: u64) -> Result<(), &'static str> {
        let node = self.nodes.get_mut(&cgroup_id).ok_or("cgroup not found")?;
        node.freezer = FreezerState::Frozen;
        Ok(())
    }

    /// Thaw a frozen cgroup.
    pub fn thaw(&mut self, cgroup_id: u64) -> Result<(), &'static str> {
        let node = self.nodes.get_mut(&cgroup_id).ok_or("cgroup not found")?;
        node.freezer = FreezerState::Thawed;
        Ok(())
    }

    /// Get a reference to a cgroup node.
    pub fn get(&self, id: u64) -> Option<&CgroupNode> { self.nodes.get(&id) }
    /// Get a mutable reference to a cgroup node.
    pub fn get_mut(&mut self, id: u64) -> Option<&mut CgroupNode> { self.nodes.get_mut(&id) }
    /// Returns the root node ID.
    pub fn root_id(&self) -> u64 { self.root_id }
    /// Returns total cgroup count.
    pub fn count(&self) -> usize { self.nodes.len() }
    /// Returns the cgroup ID for a given PID.
    pub fn task_cgroup(&self, pid: u32) -> Option<u64> { self.task_map.get(&pid).copied() }
}

impl Default for SigmaCgroupV2 {
    fn default() -> Self { Self::new() }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hierarchy() {
        let mut cg = SigmaCgroupV2::new();
        let sys_id = cg.mkdir(1, "system.slice").unwrap();
        let svc_id = cg.mkdir(sys_id, "sshd.service").unwrap();
        assert_eq!(cg.count(), 3); // root + system.slice + sshd.service
        assert!(cg.get(svc_id).unwrap().is_leaf());
    }

    #[test]
    fn test_attach_task() {
        let mut cg = SigmaCgroupV2::new();
        let id = cg.mkdir(1, "test").unwrap();
        assert!(cg.attach_task(1234, id).is_ok());
        assert_eq!(cg.task_cgroup(1234), Some(id));
        assert!(cg.get(id).unwrap().tasks().contains(&1234));
    }

    #[test]
    fn test_memory_charge() {
        let mut cg = SigmaCgroupV2::new();
        let id = cg.mkdir(1, "limited").unwrap();
        cg.get_mut(id).unwrap().mem.memory_max = 1024 * 1024; // 1MB
        cg.attach_task(100, id).unwrap();
        assert!(cg.charge_memory(100, 512 * 1024).is_ok());
        assert!(cg.charge_memory(100, 1024 * 1024).is_err()); // Over limit
    }

    #[test]
    fn test_pid_limit() {
        let mut cg = SigmaCgroupV2::new();
        let id = cg.mkdir(1, "restricted").unwrap();
        cg.get_mut(id).unwrap().pids.pids_max = 2;
        assert!(cg.attach_task(1, id).is_ok());
        assert!(cg.attach_task(2, id).is_ok());
        assert!(cg.attach_task(3, id).is_err()); // Over limit
    }

    #[test]
    fn test_freeze_thaw() {
        let mut cg = SigmaCgroupV2::new();
        let id = cg.mkdir(1, "freezable").unwrap();
        cg.freeze(id).unwrap();
        assert_eq!(cg.get(id).unwrap().freezer, FreezerState::Frozen);
        cg.thaw(id).unwrap();
        assert_eq!(cg.get(id).unwrap().freezer, FreezerState::Thawed);
    }
}
