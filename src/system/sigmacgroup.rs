//! SigmaCgroup - Control Groups Management System
//!
//! Provides comprehensive control groups management inspired by Linux cgroups v2.
//! Supports resource limiting, accounting, and control for process groups.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Cgroup controller type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CgroupController {
    Cpu,
    Memory,
    Io,
    Pids,
    CpuSet,
    Unknown,
}

/// Cgroup state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CgroupState {
    Active,
    Inactive,
    Throttled,
    Unknown,
}

/// CPU controller configuration
#[derive(Debug, Clone)]
pub struct CpuControllerConfig {
    pub cpu_shares: Option<u64>,
    pub cpu_quota: Option<i64>,
    pub cpu_period: Option<u64>,
    pub rt_runtime: Option<i64>,
    pub rt_period: Option<u64>,
}

impl CpuControllerConfig {
    pub fn new() -> Self {
        Self {
            cpu_shares: None,
            cpu_quota: None,
            cpu_period: None,
            rt_runtime: None,
            rt_period: None,
        }
    }

    pub fn with_shares(mut self, shares: u64) -> Self {
        self.cpu_shares = Some(shares);
        self
    }

    pub fn with_quota(mut self, quota: i64) -> Self {
        self.cpu_quota = Some(quota);
        self
    }

    pub fn with_period(mut self, period: u64) -> Self {
        self.cpu_period = Some(period);
        self
    }
}

impl Default for CpuControllerConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory controller configuration
#[derive(Debug, Clone)]
pub struct MemoryControllerConfig {
    pub memory_limit: Option<u64>,
    pub memory_swap_limit: Option<u64>,
    pub memory_soft_limit: Option<u64>,
    pub oom_control: Option<bool>,
}

impl MemoryControllerConfig {
    pub fn new() -> Self {
        Self {
            memory_limit: None,
            memory_swap_limit: None,
            memory_soft_limit: None,
            oom_control: None,
        }
    }

    pub fn with_limit(mut self, limit: u64) -> Self {
        self.memory_limit = Some(limit);
        self
    }

    pub fn with_swap_limit(mut self, limit: u64) -> Self {
        self.memory_swap_limit = Some(limit);
        self
    }

    pub fn with_soft_limit(mut self, limit: u64) -> Self {
        self.memory_soft_limit = Some(limit);
        self
    }

    pub fn with_oom_control(mut self, enabled: bool) -> Self {
        self.oom_control = Some(enabled);
        self
    }
}

impl Default for MemoryControllerConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// PIDs controller configuration
#[derive(Debug, Clone)]
pub struct PidsControllerConfig {
    pub pids_max: Option<i64>,
}

impl PidsControllerConfig {
    pub fn new() -> Self {
        Self { pids_max: None }
    }

    pub fn with_max(mut self, max: i64) -> Self {
        self.pids_max = Some(max);
        self
    }
}

impl Default for PidsControllerConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Cgroup
#[derive(Debug, Clone)]
pub struct Cgroup {
    pub id: String,
    pub name: String,
    pub parent: Option<String>,
    pub state: CgroupState,
    pub processes: Vec<u32>,
    pub cpu_config: Option<CpuControllerConfig>,
    pub memory_config: Option<MemoryControllerConfig>,
    pub pids_config: Option<PidsControllerConfig>,
}

