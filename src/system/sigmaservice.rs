// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaService - System Service Manager
//!
//! SigmaService provides system service management for SigmaOS, inspired by systemd/systemctl from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::fmt;

/// Service state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    /// Running
    Running,
    /// Stopped
    Stopped,
    /// Starting
    Starting,
    /// Stopping
    Stopping,
    /// Failed
    Failed,
    /// Reloading
    Reloading,
}

impl fmt::Display for ServiceState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceState::Running => write!(f, "running"),
            ServiceState::Stopped => write!(f, "stopped"),
            ServiceState::Starting => write!(f, "starting"),
            ServiceState::Stopping => write!(f, "stopping"),
            ServiceState::Failed => write!(f, "failed"),
            ServiceState::Reloading => write!(f, "reloading"),
        }
    }
}

/// Service type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceType {
    /// Simple service
    Simple,
    /// Forking service
    Forking,
    /// Oneshot service
    Oneshot,
    /// Notify service
    Notify,
}

impl fmt::Display for ServiceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceType::Simple => write!(f, "simple"),
            ServiceType::Forking => write!(f, "forking"),
            ServiceType::Oneshot => write!(f, "oneshot"),
            ServiceType::Notify => write!(f, "notify"),
        }
    }
}

/// Service
#[derive(Debug, Clone)]
pub struct Service {
    /// Service name
    pub name: String,
    /// Service description
    pub description: String,
    /// Service type
    pub service_type: ServiceType,
    /// Service state
    pub state: ServiceState,
    /// Executable path
    pub exec_path: String,
    /// Arguments
    pub args: Vec<String>,
    /// Dependencies
    pub dependencies: Vec<String>,
    /// PID
    pub pid: u32,
}

impl Service {
    /// Create a new service
    pub fn new(name: String, description: String, service_type: ServiceType, exec_path: String) -> Self {
        Self {
            name,
            description,
            service_type,
            state: ServiceState::Stopped,
            exec_path,
            args: Vec::new(),
            dependencies: Vec::new(),
            pid: 0,
        }
    }

    /// Set state
    pub fn with_state(mut self, state: ServiceState) -> Self {
        self.state = state;
        self
    }

    /// Set arguments
    pub fn with_args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }

    /// Set dependencies
    pub fn with_dependencies(mut self, dependencies: Vec<String>) -> Self {
        self.dependencies = dependencies;
        self
    }

    /// Set PID
    pub fn with_pid(mut self, pid: u32) -> Self {
        self.pid = pid;
        self
    }
}

/// SigmaService - System Service Manager
pub struct SigmaService {
    /// Services (name -> service)
    services: BTreeMap<String, Service>,
    /// Running state
    running: bool,
    /// Service counter
    service_counter: u32,
}

impl SigmaService {
    /// Create a new SigmaService instance
    pub fn new() -> Self {
        Self {
            services: BTreeMap::new(),
            running: false,
            service_counter: 0,
        }
    }

    /// Start the service manager
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the service manager
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Add a service
    pub fn add_service(&mut self, service: Service) -> Result<(), String> {
        if self.services.contains_key(&service.name) {
            return Err(format!("Service '{}' already exists", service.name));
        }
        
        self.services.insert(service.name.clone(), service);
        self.service_counter += 1;
        Ok(())
    }

    /// Remove a service
    pub fn remove_service(&mut self, name: &str) -> Result<(), String> {
        if self.services.remove(name).is_none() {
            return Err(format!("Service '{}' not found", name));
        }
        Ok(())
    }

    /// Get a service
    pub fn get_service(&self, name: &str) -> Option<&Service> {
        self.services.get(name)
    }

    /// Start a service
    pub fn start_service(&mut self, name: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaService not running".to_string());
        }
        
        match self.services.get_mut(name) {
            Some(service) => {
                service.state = ServiceState::Starting;
                service.state = ServiceState::Running;
                Ok(())
            }
            None => Err(format!("Service '{}' not found", name)),
        }
    }

    /// Stop a service
    pub fn stop_service(&mut self, name: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaService not running".to_string());
        }
        
        match self.services.get_mut(name) {
            Some(service) => {
                service.state = ServiceState::Stopping;
                service.state = ServiceState::Stopped;
                Ok(())
            }
            None => Err(format!("Service '{}' not found", name)),
        }
    }

    /// Restart a service
    pub fn restart_service(&mut self, name: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaService not running".to_string());
        }
        
        match self.services.get_mut(name) {
            Some(service) => {
                service.state = ServiceState::Stopping;
                service.state = ServiceState::Starting;
                service.state = ServiceState::Running;
                Ok(())
            }
            None => Err(format!("Service '{}' not found", name)),
        }
    }

    /// Reload a service
    pub fn reload_service(&mut self, name: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaService not running".to_string());
        }
        
        match self.services.get_mut(name) {
            Some(service) => {
                service.state = ServiceState::Reloading;
                service.state = ServiceState::Running;
                Ok(())
            }
            None => Err(format!("Service '{}' not found", name)),
        }
    }

    /// List all services
    pub fn list_services(&self) -> Vec<&Service> {
        self.services.values().collect()
    }

    /// List running services
    pub fn list_running(&self) -> Vec<&Service> {
        self.services.values()
            .filter(|s| s.state == ServiceState::Running)
            .collect()
    }

    /// List failed services
    pub fn list_failed(&self) -> Vec<&Service> {
        self.services.values()
            .filter(|s| s.state == ServiceState::Failed)
            .collect()
    }

    /// Get service count
    pub fn service_count(&self) -> usize {
        self.services.len()
    }

    /// Get service manager statistics
    pub fn service_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert("total_services".to_string(), self.service_count().to_string());
        stats.insert("running_services".to_string(), self.list_running().len().to_string());
        stats.insert("failed_services".to_string(), self.list_failed().len().to_string());
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
}

