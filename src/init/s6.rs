extern crate alloc;
// S6 Supervision Engine for SigmaOS
// Location: src/init/s6.rs

use alloc::string::String;
use alloc::collections::BTreeMap;
use super::init_abstraction::{InitSystem, InitSystemType, ServiceStatus, InitError};

pub struct S6Service {
    pub name: String,
    pub status: ServiceStatus,
    pub ready_notification: bool,
    pub enabled: bool,
}

pub struct S6Init {
    pub svscan_active: bool,
    pub services: BTreeMap<String, S6Service>,
}

impl S6Init {
    pub fn new() -> Self {
        S6Init {
            svscan_active: true,
            services: BTreeMap::new(),
        }
    }

    pub fn register_service(&mut self, name: &str) {
        self.services.insert(
            String::from(name),
            S6Service {
                name: String::from(name),
                status: ServiceStatus::Stopped,
                ready_notification: false,
                enabled: false,
            },
        );
    }

    pub fn notify_ready(&mut self, name: &str) -> bool {
        if let Some(svc) = self.services.get_mut(name) {
            svc.ready_notification = true;
            true
        } else {
            false
        }
    }
}

impl InitSystem for S6Init {
    fn init_type(&self) -> InitSystemType {
        InitSystemType::S6
    }

    fn start_service(&mut self, name: &str) -> Result<(), InitError> {
        if let Some(svc) = self.services.get_mut(name) {
            svc.status = ServiceStatus::Running;
            svc.ready_notification = false;
            Ok(())
        } else {
            Err(InitError::ServiceNotFound(String::from(name)))
        }
    }

    fn stop_service(&mut self, name: &str) -> Result<(), InitError> {
        if let Some(svc) = self.services.get_mut(name) {
            svc.status = ServiceStatus::Stopped;
            svc.ready_notification = false;
            Ok(())
        } else {
            Err(InitError::ServiceNotFound(String::from(name)))
        }
    }

    fn restart_service(&mut self, name: &str) -> Result<(), InitError> {
        self.stop_service(name)?;
        self.start_service(name)
    }

    fn service_status(&self, name: &str) -> ServiceStatus {
        self.services.get(name).map(|s| s.status).unwrap_or(ServiceStatus::Unknown)
    }

    fn enable_service(&mut self, name: &str) -> Result<(), InitError> {
        if let Some(svc) = self.services.get_mut(name) {
            svc.enabled = true;
            Ok(())
        } else {
            Err(InitError::ServiceNotFound(String::from(name)))
        }
    }

    fn disable_service(&mut self, name: &str) -> Result<(), InitError> {
        if let Some(svc) = self.services.get_mut(name) {
            svc.enabled = false;
            Ok(())
        } else {
            Err(InitError::ServiceNotFound(String::from(name)))
        }
    }
}
