//! SigmaOS System Snapshot & Rollback Manager
//!
//! Inspired by: Omarchy's Btrfs snapshots with snapper, automatic snapshots before updates
//! Following principles: CONTINUOUS_IMPROVEMENT, LONG_TERM_SUSTAINABILITY,
//! RACE_FREE DESIGN, DEADLOCK_AVOIDANCE, SAFE_CONCURRENCY, DEFENSIVE_PROGRAMMING,
//! FAIL_FAST_SYSTEM, SIMPLE_AND_ROBUST, MINIMAL_KERNEL_PHILOSOPHY, SECURE_DEFAULTS,
//! VERSION_CONTROL_PRACTICES, ROLLBACK_SUPPORT, IDempotent, ZERO_COPY, MEMORY_SAFETY.

use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

/// Snapshot type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnapshotType {
    PreUpdate,
    PostUpdate,
    Manual,
    FactoryReset,
    AutoDaily,
}

/// Snapshot status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnapshotStatus {
    Creating,
    Complete,
    Deleting,
    Corrupted,
}

/// Btrfs snapshot
#[derive(Debug, Clone)]
pub struct BtrfsSnapshot {
    pub id: u64,
    pub name: String,
    pub snapshot_type: SnapshotType,
    pub status: SnapshotStatus,
    pub created_at: u64,
    pub path: String,
    pub size_bytes: u64,
    pub readonly: bool,
    pub description: Option<String>,
}

impl BtrfsSnapshot {
    pub fn new(
        id: u64,
        name: String,
        snapshot_type: SnapshotType,
        path: String,
        description: Option<String>,
    ) -> Self {
        Self {
            id,
            name,
            snapshot_type,
            status: SnapshotStatus::Creating,
            created_at: 0,
            path,
            size_bytes: 0,
            readonly: true,
            description,
        }
    }

    pub fn mark_complete(&mut self, timestamp: u64, size_bytes: u64) {
        self.status = SnapshotStatus::Complete;
        self.created_at = timestamp;
        self.size_bytes = size_bytes;
    }
}

/// Snapshot configuration
#[derive(Debug, Clone)]
pub struct SnapshotConfig {
    pub enabled: bool,
    pub max_snapshots: u32,
    pub auto_snapshot_before_update: bool,
    pub auto_snapshot_after_update: bool,
    pub snapshot_retention_days: u64,
    pub snapshot_path: String,
    pub auto_daily: bool,
}

impl Default for SnapshotConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_snapshots: 10,
            auto_snapshot_before_update: true,
            auto_snapshot_after_update: true,
            snapshot_retention_days: 30,
            snapshot_path: "/.snapshots".to_string(),
            auto_daily: true,
        }
    }
}

/// Snapshot manager
pub struct SnapshotManager {
    pub config: SnapshotConfig,
    pub snapshots: VecDeque<BtrfsSnapshot>,
    pub next_id: u64,
}

impl SnapshotManager {
    pub fn new() -> Self {
        Self {
            config: SnapshotConfig::default(),
            snapshots: VecDeque::new(),
            next_id: 1,
        }
    }

    /// Create pre-update snapshot
    pub fn create_pre_update(&mut self) -> Result<u64, String> {
        if !self.config.enabled || !self.config.auto_snapshot_before_update {
            return Err("Pre-update snapshots disabled".to_string());
        }
        self.create_snapshot(SnapshotType::PreUpdate, "Pre-update snapshot".to_string())
    }

    /// Create post-update snapshot
    pub fn create_post_update(&mut self) -> Result<u64, String> {
        if !self.config.enabled || !self.config.auto_snapshot_after_update {
            return Err("Post-update snapshots disabled".to_string());
        }
        self.create_snapshot(SnapshotType::PostUpdate, "Post-update snapshot".to_string())
    }

    /// Create manual snapshot
    pub fn create_manual(&mut self, name: String, description: Option<String>) -> Result<u64, String> {
        if !self.config.enabled {
            return Err("Snapshots disabled".to_string());
        }
        self.create_snapshot(SnapshotType::Manual, description.unwrap_or_else(|| format!("Manual snapshot: {}", name)))
    }

