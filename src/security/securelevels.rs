extern crate alloc;
// Sovereign BSD Securelevels and Granular Linux Capabilities Subsystem
// Integrates core security paradigms from BSD securelevels and Linux capabilities into a unified microkernel privilege manager.

#[cfg(not(feature = "standalone_test"))]
use crate::klib::error::{SecurityError, SigmaError};

#[cfg(feature = "standalone_test")]
#[derive(Debug, PartialEq, Eq)]
pub enum SecurityError {
    AccessDenied,
    PrivilegeEscalationDetected,
}

#[cfg(feature = "standalone_test")]
#[derive(Debug, PartialEq, Eq)]
pub enum SigmaError {
    Security(SecurityError),
}
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU8, Ordering};

/// Granular system capabilities inspired by Linux capability sets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxCapability {
    CapSysAdmin,  // Administrative/Module load controls
    CapNetAdmin,  // Firewall and network configurations
    CapKill,      // Process killing
    CapFileWrite, // Direct raw disk and file write permissions
}

/// Strict system-wide operational states inspired by BSD securelevels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Securelevel {
    Permissive = 0,    // Normal startup defaults
    Secure = 1,        // Disables writing to mounted raw disks and changing kernel modules
    HighlySecure = 2,  // Disables changing system clocks, firewalls, or disk layouts
    NetworkSecure = 3, // Locks all network routing and sockets binding entirely
}

/// Sovereign Manager enforcing securelevel constraints and validating capabilities
pub struct SovereignSecurelevelManager {
    pub current_level: AtomicU8, // Securelevel as u8
    pub assigned_capabilities: Vec<LinuxCapability>,
}

impl SovereignSecurelevelManager {
    /// Initialize manager with permissive level
    pub fn new() -> Self {
        Self {
            current_level: AtomicU8::new(Securelevel::Permissive as u8),
            assigned_capabilities: Vec::new(),
        }
    }

    /// Retrieve the current securelevel
    pub fn securelevel(&self) -> Securelevel {
        match self.current_level.load(Ordering::SeqCst) {
            1 => Securelevel::Secure,
            2 => Securelevel::HighlySecure,
            3 => Securelevel::NetworkSecure,
            _ => Securelevel::Permissive,
        }
    }

    /// Safely raise the system securelevel.
    /// - Securelevel can only be raised, never lowered (can only be reset via reboot).
    pub fn raise_securelevel(&mut self, level: Securelevel) -> Result<(), SigmaError> {
        let current = self.securelevel();
        if level <= current {
            return Err(SigmaError::Security(SecurityError::AccessDenied));
        }
        self.current_level.store(level as u8, Ordering::SeqCst);
        Ok(())
    }

    /// Assign granular capability permissions to the active context
    pub fn grant_capability(&mut self, cap: LinuxCapability) {
        if !self.assigned_capabilities.contains(&cap) {
            self.assigned_capabilities.push(cap);
        }
    }

    /// Revoke capability permissions from the active context
    pub fn revoke_capability(&mut self, cap: LinuxCapability) {
        self.assigned_capabilities.retain(|&c| c != cap);
    }

    /// Validates whether an operation is allowed based on active capabilities AND current system-wide BSD securelevel constraints
    pub fn validate_operation(&self, required_cap: LinuxCapability) -> Result<(), SigmaError> {
        let current_level = self.securelevel();

        // 1. Core BSD Securelevel constraints override assigned capabilities
        match current_level {
            Securelevel::NetworkSecure => {
                // NetworkSecure locks out all network administration
                if required_cap == LinuxCapability::CapNetAdmin {
                    return Err(SigmaError::Security(
                        SecurityError::PrivilegeEscalationDetected,
                    ));
                }
            }
            Securelevel::HighlySecure => {
                // HighlySecure locks out file writes and network administration
                if required_cap == LinuxCapability::CapFileWrite
                    || required_cap == LinuxCapability::CapNetAdmin
                {
                    return Err(SigmaError::Security(
                        SecurityError::PrivilegeEscalationDetected,
                    ));
                }
            }
            Securelevel::Secure => {
                // Secure locks out administrative module loads and disk formats
                if required_cap == LinuxCapability::CapSysAdmin {
                    return Err(SigmaError::Security(
                        SecurityError::PrivilegeEscalationDetected,
                    ));
                }
            }
            Securelevel::Permissive => {}
        }

        // 2. Fall back to checking assigned capabilities
        if self.assigned_capabilities.contains(&required_cap) {
            Ok(())
        } else {
            Err(SigmaError::Security(SecurityError::AccessDenied))
        }
    }
}

impl Default for SovereignSecurelevelManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_securelevel_monotonically_increments() {
        let mut manager = SovereignSecurelevelManager::new();
        assert_eq!(manager.securelevel(), Securelevel::Permissive);

        // Raise level to Secure (1)
        manager.raise_securelevel(Securelevel::Secure).unwrap();
        assert_eq!(manager.securelevel(), Securelevel::Secure);

        // Attempting to lower securelevel back to Permissive (0) should fail!
        assert!(manager.raise_securelevel(Securelevel::Permissive).is_err());
        assert_eq!(manager.securelevel(), Securelevel::Secure);
    }

    #[test]
    fn test_linux_capabilities_and_securelevel_overrides() {
        let mut manager = SovereignSecurelevelManager::new();

        // Grant Network and Admin capabilities
        manager.grant_capability(LinuxCapability::CapNetAdmin);
        manager.grant_capability(LinuxCapability::CapSysAdmin);

        // Under Permissive level, both should pass validation
        assert!(manager
            .validate_operation(LinuxCapability::CapNetAdmin)
            .is_ok());
        assert!(manager
            .validate_operation(LinuxCapability::CapSysAdmin)
            .is_ok());

        // Raise securelevel to Secure (1) -> CAP_SYS_ADMIN_BIT is instantly blocked!
        manager.raise_securelevel(Securelevel::Secure).unwrap();
        assert!(manager
            .validate_operation(LinuxCapability::CapSysAdmin)
            .is_err());
        assert!(manager
            .validate_operation(LinuxCapability::CapNetAdmin)
            .is_ok()); // CapNetAdmin still allowed

        // Raise securelevel to NetworkSecure (3) -> CapNetAdmin is also blocked!
        manager
            .raise_securelevel(Securelevel::NetworkSecure)
            .unwrap();
        assert!(manager
            .validate_operation(LinuxCapability::CapNetAdmin)
            .is_err());
    }
}
