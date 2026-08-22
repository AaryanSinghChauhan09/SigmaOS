// SPDX-License-Identifier: MIT
// SigmaOS Capability-Based Security System
// Implements 64-bit hardware-enforced capability model

extern crate alloc;
use alloc::string::String;
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
        Self { bits }
    }

    pub fn allow_capability(&mut self, bit: u64) {
        self.bits |= bit;
    }

    pub fn allow_network(mut self, protocol: &str, port: u16) -> Self {
        match protocol {
            "tcp" => self.bits |= 1 << (Permission::NetworkTcp as u64),
            "udp" => self.bits |= 1 << (Permission::NetworkUdp as u64),
            _ => {}
        }
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
}

impl Default for SecurityEnforcer {
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
}
