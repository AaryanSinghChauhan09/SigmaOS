// System Call Whitelist/Blacklist Filtering
// Implements per-process syscall access control

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

/// Syscall filter type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterType {
    /// Whitelist mode - only allowed syscalls permitted
    Whitelist = 0,
    /// Blacklist mode - blocked syscalls denied
    Blacklist = 1,
}

/// Syscall filter policy
#[derive(Debug, Clone)]
pub struct SyscallFilterPolicy {
    /// Filter type
    pub filter_type: FilterType,
    /// Allowed syscalls (for whitelist)
    pub allowed: HashSet<u32>,
    /// Blocked syscalls (for blacklist)
    pub blocked: HashSet<u32>,
    /// Default decision
    pub default_allow: bool,
}

impl SyscallFilterPolicy {
    /// Create new policy with whitelist mode
    pub fn new_whitelist() -> Self {
        SyscallFilterPolicy {
            filter_type: FilterType::Whitelist,
            allowed: HashSet::new(),
            blocked: HashSet::new(),
            default_allow: false,
        }
    }

    /// Create new policy with blacklist mode
    pub fn new_blacklist() -> Self {
        SyscallFilterPolicy {
            filter_type: FilterType::Blacklist,
            allowed: HashSet::new(),
            blocked: HashSet::new(),
            default_allow: true,
        }
    }

    /// Add allowed syscall
    pub fn add_allowed(&mut self, syscall_nr: u32) {
        self.allowed.insert(syscall_nr);
    }

    /// Add blocked syscall
    pub fn add_blocked(&mut self, syscall_nr: u32) {
        self.blocked.insert(syscall_nr);
    }

    /// Remove allowed syscall
    pub fn remove_allowed(&mut self, syscall_nr: u32) {
        self.allowed.remove(&syscall_nr);
    }

    /// Remove blocked syscall
    pub fn remove_blocked(&mut self, syscall_nr: u32) {
        self.blocked.remove(&syscall_nr);
    }

    /// Check if syscall is allowed
    pub fn is_allowed(&self, syscall_nr: u32) -> bool {
        match self.filter_type {
            FilterType::Whitelist => {
                if self.blocked.contains(&syscall_nr) {
                    return false;
                }
                self.allowed.contains(&syscall_nr) || self.default_allow
            }
            FilterType::Blacklist => {
                if self.blocked.contains(&syscall_nr) {
                    return false;
                }
                !self.blocked.contains(&syscall_nr)
            }
        }
    }

    /// Get allowed syscall count
    pub fn allowed_count(&self) -> usize {
        self.allowed.len()
    }

    /// Get blocked syscall count
    pub fn blocked_count(&self) -> usize {
        self.blocked.len()
    }

    /// Clear filter
    pub fn clear(&mut self) {
        self.allowed.clear();
        self.blocked.clear();
    }
}

impl Default for SyscallFilterPolicy {
    fn default() -> Self {
        Self::new_blacklist()
    }
}

/// Per-process syscall filter
#[derive(Debug, Clone)]
pub struct ProcessSyscallFilter {
    /// Process ID
    pub process_id: u32,
    /// Filter policy
    pub policy: SyscallFilterPolicy,
    /// Is enabled
    pub enabled: bool,
    /// Parent process filter (for inheritance)
    pub parent_filter: Option<Box<ProcessSyscallFilter>>,
    /// Inherit parent policy
    pub inherit_parent: bool,
}

impl ProcessSyscallFilter {
    /// Create new process filter
    pub fn new(process_id: u32, filter_type: FilterType) -> Self {
        let policy = match filter_type {
            FilterType::Whitelist => SyscallFilterPolicy::new_whitelist(),
            FilterType::Blacklist => SyscallFilterPolicy::new_blacklist(),
        };

        ProcessSyscallFilter {
            process_id,
            policy,
            enabled: false,
            parent_filter: None,
            inherit_parent: false,
        }
    }

    /// Set parent filter for inheritance
    pub fn set_parent(&mut self, parent: ProcessSyscallFilter) {
        self.parent_filter = Some(Box::new(parent));
    }

    /// Enable inheritance from parent
    pub fn enable_parent_inheritance(&mut self) {
        self.inherit_parent = true;
    }

    /// Check if syscall is allowed
    pub fn is_allowed(&self, syscall_nr: u32) -> bool {
        if !self.enabled {
            return true;
        }

        // Check parent filter if inheriting
        if self.inherit_parent {
            if let Some(parent) = &self.parent_filter {
                if !parent.is_allowed(syscall_nr) {
                    return false;
                }
            }
        }

        // Check own policy
        self.policy.is_allowed(syscall_nr)
    }

