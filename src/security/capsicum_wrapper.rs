//! FreeBSD Capsicum Capability Wrappers
//!
//! Capsicum-inspired capability sandboxing for fine-grained file descriptor
//! rights and process capability restrictions.

use std::collections::HashMap;
use std::ops::{BitOr, BitOrAssign};
use std::sync::atomic::{AtomicU32, Ordering};

/// Capsicum capability rights
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CapRights(u64);

impl BitOr for CapRights {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        CapRights(self.0 | rhs.0)
    }
}

impl BitOrAssign for CapRights {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl CapRights {
    pub const READ: CapRights = CapRights(0x00000001);
    pub const WRITE: CapRights = CapRights(0x00000002);
    pub const EXECUTE: CapRights = CapRights(0x00000004);
    pub const SEEK: CapRights = CapRights(0x00000008);
    pub const MMAP: CapRights = CapRights(0x00000010);
    pub const MMAP_R: CapRights = CapRights(0x00000020);
    pub const MMAP_W: CapRights = CapRights(0x00000040);
    pub const MMAP_X: CapRights = CapRights(0x00000080);
    pub const MMAP_RW: CapRights = CapRights(0x000000a0);
    pub const MMAP_RX: CapRights = CapRights(0x000000c0);
    pub const MMAP_WX: CapRights = CapRights(0x000000e0);
    pub const FCNTL: CapRights = CapRights(0x00000100);
    pub const IOCTL: CapRights = CapRights(0x00000200);
    pub const STAT: CapRights = CapRights(0x00000400);
    pub const CREATE: CapRights = CapRights(0x00000800);
    pub const DELETE: CapRights = CapRights(0x00001000);
    pub const BIND: CapRights = CapRights(0x00002000);
    pub const CONNECT: CapRights = CapRights(0x00004000);
    pub const LISTEN: CapRights = CapRights(0x00008000);
    pub const ACCEPT: CapRights = CapRights(0x00010000);
    pub const GETPEERNAME: CapRights = CapRights(0x00020000);
    pub const GETSOCKNAME: CapRights = CapRights(0x00040000);
    pub const GETOPT: CapRights = CapRights(0x00080000);
    pub const SETOPT: CapRights = CapRights(0x00100000);
    pub const PEEK: CapRights = CapRights(0x00200000);
    pub const SHUTDOWN: CapRights = CapRights(0x00400000);
    pub const SEND: CapRights = CapRights(0x00800000);
    pub const RECV: CapRights = CapRights(0x01000000);
    pub const ALL: CapRights = CapRights(0xffffffffffffffff);

    pub fn new() -> Self {
        CapRights(0)
    }

    pub fn from_bits(bits: u64) -> Self {
        CapRights(bits)
    }

    pub fn bits(&self) -> u64 {
        self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn contains(&self, other: CapRights) -> bool {
        (self.0 & other.0) == other.0
    }

    pub fn insert(&mut self, other: CapRights) {
        self.0 |= other.0;
    }

    pub fn remove(&mut self, other: CapRights) {
        self.0 &= !other.0;
    }

    pub fn union(self, other: CapRights) -> CapRights {
        CapRights(self.0 | other.0)
    }

    pub fn intersection(self, other: CapRights) -> CapRights {
        CapRights(self.0 & other.0)
    }
}

impl Default for CapRights {
    fn default() -> Self {
        Self::new()
    }
}

/// File descriptor capability entry
#[derive(Debug, Clone)]
pub struct CapsicumFdEntry {
    pub fd: i32,
    pub rights: CapRights,
    pub is_valid: bool,
}

impl CapsicumFdEntry {
    pub fn new(fd: i32, rights: CapRights) -> Self {
        Self {
            fd,
            rights,
            is_valid: true,
        }
    }

    pub fn check_rights(&self, required: CapRights) -> bool {
        if !self.is_valid {
            return false;
        }
        self.rights.contains(required)
    }

    pub fn revoke(&mut self) {
        self.is_valid = false;
        self.rights = CapRights::new();
    }

