// SPDX-License-Identifier: MIT
// SigmaOS Timeshift-Parity Recovery & Snapshot Shard
// Permitting instant system-wide rollbacks of the root file system hierarchy if user updates damage any system file.

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub const MAX_SNAPSHOT_ENTRIES: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupError {
    Success = 0,
    NotFound = 1,
    CreationFailed = 2,
    RestoreFailed = 3,
    NoBackupFound = 4,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupSnapshot {
    pub id: String,
    pub timestamp: u64,
    pub label: String,
    pub files_hash: BTreeMap<String, String>,
}

pub type FsSnapshot = BackupSnapshot;

pub static GLOBAL_TIMESHIFT: std::sync::Mutex<Option<SigmaTimeshift>> = std::sync::Mutex::new(None);

pub struct SigmaTimeshift {
    pub snapshots: Vec<BackupSnapshot>,
    pub backup_schedule_enabled: bool,
    pub last_scheduled_run: u64,
}

impl SigmaTimeshiftManager {
    pub fn new() -> Self {
        SigmaTimeshiftManager {
            snapshots: Vec::new(),
            backup_schedule_enabled: true,
            last_scheduled_run: 0,
        }
    }

    pub fn create_snapshot(&mut self, label: String, system_files: BTreeMap<String, String>) -> Result<String, BackupError> {
        let timestamp = 0u64;

        let id = format!("timeshift-snap-{}", timestamp);
        let snapshot = BackupSnapshot {
            id: id.clone(),
            timestamp,
            label,
            files_hash: system_files,
        };

        self.snapshots.push(snapshot);
        Ok(id)
    }

    pub fn restore_snapshot(&self, id: &str) -> Result<BTreeMap<String, String>, BackupError> {
        if let Some(snap) = self.snapshots.iter().find(|s| s.id == id) {
            Ok(snap.files_hash.clone())
        } else {
            Err(BackupError::NoBackupFound)
        }
    }

    pub fn delete_snapshot(&mut self, id: &str) -> Result<(), BackupError> {
        if let Some(pos) = self.snapshots.iter().position(|s| s.id == id) {
            self.snapshots.remove(pos);
            Ok(())
        } else {
            Err(BackupError::NoBackupFound)
        }
    }
}

impl Default for SigmaTimeshift {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeshift_backup() {
        let mut timeshift = SigmaTimeshiftManager::new();
        let mut files = HashMap::new();
        files.insert("/etc/hosts".to_string(), "hash123".to_string());
        files.insert("/bin/sigma-sh".to_string(), "hash456".to_string());

        let id = timeshift.create_snapshot("Initial Clean Install".to_string(), files).unwrap();
        assert_eq!(timeshift.snapshots.len(), 1);

        let restored = timeshift.restore_snapshot(&id).unwrap();
        assert_eq!(restored.get("/etc/hosts").unwrap(), "hash123");

        assert!(timeshift.delete_snapshot(&id).is_ok());
        assert_eq!(timeshift.snapshots.len(), 0);
    }
}
