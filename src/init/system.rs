#![no_std]
#![allow(warnings)]
#![allow(clippy::all)]

/// OOP-based Lightweight Init System for SigmaOS
/// Implements init system using OOP principles with traits and structs
/// No dependency on external init frameworks
/// Based on Roadmap Item 5: Lightweight init system
extern crate alloc;
use alloc::boxed::Box;
use crate::klib::Vec;

use core::sync::atomic::{AtomicUsize, Ordering};

/// Service ID
pub type ServiceID = usize;

/// Service state
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Stopped = 0,
    Starting = 1,
    Running = 2,
    Stopping = 3,
    Failed = 4,
}

/// Service trait (OOP interface)
pub trait Service {
    /// Get service ID
    fn id(&self) -> ServiceID;
    /// Get service name
    fn name(&self) -> &[u8];
    /// Start service
    fn start(&mut self) -> Result<(), InitError>;
    /// Stop service
    fn stop(&mut self) -> Result<(), InitError>;
    /// Restart service
    fn restart(&mut self) -> Result<(), InitError>;
    /// Get service state
    fn state(&self) -> ServiceState;
    /// Get service info
    fn info(&self) -> ServiceInfo;

    /// Get runlevel bitmask
    fn runlevel_mask(&self) -> u8 {
        0xFF // Default: matches all runlevels
    }

    /// Get dependencies list
    fn dependencies(&self) -> Vec<ServiceID> {
        Vec::new()
    }
}

/// Init error types
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitError {
    Success = 0,
    AlreadyStarted = 1,
    AlreadyStopped = 2,
    StartFailed = 3,
    StopFailed = 4,
    PermissionDenied = 5,
    DependencyFailed = 6,
}

/// Service info
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ServiceInfo {
    pub id: ServiceID,
    pub name: [u8; 64],
    pub state: ServiceState,
    pub pid: Option<usize>,
    pub capability: ServiceCapability,
}

impl ServiceInfo {
    pub fn new(id: ServiceID) -> Self {
        ServiceInfo {
            id,
            name: [0; 64],
            state: ServiceState::Stopped,
            pid: None,
            capability: ServiceCapability::new(),
        }
    }
}

/// Service capability
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServiceCapability {
    pub can_start: bool,
    pub can_stop: bool,
    pub can_restart: bool,
}

impl ServiceCapability {
    pub fn new() -> Self {
        ServiceCapability {
            can_start: false,
            can_stop: false,
            can_restart: false,
        }
    }

    pub fn full() -> Self {
        ServiceCapability {
            can_start: true,
            can_stop: true,
            can_restart: true,
        }
    }
}

impl Default for ServiceCapability {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple service (OOP: Concrete service class)
pub struct SimpleService {
    pub id: ServiceID,
    pub name: [u8; 64],
    pub command: [u8; 256],
    pub state: AtomicUsize, // ServiceState as usize
    pub pid: AtomicUsize,
    pub capability: ServiceCapability,
    pub dependencies: Vec<ServiceID>,
    pub runlevel_mask: u8,
}

impl SimpleService {
    pub fn new(id: ServiceID, name: &[u8], command: &[u8], capability: ServiceCapability) -> Self {
        let mut name_array = [0u8; 64];
        let mut command_array = [0u8; 256];

        let name_len = name.len().min(63);
        let cmd_len = command.len().min(255);

        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
            core::ptr::copy_nonoverlapping(command.as_ptr(), command_array.as_mut_ptr(), cmd_len);
        }

        SimpleService {
            id,
            name: name_array,
            command: command_array,
            state: AtomicUsize::new(ServiceState::Stopped as usize),
            pid: AtomicUsize::new(0),
            capability,
            dependencies: Vec::new(),
            runlevel_mask: 0xFF, // runs in all runlevels by default
        }
    }

    pub fn new_with_runlevel(id: ServiceID, name: &[u8], command: &[u8], capability: ServiceCapability, runlevel_mask: u8) -> Self {
        let mut s = SimpleService::new(id, name, command, capability);
        s.runlevel_mask = runlevel_mask;
        s
    }

    pub fn add_dependency(&mut self, dependency: ServiceID) {
        self.dependencies.push(dependency);
    }

