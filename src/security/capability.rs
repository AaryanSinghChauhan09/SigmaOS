//! Capability Tokens: Privilege Isolation (Android/AOSP Absorption)
//!
//! Cryptographic capability gates replacing legacy Unix file permissions.

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
}

impl Default for CapabilityToken {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct CapabilityGate {
    pub token: CapabilityToken,
}

impl CapabilityGate {
    pub fn new() -> Self {
        Self {
            token: CapabilityToken::new(),
        }
    }

    pub fn set_capability(&mut self, token: CapabilityToken) {
        self.token = token;
    }
}

impl Default for CapabilityGate {
    fn default() -> Self {
        Self::new()
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
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    pub fn register_token(&mut self, token: CapabilityToken) {
        self.active_tokens.push(token);
    }
}
