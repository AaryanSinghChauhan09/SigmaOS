#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// Runit Supervision Engine for SigmaOS
// Location: src/init/runit.rs

use std::string::String;
use std::vec::Vec;
use std::collections::BTreeMap;
use super::init_abstraction::{InitSystem, InitSystemType, ServiceStatus, InitError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunitStage {
    Stage1OneTimeInit,
    Stage2ServiceSupervision,
    Stage3Shutdown,
}

pub struct RunitService {
    pub name: String,
    pub status: ServiceStatus,
    pub enabled: bool,
    pub run_script: String,
}

pub struct RunitInit {
    pub current_stage: RunitStage,
    pub services: BTreeMap<String, RunitService>,
}

impl RunitInit {
    pub fn new() -> Self {
        RunitInit {
            current_stage: RunitStage::Stage1OneTimeInit,
            services: BTreeMap::new(),
        }
    }

    pub fn execute_stage1(&mut self) {
        // Stage 1: One-time system setup
        self.current_stage = RunitStage::Stage2ServiceSupervision;
    }

    pub fn register_service(&mut self, name: &str, run_script: &str) {
        self.services.insert(
            String::from(name),
            RunitService {
                name: String::from(name),
                status: ServiceStatus::Stopped,
                enabled: false,
                run_script: String::from(run_script),
            },
        );
    }
}

impl InitSystem for RunitInit {
    fn init_type(&self) -> InitSystemType {
        InitSystemType::Runit
    }

    fn start_service(&mut self, name: &str) -> Result<(), InitError> {
        if let Some(svc) = self.services.get_mut(name) {
            if svc.status == ServiceStatus::Running {
                return Err(InitError::AlreadyRunning);
            }
            svc.status = ServiceStatus::Running;
            Ok(())
        } else {
            Err(InitError::ServiceNotFound(String::from(name)))
        }
    }

    fn stop_service(&mut self, name: &str) -> Result<(), InitError> {
        if let Some(svc) = self.services.get_mut(name) {
            if svc.status == ServiceStatus::Stopped {
                return Err(InitError::AlreadyStopped);
            }
            svc.status = ServiceStatus::Stopped;
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
