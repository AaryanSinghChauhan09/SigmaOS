// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaInit - Service Manager
//!
//! SigmaInit is a service manager for SigmaOS, inspired by systemd.
//! It provides service management, dependency resolution, and system state management.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Service states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    /// Service is not loaded
    Unloaded,
    /// Service is loaded but not active
    Loaded,
    /// Service is starting
    Starting,
    /// Service is running
    Running,
    /// Service is stopping
    Stopping,
    /// Service is stopped
    Stopped,
    /// Service failed to start
    Failed,
    /// Service is reloading
    Reloading,
}

impl fmt::Display for ServiceState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceState::Unloaded => write!(f, "unloaded"),
            ServiceState::Loaded => write!(f, "loaded"),
            ServiceState::Starting => write!(f, "starting"),
            ServiceState::Running => write!(f, "running"),
            ServiceState::Stopping => write!(f, "stopping"),
            ServiceState::Stopped => write!(f, "stopped"),
            ServiceState::Failed => write!(f, "failed"),
            ServiceState::Reloading => write!(f, "reloading"),
        }
    }
}

/// Service type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceType {
    /// Simple service (forking not supported)
    Simple,
    /// Forking service (daemon forks itself)
    Forking,
    /// Oneshot service (runs once and exits)
    Oneshot,
    /// DBus service (DBus-activated)
    Dbus,
    /// Notify service (uses sd_notify)
    Notify,
    /// Idle service (activated when idle)
    Idle,
}

impl fmt::Display for ServiceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceType::Simple => write!(f, "simple"),
            ServiceType::Forking => write!(f, "forking"),
            ServiceType::Oneshot => write!(f, "oneshot"),
            ServiceType::Dbus => write!(f, "dbus"),
            ServiceType::Notify => write!(f, "notify"),
            ServiceType::Idle => write!(f, "idle"),
        }
    }
}

/// Service restart policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestartPolicy {
    /// Never restart on failure
    Never,
    /// Always restart on failure
    Always,
    /// Restart on failure up to N times
    OnFailure(u32),
    /// Restart on success
    OnSuccess,
}

impl fmt::Display for RestartPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RestartPolicy::Never => write!(f, "never"),
            RestartPolicy::Always => write!(f, "always"),
            RestartPolicy::OnFailure(n) => write!(f, "on-failure:{}", n),
            RestartPolicy::OnSuccess => write!(f, "on-success"),
        }
    }
}

/// Service definition
#[derive(Debug, Clone)]
pub struct Service {
    /// Service name
    pub name: String,
    /// Service description
    pub description: String,
    /// Service type
    pub service_type: ServiceType,
    /// Executable path
    pub exec_path: String,
    /// Executable arguments
    pub exec_args: Vec<String>,
    /// Working directory
    pub working_dir: Option<String>,
    /// Environment variables
    pub environment: BTreeMap<String, String>,
    /// Dependencies (required services)
    pub dependencies: Vec<String>,
    /// Wants (soft dependencies)
    pub wants: Vec<String>,
    /// After (services to start after)
    pub after: Vec<String>,
    /// Before (services to start before)
    pub before: Vec<String>,
    /// Restart policy
    pub restart_policy: RestartPolicy,
    /// Restart delay in seconds
    pub restart_delay: u32,
    /// Maximum restart count
    pub max_restarts: u32,
    /// Current restart count
    pub restart_count: u32,
    /// Required for boot
    pub required_for_boot: bool,
    /// Service state
    pub state: ServiceState,
    /// PID (if running)
    pub pid: Option<u32>,
    /// Exit code (if stopped/failed)
    pub exit_code: Option<i32>,
    /// Service enabled at boot
    pub enabled: bool,
}

