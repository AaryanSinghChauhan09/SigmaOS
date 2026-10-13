// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaBackup - System Backup Daemon
//!
//! SigmaBackup provides system backup for SigmaOS, inspired by Borg/Duplicity from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Backup type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupType {
    /// Full backup
    Full,
    /// Incremental backup
    Incremental,
    /// Differential backup
    Differential,
}

impl fmt::Display for BackupType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackupType::Full => write!(f, "full"),
            BackupType::Incremental => write!(f, "incremental"),
            BackupType::Differential => write!(f, "differential"),
        }
    }
}

/// Backup status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupStatus {
    /// Pending
    Pending,
    /// Running
    Running,
    /// Completed
    Completed,
    /// Failed
    Failed,
}

impl fmt::Display for BackupStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackupStatus::Pending => write!(f, "pending"),
            BackupStatus::Running => write!(f, "running"),
            BackupStatus::Completed => write!(f, "completed"),
            BackupStatus::Failed => write!(f, "failed"),
        }
    }
}

/// Compression level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionLevel {
    /// No compression
    None,
    /// Fast compression
    Fast,
    /// Standard compression
    Standard,
    /// Maximum compression
    Maximum,
}

impl fmt::Display for CompressionLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompressionLevel::None => write!(f, "none"),
            CompressionLevel::Fast => write!(f, "fast"),
            CompressionLevel::Standard => write!(f, "standard"),
            CompressionLevel::Maximum => write!(f, "maximum"),
        }
    }
}

/// Backup job
#[derive(Debug, Clone)]
pub struct BackupJob {
    /// Job ID
    pub job_id: String,
    /// Source path
    pub source: String,
    /// Destination path
    pub destination: String,
    /// Backup type
    pub backup_type: BackupType,
    /// Backup status
    pub status: BackupStatus,
    /// Compression level
    pub compression: CompressionLevel,
    /// Retention count
    pub retention_count: u32,
    /// Size in MB
    pub size_mb: u64,
    /// Timestamp
    pub timestamp: u64,
}

impl BackupJob {
    /// Create a new backup job
    pub fn new(job_id: String, source: String, destination: String, backup_type: BackupType) -> Self {
        Self {
            job_id,
            source,
            destination,
            backup_type,
            status: BackupStatus::Pending,
            compression: CompressionLevel::Standard,
            retention_count: 7,
            size_mb: 0,
            timestamp: 0,
        }
    }

    /// Set compression level
    pub fn with_compression(mut self, compression: CompressionLevel) -> Self {
        self.compression = compression;
        self
    }

    /// Set retention count
    pub fn with_retention(mut self, retention: u32) -> Self {
        self.retention_count = retention;
        self
    }

    /// Set status
    pub fn with_status(mut self, status: BackupStatus) -> Self {
        self.status = status;
        self
    }
}

/// SigmaBackup - System Backup Daemon
pub struct SigmaBackup {
    /// Backup jobs (job_id -> job)
    jobs: BTreeMap<String, BackupJob>,
    /// Running state
    running: bool,
    /// Job counter
    job_counter: u32,
}

impl SigmaBackup {
    /// Create a new SigmaBackup instance
    pub fn new() -> Self {
        Self {
            jobs: BTreeMap::new(),
            running: false,
            job_counter: 0,
        }
    }

    /// Start the backup daemon
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the backup daemon
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the daemon is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Add a backup job
    pub fn add_job(&mut self, job: BackupJob) -> Result<(), String> {
        if self.jobs.contains_key(&job.job_id) {
            return Err(format!("Job '{}' already exists", job.job_id));
        }
        
        self.jobs.insert(job.job_id.clone(), job);
        self.job_counter += 1;
        Ok(())
    }

    /// Remove a backup job
    pub fn remove_job(&mut self, job_id: &str) -> Result<(), String> {
        if self.jobs.remove(job_id).is_none() {
            return Err(format!("Job '{}' not found", job_id));
        }
        Ok(())
    }

