//! Capability-based Security Framework (Capsicum-inspired)
//! Process capability constraints and sandboxing
//! Inspired by FreeBSD Capsicum with Linux seccomp enhancements
//!
//! # FreeBSD Capsicum Integration
//! This module provides a Rust-native implementation of FreeBSD's Capsicum
//! capability model, allowing fine-grained rights delegation for file descriptors
//! and process-level sandboxing.
//!
//! ## Core Concepts
//!
//! 1. **Capability Mode**: Process enters a restricted sandbox via `cap_enter()` (irreversible)
//! 2. **File Descriptor Rights**: Each FD has a capability rights mask that can only be narrowed
//! 3. **Global Namespace Denial**: In capability mode, syscalls like `open()` are denied
//!
//! ## Usage Example
//!
//! ```no_run
//! use sigmaos::security::{ProcessCapState, CapRightsMask, CAP_READ, CAP_WRITE};
//!
//! let mut state = ProcessCapState::new();
//! state.limit_fd_rights(3, CapRightsMask::new(CAP_READ)).unwrap();
//! state.cap_enter().unwrap();
//! // Now process cannot open new files, only use existing FDs with limited rights
//! ```
//!
//! ## References
//!
//! - FreeBSD `cap_enter(2)`, `cap_rights_limit(2)` man pages
//! - Capsicum: practical capabilities for UNIX (USENIX Security 2010)

#![cfg_attr(not(any(feature = "standalone_test", test)), no_std)]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::collections::BTreeMap;
#[cfg(any(feature = "standalone_test", test))]
use std::collections::BTreeMap;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::collections::BTreeSet as HashSet;
#[cfg(any(feature = "standalone_test", test))]
use std::collections::HashSet;

use crate::security::cap_rights::CapRightsMask;

/// Capability rights (inspired by FreeBSD capsicum rights)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    /// Enforce capability check for syscall operations
    /// Returns Ok(()) if allowed, Err with errno if denied
    pub fn enforce_syscall(&self, operation: &str, path: Option<&str>) -> Result<(), i32> {
        const EACCES: i32 = 13; // Permission denied
        const ECAPMODE: i32 = 94; // Not permitted in capability mode

        // Map operation strings to capability rights
        let required_right = match operation {
            "read" => CapRight::CapRead,
            "write" => CapRight::CapWrite,
            "execute" => CapRight::CapExecute,
            "fstat" => CapRight::CapFstat,
            "ioctl" => CapRight::CapIoctl,
            "connect" => CapRight::CapConnect,
            "accept" => CapRight::CapAccept,
            "bind" => CapRight::CapBind,
            "fork" | "spawn" => CapRight::CapSpawn,
            "kill" | "signal" => CapRight::CapSignal,
            _ => return Err(EACCES), // Unknown operation
        };

        // Check capability
        if let Some(resource_path) = path {
            if self.check_access(resource_path, required_right) {
                Ok(())
            } else {
                Err(if self.mode != CapMode::Unrestricted {
                    ECAPMODE
                } else {
                    EACCES
                })
            }
        } else {
            // Global capability check (no specific path)
            if self.global_rights.contains(&required_right)
                || self.mode == CapMode::Unrestricted
            {
                Ok(())
            } else {
                Err(ECAPMODE)
            }
        }
    }
}

impl Default for CapabilitySandbox {
    fn default() -> Self {
        Self::new()
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// NEW COMPLETE CAPSICUM IMPLEMENTATION (FreeBSD-inspired)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Capability mode state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityMode {
    /// Normal mode - full system access
    Normal,
    /// Capability mode - restricted sandbox (irreversible)
    Capability,
}

/// Per-FD capability rights entry
#[derive(Debug, Clone)]
pub struct FdRightsEntry {
    pub fd: i32,
    pub rights: CapRightsMask,
    pub ioctl_allowlist: Vec<u64>,
    pub fcntl_allowlist: u32,
}

impl FdRightsEntry {
    pub fn new(fd: i32, rights: CapRightsMask) -> Self {
        Self {
            fd,
            rights,
            ioctl_allowlist: Vec::new(),
            fcntl_allowlist: 0xFFFFFFFF, // All fcntls allowed by default
        }
    }
}

/// Process capability state (main Capsicum engine)
pub struct ProcessCapState {
    mode: CapabilityMode,
    fd_table: BTreeMap<i32, FdRightsEntry>,
}

impl ProcessCapState {
    pub fn new() -> Self {
        Self {
            mode: CapabilityMode::Normal,
            fd_table: BTreeMap::new(),
        }
    }

