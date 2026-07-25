//! Capability Tokens: Privilege Isolation (Android/AOSP Absorption)
//!
//! Cryptographic capability gates replacing legacy Unix file permissions.

use std::string::String;
use std::vec::Vec;

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
        }
    }

    pub fn new_with_args(id: u64, paths: &[&str], ports: &[u16]) -> Self {
        CapabilityToken {
            id,
            allowed_paths: paths.iter().map(|s| s.to_string()).collect(),
            allowed_ports: ports.to_vec(),
            is_revoked: false,
        }
    }

    pub fn from_bits(bits: u64) -> Self {
        CapabilityToken {
            id: bits,
            allowed_paths: Vec::new(),
            allowed_ports: Vec::new(),
            is_revoked: false,
        }
    }

    pub fn bits(&self) -> u64 {
        self.id
    }

    pub fn allow_network(mut self, _protocol: &str, _port: u16) -> Self {
        self.id |= 1;
        self
    }

    pub fn allow_read(mut self, _path: &str) -> Self {
        self.id |= 2;
        self
    }

    pub fn allow_write(mut self, _path: &str) -> Self {
        self.id |= 4;
        self
    }

    pub fn allow_exec(mut self) -> Self {
        self.id |= 8;
        self
    }

    pub fn allow_ipc(mut self) -> Self {
        self.id |= 16;
        self
    }

    /// Verifies if the token permits access to a given path.
    pub fn can_access_path(&self, path: &str) -> bool {
        if self.is_revoked {
            return false;
        }
        if self.allowed_paths.is_empty() {
            return true; // Allow if no specific restriction
        }
        self.allowed_paths.iter().any(|p| path.starts_with(p))
    }

    /// Verifies if the token permits binding to a network port.
    pub fn can_bind_port(&self, port: u16) -> bool {
        if self.is_revoked {
            return false;
        }
        if self.allowed_ports.is_empty() {
            return true;
        }
        self.allowed_ports.contains(&port)
    }

    pub fn revoke(&mut self) {
        self.is_revoked = true;
    }
}

#[derive(Debug, Clone, Default)]
pub struct CapabilityGate {
    pub active_token: Option<CapabilityToken>,
}

impl CapabilityGate {
    pub fn new() -> Self {
        Self { active_token: None }
    }

    pub fn set_capability(&mut self, token: CapabilityToken) {
        self.active_token = Some(token);
    }
}

pub struct SecurityEnforcer {
    active_tokens: Vec<CapabilityToken>,
}

impl Default for SecurityEnforcer {
    fn default() -> Self {
        Self::new()
    }
}

impl SecurityEnforcer {
    pub fn new() -> Self {
        SecurityEnforcer {
            active_tokens: Vec::new(),
        }
    }

    pub fn register_token(&mut self, token: CapabilityToken) {
        self.active_tokens.push(token);
    }
}
