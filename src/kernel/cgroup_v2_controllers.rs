// Linux-inspired cgroup v2 Controller Support
// Resource control and process grouping for SigmaOS

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// cgroup v2 controller types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CgroupController {
    /// CPU controller
    Cpu = 0x00000001,
    /// Memory controller
    Memory = 0x00000002,
    /// IO controller
    Io = 0x00000004,
    /// PIDs controller
    Pids = 0x00000008,
    /// Cpuset controller
    Cpuset = 0x00000010,
    /// Freezer controller
    Freezer = 0x00000020,
    /// RDMA controller
    Rdma = 0x00000040,
    /// Hugetlb controller
    Hugetlb = 0x00000080,
}

impl CgroupController {
    pub fn from_bits(bits: u32) -> Vec<Self> {
        let mut controllers = Vec::new();

        if bits & (CgroupController::Cpu as u32) != 0 {
            controllers.push(CgroupController::Cpu);
        }
        if bits & (CgroupController::Memory as u32) != 0 {
            controllers.push(CgroupController::Memory);
        }
        if bits & (CgroupController::Io as u32) != 0 {
            controllers.push(CgroupController::Io);
        }
        if bits & (CgroupController::Pids as u32) != 0 {
            controllers.push(CgroupController::Pids);
        }
        if bits & (CgroupController::Cpuset as u32) != 0 {
            controllers.push(CgroupController::Cpuset);
        }
        if bits & (CgroupController::Freezer as u32) != 0 {
            controllers.push(CgroupController::Freezer);
        }
        if bits & (CgroupController::Rdma as u32) != 0 {
            controllers.push(CgroupController::Rdma);
        }
        if bits & (CgroupController::Hugetlb as u32) != 0 {
            controllers.push(CgroupController::Hugetlb);
        }

        controllers
    }
}

/// cgroup v2 statistics
#[derive(Debug, Clone)]
pub struct CgroupStats {
    pub cpu_usage_nanos: u64,
    pub memory_usage_bytes: u64,
    pub memory_limit_bytes: u64,
    pub memory_peak_bytes: u64,
    pub pids_current: u64,
    pub pids_max: u64,
    pub io_read_bytes: u64,
    pub io_write_bytes: u64,
}

impl CgroupStats {
    pub fn new() -> Self {
        CgroupStats {
            cpu_usage_nanos: 0,
            memory_usage_bytes: 0,
            memory_limit_bytes: u64::MAX,
            memory_peak_bytes: 0,
            pids_current: 0,
            pids_max: u64::MAX,
            io_read_bytes: 0,
            io_write_bytes: 0,
        }
    }
}

impl Default for CgroupStats {
    fn default() -> Self {
        Self::new()
    }
}

/// cgroup v2 controller configuration
#[derive(Debug, Clone)]
pub struct CgroupControllerConfig {
    pub controller: CgroupController,
    pub enabled: bool,
    pub params: HashMap<String, String>,
}

impl CgroupControllerConfig {
    pub fn new(controller: CgroupController) -> Self {
        CgroupControllerConfig {
            controller,
            enabled: true,
            params: HashMap::new(),
        }
    }

    pub fn set_param(&mut self, key: String, value: String) {
        self.params.insert(key, value);
    }

    pub fn get_param(&self, key: &str) -> Option<&String> {
        self.params.get(key)
    }
}

/// cgroup v2 instance
#[derive(Debug, Clone)]
pub struct CgroupV2 {
    pub name: String,
    pub parent: Option<Arc<Mutex<CgroupV2>>>,
    pub controllers: HashMap<CgroupController, CgroupControllerConfig>,
    pub processes: Vec<u32>,
    pub stats: CgroupStats,
}

impl CgroupV2 {
    pub fn new(name: String) -> Self {
        CgroupV2 {
            name,
            parent: None,
            controllers: HashMap::new(),
            processes: Vec::new(),
            stats: CgroupStats::new(),
        }
    }

    pub fn with_parent(name: String, parent: Arc<Mutex<CgroupV2>>) -> Self {
        CgroupV2 {
            name,
            parent: Some(parent),
            controllers: HashMap::new(),
            processes: Vec::new(),
            stats: CgroupStats::new(),
        }
    }

    pub fn add_controller(&mut self, controller: CgroupController) {
        let config = CgroupControllerConfig::new(controller);
        self.controllers.insert(controller, config);
    }

    pub fn remove_controller(&mut self, controller: CgroupController) {
        self.controllers.remove(&controller);
    }

    pub fn add_process(&mut self, pid: u32) {
        self.processes.push(pid);
    }

    pub fn remove_process(&mut self, pid: u32) {
        self.processes.retain(|&p| p != pid);
    }

    pub fn process_count(&self) -> usize {
        self.processes.len()
    }

    pub fn get_controller_config(&self, controller: CgroupController) -> Option<&CgroupControllerConfig> {
        self.controllers.get(&controller)
    }

    pub fn set_controller_param(&mut self, controller: CgroupController, key: String, value: String) {
        if let Some(config) = self.controllers.get_mut(&controller) {
            config.set_param(key, value);
        }
    }
}

