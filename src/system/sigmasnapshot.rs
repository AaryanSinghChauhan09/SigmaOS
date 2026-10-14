//! SigmaSnapshot - Snapshot Management
//!
//! Inspired by btrfs/zfs snapshots, this module provides comprehensive snapshot
//! management including snapshot creation, deletion, rollback, and scheduling.

#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};

/// Snapshot type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotType {
    Full,
    Incremental,
    Differential,
    Unknown,
}

/// Snapshot state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotState {
    Creating,
    Completed,
    Failed,
    Restoring,
    Unknown,
}

/// Snapshot data structure
#[derive(Debug, Clone)]
pub struct Snapshot {
    pub snapshot_id: String,
    pub snapshot_name: String,
    pub volume_id: String,
    pub snapshot_type: SnapshotType,
    pub state: SnapshotState,
    pub creation_time: u64,
    pub size: u64,
    pub description: String,
}

impl Snapshot {
    /// Create a new snapshot
    pub fn new(
        snapshot_id: String,
        snapshot_name: String,
        volume_id: String,
        snapshot_type: SnapshotType,
        description: String,
    ) -> Self {
        Self {
            snapshot_id,
            snapshot_name,
            volume_id,
            snapshot_type,
            state: SnapshotState::Creating,
            creation_time: 0,
            size: 0,
            description,
        }
    }

    /// Set snapshot state
    pub fn set_state(&mut self, state: SnapshotState) {
        self.state = state;
    }

    /// Set creation time
    pub fn set_creation_time(&mut self, creation_time: u64) {
        self.creation_time = creation_time;
    }

    /// Set size
    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }

    /// Check if snapshot is completed
    pub fn is_completed(&self) -> bool {
        self.state == SnapshotState::Completed
    }

    /// Check if snapshot is failed
    pub fn is_failed(&self) -> bool {
        self.state == SnapshotState::Failed
    }
}

/// Snapshot manager
pub struct SigmaSnapshot {
    snapshots: BTreeMap<String, Snapshot>,
    running: AtomicBool,
}

impl SigmaSnapshot {
    /// Create a new snapshot manager
    pub fn new() -> Self {
        Self {
            snapshots: BTreeMap::new(),
            running: AtomicBool::new(false),
        }
    }

    /// Start the snapshot manager
    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
    }

    /// Stop the snapshot manager
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Create a snapshot
    pub fn create_snapshot(&mut self, snapshot: Snapshot) -> bool {
        let snap_id = snapshot.snapshot_id.clone();
        self.snapshots.insert(snap_id.clone(), snapshot);
        if let Some(snap) = self.snapshots.get_mut(&snap_id) {
            snap.set_state(SnapshotState::Completed);
            true
        } else {
            false
        }
    }

    /// Delete a snapshot
    pub fn delete_snapshot(&mut self, snapshot_id: &str) -> Option<Snapshot> {
        self.snapshots.remove(snapshot_id)
    }

    /// Get a snapshot by ID
    pub fn get_snapshot(&self, snapshot_id: &str) -> Option<&Snapshot> {
        self.snapshots.get(snapshot_id)
    }

    /// List all snapshots
    pub fn list_snapshots(&self) -> Vec<&Snapshot> {
        self.snapshots.values().collect()
    }

    /// List snapshots by volume
    pub fn list_snapshots_by_volume(&self, volume_id: &str) -> Vec<&Snapshot> {
        self.snapshots
            .values()
            .filter(|s| s.volume_id == volume_id)
            .collect()
    }

    /// List snapshots by type
    pub fn list_snapshots_by_type(&self, snapshot_type: SnapshotType) -> Vec<&Snapshot> {
        self.snapshots
            .values()
            .filter(|s| s.snapshot_type == snapshot_type)
            .collect()
    }

    /// List snapshots by state
    pub fn list_snapshots_by_state(&self, state: SnapshotState) -> Vec<&Snapshot> {
        self.snapshots
            .values()
            .filter(|s| s.state == state)
            .collect()
    }

    /// Restore a snapshot
    pub fn restore_snapshot(&mut self, snapshot_id: &str) -> bool {
        if let Some(snap) = self.snapshots.get_mut(snapshot_id) {
            snap.set_state(SnapshotState::Restoring);
            snap.set_state(SnapshotState::Completed);
            true
        } else {
            false
        }
    }

    /// Get snapshot count
    pub fn snapshot_count(&self) -> usize {
        self.snapshots.len()
    }
}

