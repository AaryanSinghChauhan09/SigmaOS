// SigmaInit - Modern Init System
// Inspired by OpenRC, runit, s6 (systemd alternatives)

#![no_std]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::boxed::Box;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Service restart policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestartPolicy {
    Always,
    OnFailure,
    Never,
}

/// Service state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Failed,
}

/// System target
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemTarget {
    Rescue,        // Single-user mode
    MultiUser,     // Console login
    Graphical,     // Desktop environment
    Cloud,         // Cloud/headless mode
    Realtime,      // Real-time mode
}

/// Service definition
#[derive(Debug, Clone)]
pub struct Service {
    pub name: String,
    pub description: String,
    pub depends: Vec<String>,
    pub command: Vec<String>,
    pub working_dir: Option<String>,
    pub environment: BTreeMap<String, String>,
    pub capabilities: Vec<String>,
    pub restart_policy: RestartPolicy,
    pub state: ServiceState,
    pub pid: Option<u32>,
    pub socket: Option<String>,
    pub timer: Option<String>,
}

impl Service {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            description: String::new(),
            depends: Vec::new(),
            command: Vec::new(),
            working_dir: None,
            environment: BTreeMap::new(),
            capabilities: Vec::new(),
            restart_policy: RestartPolicy::OnFailure,
            state: ServiceState::Stopped,
            pid: None,
            socket: None,
            timer: None,
        }
    }
    
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = String::from(desc);
        self
    }
    
    pub fn with_command(mut self, command: Vec<String>) -> Self {
        self.command = command;
        self
    }
    
    pub fn with_depends(mut self, depends: Vec<String>) -> Self {
        self.depends = depends;
        self
    }
    
    pub fn with_restart_policy(mut self, policy: RestartPolicy) -> Self {
        self.restart_policy = policy;
        self
    }
}

