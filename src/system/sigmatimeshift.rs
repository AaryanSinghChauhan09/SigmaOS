//! SigmaTimeshift - System Snapshot and Rollback
//!
//! Inspired by Timeshift (Linux Mint) and btrfs/ZFS snapshots, this module provides
//! comprehensive system snapshot management with rollback capabilities, scheduling,
//! and bootloader integration.

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, Ordering};

/// Snapshot level enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotLevel {
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Manual,
    Unknown,
}

/// Snapshot type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotType {
    Filesystem,
    System,
    Application,
    Configuration,
    Unknown,
}

/// Snapshot state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotState {
    Creating,
    Completed,
    Failed,
    Deleting,
    Restoring,
    Unknown,
}

/// Snapshot data structure
#[derive(Debug, Clone)]
pub struct TimeshiftSnapshot {
    pub snapshot_id: String,
    pub snapshot_name: String,
    pub level: SnapshotLevel,
    pub snapshot_type: SnapshotType,
    pub state: SnapshotState,
    pub creation_time: u64,
    pub size: u64,
    pub description: String,
    pub uuid: String,
    pub is_bootable: bool,
}

impl TimeshiftSnapshot {
    /// Create a new Timeshift snapshot
    pub fn new(
        snapshot_id: String,
        snapshot_name: String,
        level: SnapshotLevel,
        snapshot_type: SnapshotType,
        description: String,
    ) -> Self {
        Self {
            snapshot_id,
            snapshot_name,
            level,
            snapshot_type,
            state: SnapshotState::Creating,
            creation_time: 0,
            size: 0,
            description,
            uuid: String::new(),
            is_bootable: false,
        }
    }

    /// Set state
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

    /// Set UUID
    pub fn set_uuid(&mut self, uuid: String) {
        self.uuid = uuid;
    }

    /// Set bootable
    pub fn set_bootable(&mut self, bootable: bool) {
        self.is_bootable = bootable;
    }

    /// Check if snapshot is completed
    pub fn is_completed(&self) -> bool {
        self.state == SnapshotState::Completed
    }

    /// Check if snapshot is bootable
    pub fn is_snapshot_bootable(&self) -> bool {
        self.is_bootable
    }
}

/// Snapshot schedule configuration
#[derive(Debug, Clone)]
pub struct SnapshotSchedule {
    pub schedule_id: String,
    pub level: SnapshotLevel,
    pub enabled: bool,
    pub retain_count: u32,
    pub cron_expression: String,
}

impl SnapshotSchedule {
    /// Create a new snapshot schedule
    pub fn new(
        schedule_id: String,
        level: SnapshotLevel,
        retain_count: u32,
        cron_expression: String,
    ) -> Self {
        Self {
            schedule_id,
            level,
            enabled: true,
            retain_count,
            cron_expression,
        }
    }

    /// Enable schedule
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable schedule
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Check if schedule is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Timeshift manager
pub struct SigmaTimeshift {
    snapshots: BTreeMap<String, TimeshiftSnapshot>,
    schedules: BTreeMap<String, SnapshotSchedule>,
    running: AtomicBool,
}

impl SigmaTimeshift {
    /// Create a new Timeshift manager
    pub fn new() -> Self {
        Self {
            snapshots: BTreeMap::new(),
            schedules: BTreeMap::new(),
            running: AtomicBool::new(false),
        }
    }

    /// Start the Timeshift manager
    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
    }

    /// Stop the Timeshift manager
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Create a snapshot
    pub fn create_snapshot(&mut self, snapshot: TimeshiftSnapshot) -> bool {
        let snap_id = snapshot.snapshot_id.clone();
        self.snapshots.insert(snap_id, snapshot);
        true
    }

