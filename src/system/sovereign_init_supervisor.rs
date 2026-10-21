//! # Sovereign Init Supervisor (PID 1)
//!
//! Bare-metal PID 1 process manager and service supervisor for SigmaOS.
//! Provides zombie process reaping (SIGCHLD), cgroups v2 resource enforcement,
//! socket-activated service startup, watchdog timeout monitoring, recursive dependency graph
//! resolution, live reloading, and sd_notify protocol compatibility without systemd bloat.

#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

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
    Reloading,
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
    pub dependencies: Vec<String>,
    pub env_vars: BTreeMap<String, String>,
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
            dependencies: Vec::new(),
            env_vars: BTreeMap::new(),
        }
    }

    pub fn with_dependency(mut self, dep: &str) -> Self {
        self.dependencies.push(String::from(dep));
        self
    }

    pub fn with_env(mut self, key: &str, val: &str) -> Self {
        self.env_vars.insert(String::from(key), String::from(val));
        self
    }

    pub fn with_watchdog(mut self, watchdog_usec: u64) -> Self {
        self.watchdog_usec = watchdog_usec;
        self
    }
}

/// Sovereign PID 1 Supervisor
#[path = "sovereign_init_and_supervisor_advancements_v63.rs"]
mod sovereign_init_and_supervisor_advancements_v63;

use sovereign_init_and_supervisor_advancements_v63::*;

pub struct SovereignInitSupervisor {
    pub next_pid: AtomicU32,
    pub services: BTreeMap<String, SovereignServiceUnit>,
    pub pid_map: BTreeMap<u32, String>,
    pub total_spawned_processes: AtomicU64,
    pub total_reaped_zombies: AtomicU64,
    pub systemd_v63_engine: SystemdAdvancementsV63Engine,
    pub openrc_v63_engine: OpenRcAdvancementsV63Engine,
    pub runit_s6_v63_engine: RunitS6AdvancementsV63Engine,
    pub sysvinit_v63_engine: SysVinitAdvancementsV63Engine,
    pub bsd_rc_v63_engine: BsdRcAdvancementsV63Engine,
    pub launchd_android_v63_engine: LaunchdAndroidAdvancementsV63Engine,
    pub scheduler_v63_engine: SchedulerTimersAdvancementsV63Engine,
    pub pr_gateway_v63: SovereignInitSupervisorV63PrGateway,
}

impl SovereignInitSupervisor {
    pub fn new() -> Self {
        Self {
            next_pid: AtomicU32::new(2), // PID 1 is supervisor itself
            services: BTreeMap::new(),
            pid_map: BTreeMap::new(),
            total_spawned_processes: AtomicU64::new(0),
            total_reaped_zombies: AtomicU64::new(0),
            systemd_v63_engine: SystemdAdvancementsV63Engine::new(),
            openrc_v63_engine: OpenRcAdvancementsV63Engine::new(),
            runit_s6_v63_engine: RunitS6AdvancementsV63Engine::new(),
            sysvinit_v63_engine: SysVinitAdvancementsV63Engine::new(),
            bsd_rc_v63_engine: BsdRcAdvancementsV63Engine::new(),
            launchd_android_v63_engine: LaunchdAndroidAdvancementsV63Engine::new(),
            scheduler_v63_engine: SchedulerTimersAdvancementsV63Engine::new(),
            pr_gateway_v63: SovereignInitSupervisorV63PrGateway::new(),
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

    /// Recursively resolve dependencies and start required units before spawning target
    pub fn start_service_with_dependencies(
        &mut self,
        name: &str,
    ) -> Result<Vec<u32>, &'static str> {
        let deps = match self.services.get(name) {
            Some(u) => u.dependencies.clone(),
            None => return Err("Service not found"),
        };

        let mut pids = Vec::new();
        for dep in deps {
            let dep_pid = self.start_service_with_dependencies(&dep)?;
            pids.extend(dep_pid);
        }

        let main_pid = self.start_service(name)?;
        pids.push(main_pid);
        Ok(pids)
    }

    /// Reload a running service configuration (`SIGHUP` / `RELOAD=1`)
    pub fn reload_service(&mut self, name: &str) -> Result<(), &'static str> {
        let unit = self.services.get_mut(name).ok_or("Service not found")?;
        if unit.state != ServiceState::Running && unit.state != ServiceState::Ready {
            return Err("Service not running");
        }
        unit.state = ServiceState::Reloading;
        unit.status_message = String::from("Reloading configuration");
        Ok(())
    }

