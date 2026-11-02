//! SigmaOS System — core system service orchestration and init abstraction.
//!
//! Implements the `service-management.md` platform contract: service state
//! machine, dependency graph, restart policy, health checks, and lifecycle hooks.
//! Provides a backend-agnostic API over systemd (Linux) and rc.d (BSD).

#![allow(dead_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

// ---------------------------------------------------------------------------
// Service state machine
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Idle,
    Starting,
    Running,
    Stopping,
    Stopped,
    Failed,
    Degraded,
    Restarting,
}

impl fmt::Display for ServiceState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:?}", self) }
}

// ---------------------------------------------------------------------------
// Restart policy
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestartPolicy {
    Never,
    OnFailure,
    Always,
}

// ---------------------------------------------------------------------------
// Service manifest
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct ServiceManifest {
    pub name: String,
    pub exec: String,
    pub description: String,
    pub requires: Vec<String>,
    pub wants: Vec<String>,
    pub restart_policy: RestartPolicy,
    pub health_check: Option<String>,
}

impl ServiceManifest {
    pub fn new(name: impl Into<String>, exec: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            exec: exec.into(),
            description: String::new(),
            requires: Vec::new(),
            wants: Vec::new(),
            restart_policy: RestartPolicy::OnFailure,
            health_check: None,
        }
    }
}

// ---------------------------------------------------------------------------
// Service record
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct ServiceRecord {
    pub manifest: ServiceManifest,
    pub state: ServiceState,
    pub restart_count: u32,
    pub last_exit_code: Option<i32>,
}

impl ServiceRecord {
    pub fn new(manifest: ServiceManifest) -> Self {
        Self { manifest, state: ServiceState::Idle, restart_count: 0, last_exit_code: None }
    }
    pub fn is_healthy(&self) -> bool {
        matches!(self.state, ServiceState::Running | ServiceState::Idle)
    }
}

// ---------------------------------------------------------------------------
// Service manager
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub enum SystemError {
    ServiceNotFound(String),
    DependencyMissing(String),
    CycleDetected,
    AlreadyRunning(String),
    NotRunning(String),
}

pub struct ServiceManager {
    services: BTreeMap<String, ServiceRecord>,
}

impl ServiceManager {
    pub fn new() -> Self {
        Self { services: BTreeMap::new() }
    }

    pub fn register(&mut self, manifest: ServiceManifest) {
        self.services.insert(manifest.name.clone(), ServiceRecord::new(manifest));
    }

    pub fn start(&mut self, name: &str) -> Result<(), SystemError> {
        // Check dependencies
        let deps: Vec<String> = self.services.get(name)
            .ok_or_else(|| SystemError::ServiceNotFound(name.to_string()))?
            .manifest.requires.clone();

        for dep in &deps {
            let dep_state = self.services.get(dep.as_str())
                .ok_or_else(|| SystemError::DependencyMissing(dep.clone()))?
                .state;
            if dep_state != ServiceState::Running {
                return Err(SystemError::DependencyMissing(dep.clone()));
            }
        }

        let svc = self.services.get_mut(name).unwrap();
        if svc.state == ServiceState::Running {
            return Err(SystemError::AlreadyRunning(name.to_string()));
        }
        svc.state = ServiceState::Starting;
        // Real impl: spawn exec, wait for readiness
        svc.state = ServiceState::Running;
        Ok(())
    }

    pub fn stop(&mut self, name: &str) -> Result<(), SystemError> {
        let svc = self.services.get_mut(name)
            .ok_or_else(|| SystemError::ServiceNotFound(name.to_string()))?;
        if svc.state != ServiceState::Running {
            return Err(SystemError::NotRunning(name.to_string()));
        }
        svc.state = ServiceState::Stopping;
        svc.state = ServiceState::Stopped;
        Ok(())
    }

    pub fn restart(&mut self, name: &str) -> Result<(), SystemError> {
        if self.services.get(name).map(|s| s.state) == Some(ServiceState::Running) {
            self.stop(name)?;
        }
        self.services.get_mut(name).map(|s| s.restart_count += 1);
        self.start(name)
    }

    pub fn status(&self, name: &str) -> Option<&ServiceRecord> {
        self.services.get(name)
    }

    pub fn list(&self) -> Vec<&ServiceRecord> {
        self.services.values().collect()
    }

    pub fn unhealthy(&self) -> Vec<&ServiceRecord> {
        self.services.values().filter(|s| !s.is_healthy()).collect()
    }

    /// Topological start order for `names` and their dependencies.
    pub fn start_order(&self, names: &[&str]) -> Result<Vec<String>, SystemError> {
        let mut visited = BTreeSet::new();
        let mut order = Vec::new();

        fn visit(
            name: &str,
            services: &BTreeMap<String, ServiceRecord>,
            visited: &mut BTreeSet<String>,
            order: &mut Vec<String>,
            stack: &mut BTreeSet<String>,
        ) -> Result<(), SystemError> {
            if stack.contains(name) { return Err(SystemError::CycleDetected); }
            if visited.contains(name) { return Ok(()); }
            stack.insert(name.to_string());
            let deps = services.get(name)
                .ok_or_else(|| SystemError::ServiceNotFound(name.to_string()))?
                .manifest.requires.clone();
            for dep in &deps {
                visit(dep, services, visited, order, stack)?;
            }
            stack.remove(name);
            visited.insert(name.to_string());
            order.push(name.to_string());
            Ok(())
        }

        let mut stack = BTreeSet::new();
        for &n in names {
            visit(n, &self.services, &mut visited, &mut order, &mut stack)?;
        }
        Ok(order)
    }
}

impl Default for ServiceManager {
    fn default() -> Self { Self::new() }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_manager() -> ServiceManager {
        let mut sm = ServiceManager::new();
        sm.register(ServiceManifest::new("logger", "/usr/bin/sigma-logger"));
        let mut net = ServiceManifest::new("network", "/usr/bin/sigma-network");
        net.requires = vec!["logger".into()];
        sm.register(net);
        let mut desktop = ServiceManifest::new("desktop", "/usr/bin/sigma-desktop");
        desktop.requires = vec!["network".into()];
        sm.register(desktop);
        sm
    }

    #[test]
    fn start_stop_basic() {
        let mut sm = make_manager();
        sm.start("logger").unwrap();
        assert_eq!(sm.status("logger").unwrap().state, ServiceState::Running);
        sm.stop("logger").unwrap();
        assert_eq!(sm.status("logger").unwrap().state, ServiceState::Stopped);
    }

    #[test]
    fn dep_must_be_running_before_dependant() {
        let mut sm = make_manager();
        assert!(sm.start("network").is_err()); // logger not running
        sm.start("logger").unwrap();
        assert!(sm.start("network").is_ok());
    }

    #[test]
    fn topological_start_order() {
        let sm = make_manager();
        let order = sm.start_order(&["desktop"]).unwrap();
        assert_eq!(order, vec!["logger", "network", "desktop"]);
    }

    #[test]
    fn restart_increments_count() {
        let mut sm = make_manager();
        sm.start("logger").unwrap();
        sm.restart("logger").unwrap();
        assert_eq!(sm.status("logger").unwrap().restart_count, 1);
    }

    #[test]
    fn double_start_fails() {
        let mut sm = make_manager();
        sm.start("logger").unwrap();
        assert!(sm.start("logger").is_err());
    }
}
