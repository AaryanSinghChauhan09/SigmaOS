// SigmaOS Capability-Based Security System
// Implements 64-bit hardware-enforced capability model

use core::sync::atomic::{AtomicU64, Ordering};

/// Capability token representing access rights
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityToken {
    /// 64-bit capability bitmask
    bits: u64,
}

fn is_safe_path(path: &str) -> bool {
    // Check for directory traversal sequences to block attacks
    if path.contains("../") || path.contains("/..") || path == ".." || path.starts_with("../") {
        return false;
    }
    true
}

impl CapabilityToken {
    /// Create a new capability token with no permissions
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    /// Allow network access
    pub fn allow_network(mut self, protocol: &str, port: u16) -> Self {
        match protocol {
            "tcp" => self.bits |= 1 << 0,
            "udp" => self.bits |= 1 << 1,
            _ => {}
        }
        // Clear previous port bits (bits 16 to 31) to prevent bitmask pollution
        self.bits &= !(0xFFFF << 16);
        self.bits |= (port as u64) << 16;
        self
    }

    /// Allow file read access
    pub fn allow_read(mut self, path: &str) -> Self {
        if is_safe_path(path) && path.starts_with("/var/www") {
            self.bits |= 1 << 2;
        }
        self
    }

    /// Allow file write access
    pub fn allow_write(mut self, path: &str) -> Self {
        if is_safe_path(path) && (path.starts_with("/tmp") || path.starts_with("/home")) {
            self.bits |= 1 << 3;
        }
        self
    }

    /// Allow process execution
    pub fn allow_exec(mut self) -> Self {
        self.bits |= 1 << 4;
        self
    }

    /// Allow IPC communication
    pub fn allow_ipc(mut self) -> Self {
        self.bits |= 1 << 5;
        self
    }

    /// Check if capability has specific permission
    pub fn has_permission(&self, permission: Permission) -> bool {
        (self.bits & (1 << permission as u64)) != 0
    }

    /// Revoke all permissions
    pub fn revoke_all(&mut self) {
        self.bits = 0;
    }

    /// Get raw capability bits
    pub fn bits(&self) -> u64 {
        self.bits
    }

    pub fn allow_capability(&mut self, bitmask: u64) {
        self.bits |= bitmask;
    }

    pub fn contains(&self, bitmask: u64) -> bool {
        (self.bits & bitmask) == bitmask
    }
}

/// Permission types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    NetworkTcp = 0,
    NetworkUdp = 1,
    FileRead = 2,
    FileWrite = 3,
    ProcessExec = 4,
    Ipc = 5,
}

/// OOP SecurityEnforcer trait for policy verification
pub trait SecurityEnforcer {
    /// Verify access of a capability token for a specific permission
    fn verify_access(&self, token: &CapabilityToken, permission: Permission) -> bool;
}

/// OOP-based ZeroTrustVerifier implementing SecurityEnforcer
pub struct ZeroTrustVerifier {
    /// Zero-trust policy strictness level
    is_strict: bool,
}

impl ZeroTrustVerifier {
    /// Create a new ZeroTrustVerifier
    pub fn new(is_strict: bool) -> Self {
        Self { is_strict }
    }
}

impl SecurityEnforcer for ZeroTrustVerifier {
    fn verify_access(&self, token: &CapabilityToken, permission: Permission) -> bool {
        if self.is_strict {
            // Under strict zero-trust, we only allow access if the capability token explicitly supports it
            token.has_permission(permission)
        } else {
            // Under standard zero-trust, always verify capability has the permission
            token.has_permission(permission)
        }
    }
}

/// Capability gate for syscall validation
pub struct CapabilityGate {
    /// Current capability token
    current: AtomicU64,
}

impl CapabilityGate {
    /// Create new capability gate
    pub fn new() -> Self {
        Self {
            current: AtomicU64::new(0),
        }
    }

    /// Set current capability
    pub fn set_capability(&self, token: CapabilityToken) {
        self.current.store(token.bits(), Ordering::SeqCst);
    }

    /// Validate syscall against current capability
    pub fn validate_syscall(&self, permission: Permission) -> bool {
        let current = self.current.load(Ordering::SeqCst);
        (current & (1 << permission as u64)) != 0
    }

    /// Get current capability
    pub fn current_capability(&self) -> CapabilityToken {
        CapabilityToken {
            bits: self.current.load(Ordering::SeqCst),
        }
    }
}

impl Default for CapabilityGate {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_creation() {
        let token = CapabilityToken::new();
        assert_eq!(token.bits(), 0);
    }

    #[test]
    fn test_network_permission() {
        let token = CapabilityToken::new().allow_network("tcp", 80);
        assert!(token.has_permission(Permission::NetworkTcp));
    }

    #[test]
    fn test_file_read_permission() {
        let token = CapabilityToken::new().allow_read("/var/www");
        assert!(token.has_permission(Permission::FileRead));
    }

    #[test]
    fn test_capability_revocation() {
        let mut token = CapabilityToken::new().allow_network("tcp", 80);
        token.revoke_all();
        assert_eq!(token.bits(), 0);
    }

    #[test]
    fn test_capability_gate_validation() {
        let gate = CapabilityGate::new();
        let token = CapabilityToken::new().allow_network("tcp", 80);
        gate.set_capability(token);
        assert!(gate.validate_syscall(Permission::NetworkTcp));
    }

    #[test]
    fn test_path_traversal_rejection() {
        // Attempting traversal with /.. or ../ should not grant permission
        let token1 = CapabilityToken::new().allow_read("/var/www/../../etc/passwd");
        assert!(!token1.has_permission(Permission::FileRead));

        let token2 = CapabilityToken::new().allow_write("/tmp/../etc/shadow");
        assert!(!token2.has_permission(Permission::FileWrite));

        let token3 = CapabilityToken::new().allow_read("/var/www/safe_subdir/file.txt");
        assert!(token3.has_permission(Permission::FileRead));
    }

    #[test]
    fn test_port_mask_isolation() {
        // Setting port 80 and then port 443 should cleanly isolate the port bits
        let token = CapabilityToken::new()
            .allow_network("tcp", 80)
            .allow_network("tcp", 443);

        // Extracted port value should be exactly 443, not 507 (overlap of 80 and 443)
        let extracted_port = (token.bits() >> 16) & 0xFFFF;
        assert_eq!(extracted_port, 443);
    }
}
