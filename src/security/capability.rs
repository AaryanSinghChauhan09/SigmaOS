// Minimal capability token implementation for SigmaOS
// This provides the basic CapabilityToken structure needed by drivers

use core::default::Default;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityToken {
    pub permissions: u64,
}

impl CapabilityToken {
    pub fn new() -> Self {
        CapabilityToken {
            permissions: 0,
        }
    }

    pub fn from_bits(bits: u64) -> Self {
        CapabilityToken {
            permissions: bits,
        }
    }
    
    pub fn with_permission(mut self, permission: u64) -> Self {
        self.permissions |= permission;
        self
    }

    pub fn bits(&self) -> u64 {
        self.permissions
    }

    pub fn allow_network(self, _proto: &str, _port: u16) -> Self { self }
    pub fn allow_read(self, _path: &str) -> Self { self }
    pub fn allow_write(self, _path: &str) -> Self { self }
    pub fn allow_exec(self) -> Self { self }
    pub fn allow_ipc(self) -> Self { self }
}

impl Default for CapabilityToken {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    FileRead,
    FileWrite,
    ProcessExec,
    NetworkTcp,
    NetworkUdp,
    Ipc,
}

pub struct CapabilityGate {
    pub token: CapabilityToken,
}

impl CapabilityGate {
    pub fn new() -> Self {
        CapabilityGate {
            token: CapabilityToken::new(),
        }
    }

    pub fn set_capability(&mut self, token: CapabilityToken) {
        self.token = token;
    }
}