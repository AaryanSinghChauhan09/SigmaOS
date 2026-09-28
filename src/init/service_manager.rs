//! Service Management System (OpenRC + runit + s6 + systemd + launchd + Capsicum/Pledge Inspiration)
//! Minimal Service Model conforming to SigmaOS Sovereign Userland Architecture

use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

/// Service unit types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceType {
    Service,
    Target,
    Socket,
    Timer,
    Path,
    Mount,
    Automount,
    Swap,
}

/// Service state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Failed,
    Restarting,
}

/// Restart policy (runit / systemd / s6 inspiration)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestartPolicy {
    Always,
    OnFailure,
    Never,
    UnlessStopped,
}

/// Health check specification
#[derive(Debug, Clone)]
pub struct HealthCheckSpec {
    pub liveness_cmd: String,
    pub interval_sec: u64,
    pub timeout_sec: u64,
    pub max_retries: u32,
}

impl HealthCheckSpec {
    pub fn new(cmd: &str, interval_sec: u64) -> Self {
        Self {
            liveness_cmd: cmd.to_string(),
            interval_sec,
            timeout_sec: 5,
            max_retries: 3,
        }
    }
}

/// Capsicum/Pledge security capability & privilege restrictions
#[derive(Debug, Clone)]
pub struct PrivilegeCapabilitySet {
    pub pledge_promises: String,
    pub unveil_paths: Vec<(String, String)>,
    pub allow_raw_sockets: bool,
    pub run_as_user: String,
    pub run_as_group: String,
}

impl PrivilegeCapabilitySet {
    pub fn new(user: &str) -> Self {
        Self {
            pledge_promises: "stdio rpath wpath cpath".to_string(),
            unveil_paths: Vec::new(),
            allow_raw_sockets: false,
            run_as_user: user.to_string(),
            run_as_group: user.to_string(),
        }
    }
}

/// Sandbox isolation profile
#[derive(Debug, Clone)]
pub struct SandboxProfile {
    pub read_only_root: bool,
    pub isolate_network: bool,
    pub private_tmp: bool,
    pub max_memory_mb: u64,
    pub max_cpu_percent: u8,
}

impl SandboxProfile {
    pub fn new() -> Self {
        Self {
            read_only_root: true,
            isolate_network: false,
            private_tmp: true,
            max_memory_mb: 256,
            max_cpu_percent: 50,
        }
    }
}

impl Default for SandboxProfile {
    fn default() -> Self {
        Self::new()
    }
}

/// Minimal Service Unit Specification
pub struct ServiceUnit {
    pub name: String,
    pub service_type: ServiceType,
    pub description: String,
    pub dependencies: Vec<String>,
    pub startup_order: u32,
    pub shutdown_order: u32,
    pub state: ServiceState,
    pub exec_start: Vec<String>,
    pub exec_stop: Vec<String>,
    pub environment: Vec<(String, String)>,
    pub restart_policy: RestartPolicy,
    pub health_check: Option<HealthCheckSpec>,
    pub privilege_set: PrivilegeCapabilitySet,
    pub sandbox: SandboxProfile,
}

impl ServiceUnit {
    pub fn new(name: &str, service_type: ServiceType) -> Self {
        Self {
            name: name.to_string(),
            service_type,
            description: String::new(),
            dependencies: Vec::new(),
            startup_order: 100,
            shutdown_order: 100,
            state: ServiceState::Stopped,
            exec_start: Vec::new(),
            exec_stop: Vec::new(),
            environment: Vec::new(),
            restart_policy: RestartPolicy::OnFailure,
            health_check: None,
            privilege_set: PrivilegeCapabilitySet::new("nobody"),
            sandbox: SandboxProfile::new(),
        }
    }

    pub fn set_description(&mut self, description: &str) {
        self.description = description.to_string();
    }

    pub fn add_dependency(&mut self, dependency: &str) {
        self.dependencies.push(dependency.to_string());
    }

    pub fn add_exec_start(&mut self, command: &str) {
        self.exec_start.push(command.to_string());
    }

    pub fn add_exec_stop(&mut self, command: &str) {
        self.exec_stop.push(command.to_string());
    }

    pub fn set_environment(&mut self, key: &str, value: &str) {
        self.environment.push((key.to_string(), value.to_string()));
    }

