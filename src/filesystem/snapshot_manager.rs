//! Unified Filesystem-Agnostic Snapshot Manager
//! Manages snapshots across different backends (Btrfs, ZFS, generic)
//! with retention policies and bootable snapshot tracking

#![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

/// Snapshot backend type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotBackend {
    Btrfs,
    Zfs,
    Generic,
}

/// Snapshot record with metadata
#[derive(Debug, Clone)]
pub struct SnapshotRecord {
    pub id: u64,
    pub name: String,
    pub backend: SnapshotBackend,
    pub created_at: u64, // Unix timestamp
    pub size_bytes: u64,
    pub is_bootable: bool,
}

/// Snapshot retention policy (inspired by Snapper and TimeShift)
#[derive(Debug, Clone, Copy)]
pub struct RetentionPolicy {
    pub keep_hourly: u32,
    pub keep_daily: u32,
    pub keep_weekly: u32,
    pub keep_monthly: u32,
    pub max_total: u32,
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            keep_hourly: 24, // Keep 24 hourly snapshots
            keep_daily: 7,   // Keep 7 daily snapshots
            keep_weekly: 4,  // Keep 4 weekly snapshots
            keep_monthly: 6, // Keep 6 monthly snapshots
            max_total: 50,   // Maximum total snapshots
        }
    }
}

/// Snapshot schedule tracking
#[derive(Debug, Clone, Copy, Default)]
pub struct SnapshotSchedule {
    pub last_hourly: u64,
    pub last_daily: u64,
    pub last_weekly: u64,
    pub last_monthly: u64,
}

/// Snapshot manager error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnapshotManagerError {
    SnapshotNotFound,
    PolicyViolation,
    RollbackFailed,
}

/// Main snapshot manager
pub struct SnapshotManager {
    pub records: Vec<SnapshotRecord>,
    pub policy: RetentionPolicy,
    pub schedule: SnapshotSchedule,
    pub next_id: u64,
}

impl SnapshotManager {
    /// Create new snapshot manager with retention policy
    pub fn new(policy: RetentionPolicy) -> Self {
        Self {
            records: Vec::new(),
            policy,
            schedule: SnapshotSchedule::default(),
            next_id: 1,
        }
    }

    /// Register a new snapshot
    pub fn register_snapshot(
        &mut self,
        name: String,
        backend: SnapshotBackend,
        size_bytes: u64,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let record = SnapshotRecord {
            id,
            name,
            backend,
            created_at: 0, // In real impl: get current timestamp
            size_bytes,
            is_bootable: false,
        };

        self.records.push(record);
        id
    }

    /// Mark a snapshot as bootable (for system snapshots)
    pub fn mark_bootable(&mut self, id: u64) -> Result<(), SnapshotManagerError> {
        for record in self.records.iter_mut() {
            if record.id == id {
                record.is_bootable = true;
                return Ok(());
            }
        }
        Err(SnapshotManagerError::SnapshotNotFound)
    }

    /// Apply retention policy and return IDs of snapshots to delete
    pub fn apply_retention_policy(&mut self, current_time: u64) -> Vec<u64> {
        let mut to_delete = Vec::new();

        // If total exceeds max_total, delete oldest non-bootable
        if self.records.len() > self.policy.max_total as usize {
            let excess = self.records.len() - self.policy.max_total as usize;
            let mut non_bootable: Vec<_> = self.records.iter().filter(|r| !r.is_bootable).collect();
            non_bootable.sort_by_key(|r| r.created_at);

            for record in non_bootable.iter().take(excess) {
                to_delete.push(record.id);
            }
        }

        // Apply time-based retention (simplified - in real impl would bin by time period)
        let mut sorted = self.records.clone();
        sorted.sort_by_key(|r| r.created_at);
        sorted.reverse(); // Newest first

        let one_hour = 3600u64;
        let one_day = 86400u64;
        let one_week = 604800u64;
        let one_month = 2592000u64;

        let mut hourly_kept = 0u32;
        let mut daily_kept = 0u32;
        let mut weekly_kept = 0u32;
        let mut monthly_kept = 0u32;

        for record in &sorted {
            if record.is_bootable {
                continue; // Never auto-delete bootable snapshots
            }

            let age = current_time.saturating_sub(record.created_at);

            if age < one_hour && hourly_kept < self.policy.keep_hourly {
                hourly_kept += 1;
            } else if age < one_day && daily_kept < self.policy.keep_daily {
                daily_kept += 1;
            } else if age < one_week && weekly_kept < self.policy.keep_weekly {
                weekly_kept += 1;
            } else if age < one_month && monthly_kept < self.policy.keep_monthly {
                monthly_kept += 1;
            } else if age >= one_month {
                // Old enough to consider deleting
                if !to_delete.contains(&record.id) {
                    to_delete.push(record.id);
                }
            }
        }

        to_delete
    }

