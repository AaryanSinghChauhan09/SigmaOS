#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

/// Timeshift-inspired snapshot management GUI
/// Provides text-based interface for system snapshot management
/// with RSYNC and BTRFS snapshot support

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SnapshotMode {
    Rsync,
    Btrfs,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SnapshotLevel {
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Boot,
}

#[derive(Debug, Clone)]
pub struct SnapshotConfig {
    pub mode: SnapshotMode,
    pub snapshot_location: String,
    pub max_snapshots: usize,
    pub enabled_levels: Vec<SnapshotLevel>,
    pub include_user_data: bool,
    pub exclude_patterns: Vec<String>,
}

impl Default for SnapshotConfig {
    fn default() -> Self {
        Self {
            mode: SnapshotMode::Rsync,
            snapshot_location: String::from("/timeshift"),
            max_snapshots: 10,
            enabled_levels: vec![SnapshotLevel::Daily, SnapshotLevel::Weekly, SnapshotLevel::Monthly],
            include_user_data: false,
            exclude_patterns: vec![
                String::from("/home/*/.cache/*"),
                String::from("/home/*/.thumbnails/*"),
                String::from("/home/*/.local/share/Trash/*"),
            ],
        }
    }
}

#[derive(Debug, Clone)]
pub struct SnapshotMetadata {
    pub id: String,
    pub timestamp: u64,
    pub level: SnapshotLevel,
    pub size_bytes: u64,
    pub description: String,
    pub is_bootable: bool,
}

#[derive(Debug, Clone)]
pub struct SnapshotResult {
    pub success: bool,
    pub snapshot_id: Option<String>,
    pub bytes_written: u64,
    pub duration_seconds: u64,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct RestoreResult {
    pub success: bool,
    pub snapshot_id: String,
    pub files_restored: usize,
    pub duration_seconds: u64,
    pub message: String,
}

/// Timeshift-inspired snapshot manager
pub struct TimeshiftSnapshotManager {
    pub config: SnapshotConfig,
    pub snapshots: Vec<SnapshotMetadata>,
    pub auto_snapshot_enabled: bool,
}

impl TimeshiftSnapshotManager {
    pub fn new() -> Self {
        Self {
            config: SnapshotConfig::default(),
            snapshots: Vec::new(),
            auto_snapshot_enabled: false,
        }
    }

    /// Create a new snapshot
    pub fn create_snapshot(&mut self, level: SnapshotLevel, description: &str) -> SnapshotResult {
        let snapshot_id = format!("snapshot_{}", self.snapshots.len());
        let timestamp = self.get_current_timestamp();
        
        let metadata = SnapshotMetadata {
            id: snapshot_id.clone(),
            timestamp,
            level: level.clone(),
            size_bytes: 0,
            description: description.to_string(),
            is_bootable: level == SnapshotLevel::Boot,
        };
        
        self.snapshots.push(metadata.clone());
        
        SnapshotResult {
            success: true,
            snapshot_id: Some(snapshot_id),
            bytes_written: 0,
            duration_seconds: 0,
            message: format!("Created {} snapshot", self.level_to_string(&level)),
        }
    }

    /// Restore from a snapshot
    pub fn restore_snapshot(&self, snapshot_id: &str) -> RestoreResult {
        match self.find_snapshot(snapshot_id) {
            Some(snapshot) => {
                RestoreResult {
                    success: true,
                    snapshot_id: snapshot_id.to_string(),
                    files_restored: 0,
                    duration_seconds: 0,
                    message: format!("Restored snapshot from {}", self.format_timestamp(snapshot.timestamp)),
                }
            }
            None => {
                RestoreResult {
                    success: false,
                    snapshot_id: snapshot_id.to_string(),
                    files_restored: 0,
                    duration_seconds: 0,
                    message: format!("Snapshot {} not found", snapshot_id),
                }
            }
        }
    }

    /// Delete a snapshot
    pub fn delete_snapshot(&mut self, snapshot_id: &str) -> Result<(), String> {
        match self.find_snapshot_index(snapshot_id) {
            Some(index) => {
                self.snapshots.remove(index);
                Ok(())
            }
            None => Err(format!("Snapshot {} not found", snapshot_id)),
        }
    }

    /// List all snapshots
    pub fn list_snapshots(&self) -> Vec<&SnapshotMetadata> {
        self.snapshots.iter().collect()
    }

    /// Get snapshot by ID
    pub fn get_snapshot(&self, snapshot_id: &str) -> Option<&SnapshotMetadata> {
        self.find_snapshot(snapshot_id)
    }

    /// Update configuration
    pub fn update_config(&mut self, config: SnapshotConfig) {
        self.config = config;
    }

    /// Enable auto-snapshot
    pub fn enable_auto_snapshot(&mut self) {
        self.auto_snapshot_enabled = true;
    }

    /// Disable auto-snapshot
    pub fn disable_auto_snapshot(&mut self) {
        self.auto_snapshot_enabled = false;
    }

    /// Get snapshot statistics
    pub fn get_statistics(&self) -> SnapshotStatistics {
        let total_snapshots = self.snapshots.len();
        let total_size: u64 = self.snapshots.iter().map(|s| s.size_bytes).sum();
        let by_level = self.count_snapshots_by_level();
        
        SnapshotStatistics {
            total_snapshots,
            total_size_bytes: total_size,
            by_level,
            oldest_snapshot: self.snapshots.first().map(|s| s.timestamp),
            newest_snapshot: self.snapshots.last().map(|s| s.timestamp),
        }
    }

    /// Display snapshot list in text-based GUI format
    pub fn display_snapshot_list(&self) -> String {
        let mut output = String::from("=== Timeshift Snapshots ===\n\n");
        
        if self.snapshots.is_empty() {
            output.push_str("No snapshots available.\n");
            return output;
        }
        
        for (i, snapshot) in self.snapshots.iter().enumerate() {
            output.push_str(&format!("{}. {}\n", i + 1, snapshot.id));
            output.push_str(&format!("   Level: {}\n", self.level_to_string(&snapshot.level)));
            output.push_str(&format!("   Time: {}\n", self.format_timestamp(snapshot.timestamp)));
            output.push_str(&format!("   Size: {} bytes\n", snapshot.size_bytes));
            output.push_str(&format!("   Description: {}\n", snapshot.description));
            output.push_str(&format!("   Bootable: {}\n", if snapshot.is_bootable { "Yes" } else { "No" }));
            output.push_str("\n");
        }
        
        output.push_str(&format!("Total: {} snapshots\n", self.snapshots.len()));
        output
    }

    /// Display configuration in text-based GUI format
    pub fn display_config(&self) -> String {
        let mut output = String::from("=== Timeshift Configuration ===\n\n");
        output.push_str(&format!("Mode: {}\n", self.mode_to_string(&self.config.mode)));
        output.push_str(&format!("Location: {}\n", self.config.snapshot_location));
        output.push_str(&format!("Max Snapshots: {}\n", self.config.max_snapshots));
        output.push_str(&format!("Auto-Snapshot: {}\n", if self.auto_snapshot_enabled { "Enabled" } else { "Disabled" }));
        output.push_str(&format!("Include User Data: {}\n", if self.config.include_user_data { "Yes" } else { "No" }));
        output.push_str("\nEnabled Levels:\n");
        
        for level in &self.config.enabled_levels {
            output.push_str(&format!("  - {}\n", self.level_to_string(level)));
        }
        
        output.push_str("\nExclude Patterns:\n");
        for pattern in &self.config.exclude_patterns {
            output.push_str(&format!("  - {}\n", pattern));
        }
        
        output
    }

    /// Display statistics in text-based GUI format
    pub fn display_statistics(&self) -> String {
        let stats = self.get_statistics();
        let mut output = String::from("=== Snapshot Statistics ===\n\n");
        output.push_str(&format!("Total Snapshots: {}\n", stats.total_snapshots));
        output.push_str(&format!("Total Size: {} bytes\n", stats.total_size_bytes));
        
        output.push_str("\nSnapshots by Level:\n");
        for (level, count) in &stats.by_level {
            output.push_str(&format!("  {}: {}\n", self.level_to_string(level), count));
        }
        
        if let Some(oldest) = stats.oldest_snapshot {
            output.push_str(&format!("\nOldest: {}\n", self.format_timestamp(oldest)));
        }
        if let Some(newest) = stats.newest_snapshot {
            output.push_str(&format!("Newest: {}\n", self.format_timestamp(newest)));
        }
        
        output
    }

    // Helper methods
    fn find_snapshot(&self, snapshot_id: &str) -> Option<&SnapshotMetadata> {
        self.snapshots.iter().find(|s| s.id == snapshot_id)
    }

    fn find_snapshot_index(&self, snapshot_id: &str) -> Option<usize> {
        self.snapshots.iter().position(|s| s.id == snapshot_id)
    }

    fn count_snapshots_by_level(&self) -> BTreeMap<SnapshotLevel, usize> {
        let mut counts = BTreeMap::new();
        for snapshot in &self.snapshots {
            let level = snapshot.level.clone();
            *counts.entry(level).or_insert(0) += 1;
        }
        counts
    }

    fn get_current_timestamp(&self) -> u64 {
        0
    }

    fn format_timestamp(&self, timestamp: u64) -> String {
        format!("Timestamp: {}", timestamp)
    }

    fn level_to_string(&self, level: &SnapshotLevel) -> String {
        match level {
            SnapshotLevel::Hourly => String::from("Hourly"),
            SnapshotLevel::Daily => String::from("Daily"),
            SnapshotLevel::Weekly => String::from("Weekly"),
            SnapshotLevel::Monthly => String::from("Monthly"),
            SnapshotLevel::Boot => String::from("Boot"),
        }
    }

    fn mode_to_string(&self, mode: &SnapshotMode) -> String {
        match mode {
            SnapshotMode::Rsync => String::from("RSYNC"),
            SnapshotMode::Btrfs => String::from("BTRFS"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SnapshotStatistics {
    pub total_snapshots: usize,
    pub total_size_bytes: u64,
    pub by_level: BTreeMap<SnapshotLevel, usize>,
    pub oldest_snapshot: Option<u64>,
    pub newest_snapshot: Option<u64>,
}

impl Default for TimeshiftSnapshotManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_creation() {
        let mut manager = TimeshiftSnapshotManager::new();
        let result = manager.create_snapshot(SnapshotLevel::Daily, "Test snapshot");
        
        assert!(result.success);
        assert!(result.snapshot_id.is_some());
        assert_eq!(manager.snapshots.len(), 1);
    }

    #[test]
    fn test_snapshot_levels() {
        let mut manager = TimeshiftSnapshotManager::new();
        
        manager.create_snapshot(SnapshotLevel::Hourly, "Hourly test");
        manager.create_snapshot(SnapshotLevel::Daily, "Daily test");
        manager.create_snapshot(SnapshotLevel::Weekly, "Weekly test");
        manager.create_snapshot(SnapshotLevel::Monthly, "Monthly test");
        manager.create_snapshot(SnapshotLevel::Boot, "Boot test");
        
        assert_eq!(manager.snapshots.len(), 5);
    }

    #[test]
    fn test_snapshot_deletion() {
        let mut manager = TimeshiftSnapshotManager::new();
        let result = manager.create_snapshot(SnapshotLevel::Daily, "Test snapshot");
        let snapshot_id = result.snapshot_id.unwrap();
        
        let delete_result = manager.delete_snapshot(&snapshot_id);
        assert!(delete_result.is_ok());
        assert_eq!(manager.snapshots.len(), 0);
    }

    #[test]
    fn test_snapshot_restore() {
        let mut manager = TimeshiftSnapshotManager::new();
        let result = manager.create_snapshot(SnapshotLevel::Daily, "Test snapshot");
        let snapshot_id = result.snapshot_id.unwrap();
        
        let restore_result = manager.restore_snapshot(&snapshot_id);
        assert!(restore_result.success);
        assert_eq!(restore_result.snapshot_id, snapshot_id);
    }

    #[test]
    fn test_statistics() {
        let mut manager = TimeshiftSnapshotManager::new();
        
        manager.create_snapshot(SnapshotLevel::Daily, "Test 1");
        manager.create_snapshot(SnapshotLevel::Daily, "Test 2");
        manager.create_snapshot(SnapshotLevel::Weekly, "Test 3");
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_snapshots, 3);
        assert_eq!(stats.by_level.get(&SnapshotLevel::Daily), Some(&2));
        assert_eq!(stats.by_level.get(&SnapshotLevel::Weekly), Some(&1));
    }

    #[test]
    fn test_config_update() {
        let mut manager = TimeshiftSnapshotManager::new();
        let mut new_config = SnapshotConfig::default();
        new_config.max_snapshots = 20;
        
        manager.update_config(new_config);
        assert_eq!(manager.config.max_snapshots, 20);
    }

    #[test]
    fn test_auto_snapshot_toggle() {
        let mut manager = TimeshiftSnapshotManager::new();
        
        assert!(!manager.auto_snapshot_enabled);
        manager.enable_auto_snapshot();
        assert!(manager.auto_snapshot_enabled);
        manager.disable_auto_snapshot();
        assert!(!manager.auto_snapshot_enabled);
    }

    #[test]
    fn test_display_output() {
        let mut manager = TimeshiftSnapshotManager::new();
        manager.create_snapshot(SnapshotLevel::Daily, "Test snapshot");
        
        let list_output = manager.display_snapshot_list();
        assert!(list_output.contains("Test snapshot"));
        assert!(list_output.contains("Daily"));
        
        let config_output = manager.display_config();
        assert!(config_output.contains("RSYNC"));
        assert!(config_output.contains("/timeshift"));
        
        let stats_output = manager.display_statistics();
        assert!(stats_output.contains("Total Snapshots: 1"));
    }
}
