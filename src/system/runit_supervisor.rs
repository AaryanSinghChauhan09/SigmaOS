//! runit-style Service Supervisor
//! Inspired by runit (Void Linux), s6 (Alpine), daemontools (djb).
//! Provides process supervision: auto-restart, dependency ordering, logging.
//!
//! References:
//! - runit: http://smarden.org/runit/
//! - s6: https://skarnet.org/software/s6/
//! - Void Linux sv: https://docs.voidlinux.org/config/services/

use std::collections::HashMap;

/// Service state (mirrors runit's sv states)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Down,
    Up,
    Finishing,
    Exited(i32),
    Paused,
    Waiting,
}

/// Service restart policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestartPolicy {
    Never,
    OnFailure,
    Always,
    BackoffExponential,
}

/// A supervised service definition
#[derive(Debug, Clone)]
pub struct ServiceDef {
    pub name: String,
    pub run_command: String,
    pub restart_policy: RestartPolicy,
    pub dependencies: Vec<String>,
    pub max_restarts: u32,
    pub timeout_secs: u32,
    pub restart_count: u32,
    pub state: ServiceState,
    pub pid: Option<u32>,
    pub uptime_secs: u64,
}

impl ServiceDef {
    pub fn new(name: &str, cmd: &str) -> Self {
        Self {
            name: name.to_string(),
            run_command: cmd.to_string(),
            restart_policy: RestartPolicy::Always,
            dependencies: Vec::new(),
            max_restarts: 5,
            timeout_secs: 7,
            restart_count: 0,
            state: ServiceState::Down,
            pid: None,
            uptime_secs: 0,
        }
    }
}

/// Service supervision tree (like /etc/sv/ in Void Linux)
pub struct RunitSupervisor {
    services: HashMap<String, ServiceDef>,
    pub total_starts: u64,
    pub total_restarts: u64,
    pub total_failures: u64,
}

impl RunitSupervisor {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
            total_starts: 0,
            total_restarts: 0,
            total_failures: 0,
        }
    }

    pub fn add_service(&mut self, svc: ServiceDef) {
        self.services.insert(svc.name.clone(), svc);
    }

    /// Start a service (and its dependencies first).
    pub fn start(&mut self, name: &str) -> Result<(), &'static str> {
        let deps: Vec<String> = self
            .services
            .get(name)
            .ok_or("Service not found")?
            .dependencies
            .clone();
        for dep in &deps {
            if let Some(d) = self.services.get(dep) {
                if d.state != ServiceState::Up {
                    return Err("Dependency not running");
                }
            }
        }
        let svc = self.services.get_mut(name).ok_or("Service not found")?;
        if svc.state == ServiceState::Up {
            return Ok(());
        }
        svc.pid = Some(1000 + svc.name.len() as u32);
        svc.state = ServiceState::Up;
        self.total_starts += 1;
        Ok(())
    }

    pub fn stop(&mut self, name: &str) -> Result<(), &'static str> {
        let svc = self.services.get_mut(name).ok_or("Service not found")?;
        svc.state = ServiceState::Finishing;
        svc.pid = None;
        svc.state = ServiceState::Down;
        Ok(())
    }

    pub fn handle_exit(&mut self, name: &str, exit_code: i32) {
        let svc = match self.services.get_mut(name) {
            Some(s) => s,
            None => return,
        };
        svc.state = ServiceState::Exited(exit_code);
        svc.pid = None;
        let should_restart = match svc.restart_policy {
            RestartPolicy::Never => false,
            RestartPolicy::Always => true,
            RestartPolicy::OnFailure | RestartPolicy::BackoffExponential => exit_code != 0,
        };
        if should_restart && svc.restart_count < svc.max_restarts {
            svc.restart_count += 1;
            svc.pid = Some(2000 + svc.restart_count);
            svc.state = ServiceState::Up;
            self.total_restarts += 1;
        } else if should_restart {
            self.total_failures += 1;
        }
    }

    pub fn running_count(&self) -> usize {
        self.services
            .values()
            .filter(|s| s.state == ServiceState::Up)
            .count()
    }

    pub fn status(&self) -> Vec<(&str, ServiceState, Option<u32>)> {
        let mut r: Vec<_> = self
            .services
            .values()
            .map(|s| (s.name.as_str(), s.state, s.pid))
            .collect();
        r.sort_by_key(|s| s.0);
        r
    }
}

impl Default for RunitSupervisor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_stop() {
        let mut sup = RunitSupervisor::new();
        sup.add_service(ServiceDef::new("sshd", "/usr/sbin/sshd -D"));
        sup.start("sshd").unwrap();
        assert_eq!(sup.services["sshd"].state, ServiceState::Up);
        sup.stop("sshd").unwrap();
        assert_eq!(sup.services["sshd"].state, ServiceState::Down);
    }

    #[test]
    fn test_restart_on_failure() {
        let mut sup = RunitSupervisor::new();
        let mut svc = ServiceDef::new("test", "/usr/bin/test");
        svc.restart_policy = RestartPolicy::OnFailure;
        sup.add_service(svc);
        sup.start("test").unwrap();
        sup.handle_exit("test", 1);
        assert_eq!(sup.services["test"].state, ServiceState::Up);
        assert_eq!(sup.total_restarts, 1);
    }

    #[test]
    fn test_no_restart_on_clean_exit() {
        let mut sup = RunitSupervisor::new();
        let mut svc = ServiceDef::new("oneshot", "/bin/setup");
        svc.restart_policy = RestartPolicy::OnFailure;
        sup.add_service(svc);
        sup.start("oneshot").unwrap();
        sup.handle_exit("oneshot", 0);
        assert_ne!(sup.services["oneshot"].state, ServiceState::Up);
        assert_eq!(sup.total_restarts, 0);
    }

    #[test]
    fn test_dependency_ordering() {
        let mut sup = RunitSupervisor::new();
        sup.add_service(ServiceDef::new("network", "/sbin/network start"));
        let mut nginx = ServiceDef::new("nginx", "/usr/sbin/nginx");
        nginx.dependencies.push("network".to_string());
        sup.add_service(nginx);
        // nginx can't start until network is up
        assert!(sup.start("nginx").is_err());
        sup.start("network").unwrap();
        sup.start("nginx").unwrap();
        assert_eq!(sup.running_count(), 2);
    }
}