    pub fn get_state(&self) -> ServiceState {
        unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) }
    }

    pub fn set_state(&self, state: ServiceState) {
        self.state.store(state as usize, Ordering::SeqCst);
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

    fn start(&mut self) -> Result<(), InitError> {
        if !self.capability.can_start {
            return Err(InitError::PermissionDenied);
        }

        let current_state = self.get_state();
        if current_state == ServiceState::Running || current_state == ServiceState::Starting {
            return Err(InitError::AlreadyStarted);
        }

        self.set_state(ServiceState::Starting);
        self.set_state(ServiceState::Running);
        self.pid.store(1, Ordering::SeqCst); // Simulated PID

        Ok(())
    }

    fn stop(&mut self) -> Result<(), InitError> {
        if !self.capability.can_stop {
            return Err(InitError::PermissionDenied);
        }

        let current_state = self.get_state();
        if current_state == ServiceState::Stopped {
            return Err(InitError::AlreadyStopped);
        }

        self.set_state(ServiceState::Stopping);
        self.set_state(ServiceState::Stopped);
        self.pid.store(0, Ordering::SeqCst);

        Ok(())
    }

    fn restart(&mut self) -> Result<(), InitError> {
        if !self.capability.can_restart {
            return Err(InitError::PermissionDenied);
        }

        self.stop()?;
        self.start()
    }

    fn state(&self) -> ServiceState {
        self.get_state()
    }

    fn runlevel_mask(&self) -> u8 {
        self.runlevel_mask
    }

    fn dependencies(&self) -> Vec<ServiceID> {
        self.dependencies.clone()
    }

    fn info(&self) -> ServiceInfo {
        let pid = self.pid.load(Ordering::SeqCst);
        ServiceInfo {
            id: self.id,
            name: self.name,
            state: self.get_state(),
            pid: if pid > 0 { Some(pid) } else { None },
            capability: self.capability,
        }
    }
}

/// Init system trait (OOP interface)
pub trait InitSystem {
    /// Register service
    fn register_service(&mut self, service: Box<dyn Service>) -> Result<ServiceID, InitError>;
    /// Unregister service
    fn unregister_service(&mut self, id: ServiceID) -> Result<(), InitError>;
    /// Start service
    fn start_service(&mut self, id: ServiceID) -> Result<(), InitError>;
    /// Stop service
    fn stop_service(&mut self, id: ServiceID) -> Result<(), InitError>;
    /// Restart service
    fn restart_service(&mut self, id: ServiceID) -> Result<(), InitError>;
    /// Get service
    fn get_service(&self, id: ServiceID) -> Option<&dyn Service>;
    /// Start all services
    fn start_all(&mut self) -> Result<(), InitError>;
    /// Stop all services
    fn stop_all(&mut self) -> Result<(), InitError>;
    /// Get init statistics
    fn stats(&self) -> InitStats;
}

/// Init statistics
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InitStats {
    pub total_services: usize,
    pub running_services: usize,
    pub failed_services: usize,
    pub stopped_services: usize,
}

impl InitStats {
    pub fn new() -> Self {
        InitStats {
            total_services: 0,
            running_services: 0,
            failed_services: 0,
            stopped_services: 0,
        }
    }
}

impl Default for InitStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple init system (OOP: Concrete init class)
pub struct SimpleInitSystem {
    pub services: Vec<Option<Box<dyn Service>>>,
    pub next_id: AtomicUsize,
    pub stats: InitStats,
    pub capability: InitCapability,
    pub current_runlevel: u8,
    pub logs: Vec<[u8; 64]>,
}

/// Init capability
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InitCapability {
    pub can_register: bool,
    pub can_unregister: bool,
    pub can_manage: bool,
}

impl InitCapability {
    pub fn new() -> Self {
        InitCapability {
            can_register: false,
            can_unregister: false,
            can_manage: false,
        }
    }

    pub fn full() -> Self {
        InitCapability {
            can_register: true,
            can_unregister: true,
            can_manage: true,
        }
    }
}

impl Default for InitCapability {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleInitSystem {
    pub fn new(capability: InitCapability) -> Self {
        SimpleInitSystem {
            services: Vec::new(),
            next_id: AtomicUsize::new(1),
            stats: InitStats::new(),
            capability,
            current_runlevel: 3, // Default Linux multi-user text runlevel
            logs: Vec::new(),
        }
    }

    pub fn get_service_mut(&mut self, id: ServiceID) -> Option<&mut Box<dyn Service>> {
        for service_option in &mut self.services {
            if let Some(ref mut service) = *service_option {
                if service.id() == id {
                    return Some(service);
                }
            }
        }
        None
    }

    pub fn log_event(&mut self, msg: &[u8]) {
        let mut buf = [0u8; 64];
        let len = msg.len().min(63);
        buf[..len].copy_from_slice(&msg[..len]);
        self.logs.push(buf);
    }

