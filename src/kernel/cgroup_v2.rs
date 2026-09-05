// cgroups v2 Framework for SigmaOS (Fixed version using enum instead of trait objects)

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::path::PathBuf;

/// cgroup identifier
pub type CgroupId = u64;

/// cgroup controller type
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControllerType {
    /// CPU time allocation
    Cpu = 1,
    /// Memory management
    Memory = 2,
    /// I/O operations
    Io = 3,
    /// Process count limiting
    Pids = 4,
}

impl ControllerType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ControllerType::Cpu => "cpu",
            ControllerType::Memory => "memory",
            ControllerType::Io => "io",
            ControllerType::Pids => "pids",
        }
    }
}

/// cgroup state
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CgroupState {
    /// Active, normal operation
    Active = 1,
    /// Frozen, all processes paused
    Frozen = 2,
    /// Destroying, pending cleanup
    Destroying = 3,
}

/// CPU controller configuration
#[derive(Debug, Clone)]
pub struct CpuController {
    /// CPU quota (microseconds)
    pub cpu_quota_us: u64,
    /// CPU period (microseconds)
    pub cpu_period_us: u64,
    /// CPU shares (weight)
    pub cpu_shares: u64,
}

impl CpuController {
    pub fn new() -> Self {
        CpuController {
            cpu_quota_us: 1_000_000,
            cpu_period_us: 100_000,
            cpu_shares: 1024,
        }
    }
}

impl Default for CpuController {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory controller configuration
#[derive(Debug, Clone)]
pub struct MemoryController {
    /// Memory limit in bytes
    pub memory_limit: u64,
    /// Memory soft limit
    pub memory_soft_limit: u64,
    /// High threshold for reclaim
    pub memory_high: u64,
}

impl MemoryController {
    pub fn new() -> Self {
        MemoryController {
            memory_limit: u64::MAX,
            memory_soft_limit: u64::MAX,
            memory_high: u64::MAX,
        }
    }
}

impl Default for MemoryController {
    fn default() -> Self {
        Self::new()
    }
}

/// Process ID controller
#[derive(Debug, Clone)]
pub struct PidsController {
    /// Maximum number of processes
    pub pids_max: u64,
    /// Current process count
    pub pids_current: u64,
}

impl PidsController {
    pub fn new() -> Self {
        PidsController {
            pids_max: 4096,
            pids_current: 0,
        }
    }
}

impl Default for PidsController {
    fn default() -> Self {
        Self::new()
    }
}

/// I/O controller
#[derive(Debug, Clone)]
pub struct IoController {
    /// I/O bandwidth limit (bytes/sec)
    pub io_bandwidth_limit: u64,
    /// I/O IOPS limit
    pub io_iops_limit: u64,
}

impl IoController {
    pub fn new() -> Self {
        IoController {
            io_bandwidth_limit: u64::MAX,
            io_iops_limit: u64::MAX,
        }
    }
}

impl Default for IoController {
    fn default() -> Self {
        Self::new()
    }
}

/// Generic controller enum
#[derive(Debug, Clone)]
pub enum Controller {
    Cpu(CpuController),
    Memory(MemoryController),
    Pids(PidsController),
    Io(IoController),
}

impl Controller {
    pub fn controller_type(&self) -> ControllerType {
        match self {
            Controller::Cpu(_) => ControllerType::Cpu,
            Controller::Memory(_) => ControllerType::Memory,
            Controller::Pids(_) => ControllerType::Pids,
            Controller::Io(_) => ControllerType::Io,
        }
    }

    pub fn set_limit(&mut self, key: &str, value: u64) -> Result<(), String> {
        match self {
            Controller::Cpu(cpu) => {
                match key {
                    "cpu.max" => cpu.cpu_quota_us = value,
                    "cpu.weight" => cpu.cpu_shares = value,
                    "cpu.period_us" => cpu.cpu_period_us = value,
                    _ => return Err(format!("Unknown CPU limit key: {}", key)),
                }
            }
            Controller::Memory(mem) => {
                match key {
                    "memory.max" => mem.memory_limit = value,
                    "memory.soft_limit_in_bytes" => mem.memory_soft_limit = value,
                    "memory.high" => mem.memory_high = value,
                    _ => return Err(format!("Unknown memory limit key: {}", key)),
                }
            }
            Controller::Pids(pids) => {
                match key {
                    "pids.max" => pids.pids_max = value,
                    _ => return Err(format!("Unknown pids limit key: {}", key)),
                }
            }
            Controller::Io(io) => {
                match key {
                    "io.bandwidth" => io.io_bandwidth_limit = value,
                    "io.iops" => io.io_iops_limit = value,
                    _ => return Err(format!("Unknown io limit key: {}", key)),
                }
            }
        }
        Ok(())
    }

