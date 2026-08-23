// SigmaOS Pledge - Process Privilege Reduction Mechanism
// Inspired by OpenBSD pledge but capability-based

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

#[derive(Debug, Clone)]
pub struct UnveilEntry {
    pub path: String,
    pub permissions: String, // e.g., "r", "rw", "rx"
}

/// Process pledge manager
pub struct PledgeManager {
    /// Current pledge promise
    pledge: Option<PledgePromise>,
    /// Capability gate for validation
    gate: CapabilityGate,
    /// Unveiled paths for filesystem sandboxing
    unveiled_paths: Vec<UnveilEntry>,
}

impl PledgeManager {
    /// Create new pledge manager
    pub fn new() -> Self {
        Self {
            pledge: None,
            gate: CapabilityGate::new(0),
            unveiled_paths: Vec::new(),
        }
    }

    /// Unveil filesystem paths to restrict access (sigma_unveil)
    pub fn unveil(&mut self, path: &str, permissions: &str) -> Result<(), PledgeError> {
        self.unveiled_paths.push(UnveilEntry {
            path: path.to_string(),
            permissions: permissions.to_string(),
        });
        Ok(())
    }

    /// Validate path access against unveil permissions
    pub fn validate_unveil_access(&self, path: &str, requested_perm: char) -> bool {
        if self.unveiled_paths.is_empty() {
            return true; // If no paths are unveiled, allow all accesses
        }

        // Find the most specific match (longest prefix match)
        let mut best_match: Option<&UnveilEntry> = None;
        for entry in &self.unveiled_paths {
            if path.starts_with(&entry.path) {
                match best_match {
                    None => best_match = Some(entry),
                    Some(best) => {
                        if entry.path.len() > best.path.len() {
                            best_match = Some(entry);
                        }
                    }
                }
            }
        }

        if let Some(entry) = best_match {
            entry.permissions.contains(requested_perm)
        } else {
            false // Not in unveiled paths, block access!
        }
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
                    Permission::FileRead => token = token.allow_read(),
                    Permission::FileWrite => token = token.allow_write(),
                    Permission::ProcessExec => token = token.allow_exec(),
                    Permission::Ipc => token = token.allow_ipc(),
                    _ => {}
                }
            }
            self.gate.set_capability(token.bits());
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
    fn test_unveil_sandboxing() {
        let mut manager = PledgeManager::new();

        // Before any unveil, everything is allowed
        assert!(manager.validate_unveil_access("/var/www/index.html", 'r'));
        assert!(manager.validate_unveil_access("/etc/passwd", 'r'));

        // Unveil /var/www for read access, and /tmp for write access
        manager.unveil("/var/www", "r").unwrap();
        manager.unveil("/tmp", "rw").unwrap();

        // Check path within /var/www
        assert!(manager.validate_unveil_access("/var/www/index.html", 'r'));
        assert!(!manager.validate_unveil_access("/var/www/index.html", 'w'));

        // Check path within /tmp
        assert!(manager.validate_unveil_access("/tmp/session.log", 'r'));
        assert!(manager.validate_unveil_access("/tmp/session.log", 'w'));

        // Paths outside of unveiled must be blocked completely
        assert!(!manager.validate_unveil_access("/etc/passwd", 'r'));
    }
}
