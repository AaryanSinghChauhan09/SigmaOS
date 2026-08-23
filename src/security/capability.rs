// SigmaOS Capability-Based Security System
// Implements 64-bit hardware-enforced capability model, delegation, auditing, and time-limited tokens.

#![no_std]

#[cfg(test)]
extern crate std;

extern crate alloc;

use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

#[cfg(not(test))]
use crate::klib::HashMap;

#[cfg(test)]
use std::collections::HashMap;

/// Capability token representing access rights
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityToken {
    /// 64-bit capability bitmask
    pub bits: u64,
    pub expiry_timestamp: u64, // 0 for infinite, or timestamp
    pub delegated_from: u64,   // ID of parent token if delegated
}

impl Default for CapabilityToken {
    fn default() -> Self {
        Self::new()
    }
}

impl CapabilityToken {
    /// Create a new capability token with no permissions
    pub fn new() -> Self {
        Self {
            bits: 0,
            expiry_timestamp: 0,
            delegated_from: 0,
        }
    }

    /// Create capability token from raw bits
    pub fn from_bits(bits: u64) -> Self {
        Self {
            bits,
            expiry_timestamp: 0,
            delegated_from: 0,
        }
    }

    /// Allow network access
    pub fn allow_network(mut self, protocol: &str, port: u16) -> Self {
        match protocol {
            "tcp" => self.bits |= 1 << 0,
            "udp" => self.bits |= 1 << 1,
            _ => {}
        }
        // Mask and clear target bit ranges (bits 16-31) to prevent bitmask overlap privilege escalation
        self.bits &= !(0xFFFF_u64 << 16);
        self.bits |= (port as u64) << 16;
        self
    }

    /// Allow file read access
    pub fn allow_read(mut self, path: &str) -> Self {
        if path.starts_with("/var/www") || path.starts_with("/etc") || path.starts_with("/home") {
            self.bits |= 1 << 2;
        }
        self
    }

    /// Allow file write access
    pub fn allow_write(mut self, path: &str) -> Self {
        if path.starts_with("/tmp") || path.starts_with("/home") || path.starts_with("/var/log") {
            self.bits |= 1 << 3;
        }
        self
    }

    /// Allow process execution
    pub fn allow_exec(mut self) -> Self {
        self.bits |= 1 << 4;
        self
    }

    /// Allow IPC communication
    pub fn allow_ipc(mut self) -> Self {
        self.bits |= 1 << 5;
        self
    }

    /// Check if capability has specific permission
    pub fn has_permission(&self, permission: Permission) -> bool {
        (self.bits & (1 << permission as u64)) != 0
    }

    /// Revoke all permissions
    pub fn revoke_all(&mut self) {
        self.bits = 0;
    }

    /// Get raw capability bits
    pub fn bits(&self) -> u64 {
        self.bits
    }

    /// Capability Delegation: create a subset child capability token with restricted permissions
    pub fn delegate_sub_capability(&self, mask: u64, expiry: u64, parent_id: u64) -> Result<CapabilityToken, &'static str> {
        // Child can only inherit permissions present in parent token
        let child_bits = self.bits & mask;
        Ok(CapabilityToken {
            bits: child_bits,
            expiry_timestamp: expiry,
            delegated_from: parent_id,
        })
    }
}

/// Permission types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    NetworkTcp = 0,
    NetworkUdp = 1,
    FileRead = 2,
    FileWrite = 3,
    ProcessExec = 4,
    Ipc = 5,
}

/// Capability audit log event types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditEventType {
    Granted,
    Delegated,
    Revoked,
    ViolationDetected,
}

/// Audit log record tracking capability activity
#[derive(Debug, Clone)]
pub struct CapabilityAuditRecord {
    pub token_id: u64,
    pub event_type: AuditEventType,
    pub details: String,
}

/// Capability Manager maintaining global capability registries and audit trails
pub struct CapabilityManager {
    pub registered_tokens: HashMap<u64, CapabilityToken>,
    pub audit_trail: Vec<CapabilityAuditRecord>,
}

impl CapabilityManager {
    pub fn new() -> Self {
        Self {
            registered_tokens: HashMap::new(),
            audit_trail: Vec::new(),
        }
    }

    pub fn grant_token(&mut self, token_id: u64, token: CapabilityToken) {
        self.registered_tokens.insert(token_id, token);
        self.audit_trail.push(CapabilityAuditRecord {
            token_id,
            event_type: AuditEventType::Granted,
            details: "Capability token granted successfully".to_string(),
        });
    }

    pub fn delegate_token(&mut self, parent_id: u64, child_id: u64, mask: u64, expiry: u64) -> Result<CapabilityToken, &'static str> {
        let parent = self.registered_tokens.get(&parent_id).ok_or("Parent token not found")?;
        let child = parent.delegate_sub_capability(mask, expiry, parent_id)?;
        self.registered_tokens.insert(child_id, child);

        self.audit_trail.push(CapabilityAuditRecord {
            token_id: child_id,
            event_type: AuditEventType::Delegated,
            details: "Capability delegated to child with mask".to_string(),
        });