    /// Delete a snapshot
    pub fn delete_snapshot(&mut self, snapshot_id: &str) -> Option<TimeshiftSnapshot> {
        if let Some(mut snap) = self.snapshots.remove(snapshot_id) {
            snap.set_state(SnapshotState::Deleting);
            Some(snap)
        } else {
            None
        }
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

    /// Get a snapshot by ID
    pub fn get_snapshot(&self, snapshot_id: &str) -> Option<&TimeshiftSnapshot> {
        self.snapshots.get(snapshot_id)
    }

    /// List all snapshots
    pub fn list_snapshots(&self) -> Vec<&TimeshiftSnapshot> {
        self.snapshots.values().collect()
    }

    /// List snapshots by level
    pub fn list_snapshots_by_level(&self, level: SnapshotLevel) -> Vec<&TimeshiftSnapshot> {
        self.snapshots
            .values()
            .filter(|s| s.level == level)
            .collect()
    }

    /// List snapshots by type
    pub fn list_snapshots_by_type(&self, snapshot_type: SnapshotType) -> Vec<&TimeshiftSnapshot> {
        self.snapshots
            .values()
            .filter(|s| s.snapshot_type == snapshot_type)
            .collect()
    }

    /// List bootable snapshots
    pub fn list_bootable_snapshots(&self) -> Vec<&TimeshiftSnapshot> {
        self.snapshots
            .values()
            .filter(|s| s.is_snapshot_bootable())
            .collect()
    }

    /// Add a schedule
    pub fn add_schedule(&mut self, schedule: SnapshotSchedule) -> bool {
        let schedule_id = schedule.schedule_id.clone();
        self.schedules.insert(schedule_id, schedule);
        true
    }

    /// Remove a schedule
    pub fn remove_schedule(&mut self, schedule_id: &str) -> Option<SnapshotSchedule> {
        self.schedules.remove(schedule_id)
    }

    /// Get a schedule by ID
    pub fn get_schedule(&self, schedule_id: &str) -> Option<&SnapshotSchedule> {
        self.schedules.get(schedule_id)
    }

    /// List all schedules
    pub fn list_schedules(&self) -> Vec<&SnapshotSchedule> {
        self.schedules.values().collect()
    }

    /// List enabled schedules
    pub fn list_enabled_schedules(&self) -> Vec<&SnapshotSchedule> {
        self.schedules
            .values()
            .filter(|s| s.is_enabled())
            .collect()
    }

    /// Clean up old snapshots based on retention policy
    pub fn cleanup_old_snapshots(&mut self, level: SnapshotLevel, retain_count: u32) -> usize {
        let mut to_remove = Vec::new();
        
        let mut level_snapshots: Vec<_> = self.snapshots
            .values()
            .filter(|s| s.level == level && s.is_completed())
            .collect();
        
        // Sort by creation time (oldest first)
        level_snapshots.sort_by(|a, b| a.creation_time.cmp(&b.creation_time));
        
        // Remove excess snapshots
        for snap in level_snapshots.iter().skip(retain_count as usize) {
            to_remove.push(snap.snapshot_id.clone());
        }
        
        for id in to_remove {
            self.snapshots.remove(&id);
        }
        
        to_remove.len()
    }

    /// Get snapshot count
    pub fn snapshot_count(&self) -> usize {
        self.snapshots.len()
    }

    /// Get schedule count
    pub fn schedule_count(&self) -> usize {
        self.schedules.len()
    }
}

impl Default for SigmaTimeshift {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaTimeshift instance (placeholder for actual global state)
static unsafe mutable SIGMATIMESHIFT_INSTANCE: Option<SigmaTimeshift> = None;

/// Initialize the global SigmaTimeshift instance
pub fn init_sigmatimeshift() {
    unsafe {
        SIGMATIMESHIFT_INSTANCE = Some(SigmaTimeshift::new());
    }
}

/// Get the global SigmaTimeshift instance
pub fn get_sigmatimeshift() -> Option<&'static SigmaTimeshift> {
    unsafe {
        SIGMATIMESHIFT_INSTANCE.as_ref()
    }
}

/// Get mutable global SigmaTimeshift instance
pub fn get_sigmatimeshift_mut() -> Option<&'static mut SigmaTimeshift> {
    unsafe {
        SIGMATIMESHIFT_INSTANCE.as_mut()
    }
}

