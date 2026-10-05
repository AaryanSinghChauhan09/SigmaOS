//! Snapshot Manager
//!
//! System snapshot and backup management inspired by Linux Mint's Timeshift,
//! supporting filesystem snapshots, scheduled backups, and system rollback.

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Snapshot type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotType {
    Manual,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Boot,
}

impl SnapshotType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "manual" => Some(SnapshotType::Manual),
            "hourly" => Some(SnapshotType::Hourly),
            "daily" => Some(SnapshotType::Daily),
            "weekly" => Some(SnapshotType::Weekly),
            "monthly" => Some(SnapshotType::Monthly),
            "boot" => Some(SnapshotType::Boot),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            SnapshotType::Manual => "Manual",
            SnapshotType::Hourly => "Hourly",
            SnapshotType::Daily => "Daily",
            SnapshotType::Weekly => "Weekly",
            SnapshotType::Monthly => "Monthly",
            SnapshotType::Boot => "Boot",
        }
    }
}

/// Snapshot status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotStatus {
    Creating,
    Complete,
    Failed,
    Deleting,
}

impl SnapshotStatus {
    pub fn as_str(&self) -> &str {
        match self {
            SnapshotStatus::Creating => "Creating",
            SnapshotStatus::Complete => "Complete",
            SnapshotStatus::Failed => "Failed",
            SnapshotStatus::Deleting => "Deleting",
        }
    }
}

/// Snapshot metadata
#[derive(Debug, Clone)]
pub struct SnapshotMetadata {
    pub id: String,
    pub snapshot_type: SnapshotType,
    pub status: SnapshotStatus,
    pub created_at: u64,
    pub size_bytes: u64,
    pub description: String,
    pub is_bootable: bool,
}

impl SnapshotMetadata {
    pub fn new(id: String, snapshot_type: SnapshotType) -> Self {
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            id,
            snapshot_type,
            status: SnapshotStatus::Creating,
            created_at,
            size_bytes: 0,
            description: String::new(),
            is_bootable: false,
        }
    }

    pub fn set_status(&mut self, status: SnapshotStatus) {
        self.status = status;
    }

    pub fn set_size(&mut self, size: u64) {
        self.size_bytes = size;
    }

    pub fn set_description(&mut self, desc: String) {
        self.description = desc;
    }

    pub fn set_bootable(&mut self, bootable: bool) {
        self.is_bootable = bootable;
    }

    pub fn is_complete(&self) -> bool {
        self.status == SnapshotStatus::Complete
    }

    pub fn size_human(&self) -> String {
        const GB: u64 = 1024 * 1024 * 1024;
        const MB: u64 = 1024 * 1024;

        if self.size_bytes >= GB {
            format!("{:.1} GB", self.size_bytes as f64 / GB as f64)
        } else if self.size_bytes >= MB {
            format!("{:.1} MB", self.size_bytes as f64 / MB as f64)
        } else {
            format!("{} KB", self.size_bytes / 1024)
        }
    }
}

/// Retention policy
#[derive(Debug, Clone)]
pub struct RetentionPolicy {
    pub max_hourly: u32,
    pub max_daily: u32,
    pub max_weekly: u32,
    pub max_monthly: u32,
    pub max_manual: u32,
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            max_hourly: 6,
            max_daily: 7,
            max_weekly: 4,
            max_monthly: 6,
            max_manual: 10,
        }
    }
}

/// Snapshot manager
#[derive(Debug)]
pub struct SnapshotManager {
    snapshots: HashMap<String, SnapshotMetadata>,
    retention_policy: RetentionPolicy,
    next_id: u64,
}

impl SnapshotManager {
    pub fn new() -> Self {
        Self {
            snapshots: HashMap::new(),
            retention_policy: RetentionPolicy::default(),
            next_id: 1,
        }
    }

    pub fn with_retention(policy: RetentionPolicy) -> Self {
        Self {
            snapshots: HashMap::new(),
            retention_policy: policy,
            next_id: 1,
        }
    }

    /// Set retention policy
    pub fn set_retention_policy(&mut self, policy: RetentionPolicy) {
        self.retention_policy = policy;
    }

    /// Get retention policy
    pub fn get_retention_policy(&self) -> &RetentionPolicy {
        &self.retention_policy
    }

    /// Create a snapshot
    pub fn create_snapshot(&mut self, snapshot_type: SnapshotType, description: String) -> Result<String, String> {
        let id = format!("snapshot-{}", self.next_id);
        self.next_id += 1;

        let mut snapshot = SnapshotMetadata::new(id.clone(), snapshot_type);
        snapshot.set_description(description);

        // Simulate snapshot creation
        snapshot.set_status(SnapshotStatus::Complete);
        snapshot.set_size(1024 * 1024 * 1024); // 1 GB
        snapshot.set_bootable(true);

        self.snapshots.insert(id.clone(), snapshot);

        // Apply retention policy
        self.apply_retention();

        Ok(id)
    }

    /// Get a snapshot
    pub fn get_snapshot(&self, id: &str) -> Option<&SnapshotMetadata> {
        self.snapshots.get(id)
    }

    /// Get a snapshot mutably
    pub fn get_snapshot_mut(&mut self, id: &str) -> Option<&mut SnapshotMetadata> {
        self.snapshots.get_mut(id)
    }

    /// List all snapshots
    pub fn list_snapshots(&self) -> Vec<&SnapshotMetadata> {
        let mut snapshots: Vec<_> = self.snapshots.values().collect();
        snapshots.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        snapshots
    }

