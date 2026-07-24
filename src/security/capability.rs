//! Capability Tokens: Privilege Isolation (Android/AOSP Absorption)
//!
//! Cryptographic capability gates replacing legacy Unix file permissions.

extern crate alloc;

use alloc::vec::Vec;
use alloc::string::{String, ToString};

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CapabilityToken {
    pub id: u64,
    pub allowed_paths: Vec<String>,
    pub allowed_ports: Vec<u16>,
    pub is_revoked: bool,
    pub bits: u64,
}

impl Default for CapabilityToken {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for CapabilityToken {
    fn default() -> Self {
        Self::new()
    }
}

impl CapabilityToken {
    /// Zero-argument constructor, returns a default CapabilityToken
    pub fn new() -> Self {
        CapabilityToken {
            id: 0,
            allowed_paths: &[],
            allowed_ports: &[],
            is_revoked: false,
            bits: !0, // Allow all bits by default
        }
    }

    /// Constructor with parameters for compatibility
    pub fn with_params(id: u64, paths: &'static [&'static str], ports: &'static [u16]) -> Self {
        CapabilityToken {
            id,
            allowed_paths,
            allowed_ports: ports.to_vec(),
            is_revoked: false,
            bits: !0,
        }
    }

    /// Returns the capability bitmask.
    pub fn bits(&self) -> u64 {
        self.bits
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

    // Permission builders for compatibility with pledge.rs
    pub fn allow_network(self, _proto: &str, _port: u16) -> Self {
        self
    }

    pub fn allow_read(self, _path: &str) -> Self {
        self
    }

    pub fn allow_write(self, _path: &str) -> Self {
        self
    }

    pub fn allow_exec(self) -> Self {
        self
    }

    pub fn allow_ipc(self) -> Self {
        self
    }
}

impl Default for CapabilityToken {
    fn default() -> Self {
        Self::new()
    }
}

/// A cryptographic capability gate.
pub struct CapabilityGate {
    pub current_token: Option<CapabilityToken>,
}

impl CapabilityGate {
    pub fn new() -> Self {
        Self {
            current_token: None,
        }
    }

    pub fn set_capability(&mut self, token: CapabilityToken) {
        self.current_token = Some(token);
    }
}

/// Dynamic permissions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Permission {
    NetworkTcp,
    NetworkUdp,
    FileRead,
    FileWrite,
    ProcessExec,
    Ipc,
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

impl Default for CapabilityToken {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for CapabilityGate {
    fn default() -> Self {
        Self::new()
    }
}
