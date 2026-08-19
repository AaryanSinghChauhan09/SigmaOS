#![no_std]
#![cfg_attr(not(test), no_main)]

extern crate alloc as std_alloc;
use std_alloc::boxed::Box;

/// OOP-based Lightweight Init System for SigmaOS
/// Based on Ideas-999-Structured: Core System Item 5
/// Implements minimal init system with service management, dependency resolution, parallel startup

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ServiceID = usize;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState { Stopped = 0, Starting = 1, Running = 2, Stopping = 3, Failed = 4 }

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitError { Success = 0, ServiceNotFound = 1, DependencyFailed = 2, StartFailed = 3, StopFailed = 4 }

pub trait Service {
    fn id(&self) -> ServiceID;
    fn name(&self) -> &[u8];
    fn state(&self) -> ServiceState;
    fn dependencies(&self) -> Vec<ServiceID>;
    fn start(&mut self) -> Result<(), InitError>;
    fn stop(&mut self) -> Result<(), InitError>;
    fn restart(&mut self) -> Result<(), InitError>;
}

#[repr(C)]
pub struct SimpleService {
    pub id: ServiceID,
    pub name: [u8; 64],
    pub state: AtomicUsize,
    pub deps: Vec<ServiceID>,
    pub pid: AtomicUsize,
}

impl SimpleService {
    pub fn new(id: ServiceID, name: &[u8]) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        SimpleService {
            id,
            name: name_array,
            state: AtomicUsize::new(ServiceState::Stopped as usize),
            deps: Vec::new(),
            pid: AtomicUsize::new(0),
        }
    }
}

impl Service for SimpleService {
    fn id(&self) -> ServiceID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn state(&self) -> ServiceState {
        let val = self.state.load(Ordering::SeqCst) as u32;
        unsafe { core::mem::transmute(val) }
    }
    fn dependencies(&self) -> Vec<ServiceID> { self.deps.clone() }

    fn start(&mut self) -> Result<(), InitError> {
        self.state.store(ServiceState::Starting as usize, Ordering::SeqCst);
        self.state.store(ServiceState::Running as usize, Ordering::SeqCst);
        self.pid.store(self.id + 1000, Ordering::SeqCst);
        Ok(())
    }

    fn stop(&mut self) -> Result<(), InitError> {
        self.state.store(ServiceState::Stopping as usize, Ordering::SeqCst);
        self.state.store(ServiceState::Stopped as usize, Ordering::SeqCst);
        self.pid.store(0, Ordering::SeqCst);
        Ok(())
    }

    fn restart(&mut self) -> Result<(), InitError> {
        self.stop()?;
        self.start()?;
        Ok(())
    }
}

// ============================================================================
// Parallel Boot & Service Startup Optimization Orchestrator
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BootStage {
    EarlyKernel = 0,
    SystemDrivers = 1,
    CoreServices = 2,
    UserSpaceUI = 3,
}

#[derive(Debug, Clone)]
pub struct BootTask {
    pub task_id: usize,
    pub name: [u8; 32],
    pub stage: BootStage,
    pub startup_time_ms: usize,
    pub is_completed: bool,
}

pub struct ParallelBootOrchestrator {
    pub tasks: Vec<BootTask>,
    pub current_stage: BootStage,
    pub total_boot_time_ms: usize,
}