impl Service {
    /// Create a new service
    pub fn new(name: String, description: String, exec_path: String) -> Self {
        Self {
            name,
            description,
            service_type: ServiceType::Simple,
            exec_path,
            exec_args: Vec::new(),
            working_dir: None,
            environment: BTreeMap::new(),
            dependencies: Vec::new(),
            wants: Vec::new(),
            after: Vec::new(),
            before: Vec::new(),
            restart_policy: RestartPolicy::Never,
            restart_delay: 5,
            max_restarts: 5,
            restart_count: 0,
            required_for_boot: false,
            state: ServiceState::Unloaded,
            pid: None,
            exit_code: None,
            enabled: false,
        }
    }

    /// Set service type
    pub fn with_service_type(mut self, service_type: ServiceType) -> Self {
        self.service_type = service_type;
        self
    }

    /// Add executable argument
    pub fn with_arg(mut self, arg: String) -> Self {
        self.exec_args.push(arg);
        self
    }

    /// Set working directory
    pub fn with_working_dir(mut self, dir: String) -> Self {
        self.working_dir = Some(dir);
        self
    }

    /// Add environment variable
    pub fn with_env(mut self, key: String, value: String) -> Self {
        self.environment.insert(key, value);
        self
    }

    /// Add dependency
    pub fn with_dependency(mut self, dep: String) -> Self {
        self.dependencies.push(dep);
        self
    }

    /// Add want (soft dependency)
    pub fn with_want(mut self, want: String) -> Self {
        self.wants.push(want);
        self
    }

    /// Add after dependency
    pub fn with_after(mut self, after: String) -> Self {
        self.after.push(after);
        self
    }

    /// Add before dependency
    pub fn with_before(mut self, before: String) -> Self {
        self.before.push(before);
        self
    }

    /// Set restart policy
    pub fn with_restart_policy(mut self, policy: RestartPolicy) -> Self {
        self.restart_policy = policy;
        self
    }

    /// Set required for boot
    pub fn with_required_for_boot(mut self, required: bool) -> Self {
        self.required_for_boot = required;
        self
    }

    /// Set enabled at boot
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// System target (similar to systemd target)
#[derive(Debug, Clone)]
pub struct Target {
    /// Target name
    pub name: String,
    /// Target description
    pub description: String,
    /// Services to include in target
    pub services: Vec<String>,
    /// Dependencies
    pub dependencies: Vec<String>,
    /// Default target for this target
    pub default_to: Option<String>,
}

impl Target {
    /// Create a new target
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            services: Vec::new(),
            dependencies: Vec::new(),
            default_to: None,
        }
    }

    /// Add service to target
    pub fn with_service(mut self, service: String) -> Self {
        self.services.push(service);
        self
    }

    /// Add dependency
    pub fn with_dependency(mut self, dep: String) -> Self {
        self.dependencies.push(dep);
        self
    }

    /// Set default target
    pub fn with_default_to(mut self, target: String) -> Self {
        self.default_to = Some(target);
        self
    }
}

/// System state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemState {
    /// System is booting
    Booting,
    /// System is running
    Running,
    /// System is shutting down
    ShuttingDown,
    /// System is in maintenance mode
    Maintenance,
}

impl fmt::Display for SystemState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SystemState::Booting => write!(f, "booting"),
            SystemState::Running => write!(f, "running"),
            SystemState::ShuttingDown => write!(f, "shutting-down"),
            SystemState::Maintenance => write!(f, "maintenance"),
        }
    }
}

/// SigmaInit - Service Manager
pub struct SigmaInit {
    /// Registered services
    services: BTreeMap<String, Service>,
    /// Registered targets
    targets: BTreeMap<String, Target>,
    /// Current system state
    system_state: SystemState,
    /// Current target
    current_target: Option<String>,
    /// Running state
    running: bool,
}

impl SigmaInit {
    /// Create a new SigmaInit instance
    pub fn new() -> Self {
        Self {
            services: BTreeMap::new(),
            targets: BTreeMap::new(),
            system_state: SystemState::Booting,
            current_target: None,
            running: false,
        }
    }

    /// Register a service
    pub fn register_service(&mut self, service: Service) {
        self.services.insert(service.name.clone(), service);
    }

