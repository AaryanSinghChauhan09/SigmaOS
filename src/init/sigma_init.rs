#![no_std]
#![allow(warnings)]
#![allow(clippy::all)]
#![no_std]
#![no_main]
// #![no_std]
// #![no_main]

/// OOP-based Lightweight Init System for SigmaOS
/// Based on Ideas-999-Structured: Core System Item 5
/// Implements minimal init system with service management, dependency resolution, parallel startup,
/// and modular FirmwarePort / SecurityPort structures
extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;
/// Implements minimal init system with service management, dependency resolution, parallel startup
/// Implements minimal init system with service management, dependency resolution, parallel startup, and AI-driven diagnostics

use core::sync::atomic::{AtomicUsize, Ordering};

pub type ServiceID = usize;

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Stopped = 0,
    Starting = 1,
    Running = 2,
    Stopping = 3,
    Failed = 4,
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ServiceState { Stopped = 0, Starting = 1, Running = 2, Stopping = 3, Failed = 4 }
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState { Stopped = 0, Starting = 1, Running = 2, Stopping = 3, Failed = 4 }

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitError {
    Success = 0,
    ServiceNotFound = 1,
    DependencyFailed = 2,
    StartFailed = 3,
    StopFailed = 4,
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum InitError { Success = 0, ServiceNotFound = 1, DependencyFailed = 2, StartFailed = 3, StopFailed = 4 }
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
    fn increment_restarts(&self) -> usize;
}

pub struct SimpleService {
    pub id: ServiceID,
    pub name: [u8; 64],
    pub state: AtomicUsize,
    pub deps: Vec<ServiceID>,
    pub pid: AtomicUsize,
    pub restart_count: AtomicUsize,
}

impl SimpleService {
    pub fn new(id: ServiceID, name: &[u8]) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        for i in 0..name_len { name_array[i] = name[i]; }
        SimpleService {
            id,
            name: name_array,
            state: AtomicUsize::new(ServiceState::Stopped as usize),
            deps: Vec::new(),
            pid: AtomicUsize::new(0),
            restart_count: AtomicUsize::new(0),
        }
    }
}

impl Service for SimpleService {
    fn id(&self) -> ServiceID {
        self.id
    }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn state(&self) -> ServiceState {
        unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) }
    }
    fn dependencies(&self) -> Vec<ServiceID> {
        self.deps.clone()
    }
    fn state(&self) -> ServiceState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }
    fn dependencies(&self) -> Vec<ServiceID> { self.deps.clone() }
    fn state(&self) -> ServiceState {
        match self.state.load(Ordering::SeqCst) {
            0 => ServiceState::Stopped,
            1 => ServiceState::Starting,
            2 => ServiceState::Running,
            3 => ServiceState::Stopping,
            _ => ServiceState::Failed,
        }
    }
    fn dependencies(&self) -> Vec<ServiceID> { self.deps.clone() }

    fn start(&mut self) -> Result<(), InitError> {
        self.state
            .store(ServiceState::Starting as usize, Ordering::SeqCst);
        self.state
            .store(ServiceState::Running as usize, Ordering::SeqCst);
        self.pid.store(self.id + 1000, Ordering::SeqCst);
        Ok(())
    }

    fn stop(&mut self) -> Result<(), InitError> {
        self.state
            .store(ServiceState::Stopping as usize, Ordering::SeqCst);
        self.state
            .store(ServiceState::Stopped as usize, Ordering::SeqCst);
        self.pid.store(0, Ordering::SeqCst);
        Ok(())
    }

    fn restart(&mut self) -> Result<(), InitError> {
        self.stop()?;
        self.start()?;
        Ok(())
    }

    fn increment_restarts(&self) -> usize {
        self.restart_count.fetch_add(1, Ordering::SeqCst) + 1
    }
}

pub trait InitSystem {
    fn register_service(&mut self, service: Box<dyn Service>) -> Result<ServiceID, InitError>;
    fn start_service(&mut self, id: ServiceID) -> Result<(), InitError>;
    fn stop_service(&mut self, id: ServiceID) -> Result<(), InitError>;
    fn restart_service(&mut self, id: ServiceID) -> Result<(), InitError>;
    fn get_service(&self, id: ServiceID) -> Option<&dyn Service>;
    fn get_all_services(&self) -> Vec<ServiceID>;
}

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

    pub fn enable_parallel_startup(&mut self) {
        self.parallel_startup.store(1, Ordering::SeqCst);
    }

    pub fn disable_parallel_startup(&mut self) {
        self.parallel_startup.store(0, Ordering::SeqCst);
    }

    pub fn restart_service(&mut self, id: ServiceID) -> Result<(), InitError> {
        for svc_option in &mut self.services {
            if let Some(ref mut svc) = *svc_option {
                if svc.id() == id {
                    return svc.restart();
                }
            }
        }
        Err(InitError::ServiceNotFound)
    }
}

