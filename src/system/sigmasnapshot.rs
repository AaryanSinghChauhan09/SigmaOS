//! SigmaSnapshot - Snapshot Management
//!
//! Inspired by Btrfs/ZFS snapshots and Snapper, this module provides comprehensive
//! filesystem and volume snapshot management for SigmaOS.
//!
//! Features include:
//! - Read-only, read-write, and incremental snapshots
//! - Snapshot state lifecycle tracking
//! - Retention and policy management for automated snapshot schedules
//! - Snapshot restoration and rollback
//! - Global instance and convenience API

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};

/// Snapshot type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotType {
    ReadOnly,
    ReadWrite,
    Incremental,
    Differential,
    Full,
    Unknown,
;

/// Snapshot state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotState {
    Creating,
    Active,
    Deleted,
    Restoring,
    Failed,
    Unknown,
;

/// Snapshot schedule / trigger frequency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotSchedule {
    Manual,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Boot,
;

/// Snapshot data structure
#[derive(Debug, Clone)]
pub struct Snapshot {
    pub snapshot_id: String,
    pub snapshot_name: String,
    pub source_path: String,
    pub target_path: String,
    pub snapshot_type: SnapshotType,
    pub state: SnapshotState,
    pub schedule: SnapshotSchedule,
    pub parent_snapshot_id: Option<String>,
    pub size_bytes: u64,
    pub created_timestamp: u64,
;

impl Snapshot {
    /// Create a new snapshot instance
    pub fn new(
        snapshot_id: String,
        snapshot_name: String,
        source_path: String,
        target_path: String,
        snapshot_type: SnapshotType,
        schedule: SnapshotSchedule,
        size_bytes: u64,
    ) -> Self {
        Self {
            snapshot_id,
            snapshot_name,
            source_path,
            target_path,
            snapshot_type,
            state: SnapshotState::Active,
            schedule,
            parent_snapshot_id: None,
            size_bytes,
            created_timestamp: 0,
        }
    }

    /// Set parent snapshot ID for incremental snapshots
    pub fn set_parent(&mut self, parent_id: Option<String>) {
        self.parent_snapshot_id = parent_id;
    }

    /// Set snapshot state
    pub fn set_state(&mut self, state: SnapshotState) {
        self.state = state;
    }

    /// Check if snapshot is active and ready
    pub fn is_active(&self) -> bool {
        self.state == SnapshotState::Active
    }

    /// Check if snapshot is read-only
    pub fn is_read_only(&self) -> bool {
        self.snapshot_type == SnapshotType::ReadOnly
    }
;

/// Automated snapshot policy
#[derive(Debug, Clone)]
pub struct SnapshotPolicy {
    pub policy_id: String,
    pub source_path: String,
    pub max_snapshots: usize,
    pub auto_cleanup: bool,
    pub schedule: SnapshotSchedule,
    pub enabled: bool,
;

impl SnapshotPolicy {
    /// Create a new snapshot policy
    pub fn new(
        policy_id: String,
        source_path: String,
        max_snapshots: usize,
        auto_cleanup: bool,
        schedule: SnapshotSchedule,
    ) -> Self {
        Self {
            policy_id,
            source_path,
            max_snapshots,
            auto_cleanup,
            schedule,
            enabled: true,
        }
    }

    /// Enable or disable policy
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
;

/// Main SigmaSnapshot manager struct
pub struct SigmaSnapshot {
    snapshots: BTreeMap<String, Snapshot>,
    policies: BTreeMap<String, SnapshotPolicy>,
    running: AtomicBool,
;

impl SigmaSnapshot {
    /// Create a new SigmaSnapshot manager
    pub fn new() -> Self {
        Self {
            snapshots: BTreeMap::new(),
            policies: BTreeMap::new(),
            running: AtomicBool::new(false),
        }
    }

    /// Start the snapshot manager
    pub fn start(&self) {
        self.running.store(true, Ordering::SeqCst);
    }

    /// Stop the snapshot manager
    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if manager is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Add a snapshot
    pub fn add_snapshot(&mut self, snapshot: Snapshot) {
        self.snapshots
            .insert(snapshot.snapshot_id.clone(), snapshot);
    }

    /// Remove a snapshot by ID
    pub fn remove_snapshot(&mut self, snapshot_id: &str) -> Option<Snapshot> {
        self.snapshots.remove(snapshot_id)
    }

    /// Get snapshot by ID
    pub fn get_snapshot(&self, snapshot_id: &str) -> Option<&Snapshot> {
        self.snapshots.get(snapshot_id)
    }

