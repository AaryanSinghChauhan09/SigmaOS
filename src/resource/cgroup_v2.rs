// Cgroup v2 - Control Groups v2 for Resource Control
// Inspired by Linux cgroup v2 for unified resource management

use std::collections::HashMap;
use std::sync::atomic::AtomicU64;

/// Cgroup controller type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CgroupController {
    Memory,
    Cpu,
    Cpuset,
    Io,
    Pids,
    Rdma,
    HugeTlb,
    Cpuacct,
    Devices,
    Freezer,
    NetCls,
    NetPrio,
    PerfEvent,
}

/// Memory controller settings
#[derive(Debug)]
pub struct MemoryController {
    pub limit: u64,        // Memory limit in bytes
    pub swap_limit: u64,   // Swap limit in bytes
    pub oom_control: bool, // OOM killer control
    pub usage: AtomicU64,  // Current memory usage
}

/// CPU controller settings
#[derive(Debug)]
pub struct CpuController {
    pub shares: u64,      // CPU shares (weight)
    pub max: Option<u64>, // Maximum CPU time (quota)
    pub period: u64,      // Period in microseconds
    pub rt_runtime: u64,  // Realtime runtime
}

/// IO controller settings
#[derive(Debug, Clone)]
pub struct IoController {
    pub weight: u16,      // IO weight
    pub max: Option<u64>, // Maximum IO rate
    pub read_bps: u64,    // Read bytes per second
    pub write_bps: u64,   // Write bytes per second
}

/// PIDs controller settings
#[derive(Debug)]
pub struct PidsController {
    pub max: u64,           // Maximum number of PIDs
    pub current: AtomicU64, // Current number of PIDs
}

/// Cgroup v2
pub struct CgroupV2 {
    pub name: String,
    pub parent: Option<String>,
    pub children: Vec<String>,
    pub controllers: HashMap<CgroupController, Box<dyn CgroupControllerTrait>>,
    pub processes: Vec<u64>,
    pub enabled: bool,
}

/// Trait for cgroup controllers
pub trait CgroupControllerTrait {
    fn as_any(&self) -> &dyn std::any::Any;
    fn name(&self) -> &str;
}

impl CgroupControllerTrait for MemoryController {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn name(&self) -> &str {
        "memory"
    }
}

impl CgroupControllerTrait for CpuController {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn name(&self) -> &str {
        "cpu"
    }
}

impl CgroupControllerTrait for IoController {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn name(&self) -> &str {
        "io"
    }
}

impl CgroupControllerTrait for PidsController {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn name(&self) -> &str {
        "pids"
    }
}

impl CgroupV2 {
    pub fn new(name: String, parent: Option<String>) -> Self {
        Self {
            name,
            parent,
            children: Vec::new(),
            controllers: HashMap::new(),
            processes: Vec::new(),
            enabled: true,
        }
    }

    /// Add a child cgroup
    pub fn add_child(&mut self, child_name: String) {
        self.children.push(child_name);
    }

    /// Remove a child cgroup
    pub fn remove_child(&mut self, child_name: &str) -> bool {
        if let Some(pos) = self.children.iter().position(|c| c == child_name) {
            self.children.remove(pos);
            true
        } else {
            false
        }
    }

    /// Add a controller
    pub fn add_controller(
        &mut self,
        controller: CgroupController,
        trait_obj: Box<dyn CgroupControllerTrait>,
    ) {
        self.controllers.insert(controller, trait_obj);
    }

    /// Remove a controller
    pub fn remove_controller(&mut self, controller: CgroupController) -> bool {
        self.controllers.remove(&controller).is_some()
    }

    /// Add a process to the cgroup
    pub fn add_process(&mut self, pid: u64) {
        self.processes.push(pid);
    }

    /// Remove a process from the cgroup
    pub fn remove_process(&mut self, pid: u64) -> bool {
        if let Some(pos) = self.processes.iter().position(|&p| p == pid) {
            self.processes.remove(pos);
            true
        } else {
            false
        }
    }

    /// Get controller by type
    pub fn get_controller(
        &self,
        controller: CgroupController,
    ) -> Option<&dyn CgroupControllerTrait> {
        self.controllers.get(&controller).map(|c| c.as_ref())
    }

    /// Get process count
    pub fn process_count(&self) -> usize {
        self.processes.len()
    }

    /// Get child count
    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    /// Enable cgroup
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable cgroup
    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

/// Cgroup v2 manager
pub struct CgroupV2Manager {
    cgroups: HashMap<String, CgroupV2>,
    root_cgroup: String,
}

impl CgroupV2Manager {
    pub fn new() -> Self {
        let mut manager = Self {
            cgroups: HashMap::new(),
            root_cgroup: "/".to_string(),
        };

        // Create root cgroup
        let root = CgroupV2::new("/".to_string(), None);
        manager.cgroups.insert("/".to_string(), root);

        manager
    }

    /// Create a cgroup
    pub fn create_cgroup(
        &mut self,
        name: String,
        parent: Option<String>,
    ) -> Result<(), &'static str> {
        if self.cgroups.contains_key(&name) {
            return Err("Cgroup already exists");
        }

        if let Some(ref parent_name) = parent {
            if !self.cgroups.contains_key(parent_name) {
                return Err("Parent cgroup not found");
            }

            if let Some(parent_cgroup) = self.cgroups.get_mut(parent_name) {
                parent_cgroup.add_child(name.clone());
            }
        }

        let cgroup = CgroupV2::new(name.clone(), parent);
        self.cgroups.insert(name, cgroup);

        Ok(())
    }