    pub fn limit(&mut self, rights: CapRights) {
        self.rights = self.rights.intersection(rights);
    }
}

/// Capability mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CapSandboxMode {
    /// No capability restrictions
    Unrestricted,
    /// Capability restrictions enabled
    Restricted,
    /// Capability restrictions with sandbox
    Sandbox,
}

/// Capsicum capability manager
#[derive(Debug)]
pub struct CapsicumManager {
    entries: HashMap<i32, CapsicumFdEntry>,
    mode: CapSandboxMode,
    next_fd: AtomicU32,
    limit_count: u32,
    revoke_count: u32,
}

impl CapsicumManager {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            mode: CapSandboxMode::Unrestricted,
            next_fd: AtomicU32::new(3),
            limit_count: 0,
            revoke_count: 0,
        }
    }

    /// Set capability mode
    pub fn set_mode(&mut self, mode: CapSandboxMode) {
        self.mode = mode;
    }

    /// Get current mode
    pub fn get_mode(&self) -> CapSandboxMode {
        self.mode
    }

    /// Create a new capability entry
    pub fn create_entry(&mut self, rights: CapRights) -> i32 {
        let fd = self.next_fd.fetch_add(1, Ordering::SeqCst) as i32;
        let entry = CapsicumFdEntry::new(fd, rights);
        self.entries.insert(fd, entry);
        fd
    }

    /// Add capability entry for existing file descriptor
    pub fn add_entry(&mut self, fd: i32, rights: CapRights) {
        let entry = CapsicumFdEntry::new(fd, rights);
        self.entries.insert(fd, entry);
    }

    /// Get capability entry
    pub fn get_entry(&self, fd: i32) -> Option<&CapsicumFdEntry> {
        self.entries.get(&fd)
    }

    /// Get capability entry mutably
    pub fn get_entry_mut(&mut self, fd: i32) -> Option<&mut CapsicumFdEntry> {
        self.entries.get_mut(&fd)
    }

    /// Check if file descriptor has required rights
    pub fn check_rights(&self, fd: i32, required: CapRights) -> bool {
        if self.mode == CapSandboxMode::Unrestricted {
            return true;
        }

        match self.get_entry(fd) {
            Some(entry) => entry.check_rights(required),
            None => false,
        }
    }

    /// Limit rights on a file descriptor
    pub fn limit_rights(&mut self, fd: i32, rights: CapRights) -> Result<(), String> {
        match self.get_entry_mut(fd) {
            Some(entry) => {
                entry.limit(rights);
                self.limit_count += 1;
                Ok(())
            }
            None => Err(format!("File descriptor {} not found", fd)),
        }
    }

    /// Revoke all rights on a file descriptor
    pub fn revoke_fd(&mut self, fd: i32) -> Result<(), String> {
        match self.get_entry_mut(fd) {
            Some(entry) => {
                entry.revoke();
                self.revoke_count += 1;
                Ok(())
            }
            None => Err(format!("File descriptor {} not found", fd)),
        }
    }

    /// Revoke all file descriptors
    pub fn revoke_all(&mut self) {
        for entry in self.entries.values_mut() {
            entry.revoke();
        }
        self.revoke_count += self.entries.len() as u32;
    }

    /// Remove file descriptor entry
    pub fn remove_entry(&mut self, fd: i32) -> Option<CapsicumFdEntry> {
        self.entries.remove(&fd)
    }

    /// List all capability entries
    pub fn list_entries(&self) -> Vec<&CapsicumFdEntry> {
        self.entries.values().collect()
    }

    /// Get statistics
    pub fn get_statistics(&self) -> CapsicumStatistics {
        let valid_entries = self.entries.values().filter(|e| e.is_valid).count();
        let revoked_entries = self.entries.values().filter(|e| !e.is_valid).count();

        CapsicumStatistics {
            total_entries: self.entries.len(),
            valid_entries,
            revoked_entries,
            mode: self.mode,
            limit_count: self.limit_count,
            revoke_count: self.revoke_count,
        }
    }

    /// Reset statistics
    pub fn reset_statistics(&mut self) {
        self.limit_count = 0;
        self.revoke_count = 0;
    }
}

impl Default for CapsicumManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Capsicum statistics
#[derive(Debug, Clone)]
pub struct CapsicumStatistics {
    pub total_entries: usize,
    pub valid_entries: usize,
    pub revoked_entries: usize,
    pub mode: CapSandboxMode,
    pub limit_count: u32,
    pub revoke_count: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cap_rights() {
        let rights = CapRights::READ | CapRights::WRITE;
        assert!(rights.contains(CapRights::READ));
        assert!(rights.contains(CapRights::WRITE));
        assert!(!rights.contains(CapRights::EXECUTE));
    }

    #[test]
    fn test_cap_rights_union() {
        let rights1 = CapRights::READ;
        let rights2 = CapRights::WRITE;
        let combined = rights1.union(rights2);
        assert!(combined.contains(CapRights::READ));
        assert!(combined.contains(CapRights::WRITE));
    }

    #[test]
    fn test_cap_rights_intersection() {
        let rights1 = CapRights::READ | CapRights::WRITE;
        let rights2 = CapRights::WRITE | CapRights::EXECUTE;
        let intersection = rights1.intersection(rights2);
        assert!(!intersection.contains(CapRights::READ));
        assert!(intersection.contains(CapRights::WRITE));
        assert!(!intersection.contains(CapRights::EXECUTE));
    }