    /// Enter capability mode (irreversible operation)
    pub fn cap_enter(&mut self) -> Result<(), CapError> {
        if self.mode == CapabilityMode::Capability {
            // Already in capability mode - idempotent
            return Ok(());
        }
        self.mode = CapabilityMode::Capability;
        Ok(())
    }

    /// Check if process is in capability mode
    pub fn is_cap_mode(&self) -> bool {
        self.mode == CapabilityMode::Capability
    }

    /// Limit FD rights (can only narrow, never expand)
    pub fn limit_fd_rights(&mut self, fd: i32, rights: CapRightsMask) -> Result<(), CapError> {
        if let Some(entry) = self.fd_table.get_mut(&fd) {
            // Check that new rights are a subset (can only narrow)
            if !rights.is_subset_of(&entry.rights) {
                return Err(CapError::RightsExpansionDenied);
            }
            entry.rights = rights;
        } else {
            // New FD entry
            self.fd_table.insert(fd, FdRightsEntry::new(fd, rights));
        }
        Ok(())
    }

    /// Get FD rights
    pub fn get_fd_rights(&self, fd: i32) -> Option<&CapRightsMask> {
        self.fd_table.get(&fd).map(|e| &e.rights)
    }

    /// Check if FD has a specific right
    pub fn check_fd_right(&self, fd: i32, right: u64) -> Result<(), CapError> {
        if let Some(entry) = self.fd_table.get(&fd) {
            if entry.rights.has(right) {
                Ok(())
            } else {
                Err(CapError::InsufficientRights {
                    required: right,
                    actual: entry.rights,
                })
            }
        } else {
            // FD not in table - if in cap mode, deny; otherwise allow
            if self.is_cap_mode() {
                Err(CapError::FdNotFound)
            } else {
                Ok(())
            }
        }
    }

    /// Check if namespace access is allowed (denied in capability mode)
    pub fn check_namespace_access(&self) -> Result<(), CapError> {
        if self.is_cap_mode() {
            Err(CapError::CapModeViolation)
        } else {
            Ok(())
        }
    }

    /// Inherit rights on FD duplication (dup/dup2)
    pub fn inherit_rights_on_dup(&mut self, old_fd: i32, new_fd: i32) -> Result<(), CapError> {
        if let Some(old_entry) = self.fd_table.get(&old_fd) {
            let new_entry = FdRightsEntry {
                fd: new_fd,
                rights: old_entry.rights,
                ioctl_allowlist: old_entry.ioctl_allowlist.clone(),
                fcntl_allowlist: old_entry.fcntl_allowlist,
            };
            self.fd_table.insert(new_fd, new_entry);
            Ok(())
        } else {
            // Old FD not in table - if in cap mode, deny
            if self.is_cap_mode() {
                Err(CapError::FdNotFound)
            } else {
                Ok(())
            }
        }
    }

    /// Limit allowed ioctl commands for an FD
    pub fn limit_ioctls(&mut self, fd: i32, cmds: Vec<u64>) -> Result<(), CapError> {
        if let Some(entry) = self.fd_table.get_mut(&fd) {
            entry.ioctl_allowlist = cmds;
            Ok(())
        } else {
            Err(CapError::FdNotFound)
        }
    }

    /// Limit allowed fcntl commands for an FD
    pub fn limit_fcntls(&mut self, fd: i32, mask: u32) -> Result<(), CapError> {
        if let Some(entry) = self.fd_table.get_mut(&fd) {
            entry.fcntl_allowlist = mask;
            Ok(())
        } else {
            Err(CapError::FdNotFound)
        }
    }

    /// Check if an ioctl command is allowed
    pub fn check_ioctl(&self, fd: i32, cmd: u64) -> Result<(), CapError> {
        if let Some(entry) = self.fd_table.get(&fd) {
            if entry.ioctl_allowlist.is_empty() || entry.ioctl_allowlist.contains(&cmd) {
                Ok(())
            } else {
                Err(CapError::IoctlNotAllowed)
            }
        } else {
            if self.is_cap_mode() {
                Err(CapError::FdNotFound)
            } else {
                Ok(())
            }
        }
    }