impl ParallelBootOrchestrator {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            current_stage: BootStage::EarlyKernel,
            total_boot_time_ms: 0,
        }
    }

    pub fn register_task(&mut self, name: &str, stage: BootStage, estimated_ms: usize) {
        let mut name_arr = [0u8; 32];
        let len = name.len().min(31);
        name_arr[..len].copy_from_slice(&name.as_bytes()[..len]);

        let id = self.tasks.len() + 1;
        self.tasks.push(BootTask {
            task_id: id,
            name: name_arr,
            stage,
            startup_time_ms: estimated_ms,
            is_completed: false,
        });
    }

    /// Parallelizes task execution within each BootStage phase
    pub fn execute_parallel_boot(&mut self) -> usize {
        self.total_boot_time_ms = 0;
        let stages = [
            BootStage::EarlyKernel,
            BootStage::SystemDrivers,
            BootStage::CoreServices,
            BootStage::UserSpaceUI,
        ];

        for &stage in &stages {
            self.current_stage = stage;
            let mut stage_max_ms = 0;

            // Compute wall-clock time for parallel stage execution (max of concurrently run tasks in stage)
            for i in 0..self.tasks.len() {
                if self.tasks[i].stage == stage {
                    self.tasks[i].is_completed = true;
                    if self.tasks[i].startup_time_ms > stage_max_ms {
                        stage_max_ms = self.tasks[i].startup_time_ms;
                    }
                }
            }

            self.total_boot_time_ms += stage_max_ms;
        }

        self.total_boot_time_ms
    }
}

impl Default for ParallelBootOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

pub trait InitSystem {
    fn register_service(&mut self, service: Box<dyn Service>) -> Result<ServiceID, InitError>;
    fn start_service(&mut self, id: ServiceID) -> Result<(), InitError>;
    fn stop_service(&mut self, id: ServiceID) -> Result<(), InitError>;
    fn get_service(&self, id: ServiceID) -> Option<&dyn Service>;
    fn get_all_services(&self) -> Vec<ServiceID>;
}

#[repr(C)]
pub struct SigmaInit {
    pub services: Vec<Option<Box<dyn Service>>>,
    pub next_id: AtomicUsize,
    pub parallel_startup: AtomicUsize,
}

impl SigmaInit {
    pub fn new() -> Self {
        SigmaInit {
            services: Vec::new(),
            next_id: AtomicUsize::new(1),
            parallel_startup: AtomicUsize::new(1),
        }
    }

    pub fn restart_service(&mut self, id: ServiceID) -> Result<(), InitError> {
        self.stop_service(id)?;
        self.start_service(id)?;
        Ok(())
    }

    pub fn enable_parallel_startup(&mut self) {
        self.parallel_startup.store(1, Ordering::SeqCst);
    }

    pub fn disable_parallel_startup(&mut self) {
        self.parallel_startup.store(0, Ordering::SeqCst);
    }

    pub fn restart_service(&mut self, id: ServiceID) -> Result<(), InitError> {
        self.stop_service(id)?;
        self.start_service(id)?;
        Ok(())
    }
}

impl InitSystem for SigmaInit {
    fn register_service(&mut self, service: Box<dyn Service>) -> Result<ServiceID, InitError> {
        let id = service.id();
        self.services.push(Some(service));
        Ok(id)
    }

    fn start_service(&mut self, id: ServiceID) -> Result<(), InitError> {
        let mut deps_to_start = Vec::new();
        let mut found_idx = None;

        for (i, svc_option) in self.services.iter().enumerate() {
            if let Some(ref svc) = *svc_option {
                if svc.id() == id {
                    deps_to_start = svc.dependencies();
                    found_idx = Some(i);
                    break;
                }
            }
        }

        if let Some(idx) = found_idx {
            for dep_id in deps_to_start {
                self.start_service(dep_id)?;
            }
            if let Some(ref mut svc) = self.services[idx] {
                return svc.start();
            }
        }

        Err(InitError::ServiceNotFound)
    }

    fn stop_service(&mut self, id: ServiceID) -> Result<(), InitError> {
        for svc_option in &mut self.services {
            if let Some(ref mut svc) = *svc_option {
                if svc.id() == id {
                    return svc.stop();
                }
            }
        }
        Err(InitError::ServiceNotFound)
    }

    fn get_service(&self, id: ServiceID) -> Option<&dyn Service> {
        for svc_option in &self.services {
            if let Some(ref svc) = *svc_option {
                if svc.id() == id { return Some(svc.as_ref()); }
            }
        }
        None
    }

    fn get_all_services(&self) -> Vec<ServiceID> {
        let mut ids = Vec::new();
        for svc_option in &self.services {
            if let Some(ref svc) = *svc_option {
                ids.push(svc.id());
            }
        }
        ids
    }
}