    /// Transitions SimpleInitSystem into target runlevel (e.g. 1, 3, 5, 6)
    pub fn set_runlevel(&mut self, runlevel: u8) -> Result<(), InitError> {
        self.current_runlevel = runlevel;

        let mut log_msg = [0u8; 64];
        let prefix = b"Transitioning to Runlevel: ";
        log_msg[..prefix.len()].copy_from_slice(prefix);
        write_int(runlevel as usize, &mut log_msg, prefix.len());
        self.log_event(&log_msg);

        let mut services_to_stop = Vec::new();
        let mut services_to_start = Vec::new();

        for i in 0..self.services.len() {
            if let Some(ref svc) = self.services[i] {
                let mask = svc.runlevel_mask();
                let belongs = (mask & (1 << runlevel)) != 0;
                let is_running = svc.state() == ServiceState::Running;

                if is_running && !belongs {
                    services_to_stop.push(svc.id());
                } else if !is_running && belongs {
                    services_to_start.push(svc.id());
                }
            }
        }

        for i in 0..services_to_stop.len() {
            let id = services_to_stop[i];
            let _ = self.stop_service(id);
        }

        for i in 0..services_to_start.len() {
            let id = services_to_start[i];
            let _ = self.start_service(id);
        }

        Ok(())
    }

    fn start_service_recursive(&mut self, id: ServiceID, visited: &mut Vec<ServiceID>) -> Result<(), InitError> {
        if visited.contains(&id) {
            return Err(InitError::DependencyFailed); // cycle detected
        }
        visited.push(id);

        let mut deps = Vec::new();
        if let Some(service) = self.get_service(id) {
            deps = service.dependencies();
        } else {
            return Err(InitError::DependencyFailed);
        }

        // Start dependencies first
        for i in 0..deps.len() {
            let dep_id = deps[i];
            let dep_state = self.get_service(dep_id).map(|s| s.state()).unwrap_or(ServiceState::Failed);
            if dep_state != ServiceState::Running {
                self.start_service_recursive(dep_id, visited)?;
            }
        }

        // Start service itself
        if let Some(ref mut service) = self.get_service_mut(id) {
            let result = service.start();
            if result.is_ok() {
                let state = service.state();
                if state == ServiceState::Running {
                    self.stats.running_services += 1;
                    self.stats.stopped_services -= 1;

                    let mut msg = [0u8; 64];
                    let prefix = b"Started service ID: ";
                    msg[..prefix.len()].copy_from_slice(prefix);
                    write_int(id, &mut msg, prefix.len());
                    self.log_event(&msg);
                }
            }
            result
        } else {
            Err(InitError::PermissionDenied)
        }
    }
}

impl InitSystem for SimpleInitSystem {
    fn register_service(&mut self, service: Box<dyn Service>) -> Result<ServiceID, InitError> {
        if !self.capability.can_register {
            return Err(InitError::PermissionDenied);
        }

        let id = service.id();
        self.services.push(Some(service));
        self.stats.total_services += 1;
        self.stats.stopped_services += 1;
        Ok(id)
    }

    fn unregister_service(&mut self, id: ServiceID) -> Result<(), InitError> {
        if !self.capability.can_unregister {
            return Err(InitError::PermissionDenied);
        }

        let mut index = None;
        for (i, service_option) in self.services.iter().enumerate() {
            if let Some(ref service) = *service_option {
                if service.id() == id {
                    index = Some(i);
                    break;
                }
            }
        }

        if let Some(i) = index {
            self.services[i] = None;
            self.stats.total_services -= 1;
            Ok(())
        } else {
            Err(InitError::PermissionDenied)
        }
    }

    fn start_service(&mut self, id: ServiceID) -> Result<(), InitError> {
        if !self.capability.can_manage {
            return Err(InitError::PermissionDenied);
        }

        let mut visited = Vec::new();
        self.start_service_recursive(id, &mut visited)
    }

    fn stop_service(&mut self, id: ServiceID) -> Result<(), InitError> {
        if !self.capability.can_manage {
            return Err(InitError::PermissionDenied);
        }

        if let Some(ref mut service) = self.get_service_mut(id) {
            let result = service.stop();
            if result.is_ok() {
                let state = service.state();
                if state == ServiceState::Stopped {
                    self.stats.running_services -= 1;
                    self.stats.stopped_services += 1;

                    let mut msg = [0u8; 64];
                    let prefix = b"Stopped service ID: ";
                    msg[..prefix.len()].copy_from_slice(prefix);
                    write_int(id, &mut msg, prefix.len());
                    self.log_event(&msg);
                }
            }
            result
        } else {
            Err(InitError::PermissionDenied)
        }
    }