    /// Check if an fcntl command is allowed
    pub fn check_fcntl(&self, fd: i32, cmd: u32) -> Result<(), CapError> {
        if let Some(entry) = self.fd_table.get(&fd) {
            if (entry.fcntl_allowlist & (1 << cmd)) != 0 {
                Ok(())
            } else {
                Err(CapError::FcntlNotAllowed)
            }
        } else {
            if self.is_cap_mode() {
                Err(CapError::FdNotFound)
            } else {
                Ok(())
            }
        }
    }
}

impl Default for ProcessCapState {
    fn default() -> Self {
        Self::new()
    }
}

/// Capsicum errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapError {
    /// Not in capability mode
    NotInCapMode,
    /// Operation not permitted in capability mode
    CapModeViolation,
    /// Insufficient rights for operation
    InsufficientRights { required: u64, actual: CapRightsMask },
    /// File descriptor not found in rights table
    FdNotFound,
    /// Ioctl command not allowed
    IoctlNotAllowed,
    /// Fcntl command not allowed
    FcntlNotAllowed,
    /// Cannot expand rights (can only narrow)
    RightsExpansionDenied,
}

/// Process descriptor (for pdfork/pdkill/pdwait)
#[derive(Debug, Clone)]
pub struct ProcessDescriptor {
    pub pid: u32,
    pub pd_fd: i32,
    pub rights: CapRightsMask,
}

impl ProcessDescriptor {
    pub fn new(pid: u32, pd_fd: i32, rights: CapRightsMask) -> Self {
        Self { pid, pd_fd, rights }
    }
}

/// Process descriptor table
pub struct ProcDescTable {
    descriptors: Vec<ProcessDescriptor>,
}

impl ProcDescTable {
    pub fn new() -> Self {
        Self {
            descriptors: Vec::new(),
        }
    }

    pub fn add(&mut self, pd: ProcessDescriptor) {
        self.descriptors.push(pd);
    }

    pub fn remove(&mut self, pd_fd: i32) -> Option<ProcessDescriptor> {
        if let Some(pos) = self.descriptors.iter().position(|pd| pd.pd_fd == pd_fd) {
            Some(self.descriptors.remove(pos))
        } else {
            None
        }
    }

    pub fn lookup_by_fd(&self, pd_fd: i32) -> Option<&ProcessDescriptor> {
        self.descriptors.iter().find(|pd| pd.pd_fd == pd_fd)
    }
}

