//! SigmaServiceManager - Service Management System
//!
//! Provides comprehensive service management inspired by Arch (systemd), Debian (systemd), and CachyOS.
//! Supports service start/stop, enable/disable, status tracking, and service dependencies.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Service state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Running,
    Stopped,
    Failed,
    Unknown,
;

/// Service type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceType {
    System,
    User,
    Socket,
    Timer,
    Path,
;

/// Service
#[derive(Debug, Clone)]
pub struct Service {
    pub id: String,
    pub name: String,
    pub description: String,
    pub service_type: ServiceType,
    pub state: ServiceState,
    pub enabled: bool,
    pub pid: Option<u32>,
    pub memory_usage: u64, // in bytes
    pub cpu_usage: f32, // percentage
    pub dependencies: Vec<String>,
;

impl Service {
    pub fn new(id: String, name: String, description: String, service_type: ServiceType) -> Self {
        Self {
            id,
            name,
            description,
            service_type,
            state: ServiceState::Unknown,
            enabled: false,
            pid: None,
            memory_usage: 0,
            cpu_usage: 0.0,
            dependencies: Vec::new(),
        }
    }

    pub fn set_state(&mut self, state: ServiceState) {
        self.state = state;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_pid(&mut self, pid: Option<u32>) {
        self.pid = pid;
    }

    pub fn set_memory_usage(&mut self, usage: u64) {
        self.memory_usage = usage;
    }

    pub fn set_cpu_usage(&mut self, usage: f32) {
        self.cpu_usage = usage;
    }

    pub fn add_dependency(&mut self, dep: String) {
        self.dependencies.push(dep);
    }

    pub fn is_running(&self) -> bool {
        self.state == ServiceState::Running
    }

    pub fn is_failed(&self) -> bool {
        self.state == ServiceState::Failed
    }
;

/// SigmaServiceManager - Service management
#[derive(Debug)]
pub struct SigmaServiceManager {
    services: BTreeMap<String, Service>,
;

impl SigmaServiceManager {
    pub fn new() -> Self {
        Self {
            services: BTreeMap::new(),
        }
    }

    pub fn add_service(&mut self, service: Service) {
        self.services.insert(service.id.clone(), service);
    }

    pub fn remove_service(&mut self, id: &str) -> bool {
        self.services.remove(id).is_some()
    }

    pub fn get_service(&self, id: &str) -> Option<&Service> {
        self.services.get(id)
    }

    pub fn get_service_mut(&mut self, id: &str) -> Option<&mut Service> {
        self.services.get_mut(id)
    }

    pub fn list_running(&self) -> Vec<&Service> {
        self.services.values()
            .filter(|svc| svc.is_running())
            .collect()
    }

    pub fn list_failed(&self) -> Vec<&Service> {
        self.services.values()
            .filter(|svc| svc.is_failed())
            .collect()
    }

    pub fn list_enabled(&self) -> Vec<&Service> {
        self.services.values()
            .filter(|svc| svc.enabled)
            .collect()
    }

    pub fn list_by_type(&self, service_type: ServiceType) -> Vec<&Service> {
        self.services.values()
            .filter(|svc| svc.service_type == service_type)
            .collect()
    }

    pub fn start_service(&mut self, id: &str) -> bool {
        if let Some(service) = self.services.get_mut(id) {
            service.set_state(ServiceState::Running);
            true
        } else {
            false
        }
    }

    pub fn stop_service(&mut self, id: &str) -> bool {
        if let Some(service) = self.services.get_mut(id) {
            service.set_state(ServiceState::Stopped);
            service.set_pid(None);
            true
        } else {
            false
        }
    }

    pub fn enable_service(&mut self, id: &str) -> bool {
        if let Some(service) = self.services.get_mut(id) {
            service.set_enabled(true);
            true
        } else {
            false
        }
    }

    pub fn disable_service(&mut self, id: &str) -> bool {
        if let Some(service) = self.services.get_mut(id) {
            service.set_enabled(false);
            true
        } else {
            false
        }
    }

    pub fn service_count(&self) -> usize {
        self.services.len()
    }
;

impl Default for SigmaServiceManager {
    fn default() -> Self {
        Self::new()
    }
;

/// Global SigmaServiceManager instance
static mut SIGMASERVICEMANAGER_INSTANCE: Option<SigmaServiceManager> = None;

/// Initialize the global SigmaServiceManager instance
pub fn init_sigmaservicemanager() {
    unsafe {
        SIGMASERVICEMANAGER_INSTANCE = Some(SigmaServiceManager::new());
    }
;

/// Get the global SigmaServiceManager instance
pub fn get_sigmaservicemanager() -> Option<&'static SigmaServiceManager> {
    unsafe {
        SIGMASERVICEMANAGER_INSTANCE.as_ref()
    }
;

/// Get the global SigmaServiceManager instance (mutable)
pub fn get_sigmaservicemanager_mut() -> Option<&'static mut SigmaServiceManager> {
    unsafe {
        SIGMASERVICEMANAGER_INSTANCE.as_mut()
    }
;

/// Convenience function to add a service
pub fn add_service(service: Service) -> bool {
    if let Some(manager) = get_sigmaservicemanager_mut() {
        manager.add_service(service);
        true
    } else {
        false
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        let service = Service::new(
            "ssh".to_string(),
            "ssh".to_string(),
            "OpenSSH server daemon".to_string(),
            ServiceType::System,
        );
        assert_eq!(service.id, "ssh");
        assert_eq!(service.service_type, ServiceType::System);
    }

    #[test]
    fn test_service_modification() {
        let mut service = Service::new(
            "nginx".to_string(),
            "nginx".to_string(),
            "Nginx web server".to_string(),
            ServiceType::System,
        );
        service.set_state(ServiceState::Running);
        service.set_enabled(true);
        service.set_pid(Some(1234));
        assert!(service.is_running());
        assert!(service.enabled);
    }

    #[test]
    fn test_sigmaservicemanager_creation() {
        let manager = SigmaServiceManager::new();
        assert_eq!(manager.service_count(), 0);
    }

    #[test]
    fn test_sigmaservicemanager_management() {
        let mut manager = SigmaServiceManager::new();
        let service = Service::new(
            "cron".to_string(),
            "cron".to_string(),
            "Regular background program processing daemon".to_string(),
            ServiceType::System,
        );
        manager.add_service(service);
        assert_eq!(manager.service_count(), 1);
    }

    #[test]
    fn test_service_start_stop() {
        let mut manager = SigmaServiceManager::new();
        let service = Service::new(
            "test".to_string(),
            "test".to_string(),
            "Test service".to_string(),
            ServiceType::System,
        );
        manager.add_service(service);
        manager.start_service("test");
        assert!(manager.list_running().len() == 1);
        manager.stop_service("test");
        assert!(manager.list_running().len() == 0);
    }

    #[test]
    fn test_service_enable_disable() {
        let mut manager = SigmaServiceManager::new();
        let service = Service::new(
            "test".to_string(),
            "test".to_string(),
            "Test service".to_string(),
            ServiceType::System,
        );
        manager.add_service(service);
        manager.enable_service("test");
        assert!(manager.list_enabled().len() == 1);
        manager.disable_service("test");
        assert!(manager.list_enabled().len() == 0);
    }
;
