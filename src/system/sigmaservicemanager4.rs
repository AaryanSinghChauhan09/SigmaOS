// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Aaryan Singh Chauhan

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Service state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceState {
    Stopped,
    Running,
    Failed,
    Restarting,
}

/// Service configuration
#[derive(Debug, Clone)]
pub struct Service {
    pub id: String,
    pub name: String,
    pub description: String,
    pub command: String,
    pub state: ServiceState,
    pub auto_start: bool,
    pub dependencies: Vec<String>,
}

/// Service manager for managing system services
/// Inspired by Omarchy's restart- commands and systemd service management
#[derive(Debug, Clone)]
pub struct SigmaServiceManager4 {
    services: Vec<Service>,
}

impl SigmaServiceManager4 {
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
        }
    }

    /// Register a new service
    pub fn register_service(&mut self, service: Service) {
        self.services.push(service);
    }

    /// Get service by ID
    pub fn get_service(&self, id: &str) -> Option<&Service> {
        self.services.iter().find(|s| s.id == id)
    }

    /// Get all services
    pub fn get_services(&self) -> &[Service] {
        &self.services
    }

    /// Start a service
    pub fn start_service(&mut self, id: &str) -> Result<String, String> {
        if let Some(service) = self.services.iter_mut().find(|s| s.id == id) {
            service.state = ServiceState::Running;
            Ok(format!("Started service: {}", service.name))
        } else {
            Err(format!("Service not found: {}", id))
        }
    }

    /// Stop a service
    pub fn stop_service(&mut self, id: &str) -> Result<String, String> {
        if let Some(service) = self.services.iter_mut().find(|s| s.id == id) {
            service.state = ServiceState::Stopped;
            Ok(format!("Stopped service: {}", service.name))
        } else {
            Err(format!("Service not found: {}", id))
        }
    }

    /// Restart a service
    pub fn restart_service(&mut self, id: &str) -> Result<String, String> {
        if let Some(service) = self.services.iter_mut().find(|s| s.id == id) {
            service.state = ServiceState::Restarting;
            Ok(format!("Restarting service: {}", service.name))
        } else {
            Err(format!("Service not found: {}", id))
        }
    }

    /// Enable auto-start for a service
    pub fn enable_service(&mut self, id: &str) -> Result<String, String> {
        if let Some(service) = self.services.iter_mut().find(|s| s.id == id) {
            service.auto_start = true;
            Ok(format!("Enabled auto-start for service: {}", service.name))
        } else {
            Err(format!("Service not found: {}", id))
        }
    }

    /// Disable auto-start for a service
    pub fn disable_service(&mut self, id: &str) -> Result<String, String> {
        if let Some(service) = self.services.iter_mut().find(|s| s.id == id) {
            service.auto_start = false;
            Ok(format!("Disabled auto-start for service: {}", service.name))
        } else {
            Err(format!("Service not found: {}", id))
        }
    }

    /// Get running services
    pub fn get_running_services(&self) -> Vec<&Service> {
        self.services
            .iter()
            .filter(|s| s.state == ServiceState::Running)
            .collect()
    }

    /// Get auto-start services
    pub fn get_auto_start_services(&self) -> Vec<&Service> {
        self.services
            .iter()
            .filter(|s| s.auto_start)
            .collect()
    }

    /// Check if service dependencies are satisfied
    pub fn check_service_dependencies(&self, id: &str) -> Result<bool, String> {
        if let Some(service) = self.get_service(id) {
            let missing: Vec<_> = service
                .dependencies
                .iter()
                .filter(|dep| !self.services.iter().any(|s| &s.id == *dep))
                .collect();
            if missing.is_empty() {
                Ok(true)
            } else {
                Err(format!("Missing service dependencies: {:?}", missing))
            }
        } else {
            Err(format!("Service not found: {}", id))
        }
    }
}

