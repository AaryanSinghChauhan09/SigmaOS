// SigmaOS Capability-Based Security System
// Implements 64-bit hardware-enforced capability model

use std::string::String;
use std::vec::Vec;
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

/// Capability token representing access rights
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityToken {
    /// 64-bit capability bitmask
    bits: u64,
}

impl Default for CapabilityToken {
    fn default() -> Self {
        Self::new()
    }
}

impl CapabilityToken {
    /// Create a new capability token with no permissions
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    /// Create capability token from raw bits
    pub fn from_bits(bits: u64) -> Self {
        Self { bits }
    }

    /// Allow network access
    pub fn allow_network(mut self, protocol: &str, port: u16) -> Self {
        match protocol {
            "tcp" => self.bits |= 1 << 0,
            "udp" => self.bits |= 1 << 1,
            _ => {}
        }
        self.bits |= (port as u64) << 16;
        self
    }

    /// Allow file read access
    pub fn allow_read(mut self, path: &str) -> Self {
        if path.starts_with("/var/www") || path.starts_with("/mount") {
            self.bits |= 1 << 2;
        }
        self
    }

    /// Allow file write access
    pub fn allow_write(mut self, path: &str) -> Self {
        if path.starts_with("/tmp") || path.starts_with("/home") {
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
        }
    }
}

impl Default for CapabilityGate {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SecurityEnforcer {
    active_tokens: Vec<CapabilityToken>,
}

impl SecurityEnforcer {
    pub fn new() -> Self {
        Self {
            active_tokens: Vec::new(),
        }
    }
}

impl Default for SecurityEnforcer {
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
}