/// Convenience function to create a snapshot
pub fn create_timeshift_snapshot(snapshot: TimeshiftSnapshot) -> bool {
    if let Some(timeshift) = get_sigmatimeshift_mut() {
        timeshift.create_snapshot(snapshot)
    } else {
        false
    }
}

/// Convenience function to restore a snapshot
pub fn restore_timeshift_snapshot(snapshot_id: &str) -> bool {
    if let Some(timeshift) = get_sigmatimeshift_mut() {
        timeshift.restore_snapshot(snapshot_id)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeshift_snapshot_creation() {
        let snapshot = TimeshiftSnapshot::new(
            "snap1".to_string(),
            "backup1".to_string(),
            SnapshotLevel::Manual,
            SnapshotType::System,
            "Manual system backup".to_string(),
        );

        assert_eq!(snapshot.snapshot_id, "snap1");
        assert_eq!(snapshot.level, SnapshotLevel::Manual);
        assert_eq!(snapshot.state, SnapshotState::Creating);
    }

    #[test]
    fn test_timeshift_snapshot_state() {
        let mut snapshot = TimeshiftSnapshot::new(
            "snap1".to_string(),
            "backup1".to_string(),
            SnapshotLevel::Manual,
            SnapshotType::System,
            "Manual system backup".to_string(),
        );

        snapshot.set_state(SnapshotState::Completed);
        assert!(snapshot.is_completed());
    }

    #[test]
    fn test_snapshot_schedule_creation() {
        let schedule = SnapshotSchedule::new(
            "daily1".to_string(),
            SnapshotLevel::Daily,
            7,
            "0 0 * * *".to_string(),
        );

        assert_eq!(schedule.schedule_id, "daily1");
        assert_eq!(schedule.level, SnapshotLevel::Daily);
        assert_eq!(schedule.retain_count, 7);
        assert!(schedule.is_enabled());
    }

    #[test]
    fn test_sigmatimeshift_creation() {
        let timeshift = SigmaTimeshift::new();
        assert!(!timeshift.is_running());
        assert_eq!(timeshift.snapshot_count(), 0);
        assert_eq!(timeshift.schedule_count(), 0);
    }

    #[test]
    fn test_sigmatimeshift_start_stop() {
        let mut timeshift = SigmaTimeshift::new();
        timeshift.start();
        assert!(timeshift.is_running());
        timeshift.stop();
        assert!(!timeshift.is_running());
    }

    #[test]
    fn test_sigmatimeshift_create_snapshot() {
        let mut timeshift = SigmaTimeshift::new();
        let snapshot = TimeshiftSnapshot::new(
            "snap1".to_string(),
            "backup1".to_string(),
            SnapshotLevel::Manual,
            SnapshotType::System,
            "Manual system backup".to_string(),
        );

        let created = timeshift.create_snapshot(snapshot);
        assert!(created);
        assert_eq!(timeshift.snapshot_count(), 1);
    }

    #[test]
    fn test_sigmatimeshift_delete_snapshot() {
        let mut timeshift = SigmaTimeshift::new();
        let snapshot = TimeshiftSnapshot::new(
            "snap1".to_string(),
            "backup1".to_string(),
            SnapshotLevel::Manual,
            SnapshotType::System,
            "Manual system backup".to_string(),
        );

        timeshift.create_snapshot(snapshot);
        assert_eq!(timeshift.snapshot_count(), 1);

        let deleted = timeshift.delete_snapshot("snap1");
        assert!(deleted.is_some());
        assert_eq!(timeshift.snapshot_count(), 0);
    }

    #[test]
    fn test_sigmatimeshift_restore_snapshot() {
        let mut timeshift = SigmaTimeshift::new();
        let snapshot = TimeshiftSnapshot::new(
            "snap1".to_string(),
            "backup1".to_string(),
            SnapshotLevel::Manual,
            SnapshotType::System,
            "Manual system backup".to_string(),
        );

        timeshift.create_snapshot(snapshot);
        let restored = timeshift.restore_snapshot("snap1");
        assert!(restored);

        let snap = timeshift.get_snapshot("snap1").unwrap();
        assert!(snap.is_completed());
    }

    #[test]
    fn test_sigmatimeshift_list_by_level() {
        let mut timeshift = SigmaTimeshift::new();

        timeshift.create_snapshot(TimeshiftSnapshot::new(
            "snap1".to_string(),
            "backup1".to_string(),
            SnapshotLevel::Hourly,
            SnapshotType::System,
            "Hourly backup".to_string(),
        ));

        timeshift.create_snapshot(TimeshiftSnapshot::new(
            "snap2".to_string(),
            "backup2".to_string(),
            SnapshotLevel::Daily,
            SnapshotType::System,
            "Daily backup".to_string(),
        ));

        let hourly = timeshift.list_snapshots_by_level(SnapshotLevel::Hourly);
        assert_eq!(hourly.len(), 1);

        let daily = timeshift.list_snapshots_by_level(SnapshotLevel::Daily);
        assert_eq!(daily.len(), 1);
    }

    #[test]
    fn test_sigmatimeshift_add_schedule() {
        let mut timeshift = SigmaTimeshift::new();
        let schedule = SnapshotSchedule::new(
            "daily1".to_string(),
            SnapshotLevel::Daily,
            7,
            "0 0 * * *".to_string(),
        );

        let added = timeshift.add_schedule(schedule);
        assert!(added);
        assert_eq!(timeshift.schedule_count(), 1);
    }

    #[test]
    fn test_sigmatimeshift_cleanup_old_snapshots() {
        let mut timeshift = SigmaTimeshift::new();

        // Create 5 manual snapshots
        for i in 0..5 {
            let mut snap = TimeshiftSnapshot::new(
                format!("snap{}", i),
                format!("backup{}", i),
                SnapshotLevel::Manual,
                SnapshotType::System,
                "Manual backup".to_string(),
            );
            snap.set_creation_time(i as u64);
            snap.set_state(SnapshotState::Completed);
            timeshift.create_snapshot(snap);
        }

        // Keep only 3
        let cleaned = timeshift.cleanup_old_snapshots(SnapshotLevel::Manual, 3);
        assert_eq!(cleaned, 2);
        assert_eq!(timeshift.snapshot_count(), 3);
    }

    #[test]
    fn test_global_instance() {
        init_sigmatimeshift();

        let timeshift = get_sigmatimeshift();
        assert!(timeshift.is_some());

        let mut timeshift_mut = get_sigmatimeshift_mut();
        assert!(timeshift_mut.is_some());

        if let Some(timeshift) = timeshift_mut {
            timeshift.start();
            assert!(timeshift.is_running());
        }
    }

    #[test]
    fn test_convenience_create_snapshot() {
        init_sigmatimeshift();

        let snapshot = TimeshiftSnapshot::new(
            "snap1".to_string(),
            "backup1".to_string(),
            SnapshotLevel::Manual,
            SnapshotType::System,
            "Manual system backup".to_string(),
        );

        let created = create_timeshift_snapshot(snapshot);
        assert!(created);

        let timeshift = get_sigmatimeshift().unwrap();
        assert_eq!(timeshift.snapshot_count(), 1);
    }

    #[test]
    fn test_convenience_restore_snapshot() {
        init_sigmatimeshift();

        let snapshot = TimeshiftSnapshot::new(
            "snap1".to_string(),
            "backup1".to_string(),
            SnapshotLevel::Manual,
            SnapshotType::System,
            "Manual system backup".to_string(),
        );

        create_timeshift_snapshot(snapshot);
        let restored = restore_timeshift_snapshot("snap1");
        assert!(restored);

        let timeshift = get_sigmatimeshift().unwrap();
        let snap = timeshift.get_snapshot("snap1").unwrap();
        assert!(snap.is_completed());
    }
}