impl Cgroup {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            parent: None,
            state: CgroupState::Active,
            processes: Vec::new(),
            cpu_config: None,
            memory_config: None,
            pids_config: None,
        }
    }

    pub fn with_parent(mut self, parent: String) -> Self {
        self.parent = Some(parent);
        self
    }

    pub fn with_cpu_config(mut self, config: CpuControllerConfig) -> Self {
        self.cpu_config = Some(config);
        self
    }

    pub fn with_memory_config(mut self, config: MemoryControllerConfig) -> Self {
        self.memory_config = Some(config);
        self
    }

    pub fn with_pids_config(mut self, config: PidsControllerConfig) -> Self {
        self.pids_config = Some(config);
        self
    }

    pub fn add_process(&mut self, pid: u32) {
        self.processes.push(pid);
    }

    pub fn remove_process(&mut self, pid: u32) -> bool {
        if let Some(pos) = self.processes.iter().position(|&p| p == pid) {
            self.processes.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn activate(&mut self) {
        self.state = CgroupState::Active;
    }

    pub fn deactivate(&mut self) {
        self.state = CgroupState::Inactive;
    }

    pub fn throttle(&mut self) {
        self.state = CgroupState::Throttled;
    }
}

/// Cgroup manager
pub struct SigmaCgroup {
    cgroups: BTreeMap<String, Cgroup>,
    next_cgroup_id: u64,
}

impl SigmaCgroup {
    pub fn new() -> Self {
        Self {
            cgroups: BTreeMap::new(),
            next_cgroup_id: 1,
        }
    }

    pub fn create_cgroup(&mut self, name: String, parent: Option<String>) -> String {
        let id = format!("cgroup-{}", self.next_cgroup_id);
        self.next_cgroup_id += 1;
        let mut cgroup = Cgroup::new(id.clone(), name);
        if let Some(p) = parent {
            cgroup = cgroup.with_parent(p);
        }
        self.cgroups.insert(id.clone(), cgroup);
        id
    }

    pub fn get_cgroup(&self, id: &str) -> Option<&Cgroup> {
        self.cgroups.get(id)
    }

    pub fn get_cgroup_mut(&mut self, id: &str) -> Option<&mut Cgroup> {
        self.cgroups.get_mut(id)
    }

    pub fn list_cgroups(&self) -> Vec<&Cgroup> {
        self.cgroups.values().collect()
    }

    pub fn list_cgroups_by_parent(&self, parent: &str) -> Vec<&Cgroup> {
        self.cgroups
            .values()
            .filter(|c| c.parent.as_deref() == Some(parent))
            .collect()
    }

    pub fn list_cgroups_by_state(&self, state: CgroupState) -> Vec<&Cgroup> {
        self.cgroups.values().filter(|c| c.state == state).collect()
    }

    pub fn remove_cgroup(&mut self, id: &str) -> bool {
        self.cgroups.remove(id).is_some()
    }

    pub fn add_process_to_cgroup(&mut self, cgroup_id: &str, pid: u32) -> bool {
        if let Some(cgroup) = self.cgroups.get_mut(cgroup_id) {
            cgroup.add_process(pid);
            true
        } else {
            false
        }
    }

    pub fn remove_process_from_cgroup(&mut self, cgroup_id: &str, pid: u32) -> bool {
        if let Some(cgroup) = self.cgroups.get_mut(cgroup_id) {
            cgroup.remove_process(pid)
        } else {
            false
        }
    }

    pub fn activate_cgroup(&mut self, id: &str) -> bool {
        if let Some(cgroup) = self.cgroups.get_mut(id) {
            cgroup.activate();
            true
        } else {
            false
        }
    }

    pub fn deactivate_cgroup(&mut self, id: &str) -> bool {
        if let Some(cgroup) = self.cgroups.get_mut(id) {
            cgroup.deactivate();
            true
        } else {
            false
        }
    }

    pub fn set_cpu_config(&mut self, id: &str, config: CpuControllerConfig) -> bool {
        if let Some(cgroup) = self.cgroups.get_mut(id) {
            cgroup.cpu_config = Some(config);
            true
        } else {
            false
        }
    }

    pub fn set_memory_config(&mut self, id: &str, config: MemoryControllerConfig) -> bool {
        if let Some(cgroup) = self.cgroups.get_mut(id) {
            cgroup.memory_config = Some(config);
            true
        } else {
            false
        }
    }

    pub fn set_pids_config(&mut self, id: &str, config: PidsControllerConfig) -> bool {
        if let Some(cgroup) = self.cgroups.get_mut(id) {
            cgroup.pids_config = Some(config);
            true
        } else {
            false
        }
    }

    pub fn cgroup_count(&self) -> usize {
        self.cgroups.len()
    }

    pub fn active_cgroup_count(&self) -> usize {
        self.cgroups
            .values()
            .filter(|c| c.state == CgroupState::Active)
            .count()
    }
}

impl Default for SigmaCgroup {
    fn default() -> Self {
        Self::new()
    }
}

static mut SIGMACGROUP_INSTANCE: Option<SigmaCgroup> = None;

pub fn init_sigmacgroup() {
    unsafe {
        SIGMACGROUP_INSTANCE = Some(SigmaCgroup::new());
    }
}

pub fn get_sigmacgroup() -> &'static SigmaCgroup {
    unsafe {
        SIGMACGROUP_INSTANCE
            .as_ref()
            .expect("SigmaCgroup not initialized")
    }
}