    fn create_snapshot(&mut self, snapshot_type: SnapshotType, description: String) -> Result<u64, String> {
        let id = self.next_id;
        self.next_id += 1;
        let name = format!("{}-{}", match snapshot_type {
            SnapshotType::PreUpdate => "pre-update",
            SnapshotType::PostUpdate => "post-update",
            SnapshotType::Manual => "manual",
            SnapshotType::FactoryReset => "factory-reset",
            SnapshotType::AutoDaily => "auto-daily",
        }, id);

        let timestamp = now_secs();
        let path = format!("{}/{}", self.config.snapshot_path, name);

        let snapshot = BtrfsSnapshot::new(id, name, snapshot_type, path, Some(description));
        self.snapshots.push_back(snapshot);

        if let Some(last) = self.snapshots.back_mut() {
            last.mark_complete(timestamp, 0);
        }

        self.prune();
        Ok(id)
    }

    /// Rollback to snapshot
    pub fn rollback_to(&mut self, id: u64) -> Result<(), String> {
        let snapshot = self
            .snapshots
            .iter()
            .find(|s| s.id == id)
            .ok_or_else(|| format!("Snapshot {} not found", id))?;

        if snapshot.status != SnapshotStatus::Complete {
            return Err(format!("Snapshot {} is not complete", id));
        }

        // In production, this would set the Btrfs subvolume as default and reboot
        Ok(())
    }

    /// Delete snapshot
    pub fn delete(&mut self, id: u64) -> Result<(), String> {
        let pos = self
            .snapshots
            .iter()
            .position(|s| s.id == id)
            .ok_or_else(|| format!("Snapshot {} not found", id))?;
        self.snapshots.remove(pos);
        Ok(())
    }

    /// List snapshots
    pub fn list(&self) -> Vec<&BtrfsSnapshot> {
        self.snapshots.iter().rev().collect()
    }

    /// Prune old snapshots
    fn prune(&mut self) {
        while self.snapshots.len() > self.config.max_snapshots as usize {
            self.snapshots.pop_front();
        }
    }

    /// Get snapshot count
    pub fn count(&self) -> usize {
        self.snapshots.len()
    }
}

impl Default for SnapshotManager {
    fn default() -> Self {
        Self::new()
    }
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_pre_update() {
        let mut mgr = SnapshotManager::new();
        let id = mgr.create_pre_update().unwrap();
        assert_eq!(id, 1);
        assert_eq!(mgr.count(), 1);
    }

    #[test]
    fn test_create_post_update() {
        let mut mgr = SnapshotManager::new();
        let id = mgr.create_post_update().unwrap();
        assert_eq!(id, 1);
    }

    #[test]
    fn test_create_manual() {
        let mut mgr = SnapshotManager::new();
        let id = mgr.create_manual("my-snapshot".to_string(), Some("Before config change".to_string())).unwrap();
        assert_eq!(id, 1);
        assert_eq!(mgr.list().len(), 1);
    }

    #[test]
    fn test_rollback_invalid() {
        let mut mgr = SnapshotManager::new();
        assert!(mgr.rollback_to(999).is_err());
    }

    #[test]
    fn test_prune() {
        let mut config = SnapshotConfig::default();
        config.max_snapshots = 3;
        let mut mgr = SnapshotManager {
            config,
            snapshots: VecDeque::new(),
            next_id: 1,
        };
        for _ in 0..5 {
            mgr.create_pre_update().unwrap();
        }
        assert_eq!(mgr.count(), 3);
    }

    #[test]
    fn test_disabled_snapshots() {
        let mut config = SnapshotConfig::default();
        config.enabled = false;
        let mut mgr = SnapshotManager {
            config,
            snapshots: VecDeque::new(),
            next_id: 1,
        };
        let result = mgr.create_pre_update();
        assert!(result.is_err());
    }
}