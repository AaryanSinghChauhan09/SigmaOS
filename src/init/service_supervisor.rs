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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SocketActivationSpec {
    pub listen_port: u16,
    pub protocol: String, // "tcp" or "udp"
    pub fd_num: i32,
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
    pub socket_activation: Option<SocketActivationSpec>,
    pub watchdog_timeout_sec: u64,
    pub last_watchdog_ping_sec: u64,
    pub cpu_limit_percent: u32,
    pub memory_limit_mb: u64,
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
            socket_activation: None,
            watchdog_timeout_sec: 0, // 0 = disabled
            last_watchdog_ping_sec: 0,
            cpu_limit_percent: 100,
            memory_limit_mb: 0, // 0 = unlimited
        }
    }

    pub fn with_socket_activation(mut self, port: u16, protocol: &str, fd: i32) -> Self {
        self.socket_activation = Some(SocketActivationSpec {
            listen_port: port,
            protocol: String::from(protocol),
            fd_num: fd,
        });
        self
    }

    pub fn with_watchdog(mut self, timeout_sec: u64) -> Self {
        self.watchdog_timeout_sec = timeout_sec;
        self
    }

    pub fn with_resource_limits(mut self, cpu_pct: u32, mem_mb: u64) -> Self {
        self.cpu_limit_percent = cpu_pct;
        self.memory_limit_mb = mem_mb;
        self
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

    pub fn ping_watchdog(&mut self, name: &str, current_time_sec: u64) -> Result<(), &'static str> {
        let service = self.services.get_mut(name).ok_or("Service not found")?;
        service.last_watchdog_ping_sec = current_time_sec;
        Ok(())
    }

    pub fn check_watchdog_probes(&mut self, current_time_sec: u64) -> Vec<String> {
        let mut timed_out = Vec::new();
        for (name, service) in self.services.iter_mut() {
            if service.status == ServiceStatus::Running && service.watchdog_timeout_sec > 0 {
                let elapsed = current_time_sec.saturating_sub(service.last_watchdog_ping_sec);
                if elapsed > service.watchdog_timeout_sec {
                    timed_out.push(name.clone());
                }
            }
        }

        for name in &timed_out {
            self.handle_service_exit(name, -1);
        }

        timed_out
    }

    pub fn trigger_socket_activation(&mut self, port: u16) -> Result<u32, &'static str> {
        let mut target_service = None;
        for (name, service) in &self.services {
            if let Some(ref sock) = service.socket_activation {
                if sock.listen_port == port {
                    target_service = Some(name.clone());
                    break;
                }
            }
        }

        if let Some(name) = target_service {
            self.start_service(&name)
        } else {
            Err("No service registered for socket activation on target port")
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

    #[test]
    fn test_socket_activation() {
        let mut supervisor = ServiceSupervisor::new();
        let svc = ServiceDescriptor::new("sshd", "/usr/sbin/sshd", RestartPolicy::Always)
            .with_socket_activation(22, "tcp", 3)
            .with_resource_limits(50, 512);

        supervisor.register_service(svc);

        assert_eq!(supervisor.services["sshd"].status, ServiceStatus::Inactive);

        let pid = supervisor.trigger_socket_activation(22).unwrap();
        assert!(pid >= 100);
        assert_eq!(supervisor.services["sshd"].status, ServiceStatus::Running);

        assert!(supervisor.trigger_socket_activation(80).is_err());
    }

    #[test]
    fn test_watchdog_health_probes() {
        let mut supervisor = ServiceSupervisor::new();
        let svc = ServiceDescriptor::new("db_daemon", "/usr/bin/db", RestartPolicy::Always)
            .with_watchdog(10); // 10s watchdog

        supervisor.register_service(svc);
        supervisor.start_service("db_daemon").unwrap();

        // Initial ping at t=100
        assert!(supervisor.ping_watchdog("db_daemon", 100).is_ok());

        // Check at t=105 (elapsed 5s <= 10s) -> no timeout
        let timed_out = supervisor.check_watchdog_probes(105);
        assert!(timed_out.is_empty());

        // Check at t=115 (elapsed 15s > 10s) -> timeout and automatic restart triggered!
        let timed_out_late = supervisor.check_watchdog_probes(115);
        assert_eq!(timed_out_late, vec!["db_daemon".to_string()]);
        assert_eq!(supervisor.services["db_daemon"].restart_count, 1);
        assert_eq!(supervisor.services["db_daemon"].status, ServiceStatus::Running);
    }
}
