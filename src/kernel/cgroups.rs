// SPDX-License-Identifier: MIT
// SigmaOS Cgroups (Control Groups) Subsystem
// Resource management and process grouping inspired by Linux cgroups v2

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, AtomicU32, AtomicBool, Ordering};

/// Cgroup ID type
pub type CgroupId = u64;

/// Cgroup controller type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CgroupController {
    Cpu,
    Memory,
    Io,
    Pids,
    CpuSet,
    Devices,
    Freezer,
}

/// Cgroup state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CgroupState {
    Active,
    Frozen,
    Dying,
}

/// Cgroup resource limits
#[derive(Debug, Clone)]
pub struct CgroupLimits {
    pub cpu_shares: u64,
    pub cpu_quota_us: i64,  // -1 means unlimited
    pub cpu_period_us: u64,
    pub memory_limit_bytes: u64,
    pub memory_swap_limit_bytes: u64,
    pub pids_max: u64,
}

impl Default for CgroupLimits {
    fn default() -> Self {
        CgroupLimits {
            cpu_shares: 1024,
            cpu_quota_us: -1,
            cpu_period_us: 100000,
            memory_limit_bytes: u64::MAX,
            memory_swap_limit_bytes: u64::MAX,
            pids_max: u64::MAX,
        }
    }
}

/// Cgroup statistics
#[derive(Debug)]
pub struct CgroupStats {
    pub cpu_usage_ns: AtomicU64,
    pub memory_usage_bytes: AtomicU64,
    pub current_pids: AtomicU32,
    pub swap_usage_bytes: AtomicU64,
}

impl Default for CgroupStats {
    fn default() -> Self {
        CgroupStats {
            cpu_usage_ns: AtomicU64::new(0),
            memory_usage_bytes: AtomicU64::new(0),
            current_pids: AtomicU32::new(0),
            swap_usage_bytes: AtomicU64::new(0),
        }
    }
}

/// Cgroup descriptor
#[derive(Debug)]
pub struct Cgroup {
    pub id: CgroupId,
    pub name: String,
    pub parent_id: Option<CgroupId>,
    pub state: AtomicU32, // CgroupState as u32
    pub limits: CgroupLimits,
    pub stats: CgroupStats,
    pub enabled_controllers: u32, // Bitmask of CgroupController
    pub children: Vec<CgroupId>,
    pub processes: Vec<u32>, // PIDs
}

impl Cgroup {
    pub fn new(id: CgroupId, name: String, parent_id: Option<CgroupId>) -> Self {
        Cgroup {
            id,
            name,
            parent_id,
            state: AtomicU32::new(CgroupState::Active as u32),
            limits: CgroupLimits::default(),
            stats: CgroupStats::default(),
            enabled_controllers: 0,
            children: Vec::new(),
            processes: Vec::new(),
        }
    }

    pub fn get_state(&self) -> CgroupState {
        match self.state.load(Ordering::SeqCst) {
            0 => CgroupState::Active,
            1 => CgroupState::Frozen,
            2 => CgroupState::Dying,
            _ => CgroupState::Active,
        }
    }

    pub fn set_state(&self, state: CgroupState) {
        self.state.store(state as u32, Ordering::SeqCst);
    }

    pub fn is_frozen(&self) -> bool {
        self.get_state() == CgroupState::Frozen
    }

    pub fn enable_controller(&mut self, controller: CgroupController) {
        self.enabled_controllers |= 1 << (controller as u32);
    }

    pub fn is_controller_enabled(&self, controller: CgroupController) -> bool {
        (self.enabled_controllers & (1 << (controller as u32))) != 0
    }

    pub fn add_process(&mut self, pid: u32) {
        self.processes.push(pid);
        self.stats.current_pids.fetch_add(1, Ordering::SeqCst);
    }

    pub fn remove_process(&mut self, pid: u32) {
        if let Some(pos) = self.processes.iter().position(|&p| p == pid) {
            self.processes.remove(pos);
            self.stats.current_pids.fetch_sub(1, Ordering::SeqCst);
        }
    }

    pub fn update_cpu_usage(&self, delta_ns: u64) {
        self.stats.cpu_usage_ns.fetch_add(delta_ns, Ordering::SeqCst);
    }

    pub fn update_memory_usage(&self, delta_bytes: i64) {
        let current = self.stats.memory_usage_bytes.load(Ordering::SeqCst) as i64;
        let new = (current + delta_bytes).max(0) as u64;
        self.stats.memory_usage_bytes.store(new, Ordering::SeqCst);
    }
}

/// Cgroup subsystem
#[derive(Debug)]
pub struct CgroupSubsystem {
    cgroups: BTreeMap<CgroupId, Cgroup>,
    next_cgroup_id: AtomicU64,
    root_cgroup_id: CgroupId,
}

impl CgroupSubsystem {
    pub fn new() -> Self {
        let root_id = 1;
        let mut cgroups = BTreeMap::new();
        
        let root = Cgroup::new(root_id, "/".to_string(), None);
        cgroups.insert(root_id, root);

        CgroupSubsystem {
            cgroups,
            next_cgroup_id: AtomicU64::new(2),
            root_cgroup_id: root_id,
        }
    }

