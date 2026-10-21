// SPDX-License-Identifier: MIT
// SigmaOS Service Manager 4
// (`src/system/sigmaservicemanager4.rs`)

#![allow(dead_code)]

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::collections::BTreeMap;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::collections::BTreeMap;
#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceState {
    Stopped,
    Starting,
    Active,
    Stopping,
    Failed(String),
}

#[derive(Debug, Clone)]
pub struct Service {
    pub name: String,
    pub description: String,
    pub state: ServiceState,
    pub enabled: bool,
}

pub struct SigmaServiceManager4 {
    pub services: BTreeMap<String, Service>,
}

impl SigmaServiceManager4 {
    pub fn new() -> Self {
        let mut services = BTreeMap::new();
        services.insert(
            "ssh.service".to_string(),
            Service {
                name: "ssh.service".to_string(),
                description: "OpenSSH Daemon".to_string(),
                state: ServiceState::Active,
                enabled: true,
            },
        );
        Self { services }
    }

    pub fn start_service(&mut self, name: &str) -> bool {
        if let Some(svc) = self.services.get_mut(name) {
            svc.state = ServiceState::Active;
            true
        } else {
            false
        }
    }

    pub fn stop_service(&mut self, name: &str) -> bool {
        if let Some(svc) = self.services.get_mut(name) {
            svc.state = ServiceState::Stopped;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_manager4() {
        let mut mgr = SigmaServiceManager4::new();
        assert!(mgr.stop_service("ssh.service"));
        assert_eq!(mgr.services.get("ssh.service").unwrap().state, ServiceState::Stopped);
        assert!(mgr.start_service("ssh.service"));
        assert_eq!(mgr.services.get("ssh.service").unwrap().state, ServiceState::Active);
    }
}