/// cgroup v2 manager
pub struct CgroupV2Manager {
    cgroups: HashMap<String, Arc<Mutex<CgroupV2>>>,
    root: Arc<Mutex<CgroupV2>>,
}

impl CgroupV2Manager {
    pub fn new() -> Self {
        let root = Arc::new(Mutex::new(CgroupV2::new("/".to_string())));
        let mut manager = CgroupV2Manager {
            cgroups: HashMap::new(),
            root: root.clone(),
        };
        manager.cgroups.insert("/".to_string(), root);
        manager
    }

    pub fn create_cgroup(&mut self, name: String, parent: Option<Arc<Mutex<CgroupV2>>>) -> Result<Arc<Mutex<CgroupV2>>, String> {
        if self.cgroups.contains_key(&name) {
            return Err(format!("cgroup already exists: {}", name));
        }

        let cgroup = match parent {
            Some(p) => CgroupV2::with_parent(name.clone(), p),
            None => CgroupV2::new(name.clone()),
        };

        let cg = Arc::new(Mutex::new(cgroup));
        self.cgroups.insert(name, cg.clone());
        Ok(cg)
    }

    pub fn get_cgroup(&self, name: &str) -> Option<Arc<Mutex<CgroupV2>>> {
        self.cgroups.get(name).cloned()
    }

    pub fn remove_cgroup(&mut self, name: &str) -> Result<(), String> {
        let cg = self.cgroups.get(name)
            .ok_or_else(|| format!("cgroup not found: {}", name))?;

        let process_count = cg.lock().unwrap().process_count();
        if process_count > 0 {
            return Err(format!("cgroup has {} processes, cannot remove", process_count));
        }

        self.cgroups.remove(name);
        Ok(())
    }

    pub fn get_root(&self) -> Arc<Mutex<CgroupV2>> {
        self.root.clone()
    }

    pub fn cgroup_count(&self) -> usize {
        self.cgroups.len()
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
    fn test_cgroup_controller_from_bits() {
        let bits = (CgroupController::Cpu as u32) | (CgroupController::Memory as u32);
        let controllers = CgroupController::from_bits(bits);

        assert_eq!(controllers.len(), 2);
        assert!(controllers.contains(&CgroupController::Cpu));
        assert!(controllers.contains(&CgroupController::Memory));
    }

    #[test]
    fn test_cgroup_stats() {
        let stats = CgroupStats::new();
        assert_eq!(stats.cpu_usage_nanos, 0);
        assert_eq!(stats.memory_usage_bytes, 0);
    }

    #[test]
    fn test_cgroup_controller_config() {
        let mut config = CgroupControllerConfig::new(CgroupController::Cpu);
        config.set_param("cpu.max".to_string(), "100000".to_string());

        assert_eq!(config.get_param("cpu.max"), Some(&"100000".to_string()));
    }

    #[test]
    fn test_cgroup_creation() {
        let cgroup = CgroupV2::new("/test".to_string());
        assert_eq!(cgroup.name, "/test");
        assert!(cgroup.parent.is_none());
    }

    #[test]
    fn test_cgroup_with_parent() {
        let parent = Arc::new(Mutex::new(CgroupV2::new("/".to_string())));
        let child = CgroupV2::with_parent("/test".to_string(), parent.clone());

        assert!(child.parent.is_some());
    }

    #[test]
    fn test_cgroup_add_remove_controller() {
        let mut cgroup = CgroupV2::new("/test".to_string());

        cgroup.add_controller(CgroupController::Cpu);
        assert!(cgroup.get_controller_config(CgroupController::Cpu).is_some());

        cgroup.remove_controller(CgroupController::Cpu);
        assert!(cgroup.get_controller_config(CgroupController::Cpu).is_none());
    }

    #[test]
    fn test_cgroup_add_remove_process() {
        let mut cgroup = CgroupV2::new("/test".to_string());

        cgroup.add_process(100);
        assert_eq!(cgroup.process_count(), 1);

        cgroup.remove_process(100);
        assert_eq!(cgroup.process_count(), 0);
    }

    #[test]
    fn test_cgroup_manager_create() {
        let mut manager = CgroupV2Manager::new();

        let _cg = manager.create_cgroup("/test".to_string(), None).unwrap();
        assert_eq!(manager.cgroup_count(), 2); // root + test
    }

    #[test]
    fn test_cgroup_manager_remove() {
        let mut manager = CgroupV2Manager::new();

        let _cg = manager.create_cgroup("/test".to_string(), None).unwrap();
        manager.remove_cgroup("/test").unwrap();

        assert_eq!(manager.cgroup_count(), 1); // only root
    }

    #[test]
    fn test_cgroup_manager_remove_with_processes() {
        let mut manager = CgroupV2Manager::new();

        let cg = manager.create_cgroup("/test".to_string(), None).unwrap();
        {
            let mut cg_guard = cg.lock().unwrap();
            cg_guard.add_process(100);
        }

        let result = manager.remove_cgroup("/test");
        assert!(result.is_err());
    }
}
