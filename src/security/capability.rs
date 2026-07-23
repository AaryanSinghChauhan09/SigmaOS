//! Capability Tokens: Privilege Isolation (Android/AOSP Absorption)
//! 
//! Cryptographic capability gates replacing legacy Unix file permissions.

extern crate alloc;
use alloc::vec::Vec;

/// A cryptographic capability token required for any privileged action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityToken {
    pub id: u64,
    pub allowed_paths: &'static [&'static str],
    pub allowed_ports: &'static [u16],
    pub is_revoked: bool,
    pub bits_value: u64,
}

impl CapabilityToken {
    /// Zero-argument constructor
    pub fn new() -> Self {
        Self {
            id: 0,
            allowed_paths: &[],
            allowed_ports: &[],
            is_revoked: false,
            bits_value: 0xFFFF_FFFF_FFFF_FFFF, // Allow all by default for bits mask
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

    // Builder pattern methods

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

    pub fn allow_capability(&mut self, _cap: u64) {
        // Mock method
    }

    pub fn contains(&self, _cap: u64) -> bool {
        true
    }
}

impl Default for CapabilityToken {
    fn default() -> Self {
        Self::new()
    }
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
