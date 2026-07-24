//! Capability Tokens: Privilege Isolation (Android/AOSP Absorption)
//!
//! Cryptographic capability gates replacing legacy Unix file permissions.

use alloc::vec::Vec;

/// A cryptographic capability token required for any privileged action.
pub struct CapabilityToken {
    pub id: u64,
    pub allowed_paths: &'static [&'static str],
    pub allowed_ports: &'static [u16],
    pub is_revoked: bool,
}

impl CapabilityToken {
    pub fn new(id: u64, paths: &'static [&'static str], ports: &'static [u16]) -> Self {
        CapabilityToken {
            id,
            allowed_paths: paths,
            allowed_ports: ports,
            is_revoked: false,
        }
    }

    /// Verifies if the token permits access to a given path.
    pub fn can_access_path(&self, path: &str) -> bool {
        if self.is_revoked {
            return false;
        }
        self.allowed_paths.iter().any(|&p| path.starts_with(p))
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
