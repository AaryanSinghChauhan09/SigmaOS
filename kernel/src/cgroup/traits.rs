// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// Linux-inspired cgroups (control groups) for SigmaOS
// Zero-allocation, performance-optimized resource control

/// Control group trait
pub trait ControlGroup {
    /// Get cgroup name
    fn name(&self) -> &str;
    
    /// Get cgroup path
    fn path(&self) -> &str;
    
    /// Get cgroup parent
    fn parent(&self) -> Option<&str>;
    
    /// Add process to cgroup
    fn add_process(&mut self, pid: u32) -> Result<(), CgroupError>;
    
    /// Remove process from cgroup
    fn remove_process(&mut self, pid: u32) -> Result<(), CgroupError>;
    
    /// List processes in cgroup
    fn list_processes(&self) -> Vec<u32>;
    
    /// Get cgroup statistics
    fn stats(&self) -> CgroupStats;
}

/// Cgroup subsystem
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CgroupSubsystem {
    Cpu,
    Cpuacct,
    Cpuset,
    Memory,
    Devices,
    Freezer,
    NetCls,
    Blkio,
    PerfEvent,
    NetPrio,
    Hugetlb,
    Pids,
    Rdma,
    Misc,
}

/// Cgroup controller
pub trait CgroupController {
    /// Get controller name
    fn name(&self) -> &str;
    
    /// Get controller type
    fn controller_type(&self) -> CgroupSubsystem;
    
    /// Set parameter
    fn set_parameter(&mut self, param: &str, value: &str) -> Result<(), CgroupError>;
    
    /// Get parameter
    fn get_parameter(&self, param: &str) -> Option<String>;
    
    /// Get controller statistics
    fn stats(&self) -> ControllerStats;
}

/// CPU controller
pub struct CpuController {
    pub shares: u64,
    pub quota: i64,
    pub period: u64,
    pub rt_runtime: i64,
    pub rt_period: u64,
    pub stats: CpuStats,
}