    #[test]
    fn test_cap_entry() {
        let rights = CapRights::READ | CapRights::WRITE;
        let entry = CapsicumFdEntry::new(3, rights);
        assert!(entry.check_rights(CapRights::READ));
        assert!(!entry.check_rights(CapRights::EXECUTE));
    }

    #[test]
    fn test_cap_entry_revoke() {
        let rights = CapRights::READ;
        let mut entry = CapsicumFdEntry::new(3, rights);
        entry.revoke();
        assert!(!entry.is_valid);
        assert!(!entry.check_rights(CapRights::READ));
    }

    #[test]
    fn test_cap_entry_limit() {
        let rights = CapRights::READ | CapRights::WRITE;
        let mut entry = CapsicumFdEntry::new(3, rights);
        entry.limit(CapRights::READ);
        assert!(entry.check_rights(CapRights::READ));
        assert!(!entry.check_rights(CapRights::WRITE));
    }

    #[test]
    fn test_capsicum_manager_creation() {
        let manager = CapsicumManager::new();
        assert_eq!(manager.get_mode(), CapSandboxMode::Unrestricted);
        assert_eq!(manager.entries.len(), 0);
    }

    #[test]
    fn test_capsicum_set_mode() {
        let mut manager = CapsicumManager::new();
        manager.set_mode(CapSandboxMode::Restricted);
        assert_eq!(manager.get_mode(), CapSandboxMode::Restricted);
    }

    #[test]
    fn test_capsicum_create_entry() {
        let mut manager = CapsicumManager::new();
        let rights = CapRights::READ;
        let fd = manager.create_entry(rights);
        assert!(fd >= 3);
        assert!(manager.get_entry(fd).is_some());
    }

    #[test]
    fn test_capsicum_check_rights() {
        let mut manager = CapsicumManager::new();
        let rights = CapRights::READ | CapRights::WRITE;
        let fd = manager.create_entry(rights);

        assert!(manager.check_rights(fd, CapRights::READ));
        assert!(!manager.check_rights(fd, CapRights::EXECUTE));
    }

    #[test]
    fn test_capsicum_check_rights_unrestricted() {
        let mut manager = CapsicumManager::new();
        let rights = CapRights::READ;
        let fd = manager.create_entry(rights);

        // In unrestricted mode, all rights are granted
        assert!(manager.check_rights(fd, CapRights::EXECUTE));
    }

    #[test]
    fn test_capsicum_limit_rights() {
        let mut manager = CapsicumManager::new();
        let rights = CapRights::READ | CapRights::WRITE;
        let fd = manager.create_entry(rights);

        manager.set_mode(CapSandboxMode::Restricted);
        assert!(manager.limit_rights(fd, CapRights::READ).is_ok());

        assert!(manager.check_rights(fd, CapRights::READ));
        assert!(!manager.check_rights(fd, CapRights::WRITE));
    }

    #[test]
    fn test_capsicum_revoke_fd() {
        let mut manager = CapsicumManager::new();
        let rights = CapRights::READ;
        let fd = manager.create_entry(rights);

        assert!(manager.revoke_fd(fd).is_ok());
        assert!(!manager.check_rights(fd, CapRights::READ));
    }

    #[test]
    fn test_capsicum_revoke_all() {
        let mut manager = CapsicumManager::new();
        let rights = CapRights::READ;
        let fd1 = manager.create_entry(rights);
        let fd2 = manager.create_entry(rights);

        manager.revoke_all();
        assert!(!manager.check_rights(fd1, CapRights::READ));
        assert!(!manager.check_rights(fd2, CapRights::READ));
    }

    #[test]
    fn test_capsicum_statistics() {
        let mut manager = CapsicumManager::new();
        let rights = CapRights::READ;
        let fd1 = manager.create_entry(rights);
        let fd2 = manager.create_entry(rights);

        manager.set_mode(CapSandboxMode::Restricted);
        manager.limit_rights(fd1, CapRights::READ).ok();
        manager.revoke_fd(fd2).ok();

        let stats = manager.get_statistics();
        assert_eq!(stats.total_entries, 2);
        assert_eq!(stats.valid_entries, 1);
        assert_eq!(stats.revoked_entries, 1);
        assert_eq!(stats.limit_count, 1);
        assert_eq!(stats.revoke_count, 1);
    }

    #[test]
    fn test_capsicum_remove_entry() {
        let mut manager = CapsicumManager::new();
        let rights = CapRights::READ;
        let fd = manager.create_entry(rights);

        let removed = manager.remove_entry(fd);
        assert!(removed.is_some());
        assert!(manager.get_entry(fd).is_none());
    }
}