    /// Get a backup job
    pub fn get_job(&self, job_id: &str) -> Option<&BackupJob> {
        self.jobs.get(job_id)
    }

    /// Start a backup job
    pub fn start_job(&mut self, job_id: &str) -> Result<(), String> {
        match self.jobs.get_mut(job_id) {
            Some(job) => {
                if !self.running {
                    return Err("SigmaBackup not running".to_string());
                }
                job.status = BackupStatus::Running;
                Ok(())
            }
            None => Err(format!("Job '{}' not found", job_id)),
        }
    }

    /// Complete a backup job
    pub fn complete_job(&mut self, job_id: &str) -> Result<(), String> {
        match self.jobs.get_mut(job_id) {
            Some(job) => {
                job.status = BackupStatus::Completed;
                Ok(())
            }
            None => Err(format!("Job '{}' not found", job_id)),
        }
    }

    /// Fail a backup job
    pub fn fail_job(&mut self, job_id: &str) -> Result<(), String> {
        match self.jobs.get_mut(job_id) {
            Some(job) => {
                job.status = BackupStatus::Failed;
                Ok(())
            }
            None => Err(format!("Job '{}' not found", job_id)),
        }
    }

    /// List all jobs
    pub fn list_jobs(&self) -> Vec<&BackupJob> {
        self.jobs.values().collect()
    }

    /// List jobs by status
    pub fn list_by_status(&self, status: BackupStatus) -> Vec<&BackupJob> {
        self.jobs.values()
            .filter(|j| j.status == status)
            .collect()
    }

    /// List jobs by type
    pub fn list_by_type(&self, backup_type: BackupType) -> Vec<&BackupJob> {
        self.jobs.values()
            .filter(|j| j.backup_type == backup_type)
            .collect()
    }

    /// Get job count
    pub fn job_count(&self) -> usize {
        self.jobs.len()
    }

    /// Get backup statistics
    pub fn backup_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert("total_jobs".to_string(), self.job_count().to_string());
        stats.insert("completed_jobs".to_string(), self.list_by_status(BackupStatus::Completed).len().to_string());
        stats.insert("failed_jobs".to_string(), self.list_by_status(BackupStatus::Failed).len().to_string());
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
}

impl Default for SigmaBackup {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaBackup instance (placeholder for actual global state)
static mut SIGMABACKUP_INSTANCE: Option<SigmaBackup> = None;

/// Initialize the global SigmaBackup instance
pub fn init_sigmabackup() {
    unsafe {
        SIGMABACKUP_INSTANCE = Some(SigmaBackup::new());
    }
}

/// Get the global SigmaBackup instance
pub fn get_sigmabackup() -> Option<&'static mut SigmaBackup> {
    unsafe { SIGMABACKUP_INSTANCE.as_mut() }
}

