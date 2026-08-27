// Linux-style Control Groups (cgroups) for SigmaOS
// Implements hierarchical grouping of processes and resource limitation, prioritization, accounting, and control.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use crate::klib::BTreeMap;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CgroupError {
    Success = 0,
    NotFound = 1,
    LimitExceeded = 2,
    AlreadyExists = 3,
    ParentNotFound = 4,
    InvalidWeight = 5,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CgroupLimits {
    pub cpu_weight: u32,
    pub memory_max: u64,
    pub pids_max: u32,
}

impl CgroupLimits {
    pub fn new() -> Self {
        Self {
            cpu_weight: 100,
            memory_max: u64::MAX,
            pids_max: u32::MAX,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CgroupUsage {
    pub cpu_usage_ms: u64,
    pub memory_usage_bytes: u64,
    pub pids_count: u32,
}

impl CgroupUsage {
    pub fn new() -> Self {
        Self {
            cpu_usage_ms: 0,
            memory_usage_bytes: 0,
            pids_count: 0,
        }
    }
}

pub struct Cgroup {
    pub name: String,
    pub parent_name: Option<String>,
    pub limits: CgroupLimits,
    pub usage: CgroupUsage,
    pub pids: Vec<u64>,
}

impl Cgroup {
    pub fn new(name: &str, parent_name: Option<String>) -> Self {
        Self {
            name: String::from(name),
            parent_name,
            limits: CgroupLimits::new(),
            usage: CgroupUsage::new(),
            pids: Vec::new(),
        }
    }
}

pub struct CgroupManager {
    pub cgroups: BTreeMap<String, Cgroup>,
}

impl CgroupManager {
    pub fn new() -> Self {
        let mut manager = Self {
            cgroups: BTreeMap::new(),
        };
        // Create root cgroup
        manager.cgroups.insert(String::from("/"), Cgroup::new("/", None));
        manager
    }

    /// Create a child control group under a parent
    pub fn create_cgroup(&mut self, name: &str, parent: Option<&str>) -> Result<(), CgroupError> {
        if self.cgroups.contains_key(name) {
            return Err(CgroupError::AlreadyExists);
        }

        let parent_name = match parent {
            Some(p) => {
                if !self.cgroups.contains_key(p) {
                    return Err(CgroupError::ParentNotFound);
                }
                Some(String::from(p))
            }
            None => Some(String::from("/")),
        };

        let cgroup = Cgroup::new(name, parent_name);
        self.cgroups.insert(String::from(name), cgroup);
        Ok(())
    }

    /// Delete a control group
    pub fn delete_cgroup(&mut self, name: &str) -> Result<(), CgroupError> {
        if name == "/" {
            return Err(CgroupError::LimitExceeded); // Root cannot be deleted
        }

        if !self.cgroups.contains_key(name) {
            return Err(CgroupError::NotFound);
        }

        // Migrate PIDs to parent or root
        let parent = self.cgroups.get(name).unwrap().parent_name.clone().unwrap_or(String::from("/"));
        let pids_to_migrate = self.cgroups.get(name).unwrap().pids.clone();

        for pid in pids_to_migrate {
            let _ = self.attach_pid(&parent, pid);
        }

        self.cgroups.remove(name);
        Ok(())
    }

    /// Associate a process PID with a control group (migrates it from old group)
    pub fn attach_pid(&mut self, name: &str, pid: u64) -> Result<(), CgroupError> {
        // Verify target cgroup exists
        if !self.cgroups.contains_key(name) {
            return Err(CgroupError::NotFound);
        }

        // Check task limit/process count limit in target cgroup
        {
            let target_group = self.cgroups.get(name).unwrap();
            if target_group.usage.pids_count >= target_group.limits.pids_max {
                return Err(CgroupError::LimitExceeded);
            }
        }

        // Remove from current cgroup if associated with any
        let mut current_group_name = None;
        for (gname, group) in &self.cgroups {
            if group.pids.contains(&pid) {
                current_group_name = Some(gname.clone());
                break;
            }
        }

        if let Some(ref old_name) = current_group_name {
            if let Some(group) = self.cgroups.get_mut(old_name) {
                group.pids.retain(|&p| p != pid);
                group.usage.pids_count = group.usage.pids_count.saturating_sub(1);
            }
        }

        // Insert into new cgroup
        if let Some(group) = self.cgroups.get_mut(name) {
            group.pids.push(pid);
            group.usage.pids_count += 1;
        }

        Ok(())
    }

    /// Configure CPU weight (scheduling preference, v2 style)
    pub fn set_cpu_weight(&mut self, name: &str, weight: u32) -> Result<(), CgroupError> {
        if weight < 1 || weight > 10000 {
            return Err(CgroupError::InvalidWeight);
        }

        if let Some(group) = self.cgroups.get_mut(name) {
            group.limits.cpu_weight = weight;
            Ok(())
        } else {
            Err(CgroupError::NotFound)
        }
    }

    /// Configure memory maximum ceiling in bytes
    pub fn set_memory_max(&mut self, name: &str, max_bytes: u64) -> Result<(), CgroupError> {
        if let Some(group) = self.cgroups.get_mut(name) {
            group.limits.memory_max = max_bytes;
            Ok(())
        } else {
            Err(CgroupError::NotFound)
        }
    }

    /// Configure process count limit
    pub fn set_pids_max(&mut self, name: &str, max_pids: u32) -> Result<(), CgroupError> {
        if let Some(group) = self.cgroups.get_mut(name) {
            group.limits.pids_max = max_pids;
            Ok(())
        } else {
            Err(CgroupError::NotFound)
        }
    }

    /// Track CPU usage addition in ms
    pub fn track_cpu_usage(&mut self, pid: u64, duration_ms: u64) -> Result<(), CgroupError> {
        let mut found_name = None;
        for (name, group) in &self.cgroups {
            if group.pids.contains(&pid) {
                found_name = Some(name.clone());
                break;
            }
        }

        if let Some(ref name) = found_name {
            let mut curr = Some(name.clone());
            while let Some(cname) = curr {
                let parent = if let Some(group) = self.cgroups.get_mut(&cname) {
                    group.usage.cpu_usage_ms = group.usage.cpu_usage_ms.saturating_add(duration_ms);
                    group.parent_name.clone()
                } else {
                    None
                };
                curr = parent;
            }
            Ok(())
        } else {
            // Un-grouped processes are accounted on Root
            if let Some(group) = self.cgroups.get_mut("/") {
                group.usage.cpu_usage_ms = group.usage.cpu_usage_ms.saturating_add(duration_ms);
            }
            Ok(())
        }
    }

    /// Track resource allocation (checks limits hierarchically)
    pub fn track_memory_alloc(&mut self, pid: u64, bytes: u64) -> Result<(), CgroupError> {
        let mut found_name = None;
        for (name, group) in &self.cgroups {
            if group.pids.contains(&pid) {
                found_name = Some(name.clone());
                break;
            }
        }

        let name = found_name.unwrap_or(String::from("/"));

        // Dry run: check if any ancestor group exceeds its limits
        let mut curr = Some(name.clone());
        while let Some(cname) = curr {
            if let Some(group) = self.cgroups.get(&cname) {
                if group.usage.memory_usage_bytes.saturating_add(bytes) > group.limits.memory_max {
                    return Err(CgroupError::LimitExceeded);
                }
                curr = group.parent_name.clone();
            } else {
                break;
            }
        }

        // Apply allocations hierarchically
        let mut curr = Some(name);
        while let Some(cname) = curr {
            let parent = if let Some(group) = self.cgroups.get_mut_str(&cname) {
                group.usage.memory_usage_bytes = group.usage.memory_usage_bytes.saturating_add(bytes);
                group.parent_name.clone()
            } else {
                None
            };
            curr = parent;
        }

        Ok(())
    }

    /// Track resource release (decrements hierarchically)
    pub fn track_memory_free(&mut self, pid: u64, bytes: u64) -> Result<(), CgroupError> {
        let mut found_name = None;
        for (name, group) in &self.cgroups {
            if group.pids.contains(&pid) {
                found_name = Some(name.clone());
                break;
            }
        }

        let name = found_name.unwrap_or(String::from("/"));

        // Release hierarchically
        let mut curr = Some(name);
        while let Some(cname) = curr {
            let parent = if let Some(group) = self.cgroups.get_mut(&cname) {
                group.usage.memory_usage_bytes = group.usage.memory_usage_bytes.saturating_sub(bytes);
                group.parent_name.clone()
            } else {
                None
            };
            curr = parent;
        }

        Ok(())
    }

    /// Get current cgroup of a PID
    pub fn get_cgroup_of_pid(&self, pid: u64) -> Option<&Cgroup> {
        for group in self.cgroups.values() {
            if group.pids.contains(&pid) {
                return Some(group);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cgroup_creation_and_hierarchy() {
        let mut manager = CgroupManager::new();
        assert!(manager.cgroups.contains_key("/"));

        assert_eq!(manager.create_cgroup("/sys", Some("/")), Ok(()));
        assert_eq!(manager.create_cgroup("/sys/db", Some("/sys")), Ok(()));

        // Attempting to create duplicate group
        assert_eq!(manager.create_cgroup("/sys", Some("/")), Err(CgroupError::AlreadyExists));

        // Attempting to create under non-existent parent
        assert_eq!(manager.create_cgroup("/app/web", Some("/app")), Err(CgroupError::ParentNotFound));

        let db_group = manager.cgroups.get("/sys/db").unwrap();
        assert_eq!(db_group.parent_name.as_deref(), Some("/sys"));
    }

    #[test]
    fn test_cgroup_pid_association() {
        let mut manager = CgroupManager::new();
        assert_eq!(manager.create_cgroup("/user", None), Ok(()));
        assert_eq!(manager.create_cgroup("/user/alice", Some("/user")), Ok(()));

        assert_eq!(manager.attach_pid("/user/alice", 1001), Ok(()));
        assert_eq!(manager.get_cgroup_of_pid(1001).unwrap().name, "/user/alice");

        // Migrate PID to another group
        assert_eq!(manager.create_cgroup("/user/bob", Some("/user")), Ok(()));
        assert_eq!(manager.attach_pid("/user/bob", 1001), Ok(()));
        assert_eq!(manager.get_cgroup_of_pid(1001).unwrap().name, "/user/bob");

        // Old group should no longer contain PID
        let alice_group = manager.cgroups.get("/user/alice").unwrap();
        assert!(!alice_group.pids.contains(&1001));
        assert_eq!(alice_group.usage.pids_count, 0);

        let bob_group = manager.cgroups.get("/user/bob").unwrap();
        assert!(bob_group.pids.contains(&1001));
        assert_eq!(bob_group.usage.pids_count, 1);
    }

    #[test]
    fn test_cgroup_memory_limit_enforcement() {
        let mut manager = CgroupManager::new();
        assert_eq!(manager.create_cgroup("/app", None), Ok(()));
        assert_eq!(manager.set_memory_max("/app", 1000), Ok(()));

        assert_eq!(manager.attach_pid("/app", 101), Ok(()));

        // First allocation fits
        assert_eq!(manager.track_memory_alloc(101, 600), Ok(()));
        assert_eq!(manager.cgroups.get("/app").unwrap().usage.memory_usage_bytes, 600);

        // Second allocation exceeds limit
        assert_eq!(manager.track_memory_alloc(101, 500), Err(CgroupError::LimitExceeded));
        assert_eq!(manager.cgroups.get("/app").unwrap().usage.memory_usage_bytes, 600); // usage unchanged

        // Release some memory
        assert_eq!(manager.track_memory_free(101, 200), Ok(()));
        assert_eq!(manager.cgroups.get("/app").unwrap().usage.memory_usage_bytes, 400);

        // Now the second allocation fits
        assert_eq!(manager.track_memory_alloc(101, 500), Ok(()));
        assert_eq!(manager.cgroups.get("/app").unwrap().usage.memory_usage_bytes, 900);
    }

    #[test]
    fn test_cgroup_pids_max_limit_enforcement() {
        let mut manager = CgroupManager::new();
        assert_eq!(manager.create_cgroup("/restricted", None), Ok(()));
        assert_eq!(manager.set_pids_max("/restricted", 2), Ok(()));

        assert_eq!(manager.attach_pid("/restricted", 201), Ok(()));
        assert_eq!(manager.attach_pid("/restricted", 202), Ok(()));
        // Third attach exceeds process limit
        assert_eq!(manager.attach_pid("/restricted", 203), Err(CgroupError::LimitExceeded));
    }
}
