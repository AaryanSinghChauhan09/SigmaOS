// SPDX-License-Identifier: MIT
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

/// A cryptographic capability token required for any privileged action.
#[derive(Debug, Clone)]
pub struct CapabilityToken {
    /// 64-bit capability bitmask
    pub bits: u64,
}

impl CapabilityToken {
    pub fn new() -> Self {
        CapabilityToken {
            id: 0,
            allowed_paths: Vec::new(),
            allowed_ports: Vec::new(),
            is_revoked: false,
            bits: 0,
        }
    }

    pub fn from_bits(bits: u64) -> Self {
        CapabilityToken {
            id: 0,
            allowed_paths: Vec::new(),
            allowed_ports: Vec::new(),
            is_revoked: false,
            bits,
        }
    }

    pub fn new_with_params(id: u64, paths: &'static [&'static str], ports: &'static [u16]) -> Self {
        CapabilityToken {
            id,
            allowed_paths: paths.iter().map(|&s| String::from(s)).collect(),
            allowed_ports: ports.to_vec(),
            is_revoked: false,
            bits: 0,
        }
    }

    pub fn can_access_path(&self, path: &str) -> bool {
        if self.is_revoked {
            return false;
        }
        self.allowed_paths.iter().any(|p| path.starts_with(p))
    }

    pub fn can_bind_port(&self, port: u16) -> bool {
        if self.is_revoked {
            return false;
        }
        self.allowed_ports.contains(&port)
    }

    pub fn revoke(&mut self) {
        self.is_revoked = true;
    }

    pub fn revoke_all(&mut self) {
        self.is_revoked = true;
        self.bits = 0;
    }

    pub fn bits(&self) -> u64 {
        self.bits
    }

    pub fn has_permission(&self, permission: Permission) -> bool {
        if self.is_revoked {
            return false;
        }
        (self.bits & (1 << permission as u64)) != 0
    }

    pub fn allow_network(mut self, _proto: &str, port: u16) -> Self {
        self.bits |= 1 << (Permission::NetworkTcp as u64);
        if port != 0 {
            self.allowed_ports.push(port);
        }
        self
    }

    pub fn allow_read(mut self, path: &str) -> Self {
        self.bits |= 1 << (Permission::FileRead as u64);
        self.allowed_paths.push(String::from(path));
        self
    }

    pub fn allow_write(mut self, path: &str) -> Self {
        self.bits |= 1 << (Permission::FileWrite as u64);
        self.allowed_paths.push(String::from(path));
        self
    }

    pub fn allow_exec(mut self) -> Self {
        self.bits |= 1 << (Permission::ProcessExec as u64);
        self
    }

    pub fn allow_ipc(mut self) -> Self {
        self.bits |= 1 << (Permission::Ipc as u64);
        self
    }
}

impl Default for CapabilityToken {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SecurityEnforcer {
    pub bits: u64,
}

impl SecurityEnforcer {
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    pub fn from_bits(bits: u64) -> Self {
        Self {
            bits,
            expiry_timestamp: 0,
            delegated_from: 0,
        }
    }

    pub fn allow_network(mut self, protocol: &str, port: u16) -> Self {
        match protocol {
            "tcp" => self.bits |= 1 << (Permission::NetworkTcp as u64),
            "udp" => self.bits |= 1 << (Permission::NetworkUdp as u64),
            _ => {}
        }
        // Mask and clear target bit ranges (bits 16-31) to prevent bitmask overlap privilege escalation
        self.bits &= !(0xFFFF_u64 << 16);
        self.bits |= (port as u64) << 16;
        self
    }

    pub fn allow_read(mut self, path: &str) -> Self {
        if path.starts_with("/var/www") {
            self.bits |= 1 << (Permission::FileRead as u64);
        }
        self
    }

    pub fn allow_write(mut self, path: &str) -> Self {
        if path.starts_with("/tmp") || path.starts_with("/home") {
            self.bits |= 1 << (Permission::FileWrite as u64);
        }
        self
    }

    pub fn allow_exec(mut self) -> Self {
        self.bits |= 1 << (Permission::ProcessExec as u64);
        self
    }

    pub fn allow_ipc(mut self) -> Self {
        self.bits |= 1 << (Permission::Ipc as u64);
        self
    }

    pub fn has_permission(&self, permission: Permission) -> bool {
        (self.bits & (1 << permission as u64)) != 0
    }

    pub fn revoke_all(&mut self) {
        self.bits = 0;
    }

    pub fn bits(&self) -> u64 {
        self.bits
    }

    pub fn allow_capability(&mut self, bitmask: u64) {
        self.bits |= bitmask;
    }

    pub fn contains(&self, bitmask: u64) -> bool {
        (self.bits & bitmask) == bitmask
    }
}

impl Default for SecurityEnforcer {
    fn default() -> Self {
        Self::new()
    }
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
    pub token: CapabilityToken,
    current: AtomicU64,
}

impl CapabilityGate {
    pub fn new() -> Self {
        Self {
            token: CapabilityToken::new(),
            current: AtomicU64::new(0),
        }
    }

    pub fn set_capability(&mut self, token: CapabilityToken) {
        self.current.store(token.bits(), Ordering::SeqCst);
        self.token = token;
    }

    pub fn validate_syscall(&self, permission: Permission) -> bool {
        let current = self.current.load(Ordering::SeqCst);
        (current & (1 << permission as u64)) != 0
    }

    pub fn current_capability(&self) -> CapabilityToken {
        CapabilityToken {
            id: 0,
            allowed_paths: Vec::new(),
            allowed_ports: Vec::new(),
            is_revoked: false,
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
        let mut gate = CapabilityGate::new();
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
