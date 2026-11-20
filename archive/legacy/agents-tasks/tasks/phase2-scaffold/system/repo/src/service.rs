//! Service manager — start, stop, restart, and query system services.
//!
//! The `ServiceManager` is a pure value type (no globals, no threads).
//! It enforces dependency-ordering: a service whose dependencies are not
//! all `Running` cannot be started, keeping the system fail-fast and
//! preventing partial start-up races.

use std::collections::HashMap;
use thiserror::Error;

// ─── public types ────────────────────────────────────────────────────────────

/// Lifecycle state of a single service.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    /// Registered but not yet started.
    Stopped,
    /// Start in progress.
    Starting,
    /// Healthy and accepting requests.
    Running,
    /// Graceful stop in progress.
    Stopping,
    /// Terminated with a non-zero exit code.
    Failed,
    /// Actively reloading configuration.
    Reloading,
}

impl std::fmt::Display for ServiceState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ServiceState::Stopped => "stopped",
            ServiceState::Starting => "starting",
            ServiceState::Running => "running",
            ServiceState::Stopping => "stopping",
            ServiceState::Failed => "failed",
            ServiceState::Reloading => "reloading",
        };
        write!(f, "{}", s)
    }
}

/// Restart policy for a service.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestartPolicy {
    Never,
    OnFailure,
    Always,
}

/// Static configuration for a service (set at registration time).
#[derive(Debug, Clone)]
pub struct ServiceConfig {
    pub name: String,
    pub description: String,
    /// Path to the executable (stub — not exec'd in scaffold).
    pub exec_path: String,
    /// Hard dependencies: all must be `Running` before this can start.
    pub dependencies: Vec<String>,
    /// Whether to start this service automatically on boot.
    pub enabled: bool,
    /// Whether a start failure aborts the boot sequence.
    pub required: bool,
    pub restart_policy: RestartPolicy,
}

impl ServiceConfig {
    pub fn new(name: impl Into<String>, exec_path: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: String::new(),
            exec_path: exec_path.into(),
            dependencies: Vec::new(),
            enabled: false,
            required: false,
            restart_policy: RestartPolicy::Never,
        }
    }

    pub fn description(mut self, d: impl Into<String>) -> Self {
        self.description = d.into();
        self
    }

    pub fn enabled(mut self, v: bool) -> Self {
        self.enabled = v;
        self
    }

    pub fn required(mut self, v: bool) -> Self {
        self.required = v;
        self
    }

    pub fn depends_on(mut self, dep: impl Into<String>) -> Self {
        self.dependencies.push(dep.into());
        self
    }

    pub fn restart(mut self, policy: RestartPolicy) -> Self {
        self.restart_policy = policy;
        self
    }
}

/// A live service instance.
#[derive(Debug)]
pub struct Service {
    pub config: ServiceConfig,
    pub state: ServiceState,
    /// Simulated process ID (placeholder — no real process is spawned).
    pub pid: Option<u32>,
    pub restart_count: u32,
}

impl Service {
    fn new(config: ServiceConfig) -> Self {
        Self {
            config,
            state: ServiceState::Stopped,
            pid: None,
            restart_count: 0,
        }
    }

    /// A snapshot of the service's current status.
    pub fn status(&self) -> ServiceStatus {
        ServiceStatus {
            name: self.config.name.clone(),
            state: self.state,
            pid: self.pid,
            restart_count: self.restart_count,
        }
    }
}

/// Read-only snapshot of a service's runtime status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceStatus {
    pub name: String,
    pub state: ServiceState,
    pub pid: Option<u32>,
    pub restart_count: u32,
}

// ─── errors ──────────────────────────────────────────────────────────────────

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ServiceError {
    #[error("service '{0}' not found")]
    NotFound(String),

    #[error("service '{0}' is already running")]
    AlreadyRunning(String),

    #[error("service '{0}' is not running")]
    NotRunning(String),

    #[error("service '{0}' already registered")]
    AlreadyRegistered(String),

    #[error("dependency '{0}' of service '{1}' is not running")]
    DependencyNotRunning(String, String),
}

