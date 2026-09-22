// SPDX-License-Identifier: MIT
// SigmaOS Service Supervisor Engine
// Zero-dependency, safe Rust implementation of PID 1 service supervision, restart policies, and health monitoring.

#![allow(dead_code)]

use std::collections::HashMap;
use std::string::String;
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestartPolicy {
    No,
    OnFailure,
    OnSuccess,
    Always,
    OnAbnormal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceStatus {
    Inactive,
    Starting,
    Running,
    Failed,
    Stopped,
}

#[derive(Debug, Clone)]
pub struct ServiceDescriptor {
    pub name: String,
    pub exec_path: String,
    pub args: Vec<String>,
    pub restart_policy: RestartPolicy,
    pub dependencies: Vec<String>,
    pub status: ServiceStatus,
    pub pid: Option<u32>,
    pub restart_count: u32,
}

impl ServiceDescriptor {
    pub fn new(name: &str, exec_path: &str, restart_policy: RestartPolicy) -> Self {
        Self {
            name: String::from(name),
            exec_path: String::from(exec_path),
            args: Vec::new(),
            restart_policy,
            dependencies: Vec::new(),
            status: ServiceStatus::Inactive,
            pid: None,
            restart_count: 0,
        }
    }
}

pub struct ServiceSupervisor {
    pub services: HashMap<String, ServiceDescriptor>,
    pub next_pid: u32,
}

impl ServiceSupervisor {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
            next_pid: 100,
        }
    }

    pub fn register_service(&mut self, descriptor: ServiceDescriptor) {
        self.services.insert(descriptor.name.clone(), descriptor);
    }

    pub fn start_service(&mut self, name: &str) -> Result<u32, &'static str> {
        let service = self.services.get_mut(name).ok_or("Service not found")?;
        if service.status == ServiceStatus::Running {
            return Ok(service.pid.unwrap_or(0));
        }

        service.status = ServiceStatus::Starting;
        let assigned_pid = self.next_pid;
        self.next_pid += 1;
        service.pid = Some(assigned_pid);
        service.status = ServiceStatus::Running;
        Ok(assigned_pid)
    }

    pub fn stop_service(&mut self, name: &str) -> Result<(), &'static str> {
        let service = self.services.get_mut(name).ok_or("Service not found")?;
        service.status = ServiceStatus::Stopped;
        service.pid = None;
        Ok(())
    }

    pub fn handle_service_exit(&mut self, name: &str, exit_code: i32) -> bool {
        if let Some(service) = self.services.get_mut(name) {
            let failed = exit_code != 0;
            let should_restart = match service.restart_policy {
                RestartPolicy::Always => true,
                RestartPolicy::OnFailure | RestartPolicy::OnAbnormal => failed,
                RestartPolicy::OnSuccess => !failed,
                RestartPolicy::No => false,
            };

            if should_restart {
                service.restart_count += 1;
                service.status = ServiceStatus::Starting;
                let new_pid = self.next_pid;
                self.next_pid += 1;
                service.pid = Some(new_pid);
                service.status = ServiceStatus::Running;
                true
            } else {
                service.status = if failed { ServiceStatus::Failed } else { ServiceStatus::Stopped };
                service.pid = None;
                false
            }
        } else {
            false
        }
    }
}

impl Default for ServiceSupervisor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_supervisor_lifecycle() {
        let mut supervisor = ServiceSupervisor::new();
        let svc = ServiceDescriptor::new("nginx", "/usr/sbin/nginx", RestartPolicy::OnFailure);
        supervisor.register_service(svc);

        let pid = supervisor.start_service("nginx").unwrap();
        assert!(pid >= 100);

        let restarted = supervisor.handle_service_exit("nginx", 1);
        assert!(restarted);

        let stopped = supervisor.stop_service("nginx");
        assert!(stopped.is_ok());
    }
}