    /// Create a new cgroup
    pub fn create_cgroup(&mut self, name: String, parent_id: Option<CgroupId>) -> Result<CgroupId, &'static str> {
        let parent = match parent_id {
            Some(id) => self.cgroups.get(&id).ok_or("Parent cgroup not found")?,
            None => self.cgroups.get(&self.root_cgroup_id).ok_or("Root cgroup not found")?,
        };

        let id = self.next_cgroup_id.fetch_add(1, Ordering::SeqCst);
        let cgroup = Cgroup::new(id, name, parent_id.or(Some(self.root_cgroup_id)));
        
        if let Some(pid) = parent_id {
            if let Some(parent) = self.cgroups.get_mut(&pid) {
                parent.children.push(id);
            }
        }

        self.cgroups.insert(id, cgroup);
        Ok(id)
    }

    /// Get cgroup by ID
    pub fn get_cgroup(&self, id: CgroupId) -> Option<&Cgroup> {
        self.cgroups.get(&id)
    }

    /// Get mutable cgroup by ID
    pub fn get_cgroup_mut(&mut self, id: CgroupId) -> Option<&mut Cgroup> {
        self.cgroups.get_mut(&id)
    }

    /// Delete a cgroup
    pub fn delete_cgroup(&mut self, id: CgroupId) -> Result<(), &'static str> {
        if id == self.root_cgroup_id {
            return Err("Cannot delete root cgroup");
        }

        let cgroup = self.cgroups.get(&id).ok_or("Cgroup not found")?;
        
        if !cgroup.children.is_empty() {
            return Err("Cannot delete cgroup with children");
        }

        if !cgroup.processes.is_empty() {
            return Err("Cannot delete cgroup with processes");
        }

        if let Some(parent_id) = cgroup.parent_id {
            if let Some(parent) = self.cgroups.get_mut(&parent_id) {
                parent.children.retain(|&child_id| child_id != id);
            }
        }

        self.cgroups.remove(&id);
        Ok(())
    }

    /// Add process to cgroup
    pub fn add_process_to_cgroup(&mut self, cgroup_id: CgroupId, pid: u32) -> Result<(), &'static str> {
        let cgroup = self.cgroups.get_mut(&cgroup_id).ok_or("Cgroup not found")?;
        cgroup.add_process(pid);
        Ok(())
    }

    /// Remove process from cgroup
    pub fn remove_process_from_cgroup(&mut self, cgroup_id: CgroupId, pid: u32) -> Result<(), &'static str> {
        let cgroup = self.cgroups.get_mut(&cgroup_id).ok_or("Cgroup not found")?;
        cgroup.remove_process(pid);
        Ok(())
    }

    /// Get cgroup count
    pub fn cgroup_count(&self) -> usize {
        self.cgroups.len()
    }

    /// Get root cgroup ID
    pub fn root_cgroup_id(&self) -> CgroupId {
        self.root_cgroup_id
    }
}

impl Default for CgroupSubsystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cgroup_creation() {
        let mut subsystem = CgroupSubsystem::new();
        
        let id = subsystem.create_cgroup("test".to_string(), None).unwrap();
        assert!(id > 1);
        assert_eq!(subsystem.cgroup_count(), 2);
    }

    #[test]
    fn test_cgroup_hierarchy() {
        let mut subsystem = CgroupSubsystem::new();
        
        let parent_id = subsystem.create_cgroup("parent".to_string(), None).unwrap();
        let child_id = subsystem.create_cgroup("child".to_string(), Some(parent_id)).unwrap();
        
        let parent = subsystem.get_cgroup(parent_id).unwrap();
        assert!(parent.children.contains(&child_id));
    }

    #[test]
    fn test_cgroup_process_management() {
        let mut subsystem = CgroupSubsystem::new();
        
        let id = subsystem.create_cgroup("test".to_string(), None).unwrap();
        subsystem.add_process_to_cgroup(id, 1234).unwrap();
        
        let cgroup = subsystem.get_cgroup(id).unwrap();
        assert!(cgroup.processes.contains(&1234));
        assert_eq!(cgroup.stats.current_pids.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_cgroup_deletion() {
        let mut subsystem = CgroupSubsystem::new();
        
        let id = subsystem.create_cgroup("test".to_string(), None).unwrap();
        subsystem.delete_cgroup(id).unwrap();
        
        assert_eq!(subsystem.cgroup_count(), 1);
    }

    #[test]
    fn test_cgroup_controller_enablement() {
        let mut subsystem = CgroupSubsystem::new();
        
        let id = subsystem.create_cgroup("test".to_string(), None).unwrap();
        let cgroup = subsystem.get_cgroup_mut(id).unwrap();
        
        cgroup.enable_controller(CgroupController::Cpu);
        assert!(cgroup.is_controller_enabled(CgroupController::Cpu));
    }

    #[test]
    fn test_cgroup_freeze() {
        let mut subsystem = CgroupSubsystem::new();
        
        let id = subsystem.create_cgroup("test".to_string(), None).unwrap();
        let cgroup = subsystem.get_cgroup(id).unwrap();
        
        cgroup.set_state(CgroupState::Frozen);
        assert!(cgroup.is_frozen());
    }
}
