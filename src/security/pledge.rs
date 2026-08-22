// SigmaOS Pledge - Process Privilege Reduction Mechanism
// Inspired by OpenBSD pledge but capability-based

#[cfg(test)]
#[path = "capability.rs"]
pub mod capability;

#[cfg(test)]
use capability::{CapabilityGate, CapabilityToken, Permission};

#[cfg(not(test))]
use crate::security::capability::{CapabilityGate, CapabilityToken, Permission};

use core::sync::atomic::{AtomicBool, Ordering};

/// Pledge promise representing process permissions
#[derive(Debug)]
pub struct PledgePromise {
    /// Allowed permissions
    permissions: Vec<Permission>,
    /// Whether pledge is active
    active: AtomicBool,
}

impl Clone for PledgePromise {
    fn clone(&self) -> Self {
        Self {
            permissions: self.permissions.clone(),
            active: AtomicBool::new(self.active.load(Ordering::SeqCst)),
        }
    }
}

impl PledgePromise {
    /// Create new pledge promise with specified permissions
    pub fn new(permissions: Vec<Permission>) -> Self {
        Self {
            permissions,
            active: AtomicBool::new(false),
        }
    }

    /// Activate the pledge (can only be done once)
    pub fn activate(&self) -> Result<(), PledgeError> {
        if self.active.load(Ordering::SeqCst) {
            return Err(PledgeError::AlreadyActive);
        }
        self.active.store(true, Ordering::SeqCst);
        Ok(())
    }

    /// Check if permission is allowed
    pub fn allows(&self, permission: Permission) -> bool {
        if !self.active.load(Ordering::SeqCst) {
            return true; // Not activated yet, allow everything
        }
        self.permissions.contains(&permission)
    }

    /// Get all allowed permissions
    pub fn permissions(&self) -> &[Permission] {
        &self.permissions
    }
}

/// Pledge errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PledgeError {
    AlreadyActive,
    InvalidPermission,
    Violation,
}

/// Process pledge manager
pub struct PledgeManager {
    /// Current pledge promise
    pledge: Option<PledgePromise>,
    /// Pre-configured pledge promise for exec child process
    exec_pledge: Option<PledgePromise>,
    /// Capability gate for validation
    gate: CapabilityGate,
}

impl PledgeManager {
    /// Create new pledge manager
    pub fn new() -> Self {
        Self {
            pledge: None,
            exec_pledge: None,
            gate: CapabilityGate::new(),
        }
    }

    /// Pre-configures execpledge promise for process child execution
    pub fn execpledge(&mut self, promise: PledgePromise) -> Result<(), PledgeError> {
        if self.exec_pledge.is_some() {
            return Err(PledgeError::AlreadyActive);
        }
        self.exec_pledge = Some(promise);
        Ok(())
    }

    /// Retrieves active exec_pledge promise if configured
    pub fn active_execpledge(&self) -> Option<&PledgePromise> {
        self.exec_pledge.as_ref()
    }

    /// Set pledge promise for process
    pub fn pledge(&mut self, promise: PledgePromise) -> Result<(), PledgeError> {
        if self.pledge.is_some() {
            return Err(PledgeError::AlreadyActive);
        }
        promise.activate()?;
        self.pledge = Some(promise);

        // Update capability gate based on pledge
        if let Some(ref pledge) = self.pledge {
            let mut token = CapabilityToken::new();
            for &perm in pledge.permissions() {
                match perm {
                    Permission::NetworkTcp => token = token.allow_network("tcp", 0),
                    Permission::NetworkUdp => token = token.allow_network("udp", 0),
                    Permission::FileRead => token = token.allow_read("/"),
                    Permission::FileWrite => token = token.allow_write("/tmp"),
                    Permission::ProcessExec => token = token.allow_exec(),
                    Permission::Ipc => token = token.allow_ipc(),
                }
            }
            self.gate.set_capability(token);
        }

        Ok(())
    }

    /// Validate syscall against pledge
    pub fn validate(&self, permission: Permission) -> Result<(), PledgeError> {
        if let Some(ref pledge) = self.pledge {
            if !pledge.allows(permission) {
                return Err(PledgeError::Violation);
            }
        }
        Ok(())
    }

    /// Get current capability gate
    pub fn gate(&self) -> &CapabilityGate {
        &self.gate
    }
}

impl Default for PledgeManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Common pledge promises
pub mod promises {
    use super::{Permission, PledgePromise};

    /// Stdio promise - basic I/O only
    pub fn stdio() -> PledgePromise {
        PledgePromise::new(vec![Permission::FileRead, Permission::FileWrite])
    }

    /// Network promise - network access
    pub fn network() -> PledgePromise {
        PledgePromise::new(vec![
            Permission::NetworkTcp,
            Permission::NetworkUdp,
            Permission::FileRead,
        ])
    }

    /// Exec promise - can execute processes
    pub fn exec() -> PledgePromise {
        PledgePromise::new(vec![
            Permission::ProcessExec,
            Permission::FileRead,
            Permission::FileWrite,
        ])
    }

    /// IPC promise - inter-process communication
    pub fn ipc() -> PledgePromise {
        PledgePromise::new(vec![Permission::Ipc, Permission::FileRead])
    }

    /// Full promise - all permissions
    pub fn full() -> PledgePromise {
        PledgePromise::new(vec![
            Permission::NetworkTcp,
            Permission::NetworkUdp,
            Permission::FileRead,
            Permission::FileWrite,
            Permission::ProcessExec,
            Permission::Ipc,
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::promises::*;
    use super::*;

    #[test]
    fn test_pledge_creation() {
        let promise = PledgePromise::new(vec![Permission::FileRead]);
        assert!(!promise.active.load(Ordering::SeqCst));
    }

    #[test]
    fn test_pledge_activation() {
        let promise = PledgePromise::new(vec![Permission::FileRead]);
        assert!(promise.activate().is_ok());
        assert!(promise.activate().is_err());
    }

    #[test]
    fn test_pledge_permission_check() {
        let promise = PledgePromise::new(vec![Permission::FileRead]);
        promise.activate().unwrap();
        assert!(promise.allows(Permission::FileRead));
        assert!(!promise.allows(Permission::FileWrite));
    }

    #[test]
    fn test_pledge_manager() {
        let mut manager = PledgeManager::new();
        let promise = stdio();
        assert!(manager.pledge(promise).is_ok());
        assert!(manager.validate(Permission::FileRead).is_ok());
        assert!(manager.validate(Permission::ProcessExec).is_err());
    }

    #[test]
    fn test_common_promises() {
        let stdio_promise = stdio();
        assert!(stdio_promise.allows(Permission::FileRead));

        let network_promise = network();
        assert!(network_promise.allows(Permission::NetworkTcp));

        let full_promise = full();
        assert!(full_promise.allows(Permission::ProcessExec));
    }

    #[test]
    fn test_execpledge_manager() {
        let mut manager = PledgeManager::new();
        assert!(manager.active_execpledge().is_none());

        let exec_p = stdio();
        assert!(manager.execpledge(exec_p).is_ok());
        assert!(manager.active_execpledge().is_some());
        assert!(manager.execpledge(stdio()).is_err()); // Already set
    }
}
