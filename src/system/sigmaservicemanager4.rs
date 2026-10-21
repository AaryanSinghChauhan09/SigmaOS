// SPDX-License-Identifier: MIT
// SigmaServiceManager4 - Advanced System Service Lifecycle Supervision

#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Inactive,
    Activating,
    Active,
    Deactivating,
    Failed,
}

#[derive(Debug, Clone)]
pub struct Service {
    pub name: String,
    pub description: String,
    pub state: ServiceState,
    pub pid: Option<u32>,
}

impl Service {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            state: ServiceState::Inactive,
            pid: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SigmaServiceManager4 {
    pub services: Vec<Service>,
}

impl SigmaServiceManager4 {
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
        }
    }

    pub fn register_service(&mut self, service: Service) {
        self.services.push(service);
    }
}