pub trait DependencyResolver {
    fn resolve_startup_order(&self, services: &[ServiceID]) -> Result<Vec<ServiceID>, InitError>;
    fn detect_cycles(&self, services: &[ServiceID]) -> bool;
}

#[repr(C)]
pub struct SimpleDependencyResolver {
    pub init: SigmaInit,
}

impl SimpleDependencyResolver {
    pub fn new(init: SigmaInit) -> Self { SimpleDependencyResolver { init } }
}

impl DependencyResolver for SimpleDependencyResolver {
    fn resolve_startup_order(&self, services: &[ServiceID]) -> Result<Vec<ServiceID>, InitError> {
        let mut order = Vec::new();
        let mut visited = Vec::new();

        for &id in services {
            if !visited.contains(&id) {
                self.visit(id, &mut order, &mut visited)?;
            }
        }

        Ok(order)
    }

    fn detect_cycles(&self, services: &[ServiceID]) -> bool {
        let mut visited = Vec::new();
        let mut rec_stack = Vec::new();

        for &id in services {
            if self.has_cycle(id, &mut visited, &mut rec_stack) {
                return true;
            }
        }

        false
    }
}

impl SimpleDependencyResolver {
    fn visit(&self, id: ServiceID, order: &mut Vec<ServiceID>, visited: &mut Vec<ServiceID>) -> Result<(), InitError> {
        if visited.contains(&id) {
            return Ok(());
        }

        visited.push(id);

        if let Some(svc) = self.init.get_service(id) {
            for dep_id in svc.dependencies() {
                self.visit(dep_id, order, visited)?;
            }
        }

        order.push(id);
        Ok(())
    }

    fn has_cycle(&self, id: ServiceID, visited: &mut Vec<ServiceID>, rec_stack: &mut Vec<ServiceID>) -> bool {
        visited.push(id);
        rec_stack.push(id);

        if let Some(svc) = self.init.get_service(id) {
            for dep_id in svc.dependencies() {
                if !visited.contains(&dep_id) {
                    if self.has_cycle(dep_id, visited, rec_stack) {
                        return true;
                    }
                } else if rec_stack.contains(&dep_id) {
                    return true;
                }
            }
        }

        rec_stack.pop();
        false
    }
}

pub trait ServiceMonitor {
    fn monitor_service(&mut self, id: ServiceID) -> Result<(), InitError>;
    fn auto_restart(&mut self, id: ServiceID) -> Result<(), InitError>;
    fn get_service_status(&self, id: ServiceID) -> Option<ServiceState>;
}

#[repr(C)]
pub struct SimpleServiceMonitor {
    pub init: SigmaInit,
    pub monitored: Vec<ServiceID>,
    pub auto_restart_enabled: AtomicUsize,
}

impl SimpleServiceMonitor {
    pub fn new(init: SigmaInit) -> Self {
        SimpleServiceMonitor {
            init,
            monitored: Vec::new(),
            auto_restart_enabled: AtomicUsize::new(0),
        }
    }
}

impl ServiceMonitor for SimpleServiceMonitor {
    fn monitor_service(&mut self, id: ServiceID) -> Result<(), InitError> {
        if self.init.get_service(id).is_none() {
            return Err(InitError::ServiceNotFound);
        }
        self.monitored.push(id);
        Ok(())
    }

    fn auto_restart(&mut self, id: ServiceID) -> Result<(), InitError> {
        if self.auto_restart_enabled.load(Ordering::SeqCst) == 0 {
            return Err(InitError::StartFailed);
        }
        self.init.restart_service(id)
    }