impl Default for ProcDescTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Audit logging hook for capability violations
/// In no_std context, this is a no-op; in std context, could write to syslog
pub fn log_cap_violation(fd: i32, right: u64, pid: u32) {
    // In production: write to audit log
    // For now: no-op in no_std
    let _ = (fd, right, pid);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::security::cap_rights::{CAP_READ, CAP_WRITE, CAP_SEEK, CAP_FSTAT, CAP_PDKILL, CAP_PDWAIT};

    // ─── Legacy tests ─────────────────────────────────────────────────────────────

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

    // ─── New Capsicum tests ───────────────────────────────────────────────────────

    #[test]
    fn test_enter_capability_mode() {
        let mut state = ProcessCapState::new();
        assert!(!state.is_cap_mode());
        
        state.cap_enter().unwrap();
        assert!(state.is_cap_mode());
        
        // Should be idempotent
        state.cap_enter().unwrap();
        assert!(state.is_cap_mode());
    }

    #[test]
    fn test_capability_mode_denies_namespace_access() {
        let mut state = ProcessCapState::new();
        assert!(state.check_namespace_access().is_ok());
        
        state.cap_enter().unwrap();
        assert_eq!(state.check_namespace_access(), Err(CapError::CapModeViolation));
    }

    #[test]
    fn test_fd_rights_limit_cannot_expand() {
        let mut state = ProcessCapState::new();
        let initial_rights = CapRightsMask::new(CAP_READ | CAP_SEEK);
        state.limit_fd_rights(3, initial_rights).unwrap();
        
        // Narrowing should succeed
        let narrower_rights = CapRightsMask::new(CAP_READ);
        assert!(state.limit_fd_rights(3, narrower_rights).is_ok());
        
        // Expanding should fail
        let wider_rights = CapRightsMask::new(CAP_READ | CAP_WRITE | CAP_SEEK);
        assert_eq!(state.limit_fd_rights(3, wider_rights), Err(CapError::RightsExpansionDenied));
    }

    #[test]
    fn test_fd_rights_inheritance_on_dup() {
        let mut state = ProcessCapState::new();
        let rights = CapRightsMask::new(CAP_READ | CAP_WRITE);
        state.limit_fd_rights(3, rights).unwrap();
        
        // Dup to new FD
        state.inherit_rights_on_dup(3, 4).unwrap();
        
        // New FD should have same rights
        assert_eq!(state.get_fd_rights(4), Some(&rights));
    }

    #[test]
    fn test_check_fd_operation_with_sufficient_rights() {
        let mut state = ProcessCapState::new();
        let rights = CapRightsMask::new(CAP_READ | CAP_WRITE);
        state.limit_fd_rights(3, rights).unwrap();
        
        assert!(state.check_fd_right(3, CAP_READ).is_ok());
        assert!(state.check_fd_right(3, CAP_WRITE).is_ok());
    }

    #[test]
    fn test_check_fd_operation_with_insufficient_rights() {
        let mut state = ProcessCapState::new();
        let rights = CapRightsMask::new(CAP_READ);
        state.limit_fd_rights(3, rights).unwrap();
        
        assert!(state.check_fd_right(3, CAP_READ).is_ok());
        
        let result = state.check_fd_right(3, CAP_WRITE);
        match result {
            Err(CapError::InsufficientRights { required, .. }) => {
                assert_eq!(required, CAP_WRITE);
            }
            _ => panic!("Expected InsufficientRights error"),
        }
    }

    #[test]
    fn test_fd_not_in_table_normal_mode() {
        let state = ProcessCapState::new();
        // In normal mode, FD not in table should be allowed
        assert!(state.check_fd_right(999, CAP_READ).is_ok());
    }

    #[test]
    fn test_fd_not_in_table_cap_mode() {
        let mut state = ProcessCapState::new();
        state.cap_enter().unwrap();
        // In capability mode, FD not in table should be denied
        assert_eq!(state.check_fd_right(999, CAP_READ), Err(CapError::FdNotFound));
    }

    #[test]
    fn test_ioctl_restriction() {
        let mut state = ProcessCapState::new();
        let rights = CapRightsMask::new(CAP_READ);
        state.limit_fd_rights(3, rights).unwrap();
        
        // Limit ioctl commands
        state.limit_ioctls(3, vec![0x5401, 0x5402]).unwrap();
        
        assert!(state.check_ioctl(3, 0x5401).is_ok());
        assert!(state.check_ioctl(3, 0x5402).is_ok());
        assert_eq!(state.check_ioctl(3, 0x5403), Err(CapError::IoctlNotAllowed));
    }

    #[test]
    fn test_fcntl_restriction() {
        let mut state = ProcessCapState::new();
        let rights = CapRightsMask::new(CAP_READ);
        state.limit_fd_rights(3, rights).unwrap();
        
        // Limit fcntl commands (bit mask)
        state.limit_fcntls(3, 0b00000011).unwrap(); // Only cmd 0 and 1 allowed
        
        assert!(state.check_fcntl(3, 0).is_ok());
        assert!(state.check_fcntl(3, 1).is_ok());
        assert_eq!(state.check_fcntl(3, 2), Err(CapError::FcntlNotAllowed));
    }

    #[test]
    fn test_process_descriptor_table() {
        let mut table = ProcDescTable::new();
        
        let pd = ProcessDescriptor::new(1234, 5, CapRightsMask::new(CAP_PDKILL | CAP_PDWAIT));
        table.add(pd.clone());
        
        assert!(table.lookup_by_fd(5).is_some());
        assert_eq!(table.lookup_by_fd(5).unwrap().pid, 1234);
        
        table.remove(5);
        assert!(table.lookup_by_fd(5).is_none());
    }

    #[test]
    fn test_get_fd_rights() {
        let mut state = ProcessCapState::new();
        let rights = CapRightsMask::new(CAP_READ | CAP_FSTAT);
        state.limit_fd_rights(7, rights).unwrap();
        
        let retrieved = state.get_fd_rights(7);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap(), &rights);
        
        assert!(state.get_fd_rights(999).is_none());
    }
}
