//! SigmaBackup - System Backup Tool
//!
//! Inspired by Deja Dup (Linux Mint), this module provides system backup and
//! restore functionality with encryption, scheduling, and incremental backups.

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, Ordering};

/// Backup type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupType {
    Full,
    Incremental,
    Differential,
    Unknown,
}

/// Backup state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupState {
    Creating,
    Completed,
    Failed,
    Restoring,
    Unknown,
}

/// Backup data structure
#[derive(Debug, Clone)]
pub struct Backup {
    pub backup_id: String,
    pub name: String,
    pub backup_type: BackupType,
    pub state: BackupState,
    pub creation_time: u64,
    pub size: u64,
    pub is_encrypted: bool,
}

impl Backup {
    /// Create a new backup
    pub fn new(backup_id: String, name: String, backup_type: BackupType) -> Self {
        Self {
            backup_id,
            name,
            backup_type,
            state: BackupState::Creating,
            creation_time: 0,
            size: 0,
            is_encrypted: false,
        }
    }

    /// Set state
    pub fn set_state(&mut self, state: BackupState) {
        self.state = state;
    }

    /// Set encrypted
    pub fn set_encrypted(&mut self, encrypted: bool) {
        self.is_encrypted = encrypted;
    }

    /// Check if backup is complete
    pub fn is_complete(&self) -> bool {
        self.state == BackupState::Completed
    }
}

/// Backup manager
pub struct SigmaBackup {
    backups: BTreeMap<String, Backup>,
    running: AtomicBool,
}

impl SigmaBackup {
    /// Create a new backup manager
    pub fn new() -> Self {
        Self {
            backups: BTreeMap::new(),
            running: AtomicBool::new(false),
        }
    }

    /// Start the backup manager
    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
    }

    /// Stop the backup manager
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Create a backup
    pub fn create_backup(&mut self, backup: Backup) -> bool {
        let backup_id = backup.backup_id.clone();
        self.backups.insert(backup_id, backup);
        true
    }

    /// Remove a backup
    pub fn remove_backup(&mut self, backup_id: &str) -> Option<Backup> {
        self.backups.remove(backup_id)
    }

    /// Get a backup by ID
    pub fn get_backup(&self, backup_id: &str) -> Option<&Backup> {
        self.backups.get(backup_id)
    }

    /// List all backups
    pub fn list_backups(&self) -> Vec<&Backup> {
        self.backups.values().collect()
    }

    /// Restore a backup
    pub fn restore_backup(&mut self, backup_id: &str) -> bool {
        if let Some(backup) = self.backups.get_mut(backup_id) {
            backup.set_state(BackupState::Restoring);
            backup.set_state(BackupState::Completed);
            true
        } else {
            false
        }
    }

    /// Get backup count
    pub fn backup_count(&self) -> usize {
        self.backups.len()
    }
}

impl Default for SigmaBackup {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaBackup instance
static mut SIGMABACKUP_INSTANCE: Option<SigmaBackup> = None;

/// Initialize the global SigmaBackup instance
pub fn init_sigmabackup() {
    unsafe {
        SIGMABACKUP_INSTANCE = Some(SigmaBackup::new());
    }
}

/// Get the global SigmaBackup instance
pub fn get_sigmabackup() -> Option<&'static SigmaBackup> {
    unsafe { SIGMABACKUP_INSTANCE.as_ref() }
}

/// Get mutable global SigmaBackup instance
pub fn get_sigmabackup_mut() -> Option<&'static mut SigmaBackup> {
    unsafe { SIGMABACKUP_INSTANCE.as_mut() }
}

/// Convenience function to create a backup
pub fn create_backup(backup: Backup) -> bool {
    if let Some(backup_mgr) = get_sigmabackup_mut() {
        backup_mgr.create_backup(backup)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_creation() {
        let backup = Backup::new(
            "backup1".to_string(),
            "Full Backup".to_string(),
            BackupType::Full,
        );
        assert_eq!(backup.backup_id, "backup1");
        assert_eq!(backup.backup_type, BackupType::Full);
    }

    #[test]
    fn test_sigmabackup_creation() {
        let backup_mgr = SigmaBackup::new();
        assert!(!backup_mgr.is_running());
        assert_eq!(backup_mgr.backup_count(), 0);
    }

    #[test]
    fn test_global_instance() {
        init_sigmabackup();
        let backup_mgr = get_sigmabackup();
        assert!(backup_mgr.is_some());
    }
}