pub fn get_sigmacgroup_mut() -> &'static mut SigmaCgroup {
    unsafe {
        SIGMACGROUP_INSTANCE
            .as_mut()
            .expect("SigmaCgroup not initialized")
    }
}

pub fn create_cgroup(name: String, parent: Option<String>) -> String {
    let manager = get_sigmacgroup_mut();
    manager.create_cgroup(name, parent)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_controller_config() {
        let config = CpuControllerConfig::new()
            .with_shares(1024)
            .with_quota(100000)
            .with_period(1000000);

        assert_eq!(config.cpu_shares, Some(1024));
        assert_eq!(config.cpu_quota, Some(100000));
        assert_eq!(config.cpu_period, Some(1000000));
    }

    #[test]
    fn test_memory_controller_config() {
        let config = MemoryControllerConfig::new()
            .with_limit(1024 * 1024 * 1024)
            .with_swap_limit(2 * 1024 * 1024 * 1024)
            .with_soft_limit(512 * 1024 * 1024)
            .with_oom_control(true);

        assert_eq!(config.memory_limit, Some(1024 * 1024 * 1024));
        assert_eq!(config.memory_swap_limit, Some(2 * 1024 * 1024 * 1024));
        assert_eq!(config.memory_soft_limit, Some(512 * 1024 * 1024));
        assert_eq!(config.oom_control, Some(true));
    }

    #[test]
    fn test_pids_controller_config() {
        let config = PidsControllerConfig::new().with_max(1000);
        assert_eq!(config.pids_max, Some(1000));
    }

    #[test]
    fn test_cgroup_creation() {
        let cgroup = Cgroup::new("cgroup-1".to_string(), "test".to_string());
        assert_eq!(cgroup.id, "cgroup-1");
        assert_eq!(cgroup.name, "test");
        assert_eq!(cgroup.state, CgroupState::Active);
        assert!(cgroup.parent.is_none());
    }

    #[test]
    fn test_cgroup_builder() {
        let cgroup = Cgroup::new("cgroup-1".to_string(), "test".to_string())
            .with_parent("parent-cgroup".to_string())
            .with_cpu_config(CpuControllerConfig::new().with_shares(1024))
            .with_memory_config(MemoryControllerConfig::new().with_limit(1024 * 1024 * 1024))
            .with_pids_config(PidsControllerConfig::new().with_max(100));

        assert_eq!(cgroup.parent, Some("parent-cgroup".to_string()));
        assert!(cgroup.cpu_config.is_some());
        assert!(cgroup.memory_config.is_some());
        assert!(cgroup.pids_config.is_some());
    }

    #[test]
    fn test_cgroup_process_management() {
        let mut cgroup = Cgroup::new("cgroup-1".to_string(), "test".to_string());
        cgroup.add_process(1234);
        cgroup.add_process(5678);

        assert_eq!(cgroup.processes.len(), 2);
        assert!(cgroup.remove_process(1234));
        assert_eq!(cgroup.processes.len(), 1);
        assert!(!cgroup.remove_process(9999));
    }

    #[test]
    fn test_cgroup_state_transitions() {
        let mut cgroup = Cgroup::new("cgroup-1".to_string(), "test".to_string());
        assert_eq!(cgroup.state, CgroupState::Active);

        cgroup.deactivate();
        assert_eq!(cgroup.state, CgroupState::Inactive);

        cgroup.activate();
        assert_eq!(cgroup.state, CgroupState::Active);

        cgroup.throttle();
        assert_eq!(cgroup.state, CgroupState::Throttled);
    }

    #[test]
    fn test_cgroup_manager() {
        let manager = SigmaCgroup::new();
        assert_eq!(manager.cgroup_count(), 0);
    }

    #[test]
    fn test_create_cgroup() {
        let mut manager = SigmaCgroup::new();
        let id = manager.create_cgroup("test".to_string(), None);
        assert_eq!(id, "cgroup-1");
        assert_eq!(manager.cgroup_count(), 1);
    }

    #[test]
    fn test_create_cgroup_with_parent() {
        let mut manager = SigmaCgroup::new();
        let parent_id = manager.create_cgroup("parent".to_string(), None);
        let child_id = manager.create_cgroup("child".to_string(), Some(parent_id.clone()));

        let child = manager.get_cgroup(&child_id).unwrap();
        assert_eq!(child.parent, Some(parent_id));
    }

    #[test]
    fn test_add_remove_process_from_cgroup() {
        let mut manager = SigmaCgroup::new();
        let id = manager.create_cgroup("test".to_string(), None);

        assert!(manager.add_process_to_cgroup(&id, 1234));
        assert_eq!(manager.get_cgroup(&id).unwrap().processes.len(), 1);

        assert!(manager.remove_process_from_cgroup(&id, 1234));
        assert_eq!(manager.get_cgroup(&id).unwrap().processes.len(), 0);
    }

    #[test]
    fn test_list_cgroups_by_parent() {
        let mut manager = SigmaCgroup::new();
        let parent_id = manager.create_cgroup("parent".to_string(), None);
        manager.create_cgroup("child1".to_string(), Some(parent_id.clone()));
        manager.create_cgroup("child2".to_string(), Some(parent_id.clone()));

        let children = manager.list_cgroups_by_parent(&parent_id);
        assert_eq!(children.len(), 2);
    }

    #[test]
    fn test_list_cgroups_by_state() {
        let mut manager = SigmaCgroup::new();
        let id1 = manager.create_cgroup("test1".to_string(), None);
        let id2 = manager.create_cgroup("test2".to_string(), None);
        manager.deactivate_cgroup(&id2);

        let active = manager.list_cgroups_by_state(CgroupState::Active);
        assert_eq!(active.len(), 1);

        let inactive = manager.list_cgroups_by_state(CgroupState::Inactive);
        assert_eq!(inactive.len(), 1);
    }

    #[test]
    fn test_set_controller_configs() {
        let mut manager = SigmaCgroup::new();
        let id = manager.create_cgroup("test".to_string(), None);

        let cpu_config = CpuControllerConfig::new().with_shares(1024);
        assert!(manager.set_cpu_config(&id, cpu_config));

        let memory_config = MemoryControllerConfig::new().with_limit(1024 * 1024 * 1024);
        assert!(manager.set_memory_config(&id, memory_config));

        let pids_config = PidsControllerConfig::new().with_max(100);
        assert!(manager.set_pids_config(&id, pids_config));
    }

    #[test]
    fn test_active_cgroup_count() {
        let mut manager = SigmaCgroup::new();
        let id1 = manager.create_cgroup("test1".to_string(), None);
        let id2 = manager.create_cgroup("test2".to_string(), None);
        manager.deactivate_cgroup(&id2);

        assert_eq!(manager.active_cgroup_count(), 1);
    }

    #[test]
    fn test_global_instance() {
        init_sigmacgroup();
        let manager = get_sigmacgroup();
        assert_eq!(manager.cgroup_count(), 0);

        let manager_mut = get_sigmacgroup_mut();
        manager_mut.create_cgroup("test".to_string(), None);
        assert_eq!(manager.cgroup_count(), 1);
    }

    #[test]
    fn test_create_cgroup_convenience() {
        init_sigmacgroup();
        let id = create_cgroup("test".to_string(), None);
        assert_eq!(id, "cgroup-1");

        let manager = get_sigmacgroup();
        assert!(manager.get_cgroup(&id).is_some());
    }
}
