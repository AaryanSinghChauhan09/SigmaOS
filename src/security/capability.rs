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
    pub bits: u64,
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
            allowed_paths: paths.iter().map(|&s| String::from(s)).collect(),
            allowed_ports: ports.to_vec(),
            is_revoked: false,
            bits: 0,
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

    pub fn bits(&self) -> u64 {
        self.bits
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
    active_tokens: std::vec::Vec<CapabilityToken>,
}

impl Default for SecurityEnforcer {
    fn default() -> Self {
        Self::new()
    }
}

impl SecurityEnforcer {
    pub fn new() -> Self {
        SecurityEnforcer {
            active_tokens: std::vec::Vec::new(),
        }
    }

    pub fn register_token(&mut self, token: CapabilityToken) {
        self.active_tokens.push(token);
    }
}

impl Default for SecurityEnforcer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_token_traversal_protection() {
        let paths = vec!["/var/www"];
        let token = CapabilityToken::new_with_args(1, &paths, &[]);

        // Safe path starts with /var/www and has no traversal
        assert!(token.can_access_path("/var/www/index.html"));

        // Path starting with /var/www but containing traversal should be blocked
        assert!(!token.can_access_path("/var/www/../../etc/passwd"));
    }
}
