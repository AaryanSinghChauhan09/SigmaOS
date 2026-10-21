//! Sigma Service Manager 4
//!
//! Enterprise multi-tier service state manager and dependency supervisor.

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceState {
    Active,
    Inactive,
    Failed,
    Reloading,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Service {
    pub name: String,
    pub description: String,
    pub state: ServiceState,
    pub auto_restart: bool,
}

#[derive(Debug, Default)]
pub struct SigmaServiceManager4 {
    pub services: Vec<Service>,
}

impl SigmaServiceManager4 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_service(&mut self, svc: Service) {
        self.services.push(svc);
    }
}
