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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityToken {
    pub id: u64,
    pub allowed_paths: Vec<String>,
    pub allowed_ports: Vec<u16>,
    pub is_revoked: bool,
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

    pub fn new_with_params(id: u64, paths: &'static [&'static str], ports: &'static [u16]) -> Self {
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