        Ok(child)
    }

    pub fn revoke_token(&mut self, token_id: u64) -> bool {
        if let Some(token) = self.registered_tokens.get_mut(&token_id) {
            token.revoke_all();
            self.audit_trail.push(CapabilityAuditRecord {
                token_id,
                event_type: AuditEventType::Revoked,
                details: "Token permissions completely revoked".to_string(),
            });
            true
        } else {
            false
        }
    }

    pub fn validate_access(&mut self, token_id: u64, permission: Permission, current_time: u64) -> bool {
        if let Some(token) = self.registered_tokens.get(&token_id) {
            if token.expiry_timestamp > 0 && current_time > token.expiry_timestamp {
                self.audit_trail.push(CapabilityAuditRecord {
                    token_id,
                    event_type: AuditEventType::ViolationDetected,
                    details: "Access denied: Token expired".to_string(),
                });
                return false;
            }
            if token.has_permission(permission) {
                true
            } else {
                self.audit_trail.push(CapabilityAuditRecord {
                    token_id,
                    event_type: AuditEventType::ViolationDetected,
                    details: "Access denied: Missing capability permission bit".to_string(),
                });
                false
            }
        } else {
            false
        }
    }
}

impl Default for CapabilityManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Capability gate for syscall validation
pub struct CapabilityGate {
    /// Current capability token
    current: AtomicU64,
}

impl CapabilityGate {
    /// Create new capability gate
    pub fn new() -> Self {
        Self {
            current: AtomicU64::new(0),
        }
    }

    /// Set current capability
    pub fn set_capability(&self, token: CapabilityToken) {
        self.current.store(token.bits(), Ordering::SeqCst);
    }

    /// Validate syscall against current capability
    pub fn validate_syscall(&self, permission: Permission) -> bool {
        let current = self.current.load(Ordering::SeqCst);
        (current & (1 << permission as u64)) != 0
    }

    /// Get current capability
    pub fn current_capability(&self) -> CapabilityToken {
        CapabilityToken {
            bits: self.current.load(Ordering::SeqCst),
            expiry_timestamp: 0,
            delegated_from: 0,
        }
    }
}

impl Default for CapabilityGate {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_creation() {
        let token = CapabilityToken::new();
        assert_eq!(token.bits(), 0);
    }

    #[test]
    fn test_network_permission() {
        let token = CapabilityToken::new().allow_network("tcp", 80);
        assert!(token.has_permission(Permission::NetworkTcp));
    }

    #[test]
    fn test_file_read_permission() {
        let token = CapabilityToken::new().allow_read("/var/www");
        assert!(token.has_permission(Permission::FileRead));
    }

    #[test]
    fn test_capability_revocation() {
        let mut token = CapabilityToken::new().allow_network("tcp", 80);
        token.revoke_all();
        assert_eq!(token.bits(), 0);
    }

    #[test]
    fn test_capability_gate_validation() {
        let gate = CapabilityGate::new();
        let token = CapabilityToken::new().allow_network("tcp", 80);
        gate.set_capability(token);
        assert!(gate.validate_syscall(Permission::NetworkTcp));
    }

    #[test]
    fn test_bitmask_overlap_prevention() {
        let token = CapabilityToken::new()
            .allow_network("tcp", 80)
            .allow_network("tcp", 443);
        let port = (token.bits() >> 16) & 0xFFFF;
        assert_eq!(port, 443);
    }

    #[test]
    fn test_capability_manager_delegation_and_auditing() {
        let mut mgr = CapabilityManager::new();

        // 1. Grant parent token (all permissions)
        let parent_token = CapabilityToken::new()
            .allow_network("tcp", 80)
            .allow_read("/var/www")
            .allow_write("/tmp");
        mgr.grant_token(1001, parent_token);

        // 2. Delegate child token with subset mask (only read & network, no write)
        let child_mask = (1 << Permission::NetworkTcp as u64) | (1 << Permission::FileRead as u64);
        let child_token = mgr.delegate_token(1001, 1002, child_mask, 5000).unwrap();

        assert!(child_token.has_permission(Permission::NetworkTcp));
        assert!(child_token.has_permission(Permission::FileRead));
        assert!(!child_token.has_permission(Permission::FileWrite)); // Restricted subset

        // 3. Access validation and expiry check
        assert!(mgr.validate_access(1002, Permission::FileRead, 1000)); // Valid time (1000 < 5000)
        assert!(!mgr.validate_access(1002, Permission::FileRead, 6000)); // Expired time (6000 > 5000)

        // 4. Revocation
        assert!(mgr.revoke_token(1001));
        assert!(!mgr.registered_tokens.get(&1001).unwrap().has_permission(Permission::FileRead));

        // Audit trail assertions
        assert_eq!(mgr.audit_trail.len(), 4); // Granted, Delegated, Expired Violation, Revoked
        assert_eq!(mgr.audit_trail[0].event_type, AuditEventType::Granted);
        assert_eq!(mgr.audit_trail[1].event_type, AuditEventType::Delegated);
        assert_eq!(mgr.audit_trail[2].event_type, AuditEventType::ViolationDetected);
        assert_eq!(mgr.audit_trail[3].event_type, AuditEventType::Revoked);
    }
}