impl Default for SigmaServiceManager4 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_service() {
        let mut manager = SigmaServiceManager4::new();
        let service = Service {
            id: "test-service".to_string(),
            name: "Test Service".to_string(),
            description: "A test service".to_string(),
            command: "/usr/bin/test-service".to_string(),
            state: ServiceState::Stopped,
            auto_start: false,
            dependencies: Vec::new(),
        };
        manager.register_service(service);
        assert_eq!(manager.get_services().len(), 1);
    }

    #[test]
    fn test_start_service() {
        let mut manager = SigmaServiceManager4::new();
        let service = Service {
            id: "test-service".to_string(),
            name: "Test Service".to_string(),
            description: "A test service".to_string(),
            command: "/usr/bin/test-service".to_string(),
            state: ServiceState::Stopped,
            auto_start: false,
            dependencies: Vec::new(),
        };
        manager.register_service(service);
        manager.start_service("test-service").unwrap();
        assert_eq!(
            manager.get_service("test-service").unwrap().state,
            ServiceState::Running
        );
    }

    #[test]
    fn test_stop_service() {
        let mut manager = SigmaServiceManager4::new();
        let service = Service {
            id: "test-service".to_string(),
            name: "Test Service".to_string(),
            description: "A test service".to_string(),
            command: "/usr/bin/test-service".to_string(),
            state: ServiceState::Running,
            auto_start: false,
            dependencies: Vec::new(),
        };
        manager.register_service(service);
        manager.stop_service("test-service").unwrap();
        assert_eq!(
            manager.get_service("test-service").unwrap().state,
            ServiceState::Stopped
        );
    }

    #[test]
    fn test_restart_service() {
        let mut manager = SigmaServiceManager4::new();
        let service = Service {
            id: "test-service".to_string(),
            name: "Test Service".to_string(),
            description: "A test service".to_string(),
            command: "/usr/bin/test-service".to_string(),
            state: ServiceState::Running,
            auto_start: false,
            dependencies: Vec::new(),
        };
        manager.register_service(service);
        manager.restart_service("test-service").unwrap();
        assert_eq!(
            manager.get_service("test-service").unwrap().state,
            ServiceState::Restarting
        );
    }

    #[test]
    fn test_enable_service() {
        let mut manager = SigmaServiceManager4::new();
        let service = Service {
            id: "test-service".to_string(),
            name: "Test Service".to_string(),
            description: "A test service".to_string(),
            command: "/usr/bin/test-service".to_string(),
            state: ServiceState::Stopped,
            auto_start: false,
            dependencies: Vec::new(),
        };
        manager.register_service(service);
        manager.enable_service("test-service").unwrap();
        assert!(manager.get_service("test-service").unwrap().auto_start);
    }

    #[test]
    fn test_get_running_services() {
        let mut manager = SigmaServiceManager4::new();
        let service1 = Service {
            id: "service1".to_string(),
            name: "Service 1".to_string(),
            description: "Service 1".to_string(),
            command: "/usr/bin/service1".to_string(),
            state: ServiceState::Running,
            auto_start: false,
            dependencies: Vec::new(),
        };
        let service2 = Service {
            id: "service2".to_string(),
            name: "Service 2".to_string(),
            description: "Service 2".to_string(),
            command: "/usr/bin/service2".to_string(),
            state: ServiceState::Stopped,
            auto_start: false,
            dependencies: Vec::new(),
        };
        manager.register_service(service1);
        manager.register_service(service2);
        assert_eq!(manager.get_running_services().len(), 1);
    }

    #[test]
    fn test_check_service_dependencies() {
        let mut manager = SigmaServiceManager4::new();
        let dep = Service {
            id: "dep".to_string(),
            name: "Dependency".to_string(),
            description: "Dependency".to_string(),
            command: "/usr/bin/dep".to_string(),
            state: ServiceState::Running,
            auto_start: false,
            dependencies: Vec::new(),
        };
        let service = Service {
            id: "service".to_string(),
            name: "Service".to_string(),
            description: "Service".to_string(),
            command: "/usr/bin/service".to_string(),
            state: ServiceState::Stopped,
            auto_start: false,
            dependencies: vec!["dep".to_string()],
        };
        manager.register_service(dep);
        manager.register_service(service);
        assert!(manager.check_service_dependencies("service").is_ok());
    }
}
