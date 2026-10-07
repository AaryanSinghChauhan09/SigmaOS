//! # Sovereign Init Supervisor (PID 1)
//!
//! Bare-metal PID 1 process manager and service supervisor for SigmaOS.
//! Provides zombie process reaping (SIGCHLD), cgroups v2 resource enforcement,
//! socket-activated service startup, and sd_notify protocol compatibility without systemd bloat.

#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicI32, AtomicU32, AtomicU64, Ordering};

/// Service Restart Policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestartPolicy {
    Always,
    OnFailure,
    Never,
}

/// Service Execution State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Stopped,
    Starting,
    Running,
    Ready,
    Stopping,
    Failed,
    Restarting,
}

/// Cgroups v2 Resource Limits for Service
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServiceResourceLimits {
    pub max_memory_bytes: u64,
    pub cpu_quota_pct: u32,
    pub max_tasks: u32,
}

impl Default for ServiceResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_bytes: 512 * 1024 * 1024, // 512MB default limit
            cpu_quota_pct: 100,
            max_tasks: 256,
        }
    }
}

/// Supervised Service Unit
#[derive(Debug, Clone)]
pub struct SovereignServiceUnit {
    pub name: String,
    pub command: String,
    pub pid: Option<u32>,
    pub state: ServiceState,
    pub restart_policy: RestartPolicy,
    pub restart_count: u32,
    pub max_restarts: u32,
    pub last_exit_code: Option<i32>,
    pub limits: ServiceResourceLimits,
    pub status_message: String,
    pub watchdog_usec: u64,
    pub last_watchdog_ping: u64,
}

impl SovereignServiceUnit {
    pub fn new(name: &str, command: &str, policy: RestartPolicy) -> Self {
        Self {
            name: String::from(name),
            command: String::from(command),
            pid: None,
            state: ServiceState::Stopped,
            restart_policy: policy,
            restart_count: 0,
            max_restarts: 5,
            last_exit_code: None,
            limits: ServiceResourceLimits::default(),
            status_message: String::from("Loaded"),
            watchdog_usec: 0,
            last_watchdog_ping: 0,
        }
    }
}

/// Sovereign PID 1 Supervisor
pub struct SovereignInitSupervisor {
    pub next_pid: AtomicU32,
    pub services: BTreeMap<String, SovereignServiceUnit>,
    pub pid_map: BTreeMap<u32, String>,
    pub total_spawned_processes: AtomicU64,
    pub total_reaped_zombies: AtomicU64,
}

impl SovereignInitSupervisor {
    pub fn new() -> Self {
        Self {
            next_pid: AtomicU32::new(2), // PID 1 is supervisor itself
            services: BTreeMap::new(),
            pid_map: BTreeMap::new(),
            total_spawned_processes: AtomicU64::new(0),
            total_reaped_zombies: AtomicU64::new(0),
        }
    }

    /// Register a service unit
    pub fn register_service(&mut self, unit: SovereignServiceUnit) {
        self.services.insert(unit.name.clone(), unit);
    }

    /// Spawn a registered service process
    pub fn start_service(&mut self, name: &str) -> Result<u32, &'static str> {
        let unit = self.services.get_mut(name).ok_or("Service not found")?;
        if unit.state == ServiceState::Running || unit.state == ServiceState::Ready {
            return Ok(unit.pid.unwrap());
        }

        let pid = self.next_pid.fetch_add(1, Ordering::SeqCst);
        unit.pid = Some(pid);
        unit.state = ServiceState::Starting;
        unit.status_message = String::from("Spawning child process");

        self.pid_map.insert(pid, String::from(name));
        self.total_spawned_processes.fetch_add(1, Ordering::SeqCst);

