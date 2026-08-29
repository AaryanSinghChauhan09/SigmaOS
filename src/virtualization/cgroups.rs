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

// Cgroups - Linux-style Control Groups for resource management
// Supports CPU, memory, I/O, and device controller subsystems

// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CgroupSubsystem {
    Cpu,     // CPU scheduling and accounting
    Memory,  // Memory usage and limits
    Ios,     // I/O throttling
    Devices, // Device access control
    Cpuset,  // CPU affinity
    Pids,    // Process ID limits
    Freezer, // Process freezing
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CgroupState {
    Active,
    Frozen,
    Thawed,
}

#[derive(Debug, Clone)]
pub struct Cgroup {
    pub name: String,
    pub path: String,
    pub parent: Option<String>,
    pub state: CgroupState,
    pub subsystems: BTreeMap<CgroupSubsystem, CgroupController>,
}

#[derive(Debug, Clone)]
pub struct CgroupController {
    pub enabled: bool,
    pub parameters: BTreeMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct CpuController {
    pub shares: u64,
    pub quota: i64, // -1 for unlimited
    pub period: u64,
    pub rt_runtime: i64,
    pub rt_period: u64,
}

#[derive(Debug, Clone)]
pub struct MemoryController {
    pub limit: u64, // -1 for unlimited
    pub swap_limit: u64,
    pub soft_limit: u64,
    pub oom_control: bool,
}

#[derive(Debug, Clone)]
pub struct PidController {
    pub max: i64, // -1 for unlimited
    pub current: i64,
}

pub struct CgroupManager {
    cgroups: BTreeMap<String, Cgroup>,
    root_cgroup: Option<String>,
}

impl CgroupManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            cgroups: BTreeMap::new(),
            root_cgroup: None,
        }
    }

    /// Create a new cgroup
    pub fn create_cgroup(
        &mut self,
        name: String,
        path: String,
        parent: Option<String>,
    ) -> Result<(), &'static str> {
        if self.cgroups.contains_key(&path) {
            return Err("Cgroup already exists");
        }

        let cgroup = Cgroup {
            name: name.clone(),
            path: path.clone(),
            parent: parent.clone(),
            state: CgroupState::Active,
            subsystems: BTreeMap::new(),
        };

        self.cgroups.insert(path.clone(), cgroup);

        // Set as root if no parent
        if parent.is_none() && self.root_cgroup.is_none() {
            self.root_cgroup = Some(name);
        }

        Ok(())
    }

    /// Get a cgroup by path
    pub fn get_cgroup(&self, path: &str) -> Option<&Cgroup> {
        self.cgroups.get(path)
    }

    /// Get a mutable cgroup by path
    pub fn get_cgroup_mut(&mut self, path: &str) -> Option<&mut Cgroup> {
        self.cgroups.get_mut(path)
    }

    /// Enable a subsystem for a cgroup
    pub fn enable_subsystem(
        &mut self,
        path: &str,
        subsystem: CgroupSubsystem,
    ) -> Result<(), &'static str> {
        let cgroup = self.cgroups.get_mut(path).ok_or("Cgroup not found")?;

        let controller = CgroupController {
            enabled: true,
            parameters: BTreeMap::new(),
        };

        cgroup.subsystems.insert(subsystem, controller);
        Ok(())
    }

    /// Set a controller parameter
    pub fn set_parameter(
        &mut self,
        path: &str,
        subsystem: CgroupSubsystem,
        key: String,
        value: String,
    ) -> Result<(), &'static str> {
        let cgroup = self.cgroups.get_mut(path).ok_or("Cgroup not found")?;

        let controller = cgroup
            .subsystems
            .get_mut(&subsystem)
            .ok_or("Subsystem not enabled")?;

        controller.parameters.insert(key, value);
        Ok(())
    }

    /// Get a controller parameter
    pub fn get_parameter(
        &self,
        path: &str,
        subsystem: CgroupSubsystem,
        key: &str,
    ) -> Option<&String> {
        let cgroup = self.cgroups.get(path)?;
        let controller = cgroup.subsystems.get(&subsystem)?;
        controller.parameters.get(key)
    }

    /// Delete a cgroup
    pub fn delete_cgroup(&mut self, path: &str) -> Result<(), &'static str> {
        if let Some(cgroup) = self.cgroups.get(path) {
            if Some(&cgroup.name) == self.root_cgroup.as_ref() {
                return Err("Cannot delete root cgroup");
            }
        }

        self.cgroups.remove(path).ok_or("Cgroup not found")?;

        Ok(())
    }

    /// Set cgroup state
    pub fn set_state(&mut self, path: &str, state: CgroupState) -> Result<(), &'static str> {
        let cgroup = self.cgroups.get_mut(path).ok_or("Cgroup not found")?;

        cgroup.state = state;
        Ok(())
    }

    /// Get cgroup count
    pub fn cgroup_count(&self) -> usize {
        self.cgroups.len()
    }

    /// Get root cgroup
    pub fn root_cgroup(&self) -> Option<&String> {
        self.root_cgroup.as_ref()
    }

    /// List all cgroups
    pub fn list_cgroups(&self) -> Vec<&Cgroup> {
        self.cgroups.values().collect()
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
    fn test_create_cgroup() {
        let mut manager = CgroupManager::new();

        manager
            .create_cgroup("root".to_string(), "/".to_string(), None)
            .unwrap();
        assert_eq!(manager.cgroup_count(), 1);
        assert_eq!(manager.root_cgroup(), Some(&"root".to_string()));
    }

    #[test]
    fn test_enable_subsystem() {
        let mut manager = CgroupManager::new();

        manager
            .create_cgroup("test".to_string(), "/test".to_string(), None)
            .unwrap();
        manager
            .enable_subsystem("/test", CgroupSubsystem::Cpu)
            .unwrap();

        let cgroup = manager.get_cgroup("/test").unwrap();
        assert!(cgroup.subsystems.contains_key(&CgroupSubsystem::Cpu));
    }

    #[test]
    fn test_set_parameter() {
        let mut manager = CgroupManager::new();

        manager
            .create_cgroup("test".to_string(), "/test".to_string(), None)
            .unwrap();
        manager
            .enable_subsystem("/test", CgroupSubsystem::Cpu)
            .unwrap();
        manager
            .set_parameter(
                "/test",
                CgroupSubsystem::Cpu,
                "shares".to_string(),
                "1024".to_string(),
            )
            .unwrap();

        let value = manager.get_parameter("/test", CgroupSubsystem::Cpu, "shares");
        assert_eq!(value, Some(&"1024".to_string()));
    }

    #[test]
    fn test_delete_cgroup() {
        let mut manager = CgroupManager::new();

        manager
            .create_cgroup("root".to_string(), "/".to_string(), None)
            .unwrap();
        manager
            .create_cgroup(
                "child".to_string(),
                "/child".to_string(),
                Some("/".to_string()),
            )
            .unwrap();
        manager.delete_cgroup("/child").unwrap();

        assert_eq!(manager.cgroup_count(), 1);
    }

    #[test]
    fn test_delete_root_cgroup() {
        let mut manager = CgroupManager::new();

        manager
            .create_cgroup("root".to_string(), "/".to_string(), None)
            .unwrap();
        let result = manager.delete_cgroup("/");

        assert!(result.is_err());
    }

    #[test]
    fn test_set_state() {
        let mut manager = CgroupManager::new();

        manager
            .create_cgroup("test".to_string(), "/test".to_string(), None)
            .unwrap();
        manager.set_state("/test", CgroupState::Frozen).unwrap();

        let cgroup = manager.get_cgroup("/test").unwrap();
        assert_eq!(cgroup.state, CgroupState::Frozen);
    }

    #[test]
    fn test_list_cgroups() {
        let mut manager = CgroupManager::new();

        manager
            .create_cgroup("root".to_string(), "/".to_string(), None)
            .unwrap();
        manager
            .create_cgroup(
                "child".to_string(),
                "/child".to_string(),
                Some("/".to_string()),
            )
            .unwrap();

        let cgroups = manager.list_cgroups();
        assert_eq!(cgroups.len(), 2);
    }

    #[test]
    fn test_multiple_subsystems() {
        let mut manager = CgroupManager::new();

        manager
            .create_cgroup("test".to_string(), "/test".to_string(), None)
            .unwrap();
        manager
            .enable_subsystem("/test", CgroupSubsystem::Cpu)
            .unwrap();
        manager
            .enable_subsystem("/test", CgroupSubsystem::Memory)
            .unwrap();
        manager
            .enable_subsystem("/test", CgroupSubsystem::Ios)
            .unwrap();

        let cgroup = manager.get_cgroup("/test").unwrap();
        assert_eq!(cgroup.subsystems.len(), 3);
    }
}