    /// Get mutable snapshot by ID
    pub fn get_snapshot_mut(&mut self, snapshot_id: &str) -> Option<&mut Snapshot> {
        self.snapshots.get_mut(snapshot_id)
    }

    /// List all snapshots
    pub fn list_snapshots(&self) -> Vec<&Snapshot> {
        self.snapshots.values().collect()
    }

    /// List snapshots by source path
    pub fn list_snapshots_by_source(&self, source_path: &str) -> Vec<&Snapshot> {
        self.snapshots
            .values()
            .filter(|s| s.source_path == source_path)
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

    /// Restore/rollback to a snapshot
    pub fn restore_snapshot(&mut self, snapshot_id: &str) -> bool {
        if let Some(snapshot) = self.snapshots.get_mut(snapshot_id) {
            snapshot.set_state(SnapshotState::Restoring);
            // After successful restore, mark active again
            snapshot.set_state(SnapshotState::Active);
            true
        } else {
            false
        }
    }

    /// Add an automated snapshot policy
    pub fn add_policy(&mut self, policy: SnapshotPolicy) {
        self.policies.insert(policy.policy_id.clone(), policy);
    }

    /// Remove a policy by ID
    pub fn remove_policy(&mut self, policy_id: &str) -> Option<SnapshotPolicy> {
        self.policies.remove(policy_id)
    }

    /// Get a policy by ID
    pub fn get_policy(&self, policy_id: &str) -> Option<&SnapshotPolicy> {
        self.policies.get(policy_id)
    }

    /// List all policies
    pub fn list_policies(&self) -> Vec<&SnapshotPolicy> {
        self.policies.values().collect()
    }

    /// Get snapshot count
    pub fn snapshot_count(&self) -> usize {
        self.snapshots.len()
    }

    /// Get policy count
    pub fn policy_count(&self) -> usize {
        self.policies.len()
    }

    /// Cleanup snapshots exceeding max retention policy
    pub fn cleanup_old_snapshots(&mut self, source_path: &str) -> usize {
        let max_allowed = self
            .policies
            .values()
            .find(|p| p.source_path == source_path && p.enabled && p.auto_cleanup)
            .map(|p| p.max_snapshots)
            .unwrap_or(usize::MAX);

        let ids_to_remove: Vec<String> = {
            let mut matching: Vec<&Snapshot> = self
                .snapshots
                .values()
                .filter(|s| s.source_path == source_path)
                .collect();

            if matching.len() > max_allowed {
                let remove_count = matching.len() - max_allowed;
                matching.truncate(remove_count);
                matching.iter().map(|s| s.snapshot_id.clone()).collect()
            } else {
                Vec::new()
            }
        };

        let removed = ids_to_remove.len();
        for id in ids_to_remove {
            self.snapshots.remove(&id);
        }
        removed
    }
;

impl Default for SigmaSnapshot {
    fn default() -> Self {
        Self::new()
    }
;

/// Global SigmaSnapshot instance (placeholder for global state)
static mut SIGMASNAPSHOT_INSTANCE: Option<SigmaSnapshot> = None;

/// Initialize the global SigmaSnapshot instance
pub fn init_sigmasnapshot() {
    unsafe {
        SIGMASNAPSHOT_INSTANCE = Some(SigmaSnapshot::new());
    }
;

/// Get the global SigmaSnapshot instance
pub fn get_sigmasnapshot() -> Option<&'static SigmaSnapshot> {
    unsafe { SIGMASNAPSHOT_INSTANCE.as_ref() }
;

/// Get mutable global SigmaSnapshot instance
pub fn get_sigmasnapshot_mut() -> Option<&'static mut SigmaSnapshot> {
    unsafe { SIGMASNAPSHOT_INSTANCE.as_mut() }
;

/// Convenience function to add a snapshot
pub fn add_snapshot(snapshot: Snapshot) {
    if let Some(mgr) = get_sigmasnapshot_mut() {
        mgr.add_snapshot(snapshot);
    }
;

/// Convenience function to restore a snapshot
pub fn restore_snapshot(snapshot_id: &str) -> bool {
    if let Some(mgr) = get_sigmasnapshot_mut() {
        mgr.restore_snapshot(snapshot_id)
    } else {
        false
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_creation() {
        let snapshot = Snapshot::new(
            "snap-1".to_string(),
            "root-snap-20261013".to_string(),
            "/".to_string(),
            "/.snapshots/1".to_string(),
            SnapshotType::ReadOnly,
            SnapshotSchedule::Daily,
            500_000_000,
        );

        assert_eq!(snapshot.snapshot_id, "snap-1");
        assert_eq!(snapshot.snapshot_name, "root-snap-20261013");
        assert_eq!(snapshot.source_path, "/");
        assert_eq!(snapshot.snapshot_type, SnapshotType::ReadOnly);
        assert_eq!(snapshot.state, SnapshotState::Active);
        assert!(snapshot.is_active());
        assert!(snapshot.is_read_only());
        assert_eq!(snapshot.size_bytes, 500_000_000);
    }

    #[test]
    fn test_snapshot_lifecycle_and_states() {
        let mut snapshot = Snapshot::new(
            "snap-2".to_string(),
            "home-snap-1".to_string(),
            "/home".to_string(),
            "/.snapshots/home-1".to_string(),
            SnapshotType::ReadWrite,
            SnapshotSchedule::Hourly,
            1_000_000,
        );

        snapshot.set_state(SnapshotState::Restoring);
        assert_eq!(snapshot.state, SnapshotState::Restoring);
        assert!(!snapshot.is_active());

        snapshot.set_parent(Some("snap-1".to_string()));
        assert_eq!(snapshot.parent_snapshot_id, Some("snap-1".to_string()));
    }

    #[test]
    fn test_sigmasnapshot_manager() {
        let mut mgr = SigmaSnapshot::new();
        mgr.start();
        assert!(mgr.is_running());

        let s1 = Snapshot::new(
            "s1".to_string(),
            "snap-root-1".to_string(),
            "/".to_string(),
            "/.snapshots/s1".to_string(),
            SnapshotType::ReadOnly,
            SnapshotSchedule::Daily,
            100,
        );
        let s2 = Snapshot::new(
            "s2".to_string(),
            "snap-root-2".to_string(),
            "/".to_string(),
            "/.snapshots/s2".to_string(),
            SnapshotType::Incremental,
            SnapshotSchedule::Hourly,
            50,
        );
        let s3 = Snapshot::new(
            "s3".to_string(),
            "snap-data-1".to_string(),
            "/data".to_string(),
            "/data/.snapshots/s3".to_string(),
            SnapshotType::Full,
            SnapshotSchedule::Weekly,
            500,
        );

        mgr.add_snapshot(s1);
        mgr.add_snapshot(s2);
        mgr.add_snapshot(s3);
        assert_eq!(mgr.snapshot_count(), 3);

        let root_snaps = mgr.list_snapshots_by_source("/");
        assert_eq!(root_snaps.len(), 2);

        let inc_snaps = mgr.list_snapshots_by_type(SnapshotType::Incremental);
        assert_eq!(inc_snaps.len(), 1);

        assert!(mgr.restore_snapshot("s1"));

        let removed = mgr.remove_snapshot("s3");
        assert!(removed.is_some());
        assert_eq!(mgr.snapshot_count(), 2);

        mgr.stop();
        assert!(!mgr.is_running());
    }

    #[test]
    fn test_snapshot_policy_and_cleanup() {
        let mut mgr = SigmaSnapshot::new();
        let policy = SnapshotPolicy::new(
            "pol-root".to_string(),
            "/".to_string(),
            2,
            true,
            SnapshotSchedule::Hourly,
        );
        mgr.add_policy(policy);
        assert_eq!(mgr.policy_count(), 1);

        for i in 1..=4 {
            let snap = Snapshot::new(
                alloc::format!("snap-{}", i),
                alloc::format!("name-{}", i),
                "/".to_string(),
                alloc::format!("/.snapshots/{}", i),
                SnapshotType::ReadOnly,
                SnapshotSchedule::Hourly,
                100,
            );
            mgr.add_snapshot(snap);
        }
        assert_eq!(mgr.snapshot_count(), 4);

        let cleaned = mgr.cleanup_old_snapshots("/");
        assert_eq!(cleaned, 2);
        assert_eq!(mgr.snapshot_count(), 2);
    }

    #[test]
    fn test_global_instance() {
        init_sigmasnapshot();
        let snap = Snapshot::new(
            "global-snap".to_string(),
            "global-name".to_string(),
            "/var".to_string(),
            "/var/.snapshots/1".to_string(),
            SnapshotType::ReadOnly,
            SnapshotSchedule::Manual,
            1024,
        );
        add_snapshot(snap);

        let mgr = get_sigmasnapshot().unwrap();
        assert_eq!(mgr.snapshot_count(), 1);
        assert!(restore_snapshot("global-snap"));
    }
;
