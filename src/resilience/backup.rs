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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FsSnapshot {
    pub id: usize,
    pub timestamp: u64,
    pub active: bool,
    pub root_hash: u32,
}

pub struct HardwareTimeshift {
    pub snapshots: core::cell::RefCell<[Option<FsSnapshot>; MAX_SNAPSHOT_ENTRIES]>,
    pub backup_active: AtomicBool,
    pub next_id: AtomicUsize,
}

unsafe impl Sync for HardwareTimeshift {}

impl HardwareTimeshift {
    pub const fn new() -> Self {
        const EMPTY_SNAPSHOT: Option<FsSnapshot> = None;
        Self {
            snapshots: core::cell::RefCell::new([EMPTY_SNAPSHOT; MAX_SNAPSHOT_ENTRIES]),
            backup_active: AtomicBool::new(true),
            next_id: AtomicUsize::new(1),
        }
    }

    pub fn create_snapshot(
        &self,
        timestamp: u64,
        current_fhs_hash: u32,
    ) -> Result<usize, &'static str> {
        if !self.backup_active.load(Ordering::SeqCst) {
            return Err("Timeshift: Backup service is currently deactivated.");
        }

        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let snapshot = FsSnapshot {
            id,
            timestamp,
            active: true,
            root_hash: current_fhs_hash,
        };

        let mut list = self.snapshots.borrow_mut();
        let idx = (id - 1) % MAX_SNAPSHOT_ENTRIES;
        list[idx] = Some(snapshot);

        Ok(id)
    }

    pub fn rollback_to_snapshot(&self, snapshot_id: usize) -> Result<u32, &'static str> {
        let list = self.snapshots.borrow();
        for slot in list.iter() {
            if let Some(ref snapshot) = slot {
                if snapshot.id == snapshot_id {
                    return Ok(snapshot.root_hash);
                }
            }
        }
        Err("Timeshift: Selected snapshot ID not found in system registers.")
    }
}

pub static GLOBAL_TIMESHIFT: HardwareTimeshift = HardwareTimeshift::new();

pub struct SigmaTimeshiftManager {
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
