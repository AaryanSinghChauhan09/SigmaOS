//! Capability-based Security Framework (Capsicum-inspired)
//! Process capability constraints and sandboxing
//! Inspired by FreeBSD Capsicum with Linux seccomp enhancements

use std::collections::HashSet;
use std::string::String;
use std::vec::Vec;

/// Capability rights (inspired by FreeBSD capsicum rights)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CapRight {
    /// Capability to read from a resource
    CapRead = 0x00000001,
    /// Capability to write to a resource
    CapWrite = 0x00000002,
    /// Capability to execute a resource
    CapExecute = 0x00000004,
    /// Capability to seek within a resource
    CapSeek = 0x00000008,
    /// Capability to create new resources
    CapCreate = 0x00000010,
    /// Capability to perform fstat operations
    CapFstat = 0x00000020,
    /// Capability to perform ioctl operations
    CapIoctl = 0x00000040,
    /// Capability to connect to network endpoints
    CapConnect = 0x00000080,
    /// Capability to accept network connections
    CapAccept = 0x00000100,
    /// Capability to bind to network addresses
    CapBind = 0x00000200,
    /// Capability to perform DNS lookups
    CapDnsLookup = 0x00000400,
    /// Capability to spawn child processes
    CapSpawn = 0x00000800,
    /// Capability to signal other processes
    CapSignal = 0x00001000,
    /// Capability to wait for child processes
    CapWait = 0x00002000,
}

/// Capability mode (sandboxing level)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapMode {
    /// No restrictions (default)
    Unrestricted = 0,
    /// Restrict to allowed capabilities only
    Restricted = 1,
    /// Strict mode - no capability expansion allowed
    Strict = 2,
}

/// Capability entry describing a resource and its rights
#[derive(Debug, Clone)]
pub struct CapEntry {
    pub resource_path: String,
    pub rights: HashSet<CapRight>,
}

impl CapEntry {
    pub fn new(path: &str, rights: &[CapRight]) -> Self {
        Self {
            resource_path: path.to_string(),
            rights: rights.iter().cloned().collect(),
        }
    }

    /// Check if a specific right is granted
    pub fn has_right(&self, right: CapRight) -> bool {
        self.rights.contains(&right)
    }

    /// Grant additional rights
    pub fn grant_right(&mut self, right: CapRight) {
        self.rights.insert(right);
    }

    /// Revoke a right
    pub fn revoke_right(&mut self, right: CapRight) {
        self.rights.remove(&right);
    }
}

/// Capability-based sandbox manager
pub struct CapabilitySandbox {
    mode: CapMode,
    entries: Vec<CapEntry>,
    global_rights: HashSet<CapRight>,
}

impl CapabilitySandbox {
    pub fn new() -> Self {
        Self {
            mode: CapMode::Unrestricted,
            entries: Vec::new(),
            global_rights: HashSet::new(),
        }
    }

    /// Set capability mode
    pub fn set_mode(&mut self, mode: CapMode) {
        self.mode = mode;
    }

    /// Get current capability mode
    pub fn get_mode(&self) -> CapMode {
        self.mode
    }

    /// Add a capability entry
    pub fn add_entry(&mut self, entry: CapEntry) {
        self.entries.push(entry);
    }

    /// Grant a global capability right
    pub fn grant_global_right(&mut self, right: CapRight) {
        self.global_rights.insert(right);
    }

    /// Revoke a global capability right
    pub fn revoke_global_right(&mut self, right: CapRight) {
        self.global_rights.remove(&right);
    }

    /// Check if a specific operation is allowed on a resource
    pub fn check_access(&self, path: &str, required_right: CapRight) -> bool {
        // In unrestricted mode, allow everything
        if self.mode == CapMode::Unrestricted {
            return true;
        }

        // Check global rights first
        if self.global_rights.contains(&required_right) {
            return true;
        }

        // Check specific resource entries
        for entry in &self.entries {
            if path.starts_with(&entry.resource_path) {
                return entry.has_right(required_right);
            }
        }

        // Default deny in restricted mode
        false
    }