impl Default for SigmaInit {
    fn default() -> Self {
        Self::new()
    }

    // =========================================================================
    // SigmaInit Evolution: Parallel Startup Schedule, Bottleneck Prediction, Self-Healing
    // =========================================================================

    pub fn parallel_DAG_startup(&mut self) -> Result<Vec<Vec<ServiceID>>, InitError> {
        // Parallel Startup DAG scheduler: Schedules independent services to launch on different parallel cores
        let ids = self.get_all_services();
        let mut scheduled = Vec::new();
        let mut completed = Vec::new();

        while completed.len < ids.len {
            let mut current_wave = Vec::new();
            for i in 0..self.services.len {
                let svc_option = unsafe { &*self.services.data.add(i) };
                if let Some(ref svc) = *svc_option {
                    let id = svc.id();
                    if completed.contains(&id) {
                        continue;
                    }
                    // Check if all dependencies are completed
                    let mut deps_satisfied = true;
                    let deps = svc.dependencies();
                    for j in 0..deps.len {
                        let dep_id = unsafe { *deps.data.add(j) };
                        if !completed.contains(&dep_id) {
                            deps_satisfied = false;
                            break;
                        }
                    }
                    if deps_satisfied {
                        current_wave.push(id);
                    }
                }
            }

            if current_wave.len == 0 {
                // Dependency deadlock/cycle detected
                return Err(InitError::DependencyFailed);
            }

            // Move current wave to completed and scheduled
            for j in 0..current_wave.len {
                let id = unsafe { *current_wave.data.add(j) };
                completed.push(id);
            }
            scheduled.push(current_wave);
        }

        Ok(scheduled)
    }

    pub fn predict_boot_bottleneck(&self) -> Option<ServiceID> {
        // AI-driven Bottleneck prediction: identifies the service with the highest dependent weight
        let ids = self.get_all_services();
        if ids.len == 0 {
            return None;
        }

        let mut max_deps = 0;
        let mut bottleneck_id = None;

        for i in 0..self.services.len {
            let svc_option = unsafe { &*self.services.data.add(i) };
            if let Some(ref svc) = *svc_option {
                let mut dep_weight = 0;
                // Count how many other services depend on this service
                for j in 0..self.services.len {
                    let other_option = unsafe { &*self.services.data.add(j) };
                    if let Some(ref other) = *other_option {
                        if other.dependencies().contains(&svc.id()) {
                            dep_weight += 1;
                        }
                    }
                }
                if dep_weight > max_deps {
                    max_deps = dep_weight;
                    bottleneck_id = Some(svc.id());
                }
            }
        }

        if bottleneck_id.is_none() {
            // fallback
            unsafe { Some(*ids.data.add(0)) }
        } else {
            bottleneck_id
        }
    }

    pub fn self_healing_restart(&mut self, id: ServiceID) -> Result<(), InitError> {
        // Self-Healing Restart: implements exponential backoff to intelligently restart failed daemons
        for i in 0..self.services.len {
            let svc_option = unsafe { &mut *self.services.data.add(i) };
            if let Some(ref mut svc) = *svc_option {
                if svc.id() == id {
                    let count = svc.increment_restarts();
                    if count > 5 {
                        // Prevent blind infinite restart loops, mark as Failed
                        return Err(InitError::StartFailed);
                    }
                    // Exponential backoff logic (simulated delay ticks)
                    let _backoff_delay = 1 << count;
                    return svc.restart();
                }
            }
        }
        Err(InitError::ServiceNotFound)
    }
}

impl InitSystem for SigmaInit {
    fn register_service(&mut self, service: Box<dyn Service>) -> Result<ServiceID, InitError> {
        let id = service.id();
        self.services.push(Some(service));
        Ok(id)
    }