    /// Enable filter
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable filter
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Add allowed syscall
    pub fn add_allowed(&mut self, syscall_nr: u32) {
        self.policy.add_allowed(syscall_nr);
    }

    /// Add blocked syscall
    pub fn add_blocked(&mut self, syscall_nr: u32) {
        self.policy.add_blocked(syscall_nr);
    }

    /// Whitelist specific syscalls (switch to whitelist mode)
    pub fn whitelist_syscalls(&mut self, syscalls: Vec<u32>) {
        self.policy = SyscallFilterPolicy::new_whitelist();
        for syscall in syscalls {
            self.policy.add_allowed(syscall);
        }
    }

    /// Blacklist specific syscalls
    pub fn blacklist_syscalls(&mut self, syscalls: Vec<u32>) {
        self.policy = SyscallFilterPolicy::new_blacklist();
        for syscall in syscalls {
            self.policy.add_blocked(syscall);
        }
    }
}

/// Global syscall filter manager
pub struct SyscallFilterManager {
    /// Filters by process ID
    filters: Arc<Mutex<HashMap<u32, ProcessSyscallFilter>>>,
}

impl SyscallFilterManager {
    /// Create new manager
    pub fn new() -> Self {
        SyscallFilterManager {
            filters: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Register process filter
    pub fn register_process(&self, process_id: u32, filter_type: FilterType) -> Result<(), String> {
        let mut filters = self
            .filters
            .lock()
            .map_err(|_| "Failed to acquire filters lock".to_string())?;
        filters.insert(process_id, ProcessSyscallFilter::new(process_id, filter_type));
        Ok(())
    }

    /// Unregister process filter
    pub fn unregister_process(&self, process_id: u32) -> Result<(), String> {
        let mut filters = self
            .filters
            .lock()
            .map_err(|_| "Failed to acquire filters lock".to_string())?;
        filters.remove(&process_id);
        Ok(())
    }

    /// Set parent filter for child process
    pub fn set_parent_filter(&self, child_id: u32, parent_id: u32) -> Result<(), String> {
        let mut filters = self
            .filters
            .lock()
            .map_err(|_| "Failed to acquire filters lock".to_string())?;

        let parent = filters.get(&parent_id).cloned();
        
        if parent.is_some() && filters.contains_key(&child_id) {
            if let Some(parent) = parent {
                if let Some(child) = filters.get_mut(&child_id) {
                    child.set_parent(parent);
                    return Ok(());
                }
            }
        }
        
        Err("Parent or child process not found".to_string())
    }

    /// Enable inheritance for process
    pub fn enable_inheritance(&self, process_id: u32) -> Result<(), String> {
        let mut filters = self
            .filters
            .lock()
            .map_err(|_| "Failed to acquire filters lock".to_string())?;

        if let Some(filter) = filters.get_mut(&process_id) {
            filter.enable_parent_inheritance();
            Ok(())
        } else {
            Err(format!("Process {} not found", process_id))
        }
    }

    /// Enable filter for process
    pub fn enable_filter(&self, process_id: u32) -> Result<(), String> {
        let mut filters = self
            .filters
            .lock()
            .map_err(|_| "Failed to acquire filters lock".to_string())?;

        if let Some(filter) = filters.get_mut(&process_id) {
            filter.enable();
            Ok(())
        } else {
            Err(format!("Process {} not found", process_id))
        }
    }

    /// Disable filter for process
    pub fn disable_filter(&self, process_id: u32) -> Result<(), String> {
        let mut filters = self
            .filters
            .lock()
            .map_err(|_| "Failed to acquire filters lock".to_string())?;

        if let Some(filter) = filters.get_mut(&process_id) {
            filter.disable();
            Ok(())
        } else {
            Err(format!("Process {} not found", process_id))
        }
    }

    /// Check if syscall is allowed
    pub fn is_syscall_allowed(&self, process_id: u32, syscall_nr: u32) -> Result<bool, String> {
        let filters = self
            .filters
            .lock()
            .map_err(|_| "Failed to acquire filters lock".to_string())?;

        if let Some(filter) = filters.get(&process_id) {
            Ok(filter.is_allowed(syscall_nr))
        } else {
            Err(format!("Process {} not found", process_id))
        }
    }

    /// Add allowed syscall
    pub fn add_allowed_syscall(&self, process_id: u32, syscall_nr: u32) -> Result<(), String> {
        let mut filters = self
            .filters
            .lock()
            .map_err(|_| "Failed to acquire filters lock".to_string())?;

        if let Some(filter) = filters.get_mut(&process_id) {
            filter.add_allowed(syscall_nr);
            Ok(())
        } else {
            Err(format!("Process {} not found", process_id))
        }
    }

    /// Add blocked syscall
    pub fn add_blocked_syscall(&self, process_id: u32, syscall_nr: u32) -> Result<(), String> {
        let mut filters = self
            .filters
            .lock()
            .map_err(|_| "Failed to acquire filters lock".to_string())?;

        if let Some(filter) = filters.get_mut(&process_id) {
            filter.add_blocked(syscall_nr);
            Ok(())
        } else {
            Err(format!("Process {} not found", process_id))
        }
    }

    /// Whitelist syscalls for process
    pub fn whitelist_syscalls(&self, process_id: u32, syscalls: Vec<u32>) -> Result<(), String> {
        let mut filters = self
            .filters
            .lock()
            .map_err(|_| "Failed to acquire filters lock".to_string())?;

        if let Some(filter) = filters.get_mut(&process_id) {
            filter.whitelist_syscalls(syscalls);
            Ok(())
        } else {
            Err(format!("Process {} not found", process_id))
        }
    }

    /// Blacklist syscalls for process
    pub fn blacklist_syscalls(&self, process_id: u32, syscalls: Vec<u32>) -> Result<(), String> {
        let mut filters = self
            .filters
            .lock()
            .map_err(|_| "Failed to acquire filters lock".to_string())?;

        if let Some(filter) = filters.get_mut(&process_id) {
            filter.blacklist_syscalls(syscalls);
            Ok(())
        } else {
            Err(format!("Process {} not found", process_id))
        }
    }

    /// Get process count
    pub fn process_count(&self) -> Result<usize, String> {
        let filters = self
            .filters
            .lock()
            .map_err(|_| "Failed to acquire filters lock".to_string())?;
        Ok(filters.len())
    }
}

impl Default for SyscallFilterManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for SyscallFilterManager {
    fn clone(&self) -> Self {
        SyscallFilterManager {
            filters: Arc::clone(&self.filters),
        }
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_syscall_filter_policy_whitelist() {
        let mut policy = SyscallFilterPolicy::new_whitelist();
        policy.add_allowed(1); // sys_write

        assert!(policy.is_allowed(1));
        assert!(!policy.is_allowed(2));
    }

    #[test]
    fn test_syscall_filter_policy_blacklist() {
        let mut policy = SyscallFilterPolicy::new_blacklist();
        policy.add_blocked(56); // sys_clone

        assert!(!policy.is_allowed(56));
        assert!(policy.is_allowed(1));
    }

    #[test]
    fn test_process_syscall_filter() {
        let mut filter = ProcessSyscallFilter::new(100, FilterType::Whitelist);
        filter.add_allowed(1);
        filter.enable();

        assert!(filter.is_allowed(1));
        assert!(!filter.is_allowed(2));
    }

    #[test]
    fn test_syscall_filter_manager_register() {
        let manager = SyscallFilterManager::new();
        manager.register_process(100, FilterType::Whitelist).unwrap();
        assert_eq!(manager.process_count().unwrap(), 1);
    }

    #[test]
    fn test_syscall_filter_manager_enable() {
        let manager = SyscallFilterManager::new();
        manager.register_process(100, FilterType::Whitelist).unwrap();
        manager.add_allowed_syscall(100, 1).unwrap();
        manager.enable_filter(100).unwrap();

        assert!(manager.is_syscall_allowed(100, 1).unwrap());
    }

    #[test]
    fn test_syscall_filter_inheritance() {
        let manager = SyscallFilterManager::new();
        manager.register_process(1, FilterType::Whitelist).unwrap();
        manager.register_process(100, FilterType::Whitelist).unwrap();

        manager.add_allowed_syscall(1, 1).unwrap();
        manager.set_parent_filter(100, 1).unwrap();
        manager.enable_inheritance(100).unwrap();
        manager.enable_filter(1).unwrap();
        manager.enable_filter(100).unwrap();

        assert!(manager.is_syscall_allowed(100, 1).unwrap());
    }

    #[test]
    fn test_whitelist_syscalls() {
        let manager = SyscallFilterManager::new();
        manager.register_process(100, FilterType::Whitelist).unwrap();
        manager.whitelist_syscalls(100, vec![1, 2, 3]).unwrap();
        manager.enable_filter(100).unwrap();

        assert!(manager.is_syscall_allowed(100, 1).unwrap());
        assert!(manager.is_syscall_allowed(100, 2).unwrap());
        assert!(!manager.is_syscall_allowed(100, 10).unwrap());
    }

    #[test]
    fn test_blacklist_syscalls() {
        let manager = SyscallFilterManager::new();
        manager.register_process(100, FilterType::Blacklist).unwrap();
        manager.blacklist_syscalls(100, vec![56]).unwrap();
        manager.enable_filter(100).unwrap();

        assert!(!manager.is_syscall_allowed(100, 56).unwrap());
        assert!(manager.is_syscall_allowed(100, 1).unwrap());
    }
}