    fn restart_service(&mut self, id: ServiceID) -> Result<(), InitError> {
        if !self.capability.can_manage {
            return Err(InitError::PermissionDenied);
        }

        if let Some(ref mut service) = self.get_service_mut(id) {
            service.restart()
        } else {
            Err(InitError::PermissionDenied)
        }
    }

    fn get_service(&self, id: ServiceID) -> Option<&dyn Service> {
        for service_option in &self.services {
            if let Some(ref service) = *service_option {
                if service.id() == id {
                    return Some(service.as_ref());
                }
            }
        }
        None
    }

    fn start_all(&mut self) -> Result<(), InitError> {
        for i in 0..self.services.len() {
            if let Some(ref mut service) = self.services[i] {
                let _ = service.start();
            }
        }
        Ok(())
    }

    fn stop_all(&mut self) -> Result<(), InitError> {
        for i in 0..self.services.len() {
            if let Some(ref mut service) = self.services[i] {
                let _ = service.stop();
            }
        }
        Ok(())
    }

    fn stats(&self) -> InitStats {
        self.stats
    }
}

fn write_int(mut val: usize, buf: &mut [u8], mut idx: usize) -> usize {
    if val == 0 {
        if idx < buf.len() {
            buf[idx] = b'0';
            idx += 1;
        }
        return idx;
    }
    let mut digits = [0u8; 12];
    let mut d_idx = 0;
    while val > 0 && d_idx < 12 {
        digits[d_idx] = (val % 10) as u8 + b'0';
        val /= 10;
        d_idx += 1;
    }
    while d_idx > 0 && idx < buf.len() {
        d_idx -= 1;
        buf[idx] = digits[d_idx];
        idx += 1;
    }
    idx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_lifecycle_flow() {
        let cap = ServiceCapability::full();
        let mut svc = SimpleService::new(101, b"loggerd", b"/usr/bin/loggerd", cap);
        assert_eq!(svc.id(), 101);
        assert_eq!(svc.name(), b"loggerd");
        assert_eq!(svc.get_state(), ServiceState::Stopped);

        svc.start().unwrap();
        assert_eq!(svc.get_state(), ServiceState::Running);

        svc.stop().unwrap();
        assert_eq!(svc.get_state(), ServiceState::Stopped);
    }

    #[test]
    fn test_init_system_management() {
        let cap = InitCapability::full();
        let mut init = SimpleInitSystem::new(cap);

        let svc_cap = ServiceCapability::full();
        let svc = SimpleService::new(101, b"loggerd", b"/usr/bin/loggerd", svc_cap);
        let id = init.register_service(Box::new(svc)).unwrap();
        assert_eq!(id, 101);

        assert!(init.get_service(101).is_some());
        init.start_service(101).unwrap();
        assert_eq!(init.stats().running_services, 1);

        init.stop_service(101).unwrap();
        assert_eq!(init.stats().running_services, 0);

        init.unregister_service(101).unwrap();
    }

    #[test]
    fn test_init_runlevel_switching_and_recursive_dependencies() {
        let cap = InitCapability::full();
        let mut init = SimpleInitSystem::new(cap);

        let svc_cap = ServiceCapability::full();
        // Service 10: multi-user text & graphical runlevels (Runlevel 3 and 5)
        let mut s1 = SimpleService::new_with_runlevel(10, b"dbus", b"dbus-daemon", svc_cap, (1 << 3) | (1 << 5));

        // Service 20: graphical runlevel only (Runlevel 5), requires dbus (10)
        let mut s2 = SimpleService::new_with_runlevel(20, b"gdm", b"gdm", svc_cap, 1 << 5);
        s2.add_dependency(10);

        init.register_service(Box::new(s1)).unwrap();
        init.register_service(Box::new(s2)).unwrap();

        // Transition to runlevel 3: only s1 should start
        init.set_runlevel(3).unwrap();
        assert_eq!(init.get_service(10).unwrap().state(), ServiceState::Running);
        assert_eq!(init.get_service(20).unwrap().state(), ServiceState::Stopped);

        // Transition to runlevel 1 (single-user): s1 should stop
        init.set_runlevel(1).unwrap();
        assert_eq!(init.get_service(10).unwrap().state(), ServiceState::Stopped);

        // Transition to runlevel 5: s2 should start, and recursively start s1 first!
        init.set_runlevel(5).unwrap();
        assert_eq!(init.get_service(10).unwrap().state(), ServiceState::Running);
        assert_eq!(init.get_service(20).unwrap().state(), ServiceState::Running);

        // Assert log circular buffer recorded events
        assert!(init.logs.len() > 0);
    }
}