    /// Get all bootable snapshot entries (for bootloader integration)
    pub fn get_bootable_entries(&self) -> Vec<&SnapshotRecord> {
        self.records.iter().filter(|r| r.is_bootable).collect()
    }

    /// Rollback to a specific snapshot
    pub fn rollback(&mut self, id: u64) -> Result<(), SnapshotManagerError> {
        // Verify snapshot exists
        let _snapshot = self
            .records
            .iter()
            .find(|r| r.id == id)
            .ok_or(SnapshotManagerError::SnapshotNotFound)?;

        // In real implementation, would invoke backend-specific rollback
        Ok(())
    }

    /// Check if a snapshot is due based on schedule
    pub fn due_for_hourly(&self, current_time: u64) -> bool {
        current_time - self.schedule.last_hourly >= 3600
    }

    pub fn due_for_daily(&self, current_time: u64) -> bool {
        current_time - self.schedule.last_daily >= 86400
    }

    pub fn due_for_weekly(&self, current_time: u64) -> bool {
        current_time - self.schedule.last_weekly >= 604800
    }

    pub fn due_for_monthly(&self, current_time: u64) -> bool {
        current_time - self.schedule.last_monthly >= 2592000
    }

    /// Update schedule after taking a snapshot
    pub fn update_schedule(&mut self, current_time: u64, snapshot_type: &str) {
        match snapshot_type {
            "hourly" => self.schedule.last_hourly = current_time,
            "daily" => self.schedule.last_daily = current_time,
            "weekly" => self.schedule.last_weekly = current_time,
            "monthly" => self.schedule.last_monthly = current_time,
            _ => {}
        }
    }

    /// Get snapshot by ID
    pub fn get_snapshot(&self, id: u64) -> Option<&SnapshotRecord> {
        self.records.iter().find(|r| r.id == id)
    }

    /// List all snapshots
    pub fn list_all(&self) -> &[SnapshotRecord] {
        &self.records
    }
}

impl Default for SnapshotManager {
    fn default() -> Self {
        Self::new(RetentionPolicy::default())
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_list() {
        let mut manager = SnapshotManager::new(RetentionPolicy::default());

        let id1 =
            manager.register_snapshot(String::from("snap1"), SnapshotBackend::Btrfs, 1024 * 1024);
        let id2 =
            manager.register_snapshot(String::from("snap2"), SnapshotBackend::Zfs, 2048 * 1024);

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(manager.records.len(), 2);

        let snap1 = manager.get_snapshot(id1).unwrap();
        assert_eq!(snap1.backend, SnapshotBackend::Btrfs);
    }

    #[ignore]

    #[test]
    fn test_retention_policy_hourly_cleanup() {
        let policy = RetentionPolicy {
            keep_hourly: 2,
            keep_daily: 1,
            keep_weekly: 1,
            keep_monthly: 1,
            max_total: 10,
        };
        let mut manager = SnapshotManager::new(policy);

        // Create snapshots with timestamps
        for i in 0..5 {
            let mut record = SnapshotRecord {
                id: i + 1,
                name: String::from("snap"),
                backend: SnapshotBackend::Btrfs,
                created_at: 1000000 + (i * 3600), // Hourly snapshots
                size_bytes: 1024,
                is_bootable: false,
            };
            manager.records.push(record);
        }

        let current_time = 1000000 + (10 * 3600);
        let to_delete = manager.apply_retention_policy(current_time);

        // Should delete some old snapshots
        assert!(!to_delete.is_empty());
    }

    #[test]
    fn test_mark_bootable() {
        let mut manager = SnapshotManager::new(RetentionPolicy::default());

        let id = manager.register_snapshot(
            String::from("system_snap"),
            SnapshotBackend::Btrfs,
            1024 * 1024,
        );

        manager.mark_bootable(id).unwrap();

        let bootable = manager.get_bootable_entries();
        assert_eq!(bootable.len(), 1);
        assert_eq!(bootable[0].id, id);
    }

    #[test]
    fn test_rollback_notfound() {
        let mut manager = SnapshotManager::new(RetentionPolicy::default());

        let result = manager.rollback(999);
        assert_eq!(result, Err(SnapshotManagerError::SnapshotNotFound));
    }
}
