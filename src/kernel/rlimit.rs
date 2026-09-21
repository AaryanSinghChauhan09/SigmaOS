// Linux-inspired resource limits for process control
// Provides resource limit management (RLIMIT)

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Resource types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RlimitResource {
    Cpu = 0,           // CPU time in seconds
    Fsize = 1,         // Maximum file size
    Data = 2,          // Maximum data size
    Stack = 3,         // Maximum stack size
    Core = 4,          // Core file size
    Nproc = 6,         // Number of processes
    Nofile = 7,        // Number of open files
    Memlock = 8,       // Locked memory size
    As = 9,            // Address space limit
    Locks = 10,        // Number of file locks
    Sigpending = 11,   // Pending signals
    Msgqueue = 12,     // Bytes in message queues
    Nice = 13,         // Max nice priority
    Rtprio = 14,       // Real-time priority
    Rttime = 15,       // Real-time timeout
}

/// Resource limit value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rlimit {
    pub cur: u64,  // Current limit
    pub max: u64,  // Maximum limit (hard limit)
}

impl Rlimit {
    pub fn new(cur: u64, max: u64) -> Self {
        Self { cur, max }
    }

    /// Check if value is within limit
    pub fn check(&self, value: u64) -> bool {
        value <= self.cur
    }

    /// Set current limit
    pub fn set_cur(&mut self, cur: u64) -> Result<(), String> {
        if cur > self.max {
            return Err("Current limit cannot exceed hard limit".to_string());
        }
        self.cur = cur;
        Ok(())
    }

    /// Set hard limit
    pub fn set_max(&mut self, max: u64) {
        self.max = max;
        if self.cur > max {
            self.cur = max;
        }
    }
}

/// Resource limits for a process
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub limits: HashMap<RlimitResource, Rlimit>,
}

impl ResourceLimits {
    pub fn new() -> Self {
        let mut limits = HashMap::new();

        // Default limits
        limits.insert(RlimitResource::Cpu, Rlimit::new(u64::MAX, u64::MAX));
        limits.insert(RlimitResource::Fsize, Rlimit::new(u64::MAX, u64::MAX));
        limits.insert(RlimitResource::Data, Rlimit::new(u64::MAX, u64::MAX));
        limits.insert(RlimitResource::Stack, Rlimit::new(8 * 1024 * 1024, u64::MAX));
        limits.insert(RlimitResource::Core, Rlimit::new(0, u64::MAX));
        limits.insert(RlimitResource::Nproc, Rlimit::new(u64::MAX, u64::MAX));
        limits.insert(RlimitResource::Nofile, Rlimit::new(1024, 4096));
        limits.insert(RlimitResource::Memlock, Rlimit::new(64 * 1024 * 1024, 64 * 1024 * 1024));
        limits.insert(RlimitResource::As, Rlimit::new(u64::MAX, u64::MAX));
        limits.insert(RlimitResource::Locks, Rlimit::new(u64::MAX, u64::MAX));
        limits.insert(RlimitResource::Sigpending, Rlimit::new(u64::MAX, u64::MAX));
        limits.insert(RlimitResource::Msgqueue, Rlimit::new(8 * 1024 * 1024, u64::MAX));
        limits.insert(RlimitResource::Nice, Rlimit::new(0, 0));
        limits.insert(RlimitResource::Rtprio, Rlimit::new(0, 0));
        limits.insert(RlimitResource::Rttime, Rlimit::new(u64::MAX, u64::MAX));

        Self { limits }
    }

    /// Get resource limit
    pub fn get(&self, resource: RlimitResource) -> Option<Rlimit> {
        self.limits.get(&resource).copied()
    }

    /// Set resource limit
    pub fn set(&mut self, resource: RlimitResource, rlimit: Rlimit) -> Result<(), String> {
        if rlimit.cur > rlimit.max {
            return Err("Current limit cannot exceed hard limit".to_string());
        }
        self.limits.insert(resource, rlimit);
        Ok(())
    }

    /// Check if resource usage is within limit
    pub fn check(&self, resource: RlimitResource, value: u64) -> bool {
        match self.get(resource) {
            Some(rlimit) => rlimit.check(value),
            None => true, // No limit set
        }
    }

    /// Get current limit
    pub fn get_cur(&self, resource: RlimitResource) -> Option<u64> {
        self.get(resource).map(|r| r.cur)
    }

    /// Get hard limit
    pub fn get_max(&self, resource: RlimitResource) -> Option<u64> {
        self.get(resource).map(|r| r.max)
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self::new()
    }
}

/// Resource limits manager for system-wide resource limit management
pub struct ResourceLimitsManager {
    limits: Arc<Mutex<HashMap<u64, ResourceLimits>>>,
    next_pid: Arc<Mutex<u64>>,
}

impl ResourceLimitsManager {
    pub fn new() -> Self {
        Self {
            limits: Arc::new(Mutex::new(HashMap::new())),
            next_pid: Arc::new(Mutex::new(1)),
        }
    }

    /// Create resource limits for a process
    pub fn create_limits(&self) -> u64 {
        let mut next_pid = self.next_pid.lock().unwrap();
        let pid = *next_pid;
        *next_pid += 1;
        drop(next_pid);

        let limits = ResourceLimits::new();
        let mut limits_map = self.limits.lock().unwrap();
        limits_map.insert(pid, limits);

        pid
    }

    /// Get resource limits for a process
    pub fn get_limits(&self, pid: u64) -> Option<ResourceLimits> {
        let limits_map = self.limits.lock().unwrap();
        limits_map.get(&pid).cloned()
    }

    /// Remove resource limits
    pub fn remove_limits(&self, pid: u64) -> Result<(), String> {
        let mut limits_map = self.limits.lock().unwrap();
        match limits_map.remove(&pid) {
            Some(_) => Ok(()),
            None => Err(format!("Resource limits for pid {} not found", pid)),
        }
    }

