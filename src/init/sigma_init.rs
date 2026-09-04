
/// OOP-based Lightweight Init System for SigmaOS
/// Based on Ideas-999-Structured: Core System Item 5
/// Implements minimal init system with service management, dependency resolution, parallel startup


use core::sync::atomic::{AtomicUsize, Ordering};
use std::vec::Vec;
use std::boxed::Box;

pub type ServiceID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ServiceState { Stopped = 0, Starting = 1, Running = 2, Stopping = 3, Failed = 4 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
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
        match self.state.load(Ordering::SeqCst) {
            0 => ServiceState::Stopped,
            1 => ServiceState::Starting,
            2 => ServiceState::Running,
            3 => ServiceState::Stopping,
            4 => ServiceState::Failed,
            _ => ServiceState::Stopped,
        }
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

pub trait InitSystem {
    fn register_service(&mut self, service: Box<dyn Service>) -> Result<ServiceID, InitError>;
    fn start_service(&mut self, id: ServiceID) -> Result<(), InitError>;
    fn stop_service(&mut self, id: ServiceID) -> Result<(), InitError>;
    fn restart_service(&mut self, id: ServiceID) -> Result<(), InitError> {
        self.stop_service(id)?;
        self.start_service(id)
    }
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

    pub fn enable_parallel_startup(&mut self) {
        self.parallel_startup.store(1, Ordering::SeqCst);
    }

    pub fn disable_parallel_startup(&mut self) {
        self.parallel_startup.store(0, Ordering::SeqCst);
    }
}

impl InitSystem for SigmaInit {
    fn register_service(&mut self, service: Box<dyn Service>) -> Result<ServiceID, InitError> {
        let id = service.id();
        self.services.push(Some(service));
        Ok(id)
    }

    fn start_service(&mut self, id: ServiceID) -> Result<(), InitError> {
        // First, find the service and collect its dependencies
        let deps = {
            let mut deps_vec = Vec::new();
            for svc_option in &self.services {
                if let Some(ref svc) = *svc_option {
                    if svc.id() == id {
                        deps_vec = svc.dependencies();
                        break;
                    }
                }
            }
            deps_vec
        };
        
        // Start dependencies first
        for dep_id in deps {
            self.start_service(dep_id)?;
        }
        
        // Now start the service itself
        for svc_option in &mut self.services {
            if let Some(ref mut svc) = *svc_option {
                if svc.id() == id {
                    return svc.start();
                }
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

#[cfg(test)]
mod tests {
    use super::*;

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
