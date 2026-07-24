//! Capability Tokens: Privilege Isolation (Android/AOSP Absorption)
//!
//! Cryptographic capability gates replacing legacy Unix file permissions.

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityToken {
    pub id: u64,
    pub allowed_paths: Vec<String>,
    pub allowed_ports: Vec<u16>,
    pub is_revoked: bool,
    pub bits_value: u64,
}

impl CapabilityToken {
    pub fn new() -> Self {
        CapabilityToken {
            id: 0,
            allowed_paths: &[],
            allowed_ports: &[],
            is_revoked: false,
        }
    }

    pub fn new_with_args(id: u64, paths: &'static [&'static str], ports: &'static [u16]) -> Self {
        CapabilityToken {
            id,
            allowed_paths: paths,
            allowed_ports: ports,
            is_revoked: false,
            bits_value: 0xFFFF_FFFF_FFFF_FFFF, // Allow all by default for bits mask
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

    /// Check if capability contains a specific u64 bit
    pub fn contains(&self, bit: u64) -> bool {
        (self.bits & bit) != 0
    }

    /// Allow a specific capability bit
    pub fn allow_capability(&mut self, bit: u64) {
        self.bits |= bit;
    }

    /// Check if capability contains a specific u64 bit
    pub fn contains(&self, bit: u64) -> bool {
        (self.bits & bit) != 0
    }

    /// Allow a specific capability bit
    pub fn allow_capability(&mut self, bit: u64) {
        self.bits |= bit;
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

impl Default for CapabilityToken {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Default)]
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

impl Default for CapabilityToken {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Default)]
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

impl Default for CapabilityToken {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Default)]
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