    pub fn get_limit(&self, key: &str) -> Result<u64, String> {
        match self {
            Controller::Cpu(cpu) => {
                match key {
                    "cpu.max" => Ok(cpu.cpu_quota_us),
                    "cpu.weight" => Ok(cpu.cpu_shares),
                    "cpu.period_us" => Ok(cpu.cpu_period_us),
                    _ => Err(format!("Unknown CPU limit key: {}", key)),
                }
            }
            Controller::Memory(mem) => {
                match key {
                    "memory.max" => Ok(mem.memory_limit),
                    "memory.soft_limit_in_bytes" => Ok(mem.memory_soft_limit),
                    "memory.high" => Ok(mem.memory_high),
                    _ => Err(format!("Unknown memory limit key: {}", key)),
                }
            }
            Controller::Pids(pids) => {
                match key {
                    "pids.max" => Ok(pids.pids_max),
                    "pids.current" => Ok(pids.pids_current),
                    _ => Err(format!("Unknown pids limit key: {}", key)),
                }
            }
            Controller::Io(io) => {
                match key {
                    "io.bandwidth" => Ok(io.io_bandwidth_limit),
                    "io.iops" => Ok(io.io_iops_limit),
                    _ => Err(format!("Unknown io limit key: {}", key)),
                }
            }
        }
    }
}

/// cgroup representation
#[derive(Debug)]
pub struct Cgroup {
    /// Unique cgroup identifier
    pub id: CgroupId,
    /// cgroup name/path
    pub path: PathBuf,
    /// Parent cgroup ID
    pub parent_id: Option<CgroupId>,
    /// Child cgroup IDs
    pub children: HashSet<CgroupId>,
    /// Process IDs in this cgroup
    pub processes: HashSet<u32>,
    /// Controllers
    pub controllers: HashMap<ControllerType, Controller>,
    /// Current state
    pub state: CgroupState,
    /// Creation timestamp
    pub created_at: u64,
}

impl Cgroup {
    /// Create new cgroup
    pub fn new(id: CgroupId, path: PathBuf, parent_id: Option<CgroupId>) -> Self {
        let created_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        Cgroup {
            id,
            path,
            parent_id,
            children: HashSet::new(),
            processes: HashSet::new(),
            controllers: HashMap::new(),
            state: CgroupState::Active,
            created_at,
        }
    }

    /// Add child cgroup
    pub fn add_child(&mut self, child_id: CgroupId) {
        self.children.insert(child_id);
    }

    /// Remove child cgroup
    pub fn remove_child(&mut self, child_id: CgroupId) {
        self.children.remove(&child_id);
    }

    /// Add process to cgroup
    pub fn add_process(&mut self, process_id: u32) -> Result<(), String> {
        if self.state != CgroupState::Active {
            return Err("cgroup is not active".to_string());
        }
        self.processes.insert(process_id);
        Ok(())
    }

    /// Remove process from cgroup
    pub fn remove_process(&mut self, process_id: u32) -> Result<(), String> {
        self.processes.remove(&process_id);
        Ok(())
    }

    /// Register controller
    pub fn register_controller(&mut self, controller: Controller) -> Result<(), String> {
        let controller_type = controller.controller_type();
        self.controllers.insert(controller_type, controller);
        Ok(())
    }

    /// Unregister controller
    pub fn unregister_controller(&mut self, controller_type: ControllerType) -> Result<(), String> {
        self.controllers.remove(&controller_type);
        Ok(())
    }

    /// Set controller limit
    pub fn set_controller_limit(
        &mut self,
        controller_type: ControllerType,
        key: &str,
        value: u64,
    ) -> Result<(), String> {
        if let Some(controller) = self.controllers.get_mut(&controller_type) {
            controller.set_limit(key, value)?;
            Ok(())
        } else {
            Err(format!("Controller {:?} not registered", controller_type))
        }
    }

    /// Get controller limit
    pub fn get_controller_limit(
        &self,
        controller_type: ControllerType,
        key: &str,
    ) -> Result<u64, String> {
        if let Some(controller) = self.controllers.get(&controller_type) {
            controller.get_limit(key)
        } else {
            Err(format!("Controller {:?} not registered", controller_type))
        }
    }

    /// Freeze cgroup (pause all processes)
    pub fn freeze(&mut self) -> Result<(), String> {
        self.state = CgroupState::Frozen;
        Ok(())
    }

    /// Thaw cgroup (resume all processes)
    pub fn thaw(&mut self) -> Result<(), String> {
        self.state = CgroupState::Active;
        Ok(())
    }

    /// Get process count
    pub fn process_count(&self) -> usize {
        self.processes.len()
    }

