//! # Cgroup V2 Controller Implementation
//!
//! Linux cgroup v2 unified hierarchy controller.
//! Provides resource limits and accounting for processes.

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

/// Cgroup controller types (Linux cgroup v2 controllers)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum ControllerType {
    /// CPU controller
    Cpu = 0,
    /// Memory controller
    Memory = 1,
    /// I/O controller
    Io = 2,
    /// Process number controller
    Pids = 3,
    /// CPU set controller
    Cpuset = 4,
    /// Perf events controller
    PerfEvent = 5,
    /// Freezer controller
    Freezer = 6,
}

/// CPU controller configuration
#[derive(Debug, Clone, Copy)]
pub struct CpuController {
    /// CPU weight (1-10000, default 100)
    pub weight: u32,
    /// CPU quota (microseconds per period)
    pub quota: i64,
    /// CPU period (microseconds, default 100000 = 100ms)
    pub period: u64,
    /// Maximum CPU usage (percent)
    pub max: u32,
}

impl CpuController {
    pub const DEFAULT: Self = Self {
        weight: 100,
        quota: -1, // Unlimited
        period: 100000,
        max: 100,
    };
}

/// Memory controller configuration
#[derive(Debug)]
pub struct MemoryController {
    /// Memory limit (bytes, 0 = unlimited)
    pub max: u64,
    /// Memory low watermark (best-effort protection)
    pub low: u64,
    /// Memory high watermark (throttle threshold)
    pub high: u64,
    /// Current memory usage
    pub current: AtomicU64,
    /// Swap limit (bytes)
    pub swap_max: u64,
}

impl MemoryController {
    pub const UNLIMITED: Self = Self {
        max: u64::MAX,
        low: 0,
        high: u64::MAX,
        current: AtomicU64::new(0),
        swap_max: u64::MAX,
    };

    pub fn charge(&self, bytes: u64) -> Result<(), CgroupError> {
        let new_usage = self.current.fetch_add(bytes, Ordering::SeqCst) + bytes;
        if new_usage > self.max {
            self.current.fetch_sub(bytes, Ordering::SeqCst);
            return Err(CgroupError::MemoryLimit);
        }
        Ok(())
    }

    pub fn uncharge(&self, bytes: u64) {
        self.current.fetch_sub(bytes, Ordering::SeqCst);
    }

    pub fn usage(&self) -> u64 {
        self.current.load(Ordering::SeqCst)
    }
}

/// I/O controller configuration
#[derive(Debug, Clone, Copy)]
pub struct IoController {
    /// Read bytes per second limit
    pub rbps: u64,
    /// Write bytes per second limit
    pub wbps: u64,
    /// Read IOPS limit
    pub riops: u64,
    /// Write IOPS limit
    pub wiops: u64,
    /// I/O weight (1-10000, default 100)
    pub weight: u32,
}

impl IoController {
    pub const DEFAULT: Self = Self {
        rbps: u64::MAX,
        wbps: u64::MAX,
        riops: u64::MAX,
        wiops: u64::MAX,
        weight: 100,
    };
}

/// PIDs controller configuration
#[derive(Debug)]
pub struct PidsController {
    /// Maximum number of processes (0 = unlimited)
    pub max: u64,
    /// Current number of processes
    pub current: AtomicUsize,
}

impl PidsController {
    pub const UNLIMITED: Self = Self {
        max: u64::MAX,
        current: AtomicUsize::new(0),
    };

    pub fn can_fork(&self) -> bool {
        let current = self.current.load(Ordering::Acquire);
        current < self.max as usize
    }

    pub fn charge(&self) -> Result<(), CgroupError> {
        let new_count = self.current.fetch_add(1, Ordering::SeqCst) + 1;
        if new_count > self.max as usize {
            self.current.fetch_sub(1, Ordering::SeqCst);
            return Err(CgroupError::PidLimit);
        }
        Ok(())
    }

    pub fn uncharge(&self) {
        self.current.fetch_sub(1, Ordering::SeqCst);
    }
}