    /// Register a target
    pub fn register_target(&mut self, target: Target) {
        self.targets.insert(target.name.clone(), target);
    }

    /// Get a service by name
    pub fn get_service(&self, name: &str) -> Option<&Service> {
        self.services.get(name)
    }

    /// Get a mutable service by name
    pub fn get_service_mut(&mut self, name: &str) -> Option<&mut Service> {
        self.services.get_mut(name)
    }

    /// Get a target by name
    pub fn get_target(&self, name: &str) -> Option<&Target> {
        self.targets.get(name)
    }

    /// Start the service manager
    pub fn start(&mut self) {
        self.running = true;
        // In a real implementation, this would start a background thread
        // to manage services and handle events
    }

    /// Stop the service manager
    pub fn stop(&mut self) {
        self.running = false;
        // In a real implementation, this would gracefully shutdown all services
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Start a service
    pub fn start_service(&mut self, name: &str) -> Result<(), String> {
        let deps = match self.services.get(name) {
            Some(s) => s.dependencies.clone(),
            None => return Err(format!("Service '{}' not found", name)),
        };

        // Check dependencies
        for dep in &deps {
            if let Some(dep_service) = self.services.get(dep) {
                if dep_service.state != ServiceState::Running {
                    return Err(format!("Dependency '{}' is not running", dep));
                }
            }
        }

        let num_services = self.services.len();
        let service = self.services.get_mut(name)
            .ok_or_else(|| format!("Service '{}' not found", name))?;

        // Update state
        service.state = ServiceState::Starting;

        // In a real implementation, this would actually start the process
        // For now, simulate success
        service.state = ServiceState::Running;
        service.pid = Some(1000 + num_services as u32); // Placeholder PID
        service.exit_code = None;

        Ok(())
    }

    /// Stop a service
    pub fn stop_service(&mut self, name: &str) -> Result<(), String> {
        let service = self.services.get_mut(name)
            .ok_or_else(|| format!("Service '{}' not found", name))?;

        if service.state != ServiceState::Running {
            return Err(format!("Service '{}' is not running", name));
        }

        // Update state
        service.state = ServiceState::Stopping;

        // In a real implementation, this would send SIGTERM to the process
        // For now, simulate success
        service.state = ServiceState::Stopped;
        service.pid = None;
        service.exit_code = Some(0);

        Ok(())
    }

    /// Restart a service
    pub fn restart_service(&mut self, name: &str) -> Result<(), String> {
        self.stop_service(name)?;
        self.start_service(name)
    }

    /// Reload a service
    pub fn reload_service(&mut self, name: &str) -> Result<(), String> {
        let service = self.services.get_mut(name)
            .ok_or_else(|| format!("Service '{}' not found", name))?;

        if service.state != ServiceState::Running {
            return Err(format!("Service '{}' is not running", name));
        }

        // Update state
        service.state = ServiceState::Reloading;

        // In a real implementation, this would send SIGHUP to the process
        // For now, simulate success
        service.state = ServiceState::Running;

        Ok(())
    }

    /// Enable a service (start at boot)
    pub fn enable_service(&mut self, name: &str) -> Result<(), String> {
        let service = self.services.get_mut(name)
            .ok_or_else(|| format!("Service '{}' not found", name))?;

        service.enabled = true;
        Ok(())
    }

    /// Disable a service (don't start at boot)
    pub fn disable_service(&mut self, name: &str) -> Result<(), String> {
        let service = self.services.get_mut(name)
            .ok_or_else(|| format!("Service '{}' not found", name))?;

        service.enabled = false;
        Ok(())
    }

    /// Switch to a target
    pub fn switch_target(&mut self, target_name: &str) -> Result<(), String> {
        let (stop_services, start_services) = {
            let target = self.targets.get(target_name)
                .ok_or_else(|| format!("Target '{}' not found", target_name))?;

            let mut stop_list = Vec::new();
            if let Some(current) = &self.current_target {
                if let Some(current_target) = self.targets.get(current) {
                    stop_list = current_target.services.clone();
                }
            }
            (stop_list, target.services.clone())
        };

        for service_name in &stop_services {
            let _ = self.stop_service(service_name);
        }

        for service_name in &start_services {
            let _ = self.start_service(service_name);
        }

        self.current_target = Some(target_name.to_string());
        Ok(())
    }

    /// Get system state
    pub fn system_state(&self) -> SystemState {
        self.system_state
    }

    /// Set system state
    pub fn set_system_state(&mut self, state: SystemState) {
        self.system_state = state;
    }

    /// Get current target
    pub fn current_target(&self) -> Option<&str> {
        self.current_target.as_deref()
    }

    /// Get all services
    pub fn get_all_services(&self) -> Vec<&Service> {
        self.services.values().collect()
    }

    /// Get all targets
    pub fn get_all_targets(&self) -> Vec<&Target> {
        self.targets.values().collect()
    }

    /// Get services by state
    pub fn get_services_by_state(&self, state: ServiceState) -> Vec<&Service> {
        self.services
            .values()
            .filter(|s| s.state == state)
            .collect()
    }

    /// Get enabled services
    pub fn get_enabled_services(&self) -> Vec<&Service> {
        self.services
            .values()
            .filter(|s| s.enabled)
            .collect()
    }

    /// Get failed services
    pub fn get_failed_services(&self) -> Vec<&Service> {
        self.services
            .values()
            .filter(|s| s.state == ServiceState::Failed)
            .collect()
    }
}

impl Default for SigmaInit {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaInit instance (placeholder for actual global state)
static mut SIGMAINIT_INSTANCE: Option<SigmaInit> = None;

/// Initialize the global SigmaInit instance
pub fn init_sigmainit() {
    unsafe {
        SIGMAINIT_INSTANCE = Some(SigmaInit::new());
    }
}

/// Get the global SigmaInit instance
pub fn get_sigmainit() -> Option<&'static mut SigmaInit> {
    unsafe { SIGMAINIT_INSTANCE.as_mut() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_state_display() {
        assert_eq!(ServiceState::Running.to_string(), "running");
        assert_eq!(ServiceState::Stopped.to_string(), "stopped");
    }

    #[test]
    fn test_service_type_display() {
        assert_eq!(ServiceType::Simple.to_string(), "simple");
        assert_eq!(ServiceType::Forking.to_string(), "forking");
    }

    #[test]
    fn test_restart_policy_display() {
        assert_eq!(RestartPolicy::Never.to_string(), "never");
        assert_eq!(RestartPolicy::Always.to_string(), "always");
        assert_eq!(RestartPolicy::OnFailure(3).to_string(), "on-failure:3");
    }

    #[test]
    fn test_service_creation() {
        let service = Service::new(
            "test".to_string(),
            "Test service".to_string(),
            "/usr/bin/test".to_string()
        )
        .with_service_type(ServiceType::Simple)
        .with_enabled(true);

        assert_eq!(service.name, "test");
        assert_eq!(service.service_type, ServiceType::Simple);
        assert_eq!(service.enabled, true);
    }

    #[test]
    fn test_service_with_args() {
        let service = Service::new(
            "test".to_string(),
            "Test service".to_string(),
            "/usr/bin/test".to_string()
        )
        .with_arg("--daemon".to_string())
        .with_arg("--config".to_string())
        .with_arg("/etc/test.conf".to_string());

        assert_eq!(service.exec_args.len(), 3);
        assert_eq!(service.exec_args[0], "--daemon");
    }

    #[test]
    fn test_service_with_env() {
        let service = Service::new(
            "test".to_string(),
            "Test service".to_string(),
            "/usr/bin/test".to_string()
        )
        .with_env("PATH".to_string(), "/usr/bin".to_string())
        .with_env("HOME".to_string(), "/root".to_string());

        assert_eq!(service.environment.len(), 2);
        assert_eq!(service.environment.get("PATH"), Some(&"/usr/bin".to_string()));
    }

    #[test]
    fn test_target_creation() {
        let target = Target::new(
            "multi-user".to_string(),
            "Multi-user target".to_string()
        )
        .with_service("network".to_string())
        .with_service("syslog".to_string());

        assert_eq!(target.name, "multi-user");
        assert_eq!(target.services.len(), 2);
    }

    #[test]
    fn test_sigmainit_creation() {
        let init = SigmaInit::new();
        assert!(!init.is_running());
        init.start();
        assert!(init.is_running());
        init.stop();
        assert!(!init.is_running());
    }

    #[test]
    fn test_service_registration() {
        let mut init = SigmaInit::new();
        let service = Service::new(
            "test".to_string(),
            "Test service".to_string(),
            "/usr/bin/test".to_string()
        );

        init.register_service(service);
        assert!(init.get_service("test").is_some());
    }

    #[test]
    fn test_target_registration() {
        let mut init = SigmaInit::new();
        let target = Target::new(
            "multi-user".to_string(),
            "Multi-user target".to_string()
        );

        init.register_target(target);
        assert!(init.get_target("multi-user").is_some());
    }

    #[test]
    fn test_start_service() {
        let mut init = SigmaInit::new();
        let service = Service::new(
            "test".to_string(),
            "Test service".to_string(),
            "/usr/bin/test".to_string()
        );

        init.register_service(service);
        let result = init.start_service("test");
        assert!(result.is_ok());

        let service = init.get_service("test").unwrap();
        assert_eq!(service.state, ServiceState::Running);
        assert!(service.pid.is_some());
    }

    #[test]
    fn test_stop_service() {
        let mut init = SigmaInit::new();
        let service = Service::new(
            "test".to_string(),
            "Test service".to_string(),
            "/usr/bin/test".to_string()
        );

        init.register_service(service);
        init.start_service("test").unwrap();
        let result = init.stop_service("test");
        assert!(result.is_ok());

        let service = init.get_service("test").unwrap();
        assert_eq!(service.state, ServiceState::Stopped);
        assert!(service.pid.is_none());
    }

    #[test]
    fn test_restart_service() {
        let mut init = SigmaInit::new();
        let service = Service::new(
            "test".to_string(),
            "Test service".to_string(),
            "/usr/bin/test".to_string()
        );

        init.register_service(service);
        init.start_service("test").unwrap();
        let result = init.restart_service("test");
        assert!(result.is_ok());

        let service = init.get_service("test").unwrap();
        assert_eq!(service.state, ServiceState::Running);
    }

    #[test]
    fn test_enable_disable_service() {
        let mut init = SigmaInit::new();
        let service = Service::new(
            "test".to_string(),
            "Test service".to_string(),
            "/usr/bin/test".to_string()
        );

        init.register_service(service);
        init.enable_service("test").unwrap();

        let service = init.get_service("test").unwrap();
        assert!(service.enabled);

        init.disable_service("test").unwrap();

        let service = init.get_service("test").unwrap();
        assert!(!service.enabled);
    }

    #[test]
    fn test_get_services_by_state() {
        let mut init = SigmaInit::new();
        
        let service1 = Service::new(
            "test1".to_string(),
            "Test service 1".to_string(),
            "/usr/bin/test1".to_string()
        );
        let service2 = Service::new(
            "test2".to_string(),
            "Test service 2".to_string(),
            "/usr/bin/test2".to_string()
        );

        init.register_service(service1);
        init.register_service(service2);
        init.start_service("test1").unwrap();

        let running = init.get_services_by_state(ServiceState::Running);
        assert_eq!(running.len(), 1);
        assert_eq!(running[0].name, "test1");
    }

    #[test]
    fn test_system_state() {
        let init = SigmaInit::new();
        assert_eq!(init.system_state(), SystemState::Booting);
        assert_eq!(init.system_state().to_string(), "booting");
    }
}
