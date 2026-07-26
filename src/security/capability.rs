// SigmaOS Capability-Based Security System
// Implements 64-bit hardware-enforced capability model

<<<<<<< HEAD
use std::string::String;
use std::vec::Vec;

/// Capability token representing access rights
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityToken {
    /// 64-bit capability bitmask
    bits: u64,
}

impl CapabilityToken {
    /// Create a new capability token with no permissions
=======
use std::vec::Vec;
use std::string::String;
use std::string::ToString;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    NetworkTcp,
    NetworkUdp,
    FileRead,
    FileWrite,
    ProcessExec,
    Ipc,
}

/// A cryptographic capability token required for any privileged action.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityToken {
    pub id: u64,
    pub allowed_paths: Vec<String>,
    pub allowed_ports: Vec<u16>,
    pub is_revoked: bool,
    pub permissions: Vec<Permission>,
}

impl Default for CapabilityToken {
    fn default() -> Self {
        Self::new()
    }
}

impl CapabilityToken {
    pub fn new() -> Self {
        CapabilityToken {
            id: 0,
            allowed_paths: Vec::new(),
            allowed_ports: Vec::new(),
            is_revoked: false,
            permissions: Vec::new(),
        }
    }

    pub fn bits(&self) -> u64 {
        let mut bits = 0;
        for &perm in &self.permissions {
            match perm {
                Permission::NetworkTcp => bits |= 1 << 0,
                Permission::NetworkUdp => bits |= 1 << 1,
                Permission::FileRead => bits |= 1 << 2,
                Permission::FileWrite => bits |= 1 << 3,
                Permission::ProcessExec => bits |= 1 << 4,
                Permission::Ipc => bits |= 1 << 5,
            }
        }
        bits
    }

    pub fn allow_network(mut self, _proto: &str, port: u16) -> Self {
        self.allowed_ports.push(port);
        self.permissions.push(Permission::NetworkTcp);
        self.permissions.push(Permission::NetworkUdp);
        self
    }

    pub fn allow_read(mut self, path: &str) -> Self {
        self.allowed_paths.push(path.to_string());
        self.permissions.push(Permission::FileRead);
        self
    }

    pub fn allow_write(mut self, path: &str) -> Self {
        self.allowed_paths.push(path.to_string());
        self.permissions.push(Permission::FileWrite);
        self
    }

    pub fn allow_exec(mut self) -> Self {
        self.permissions.push(Permission::ProcessExec);
        self
    }

    pub fn allow_ipc(mut self) -> Self {
        self.permissions.push(Permission::Ipc);
        self
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
}

pub struct CapabilityGate {
    pub token: Option<CapabilityToken>,
}

impl CapabilityGate {
    pub fn new() -> Self {
        Self { token: None }
    }
    pub fn set_capability(&mut self, token: CapabilityToken) {
        self.token = Some(token);
    }
}

pub struct SecurityEnforcer {
    active_tokens: Vec<CapabilityToken>,
}

impl SecurityEnforcer {
>>>>>>> origin/improve-os-architecture-13148548228877311559
    pub fn new() -> Self {
        Self { bits: 0 }
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
        if path.starts_with("/var/www") {
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
