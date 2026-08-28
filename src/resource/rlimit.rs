extern crate alloc;
// BSD-style Resource Limits (rlimits) for SigmaOS
// Implements process-specific soft and hard limits on system resources.


use alloc::collections::BTreeMap;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResourceLimitType {
    CpuTime = 0,       // CPU time in seconds
    FileSize = 1,      // Maximum file size in bytes
    DataSize = 2,      // Maximum data segment (heap) size in bytes
    StackSize = 3,     // Maximum stack size in bytes
    CoreSize = 4,      // Maximum core file size in bytes
    NoFile = 5,        // Maximum number of open files
    AddressSpace = 6,  // Maximum address space (virtual memory) in bytes
    MaxProcesses = 7,  // Maximum processes per user/session
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RLimitError {
    Success = 0,
    NotFound = 1,
    PermissionDenied = 2, // Hard limit cannot be raised by non-privileged
    InvalidLimit = 3,     // Soft limit cannot exceed hard limit
    Exceeded = 4,         // Requested amount exceeds limit
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RLimit {
    pub soft_limit: u64,
    pub hard_limit: u64,
}

impl RLimit {
    pub fn new(soft: u64, hard: u64) -> Self {
        RLimit {
            soft_limit: soft,
            hard_limit: hard,
        }
    }

    pub fn unlimited() -> Self {
        RLimit {
            soft_limit: u64::MAX,
            hard_limit: u64::MAX,
        }
    }
}

pub struct ProcessResourceLimiter {
    // Maps PID to a map of its resource limits
    pub process_limits: BTreeMap<u64, BTreeMap<ResourceLimitType, RLimit>>,
    // Mock current usage mapping (PID -> ResourceType -> current usage value)
    pub current_usage: BTreeMap<u64, BTreeMap<ResourceLimitType, u64>>,
}

impl ProcessResourceLimiter {
    pub fn new() -> Self {
        ProcessResourceLimiter {
            process_limits: BTreeMap::new(),
            current_usage: BTreeMap::new(),
        }
    }

    /// Register default limits for a new process PID
    pub fn register_process(&mut self, pid: u64) {
        let mut limits = BTreeMap::new();
        limits.insert(ResourceLimitType::CpuTime, RLimit::unlimited());
        limits.insert(ResourceLimitType::FileSize, RLimit::new(10 * 1024 * 1024, 100 * 1024 * 1024)); // 10MB / 100MB
        limits.insert(ResourceLimitType::DataSize, RLimit::new(64 * 1024 * 1024, 256 * 1024 * 1024)); // 64MB / 256MB
        limits.insert(ResourceLimitType::StackSize, RLimit::new(8 * 1024 * 1024, 32 * 1024 * 1024)); // 8MB / 32MB
        limits.insert(ResourceLimitType::CoreSize, RLimit::new(0, 1024 * 1024)); // 0 / 1MB
        limits.insert(ResourceLimitType::NoFile, RLimit::new(256, 1024)); // 256 / 1024 FDs
        limits.insert(ResourceLimitType::AddressSpace, RLimit::unlimited());
        limits.insert(ResourceLimitType::MaxProcesses, RLimit::new(10, 50));

        self.process_limits.insert(pid, limits);
        self.current_usage.insert(pid, BTreeMap::new());
    }

    /// Retrieve the limit of a process for a given resource type
    pub fn get_limit(&self, pid: u64, limit_type: ResourceLimitType) -> Option<&RLimit> {
        self.process_limits.get(&pid)?.get(&limit_type)
    }

    /// Adjust limit for a process. Ensures soft limit does not exceed hard limit,
    /// and that raising hard limits requires privilege (checked via parameter).
    pub fn set_limit(
        &mut self,
        pid: u64,
        limit_type: ResourceLimitType,
        soft: u64,
        hard: u64,
        is_privileged: bool,
    ) -> Result<(), RLimitError> {
        if soft > hard {
            return Err(RLimitError::InvalidLimit);
        }

        let current_limits = self.process_limits.get(&pid).ok_or(RLimitError::NotFound)?;
        if let Some(existing) = current_limits.get(&limit_type) {
            // Unprivileged process cannot raise the hard limit
            if hard > existing.hard_limit && !is_privileged {
                return Err(RLimitError::PermissionDenied);
            }
        }

        if let Some(limits) = self.process_limits.get_mut(&pid) {
            limits.insert(limit_type, RLimit::new(soft, hard));
            Ok(())
        } else {
            Err(RLimitError::NotFound)
        }
    }

    /// Check if requesting an addition of `amount` to `limit_type` usage for `pid` violates the soft or hard limits.
    pub fn check_and_add_usage(
        &mut self,
        pid: u64,
        limit_type: ResourceLimitType,
        amount: u64,
    ) -> Result<(), RLimitError> {
        let limit = *self.process_limits.get(&pid).ok_or(RLimitError::NotFound)?.get(&limit_type).ok_or(RLimitError::NotFound)?;
        let usage_map = self.current_usage.get_mut(&pid).ok_or(RLimitError::NotFound)?;
        let current = usage_map.get(&limit_type).cloned().unwrap_or(0);

        let new_usage = current.saturating_add(amount);
        if new_usage > limit.soft_limit {
            return Err(RLimitError::Exceeded);
        }

        usage_map.insert(limit_type, new_usage);
        Ok(())
    }

    /// Subtract/release usage for `pid`
    pub fn release_usage(&mut self, pid: u64, limit_type: ResourceLimitType, amount: u64) -> Result<(), RLimitError> {
        let usage_map = self.current_usage.get_mut(&pid).ok_or(RLimitError::NotFound)?;
        let current = usage_map.get(&limit_type).cloned().unwrap_or(0);
        let new_usage = current.saturating_sub(amount);
        usage_map.insert(limit_type, new_usage);
        Ok(())
    }

    /// Get current usage
    pub fn get_usage(&self, pid: u64, limit_type: ResourceLimitType) -> u64 {
        self.current_usage
            .get(&pid)
            .and_then(|m| m.get(&limit_type))
            .cloned()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rlimit_registration_and_retrieval() {
        let mut limiter = ProcessResourceLimiter::new();
        limiter.register_process(42);

        let limit = limiter.get_limit(42, ResourceLimitType::FileSize).unwrap();
        assert_eq!(limit.soft_limit, 10 * 1024 * 1024);
        assert_eq!(limit.hard_limit, 100 * 1024 * 1024);

        let non_existent = limiter.get_limit(99, ResourceLimitType::FileSize);
        assert!(non_existent.is_none());
    }

    #[test]
    fn test_rlimit_setting_validations() {
        let mut limiter = ProcessResourceLimiter::new();
        limiter.register_process(42);

        // Setting soft > hard should fail
        assert_eq!(
            limiter.set_limit(42, ResourceLimitType::NoFile, 2000, 1000, false),
            Err(RLimitError::InvalidLimit)
        );

        // Unprivileged attempt to raise hard limit should fail
        assert_eq!(
            limiter.set_limit(42, ResourceLimitType::NoFile, 500, 2000, false),
            Err(RLimitError::PermissionDenied)
        );

        // Privileged attempt to raise hard limit should succeed
        assert_eq!(
            limiter.set_limit(42, ResourceLimitType::NoFile, 500, 2000, true),
            Ok(())
        );

        let limit = limiter.get_limit(42, ResourceLimitType::NoFile).unwrap();
        assert_eq!(limit.soft_limit, 500);
        assert_eq!(limit.hard_limit, 2000);
    }

    #[test]
    fn test_rlimit_enforcement_and_tracking() {
        let mut limiter = ProcessResourceLimiter::new();
        limiter.register_process(42);

        // Set max files limit to 3 / 5
        assert_eq!(limiter.set_limit(42, ResourceLimitType::NoFile, 3, 5, false), Ok(()));

        // Add 2 file descriptors - should succeed
        assert_eq!(limiter.check_and_add_usage(42, ResourceLimitType::NoFile, 2), Ok(()));
        assert_eq!(limiter.get_usage(42, ResourceLimitType::NoFile), 2);

        // Adding 2 more would be 4, which exceeds soft limit (3) - should fail
        assert_eq!(
            limiter.check_and_add_usage(42, ResourceLimitType::NoFile, 2),
            Err(RLimitError::Exceeded)
        );
        assert_eq!(limiter.get_usage(42, ResourceLimitType::NoFile), 2); // usage unchanged

        // Release 1 file descriptor
        assert_eq!(limiter.release_usage(42, ResourceLimitType::NoFile, 1), Ok(()));
        assert_eq!(limiter.get_usage(42, ResourceLimitType::NoFile), 1);

        // Now adding 2 more works because 1 + 2 = 3 <= soft limit (3)
        assert_eq!(limiter.check_and_add_usage(42, ResourceLimitType::NoFile, 2), Ok(()));
        assert_eq!(limiter.get_usage(42, ResourceLimitType::NoFile), 3);
    }
}