    /// Get child count
    pub fn child_count(&self) -> usize {
        self.children.len()
    }
}

/// cgroup hierarchy manager
pub struct CgroupHierarchy {
    /// Root cgroup
    root: Arc<Mutex<Cgroup>>,
    /// All cgroups by ID
    cgroups: Arc<Mutex<HashMap<CgroupId, Arc<Mutex<Cgroup>>>>>,
    /// Next cgroup ID
    next_id: Arc<Mutex<CgroupId>>,
}

impl CgroupHierarchy {
    /// Create new cgroup hierarchy
    pub fn new() -> Result<Self, String> {
        let root = Arc::new(Mutex::new(Cgroup::new(1, PathBuf::from("/"), None)));
        let mut cgroups = HashMap::new();
        cgroups.insert(1, Arc::clone(&root));

        Ok(CgroupHierarchy {
            root,
            cgroups: Arc::new(Mutex::new(cgroups)),
            next_id: Arc::new(Mutex::new(2)),
        })
    }

    /// Create child cgroup
    pub fn create_cgroup(&self, path: PathBuf, parent_id: Option<CgroupId>) -> Result<CgroupId, String> {
        let mut id_guard = self
            .next_id
            .lock()
            .map_err(|_| "Failed to acquire ID lock".to_string())?;
        let cgroup_id = *id_guard;
        *id_guard = id_guard.wrapping_add(1);
        drop(id_guard);

        let cgroup = Arc::new(Mutex::new(Cgroup::new(cgroup_id, path, parent_id)));

        let mut cgroups = self
            .cgroups
            .lock()
            .map_err(|_| "Failed to acquire cgroups lock".to_string())?;

        if let Some(parent_id) = parent_id {
            if let Some(parent) = cgroups.get(&parent_id) {
                let mut parent_guard = parent
                    .lock()
                    .map_err(|_| "Failed to acquire parent lock".to_string())?;
                parent_guard.add_child(cgroup_id);
            } else {
                return Err(format!("Parent cgroup {} not found", parent_id));
            }
        }

        cgroups.insert(cgroup_id, cgroup);
        Ok(cgroup_id)
    }

    /// Remove cgroup
    pub fn remove_cgroup(&self, cgroup_id: CgroupId) -> Result<(), String> {
        if cgroup_id == 1 {
            return Err("Cannot remove root cgroup".to_string());
        }

        let mut cgroups = self
            .cgroups
            .lock()
            .map_err(|_| "Failed to acquire cgroups lock".to_string())?;

        if let Some(cgroup) = cgroups.get(&cgroup_id) {
            let cgroup_guard = cgroup
                .lock()
                .map_err(|_| "Failed to acquire cgroup lock".to_string())?;

            if cgroup_guard.process_count() > 0 {
                return Err("Cannot remove cgroup with processes".to_string());
            }

            if let Some(parent_id) = cgroup_guard.parent_id {
                if let Some(parent) = cgroups.get(&parent_id) {
                    let mut parent_guard = parent
                        .lock()
                        .map_err(|_| "Failed to acquire parent lock".to_string())?;
                    parent_guard.remove_child(cgroup_id);
                }
            }
        }

        cgroups.remove(&cgroup_id);
        Ok(())
    }

    /// Add process to cgroup
    pub fn add_process_to_cgroup(&self, cgroup_id: CgroupId, process_id: u32) -> Result<(), String> {
        let cgroups = self
            .cgroups
            .lock()
            .map_err(|_| "Failed to acquire cgroups lock".to_string())?;

        if let Some(cgroup) = cgroups.get(&cgroup_id) {
            let mut cgroup_guard = cgroup
                .lock()
                .map_err(|_| "Failed to acquire cgroup lock".to_string())?;
            cgroup_guard.add_process(process_id)?;
            Ok(())
        } else {
            Err(format!("cgroup {} not found", cgroup_id))
        }
    }

    /// Remove process from cgroup
    pub fn remove_process_from_cgroup(&self, cgroup_id: CgroupId, process_id: u32) -> Result<(), String> {
        let cgroups = self
            .cgroups
            .lock()
            .map_err(|_| "Failed to acquire cgroups lock".to_string())?;

        if let Some(cgroup) = cgroups.get(&cgroup_id) {
            let mut cgroup_guard = cgroup
                .lock()
                .map_err(|_| "Failed to acquire cgroup lock".to_string())?;
            cgroup_guard.remove_process(process_id)?;
            Ok(())
        } else {
            Err(format!("cgroup {} not found", cgroup_id))
        }
    }

    /// Register controller in cgroup
    pub fn register_controller(&self, cgroup_id: CgroupId, controller: Controller) -> Result<(), String> {
        let cgroups = self
            .cgroups
            .lock()
            .map_err(|_| "Failed to acquire cgroups lock".to_string())?;

        if let Some(cgroup) = cgroups.get(&cgroup_id) {
            let mut cgroup_guard = cgroup
                .lock()
                .map_err(|_| "Failed to acquire cgroup lock".to_string())?;
            cgroup_guard.register_controller(controller)?;
            Ok(())
        } else {
            Err(format!("cgroup {} not found", cgroup_id))
        }
    }

