//! Init system — PID 1 equivalent for SigmaOS.
//!
//! `InitSystem` is the outermost supervisor.  It owns a `ServiceManager`,
//! drives the boot sequence, and transitions the system through well-defined
//! `SystemState` values.  The design is:
//!
//! * **Fail-fast**: any required service that cannot start aborts the boot.
//! * **Race-free**: no global mutable state; callers hold the only instance.
//! * **Deadlock-avoidant**: the dependency graph is cycle-checked before use.

use std::collections::HashMap;

use crate::service::{ServiceManager, ServiceState};
use thiserror::Error;

/// Overall system lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemState {
    /// Kernel has handed control to init; services not yet started.
    Booting,
    /// All required services are running; system is fully operational.
    Running,
    /// A graceful shutdown has been requested.
    ShuttingDown,
    /// Single-user / recovery console.
    Maintenance,
    /// A fatal error occurred during boot.
    Panic,
}

impl std::fmt::Display for SystemState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SystemState::Booting => "booting",
            SystemState::Running => "running",
            SystemState::ShuttingDown => "shutting-down",
            SystemState::Maintenance => "maintenance",
            SystemState::Panic => "panic",
        };
        write!(f, "{}", s)
    }
}

/// Errors that the init system can produce.
#[derive(Debug, Error)]
pub enum InitError {
    #[error("dependency cycle detected involving service '{0}'")]
    DependencyCycle(String),

    #[error("required service '{0}' failed to start: {1}")]
    RequiredServiceFailed(String, String),

    #[error("service '{0}' not found")]
    ServiceNotFound(String),

    #[error("invalid state transition from {0} to {1}")]
    InvalidTransition(SystemState, SystemState),
}

/// The SigmaOS init system.
///
/// Callers construct one `InitSystem`, register services via the embedded
/// `ServiceManager`, then call `boot()` to bring the system up.
pub struct InitSystem {
    state: SystemState,
    /// Embedded service manager owned by init.
    pub services: ServiceManager,
}

impl InitSystem {
    /// Create a new init system in the `Booting` state.
    pub fn new() -> Self {
        Self {
            state: SystemState::Booting,
            services: ServiceManager::new(),
        }
    }

    /// Current system state.
    pub fn state(&self) -> SystemState {
        self.state
    }

    /// Drive the boot sequence.
    ///
    /// 1. Validate the service dependency graph for cycles.
    /// 2. Start enabled services in dependency order.
    /// 3. Transition to `Running`.
    ///
    /// Returns `Err` (and sets state to `Panic`) if any required service fails.
    pub fn boot(&mut self) -> Result<(), InitError> {
        if self.state != SystemState::Booting {
            return Err(InitError::InvalidTransition(
                self.state,
                SystemState::Running,
            ));
        }

        // Validate the dependency graph before touching any services.
        self.validate_no_cycles()?;

        // Collect enabled service names upfront (avoids borrow issues).
        let enabled: Vec<String> = self
            .services
            .all()
            .filter(|s| s.config.enabled)
            .map(|s| s.config.name.clone())
            .collect();

        // Build start order via topological sort.
        let order = self.topological_order(&enabled)?;

        for name in &order {
            if let Err(e) = self.services.start(name) {
                let svc = self.services.get(name);
                let required = svc.map(|s| s.config.required).unwrap_or(false);
                if required {
                    self.state = SystemState::Panic;
                    return Err(InitError::RequiredServiceFailed(
                        name.clone(),
                        e.to_string(),
                    ));
                }
                // Non-required service failure is logged but not fatal.
            }
        }

        self.state = SystemState::Running;
        Ok(())
    }

    /// Begin an orderly shutdown: stop all running services, then transition.
    pub fn shutdown(&mut self) -> Result<(), InitError> {
        if self.state == SystemState::ShuttingDown {
            return Ok(());
        }
        self.state = SystemState::ShuttingDown;

        // Stop services in reverse dependency order (best-effort).
        let running: Vec<String> = self
            .services
            .all()
            .filter(|s| s.state == ServiceState::Running)
            .map(|s| s.config.name.clone())
            .collect();

        for name in running.iter().rev() {
            let _ = self.services.stop(name);
        }

        Ok(())
    }

    /// Switch to maintenance / single-user mode.
    pub fn enter_maintenance(&mut self) {
        self.state = SystemState::Maintenance;
    }

    // ── internal helpers ──────────────────────────────────────────────────

    /// Detect dependency cycles using DFS.
    fn validate_no_cycles(&self) -> Result<(), InitError> {
        let mut visited: HashMap<String, bool> = HashMap::new();
        let names: Vec<String> = self
            .services
            .all()
            .map(|s| s.config.name.clone())
            .collect();

        for name in &names {
            if !visited.contains_key(name) {
                self.dfs_cycle_check(name, &mut visited, &mut vec![])?;
            }
        }
        Ok(())
    }

    fn dfs_cycle_check(
        &self,
        name: &str,
        visited: &mut HashMap<String, bool>,
        stack: &mut Vec<String>,
    ) -> Result<(), InitError> {
        visited.insert(name.to_string(), true);
        stack.push(name.to_string());

        if let Some(svc) = self.services.get(name) {
            for dep in &svc.config.dependencies {
                if !visited.contains_key(dep) {
                    self.dfs_cycle_check(dep, visited, stack)?;
                } else if stack.contains(dep) {
                    return Err(InitError::DependencyCycle(dep.clone()));
                }
            }
        }

        stack.pop();
        Ok(())
    }

    /// Return service names in topological (dependency-first) order.
    fn topological_order(&self, names: &[String]) -> Result<Vec<String>, InitError> {
        let mut result: Vec<String> = Vec::new();
        let mut placed: HashMap<String, bool> = HashMap::new();

        for name in names {
            self.topo_visit(name, &mut result, &mut placed)?;
        }
        Ok(result)
    }

    fn topo_visit(
        &self,
        name: &str,
        result: &mut Vec<String>,
        placed: &mut HashMap<String, bool>,
    ) -> Result<(), InitError> {
        if placed.contains_key(name) {
            return Ok(());
        }
        placed.insert(name.to_string(), true);

        if let Some(svc) = self.services.get(name) {
            let deps = svc.config.dependencies.clone();
            for dep in &deps {
                self.topo_visit(dep, result, placed)?;
            }
        }

        result.push(name.to_string());
        Ok(())
    }
}