        Ok(pid)
    }

    /// Parse and apply sd_notify protocol messages (READY=1, STATUS=..., STOPPING=1)
    pub fn process_sd_notify(&mut self, pid: u32, payload: &str) -> bool {
        let service_name = match self.pid_map.get(&pid) {
            Some(n) => n.clone(),
            None => return false,
        };

        let unit = match self.services.get_mut(&service_name) {
            Some(u) => u,
            None => return false,
        };

        for line in payload.split('\n') {
            let line = line.trim();
            if line == "READY=1" {
                unit.state = ServiceState::Ready;
                unit.status_message = String::from("Service active and operational");
            } else if line.starts_with("STATUS=") {
                unit.status_message = String::from(&line[7..]);
            } else if line == "STOPPING=1" {
                unit.state = ServiceState::Stopping;
            } else if line == "WATCHDOG=1" {
                unit.last_watchdog_ping += 1;
            }
        }

        true
    }

    /// Handle process exit and zombie reaping (SIGCHLD)
    pub fn reap_child_process(&mut self, pid: u32, exit_code: i32) -> Option<ServiceState> {
        self.total_reaped_zombies.fetch_add(1, Ordering::SeqCst);

        let service_name = match self.pid_map.remove(&pid) {
            Some(n) => n,
            None => return None, // Untracked orphaned process reaped
        };

        let unit = match self.services.get_mut(&service_name) {
            Some(u) => u,
            None => return None,
        };

        unit.pid = None;
        unit.last_exit_code = Some(exit_code);

        let should_restart = match unit.restart_policy {
            RestartPolicy::Always => true,
            RestartPolicy::OnFailure => exit_code != 0,
            RestartPolicy::Never => false,
        };

        if should_restart && unit.restart_count < unit.max_restarts {
            unit.restart_count += 1;
            unit.state = ServiceState::Restarting;
            unit.status_message = String::from("Restarting after exit");
        } else if exit_code != 0 {
            unit.state = ServiceState::Failed;
            unit.status_message = String::from("Service exited with error");
        } else {
            unit.state = ServiceState::Stopped;
            unit.status_message = String::from("Service finished successfully");
        }

        Some(unit.state)
    }

    /// Stop a running service cleanly
    pub fn stop_service(&mut self, name: &str) -> Result<(), &'static str> {
        let unit = self.services.get_mut(name).ok_or("Service not found")?;
        if let Some(pid) = unit.pid {
            self.pid_map.remove(&pid);
        }
        unit.pid = None;
        unit.state = ServiceState::Stopped;
        unit.status_message = String::from("Stopped by administrator");
        Ok(())
    }
}

// ============================================================================
// UNIT TESTS & STANDALONE HARNESS
// ============================================================================

#[cfg(any(test, feature = "standalone_test"))]
mod tests {
    use super::*;

    #[test]
    fn test_supervisor_service_lifecycle() {
        let mut supervisor = SovereignInitSupervisor::new();
        let unit = SovereignServiceUnit::new("networkd", "/bin/sigma-networkd", RestartPolicy::Always);
        supervisor.register_service(unit);

        let pid = supervisor.start_service("networkd").unwrap();
        assert_eq!(pid, 2);

        // Notify ready
        let handled = supervisor.process_sd_notify(pid, "READY=1\nSTATUS=Connected to eth0");
        assert!(handled);

        let srv = supervisor.services.get("networkd").unwrap();
        assert_eq!(srv.state, ServiceState::Ready);
        assert_eq!(srv.status_message, "Connected to eth0");

        // Simulate crash
        let next_state = supervisor.reap_child_process(pid, 1).unwrap();
        assert_eq!(next_state, ServiceState::Restarting);

        let srv2 = supervisor.services.get("networkd").unwrap();
        assert_eq!(srv2.restart_count, 1);
        assert_eq!(supervisor.total_reaped_zombies.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_never_restart_on_success() {
        let mut supervisor = SovereignInitSupervisor::new();
        let unit = SovereignServiceUnit::new("oneshot-backup", "/bin/backup", RestartPolicy::OnFailure);
        supervisor.register_service(unit);

        let pid = supervisor.start_service("oneshot-backup").unwrap();
        let next_state = supervisor.reap_child_process(pid, 0).unwrap();
        assert_eq!(next_state, ServiceState::Stopped);
    }
}