// ─── service manager ─────────────────────────────────────────────────────────

/// Manages all registered services.
///
/// Pure value type — all state lives inside this struct.
pub struct ServiceManager {
    services: HashMap<String, Service>,
    next_pid: u32,
}

impl ServiceManager {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
            next_pid: 1000,
        }
    }

    /// Register a new service.  Returns `Err` if the name is already taken.
    pub fn register(&mut self, config: ServiceConfig) -> Result<(), ServiceError> {
        let name = config.name.clone();
        if self.services.contains_key(&name) {
            return Err(ServiceError::AlreadyRegistered(name));
        }
        self.services.insert(name, Service::new(config));
        Ok(())
    }

    /// Look up a service by name (immutable).
    pub fn get(&self, name: &str) -> Option<&Service> {
        self.services.get(name)
    }

    /// Look up a service by name (mutable).
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Service> {
        self.services.get_mut(name)
    }

    /// Iterate over all registered services.
    pub fn all(&self) -> impl Iterator<Item = &Service> {
        self.services.values()
    }

    /// Start a service.
    ///
    /// Fails if any hard dependency is not in the `Running` state.
    pub fn start(&mut self, name: &str) -> Result<(), ServiceError> {
        // Collect dependency list without holding a mutable borrow.
        let deps = self
            .services
            .get(name)
            .ok_or_else(|| ServiceError::NotFound(name.to_string()))?
            .config
            .dependencies
            .clone();

        // Validate dependencies (immutable borrow is fine here).
        for dep in &deps {
            match self.services.get(dep) {
                Some(d) if d.state == ServiceState::Running => {}
                Some(_) => {
                    return Err(ServiceError::DependencyNotRunning(
                        dep.clone(),
                        name.to_string(),
                    ))
                }
                None => {
                    return Err(ServiceError::DependencyNotRunning(
                        dep.clone(),
                        name.to_string(),
                    ))
                }
            }
        }

        let pid = self.next_pid;
        self.next_pid += 1;

        let svc = self
            .services
            .get_mut(name)
            .ok_or_else(|| ServiceError::NotFound(name.to_string()))?;

        if svc.state == ServiceState::Running {
            return Err(ServiceError::AlreadyRunning(name.to_string()));
        }

        svc.state = ServiceState::Starting;
        // In production this would exec the binary; here we stub success.
        svc.state = ServiceState::Running;
        svc.pid = Some(pid);
        Ok(())
    }

    /// Stop a running service.
    pub fn stop(&mut self, name: &str) -> Result<(), ServiceError> {
        let svc = self
            .services
            .get_mut(name)
            .ok_or_else(|| ServiceError::NotFound(name.to_string()))?;

        if svc.state != ServiceState::Running {
            return Err(ServiceError::NotRunning(name.to_string()));
        }

        svc.state = ServiceState::Stopping;
        // In production: send SIGTERM, wait, send SIGKILL if needed.
        svc.state = ServiceState::Stopped;
        svc.pid = None;
        Ok(())
    }

    /// Restart a service (stop → start).
    pub fn restart(&mut self, name: &str) -> Result<(), ServiceError> {
        self.stop(name)?;
        self.start(name)?;
        self.services
            .get_mut(name)
            .map(|s| s.restart_count += 1);
        Ok(())
    }

    /// Return a status snapshot for every registered service.
    pub fn status_all(&self) -> Vec<ServiceStatus> {
        let mut out: Vec<ServiceStatus> = self.services.values().map(|s| s.status()).collect();
        out.sort_by(|a, b| a.name.cmp(&b.name));
        out
    }

    /// Number of registered services.
    pub fn len(&self) -> usize {
        self.services.len()
    }

    /// True if no services are registered.
    pub fn is_empty(&self) -> bool {
        self.services.is_empty()
    }
}
