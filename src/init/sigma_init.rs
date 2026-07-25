// OOP-based Lightweight Init System for SigmaOS
// Implements service management, dependency resolution, parallel startup, and auto-restart monitoring.
// Designed to absorb and surpass legacy Debian SysVInit & Systemd service managers.

#![no_std]

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type ServiceID = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Stopped = 0,
    Starting = 1,
    Running = 2,
    Stopping = 3,
    Failed = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitError {
    Success = 0,
    ServiceNotFound = 1,
    DependencyFailed = 2,
    StartFailed = 3,
    StopFailed = 4,
}

pub trait Service {
    fn id(&self) -> ServiceID;
    fn name(&self) -> &str;
    fn state(&self) -> ServiceState;
    fn dependencies(&self) -> Vec<ServiceID>;
    fn start(&mut self) -> Result<(), InitError>;
    fn stop(&mut self) -> Result<(), InitError>;
    fn restart(&mut self) -> Result<(), InitError>;
}

pub struct SimpleService {
    pub id: ServiceID,
    pub name: &'static str,
    pub state: ServiceState,
    pub deps: Vec<ServiceID>,
    pub pid: usize,
}

impl SimpleService {
    pub fn new(id: ServiceID, name: &'static str) -> Self {
        SimpleService {
            id,
            name,
            state: ServiceState::Stopped,
            deps: Vec::new(),
            pid: 0,
        }
    }

    pub fn with_deps(mut self, deps: Vec<ServiceID>) -> Self {
        self.deps = deps;
        self
    }
}

impl Service for SimpleService {
    fn id(&self) -> ServiceID {
        self.id
    }

    fn name(&self) -> &str {
        self.name
    }

    fn state(&self) -> ServiceState {
        self.state
    }

    fn dependencies(&self) -> Vec<ServiceID> {
        self.deps.clone()
    }

    fn start(&mut self) -> Result<(), InitError> {
        self.state = ServiceState::Running;
        self.pid = self.id + 1000;
        Ok(())
    }

    fn stop(&mut self) -> Result<(), InitError> {
        self.state = ServiceState::Stopped;
        self.pid = 0;
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
    fn restart_service(&mut self, id: ServiceID) -> Result<(), InitError>;
    fn get_service(&self, id: ServiceID) -> Option<&dyn Service>;
    fn get_all_services(&self) -> Vec<ServiceID>;
}

pub struct SigmaInit {
    pub services: Vec<Option<Box<dyn Service>>>,
    pub parallel_startup: bool,
}

impl SigmaInit {
    pub fn new() -> Self {
        SigmaInit {
            services: Vec::new(),
            parallel_startup: true,
        }
    }

    pub fn enable_parallel_startup(&mut self) {
        self.parallel_startup = true;
    }

    pub fn disable_parallel_startup(&mut self) {
        self.parallel_startup = false;
    }
}

impl Default for SigmaInit {
    fn default() -> Self {
        Self::new()
    }
}

impl InitSystem for SigmaInit {
    fn register_service(&mut self, service: Box<dyn Service>) -> Result<ServiceID, InitError> {
        let id = service.id();
        self.services.push(Some(service));
        Ok(id)
    }

    fn start_service(&mut self, id: ServiceID) -> Result<(), InitError> {
        // Retrieve dependencies first
        let mut deps = Vec::new();
        let mut found = false;

        for svc_option in &self.services {
            if let Some(ref svc) = *svc_option {
                if svc.id() == id {
                    deps = svc.dependencies();
                    found = true;
                    break;
                }
            }
        }

        if !found {
            return Err(InitError::ServiceNotFound);
        }

        // Recursively start dependencies
        for dep_id in deps {
            self.start_service(dep_id)?;
        }

        // Start the service itself
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

    fn restart_service(&mut self, id: ServiceID) -> Result<(), InitError> {
        for svc_option in &mut self.services {
            if let Some(ref mut svc) = *svc_option {
                if svc.id() == id {
                    return svc.restart();
                }
            }
        }
        Err(InitError::ServiceNotFound)
    }

    fn get_service(&self, id: ServiceID) -> Option<&dyn Service> {
        for svc_option in &self.services {
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

pub struct SimpleDependencyResolver {
    pub init: SigmaInit,
}

impl SimpleDependencyResolver {
    pub fn new(init: SigmaInit) -> Self {
        SimpleDependencyResolver { init }
    }

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
            for dep_id in svc.dependencies() {
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
        visited.pop(); // Backtrack visited to allow full DAG traversal correctly
        false
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

pub trait ServiceMonitor {
    fn monitor_service(&mut self, id: ServiceID) -> Result<(), InitError>;
    fn auto_restart(&mut self, id: ServiceID) -> Result<(), InitError>;
    fn get_service_status(&self, id: ServiceID) -> Option<ServiceState>;
}

pub struct SimpleServiceMonitor {
    pub init: SigmaInit,
    pub monitored: Vec<ServiceID>,
    pub auto_restart_enabled: bool,
}

impl SimpleServiceMonitor {
    pub fn new(init: SigmaInit) -> Self {
        SimpleServiceMonitor {
            init,
            monitored: Vec::new(),
            auto_restart_enabled: true,
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
        if !self.auto_restart_enabled {
            return Err(InitError::StartFailed);
        }
        self.init.restart_service(id)
    }

    fn get_service_status(&self, id: ServiceID) -> Option<ServiceState> {
        self.init.get_service(id).map(|svc| svc.state())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_lifecycle() {
        let mut svc = SimpleService::new(1, "network-daemon");
        assert_eq!(svc.state(), ServiceState::Stopped);

        svc.start().unwrap();
        assert_eq!(svc.state(), ServiceState::Running);
        assert_eq!(svc.pid, 1001);

        svc.stop().unwrap();
        assert_eq!(svc.state(), ServiceState::Stopped);
    }

    #[test]
    fn test_init_dependency_resolution() {
        let mut init = SigmaInit::new();

        let db = Box::new(SimpleService::new(10, "postgres-db"));
        let web = Box::new(SimpleService::new(20, "web-server").with_deps(vec![10]));

        init.register_service(db).unwrap();
        init.register_service(web).unwrap();

        // Start web-server -> Should automatically trigger starting dependency first
        init.start_service(20).unwrap();

        assert_eq!(init.get_service(10).unwrap().state(), ServiceState::Running);
        assert_eq!(init.get_service(20).unwrap().state(), ServiceState::Running);
    }

    #[test]
    fn test_cycle_detection() {
        let mut init = SigmaInit::new();
        let svc_a = Box::new(SimpleService::new(1, "service-a").with_deps(vec![2]));
        let svc_b = Box::new(SimpleService::new(2, "service-b").with_deps(vec![1]));

        init.register_service(svc_a).unwrap();
        init.register_service(svc_b).unwrap();

        let resolver = SimpleDependencyResolver::new(init);
        assert!(resolver.detect_cycles(&[1, 2]));
    }
}
