//! Capability Tokens: Privilege Isolation (Android/AOSP Absorption)
//!
//! Cryptographic capability gates replacing legacy Unix file permissions.

extern crate alloc;
use alloc::vec::Vec;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityToken {
    pub id: u64,
    pub allowed_paths: Vec<String>,
    pub allowed_ports: Vec<u16>,
    pub is_revoked: bool,
    pub bits_value: u64,
}

impl CapabilityToken {
    /// Zero-argument constructor
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
    }

    /// Construct with ID only
    pub fn new_with_id(id: u64) -> Self {
        Self {
            id,
            allowed_paths: &[],
            allowed_ports: &[],
            is_revoked: false,
            bits_value: 0,
        }
    }

    /// Support bits representation
    pub fn bits(&self) -> u64 {
        self.bits_value
    }

    /// Verifies if the token permits access to a given path.
    pub fn can_access_path(&self, path: &str) -> bool {
        if self.is_revoked {
            return false;
        }
        if self.allowed_paths.is_empty() {
            return true; // Allow if no specific restriction
        }
        self.allowed_paths.iter().any(|&p| path.starts_with(p))
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