/// Cgroup statistics
#[derive(Debug, Clone, Copy)]
pub struct CgroupStats {
    /// CPU usage (nanoseconds)
    pub cpu_usage_ns: u64,
    /// Memory usage (bytes)
    pub memory_usage: u64,
    /// Number of processes
    pub num_processes: usize,
    /// I/O read bytes
    pub io_read_bytes: u64,
    /// I/O write bytes
    pub io_write_bytes: u64,
}

/// Cgroup node in unified hierarchy
pub struct Cgroup {
    /// Cgroup path (e.g., "/system.slice/myapp.service")
    pub path: String,
    /// Parent cgroup
    pub parent: Option<*mut Cgroup>,
    /// Child cgroups
    pub children: Vec<*mut Cgroup>,
    /// Process IDs in this cgroup
    pub pids: Vec<u32>,
    /// CPU controller
    pub cpu: CpuController,
    /// Memory controller
    pub memory: MemoryController,
    /// I/O controller
    pub io: IoController,
    /// PIDs controller
    pub pids_ctrl: PidsController,
    /// Enabled controllers
    pub enabled_controllers: u8, // Bitmap of ControllerType
}

impl Cgroup {
    pub fn new(path: String) -> Self {
        Self {
            path,
            parent: None,
            children: Vec::new(),
            pids: Vec::new(),
            cpu: CpuController::DEFAULT,
            memory: MemoryController::UNLIMITED,
            io: IoController::DEFAULT,
            pids_ctrl: PidsController::UNLIMITED,
            enabled_controllers: 0,
        }
    }

    /// Enable a controller
    pub fn enable_controller(&mut self, controller: ControllerType) {
        self.enabled_controllers |= 1 << (controller as u8);
    }

    /// Check if controller is enabled
    pub fn is_controller_enabled(&self, controller: ControllerType) -> bool {
        (self.enabled_controllers & (1 << (controller as u8))) != 0
    }

    /// Add process to cgroup
    pub fn add_process(&mut self, pid: u32) -> Result<(), CgroupError> {
        if self.is_controller_enabled(ControllerType::Pids) {
            self.pids_ctrl.charge()?;
        }
        self.pids.push(pid);
        Ok(())
    }

    /// Remove process from cgroup
    pub fn remove_process(&mut self, pid: u32) -> bool {
        if let Some(pos) = self.pids.iter().position(|&p| p == pid) {
            self.pids.remove(pos);
            if self.is_controller_enabled(ControllerType::Pids) {
                self.pids_ctrl.uncharge();
            }
            true
        } else {
            false
        }
    }

    /// Get cgroup statistics
    pub fn stats(&self) -> CgroupStats {
        CgroupStats {
            cpu_usage_ns: 0, // Would be tracked by CPU controller
            memory_usage: self.memory.usage(),
            num_processes: self.pids.len(),
            io_read_bytes: 0, // Would be tracked by I/O controller
            io_write_bytes: 0,
        }
    }
}

/// Cgroup hierarchy manager
pub struct CgroupManager {
    /// Root cgroup
    root: *mut Cgroup,
    /// All cgroups (indexed by path)
    cgroups: BTreeMap<String, *mut Cgroup>,
}

impl CgroupManager {
    pub fn new() -> Self {
        let root = Box::into_raw(Box::new(Cgroup::new(String::from("/"))));
        let mut cgroups = BTreeMap::new();
        cgroups.insert(String::from("/"), root);

        Self { root, cgroups }
    }

    /// Create new cgroup
    pub fn create_cgroup(
        &mut self,
        path: String,
        parent_path: &str,
    ) -> Result<*mut Cgroup, CgroupError> {
        if self.cgroups.contains_key(&path) {
            return Err(CgroupError::AlreadyExists);
        }

        let parent = self.cgroups.get(parent_path).ok_or(CgroupError::NotFound)?;
        let mut cgroup = Box::new(Cgroup::new(path.clone()));
        cgroup.parent = Some(*parent);

        let cgroup_ptr = Box::into_raw(cgroup);
        unsafe {
            (**parent).children.push(cgroup_ptr);
        }

        self.cgroups.insert(path, cgroup_ptr);
        Ok(cgroup_ptr)
    }