    /// Delete a cgroup
    pub fn delete_cgroup(&mut self, name: &str) -> Result<(), &'static str> {
        if name == self.root_cgroup {
            return Err("Cannot delete root cgroup");
        }

        let cgroup = self.cgroups.remove(name).ok_or("Cgroup not found")?;

        if let Some(ref parent_name) = cgroup.parent {
            if let Some(parent_cgroup) = self.cgroups.get_mut(parent_name) {
                parent_cgroup.remove_child(name);
            }
        }

        Ok(())
    }

    /// Get a cgroup
    pub fn get_cgroup(&self, name: &str) -> Option<&CgroupV2> {
        self.cgroups.get(name)
    }

    /// Get mutable cgroup
    pub fn get_cgroup_mut(&mut self, name: &str) -> Option<&mut CgroupV2> {
        self.cgroups.get_mut(name)
    }

    /// Add memory controller to cgroup
    pub fn add_memory_controller(
        &mut self,
        cgroup_name: &str,
        limit: u64,
        swap_limit: u64,
    ) -> Result<(), &'static str> {
        let cgroup = self
            .cgroups
            .get_mut(cgroup_name)
            .ok_or("Cgroup not found")?;

        let memory_controller = MemoryController {
            limit,
            swap_limit,
            oom_control: true,
            usage: AtomicU64::new(0),
        };

        cgroup.add_controller(CgroupController::Memory, Box::new(memory_controller));
        Ok(())
    }

    /// Add CPU controller to cgroup
    pub fn add_cpu_controller(
        &mut self,
        cgroup_name: &str,
        shares: u64,
        max: Option<u64>,
    ) -> Result<(), &'static str> {
        let cgroup = self
            .cgroups
            .get_mut(cgroup_name)
            .ok_or("Cgroup not found")?;

        let cpu_controller = CpuController {
            shares,
            max,
            period: 100000, // 100ms default
            rt_runtime: 0,
        };

        cgroup.add_controller(CgroupController::Cpu, Box::new(cpu_controller));
        Ok(())
    }

    /// Add PIDs controller to cgroup
    pub fn add_pids_controller(&mut self, cgroup_name: &str, max: u64) -> Result<(), &'static str> {
        let cgroup = self
            .cgroups
            .get_mut(cgroup_name)
            .ok_or("Cgroup not found")?;

        let pids_controller = PidsController {
            max,
            current: AtomicU64::new(0),
        };

        cgroup.add_controller(CgroupController::Pids, Box::new(pids_controller));
        Ok(())
    }

    /// Move process to cgroup
    pub fn move_process(&mut self, cgroup_name: &str, pid: u64) -> Result<(), &'static str> {
        let cgroup = self
            .cgroups
            .get_mut(cgroup_name)
            .ok_or("Cgroup not found")?;

        cgroup.add_process(pid);
        Ok(())
    }

    /// Get cgroup count
    pub fn cgroup_count(&self) -> usize {
        self.cgroups.len()
    }

    /// List all cgroups
    pub fn list_cgroups(&self) -> Vec<String> {
        self.cgroups.keys().cloned().collect()
    }
}

impl Default for CgroupV2Manager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_cgroup() {
        let mut manager = CgroupV2Manager::new();

        assert!(manager
            .create_cgroup("/test".to_string(), Some("/".to_string()))
            .is_ok());
        assert_eq!(manager.cgroup_count(), 2);
    }

    #[test]
    fn test_delete_cgroup() {
        let mut manager = CgroupV2Manager::new();

        manager
            .create_cgroup("/test".to_string(), Some("/".to_string()))
            .unwrap();
        assert!(manager.delete_cgroup("/test").is_ok());
        assert_eq!(manager.cgroup_count(), 1);
    }

    #[test]
    fn test_delete_root() {
        let mut manager = CgroupV2Manager::new();

        assert!(manager.delete_cgroup("/").is_err());
    }

    #[test]
    fn test_add_memory_controller() {
        let mut manager = CgroupV2Manager::new();

        manager
            .create_cgroup("/test".to_string(), Some("/".to_string()))
            .unwrap();
        assert!(manager
            .add_memory_controller("/test", 1024 * 1024 * 1024, 512 * 1024 * 1024)
            .is_ok());
    }

    #[test]
    fn test_add_cpu_controller() {
        let mut manager = CgroupV2Manager::new();

        manager
            .create_cgroup("/test".to_string(), Some("/".to_string()))
            .unwrap();
        assert!(manager
            .add_cpu_controller("/test", 1024, Some(500000))
            .is_ok());
    }

    #[test]
    fn test_add_pids_controller() {
        let mut manager = CgroupV2Manager::new();

        manager
            .create_cgroup("/test".to_string(), Some("/".to_string()))
            .unwrap();
        assert!(manager.add_pids_controller("/test", 100).is_ok());
    }

    #[test]
    fn test_move_process() {
        let mut manager = CgroupV2Manager::new();

        manager
            .create_cgroup("/test".to_string(), Some("/".to_string()))
            .unwrap();
        assert!(manager.move_process("/test", 1234).is_ok());

        let cgroup = manager.get_cgroup("/test").unwrap();
        assert_eq!(cgroup.process_count(), 1);
    }

    #[test]
    fn test_enable_disable() {
        let mut manager = CgroupV2Manager::new();

        manager
            .create_cgroup("/test".to_string(), Some("/".to_string()))
            .unwrap();

        let cgroup = manager.get_cgroup_mut("/test").unwrap();
        cgroup.disable();
        assert!(!cgroup.enabled);

        cgroup.enable();
        assert!(cgroup.enabled);
    }
}
