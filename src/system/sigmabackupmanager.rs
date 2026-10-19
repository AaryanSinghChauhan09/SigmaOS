//! SigmaBackupManager - Backup Management System
//!
//! Provides comprehensive backup management inspired by Linux Mint and Omarchy.
//! Supports system backups, file backups, scheduled backups, and backup restoration.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Backup type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupType {
    Full,
    Incremental,
    Differential,
}

/// Backup status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Restoring,
}

/// Compression type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionType {
    None,
    Gzip,
    Bzip2,
    Xz,
    Zstd,
}

/// Backup job
#[derive(Debug, Clone)]
pub struct BackupJob {
    pub id: String,
    pub name: String,
    pub source_path: String,
    pub destination_path: String,
    pub backup_type: BackupType,
    pub compression: CompressionType,
    pub status: BackupStatus,
    pub size: u64, // in bytes
    pub compressed_size: u64,
    pub created_at: String,
    pub completed_at: Option<String>,
    pub file_count: u32,
}

impl BackupJob {
    pub fn new(
        id: String,
        name: String,
        source_path: String,
        destination_path: String,
        backup_type: BackupType,
        compression: CompressionType,
    ) -> Self {
        Self {
            id,
            name,
            source_path,
            destination_path,
            backup_type,
            compression,
            status: BackupStatus::Pending,
            size: 0,
            compressed_size: 0,
            created_at: String::new(),
            completed_at: None,
            file_count: 0,
        }
    }

    pub fn set_status(&mut self, status: BackupStatus) {
        self.status = status;
    }

    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }

    pub fn set_compressed_size(&mut self, size: u64) {
        self.compressed_size = size;
    }

    pub fn set_created_at(&mut self, time: String) {
        self.created_at = time;
    }

    pub fn set_completed_at(&mut self, time: Option<String>) {
        self.completed_at = time;
    }

    pub fn set_file_count(&mut self, count: u32) {
        self.file_count = count;
    }

    pub fn compression_ratio(&self) -> f32 {
        if self.size > 0 {
            (self.compressed_size as f32) / (self.size as f32)
        } else {
            1.0
        }
    }

    pub fn is_complete(&self) -> bool {
        self.status == BackupStatus::Completed
    }
}

/// Backup schedule
#[derive(Debug, Clone)]
pub struct BackupSchedule {
    pub id: String,
    pub name: String,
    pub source_path: String,
    pub destination_path: String,
    pub backup_type: BackupType,
    pub compression: CompressionType,
    pub schedule_type: String, // daily, weekly, monthly
    pub day_of_week: Option<String>,
    pub time: String,
    pub retention_count: u32, // number of backups to keep
    pub enabled: bool,
}

impl BackupSchedule {
    pub fn new(
        id: String,
        name: String,
        source_path: String,
        destination_path: String,
        backup_type: BackupType,
        compression: CompressionType,
        schedule_type: String,
        time: String,
        retention_count: u32,
    ) -> Self {
        Self {
            id,
            name,
            source_path,
            destination_path,
            backup_type,
            compression,
            schedule_type,
            day_of_week: None,
            time,
            retention_count,
            enabled: true,
        }
    }

