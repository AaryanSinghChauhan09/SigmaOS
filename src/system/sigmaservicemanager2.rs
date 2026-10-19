//! SigmaServiceManager - System Service Manager
//!
//! Inspired by systemd and OpenRC, this module provides comprehensive system service
//! management including service start/stop, enable/disable, and dependency management.

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, Ordering};

/// Service state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Stopped,
    Running,
    Failed,
    Unknown,
;

/// Service data structure
#[derive(Debug, Clone)]
pub struct Service {
    pub service_id: String,
    pub name: String,
    pub state: ServiceState,
    pub enabled: bool,
;

impl Service {
    pub fn new(service_id: String, name: String) -> Self {
        Self {
            service_id,
            name,
            state: ServiceState::Stopped,
            enabled: false,
        }
    }
;

/// Service manager
pub struct SigmaServiceManager {
    services: BTreeMap<String, Service>,
    running: AtomicBool,
;

impl SigmaServiceManager {
    pub fn new() -> Self {
        Self {
            services: BTreeMap::new(),
            running: AtomicBool::new(false),
        }
    }

    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
    }
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    pub fn add_service(&mut self, service: Service) -> bool {
        self.services
            .insert(service.service_id.clone(), service)
            .is_some()
    }

    pub fn start_service(&mut self, service_id: &str) -> bool {
        if let Some(service) = self.services.get_mut(service_id) {
            service.state = ServiceState::Running;
            true
        } else {
            false
        }
    }

    pub fn stop_service(&mut self, service_id: &str) -> bool {
        if let Some(service) = self.services.get_mut(service_id) {
            service.state = ServiceState::Stopped;
            true
        } else {
            false
        }
    }

    pub fn enable_service(&mut self, service_id: &str) -> bool {
        if let Some(service) = self.services.get_mut(service_id) {
            service.enabled = true;
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

static mut SIGMASERVICEMANAGER_INSTANCE: Option<SigmaServiceManager> = None;

pub fn init_sigmaservicemanager() {
    unsafe {
        SIGMASERVICEMANAGER_INSTANCE = Some(SigmaServiceManager::new());
    }
;

pub fn get_sigmaservicemanager() -> Option<&'static SigmaServiceManager> {
    unsafe { SIGMASERVICEMANAGER_INSTANCE.as_ref() }
;

pub fn get_sigmaservicemanager_mut() -> Option<&'static mut SigmaServiceManager> {
    unsafe { SIGMASERVICEMANAGER_INSTANCE.as_mut() }
;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sigmaservicemanager_creation() {
        let mgr = SigmaServiceManager::new();
        assert!(!mgr.is_running());
    }
;
