// Capability-Native Syscall Enforcement
// Implements fine-grained capability tokens at syscall level

use crate::capability::{Capability, CapabilitySet, CapabilityToken};
use crate::error::{SyscallError, Result};

/// Syscall capability enforcement layer
pub struct CapabilitySyscallEnforcer {
    /// Current process capability set
    capabilities: CapabilitySet,
    /// Capability audit log
    audit_log: Vec<SyscallAuditEntry>,
}

/// Audit entry for syscall capability checks
#[derive(Debug, Clone)]
pub struct SyscallAuditEntry {
    pub timestamp: u64,
    pub process_id: u64,
    pub syscall_number: u64,
    pub capability_required: Capability,
    pub granted: bool,
    pub reason: String,
}

impl CapabilitySyscallEnforcer {
    /// Create a new capability enforcer
    pub fn new(capabilities: CapabilitySet) -> Self {
        Self {
            capabilities,
            audit_log: Vec::new(),
        }
    }

    /// Check if a syscall is permitted with current capabilities
    pub fn check_syscall(&mut self, syscall: u64, args: &[u64]) -> Result<()> {
        let required_cap = self.get_required_capability(syscall, args);
        let granted = self.capabilities.has(&required_cap);
        
        let entry = SyscallAuditEntry {
            timestamp: self.get_timestamp(),
            process_id: self.get_process_id(),
            syscall_number: syscall,
            capability_required: required_cap.clone(),
            granted,
            reason: if granted {
                "Capability granted".to_string()
            } else {
                format!("Missing capability: {:?}", required_cap)
            },
        };

        self.audit_log.push(entry);

        if granted {
            Ok(())
        } else {
            Err(SyscallError::PermissionDenied(format!(
                "Syscall {} requires capability {:?}",
                syscall, required_cap
            )))
        }
    }

    /// Get the required capability for a syscall
    fn get_required_capability(&self, syscall: u64, _args: &[u64]) -> Capability {
        match syscall {
            // File operations
            1 => Capability::FileRead,
            2 => Capability::FileWrite,
            3 => Capability::FileExecute,
            
            // Network operations
            10 => Capability::NetworkBind,
            11 => Capability::NetworkConnect,
            12 => Capability::NetworkListen,
            
            // Process operations
            20 => Capability::ProcessCreate,
            21 => Capability::ProcessKill,
            22 => Capability::ProcessDebug,
            
            // Memory operations
            30 => Capability::MemoryAllocate,
            31 => Capability::MemoryProtect,
            
            // Device operations
            40 => Capability::DeviceRead,
            41 => Capability::DeviceWrite,
            
            // System operations
            50 => Capability::SystemAdmin,
            
            _ => Capability::Unknown,
        }
    }

    /// Grant a capability to the current process
    pub fn grant_capability(&mut self, capability: Capability) {
        self.capabilities.insert(capability);
    }

    /// Revoke a capability from the current process
    pub fn revoke_capability(&mut self, capability: &Capability) {
        self.capabilities.remove(capability);
    }

    /// Delegate a capability to another process
    pub fn delegate_capability(&self, capability: Capability, target_pid: u64) -> Result<CapabilityToken> {
        if !self.capabilities.has(&capability) {
            return Err(SyscallError::PermissionDenied(
                "Cannot delegate capability you don't possess".to_string()
            ));
        }

        let token = CapabilityToken::new(capability, self.get_process_id(), target_pid);
        Ok(token)
    }

    /// Validate a capability token
    pub fn validate_token(&self, token: &CapabilityToken) -> bool {
        token.is_valid_for(self.get_process_id())
    }

    /// Get the audit log
    pub fn get_audit_log(&self) -> &[SyscallAuditEntry] {
        &self.audit_log
    }

    /// Clear the audit log
    pub fn clear_audit_log(&mut self) {
        self.audit_log.clear();
    }

    /// Get current timestamp
    fn get_timestamp(&self) -> u64 {
        // In real implementation, this would get actual timestamp
        0
    }

    /// Get current process ID
    fn get_process_id(&self) -> u64 {
        // In real implementation, this would get actual PID
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_enforcement() {
        let mut capabilities = CapabilitySet::new();
        capabilities.insert(Capability::FileRead);
        
        let mut enforcer = CapabilitySyscallEnforcer::new(capabilities);
        
        // Should succeed - has capability
        assert!(enforcer.check_syscall(1, &[]).is_ok());
        
        // Should fail - missing capability
        assert!(enforcer.check_syscall(2, &[]).is_err());
    }

    #[test]
    fn test_capability_delegation() {
        let mut capabilities = CapabilitySet::new();
        capabilities.insert(Capability::FileRead);
        
        let enforcer = CapabilitySyscallEnforcer::new(capabilities);
        
        // Should succeed - has capability to delegate
        assert!(enforcer.delegate_capability(Capability::FileRead, 100).is_ok());
        
        // Should fail - doesn't have capability
        assert!(enforcer.delegate_capability(Capability::FileWrite, 100).is_err());
    }

    #[test]
    fn test_audit_logging() {
        let mut capabilities = CapabilitySet::new();
        capabilities.insert(Capability::FileRead);
        
        let mut enforcer = CapabilitySyscallEnforcer::new(capabilities);
        
        enforcer.check_syscall(1, &[]).unwrap();
        enforcer.check_syscall(2, &[]).unwrap_err();
        
        assert_eq!(enforcer.get_audit_log().len(), 2);
    }
}
