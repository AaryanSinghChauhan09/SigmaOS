//! Orchestration layer — coordinates groups of services as logical units.
//!
//! `Orchestrator` sits above `ServiceManager`: it understands higher-level
//! deployment targets (think systemd "targets" or Kubernetes deployments) and
//! delegates individual service control to the manager it wraps.

use std::collections::HashMap;

use crate::service::{ServiceError, ServiceManager};
use thiserror::Error;

// ─── errors ──────────────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum OrchestrationError {
    #[error("target group '{0}' not found")]
    GroupNotFound(String),

    #[error("target group '{0}' already exists")]
    GroupExists(String),

    #[error("service error in group '{0}': {1}")]
    Service(String, #[source] ServiceError),
}

// ─── types ───────────────────────────────────────────────────────────────────

/// A named logical grouping of services (analogous to a systemd target).
#[derive(Debug, Clone)]
pub struct ServiceGroup {
    /// Human-readable name.
    pub name: String,
    /// Names of services that belong to this group.
    pub members: Vec<String>,
    /// Groups that must be active before this one can start.
    pub after: Vec<String>,
}

impl ServiceGroup {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            members: Vec::new(),
            after: Vec::new(),
        }
    }

    pub fn add_member(mut self, svc: impl Into<String>) -> Self {
        self.members.push(svc.into());
        self
    }

    pub fn after(mut self, group: impl Into<String>) -> Self {
        self.after.push(group.into());
        self
    }
}

// ─── orchestrator ────────────────────────────────────────────────────────────

/// Coordinates multi-service deployments on top of a `ServiceManager`.
pub struct Orchestrator {
    pub manager: ServiceManager,
    groups: HashMap<String, ServiceGroup>,
}

impl Orchestrator {
    /// Create an orchestrator wrapping an existing service manager.
    pub fn new(manager: ServiceManager) -> Self {
        Self {
            manager,
            groups: HashMap::new(),
        }
    }

    /// Register a service group.
    pub fn register_group(&mut self, group: ServiceGroup) -> Result<(), OrchestrationError> {
        let name = group.name.clone();
        if self.groups.contains_key(&name) {
            return Err(OrchestrationError::GroupExists(name));
        }
        self.groups.insert(name, group);
        Ok(())
    }

    /// Look up a group by name.
    pub fn get_group(&self, name: &str) -> Option<&ServiceGroup> {
        self.groups.get(name)
    }

    /// Start every service in a group, in member order.
    pub fn start_group(&mut self, name: &str) -> Result<(), OrchestrationError> {
        let members = self
            .groups
            .get(name)
            .ok_or_else(|| OrchestrationError::GroupNotFound(name.to_string()))?
            .members
            .clone();

        for svc in &members {
            self.manager
                .start(svc)
                .map_err(|e| OrchestrationError::Service(name.to_string(), e))?;
        }
        Ok(())
    }

    /// Stop every service in a group, in reverse member order.
    pub fn stop_group(&mut self, name: &str) -> Result<(), OrchestrationError> {
        let members = self
            .groups
            .get(name)
            .ok_or_else(|| OrchestrationError::GroupNotFound(name.to_string()))?
            .members
            .clone();

        for svc in members.iter().rev() {
            // Best-effort: log but don't abort on individual stop failures.
            let _ = self.manager.stop(svc);
        }
        Ok(())
    }

    /// Restart every service in a group.
    pub fn restart_group(&mut self, name: &str) -> Result<(), OrchestrationError> {
        let members = self
            .groups
            .get(name)
            .ok_or_else(|| OrchestrationError::GroupNotFound(name.to_string()))?
            .members
            .clone();

        for svc in &members {
            self.manager
                .restart(svc)
                .map_err(|e| OrchestrationError::Service(name.to_string(), e))?;
        }
        Ok(())
    }

    /// How many groups are registered.
    pub fn group_count(&self) -> usize {
        self.groups.len()
    }
}