impl Default for SigmaService {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaService instance (placeholder for actual global state)
static mut SIGMASERVICE_INSTANCE: Option<SigmaService> = None;

/// Initialize the global SigmaService instance
pub fn init_sigmaservice() {
    unsafe {
        SIGMASERVICE_INSTANCE = Some(SigmaService::new());
    }
}

/// Get the global SigmaService instance
pub fn get_sigmaservice() -> Option<&'static mut SigmaService> {
    unsafe { SIGMASERVICE_INSTANCE.as_mut() }
}

/// Convenience function to start a service
pub fn start_service(name: &str) -> Result<(), String> {
    if let Some(service) = get_sigmaservice() {
        service.start_service(name)
    } else {
        Err("SigmaService not initialized".to_string())
    }
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
    fn test_service() {
        let service = Service::new(
            "nginx".to_string(),
            "Web server".to_string(),
            ServiceType::Simple,
            "/usr/bin/nginx".to_string()
        )
        .with_state(ServiceState::Running)
        .with_args(vec!["-g".to_string(), "daemon off;".to_string()])
        .with_pid(1234);

        assert_eq!(service.name, "nginx");
        assert_eq!(service.state, ServiceState::Running);
        assert_eq!(service.pid, 1234);
    }

    #[test]
    fn test_sigmaservice() {
        let mut service = SigmaService::new();
        assert!(!service.is_running());
        service.start();
        assert!(service.is_running());
        service.stop();
        assert!(!service.is_running());
    }

    #[test]
    fn test_sigmaservice_add_service() {
        let mut service = SigmaService::new();
        
        let svc = Service::new(
            "nginx".to_string(),
            "Web server".to_string(),
            ServiceType::Simple,
            "/usr/bin/nginx".to_string()
        );
        
        assert!(service.add_service(svc).is_ok());
        assert_eq!(service.service_count(), 1);
    }

    #[test]
    fn test_sigmaservice_start_service() {
        let mut service = SigmaService::new();
        service.start();
        
        let svc = Service::new(
            "nginx".to_string(),
            "Web server".to_string(),
            ServiceType::Simple,
            "/usr/bin/nginx".to_string()
        );
        
        service.add_service(svc).unwrap();
        assert!(service.start_service("nginx").is_ok());
        
        let svc = service.get_service("nginx").unwrap();
        assert_eq!(svc.state, ServiceState::Running);
    }

    #[test]
    fn test_sigmaservice_stop_service() {
        let mut service = SigmaService::new();
        service.start();
        
        let svc = Service::new(
            "nginx".to_string(),
            "Web server".to_string(),
            ServiceType::Simple,
            "/usr/bin/nginx".to_string()
        )
        .with_state(ServiceState::Running);
        
        service.add_service(svc).unwrap();
        assert!(service.stop_service("nginx").is_ok());
        
        let svc = service.get_service("nginx").unwrap();
        assert_eq!(svc.state, ServiceState::Stopped);
    }

    #[test]
    fn test_sigmaservice_restart_service() {
        let mut service = SigmaService::new();
        service.start();
        
        let svc = Service::new(
            "nginx".to_string(),
            "Web server".to_string(),
            ServiceType::Simple,
            "/usr/bin/nginx".to_string()
        )
        .with_state(ServiceState::Running);
        
        service.add_service(svc).unwrap();
        assert!(service.restart_service("nginx").is_ok());
        
        let svc = service.get_service("nginx").unwrap();
        assert_eq!(svc.state, ServiceState::Running);
    }

    #[test]
    fn test_sigmaservice_list_running() {
        let mut service = SigmaService::new();
        
        let svc = Service::new(
            "nginx".to_string(),
            "Web server".to_string(),
            ServiceType::Simple,
            "/usr/bin/nginx".to_string()
        )
        .with_state(ServiceState::Running);
        
        service.add_service(svc).unwrap();
        
        let running = service.list_running();
        assert_eq!(running.len(), 1);
    }

    #[test]
    fn test_sigmaservice_stats() {
        let mut service = SigmaService::new();
        
        let svc = Service::new(
            "nginx".to_string(),
            "Web server".to_string(),
            ServiceType::Simple,
            "/usr/bin/nginx".to_string()
        )
        .with_state(ServiceState::Running);
        
        service.add_service(svc).unwrap();
        
        let stats = service.service_stats();
        assert_eq!(stats.get("total_services"), Some(&"1".to_string()));
        assert_eq!(stats.get("running_services"), Some(&"1".to_string()));
    }

    #[test]
    fn test_start_service_convenience() {
        init_sigmaservice();
        
        let mut service = get_sigmaservice().unwrap();
        service.start();
        
        let svc = Service::new(
            "nginx".to_string(),
            "Web server".to_string(),
            ServiceType::Simple,
            "/usr/bin/nginx".to_string()
        );
        
        service.add_service(svc).unwrap();
        
        let result = start_service("nginx");
        assert!(result.is_ok());
    }
}