    fn get_service_status(&self, id: ServiceID) -> Option<ServiceState> {
        self.init.get_service(id).map(|svc| svc.state())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerDaemonType {
    SystemDaemon, // PID 1 System Docker equivalent managing core OS containers
    UserDaemon,   // User Docker equivalent managing user workloads
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerState {
    Created,
    Running,
    Exited,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SovereignSystemContainer {
    pub container_id: u32,
    pub name: [u8; 32],
    pub image_name: [u8; 32],
    pub state: ContainerState,
}

/// RancherOS-style Dual Container Daemon Init System
pub struct RancherContainerInit {
    pub system_daemon_active: bool,
    pub user_daemon_active: bool,
    pub system_containers: Vec<SovereignSystemContainer>,
    pub user_containers: Vec<SovereignSystemContainer>,
}

impl RancherContainerInit {
    pub fn new() -> Self {
        Self {
            system_daemon_active: false,
            user_daemon_active: false,
            system_containers: Vec::new(),
            user_containers: Vec::new(),
        }
    }

    /// Initializes PID 1 System Daemon managing system containers (syslog, udev, etc.)
    pub fn start_system_daemon(&mut self) {
        self.system_daemon_active = true;
        // Seed default RancherOS system-level containers
        let mut sys_log = SovereignSystemContainer {
            container_id: 1,
            name: [0; 32],
            image_name: [0; 32],
            state: ContainerState::Running,
        };
        sys_log.name[..6].copy_from_slice(b"syslog");
        sys_log.image_name[..13].copy_from_slice(b"system-syslog");

        let mut sys_udev = SovereignSystemContainer {
            container_id: 2,
            name: [0; 32],
            image_name: [0; 32],
            state: ContainerState::Running,
        };
        sys_udev.name[..4].copy_from_slice(b"udev");
        sys_udev.image_name[..11].copy_from_slice(b"system-udev");

        self.system_containers.push(sys_log);
        self.system_containers.push(sys_udev);
    }

    /// System Docker starts the secondary User Docker daemon to host user applications
    pub fn start_user_daemon(&mut self) -> Result<(), &'static str> {
        if !self.system_daemon_active {
            return Err("Cannot start User Daemon: System Daemon (PID 1) must be active first");
        }
        self.user_daemon_active = true;
        Ok(())
    }

    /// Spawn a new container managed by either the System or User daemon
    pub fn launch_container(
        &mut self,
        name: &str,
        image: &str,
        daemon: ContainerDaemonType,
    ) -> Result<u32, &'static str> {
        let mut name_arr = [0u8; 32];
        let mut img_arr = [0u8; 32];

        let n_len = name.len().min(31);
        let i_len = image.len().min(31);
        name_arr[..n_len].copy_from_slice(&name.as_bytes()[..n_len]);
        img_arr[..i_len].copy_from_slice(&image.as_bytes()[..i_len]);

        match daemon {
            ContainerDaemonType::SystemDaemon => {
                if !self.system_daemon_active {
                    return Err("System Daemon inactive");
                }
                let id = (self.system_containers.len() + 1) as u32;
                self.system_containers.push(SovereignSystemContainer {
                    container_id: id,
                    name: name_arr,
                    image_name: img_arr,
                    state: ContainerState::Running,
                });
                Ok(id)
            }
            ContainerDaemonType::UserDaemon => {
                if !self.user_daemon_active {
                    return Err("User Daemon inactive");
                }
                let id = (self.user_containers.len() + 1) as u32;
                self.user_containers.push(SovereignSystemContainer {
                    container_id: id,
                    name: name_arr,
                    image_name: img_arr,
                    state: ContainerState::Running,
                });
                Ok(id)
            }
        }
    }
}

impl Default for RancherContainerInit {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::Deref;
        self.deref().iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::DerefMut;
        self.deref_mut().iter_mut()
    }
}

impl<T> IntoIterator for Vec<T> {
    type Item = T;
    type IntoIter = VecIntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        VecIntoIter { vec: self, index: 0 }
    }
}

pub struct VecIntoIter<T> {
    vec: Vec<T>,
    index: usize,
}

impl<T> Iterator for VecIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len {
            unsafe {
                let item = core::ptr::read(self.vec.data.add(self.index));
                self.index += 1;
                Some(item)
            }
        } else {
            None
        }
    }
}

impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    fn clone(&self) -> Vec<T> {
        let mut new_vec = Vec::new();
        for i in 0..self.len {
            unsafe {
                let item = core::ptr::read(self.data.add(i));
                new_vec.push(item);
            }
        }
        new_vec
    }
    fn len(&self) -> usize {
        self.len
    }
    fn contains(&self, item: &T) -> bool where T: PartialEq {
        for i in 0..self.len {
            unsafe {
                if &*self.data.add(i) == item { return true; }
            }
        }
        false
    }
    fn pop(&mut self) -> Option<T> {
        if self.len == 0 { return None; }
        self.len -= 1;
        unsafe { Some(core::ptr::read(self.data.add(self.len))) }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

#[cfg(not(target_os = "none"))]
#[no_mangle]
pub unsafe extern "C" fn alloc(size: usize) -> *mut u8 {
    use std_alloc::alloc::{alloc as std_alloc_fn, Layout};
    let layout = Layout::from_size_align(size, 8).unwrap();
    std_alloc_fn(layout)
}

#[cfg(not(target_os = "none"))]
#[no_mangle]
pub unsafe extern "C" fn free(ptr: *mut u8) {
    let _ = ptr;
}

#[cfg(target_os = "none")]
extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }

/// Runit-inspired lightweight service supervisor (runsv / runsvdir equivalent)
pub struct RunitServiceSupervisor {
    pub service_name: [u8; 32],
    pub pid: AtomicUsize,
    pub restart_count: AtomicUsize,
    pub is_down: bool,
    pub auto_respawn: bool,
}

impl RunitServiceSupervisor {
    pub fn new(name: &str) -> Self {
        let mut name_arr = [0u8; 32];
        let len = name.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_bytes().as_ptr(), name_arr.as_mut_ptr(), len);
        }
        RunitServiceSupervisor {
            service_name: name_arr,
            pid: AtomicUsize::new(0),
            restart_count: AtomicUsize::new(0),
            is_down: false,
            auto_respawn: true,
        }
    }

    /// Simulates runsv execution loop spawning or restarting service
    pub fn runsv_step(&mut self) -> Result<usize, InitError> {
        if self.is_down {
            return Err(InitError::StopFailed);
        }
        let current_pid = self.pid.load(Ordering::SeqCst);
        if current_pid == 0 {
            // Service crashed or not started; respawn immediately
            let new_pid = 2000 + self.restart_count.fetch_add(1, Ordering::SeqCst);
            self.pid.store(new_pid, Ordering::SeqCst);
            Ok(new_pid)
        } else {
            Ok(current_pid)
        }
    }

    /// Sends SIGTERM equivalent to stop supervised process (sv down equivalent)
    pub fn sv_down(&mut self) {
        self.is_down = true;
        self.pid.store(0, Ordering::SeqCst);
    }

    /// Restores supervised process monitoring (sv up equivalent)
    pub fn sv_up(&mut self) {
        self.is_down = false;
    }
}

/// OpenRC-inspired Runlevel Target Manager (boot, default, nonetwork, shutdown)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenRcRunlevel {
    SysInit,
    Boot,
    Default,
    NoNetwork,
    Shutdown,
}

pub struct OpenRcRunlevelManager {
    pub current_runlevel: OpenRcRunlevel,
    pub active_services: Vec<[u8; 32]>,
}

impl OpenRcRunlevelManager {
    pub fn new() -> Self {
        OpenRcRunlevelManager {
            current_runlevel: OpenRcRunlevel::SysInit,
            active_services: Vec::new(),
        }
    }

    /// Switch OpenRC runlevel and trigger init scripts in topological order
    pub fn transition_runlevel(&mut self, target: OpenRcRunlevel) -> Result<(), InitError> {
        self.current_runlevel = target;
        match target {
            OpenRcRunlevel::Boot => {
                self.add_active_service(b"devfs");
                self.add_active_service(b"procfs");
                self.add_active_service(b"sysfs");
            }
            OpenRcRunlevel::Default => {
                self.add_active_service(b"sshd");
                self.add_active_service(b"chronyd");
                self.add_active_service(b"networking");
            }
            OpenRcRunlevel::NoNetwork => {
                self.active_services = Vec::new();
            }
            OpenRcRunlevel::Shutdown => {
                self.active_services = Vec::new(); // Stop all running services
            }
            _ => {}
        }
        Ok(())
    }

