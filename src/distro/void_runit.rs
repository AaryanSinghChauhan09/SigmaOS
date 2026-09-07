/*
 * SigmaOS - Void Linux runit Service Supervision Engine
 *
 * Implements Void Linux runit inspired service supervision, service control stage execution,
 * health checking, and automatic restart policy governance.
 */

#![cfg_attr(not(test), no_std)]

#[cfg(not(test))]
extern crate alloc;

#[cfg(not(test))]
use alloc::collections::BTreeMap;
#[cfg(not(test))]
use alloc::string::String;
#[cfg(not(test))]
use alloc::vec::Vec;

#[cfg(test)]
use std::collections::BTreeMap;
#[cfg(test)]
use std::string::String;
#[cfg(test)]
use std::vec::Vec;

/// Runit Service Status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunitServiceStatus {
    Down,
    Starting,
    Running,
    Stopping,
    Failed,
}

/// Runit Service Definition
#[derive(Debug, Clone)]
pub struct RunitService {
    pub name: String,
    pub status: RunitServiceStatus,
    pub pid: Option<u32>,
    pub auto_restart: bool,
    pub health_check_failures: u32,
    pub max_allowed_failures: u32,
}

impl RunitService {
    pub fn new(name: &str, auto_restart: bool, max_allowed_failures: u32) -> Self {
        Self {
            name: String::from(name),
            status: RunitServiceStatus::Down,
            pid: None,
            auto_restart,
            health_check_failures: 0,
            max_allowed_failures,
        }
    }

    pub fn start(&mut self) -> bool {
        if self.status == RunitServiceStatus::Running {
            return true;
        }
        self.status = RunitServiceStatus::Running;
        self.pid = Some(1000 + (self.name.len() as u32));
        true
    }

    pub fn stop(&mut self) -> bool {
        self.status = RunitServiceStatus::Down;
        self.pid = None;
        true
    }

    pub fn check_health(&mut self, is_healthy: bool) -> RunitServiceStatus {
        if is_healthy {
            self.health_check_failures = 0;
            if self.status == RunitServiceStatus::Starting {
                self.status = RunitServiceStatus::Running;
            }
        } else {
            self.health_check_failures += 1;
            if self.health_check_failures >= self.max_allowed_failures {
                self.status = RunitServiceStatus::Failed;
                if self.auto_restart {
                    self.start();
                }
            }
        }
        self.status
    }
}

/// Runit Service Supervisor Engine
#[derive(Debug, Default, Clone)]
pub struct RunitSupervisor {
    pub services: BTreeMap<String, RunitService>,
}

impl RunitSupervisor {
    pub fn new() -> Self {
        Self {
            services: BTreeMap::new(),
        }
    }

    pub fn register_service(&mut self, service: RunitService) {
        self.services.insert(service.name.clone(), service);
    }

    pub fn start_service(&mut self, name: &str) -> bool {
        if let Some(service) = self.services.get_mut(name) {
            service.start()
        } else {
            false
        }
    }

    pub fn stop_service(&mut self, name: &str) -> bool {
        if let Some(service) = self.services.get_mut(name) {
            service.stop()
        } else {
            false
        }
    }

    pub fn start_all(&mut self) {
        let keys: Vec<String> = self.services.keys().cloned().collect();
        for key in keys {
            if let Some(s) = self.services.get_mut(&key) {
                s.start();
            }
        }
    }

    pub fn stop_all(&mut self) {
        let keys: Vec<String> = self.services.keys().cloned().collect();
        for key in keys {
            if let Some(s) = self.services.get_mut(&key) {
                s.stop();
            }
        }
    }

    pub fn active_service_count(&self) -> usize {
        self.services
            .values()
            .filter(|s| s.status == RunitServiceStatus::Running)
            .count()
    }

    pub fn monitor_service_health(&mut self, name: &str, is_healthy: bool) -> Option<RunitServiceStatus> {
        if let Some(service) = self.services.get_mut(name) {
            Some(service.check_health(is_healthy))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runit_supervisor_lifecycle() {
        let mut supervisor = RunitSupervisor::new();
        let service = RunitService::new("dhcpcd", true, 3);

        supervisor.register_service(service);
        assert_eq!(supervisor.active_service_count(), 0);

        assert!(supervisor.start_service("dhcpcd"));
        assert_eq!(supervisor.active_service_count(), 1);

        assert_eq!(
            supervisor.monitor_service_health("dhcpcd", true),
            Some(RunitServiceStatus::Running)
        );

        assert!(supervisor.stop_service("dhcpcd"));
        assert_eq!(supervisor.active_service_count(), 0);
    }

    #[test]
    fn test_runit_auto_restart() {
        let mut supervisor = RunitSupervisor::new();
        let service = RunitService::new("sshd", true, 2);

        supervisor.register_service(service);
        supervisor.start_service("sshd");

        // Fail 1
        supervisor.monitor_service_health("sshd", false);
        assert_eq!(supervisor.active_service_count(), 1);

        // Fail 2 (exceeds max_allowed_failures 2) -> auto_restart triggers start()
        let status = supervisor.monitor_service_health("sshd", false);
        assert_eq!(status, Some(RunitServiceStatus::Running));
        assert_eq!(supervisor.active_service_count(), 1);
    }
}