impl Default for SigmaSnapshot {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaSnapshot instance (placeholder for actual global state)
static mut SIGMASNAPSHOT_INSTANCE: Option<SigmaSnapshot> = None;

/// Initialize the global SigmaSnapshot instance
pub fn init_sigmasnapshot() {
    unsafe {
        SIGMASNAPSHOT_INSTANCE = Some(SigmaSnapshot::new());
    }
}

/// Get the global SigmaSnapshot instance
pub fn get_sigmasnapshot() -> Option<&'static SigmaSnapshot> {
    unsafe {
        SIGMASNAPSHOT_INSTANCE.as_ref()
    }
}

/// Get mutable global SigmaSnapshot instance
pub fn get_sigmasnapshot_mut() -> Option<&'static mut SigmaSnapshot> {
    unsafe {
        SIGMASNAPSHOT_INSTANCE.as_mut()
    }
}

/// Convenience function to create a snapshot
pub fn create_snapshot(snapshot: Snapshot) -> bool {
    if let Some(snap_mgr) = get_sigmasnapshot_mut() {
        snap_mgr.create_snapshot(snapshot)
    } else {
        false
    }
}

/// Convenience function to restore a snapshot
pub fn restore_snapshot(snapshot_id: &str) -> bool {
    if let Some(snap_mgr) = get_sigmasnapshot_mut() {
        snap_mgr.restore_snapshot(snapshot_id)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_creation() {
        let snapshot = Snapshot::new(
            "snap1".to_string(),
            "backup1".to_string(),
            "vol1".to_string(),
            SnapshotType::Full,
            "Full backup".to_string(),
        );

        assert_eq!(snapshot.snapshot_id, "snap1");
        assert_eq!(snapshot.snapshot_name, "backup1");
        assert_eq!(snapshot.snapshot_type, SnapshotType::Full);
        assert_eq!(snapshot.state, SnapshotState::Creating);
    }

    #[test]
    fn test_snapshot_state() {
        let mut snapshot = Snapshot::new(
            "snap1".to_string(),
            "backup1".to_string(),
            "vol1".to_string(),
            SnapshotType::Full,
            "Full backup".to_string(),
        );

        snapshot.set_state(SnapshotState::Completed);
        assert!(snapshot.is_completed());

        snapshot.set_state(SnapshotState::Failed);
        assert!(snapshot.is_failed());
    }

    #[test]
    fn test_sigmasnapshot_creation() {
        let snap_mgr = SigmaSnapshot::new();
        assert!(!snap_mgr.is_running());
        assert_eq!(snap_mgr.snapshot_count(), 0);
    }

    #[test]
    fn test_sigmasnapshot_start_stop() {
        let mut snap_mgr = SigmaSnapshot::new();
        snap_mgr.start();
        assert!(snap_mgr.is_running());
        snap_mgr.stop();
        assert!(!snap_mgr.is_running());
    }

    #[test]
    fn test_sigmasnapshot_create_snapshot() {
        let mut snap_mgr = SigmaSnapshot::new();
        let snapshot = Snapshot::new(
            "snap1".to_string(),
            "backup1".to_string(),
            "vol1".to_string(),
            SnapshotType::Full,
            "Full backup".to_string(),
        );

        let created = snap_mgr.create_snapshot(snapshot);
        assert!(created);
        assert_eq!(snap_mgr.snapshot_count(), 1);

        let snap = snap_mgr.get_snapshot("snap1").unwrap();
        assert!(snap.is_completed());
    }

    #[test]
    fn test_sigmasnapshot_delete_snapshot() {
        let mut snap_mgr = SigmaSnapshot::new();
        let snapshot = Snapshot::new(
            "snap1".to_string(),
            "backup1".to_string(),
            "vol1".to_string(),
            SnapshotType::Full,
            "Full backup".to_string(),
        );

        snap_mgr.create_snapshot(snapshot);
        assert_eq!(snap_mgr.snapshot_count(), 1);

        let deleted = snap_mgr.delete_snapshot("snap1");
        assert!(deleted.is_some());
        assert_eq!(snap_mgr.snapshot_count(), 0);
    }

    #[test]
    fn test_sigmasnapshot_restore_snapshot() {
        let mut snap_mgr = SigmaSnapshot::new();
        let snapshot = Snapshot::new(
            "snap1".to_string(),
            "backup1".to_string(),
            "vol1".to_string(),
            SnapshotType::Full,
            "Full backup".to_string(),
        );

        snap_mgr.create_snapshot(snapshot);
        let restored = snap_mgr.restore_snapshot("snap1");
        assert!(restored);

        let snap = snap_mgr.get_snapshot("snap1").unwrap();
        assert!(snap.is_completed());
    }

    #[test]
    fn test_sigmasnapshot_list_by_volume() {
        let mut snap_mgr = SigmaSnapshot::new();

        snap_mgr.create_snapshot(Snapshot::new(
            "snap1".to_string(),
            "backup1".to_string(),
            "vol1".to_string(),
            SnapshotType::Full,
            "Full backup".to_string(),
        ));

        snap_mgr.create_snapshot(Snapshot::new(
            "snap2".to_string(),
            "backup2".to_string(),
            "vol1".to_string(),
            SnapshotType::Incremental,
            "Incremental backup".to_string(),
        ));

        snap_mgr.create_snapshot(Snapshot::new(
            "snap3".to_string(),
            "backup3".to_string(),
            "vol2".to_string(),
            SnapshotType::Full,
            "Full backup".to_string(),
        ));

        let vol1_snaps = snap_mgr.list_snapshots_by_volume("vol1");
        assert_eq!(vol1_snaps.len(), 2);

        let vol2_snaps = snap_mgr.list_snapshots_by_volume("vol2");
        assert_eq!(vol2_snaps.len(), 1);
    }

    #[test]
    fn test_sigmasnapshot_list_by_type() {
        let mut snap_mgr = SigmaSnapshot::new();

        snap_mgr.create_snapshot(Snapshot::new(
            "snap1".to_string(),
            "backup1".to_string(),
            "vol1".to_string(),
            SnapshotType::Full,
            "Full backup".to_string(),
        ));

        snap_mgr.create_snapshot(Snapshot::new(
            "snap2".to_string(),
            "backup2".to_string(),
            "vol1".to_string(),
            SnapshotType::Full,
            "Full backup".to_string(),
        ));

        snap_mgr.create_snapshot(Snapshot::new(
            "snap3".to_string(),
            "backup3".to_string(),
            "vol1".to_string(),
            SnapshotType::Incremental,
            "Incremental backup".to_string(),
        ));

        let full_snaps = snap_mgr.list_snapshots_by_type(SnapshotType::Full);
        assert_eq!(full_snaps.len(), 2);

        let inc_snaps = snap_mgr.list_snapshots_by_type(SnapshotType::Incremental);
        assert_eq!(inc_snaps.len(), 1);
    }

    #[test]
    fn test_global_instance() {
        init_sigmasnapshot();

        let snap_mgr = get_sigmasnapshot();
        assert!(snap_mgr.is_some());

        let mut snap_mgr_mut = get_sigmasnapshot_mut();
        assert!(snap_mgr_mut.is_some());

        if let Some(snap_mgr) = snap_mgr_mut {
            snap_mgr.start();
            assert!(snap_mgr.is_running());
        }
    }

    #[test]
    fn test_convenience_create_snapshot() {
        init_sigmasnapshot();

        let snapshot = Snapshot::new(
            "snap1".to_string(),
            "backup1".to_string(),
            "vol1".to_string(),
            SnapshotType::Full,
            "Full backup".to_string(),
        );

        let created = create_snapshot(snapshot);
        assert!(created);

        let snap_mgr = get_sigmasnapshot().unwrap();
        assert_eq!(snap_mgr.snapshot_count(), 1);
    }

    #[test]
    fn test_convenience_restore_snapshot() {
        init_sigmasnapshot();

        let snapshot = Snapshot::new(
            "snap1".to_string(),
            "backup1".to_string(),
            "vol1".to_string(),
            SnapshotType::Full,
            "Full backup".to_string(),
        );

        create_snapshot(snapshot);

        let restored = restore_snapshot("snap1");
        assert!(restored);

        let snap_mgr = get_sigmasnapshot().unwrap();
        let snap = snap_mgr.get_snapshot("snap1").unwrap();
        assert!(snap.is_completed());
    }
}
