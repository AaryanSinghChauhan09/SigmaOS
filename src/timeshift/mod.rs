// Timeshift System Restore Tool
// Inspired by Linux Mint Timeshift
// Provides system snapshots with RSYNC+hardlinks or BTRFS snapshot support

use std::collections::HashMap;
use std::path::PathBuf;

/// Snapshot mode
#[derive(Debug, Clone, PartialEq)]
pub enum SnapshotMode {
    Rsync,
    Btrfs,
    Custom,
}

/// Snapshot level for retention policy
#[derive(Debug, Clone, PartialEq)]
pub enum SnapshotLevel {
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

/// Snapshot with metadata
#[derive(Debug, Clone)]
pub struct Snapshot {
    pub id: String,
    pub timestamp: i64,
    pub mode: SnapshotMode,
    pub level: SnapshotLevel,
    pub size_bytes: u64,
    pub path: PathBuf,
    pub is_boot_snapshot: bool,
    pub tags: Vec<String>,
}

impl Snapshot {
    pub fn new(id: String, mode: SnapshotMode, level: SnapshotLevel, path: PathBuf) -> Self {
        Self {
            id,
            timestamp: chrono::Utc::now().timestamp(),
            mode,
            level,
            size_bytes: 0,
            path,
            is_boot_snapshot: false,
            tags: Vec::new(),
        }
    }

    pub fn with_size(mut self, size: u64) -> Self {
        self.size_bytes = size;
        self
    }