    /// Get cgroup info
    pub fn get_cgroup_info(&self, cgroup_id: CgroupId) -> Result<(PathBuf, usize, usize), String> {
        let cgroups = self
            .cgroups
            .lock()
            .map_err(|_| "Failed to acquire cgroups lock".to_string())?;

        if let Some(cgroup) = cgroups.get(&cgroup_id) {
            let cgroup_guard = cgroup
                .lock()
                .map_err(|_| "Failed to acquire cgroup lock".to_string())?;
            Ok((
                cgroup_guard.path.clone(),
                cgroup_guard.process_count(),
                cgroup_guard.child_count(),
            ))
        } else {
            Err(format!("cgroup {} not found", cgroup_id))
        }
    }

    /// Freeze cgroup
    pub fn freeze_cgroup(&self, cgroup_id: CgroupId) -> Result<(), String> {
        let cgroups = self
            .cgroups
            .lock()
            .map_err(|_| "Failed to acquire cgroups lock".to_string())?;

        if let Some(cgroup) = cgroups.get(&cgroup_id) {
            let mut cgroup_guard = cgroup
                .lock()
                .map_err(|_| "Failed to acquire cgroup lock".to_string())?;
            cgroup_guard.freeze()?;
            Ok(())
        } else {
            Err(format!("cgroup {} not found", cgroup_id))
        }
    }

    /// Thaw cgroup
    pub fn thaw_cgroup(&self, cgroup_id: CgroupId) -> Result<(), String> {
        let cgroups = self
            .cgroups
            .lock()
            .map_err(|_| "Failed to acquire cgroups lock".to_string())?;

        if let Some(cgroup) = cgroups.get(&cgroup_id) {
            let mut cgroup_guard = cgroup
                .lock()
                .map_err(|_| "Failed to acquire cgroup lock".to_string())?;
            cgroup_guard.thaw()?;
            Ok(())
        } else {
            Err(format!("cgroup {} not found", cgroup_id))
        }
    }

    /// Get total cgroup count
    pub fn cgroup_count(&self) -> Result<usize, String> {
        let cgroups = self
            .cgroups
            .lock()
            .map_err(|_| "Failed to acquire cgroups lock".to_string())?;
        Ok(cgroups.len())
    }
}

impl Default for CgroupHierarchy {
    fn default() -> Self {
        Self::new().expect("Failed to create cgroup hierarchy")
    }
}

impl Clone for CgroupHierarchy {
    fn clone(&self) -> Self {
        CgroupHierarchy {
            root: Arc::clone(&self.root),
            cgroups: Arc::clone(&self.cgroups),
            next_id: Arc::clone(&self.next_id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_controller_type_strings() {
        assert_eq!(ControllerType::Cpu.as_str(), "cpu");
        assert_eq!(ControllerType::Memory.as_str(), "memory");
        assert_eq!(ControllerType::Pids.as_str(), "pids");
    }

    #[test]
    fn test_cpu_controller() {
        let cpu = CpuController::new();
        assert_eq!(cpu.cpu_quota_us, 1_000_000);
    }

    #[test]
    fn test_memory_controller() {
        let mem = MemoryController::new();
        assert_eq!(mem.memory_limit, u64::MAX);
    }

    #[test]
    fn test_controller_enum() {
        let cpu = Controller::Cpu(CpuController::new());
        assert_eq!(cpu.controller_type(), ControllerType::Cpu);
    }

    #[test]
    fn test_cgroup_creation() {
        let cgroup = Cgroup::new(1, PathBuf::from("/test"), None);
        assert_eq!(cgroup.id, 1);
        assert_eq!(cgroup.state, CgroupState::Active);
    }

    #[test]
    fn test_cgroup_hierarchy() {
        let hierarchy = CgroupHierarchy::new().unwrap();
        assert_eq!(hierarchy.cgroup_count().unwrap(), 1);
    }

    #[test]
    fn test_cgroup_hierarchy_create_child() {
        let hierarchy = CgroupHierarchy::new().unwrap();
        let child_id = hierarchy
            .create_cgroup(PathBuf::from("/child"), Some(1))
            .unwrap();
        assert_eq!(child_id, 2);
    }

    #[test]
    fn test_cgroup_add_process() {
        let hierarchy = CgroupHierarchy::new().unwrap();
        let child_id = hierarchy
            .create_cgroup(PathBuf::from("/child"), Some(1))
            .unwrap();
        hierarchy.add_process_to_cgroup(child_id, 100).unwrap();
    }
}