impl CpuController {
    pub const fn new() -> Self {
        Self {
            shares: 1024,
            quota: -1,
            period: 100000,
            rt_runtime: -1,
            rt_period: 1000000,
            stats: CpuStats::new(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CpuStats {
    pub nr_periods: u64,
    pub nr_throttled: u64,
    pub throttled_time: u64,
    pub usage_usec: u64,
    pub user_usec: u64,
    pub system_usec: u64,
}

impl CpuStats {
    pub const fn new() -> Self {
        Self {
            nr_periods: 0,
            nr_throttled: 0,
            throttled_time: 0,
            usage_usec: 0,
            user_usec: 0,
            system_usec: 0,
        }
    }
}

/// Memory controller
pub struct MemoryController {
    pub limit: u64,
    pub soft_limit: u64,
    pub swap_limit: u64,
    pub oom_control: bool,
    pub stats: MemoryStats,
}

impl MemoryController {
    pub const fn new() -> Self {
        Self {
            limit: u64::MAX,
            soft_limit: u64::MAX,
            swap_limit: u64::MAX,
            oom_control: true,
            stats: MemoryStats::new(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MemoryStats {
    pub cache: u64,
    pub rss: u64,
    pub rss_huge: u64,
    pub mapped_file: u64,
    pub dirty: u64,
    pub writeback: u64,
    pub swap: u64,
    pub pgpgin: u64,
    pub pgpgout: u64,
    pub pgfault: u64,
    pub pgmajfault: u64,
    pub total_inactive_anon: u64,
    pub total_active_anon: u64,
    pub total_inactive_file: u64,
    pub total_active_file: u64,
    pub total_unevictable: u64,
}

impl MemoryStats {
    pub const fn new() -> Self {
        Self {
            cache: 0,
            rss: 0,
            rss_huge: 0,
            mapped_file: 0,
            dirty: 0,
            writeback: 0,
            swap: 0,
            pgpgin: 0,
            pgpgout: 0,
            pgfault: 0,
            pgmajfault: 0,
            total_inactive_anon: 0,
            total_active_anon: 0,
            total_inactive_file: 0,
            total_active_file: 0,
            total_unevictable: 0,
        }
    }
}

/// PIDs controller
pub struct PidsController {
    pub max: i64,
    pub current: i64,
}

impl PidsController {
    pub const fn new() -> Self {
        Self {
            max: -1,
            current: 0,
        }
    }
}

/// Cgroup statistics
pub struct CgroupStats {
    pub nr_processes: u32,
    pub nr_descendants: u32,
    pub nr_dying_descendants: u32,
}

impl CgroupStats {
    pub const fn new() -> Self {
        Self {
            nr_processes: 0,
            nr_descendants: 0,
            nr_dying_descendants: 0,
        }
    }
}

/// Controller statistics
pub struct ControllerStats {
    pub cpu: Option<CpuStats>,
    pub memory: Option<MemoryStats>,
    pub pids: Option<PidsStats>,
}

#[derive(Debug, Clone, Copy)]
pub struct PidsStats {
    pub current: i64,
    pub max: i64,
}

/// Cgroup hierarchy
pub struct CgroupHierarchy {
    pub root: CgroupNode,
    pub subsystems: Vec<CgroupSubsystem>,
}

pub struct CgroupNode {
    pub name: String,
    pub path: String,
    pub children: Vec<CgroupNode>,
    pub controllers: Vec<Box<dyn CgroupController>>,
}

impl CgroupNode {
    pub fn new(name: &str, path: &str) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_string(),
            children: Vec::new(),
            controllers: Vec::new(),
        }
    }
    
    pub fn add_child(&mut self, child: CgroupNode) {
        self.children.push(child);
    }
    
    pub fn add_controller(&mut self, controller: Box<dyn CgroupController>) {
        self.controllers.push(controller);
    }
}

/// Cgroup manager
pub trait CgroupManager {
    /// Initialize cgroup manager
    fn init(&mut self) -> Result<(), CgroupError>;
    
    /// Create cgroup
    fn create_cgroup(&mut self, path: &str) -> Result<(), CgroupError>;
    
    /// Remove cgroup
    fn remove_cgroup(&mut self, path: &str) -> Result<(), CgroupError>;
    
    /// Get cgroup
    fn get_cgroup(&self, path: &str) -> Option<&dyn ControlGroup>;
    
    /// List cgroups
    fn list_cgroups(&self) -> Vec<&str>;
    
    /// Mount subsystem
    fn mount_subsystem(&mut self, subsystem: CgroupSubsystem, path: &str) -> Result<(), CgroupError>;
    
    /// Unmount subsystem
    fn unmount_subsystem(&mut self, path: &str) -> Result<(), CgroupError>;
}

/// Cgroup error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CgroupError {
    CgroupNotFound,
    CgroupExists,
    PermissionDenied,
    InvalidPath,
    SubsystemNotFound,
    ControllerNotFound,
    ProcessNotFound,
    InvalidParameter,
    ResourceLimit,
    Other,
}

/// Cgroup v2 unified hierarchy
pub struct CgroupV2 {
    pub unified: bool,
    pub controllers: Vec<CgroupSubsystem>,
}

impl CgroupV2 {
    pub const fn new() -> Self {
        Self {
            unified: false,
            controllers: Vec::new(),
        }
    }
    
    pub fn is_unified(&self) -> bool {
        self.unified
    }
}

/// Cgroup mount options
pub mod mount_options {
    pub const MEMORY: &str = "memory";
    pub const CPU: &str = "cpu";
    pub const CPUSET: &str = "cpuset";
    pub const CPUACCT: &str = "cpuacct";
    pub const DEVICES: &str = "devices";
    pub const FREEZER: &str = "freezer";
    pub const NET_CLS: &str = "net_cls";
    pub const NET_PRIO: &str = "net_prio";
    pub const BLKIO: &str = "blkio";
    pub const PERF_EVENT: &str = "perf_event";
    pub const HUGETLB: &str = "hugetlb";
    pub const PIDS: &str = "pids";
    pub const RDMA: &str = "rdma";
    pub const MISC: &str = "misc";
}

/// Standard cgroup paths
pub mod paths {
    pub const CGROUP_ROOT: &str = "/sys/fs/cgroup";
    pub const CGROUP_V2_ROOT: &str = "/sys/fs/cgroup/unified";
    pub const CGROUP_CPU: &str = "/sys/fs/cgroup/cpu";
    pub const CGROUP_CPUACCT: &str = "/sys/fs/cgroup/cpuacct";
    pub const CGROUP_CPUSET: &str = "/sys/fs/cgroup/cpuset";
    pub const CGROUP_MEMORY: &str = "/sys/fs/cgroup/memory";
    pub const CGROUP_DEVICES: &str = "/sys/fs/cgroup/devices";
    pub const CGROUP_FREEZER: &str = "/sys/fs/cgroup/freezer";
    pub const CGROUP_NET_CLS: &str = "/sys/fs/cgroup/net_cls";
    pub const CGROUP_NET_PRIO: &str = "/sys/fs/cgroup/net_prio";
    pub const CGROUP_BLKIO: &str = "/sys/fs/cgroup/blkio";
    pub const CGROUP_PERF_EVENT: &str = "/sys/fs/cgroup/perf_event";
    pub const CGROUP_HUGETLB: &str = "/sys/fs/cgroup/hugetlb";
    pub const CGROUP_PIDS: &str = "/sys/fs/cgroup/pids";
    pub const CGROUP_RDMA: &str = "/sys/fs/cgroup/rdma";
    pub const CGROUP_MISC: &str = "/sys/fs/cgroup/misc";
    pub const SYSTEMD_CGROUP: &str = "/sys/fs/cgroup/systemd";
}
