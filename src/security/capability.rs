//! Capability Tokens: Privilege Isolation (Android/AOSP Absorption)
//!
//! Cryptographic capability gates replacing legacy Unix file permissions.

extern crate alloc;
use alloc::vec::Vec;
<<<<<<< HEAD
use alloc::string::String;
=======
use alloc::string::{String, ToString};
>>>>>>> origin/jules-18101178622594638830-97dc43c6

/// Permission enum representing privilege actions
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
#[derive(Debug, Clone)]
pub struct CapabilityToken {
    pub id: u64,
    pub allowed_paths: Vec<String>,
    pub allowed_ports: Vec<u16>,
    pub is_revoked: bool,
<<<<<<< HEAD
    pub bits: u64,
=======
>>>>>>> origin/jules-18101178622594638830-97dc43c6
}

impl CapabilityToken {
    pub fn new() -> Self {
        CapabilityToken {
            id: 0,
            allowed_paths: Vec::new(),
            allowed_ports: Vec::new(),
            is_revoked: false,
<<<<<<< HEAD
            bits: 0,
=======
>>>>>>> origin/jules-18101178622594638830-97dc43c6
        }
    }

    pub fn new_with_params(id: u64, paths: &'static [&'static str], ports: &'static [u16]) -> Self {
<<<<<<< HEAD
        CapabilityToken {
            id,
            allowed_paths: paths.iter().map(|&s| String::from(s)).collect(),
            allowed_ports: ports.to_vec(),
            is_revoked: false,
            bits: 0,
        }
=======
        let mut allowed_paths = Vec::new();
        for &path in paths {
            allowed_paths.push(path.to_string());
        }
        CapabilityToken {
            id,
            allowed_paths,
            allowed_ports: ports.to_vec(),
            is_revoked: false,
        }
    }

    /// Retrieve the token bits/id
    pub fn bits(&self) -> u64 {
        self.id
    }

    /// Builder to allow a network port
    pub fn allow_network(mut self, _proto: &str, port: u16) -> Self {
        // mitigate port-allocation bitmask pollution by masking out bits 16-31
        let masked_port = port & 0xFFFF;
        self.allowed_ports.push(masked_port);
        self
    }

    /// Builder to allow read access on a path
    pub fn allow_read(mut self, path: &str) -> Self {
        if is_safe_path(path) {
            self.allowed_paths.push(path.to_string());
        }
        self
    }

    /// Builder to allow write access on a path
    pub fn allow_write(mut self, path: &str) -> Self {
        if is_safe_path(path) {
            self.allowed_paths.push(path.to_string());
        }
        self
    }

    /// Builder to allow process execution
    pub fn allow_exec(self) -> Self {
        self
    }

    /// Builder to allow IPC access
    pub fn allow_ipc(self) -> Self {
        self
>>>>>>> origin/jules-18101178622594638830-97dc43c6
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

    pub fn allow_network(mut self, _proto: &str, port: u16) -> Self {
        self.bits |= 1;
        if port != 0 {
            self.allowed_ports.push(port);
        }
        self
    }

    pub fn allow_read(mut self, path: &str) -> Self {
        self.bits |= 2;
        self.allowed_paths.push(String::from(path));
        self
    }

    pub fn allow_write(mut self, path: &str) -> Self {
        self.bits |= 4;
        self.allowed_paths.push(String::from(path));
        self
    }

    pub fn allow_exec(mut self) -> Self {
        self.bits |= 8;
        self
    }

    pub fn allow_ipc(mut self) -> Self {
        self.bits |= 16;
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

<<<<<<< HEAD
impl Default for CapabilityToken {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct CapabilityGate {
    pub token: CapabilityToken,
=======
/// Checks if a path is safe to prevent directory traversals and sandbox escapes
pub fn is_safe_path(path: &str) -> bool {
    if path.contains("..") {
        return false;
    }
    true
}

/// Capability gate for process-level capability verification
#[derive(Debug, Clone)]
pub struct CapabilityGate {
    pub active_token: Option<CapabilityToken>,
>>>>>>> origin/jules-18101178622594638830-97dc43c6
}

impl CapabilityGate {
    pub fn new() -> Self {
<<<<<<< HEAD
        Self {
            token: CapabilityToken::new(),
        }
    }

    pub fn set_capability(&mut self, token: CapabilityToken) {
        self.token = token;
    }

    pub fn validate_syscall(&self, permission: Permission) -> bool {
        self.token.has_permission(permission)
    }
}

impl Default for CapabilityGate {
    fn default() -> Self {
        Self::new()
=======
        Self { active_token: None }
    }

    pub fn set_capability(&mut self, token: CapabilityToken) {
        self.active_token = Some(token);
>>>>>>> origin/jules-18101178622594638830-97dc43c6
    }
}

pub struct SecurityEnforcer {
    pub bits: u64,
}

impl SecurityEnforcer {
    pub fn new() -> Self {
<<<<<<< HEAD
        Self { bits: 0 }
    }

    pub fn from_bits(bits: u64) -> Self {
        Self { bits }
    }

    pub fn allow_capability(&mut self, bit: u64) {
        self.bits |= bit;
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
        if path.starts_with("/var/www") || path == "/" {
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

    pub fn allow_capability(&mut self, bitmask: u64) {
        self.bits |= bitmask;
    }

    pub fn contains(&self, bitmask: u64) -> bool {
        (self.bits & bitmask) == bitmask
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
=======
        SecurityEnforcer {
            active_tokens: Vec::new(),
        }
    }

    pub fn register_token(&mut self, token: CapabilityToken) {
        self.active_tokens.push(token);
    }
}

impl Default for CapabilityToken {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for CapabilityGate {
    fn default() -> Self {
        Self::new()
>>>>>>> origin/jules-18101178622594638830-97dc43c6
    }
}