    /// List snapshots by type
    pub fn list_by_type(&self, snapshot_type: SnapshotType) -> Vec<&SnapshotMetadata> {
        self.snapshots.values()
            .filter(|s| s.snapshot_type == snapshot_type)
            .collect()
    }

    /// Delete a snapshot
    pub fn delete_snapshot(&mut self, id: &str) -> Result<(), String> {
        let snapshot = self.snapshots.get_mut(id)
            .ok_or_else(|| format!("Snapshot {} not found", id))?;

        snapshot.set_status(SnapshotStatus::Deleting);

        // Simulate deletion
        self.snapshots.remove(id);

        Ok(())
    }

    /// Restore from a snapshot
    pub fn restore_snapshot(&mut self, id: &str) -> Result<(), String> {
        let snapshot = self.snapshots.get(id)
            .ok_or_else(|| format!("Snapshot {} not found", id))?;

        if !snapshot.is_complete() {
            return Err(format!("Snapshot {} is not complete", id));
        }

        if !snapshot.is_bootable {
            return Err(format!("Snapshot {} is not bootable", id));
        }

        // Simulate restoration
        Ok(())
    }

    /// Apply retention policy
    fn apply_retention(&mut self) {
        let types = vec![
            (SnapshotType::Hourly, self.retention_policy.max_hourly),
            (SnapshotType::Daily, self.retention_policy.max_daily),
            (SnapshotType::Weekly, self.retention_policy.max_weekly),
            (SnapshotType::Monthly, self.retention_policy.max_monthly),
            (SnapshotType::Manual, self.retention_policy.max_manual),
        ];

        for (snapshot_type, max_count) in types {
            let mut snapshots: Vec<_> = self.snapshots.values()
                .filter(|s| s.snapshot_type == snapshot_type)
                .collect();

            snapshots.sort_by(|a, b| b.created_at.cmp(&a.created_at));

            let to_remove: Vec<String> = snapshots.iter()
                .skip(max_count as usize)
                .map(|s| s.id.clone())
                .collect();

            for id in to_remove {
                let _ = self.delete_snapshot(&id);
            }
        }
    }

    /// Get statistics
    pub fn get_statistics(&self) -> SnapshotStatistics {
        let total_snapshots = self.snapshots.len();
        let total_size: u64 = self.snapshots.values()
            .map(|s| s.size_bytes)
            .sum();

        let by_type = |t| self.snapshots.values().filter(|s| s.snapshot_type == t).count();

        SnapshotStatistics {
            total_snapshots,
            total_size,
            manual_snapshots: by_type(SnapshotType::Manual),
            hourly_snapshots: by_type(SnapshotType::Hourly),
            daily_snapshots: by_type(SnapshotType::Daily),
            weekly_snapshots: by_type(SnapshotType::Weekly),
            monthly_snapshots: by_type(SnapshotType::Monthly),
            boot_snapshots: by_type(SnapshotType::Boot),
        }
    }
}

impl Default for SnapshotManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Snapshot statistics
#[derive(Debug, Clone)]
pub struct SnapshotStatistics {
    pub total_snapshots: usize,
    pub total_size: u64,
    pub manual_snapshots: usize,
    pub hourly_snapshots: usize,
    pub daily_snapshots: usize,
    pub weekly_snapshots: usize,
    pub monthly_snapshots: usize,
    pub boot_snapshots: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_type_from_str() {
        assert_eq!(SnapshotType::from_str("manual"), Some(SnapshotType::Manual));
        assert_eq!(SnapshotType::from_str("daily"), Some(SnapshotType::Daily));
    }

    #[test]
    fn test_snapshot_metadata_creation() {
        let snapshot = SnapshotMetadata::new("test".to_string(), SnapshotType::Manual);
        assert_eq!(snapshot.id, "test");
        assert_eq!(snapshot.snapshot_type, SnapshotType::Manual);
    }

    #[test]
    fn test_snapshot_manager_creation() {
        let manager = SnapshotManager::new();
        assert_eq!(manager.list_snapshots().len(), 0);
    }

    #[test]
    fn test_create_snapshot() {
        let mut manager = SnapshotManager::new();
        let id = manager.create_snapshot(SnapshotType::Manual, "Test snapshot".to_string()).unwrap();
        assert!(manager.get_snapshot(&id).is_some());
    }

    #[test]
    fn test_delete_snapshot() {
        let mut manager = SnapshotManager::new();
        let id = manager.create_snapshot(SnapshotType::Manual, "Test".to_string()).unwrap();
        assert!(manager.delete_snapshot(&id).is_ok());
        assert!(manager.get_snapshot(&id).is_none());
    }

    #[test]
    fn test_restore_snapshot() {
        let mut manager = SnapshotManager::new();
        let id = manager.create_snapshot(SnapshotType::Manual, "Test".to_string()).unwrap();
        assert!(manager.restore_snapshot(&id).is_ok());
    }

    #[test]
    fn test_retention_policy() {
        let mut manager = SnapshotManager::new();
        for _ in 0..10 {
            manager.create_snapshot(SnapshotType::Daily, "Test".to_string()).ok();
        }
        let stats = manager.get_statistics();
        assert!(stats.daily_snapshots <= 7);
    }

    #[test]
    fn test_statistics() {
        let mut manager = SnapshotManager::new();
        manager.create_snapshot(SnapshotType::Manual, "Test".to_string()).ok();
        let stats = manager.get_statistics();
        assert_eq!(stats.total_snapshots, 1);
    }
}