    /// Get cgroup by path
    pub fn get_cgroup(&self, path: &str) -> Option<&mut Cgroup> {
        self.cgroups.get(path).map(|&ptr| unsafe { &mut *ptr })
    }

    /// Move process to cgroup
    pub fn move_process(&mut self, pid: u32, from: &str, to: &str) -> Result<(), CgroupError> {
        // Remove from old cgroup
        if let Some(old_cgroup) = self.get_cgroup(from) {
            old_cgroup.remove_process(pid);
        }

        // Add to new cgroup
        if let Some(new_cgroup) = self.get_cgroup(to) {
            new_cgroup.add_process(pid)?;
            Ok(())
        } else {
            Err(CgroupError::NotFound)
        }
    }

    /// Remove cgroup
    pub fn remove_cgroup(&mut self, path: &str) -> Result<(), CgroupError> {
        if path == "/" {
            return Err(CgroupError::CannotRemoveRoot);
        }

        let cgroup_ptr = self.cgroups.remove(path).ok_or(CgroupError::NotFound)?;
        let cgroup = unsafe { &mut *cgroup_ptr };

        if !cgroup.children.is_empty() {
            return Err(CgroupError::HasChildren);
        }

        if !cgroup.pids.is_empty() {
            return Err(CgroupError::HasProcesses);
        }

        // Remove from parent's children list
        if let Some(parent_ptr) = cgroup.parent {
            unsafe {
                let parent = &mut *parent_ptr;
                parent.children.retain(|&p| p != cgroup_ptr);
            }
        }

        // Free memory
        unsafe {
            let _ = Box::from_raw(cgroup_ptr);
        }

        Ok(())
    }

    /// Get all cgroup paths
    pub fn list_cgroups(&self) -> Vec<String> {
        self.cgroups.keys().cloned().collect()
    }
}

/// Cgroup errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CgroupError {
    /// Cgroup already exists
    AlreadyExists,
    /// Cgroup not found
    NotFound,
    /// Cannot remove root cgroup
    CannotRemoveRoot,
    /// Cgroup has children
    HasChildren,
    /// Cgroup has processes
    HasProcesses,
    /// Memory limit exceeded
    MemoryLimit,
    /// PID limit exceeded
    PidLimit,
    /// I/O limit exceeded
    IoLimit,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_controller() {
        let mem = MemoryController {
            max: 1024,
            low: 0,
            high: 1024,
            current: AtomicU64::new(0),
            swap_max: 0,
        };

        assert!(mem.charge(512).is_ok());
        assert_eq!(mem.usage(), 512);
        assert!(mem.charge(600).is_err()); // Exceeds limit
        mem.uncharge(512);
        assert_eq!(mem.usage(), 0);
    }

    #[test]
    fn test_pids_controller() {
        let pids = PidsController {
            max: 10,
            current: AtomicUsize::new(0),
        };

        for _ in 0..10 {
            assert!(pids.charge().is_ok());
        }
        assert!(pids.charge().is_err()); // Exceeds limit
    }

    #[test]
    fn test_cgroup_creation() {
        let cgroup = Cgroup::new(String::from("/test"));
        assert_eq!(cgroup.path, "/test");
        assert_eq!(cgroup.pids.len(), 0);
    }

    #[test]
    fn test_cgroup_manager() {
        let mut manager = CgroupManager::new();
        manager
            .create_cgroup(String::from("/user.slice"), "/")
            .unwrap();

        assert!(manager.get_cgroup("/user.slice").is_some());
        assert_eq!(manager.list_cgroups().len(), 2);
    }

    #[test]
    fn test_cgroup_process_management() {
        let mut cgroup = Cgroup::new(String::from("/test"));
        cgroup.enable_controller(ControllerType::Pids);
        cgroup.pids_ctrl.max = 5;

        assert!(cgroup.add_process(100).is_ok());
        assert_eq!(cgroup.pids.len(), 1);
        assert!(cgroup.remove_process(100));
        assert_eq!(cgroup.pids.len(), 0);
    }
}