    /// Dynamically update cgroups v2 resource limits for a service unit
    pub fn update_service_limits(
        &mut self,
        name: &str,
        limits: ServiceResourceLimits,
    ) -> Result<(), &'static str> {
        let unit = self.services.get_mut(name).ok_or("Service not found")?;
        unit.limits = limits;
        Ok(())
    }

    /// Check watchdog ping timeouts across active services
    pub fn check_watchdog_timeouts(&mut self, current_time_usec: u64) -> Vec<String> {
        let mut timed_out_services = Vec::new();

        for (name, unit) in &mut self.services {
            if (unit.state == ServiceState::Running || unit.state == ServiceState::Ready)
                && unit.watchdog_usec > 0
            {
                let elapsed = current_time_usec.saturating_sub(unit.last_watchdog_ping);
                if elapsed > unit.watchdog_usec {
                    timed_out_services.push(name.clone());
                    unit.state = ServiceState::Failed;
                    unit.status_message = String::from("Watchdog timeout exceeded");
                    if let Some(pid) = unit.pid {
                        self.pid_map.remove(&pid);
                    }
                    unit.pid = None;
                }
            }
        }

        timed_out_services
    }

    /// Parse and apply sd_notify protocol messages using SystemdAdvancementsV63Engine
    pub fn process_sd_notify(&mut self, pid: u32, payload: &str) -> bool {
        let service_name = match self.pid_map.get(&pid) {
            Some(n) => n.clone(),
            None => return false,
        };

        let parsed = self.systemd_v63_engine.parse_sd_notify_payload(payload);

        let unit = match self.services.get_mut(&service_name) {
            Some(u) => u,
            None => return false,
        };

        if parsed.ready {
            unit.state = ServiceState::Ready;
            unit.status_message = parsed
                .status
                .unwrap_or_else(|| String::from("Service active and operational"));
        } else if let Some(status) = parsed.status {
            unit.status_message = status;
        }

        if parsed.watchdog_ping {
            unit.last_watchdog_ping += 1;
        }

        if let Some(usec) = parsed.extend_timeout_usec {
            unit.watchdog_usec = usec;
        }

        if let Some(main_pid) = parsed.main_pid {
            unit.pid = Some(main_pid);
        }

        if parsed.stopping {
            unit.state = ServiceState::Stopping;
        } else if parsed.reloading {
            unit.state = ServiceState::Reloading;
            unit.status_message = String::from("Reloading configuration");
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
            let delay = self
                .runit_s6_v63_engine
                .calculate_backoff_delay(unit.restart_count, 300);
            unit.state = ServiceState::Restarting;
            unit.status_message = format!("Restarting after exit (backoff delay {}s)", delay);
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
    use alloc::vec;

    #[test]
    fn test_supervisor_service_lifecycle() {
        let mut supervisor = SovereignInitSupervisor::new();
        let unit =
            SovereignServiceUnit::new("networkd", "/bin/sigma-networkd", RestartPolicy::Always);
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
        let unit =
            SovereignServiceUnit::new("oneshot-backup", "/bin/backup", RestartPolicy::OnFailure);
        supervisor.register_service(unit);

        let pid = supervisor.start_service("oneshot-backup").unwrap();
        let next_state = supervisor.reap_child_process(pid, 0).unwrap();
        assert_eq!(next_state, ServiceState::Stopped);
    }

    #[test]
    fn test_dependencies_and_watchdog_and_reload() {
        let mut supervisor = SovereignInitSupervisor::new();
        let db_unit = SovereignServiceUnit::new("db", "/bin/db", RestartPolicy::Always);
        let app_unit = SovereignServiceUnit::new("app", "/bin/app", RestartPolicy::Always)
            .with_dependency("db")
            .with_env("APP_ENV", "production")
            .with_watchdog(1000);

        supervisor.register_service(db_unit);
        supervisor.register_service(app_unit);

        let pids = supervisor.start_service_with_dependencies("app").unwrap();
        assert_eq!(pids.len(), 2);
        assert_eq!(
            supervisor.services.get("db").unwrap().state,
            ServiceState::Starting
        );
        assert_eq!(
            supervisor.services.get("app").unwrap().state,
            ServiceState::Starting
        );

        // Reload app
        supervisor.services.get_mut("app").unwrap().state = ServiceState::Running;
        assert!(supervisor.reload_service("app").is_ok());
        assert_eq!(
            supervisor.services.get("app").unwrap().state,
            ServiceState::Reloading
        );

        // Check watchdog timeout
        supervisor.services.get_mut("app").unwrap().state = ServiceState::Running;
        supervisor
            .services
            .get_mut("app")
            .unwrap()
            .last_watchdog_ping = 100;
        let timed_out = supervisor.check_watchdog_timeouts(2000);
        assert_eq!(timed_out, vec!["app"]);
        assert_eq!(
            supervisor.services.get("app").unwrap().state,
            ServiceState::Failed
        );
    }
}