    fn start_service(&mut self, id: ServiceID) -> Result<(), InitError> {
        // Fetch dependencies first to avoid double borrowing
        let mut deps = Vec::new();
        for svc_option in &self.services {
            if let Some(ref svc) = *svc_option {
                if svc.id() == id {
                    deps = svc.dependencies();
                    break;
                }
            }
        }

        for dep_id in deps {
            self.start_service(dep_id)?;
        }

        // Start main service
        for svc_option in &mut self.services {
        for svc_option in &mut self.services {
        for i in 0..self.services.len {
            let svc_option = unsafe { &mut *self.services.data.add(i) };
            if let Some(ref mut svc) = *svc_option {
                if svc.id() == id {
                    let deps = svc.dependencies();
                    for dep_id in deps {
                        self.start_service(dep_id)?;
                    }
                    let deps = svc.dependencies();
                    for j in 0..deps.len {
                        let dep_id = unsafe { *deps.data.add(j) };
                        self.start_service(dep_id)?;
                    }
                    return svc.start();
                }
            }
        }
        Err(InitError::ServiceNotFound)
    }

    fn stop_service(&mut self, id: ServiceID) -> Result<(), InitError> {
        for i in 0..self.services.len {
            let svc_option = unsafe { &mut *self.services.data.add(i) };
            if let Some(ref mut svc) = *svc_option {
                if svc.id() == id {
                    return svc.stop();
                }
            }
        }
        Err(InitError::ServiceNotFound)
    }

    fn restart_service(&mut self, id: ServiceID) -> Result<(), InitError> {
        for i in 0..self.services.len {
            let svc_option = unsafe { &mut *self.services.data.add(i) };
            if let Some(ref mut svc) = *svc_option {
                if svc.id() == id {
                    return svc.restart();
                }
            }
        }
        Err(InitError::ServiceNotFound)
    }

    fn get_service(&self, id: ServiceID) -> Option<&dyn Service> {
        for i in 0..self.services.len {
            let svc_option = unsafe { &*self.services.data.add(i) };
            if let Some(ref svc) = *svc_option {
                if svc.id() == id {
                    return Some(svc.as_ref());
                }
            }
        }
        None
    }

    fn get_all_services(&self) -> Vec<ServiceID> {
        let mut ids = Vec::new();
        for i in 0..self.services.len {
            let svc_option = unsafe { &*self.services.data.add(i) };
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

pub struct SimpleDependencyResolver {
    pub init: SigmaInit,
}

impl SimpleDependencyResolver {
    pub fn new(init: SigmaInit) -> Self {
        SimpleDependencyResolver { init }
    }
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
    fn visit(
        &self,
        id: ServiceID,
        order: &mut Vec<ServiceID>,
        visited: &mut Vec<ServiceID>,
    ) -> Result<(), InitError> {
        if visited.contains(&id) {
            return Ok(());
        }

        visited.push(id);

        if let Some(svc) = self.init.get_service(id) {
            let deps = svc.dependencies();
            for j in 0..deps.len {
                let dep_id = unsafe { *deps.data.add(j) };
                self.visit(dep_id, order, visited)?;
            }
        }

        order.push(id);
        Ok(())
    }

    fn has_cycle(
        &self,
        id: ServiceID,
        visited: &mut Vec<ServiceID>,
        rec_stack: &mut Vec<ServiceID>,
    ) -> bool {
        visited.push(id);
        rec_stack.push(id);

        if let Some(svc) = self.init.get_service(id) {
            let deps = svc.dependencies();
            for j in 0..deps.len {
                let dep_id = unsafe { *deps.data.add(j) };
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

/// Advanced OOP-driven Firmware Port Class Hierarchy
pub trait FirmwarePort {
    fn boot_type(&self) -> &'static str;
    fn handoff(&self) -> Result<(), &'static str>;
}
struct Vec<T> { data: *mut T, len: usize, capacity: usize }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerDaemonType {
    SystemDaemon, // PID 1 System Docker equivalent managing core OS containers
    UserDaemon,   // User Docker equivalent managing user workloads
}
struct Vec<T> { data: *mut T, len: usize, capacity: usize }
pub struct Vec<T> { pub data: *mut T, pub len: usize, pub capacity: usize }

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
impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
impl<T> Vec<T> {
    pub fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    pub fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
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
    fn clone(&self) -> Vec<T> {
        let mut new_vec = Vec::new();
        for i in 0..self.len {
            unsafe {
                let item = core::ptr::read(self.data.add(i));
                new_vec.push(item);
            }
    pub fn clone(&self) -> Vec<T> {
        let mut new_vec = Vec::new();
        for i in 0..self.len {
            unsafe {
                let item = core::ptr::read(self.data.add(i));
                new_vec.push(item);
            }
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
    fn contains(&self, item: &T) -> bool where T: PartialEq {
        for i in 0..self.len {
            unsafe {
                if &*self.data.add(i) == item { return true; }
    pub fn contains(&self, item: &T) -> bool where T: PartialEq {
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
        false
    }
    pub fn pop(&mut self) -> Option<T> {
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
    pub fn as_slice(&self) -> &[T] {
        if self.len == 0 {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl Default for RancherContainerInit {
    fn default() -> Self {
        Self::new()
    }
}

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

pub struct BIOSPort;
impl FirmwarePort for BIOSPort {
    fn boot_type(&self) -> &'static str {
        "Legacy BIOS (MBR)"
    }
    fn handoff(&self) -> Result<(), &'static str> {
        Ok(())
    }
}

pub struct UEFIPort;
impl FirmwarePort for UEFIPort {
    fn boot_type(&self) -> &'static str {
        "Modern UEFI (GPT)"
    }
    fn handoff(&self) -> Result<(), &'static str> {
        Ok(())
    }
}

pub struct CorebootPort;
impl FirmwarePort for CorebootPort {
    fn boot_type(&self) -> &'static str {
        "Coreboot (Open Source Firmware)"
    }
    fn handoff(&self) -> Result<(), &'static str> {
        Ok(())
    }
}

/// Advanced OOP-driven Security Port Class Hierarchy
pub trait SecurityPort {
    fn policy_name(&self) -> &'static str;
    fn check_capability(&self, cap: u32) -> bool;
}

pub struct DACPort;
impl SecurityPort for DACPort {
    fn policy_name(&self) -> &'static str {
        "Discretionary Access Control (DAC)"
    }
    fn check_capability(&self, _cap: u32) -> bool {
        true
    }
}

pub struct SELinuxPort;
impl SecurityPort for SELinuxPort {
    fn policy_name(&self) -> &'static str {
        "Security-Enhanced Linux (SELinux)"
    }
    fn check_capability(&self, cap: u32) -> bool {
        cap > 10
    }
}

pub struct ZeroTrustPort;
impl SecurityPort for ZeroTrustPort {
    fn policy_name(&self) -> &'static str {
        "Zero-Trust Enforcement Security"
    }
    fn check_capability(&self, _cap: u32) -> bool {
        false
    } // Absolute strict verification
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_dependency_resolution() {
        let mut init = SigmaInit::new();

        let mut svc1 = SimpleService::new(1, b"udev");
        let mut svc2 = SimpleService::new(2, b"display");
        svc2.deps.push(1);

        init.register_service(Box::new(svc1)).unwrap();
        init.register_service(Box::new(svc2)).unwrap();

        let resolver = SimpleDependencyResolver::new(init);
        let order = resolver.resolve_startup_order(&[2]).unwrap();
        assert_eq!(order.len(), 2);
        assert_eq!(order[0], 1); // udev must start first
        assert_eq!(order[1], 2);
    }

    #[test]
    fn test_firmware_ports() {
        let bios: Box<dyn FirmwarePort> = Box::new(BIOSPort);
        let uefi: Box<dyn FirmwarePort> = Box::new(UEFIPort);
        let coreboot: Box<dyn FirmwarePort> = Box::new(CorebootPort);

        assert_eq!(bios.boot_type(), "Legacy BIOS (MBR)");
        assert_eq!(uefi.boot_type(), "Modern UEFI (GPT)");
        assert_eq!(coreboot.boot_type(), "Coreboot (Open Source Firmware)");

        assert!(bios.handoff().is_ok());
    }

    #[test]
    fn test_security_ports() {
        let dac: Box<dyn SecurityPort> = Box::new(DACPort);
        let selinux: Box<dyn SecurityPort> = Box::new(SELinuxPort);
        let zt: Box<dyn SecurityPort> = Box::new(ZeroTrustPort);

        assert_eq!(dac.policy_name(), "Discretionary Access Control (DAC)");
        assert_eq!(selinux.policy_name(), "Security-Enhanced Linux (SELinux)");
        assert_eq!(zt.policy_name(), "Zero-Trust Enforcement Security");

        assert!(dac.check_capability(1));
        assert!(selinux.check_capability(20));
        assert!(!zt.check_capability(1));
    }
}
extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }

#[cfg(test)]
mod tests {

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
extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
// Allocator shim: uses std allocator on hosted targets (test/dev) and extern C on bare-metal
#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use std::alloc::{alloc as std_alloc, Layout};
    if let Ok(layout) = Layout::from_size_align(size, 8) {
        std_alloc(layout)
    } else {
        core::ptr::null_mut()
    }
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8) {
    let _ = ptr;
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
