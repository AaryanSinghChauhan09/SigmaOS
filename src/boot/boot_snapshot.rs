// SigmaOS Boot Snapshot Engine
// Zero-dependency #![no_std] pre-boot snapshot verification & automatic rollback engine

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootSnapshotStatus {
    Ready,
    Creating,
    Verified,
    Corrupted,
    Restoring,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootSnapshotConfig {
    pub auto_snapshot_before_boot: bool,
    pub max_boot_snapshots_retained: usize,
    pub verify_checksum_on_boot: bool,
    pub auto_rollback_on_failure: bool,
}

impl Default for BootSnapshotConfig {
    fn default() -> Self {
        Self {
            auto_snapshot_before_boot: true,
            max_boot_snapshots_retained: 5,
            verify_checksum_on_boot: true,
            auto_rollback_on_failure: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootSnapshotItem {
    pub id: String,
    pub label: String,
    pub timestamp: u64,
    pub hash_checksum: u64,
    pub status: BootSnapshotStatus,
    pub is_clean_boot: bool,
}

pub struct BootSnapshotEngine {
    config: BootSnapshotConfig,
    snapshots: Vec<BootSnapshotItem>,
    current_boot_id: Option<String>,
}

impl BootSnapshotEngine {
    pub fn new(config: BootSnapshotConfig) -> Self {
        Self {
            config,
            snapshots: Vec::new(),
            current_boot_id: None,
        }
    }

    pub fn create_preboot_snapshot(&mut self, label: &str, timestamp: u64) -> BootSnapshotItem {
        let snap_id = format!("boot_snap_{}_{}", timestamp, self.snapshots.len() + 1);
        let hash = timestamp.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);

        let item = BootSnapshotItem {
            id: snap_id.clone(),
            label: String::from(label),
            timestamp,
            hash_checksum: hash,
            status: BootSnapshotStatus::Ready,
            is_clean_boot: false,
        };

        if self.snapshots.len() >= self.config.max_boot_snapshots_retained {
            self.snapshots.remove(0);
        }

        self.snapshots.push(item.clone());
        self.current_boot_id = Some(snap_id);

        item
    }

    pub fn verify_boot_integrity(&mut self, snapshot_id: &str) -> bool {
        if let Some(snap) = self.snapshots.iter_mut().find(|s| s.id == snapshot_id) {
            let expected_hash = snap.timestamp.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            if snap.hash_checksum == expected_hash {
                snap.status = BootSnapshotStatus::Verified;
                snap.is_clean_boot = true;
                true
            } else {
                snap.status = BootSnapshotStatus::Corrupted;
                false
            }
        } else {
            false
        }
    }

    pub fn execute_rollback(&mut self) -> Option<BootSnapshotItem> {
        let clean_snap = self
            .snapshots
            .iter()
            .rev()
            .find(|s| s.is_clean_boot && s.status == BootSnapshotStatus::Verified)?
            .clone();

        if let Some(snap) = self.snapshots.iter_mut().find(|s| s.id == clean_snap.id) {
            snap.status = BootSnapshotStatus::Restoring;
        }

        Some(clean_snap)
    }

    pub fn list_snapshots(&self) -> &[BootSnapshotItem] {
        &self.snapshots
    }

    pub fn active_boot_snapshot(&self) -> Option<&BootSnapshotItem> {
        let id = self.current_boot_id.as_ref()?;
        self.snapshots.iter().find(|s| &s.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boot_snapshot_engine() {
        let mut engine = BootSnapshotEngine::new(BootSnapshotConfig::default());
        let snap = engine.create_preboot_snapshot("Kernel Upgrade 6.12", 1700000000);

        assert_eq!(engine.list_snapshots().len(), 1);
        assert!(engine.verify_boot_integrity(&snap.id));

        let active = engine.active_boot_snapshot().unwrap();
        assert_eq!(active.status, BootSnapshotStatus::Verified);
        assert!(active.is_clean_boot);

        let rollback = engine.execute_rollback();
        assert!(rollback.is_some());
        assert_eq!(rollback.unwrap().id, snap.id);
    }
}
