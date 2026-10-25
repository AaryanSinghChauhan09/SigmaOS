// SPDX-License-Identifier: GPL-2.0-or-later
// Principle: SELF-HEALING, FAULT ISOLATION, MICROKERNEL ROBUSTNESS, SERVICE REINCARNATION

use alloc::{vec::Vec, string::String, collections::BTreeMap};

/// Policy controlling how a driver is restarted upon failure
#[derive(Debug, Clone)]
pub struct DriverPolicy {
    pub max_restarts: u32,
    pub backoff_ms: u64,
    pub restart_on_crash: bool,
    pub critical: bool, // if true, panic kernel on failure (last resort)
}

impl DriverPolicy {
    pub fn default() -> Self {
        Self {
            max_restarts: 5,
            backoff_ms: 500,
            restart_on_crash: true,
            critical: false,
        }
    }

    pub fn critical() -> Self {
        Self { max_restarts: 3, backoff_ms: 100, restart_on_crash: true, critical: true }
    }
}

/// The lifecycle state of a driver/service
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverState {
    Running,
    Crashed,
    Restarting,
    Disabled,
}

/// A single service record maintained by the reincarnation server
#[derive(Debug, Clone)]
pub struct ServiceRecord {
    pub name: String,
    pub pid: u32,
    pub state: DriverState,
    pub policy: DriverPolicy,
    pub restart_count: u32,
    pub last_crash_tick: u64,
    pub uptime_ticks: u64,
}

impl ServiceRecord {
    pub fn new(name: String, pid: u32, policy: DriverPolicy) -> Self {
        Self {
            name,
            pid,
            state: DriverState::Running,
            policy,
            restart_count: 0,
            last_crash_tick: 0,
            uptime_ticks: 0,
        }
    }

    /// Returns true if the service can be restarted per its policy
    pub fn can_restart(&self) -> bool {
        self.policy.restart_on_crash && self.restart_count < self.policy.max_restarts
    }

    /// Called by the server to perform a restart simulation
    pub fn do_restart(&mut self, new_pid: u32, tick: u64) {
        self.pid = new_pid;
        self.state = DriverState::Running;
        self.restart_count += 1;
        self.uptime_ticks = tick;
    }
}

/// Minix 3-style Reincarnation Server
pub struct ReincarnationServer {
    services: BTreeMap<String, ServiceRecord>,
    next_pid: u32,
    current_tick: u64,
    event_log: Vec<String>,
}

impl ReincarnationServer {
    pub fn new() -> Self {
        Self {
            services: BTreeMap::new(),
            next_pid: 100,
            current_tick: 0,
            event_log: Vec::new(),
        }
    }

    pub fn register_service(&mut self, name: String, policy: DriverPolicy) -> u32 {
        let pid = self.next_pid;
        self.next_pid += 1;
        let record = ServiceRecord::new(name.clone(), pid, policy);
        self.log(alloc::format!("registered service '{}' pid={}", name, pid));
        self.services.insert(name, record);
        pid
    }

    pub fn report_crash(&mut self, name: &str) -> Result<(), &'static str> {
        let tick = self.current_tick;
        let record = self.services.get_mut(name).ok_or("service not found")?;
        record.state = DriverState::Crashed;
        record.last_crash_tick = tick;
        self.log(alloc::format!("service '{}' crashed at tick {}", name, tick));
        Ok(())
    }

    pub fn restart_driver(&mut self, name: &str) -> Result<u32, &'static str> {
        let tick = self.current_tick;
        let next_pid = self.next_pid;
        let record = self.services.get_mut(name).ok_or("service not found")?;

        if record.state != DriverState::Crashed && record.state != DriverState::Restarting {
            return Err("service is not in a restartable state");
        }
        if !record.can_restart() {
            record.state = DriverState::Disabled;
            return Err("max restarts exceeded, service disabled");
        }

        record.state = DriverState::Restarting;
        record.do_restart(next_pid, tick);
        self.next_pid += 1;
        self.log(alloc::format!("restarted '{}' as pid={}", name, next_pid));
        Ok(next_pid)
    }

    /// Periodic health check — advances tick and auto-restarts crashed services
    pub fn tick(&mut self) {
        self.current_tick += 1;
        let names: Vec<String> = self.services.keys().cloned().collect();
        for name in names {
            let should_restart = {
                let rec = &self.services[&name];
                rec.state == DriverState::Crashed && rec.can_restart()
            };
            if should_restart {
                let _ = self.restart_driver(&name);
            }
            // Increment uptime for running services
            if let Some(rec) = self.services.get_mut(&name) {
                if rec.state == DriverState::Running {
                    rec.uptime_ticks += 1;
                }
            }
        }
    }

    pub fn get_service(&self, name: &str) -> Option<&ServiceRecord> {
        self.services.get(name)
    }

    pub fn list_services(&self) -> Vec<&ServiceRecord> {
        self.services.values().collect()
    }

    fn log(&mut self, msg: String) {
        self.event_log.push(msg);
    }

    pub fn event_log(&self) -> &[String] {
        &self.event_log
    }

    pub fn current_tick(&self) -> u64 {
        self.current_tick
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_crash_restart() {
        let mut srv = ReincarnationServer::new();
        let pid = srv.register_service(String::from("disk"), DriverPolicy::default());
        assert!(pid >= 100);

        srv.report_crash("disk").unwrap();
        assert_eq!(srv.get_service("disk").unwrap().state, DriverState::Crashed);

        let new_pid = srv.restart_driver("disk").unwrap();
        assert_ne!(new_pid, pid);
        assert_eq!(srv.get_service("disk").unwrap().state, DriverState::Running);
        assert_eq!(srv.get_service("disk").unwrap().restart_count, 1);
    }

    #[test]
    fn test_max_restarts_disables_service() {
        let policy = DriverPolicy { max_restarts: 2, backoff_ms: 0, restart_on_crash: true, critical: false };
        let mut srv = ReincarnationServer::new();
        srv.register_service(String::from("net"), policy);

        for _ in 0..2 {
            srv.report_crash("net").unwrap();
            srv.restart_driver("net").unwrap();
        }
        srv.report_crash("net").unwrap();
        let result = srv.restart_driver("net");
        assert!(result.is_err());
        assert_eq!(srv.get_service("net").unwrap().state, DriverState::Disabled);
    }

    #[test]
    fn test_tick_auto_restarts() {
        let mut srv = ReincarnationServer::new();
        srv.register_service(String::from("audio"), DriverPolicy::default());
        srv.report_crash("audio").unwrap();
        srv.tick();
        assert_eq!(srv.get_service("audio").unwrap().state, DriverState::Running);
    }

    #[test]
    fn test_uptime_tracking() {
        let mut srv = ReincarnationServer::new();
        srv.register_service(String::from("fs"), DriverPolicy::default());
        srv.tick();
        srv.tick();
        srv.tick();
        assert_eq!(srv.get_service("fs").unwrap().uptime_ticks, 3);
    }

    #[test]
    fn test_event_log() {
        let mut srv = ReincarnationServer::new();
        srv.register_service(String::from("kbd"), DriverPolicy::default());
        srv.report_crash("kbd").unwrap();
        assert!(srv.event_log().len() >= 2);
    }
}