    fn add_active_service(&mut self, name: &[u8]) {
        let mut arr = [0u8; 32];
        let len = name.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), arr.as_mut_ptr(), len);
        }
        self.active_services.push(arr);
    }
}

impl Default for OpenRcRunlevelManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_boot_orchestrator() {
        let mut orch = ParallelBootOrchestrator::new();
        orch.register_task("vfs_init", BootStage::EarlyKernel, 10);
        orch.register_task("mm_init", BootStage::EarlyKernel, 15);
        orch.register_task("nvme_driver", BootStage::SystemDrivers, 50);
        orch.register_task("wifi_driver", BootStage::SystemDrivers, 40);

        let total_ms = orch.execute_parallel_boot();
        // EarlyKernel max is 15ms, SystemDrivers max is 50ms -> Parallel Total = 65ms
        assert_eq!(total_ms, 65);
        assert!(orch.tasks.iter().all(|t| t.is_completed));
    }

    #[test]
    fn test_runit_supervisor_restart_loop() {
        let mut supervisor = RunitServiceSupervisor::new("sshd");
        assert_eq!(supervisor.pid.load(Ordering::SeqCst), 0);

        // First step starts the service
        let pid1 = supervisor.runsv_step().unwrap();
        assert_eq!(pid1, 2000);

        // Simulate crash
        supervisor.pid.store(0, Ordering::SeqCst);
        let pid2 = supervisor.runsv_step().unwrap();
        assert_eq!(pid2, 2001); // Respawned with new PID

        // sv down stops supervision
        supervisor.sv_down();
        assert_eq!(supervisor.runsv_step(), Err(InitError::StopFailed));
    }

    #[test]
    fn test_openrc_runlevel_transitions() {
        let mut manager = OpenRcRunlevelManager::new();
        assert_eq!(manager.current_runlevel, OpenRcRunlevel::SysInit);

        manager.transition_runlevel(OpenRcRunlevel::Boot).unwrap();
        assert_eq!(manager.current_runlevel, OpenRcRunlevel::Boot);
        assert_eq!(manager.active_services.len(), 3);

        manager.transition_runlevel(OpenRcRunlevel::Shutdown).unwrap();
        assert_eq!(manager.current_runlevel, OpenRcRunlevel::Shutdown);
        assert_eq!(manager.active_services.len(), 0);
    }

    #[test]
    fn test_rancher_container_init() {
        let mut r_init = RancherContainerInit::new();
        assert!(!r_init.system_daemon_active);
        assert!(!r_init.user_daemon_active);

        // Try launching a container before starting System daemon -> should fail
        assert!(r_init.launch_container("test", "img", ContainerDaemonType::SystemDaemon).is_err());

        // Start system daemon (PID 1)
        r_init.start_system_daemon();
        assert!(r_init.system_daemon_active);
        assert_eq!(r_init.system_containers.len(), 2); // syslog and udev seeded

        // Launch system-level container (e.g. ntp daemon)
        let ntp_id = r_init.launch_container("ntpd", "system-ntpd", ContainerDaemonType::SystemDaemon).unwrap();
        assert_eq!(ntp_id, 3);
        assert_eq!(r_init.system_containers.len(), 3);

        // Try starting user daemon before starting system daemon -> should succeed now
        assert!(r_init.start_user_daemon().is_ok());
        assert!(r_init.user_daemon_active);

        // Launch user-level workload container
        let web_id = r_init.launch_container("nginx", "user-nginx", ContainerDaemonType::UserDaemon).unwrap();
        assert_eq!(web_id, 1);
        assert_eq!(r_init.user_containers.len(), 1);
    }
}