    /// Get resource limit
    pub fn get(&self, pid: u64, resource: RlimitResource) -> Option<Rlimit> {
        let limits_map = self.limits.lock().unwrap();
        limits_map.get(&pid).and_then(|l| l.get(resource))
    }

    /// Set resource limit
    pub fn set(&self, pid: u64, resource: RlimitResource, rlimit: Rlimit) -> Result<(), String> {
        let mut limits_map = self.limits.lock().unwrap();
        match limits_map.get_mut(&pid) {
            Some(limits) => limits.set(resource, rlimit),
            None => Err(format!("Resource limits for pid {} not found", pid)),
        }
    }

    /// Check resource usage
    pub fn check(&self, pid: u64, resource: RlimitResource, value: u64) -> bool {
        let limits_map = self.limits.lock().unwrap();
        limits_map.get(&pid).map_or(true, |l| l.check(resource, value))
    }

    /// Get limits count
    pub fn limits_count(&self) -> usize {
        let limits_map = self.limits.lock().unwrap();
        limits_map.len()
    }
}

impl Default for ResourceLimitsManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rlimit() {
        let rlimit = Rlimit::new(1024, 4096);
        assert_eq!(rlimit.cur, 1024);
        assert_eq!(rlimit.max, 4096);
    }

    #[test]
    fn test_rlimit_check() {
        let rlimit = Rlimit::new(1024, 4096);
        assert!(rlimit.check(512));
        assert!(!rlimit.check(2048));
    }

    #[test]
    fn test_rlimit_set_cur() {
        let mut rlimit = Rlimit::new(1024, 4096);
        rlimit.set_cur(2048).unwrap();
        assert_eq!(rlimit.cur, 2048);
    }

    #[test]
    fn test_rlimit_set_cur_exceeds_max() {
        let mut rlimit = Rlimit::new(1024, 4096);
        assert!(rlimit.set_cur(8192).is_err());
    }

    #[test]
    fn test_rlimit_set_max() {
        let mut rlimit = Rlimit::new(1024, 4096);
        rlimit.set_max(2048);
        assert_eq!(rlimit.max, 2048);
        assert_eq!(rlimit.cur, 1024); // cur unchanged
    }

    #[test]
    fn test_rlimit_set_max_adjusts_cur() {
        let mut rlimit = Rlimit::new(4096, 8192);
        rlimit.set_max(2048);
        assert_eq!(rlimit.max, 2048);
        assert_eq!(rlimit.cur, 2048); // cur adjusted
    }

    #[test]
    fn test_resource_limits() {
        let limits = ResourceLimits::new();
        assert!(limits.get(RlimitResource::Nofile).is_some());
    }

    #[test]
    fn test_resource_limits_get() {
        let limits = ResourceLimits::new();
        let rlimit = limits.get(RlimitResource::Nofile).unwrap();
        assert_eq!(rlimit.cur, 1024);
        assert_eq!(rlimit.max, 4096);
    }

    #[test]
    fn test_resource_limits_set() {
        let mut limits = ResourceLimits::new();
        let rlimit = Rlimit::new(2048, 8192);
        limits.set(RlimitResource::Nofile, rlimit).unwrap();

        let retrieved = limits.get(RlimitResource::Nofile).unwrap();
        assert_eq!(retrieved.cur, 2048);
    }

    #[test]
    fn test_resource_limits_set_invalid() {
        let mut limits = ResourceLimits::new();
        let rlimit = Rlimit::new(8192, 4096); // cur > max
        assert!(limits.set(RlimitResource::Nofile, rlimit).is_err());
    }

    #[test]
    fn test_resource_limits_check() {
        let limits = ResourceLimits::new();
        assert!(limits.check(RlimitResource::Nofile, 512));
        assert!(!limits.check(RlimitResource::Nofile, 2048));
    }

    #[test]
    fn test_resource_limits_manager() {
        let manager = ResourceLimitsManager::new();

        let pid = manager.create_limits();
        assert_eq!(pid, 1);
        assert_eq!(manager.limits_count(), 1);
    }

    #[test]
    fn test_resource_limits_manager_get() {
        let manager = ResourceLimitsManager::new();

        let pid = manager.create_limits();
        let rlimit = manager.get(pid, RlimitResource::Nofile).unwrap();

        assert_eq!(rlimit.cur, 1024);
    }

    #[test]
    fn test_resource_limits_manager_set() {
        let manager = ResourceLimitsManager::new();

        let pid = manager.create_limits();
        let rlimit = Rlimit::new(2048, 8192);
        manager.set(pid, RlimitResource::Nofile, rlimit).unwrap();

        let retrieved = manager.get(pid, RlimitResource::Nofile).unwrap();
        assert_eq!(retrieved.cur, 2048);
    }

    #[test]
    fn test_resource_limits_manager_check() {
        let manager = ResourceLimitsManager::new();

        let pid = manager.create_limits();
        assert!(manager.check(pid, RlimitResource::Nofile, 512));
        assert!(!manager.check(pid, RlimitResource::Nofile, 2048));
    }

    #[test]
    fn test_resource_limits_manager_remove() {
        let manager = ResourceLimitsManager::new();

        let pid = manager.create_limits();
        manager.remove_limits(pid).unwrap();

        assert_eq!(manager.limits_count(), 0);
    }

    #[test]
    fn test_resource_limits_manager_invalid() {
        let manager = ResourceLimitsManager::new();
        assert!(manager.get(999, RlimitResource::Nofile).is_none());
        assert!(manager.set(999, RlimitResource::Nofile, Rlimit::new(1024, 4096)).is_err());
    }
}