    pub fn set_day_of_week(&mut self, day: Option<String>) {
        self.day_of_week = day;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

/// SigmaBackupManager - Backup management system
#[derive(Debug)]
pub struct SigmaBackupManager {
    backups: BTreeMap<String, BackupJob>,
    schedules: BTreeMap<String, BackupSchedule>,
}

impl SigmaBackupManager {
    pub fn new() -> Self {
        Self {
            backups: BTreeMap::new(),
            schedules: BTreeMap::new(),
        }
    }

    pub fn add_backup(&mut self, backup: BackupJob) {
        self.backups.insert(backup.id.clone(), backup);
    }

    pub fn remove_backup(&mut self, id: &str) -> bool {
        self.backups.remove(id).is_some()
    }

    pub fn get_backup(&self, id: &str) -> Option<&BackupJob> {
        self.backups.get(id)
    }

    pub fn get_backup_mut(&mut self, id: &str) -> Option<&mut BackupJob> {
        self.backups.get_mut(id)
    }

    pub fn add_schedule(&mut self, schedule: BackupSchedule) {
        self.schedules.insert(schedule.id.clone(), schedule);
    }

    pub fn remove_schedule(&mut self, id: &str) -> bool {
        self.schedules.remove(id).is_some()
    }

    pub fn get_schedule(&self, id: &str) -> Option<&BackupSchedule> {
        self.schedules.get(id)
    }

    pub fn list_by_type(&self, backup_type: BackupType) -> Vec<&BackupJob> {
        self.backups
            .values()
            .filter(|backup| backup.backup_type == backup_type)
            .collect()
    }

    pub fn list_completed(&self) -> Vec<&BackupJob> {
        self.backups
            .values()
            .filter(|backup| backup.is_complete())
            .collect()
    }

    pub fn list_failed(&self) -> Vec<&BackupJob> {
        self.backups
            .values()
            .filter(|backup| backup.status == BackupStatus::Failed)
            .collect()
    }

    pub fn total_backup_size(&self) -> u64 {
        self.backups.values().map(|backup| backup.size).sum()
    }

    pub fn total_compressed_size(&self) -> u64 {
        self.backups
            .values()
            .map(|backup| backup.compressed_size)
            .sum()
    }

    pub fn backup_count(&self) -> usize {
        self.backups.len()
    }

    pub fn schedule_count(&self) -> usize {
        self.schedules.len()
    }
}

impl Default for SigmaBackupManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaBackupManager instance
static mut SIGMABACKUPMANAGER_INSTANCE: Option<SigmaBackupManager> = None;

/// Initialize the global SigmaBackupManager instance
pub fn init_sigmabackupmanager() {
    unsafe {
        SIGMABACKUPMANAGER_INSTANCE = Some(SigmaBackupManager::new());
    }
}

/// Get the global SigmaBackupManager instance
pub fn get_sigmabackupmanager() -> Option<&'static SigmaBackupManager> {
    unsafe { SIGMABACKUPMANAGER_INSTANCE.as_ref() }
}

/// Get the global SigmaBackupManager instance (mutable)
pub fn get_sigmabackupmanager_mut() -> Option<&'static mut SigmaBackupManager> {
    unsafe { SIGMABACKUPMANAGER_INSTANCE.as_mut() }
}

/// Convenience function to add a backup
pub fn add_backup(backup: BackupJob) -> bool {
    if let Some(manager) = get_sigmabackupmanager_mut() {
        manager.add_backup(backup);
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_job_creation() {
        let backup = BackupJob::new(
            "backup1".to_string(),
            "System Backup".to_string(),
            "/home".to_string(),
            "/backup".to_string(),
            BackupType::Full,
            CompressionType::Gzip,
        );
        assert_eq!(backup.id, "backup1");
        assert_eq!(backup.status, BackupStatus::Pending);
    }

    #[test]
    fn test_backup_job_modification() {
        let mut backup = BackupJob::new(
            "backup1".to_string(),
            "Test".to_string(),
            "/src".to_string(),
            "/dst".to_string(),
            BackupType::Incremental,
            CompressionType::Xz,
        );
        backup.set_status(BackupStatus::Completed);
        backup.set_size(1000000);
        backup.set_compressed_size(500000);
        backup.set_file_count(100);
        assert!(backup.is_complete());
        assert_eq!(backup.compression_ratio(), 0.5);
    }

    #[test]
    fn test_backup_schedule_creation() {
        let schedule = BackupSchedule::new(
            "sched1".to_string(),
            "Daily Backup".to_string(),
            "/home".to_string(),
            "/backup".to_string(),
            BackupType::Incremental,
            CompressionType::Gzip,
            "daily".to_string(),
            "02:00".to_string(),
            7,
        );
        assert_eq!(schedule.id, "sched1");
        assert_eq!(schedule.retention_count, 7);
    }

    #[test]
    fn test_sigmabackupmanager_creation() {
        let manager = SigmaBackupManager::new();
        assert_eq!(manager.backup_count(), 0);
        assert_eq!(manager.schedule_count(), 0);
    }

    #[test]
    fn test_sigmabackupmanager_management() {
        let mut manager = SigmaBackupManager::new();
        let backup = BackupJob::new(
            "backup1".to_string(),
            "Test".to_string(),
            "/src".to_string(),
            "/dst".to_string(),
            BackupType::Full,
            CompressionType::None,
        );
        manager.add_backup(backup);
        assert_eq!(manager.backup_count(), 1);
    }
}
