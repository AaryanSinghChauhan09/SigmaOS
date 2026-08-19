// SigmaOS Polish-Parity System Backup (SigmaTimeshift)
// Designed for automated, transaction-safe snapshots and system recovery

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupError {
    Success = 0,
    SnapshotFailed = 1,
    RestoreFailed = 2,
    NoBackupFound = 3,
}

pub struct BackupSnapshot {
    pub id: String,
    pub timestamp: u64,
    pub label: String,
    pub files_hash: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct FsSnapshot {
    pub id: String,
    pub path: String,
    pub timestamp: u64,
}

pub struct SigmaTimeshift {
    pub snapshots: Vec<BackupSnapshot>,
    pub backup_schedule_enabled: bool,
    pub last_scheduled_run: u64,
}

impl SigmaTimeshift {
    pub fn new() -> Self {
        SigmaTimeshift {
            snapshots: Vec::new(),
            backup_schedule_enabled: true,
            last_scheduled_run: 0,
        }
    }

    pub fn create_snapshot(&mut self, label: String, system_files: HashMap<String, String>) -> Result<String, BackupError> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

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

    pub fn restore_snapshot(&self, id: &str) -> Result<HashMap<String, String>, BackupError> {
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

pub static GLOBAL_TIMESHIFT: SigmaTimeshift = SigmaTimeshift {
    snapshots: Vec::new(),
    backup_schedule_enabled: true,
    last_scheduled_run: 0,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeshift_backup() {
        let mut timeshift = SigmaTimeshift::new();
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
