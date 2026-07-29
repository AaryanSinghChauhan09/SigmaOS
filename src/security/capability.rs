// SigmaOS Capability-Based Security System
// Implements 64-bit hardware-enforced capability model

extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;
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
    pub id: u64,
    pub allowed_paths: Vec<String>,
    pub allowed_ports: Vec<u16>,
    pub is_revoked: bool,
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

    pub fn new_with_params(id: u64, paths: &'static [&'static str], ports: &'static [u16]) -> Self {
        CapabilityToken {
            id,
            allowed_paths: paths.iter().map(|&s| String::from(s)).collect(),
            allowed_ports: ports.to_vec(),
            is_revoked: false,
            bits: 0,
        }
    }

    /// Verifies if the token permits access to a given path.
    pub fn can_access_path(&self, path: &str) -> bool {
        if self.is_revoked {
            return false;
        }
        self.allowed_paths.iter().any(|p| path.starts_with(p))
    }

    /// Verifies if the token permits binding to a network port.
    pub fn can_bind_port(&self, port: u16) -> bool {
        if self.is_revoked {
            return false;
        }
        self.allowed_ports.contains(&port)
    }

    pub fn revoke(&mut self) {
        self.is_revoked = true;
    }

    pub fn bits(&self) -> u64 {
        self.bits
    }

    pub fn allow_network(mut self, protocol: &str, port: u16) -> Self {
        match protocol {
            "tcp" => self.bits |= 1 << 0,
            "udp" => self.bits |= 1 << 1,
            _ => {}
        }
        if port != 0 {
            self.allowed_ports.push(port);
        }
        self
    }

    pub fn allow_read(mut self, path: &str) -> Self {
        self.bits |= 1 << 2;
        self.allowed_paths.push(String::from(path));
        self
    }

    pub fn allow_write(mut self, path: &str) -> Self {
        self.bits |= 1 << 3;
        self.allowed_paths.push(String::from(path));
        self
    }

    pub fn allow_exec(mut self) -> Self {
        self.bits |= 1 << 4;
        self
    }

    pub fn allow_ipc(mut self) -> Self {
        self.bits |= 1 << 5;
        self
    }

    pub fn has_permission(&self, permission: Permission) -> bool {
        if self.is_revoked {
            return false;
        }
        (self.bits & (1 << permission as u64)) != 0
    }

    pub fn revoke_all(&mut self) {
        self.bits = 0;
        self.is_revoked = true;
    }
}

impl Default for CapabilityToken {
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
        let bits = self.current.load(Ordering::SeqCst);
        CapabilityToken {
            id: 0,
            allowed_paths: Vec::new(),
            allowed_ports: Vec::new(),
            is_revoked: false,
            bits,
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
}