    /// Check if spawning child processes is allowed
    pub fn can_spawn(&self) -> bool {
        self.check_access("", CapRight::CapSpawn)
    }

    /// Check if network operations are allowed
    pub fn can_network(&self) -> bool {
        self.global_rights.contains(&CapRight::CapConnect)
            || self.global_rights.contains(&CapRight::CapAccept)
            || self.global_rights.contains(&CapRight::CapBind)
    }

    /// Enter capability mode (sandbox activation)
    pub fn enter_mode(&mut self, mode: CapMode) -> Result<(), String> {
        self.mode = mode;
        Ok(())
    }

    /// Get all capability entries
    pub fn list_entries(&self) -> &[CapEntry] {
        &self.entries
    }

    /// Clear all capability entries
    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }
}

impl Default for CapabilitySandbox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cap_entry_creation() {
        let entry = CapEntry::new("/etc/passwd", &[CapRight::CapRead]);
        assert!(entry.has_right(CapRight::CapRead));
        assert!(!entry.has_right(CapRight::CapWrite));
    }

    #[test]
    fn test_cap_entry_grant_revoke() {
        let mut entry = CapEntry::new("/tmp", &[CapRight::CapRead]);
        entry.grant_right(CapRight::CapWrite);
        assert!(entry.has_right(CapRight::CapWrite));
        entry.revoke_right(CapRight::CapWrite);
        assert!(!entry.has_right(CapRight::CapWrite));
    }

    #[test]
    fn test_sandbox_unrestricted_mode() {
        let sandbox = CapabilitySandbox::new();
        assert_eq!(sandbox.get_mode(), CapMode::Unrestricted);
        assert!(sandbox.check_access("/any/path", CapRight::CapRead));
    }

    #[test]
    fn test_sandbox_restricted_mode() {
        let mut sandbox = CapabilitySandbox::new();
        sandbox.set_mode(CapMode::Restricted);
        let entry = CapEntry::new("/etc/passwd", &[CapRight::CapRead]);
        sandbox.add_entry(entry);

        assert!(sandbox.check_access("/etc/passwd", CapRight::CapRead));
        assert!(!sandbox.check_access("/etc/passwd", CapRight::CapWrite));
        assert!(!sandbox.check_access("/etc/shadow", CapRight::CapRead));
    }

    #[test]
    fn test_global_rights() {
        let mut sandbox = CapabilitySandbox::new();
        sandbox.set_mode(CapMode::Restricted);
        sandbox.grant_global_right(CapRight::CapSpawn);

        assert!(sandbox.can_spawn());
        sandbox.revoke_global_right(CapRight::CapSpawn);
        assert!(!sandbox.can_spawn());
    }

    #[test]
    fn test_network_capabilities() {
        let mut sandbox = CapabilitySandbox::new();
        sandbox.set_mode(CapMode::Restricted);
        sandbox.grant_global_right(CapRight::CapConnect);

        assert!(sandbox.can_network());
    }

    #[test]
    fn test_strict_mode() {
        let mut sandbox = CapabilitySandbox::new();
        sandbox.enter_mode(CapMode::Strict).unwrap();
        assert_eq!(sandbox.get_mode(), CapMode::Strict);
    }

    #[test]
    fn test_path_prefix_matching() {
        let mut sandbox = CapabilitySandbox::new();
        sandbox.set_mode(CapMode::Restricted);
        let entry = CapEntry::new("/home/user", &[CapRight::CapRead, CapRight::CapWrite]);
        sandbox.add_entry(entry);

        assert!(sandbox.check_access("/home/user/file.txt", CapRight::CapRead));
        assert!(sandbox.check_access("/home/user/subdir/doc.pdf", CapRight::CapWrite));
        assert!(!sandbox.check_access("/etc/passwd", CapRight::CapRead));
    }
}
