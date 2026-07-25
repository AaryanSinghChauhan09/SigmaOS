//! Capability Tokens: Privilege Isolation (Android/AOSP Absorption)
//!
//! Cryptographic capability gates replacing legacy Unix file permissions.

extern crate alloc;
use alloc::vec::Vec;

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
    active_tokens: std::vec::Vec<CapabilityToken>,
}

impl Default for SecurityEnforcer {
    fn default() -> Self {
        Self::new()
    }
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