    pub fn start(&mut self) -> Result<(), ServiceError> {
        if self.state == ServiceState::Running {
            return Err(ServiceError::AlreadyRunning);
        }

        self.state = ServiceState::Starting;
        self.state = ServiceState::Running;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), ServiceError> {
        if self.state == ServiceState::Stopped {
            return Err(ServiceError::AlreadyStopped);
        }

        self.state = ServiceState::Stopping;
        self.state = ServiceState::Stopped;
        Ok(())
    }

    pub fn restart(&mut self) -> Result<(), ServiceError> {
        self.stop()?;
        self.start()
    }

    pub fn get_status(&self) -> ServiceStatus {
        ServiceStatus {
            name: self.name.clone(),
            state: self.state,
            dependencies: self.dependencies.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ServiceStatus {
    pub name: String,
    pub state: ServiceState,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceError {
    AlreadyRunning,
    AlreadyStopped,
    DependencyFailed,
    ExecutionFailed,
    NotFound,
}

/// Service manager with dependency resolution and OpenRC / runit semantics
pub struct ServiceManager {
    pub services: Vec<ServiceUnit>,
    pub targets: Vec<TargetUnit>,
}

impl ServiceManager {
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
            targets: Vec::new(),
        }
    }

    pub fn add_service(&mut self, service: ServiceUnit) {
        self.services.push(service);
    }

    pub fn add_target(&mut self, target: TargetUnit) {
        self.targets.push(target);
    }

    pub fn get_service(&mut self, name: &str) -> Option<&mut ServiceUnit> {
        self.services.iter_mut().find(|s| s.name == name)
    }

    pub fn start_service(&mut self, name: &str) -> Result<(), ServiceError> {
        if let Some(service) = self.get_service(name) {
            let deps = service.dependencies.clone();
            for dep in deps {
                if let Some(dep_service) = self.get_service(&dep) {
                    if dep_service.state != ServiceState::Running {
                        self.start_service(&dep)?;
                    }
                }
            }
            if let Some(srv) = self.get_service(name) {
                srv.start()
            } else {
                Err(ServiceError::NotFound)
            }
        } else {
            Err(ServiceError::NotFound)
        }
    }

    pub fn stop_service(&mut self, name: &str) -> Result<(), ServiceError> {
        if let Some(service) = self.get_service(name) {
            service.stop()
        } else {
            Err(ServiceError::NotFound)
        }
    }

    pub fn list_services(&self) -> Vec<&ServiceUnit> {
        self.services.iter().collect()
    }
}

impl Default for ServiceManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Target unit
pub struct TargetUnit {
    pub name: String,
    pub description: String,
    pub requires: Vec<String>,
    pub wanted_by: Vec<String>,
}

impl TargetUnit {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: String::new(),
            requires: Vec::new(),
            wanted_by: Vec::new(),
        }
    }

    pub fn set_description(&mut self, description: &str) {
        self.description = description.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimal_service_model_lifecycle() {
        let mut service = ServiceUnit::new("networkd", ServiceType::Service);
        service.set_description("Network daemon");
        service.add_exec_start("/usr/bin/networkd");
        service.restart_policy = RestartPolicy::Always;
        service.health_check = Some(HealthCheckSpec::new("/usr/bin/ping -c 1 127.0.0.1", 10));

        assert_eq!(service.state, ServiceState::Stopped);
        assert!(service.start().is_ok());
        assert_eq!(service.state, ServiceState::Running);
        assert_eq!(service.restart_policy, RestartPolicy::Always);

        let status = service.get_status();
        assert_eq!(status.name, "networkd");
        assert_eq!(status.state, ServiceState::Running);
    }

    #[test]
    fn test_service_manager_dependency_start() {
        let mut manager = ServiceManager::new();

        let mut dep_service = ServiceUnit::new("dbus", ServiceType::Service);
        dep_service.add_exec_start("/usr/bin/dbus-daemon");
        manager.add_service(dep_service);

        let mut app_service = ServiceUnit::new("desktop-shell", ServiceType::Service);
        app_service.add_dependency("dbus");
        app_service.add_exec_start("/usr/bin/shell");
        manager.add_service(app_service);

        assert!(manager.start_service("desktop-shell").is_ok());
        assert_eq!(manager.get_service("dbus").unwrap().state, ServiceState::Running);
        assert_eq!(manager.get_service("desktop-shell").unwrap().state, ServiceState::Running);
    }
}