/// Dependency graph for service management
pub struct DependencyGraph {
    services: BTreeMap<String, Vec<String>>, // service -> dependencies
    reverse_deps: BTreeMap<String, Vec<String>>, // service -> dependents
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            services: BTreeMap::new(),
            reverse_deps: BTreeMap::new(),
        }
    }
    
    pub fn add_service(&mut self, name: &str, dependencies: Vec<String>) {
        self.services.insert(String::from(name), dependencies.clone());
        
        for dep in &dependencies {
            self.reverse_deps
                .entry(dep.clone())
                .or_insert_with(Vec::new)
                .push(String::from(name));
        }
    }
    
    pub fn get_dependencies(&self, name: &str) -> Option<&Vec<String>> {
        self.services.get(name)
    }
    
    pub fn get_dependents(&self, name: &str) -> Option<&Vec<String>> {
        self.reverse_deps.get(name)
    }
    
    pub fn topological_sort(&self) -> Result<Vec<String>, DependencyError> {
        let mut visited = BTreeMap::new();
        let mut temp_mark = BTreeMap::new();
        let mut result = Vec::new();
        
        for name in self.services.keys() {
            if !visited.contains_key(name) {
                self.visit(name, &mut visited, &mut temp_mark, &mut result)?;
            }
        }
        
        Ok(result)
    }
    
    fn visit(
        &self,
        name: &str,
        visited: &mut BTreeMap<String, bool>,
        temp_mark: &mut BTreeMap<String, bool>,
        result: &mut Vec<String>,
    ) -> Result<(), DependencyError> {
        if temp_mark.contains_key(name) {
            return Err(DependencyError::Cycle);
        }
        
        if visited.contains_key(name) {
            return Ok(());
        }
        
        temp_mark.insert(String::from(name), true);
        
        if let Some(deps) = self.services.get(name) {
            for dep in deps {
                self.visit(dep, visited, temp_mark, result)?;
            }
        }
        
        temp_mark.remove(name);
        visited.insert(String::from(name), true);
        result.push(String::from(name));
        
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DependencyError {
    Cycle,
    NotFound,
}

/// Process supervisor (runit/s6-style)
pub struct Supervisor {
    services: BTreeMap<String, Service>,
    dependency_graph: DependencyGraph,
}

impl Supervisor {
    pub fn new() -> Self {
        Self {
            services: BTreeMap::new(),
            dependency_graph: DependencyGraph::new(),
        }
    }
    
    pub fn add_service(&mut self, service: Service) {
        let name = service.name.clone();
        let deps = service.depends.clone();
        self.dependency_graph.add_service(&name, deps);
        self.services.insert(name, service);
    }
    
    pub fn start_service(&mut self, name: &str) -> Result<(), ServiceError> {
        // Start dependencies first
        if let Some(deps) = self.dependency_graph.get_dependencies(name) {
            for dep in deps {
                self.start_service(dep)?;
            }
        }
        
        if let Some(service) = self.services.get_mut(name) {
            service.state = ServiceState::Starting;
            // In a real implementation, this would fork and execute
            service.state = ServiceState::Running;
            service.pid = Some(42); // Mock PID
            Ok(())
        } else {
            Err(ServiceError::NotFound)
        }
    }
    
    pub fn stop_service(&mut self, name: &str) -> Result<(), ServiceError> {
        // Stop dependents first
        if let Some(dependents) = self.dependency_graph.get_dependents(name) {
            for dep in dependents {
                self.stop_service(dep)?;
            }
        }
        
        if let Some(service) = self.services.get_mut(name) {
            service.state = ServiceState::Stopping;
            // In a real implementation, this would send SIGTERM
            service.state = ServiceState::Stopped;
            service.pid = None;
            Ok(())
        } else {
            Err(ServiceError::NotFound)
        }
    }
    
    pub fn restart_service(&mut self, name: &str) -> Result<(), ServiceError> {
        self.stop_service(name)?;
        self.start_service(name)
    }
    
    pub fn get_service_state(&self, name: &str) -> Option<ServiceState> {
        self.services.get(name).map(|s| s.state)
    }
    
    pub fn start_target(&mut self, target: SystemTarget) -> Result<(), ServiceError> {
        match target {
            SystemTarget::Rescue => {
                self.start_service("syslog")?;
                self.start_service(" rescue-shell")?;
            }
            SystemTarget::MultiUser => {
                self.start_service("syslog")?;
                self.start_service("network")?;
                self.start_service("sshd")?;
                self.start_service("cron")?;
            }
            SystemTarget::Graphical => {
                self.start_target(SystemTarget::MultiUser)?;
                self.start_service("display-manager")?;
                self.start_service("desktop-environment")?;
            }
            SystemTarget::Cloud => {
                self.start_service("syslog")?;
                self.start_service("network")?;
                self.start_service("cloud-init")?;
                self.start_service("sshd")?;
            }
            SystemTarget::Realtime => {
                self.start_service("syslog")?;
                self.start_service("realtime-scheduler")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceError {
    NotFound,
    AlreadyRunning,
    AlreadyStopped,
    DependencyFailed,
    StartFailed,
    StopFailed,
}

/// Main SigmaInit manager
pub struct SigmaInit {
    supervisor: Supervisor,
    current_target: SystemTarget,
    boot_complete: AtomicBool,
}

impl SigmaInit {
    pub fn new() -> Self {
        Self {
            supervisor: Supervisor::new(),
            current_target: SystemTarget::MultiUser,
            boot_complete: AtomicBool::new(false),
        }
    }
    
    pub fn load_services(&mut self, services: Vec<Service>) {
        for service in services {
            self.supervisor.add_service(service);
        }
    }
    
    pub fn boot(&mut self, target: SystemTarget) -> Result<(), ServiceError> {
        self.current_target = target;
        
        // Perform topological sort and start services in order
        let order = self.supervisor.dependency_graph.topological_sort()
            .map_err(|_| ServiceError::DependencyFailed)?;
        
        for service_name in order {
            if let Err(e) = self.supervisor.start_service(&service_name) {
                return Err(e);
            }
        }
        
        self.boot_complete.store(true, Ordering::SeqCst);
        Ok(())
    }
    
    pub fn shutdown(&mut self) -> Result<(), ServiceError> {
        // Stop services in reverse dependency order
        let order = self.supervisor.dependency_graph.topological_sort()
            .map_err(|_| ServiceError::DependencyFailed)?;
        
        for service_name in order.into_iter().rev() {
            let _ = self.supervisor.stop_service(&service_name);
        }
        
        self.boot_complete.store(false, Ordering::SeqCst);
        Ok(())
    }
    
    pub fn is_boot_complete(&self) -> bool {
        self.boot_complete.load(Ordering::SeqCst)
    }
    
    pub fn switch_target(&mut self, target: SystemTarget) -> Result<(), ServiceError> {
        self.supervisor.start_target(target)?;
        self.current_target = target;
        Ok(())
    }
}

impl Default for SigmaInit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        let service = Service::new("test-service")
            .with_description("Test service")
            .with_command(vec![String::from("/bin/test")])
            .with_restart_policy(RestartPolicy::Always);
        
        assert_eq!(service.name, "test-service");
        assert_eq!(service.restart_policy, RestartPolicy::Always);
    }

    #[test]
    fn test_dependency_graph() {
        let mut graph = DependencyGraph::new();
        graph.add_service("network", vec![]);
        graph.add_service("sshd", vec![String::from("network")]);
        
        let deps = graph.get_dependencies("sshd");
        assert!(deps.is_some());
        assert_eq!(deps.unwrap(), &vec![String::from("network")]);
    }

    #[test]
    fn test_topological_sort() {
        let mut graph = DependencyGraph::new();
        graph.add_service("network", vec![]);
        graph.add_service("syslog", vec![]);
        graph.add_service("sshd", vec![String::from("network"), String::from("syslog")]);
        
        let order = graph.topological_sort().unwrap();
        assert!(order.len() == 3);
        // network and syslog should come before sshd
        let sshd_idx = order.iter().position(|x| x == "sshd").unwrap();
        let network_idx = order.iter().position(|x| x == "network").unwrap();
        let syslog_idx = order.iter().position(|x| x == "syslog").unwrap();
        assert!(sshd_idx > network_idx);
        assert!(sshd_idx > syslog_idx);
    }

    #[test]
    fn test_supervisor() {
        let mut supervisor = Supervisor::new();
        
        let network = Service::new("network")
            .with_command(vec![String::from("/bin/network")]);
        
        let sshd = Service::new("sshd")
            .with_command(vec![String::from("/bin/sshd")])
            .with_depends(vec![String::from("network")]);
        
        supervisor.add_service(network);
        supervisor.add_service(sshd);
        
        assert!(supervisor.start_service("sshd").is_ok());
        assert_eq!(supervisor.get_service_state("network"), Some(ServiceState::Running));
        assert_eq!(supervisor.get_service_state("sshd"), Some(ServiceState::Running));
    }

    #[test]
    fn test_sigmainit_boot() {
        let mut init = SigmaInit::new();
        
        let syslog = Service::new("syslog")
            .with_command(vec![String::from("/bin/syslog")]);
        
        let network = Service::new("network")
            .with_command(vec![String::from("/bin/network")])
            .with_depends(vec![String::from("syslog")]);
        
        init.load_services(vec![syslog, network]);
        
        assert!(init.boot(SystemTarget::MultiUser).is_ok());
        assert!(init.is_boot_complete());
    }
}