    pub fn with_boot_snapshot(mut self, is_boot: bool) -> Self {
        self.is_boot_snapshot = is_boot;
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn add_tag(&mut self, tag: String) {
        self.tags.push(tag);
    }

    /// Format size in human-readable format
    pub fn format_size(&self) -> String {
        let bytes = self.size_bytes;
        if bytes < 1024 {
            format!("{} B", bytes)
        } else if bytes < 1024 * 1024 {
            format!("{:.1} KB", bytes as f64 / 1024.0)
        } else if bytes < 1024 * 1024 * 1024 {
            format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
        } else if bytes < 1024 * 1024 * 1024 * 1024 {
            format!("{:.1} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
        } else {
            format!("{:.1} TB", bytes as f64 / (1024.0 * 1024.0 * 1024.0 * 1024.0))
        }
    }

    /// Format timestamp as human-readable date
    pub fn format_timestamp(&self) -> String {
        let dt = chrono::DateTime::from_timestamp(self.timestamp, 0).unwrap();
        dt.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

/// Snapshot schedule configuration
#[derive(Debug, Clone)]
pub struct SnapshotSchedule {
    pub mode: SnapshotMode,
    pub enabled: bool,
    pub hourly_keep: u32,
    pub daily_keep: u32,
    pub weekly_keep: u32,
    pub monthly_keep: u32,
    pub yearly_keep: u32,
    pub include_home: bool,
    pub include_root: bool,
    pub include_hidden_only: bool,
}

impl SnapshotSchedule {
    pub fn new(mode: SnapshotMode) -> Self {
        Self {
            mode,
            enabled: true,
            hourly_keep: 0,
            daily_keep: 5,
            weekly_keep: 3,
            monthly_keep: 2,
            yearly_keep: 1,
            include_home: false,
            include_root: false,
            include_hidden_only: false,
        }
    }

    pub fn with_hourly_keep(mut self, keep: u32) -> Self {
        self.hourly_keep = keep;
        self
    }

    pub fn with_daily_keep(mut self, keep: u32) -> Self {
        self.daily_keep = keep;
        self
    }

    pub fn with_weekly_keep(mut self, keep: u32) -> Self {
        self.weekly_keep = keep;
        self
    }

    pub fn with_monthly_keep(mut self, keep: u32) -> Self {
        self.monthly_keep = keep;
        self
    }

    pub fn with_yearly_keep(mut self, keep: u32) -> Self {
        self.yearly_keep = keep;
        self
    }

    pub fn with_home_inclusion(mut self, include: bool, hidden_only: bool) -> Self {
        self.include_home = include;
        self.include_hidden_only = hidden_only;
        self
    }

    pub fn with_root_inclusion(mut self, include: bool) -> Self {
        self.include_root = include;
        self
    }
}

/// Exclude pattern for snapshot
#[derive(Debug, Clone)]
pub struct ExcludePattern {
    pub pattern: String,
    pub is_regex: bool,
}

impl ExcludePattern {
    pub fn new(pattern: String, is_regex: bool) -> Self {
        Self { pattern, is_regex }
    }
}

/// Timeshift backup manager
#[derive(Debug, Clone)]
pub struct TimeshiftManager {
    snapshots: Vec<Snapshot>,
    schedule: SnapshotSchedule,
    snapshot_path: PathBuf,
    exclude_patterns: Vec<ExcludePattern>,
}

impl TimeshiftManager {
    pub fn new(snapshot_path: PathBuf, mode: SnapshotMode) -> Self {
        Self {
            snapshots: Vec::new(),
            schedule: SnapshotSchedule::new(mode),
            snapshot_path,
            exclude_patterns: Vec::new(),
        }
    }

    /// Set snapshot schedule
    pub fn set_schedule(&mut self, schedule: SnapshotSchedule) {
        self.schedule = schedule;
    }

    /// Get schedule
    pub fn get_schedule(&self) -> &SnapshotSchedule {
        &self.schedule
    }

    /// Create a new snapshot
    pub fn create_snapshot(&mut self, level: SnapshotLevel, is_boot: bool) -> Result<Snapshot, String> {
        let id = self.generate_snapshot_id();
        let snapshot_path = self.snapshot_path.join(&id);
        
        let mut snapshot = Snapshot::new(id.clone(), self.schedule.mode.clone(), level, snapshot_path)
            .with_boot_snapshot(is_boot);
        
        // In a real implementation, this would run rsync or BTRFS snapshot
        // For now, we'll simulate it
        snapshot = snapshot.with_size(1024 * 1024 * 100); // 100 MB
        
        self.snapshots.push(snapshot.clone());
        Ok(snapshot)
    }

    /// Get all snapshots
    pub fn get_snapshots(&self) -> &[Snapshot] {
        &self.snapshots
    }

    /// Get snapshots by level
    pub fn get_snapshots_by_level(&self, level: SnapshotLevel) -> Vec<&Snapshot> {
        self.snapshots
            .iter()
            .filter(|s| s.level == level)
            .collect()
    }

    /// Delete a snapshot
    pub fn delete_snapshot(&mut self, id: &str) -> Result<(), String> {
        let index = self.snapshots.iter().position(|s| s.id == id)
            .ok_or_else(|| format!("Snapshot {} not found", id))?;
        
        self.snapshots.remove(index);
        Ok(())
    }

    /// Restore a snapshot
    pub fn restore_snapshot(&self, id: &str) -> Result<String, String> {
        let snapshot = self.snapshots.iter()
            .find(|s| s.id == id)
            .ok_or_else(|| format!("Snapshot {} not found", id))?;
        
        // In a real implementation, this would restore from the snapshot
        Ok(format!("Restored snapshot from {}", snapshot.format_timestamp()))
    }

    /// Add exclude pattern
    pub fn add_exclude_pattern(&mut self, pattern: ExcludePattern) {
        self.exclude_patterns.push(pattern);
    }

    /// Get exclude patterns
    pub fn get_exclude_patterns(&self) -> &[ExcludePattern] {
        &self.exclude_patterns
    }

    /// Prune old snapshots based on retention policy
    pub fn prune_snapshots(&mut self) -> usize {
        let mut pruned = 0;
        
        let keep_counts = [
            (SnapshotLevel::Hourly, self.schedule.hourly_keep),
            (SnapshotLevel::Daily, self.schedule.daily_keep),
            (SnapshotLevel::Weekly, self.schedule.weekly_keep),
            (SnapshotLevel::Monthly, self.schedule.monthly_keep),
            (SnapshotLevel::Yearly, self.schedule.yearly_keep),
        ];
        
        for (level, keep) in keep_counts {
            if keep == 0 {
                continue;
            }
            
            let level_snapshots: Vec<_> = self.snapshots
                .iter()
                .filter(|s| s.level == level)
                .collect();
            
            if level_snapshots.len() > keep as usize {
                let to_remove = level_snapshots.len() - keep as usize;
                let mut indices: Vec<_> = level_snapshots
                    .iter()
                    .enumerate()
                    .filter(|(_, s)| !s.is_boot_snapshot)
                    .map(|(i, _)| i)
                    .collect();
                
                // Sort by timestamp to remove oldest first
                indices.sort_by(|a, b| {
                    let snap_a = &self.snapshots[*a];
                    let snap_b = &self.snapshots[*b];
                    snap_a.timestamp.cmp(&snap_b.timestamp)
                });
                
                // Remove oldest first
                for index in indices.iter().take(to_remove) {
                    if !self.snapshots[*index].is_boot_snapshot {
                        self.snapshots.remove(*index);
                        pruned += 1;
                    }
                }
            }
        }
        
        pruned
    }

    /// Get statistics
    pub fn get_statistics(&self) -> TimeshiftStatistics {
        let total_size: u64 = self.snapshots.iter().map(|s| s.size_bytes).sum();
        let hourly = self.snapshots.iter().filter(|s| s.level == SnapshotLevel::Hourly).count();
        let daily = self.snapshots.iter().filter(|s| s.level == SnapshotLevel::Daily).count();
        let weekly = self.snapshots.iter().filter(|s| s.level == SnapshotLevel::Weekly).count();
        let monthly = self.snapshots.iter().filter(|s| s.level == SnapshotLevel::Monthly).count();
        let yearly = self.snapshots.iter().filter(|s| s.level == SnapshotLevel::Yearly).count();
        let boot = self.snapshots.iter().filter(|s| s.is_boot_snapshot).count();

        TimeshiftStatistics {
            total_snapshots: self.snapshots.len(),
            total_size,
            hourly,
            daily,
            weekly,
            monthly,
            yearly,
            boot,
        }
    }

    fn generate_snapshot_id(&self) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        format!("{}", timestamp)
    }
}

impl Default for TimeshiftManager {
    fn default() -> Self {
        Self::new(PathBuf::from("/timeshift"), SnapshotMode::Rsync)
    }
}

/// Timeshift statistics
#[derive(Debug, Clone, PartialEq)]
pub struct TimeshiftStatistics {
    pub total_snapshots: usize,
    pub total_size: u64,
    pub hourly: usize,
    pub daily: usize,
    pub weekly: usize,
    pub monthly: usize,
    pub yearly: usize,
    pub boot: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_creation() {
        let manager = TimeshiftManager::new(PathBuf::from("/timeshift"), SnapshotMode::Rsync);
        assert_eq!(manager.get_snapshots().len(), 0);
    }

    #[test]
    fn test_create_snapshot() {
        let mut manager = TimeshiftManager::new(PathBuf::from("/timeshift"), SnapshotMode::Rsync);
        let snapshot = manager.create_snapshot(SnapshotLevel::Daily, false).unwrap();
        assert_eq!(manager.get_snapshots().len(), 1);
        assert_eq!(snapshot.level, SnapshotLevel::Daily);
    }

    #[test]
    fn test_delete_snapshot() {
        let mut manager = TimeshiftManager::new(PathBuf::from("/timeshift"), SnapshotMode::Rsync);
        let snapshot = manager.create_snapshot(SnapshotLevel::Daily, false).unwrap();
        let id = snapshot.id.clone();
        manager.delete_snapshot(&id).unwrap();
        assert_eq!(manager.get_snapshots().len(), 0);
    }

    #[test]
    fn test_get_snapshots_by_level() {
        let mut manager = TimeshiftManager::new(PathBuf::from("/timeshift"), SnapshotMode::Rsync);
        manager.create_snapshot(SnapshotLevel::Daily, false).unwrap();
        manager.create_snapshot(SnapshotLevel::Hourly, false).unwrap();
        manager.create_snapshot(SnapshotLevel::Daily, false).unwrap();
        
        let daily = manager.get_snapshots_by_level(SnapshotLevel::Daily);
        assert_eq!(daily.len(), 2);
    }

    #[test]
    fn test_snapshot_schedule() {
        let schedule = SnapshotSchedule::new(SnapshotMode::Rsync)
            .with_daily_keep(10)
            .with_weekly_keep(5);
        
        assert_eq!(schedule.daily_keep, 10);
        assert_eq!(schedule.weekly_keep, 5);
    }

    #[test]
    fn test_exclude_patterns() {
        let mut manager = TimeshiftManager::new(PathBuf::from("/timeshift"), SnapshotMode::Rsync);
        manager.add_exclude_pattern(ExcludePattern::new("/home/user/Downloads".to_string(), false));
        manager.add_exclude_pattern(ExcludePattern::new(".*\\.log".to_string(), true));
        
        assert_eq!(manager.get_exclude_patterns().len(), 2);
    }

    #[test]
    fn test_prune_snapshots() {
        let mut manager = TimeshiftManager::new(PathBuf::from("/timeshift"), SnapshotMode::Rsync);
        manager.set_schedule(SnapshotSchedule::new(SnapshotMode::Rsync).with_daily_keep(2));
        
        // Create 5 daily snapshots
        for _ in 0..5 {
            manager.create_snapshot(SnapshotLevel::Daily, false).unwrap();
        }
        
        let pruned = manager.prune_snapshots();
        assert!(pruned > 0);
        assert!(manager.get_snapshots_by_level(SnapshotLevel::Daily).len() <= 2);
    }

    #[test]
    fn test_statistics() {
        let mut manager = TimeshiftManager::new(PathBuf::from("/timeshift"), SnapshotMode::Rsync);
        manager.create_snapshot(SnapshotLevel::Daily, false).unwrap();
        manager.create_snapshot(SnapshotLevel::Hourly, false).unwrap();
        manager.create_snapshot(SnapshotLevel::Weekly, false).unwrap();
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_snapshots, 3);
        assert_eq!(stats.daily, 1);
        assert_eq!(stats.hourly, 1);
        assert_eq!(stats.weekly, 1);
    }

    #[test]
    fn test_boot_snapshot() {
        let mut manager = TimeshiftManager::new(PathBuf::from("/timeshift"), SnapshotMode::Rsync);
        let snapshot = manager.create_snapshot(SnapshotLevel::Hourly, true).unwrap();
        assert!(snapshot.is_boot_snapshot);
        
        let stats = manager.get_statistics();
        assert_eq!(stats.boot, 1);
    }
}
