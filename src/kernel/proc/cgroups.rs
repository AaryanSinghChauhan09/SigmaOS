/// SigmaOS cgroups v2 resource controller implementation
/// Controls CPU limits, Memory limits, and PID limits
/// Enhanced with active memory quota tracking and CPU time-slice throttling

use crate::klib::HashMap;
use std::string::{String, ToString};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceLimits {
    pub cpu_shares: u32,
    pub max_memory_bytes: u64,
    pub max_pids: u32,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        ResourceLimits {
            cpu_shares: 1024,
            max_memory_bytes: 4 * 1024 * 1024 * 1024, // 4GB
            max_pids: 2048,
        }
    }
}

pub struct Cgroup {
    pub name: String,
    pub limits: ResourceLimits,
    pub pids: Vec<u64>,
    pub current_memory_bytes: u64,
    pub current_cpu_time_us: u64,
}

impl Cgroup {
    pub fn new(name: &str) -> Self {
        Cgroup {
            name: name.to_string(),
            limits: ResourceLimits::default(),
            pids: Vec::new(),
            current_memory_bytes: 0,
            current_cpu_time_us: 0,
        }
    }
}

pub struct CgroupManager {
    groups: HashMap<String, Cgroup>,
}

impl CgroupManager {
    pub fn new() -> Self {
        let mut groups = HashMap::new();
        groups.insert("root".to_string(), Cgroup::new("root"));
        CgroupManager { groups }
    }

    pub fn create_group(&mut self, name: &str, limits: ResourceLimits) -> Result<(), &'static str> {
        if self.groups.contains_key(name) {
            return Err("Cgroup already exists");
        }
        let mut group = Cgroup::new(name);
        group.limits = limits;
        self.groups.insert(name.to_string(), group);
        Ok(())
    }

    pub fn attach_process(&mut self, name: &str, pid: u64) -> Result<(), &'static str> {
        // Remove PID from previous groups first
        for g in self.groups.values_mut() {
            g.pids.retain(|&x| x != pid);
        }

        let group = self.groups.get_mut(name).ok_or("Cgroup not found")?;
        if group.pids.len() >= group.limits.max_pids as usize {
            return Err("PID limit exceeded for this cgroup");
        }
        group.pids.push(pid);
        Ok(())
    }

    /// Evaluates if a memory allocation request exceeds cgroup memory quota limits
    pub fn check_memory_quota(&mut self, name: &str, bytes_to_alloc: u64) -> Result<(), &'static str> {
        let group = self.groups.get_mut(name).ok_or("Cgroup not found")?;
        if group.current_memory_bytes + bytes_to_alloc > group.limits.max_memory_bytes {
            return Err("Cgroup memory quota exceeded (OOM mitigation triggered)");
        }
        group.current_memory_bytes += bytes_to_alloc;
        Ok(())
    }

    /// Releases memory usage when a process deallocates memory
    pub fn release_memory(&mut self, name: &str, bytes_to_free: u64) {
        if let Some(group) = self.groups.get_mut(name) {
            group.current_memory_bytes = group.current_memory_bytes.saturating_sub(bytes_to_free);
        }
    }

    /// Evaluates CPU time-slice quota throttling against cpu_shares
    pub fn check_cpu_quota(&mut self, name: &str, cpu_time_us: u64) -> Result<bool, &'static str> {
        let group = self.groups.get_mut(name).ok_or("Cgroup not found")?;
        group.current_cpu_time_us += cpu_time_us;

        let max_allowed_us = (group.limits.cpu_shares as u64) * 1000;
        if group.current_cpu_time_us > max_allowed_us {
            Ok(true) // Throttling required
        } else {
            Ok(false) // Within quota
        }
    }

    pub fn get_limits(&self, name: &str) -> Option<ResourceLimits> {
        self.groups.get(name).map(|g| g.limits)
    }

    pub fn get_group_of_pid(&self, pid: u64) -> Option<&Cgroup> {
        self.groups.values().find(|g| g.pids.contains(&pid))
    }
}

impl Default for CgroupManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cgroup_lifecycle() {
        let mut cgm = CgroupManager::new();
        let custom_limits = ResourceLimits {
            cpu_shares: 512,
            max_memory_bytes: 1024 * 1024,
            max_pids: 2,
        };

        cgm.create_group("database", custom_limits).unwrap();
        assert_eq!(cgm.get_limits("database").unwrap().cpu_shares, 512);

        cgm.attach_process("database", 201).unwrap();
        cgm.attach_process("database", 202).unwrap();
        assert_eq!(
            cgm.attach_process("database", 203),
            Err("PID limit exceeded for this cgroup")
        );

        let group = cgm.get_group_of_pid(201).unwrap();
        assert_eq!(group.name, "database");
    }

    #[test]
    fn test_cgroup_memory_and_cpu_quotas() {
        let mut cgm = CgroupManager::new();
        let limits = ResourceLimits {
            cpu_shares: 100, // 100,000 us quota
            max_memory_bytes: 1024 * 1024, // 1 MB
            max_pids: 10,
        };

        cgm.create_group("sandbox", limits).unwrap();

        // 1. Memory quota check
        assert!(cgm.check_memory_quota("sandbox", 512 * 1024).is_ok());
        assert!(cgm.check_memory_quota("sandbox", 600 * 1024).is_err()); // Exceeds 1MB

        cgm.release_memory("sandbox", 512 * 1024);
        assert!(cgm.check_memory_quota("sandbox", 200 * 1024).is_ok());

        // 2. CPU quota check
        let throttled_before = cgm.check_cpu_quota("sandbox", 50_000).unwrap();
        assert!(!throttled_before);

        let throttled_after = cgm.check_cpu_quota("sandbox", 60_000).unwrap();
        assert!(throttled_after); // 110,000 us > 100,000 us threshold
    }
}
