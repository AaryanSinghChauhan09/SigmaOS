#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

/// SigmaOS cgroups v2 resource controller implementation
/// Controls CPU limits, Memory limits, and PID limits
use crate::klib::BTreeMap;
use alloc::string::{String, ToString};

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
}

impl Cgroup {
    pub fn new(name: &str) -> Self {
        Cgroup {
            name: name.to_string(),
            limits: ResourceLimits::default(),
            pids: Vec::new(),
        }
    }
}

pub struct CgroupManager {
    groups: BTreeMap<String, Cgroup>,
}

impl CgroupManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut groups = BTreeMap::new();
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
}