/// Convenience function to start a backup job
pub fn start_backup_job(job_id: &str) -> Result<(), String> {
    if let Some(backup) = get_sigmabackup() {
        backup.start_job(job_id)
    } else {
        Err("SigmaBackup not initialized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_type_display() {
        assert_eq!(BackupType::Full.to_string(), "full");
        assert_eq!(BackupType::Incremental.to_string(), "incremental");
    }

    #[test]
    fn test_backup_status_display() {
        assert_eq!(BackupStatus::Pending.to_string(), "pending");
        assert_eq!(BackupStatus::Completed.to_string(), "completed");
    }

    #[test]
    fn test_compression_level_display() {
        assert_eq!(CompressionLevel::Fast.to_string(), "fast");
        assert_eq!(CompressionLevel::Maximum.to_string(), "maximum");
    }

    #[test]
    fn test_backup_job() {
        let job = BackupJob::new(
            "job1".to_string(),
            "/home".to_string(),
            "/backup".to_string(),
            BackupType::Full
        )
        .with_compression(CompressionLevel::Standard)
        .with_retention(7)
        .with_status(BackupStatus::Completed);

        assert_eq!(job.job_id, "job1");
        assert_eq!(job.compression, CompressionLevel::Standard);
        assert_eq!(job.retention_count, 7);
    }

    #[test]
    fn test_sigmabackup() {
        let mut backup = SigmaBackup::new();
        assert!(!backup.is_running());
        backup.start();
        assert!(backup.is_running());
        backup.stop();
        assert!(!backup.is_running());
    }

    #[test]
    fn test_sigmabackup_add_job() {
        let mut backup = SigmaBackup::new();
        
        let job = BackupJob::new(
            "job1".to_string(),
            "/home".to_string(),
            "/backup".to_string(),
            BackupType::Full
        );
        
        assert!(backup.add_job(job).is_ok());
        assert_eq!(backup.job_count(), 1);
    }

    #[test]
    fn test_sigmabackup_start_job() {
        let mut backup = SigmaBackup::new();
        backup.start();
        
        let job = BackupJob::new(
            "job1".to_string(),
            "/home".to_string(),
            "/backup".to_string(),
            BackupType::Full
        );
        
        backup.add_job(job).unwrap();
        assert!(backup.start_job("job1").is_ok());
        
        let job = backup.get_job("job1").unwrap();
        assert_eq!(job.status, BackupStatus::Running);
    }

    #[test]
    fn test_sigmabackup_complete_job() {
        let mut backup = SigmaBackup::new();
        
        let job = BackupJob::new(
            "job1".to_string(),
            "/home".to_string(),
            "/backup".to_string(),
            BackupType::Full
        );
        
        backup.add_job(job).unwrap();
        assert!(backup.complete_job("job1").is_ok());
        
        let job = backup.get_job("job1").unwrap();
        assert_eq!(job.status, BackupStatus::Completed);
    }

    #[test]
    fn test_sigmabackup_fail_job() {
        let mut backup = SigmaBackup::new();
        
        let job = BackupJob::new(
            "job1".to_string(),
            "/home".to_string(),
            "/backup".to_string(),
            BackupType::Full
        );
        
        backup.add_job(job).unwrap();
        assert!(backup.fail_job("job1").is_ok());
        
        let job = backup.get_job("job1").unwrap();
        assert_eq!(job.status, BackupStatus::Failed);
    }

    #[test]
    fn test_sigmabackup_list_by_status() {
        let mut backup = SigmaBackup::new();
        
        let job = BackupJob::new(
            "job1".to_string(),
            "/home".to_string(),
            "/backup".to_string(),
            BackupType::Full
        )
        .with_status(BackupStatus::Completed);
        
        backup.add_job(job).unwrap();
        
        let completed = backup.list_by_status(BackupStatus::Completed);
        assert_eq!(completed.len(), 1);
    }

    #[test]
    fn test_sigmabackup_list_by_type() {
        let mut backup = SigmaBackup::new();
        
        let job = BackupJob::new(
            "job1".to_string(),
            "/home".to_string(),
            "/backup".to_string(),
            BackupType::Full
        );
        
        backup.add_job(job).unwrap();
        
        let full = backup.list_by_type(BackupType::Full);
        assert_eq!(full.len(), 1);
    }

    #[test]
    fn test_sigmabackup_stats() {
        let mut backup = SigmaBackup::new();
        
        let job = BackupJob::new(
            "job1".to_string(),
            "/home".to_string(),
            "/backup".to_string(),
            BackupType::Full
        )
        .with_status(BackupStatus::Completed);
        
        backup.add_job(job).unwrap();
        
        let stats = backup.backup_stats();
        assert_eq!(stats.get("total_jobs"), Some(&"1".to_string()));
        assert_eq!(stats.get("completed_jobs"), Some(&"1".to_string()));
    }

    #[test]
    fn test_start_backup_job_convenience() {
        init_sigmabackup();
        
        let mut backup = get_sigmabackup().unwrap();
        backup.start();
        
        let job = BackupJob::new(
            "job1".to_string(),
            "/home".to_string(),
            "/backup".to_string(),
            BackupType::Full
        );
        
        backup.add_job(job).unwrap();
        
        let result = start_backup_job("job1");
        assert!(result.is_ok());
    }
}
