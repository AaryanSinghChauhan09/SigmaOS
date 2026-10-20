// SPDX-License-Identifier: MIT
// SigmaOS - Snapshot Manager
// Snapshot management inspired by Arch (btrfs snapshots), Debian (LVM snapshots), and CachyOS

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Snapshot type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotType {
    Filesystem,
    Volume,
    System,
}

/// Snapshot state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotState {
    Creating,
    Available,
    Restoring,
    Deleting,
    Failed,
}

/// Snapshot
#[derive(Debug, Clone)]
pub struct Snapshot {
    pub id: String,
    pub name: String,
    pub snapshot_type: SnapshotType,
    pub state: SnapshotState,
    pub size_bytes: u64,
    pub timestamp: u64,
    pub source_path: String,
}

/// Snapshot manager
pub struct SigmaSnapshotManager {
    snapshots: Vec<Snapshot>,
}

impl SigmaSnapshotManager {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
        }
    }

    pub fn create_snapshot(&mut self, name: String, snapshot_type: SnapshotType, source_path: String) -> Snapshot {
        let snapshot = Snapshot {
            id: alloc::format!("snapshot_{}", self.snapshots.len()),
            name,
            snapshot_type,
            state: SnapshotState::Creating,
            size_bytes: 0,
            timestamp: 0,
            source_path,
        };
        self.snapshots.push(snapshot.clone());
        snapshot
    }

    pub fn get_snapshots(&self) -> &[Snapshot] {
        &self.snapshots
    }

    pub fn get_snapshot(&self, id: &str) -> Option<&Snapshot> {
        self.snapshots.iter().find(|s| s.id == id)
    }

    pub fn get_snapshot_mut(&mut self, id: &str) -> Option<&mut Snapshot> {
        self.snapshots.iter_mut().find(|s| s.id == id)
    }

    pub fn complete_snapshot(&mut self, id: String, size_bytes: u64) -> Result<(), String> {
        if let Some(snapshot) = self.get_snapshot_mut(&id) {
            snapshot.state = SnapshotState::Available;
            snapshot.size_bytes = size_bytes;
            Ok(())
        } else {
            Err(format!("Snapshot {} not found", id))
        }
    }

    pub fn restore_snapshot(&mut self, id: String) -> Result<(), String> {
        if let Some(snapshot) = self.get_snapshot_mut(&id) {
            snapshot.state = SnapshotState::Restoring;
            Ok(())
        } else {
            Err(format!("Snapshot {} not found", id))
        }
    }

    pub fn delete_snapshot(&mut self, id: String) -> Result<(), String> {
        if let Some(idx) = self.snapshots.iter().position(|s| s.id == id) {
            self.snapshots.remove(idx);
            Ok(())
        } else {
            Err(format!("Snapshot {} not found", id))
        }
    }

    pub fn get_snapshots_by_type(&self, snapshot_type: SnapshotType) -> Vec<&Snapshot> {
        self.snapshots.iter().filter(|s| s.snapshot_type == snapshot_type).collect()
    }

    pub fn get_total_snapshot_size(&self) -> u64 {
        self.snapshots.iter().map(|s| s.size_bytes).sum()
    }
}

impl Default for SigmaSnapshotManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_manager_creation() {
        let manager = SigmaSnapshotManager::new();
        assert_eq!(manager.get_snapshots().len(), 0);
    }

    #[test]
    fn test_create_snapshot() {
        let mut manager = SigmaSnapshotManager::new();
        manager.create_snapshot("test".to_string(), SnapshotType::Filesystem, "/".to_string());
        assert_eq!(manager.get_snapshots().len(), 1);
    }

    #[test]
    fn test_complete_snapshot() {
        let mut manager = SigmaSnapshotManager::new();
        let snapshot = manager.create_snapshot("test".to_string(), SnapshotType::Filesystem, "/".to_string());
        manager.complete_snapshot(snapshot.id, 1000).unwrap();
        assert_eq!(manager.get_snapshots()[0].state, SnapshotState::Available);
        assert_eq!(manager.get_snapshots()[0].size_bytes, 1000);
    }

    #[test]
    fn test_restore_snapshot() {
        let mut manager = SigmaSnapshotManager::new();
        let snapshot = manager.create_snapshot("test".to_string(), SnapshotType::Filesystem, "/".to_string());
        manager.complete_snapshot(snapshot.id.clone(), 1000).unwrap();
        manager.restore_snapshot(snapshot.id).unwrap();
        assert_eq!(manager.get_snapshots()[0].state, SnapshotState::Restoring);
    }

    #[test]
    fn test_delete_snapshot() {
        let mut manager = SigmaSnapshotManager::new();
        let snapshot = manager.create_snapshot("test".to_string(), SnapshotType::Filesystem, "/".to_string());
        manager.delete_snapshot(snapshot.id).unwrap();
        assert_eq!(manager.get_snapshots().len(), 0);
    }

    #[test]
    fn test_snapshots_by_type() {
        let mut manager = SigmaSnapshotManager::new();
        manager.create_snapshot("fs".to_string(), SnapshotType::Filesystem, "/".to_string());
        manager.create_snapshot("vol".to_string(), SnapshotType::Volume, "/dev/sda1".to_string());
        assert_eq!(manager.get_snapshots_by_type(SnapshotType::Filesystem).len(), 1);
    }

    #[test]
    fn test_total_snapshot_size() {
        let mut manager = SigmaSnapshotManager::new();
        let snapshot1 = manager.create_snapshot("test1".to_string(), SnapshotType::Filesystem, "/".to_string());
        let snapshot2 = manager.create_snapshot("test2".to_string(), SnapshotType::Filesystem, "/".to_string());
        manager.complete_snapshot(snapshot1.id, 1000).unwrap();
        manager.complete_snapshot(snapshot2.id, 2000).unwrap();
        assert_eq!(manager.get_total_snapshot_size(), 3000);
    }
}
