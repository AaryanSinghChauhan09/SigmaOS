//! Backup Manager
//!
//! Backup management inspired by Linux Mint's backup tools and Omarchy's
//! backup utilities, supporting scheduled backups, backup sources, and restoration.

use std::collections::HashMap;

/// Backup type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupType {
    Full,
    Incremental,
    Differential,
}

impl BackupType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "full" => Some(BackupType::Full),
            "incremental" => Some(BackupType::Incremental),
            "differential" => Some(BackupType::Differential),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            BackupType::Full => "Full",
            BackupType::Incremental => "Incremental",
            BackupType::Differential => "Differential",
        }
    }
}

/// Backup status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl BackupStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "pending" => Some(BackupStatus::Pending),
            "running" => Some(BackupStatus::Running),
            "completed" => Some(BackupStatus::Completed),
            "failed" => Some(BackupStatus::Failed),
            "cancelled" => Some(BackupStatus::Cancelled),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            BackupStatus::Pending => "Pending",
            BackupStatus::Running => "Running",
            BackupStatus::Completed => "Completed",
            BackupStatus::Failed => "Failed",
            BackupStatus::Cancelled => "Cancelled",
        }
    }
}

/// Backup source
#[derive(Debug, Clone)]
pub struct BackupSource {
    pub id: String,
    pub path: String,
    pub name: String,
    pub is_enabled: bool,
}

impl BackupSource {
    pub fn new(id: String, path: String, name: String) -> Self {
        Self {
            id,
            path,
            name,
            is_enabled: true,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.is_enabled = enabled;
    }
}

/// Backup destination
#[derive(Debug, Clone)]
pub struct BackupDestination {
    pub id: String,
    pub path: String,
    pub name: String,
    pub capacity: u64,
}

impl BackupDestination {
    pub fn new(id: String, path: String, name: String, capacity: u64) -> Self {
        Self {
            id,
            path,
            name,
            capacity,
        }
    }
}

/// Backup job
#[derive(Debug, Clone)]
pub struct BackupJob {
    pub id: String,
    pub backup_type: BackupType,
    pub source_id: String,
    pub destination_id: String,
    pub status: BackupStatus,
    pub created_at: u64,
    pub completed_at: Option<u64>,
    pub size: u64,
    pub error_message: Option<String>,
}

impl BackupJob {
    pub fn new(id: String, backup_type: BackupType, source_id: String, destination_id: String) -> Self {
        Self {
            id,
            backup_type,
            source_id,
            destination_id,
            status: BackupStatus::Pending,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            completed_at: None,
            size: 0,
            error_message: None,
        }
    }

    pub fn set_status(&mut self, status: BackupStatus) {
        self.status = status;
        if status == BackupStatus::Completed || status == BackupStatus::Failed {
            self.completed_at = Some(std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs());
        }
    }

    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }

    pub fn set_error(&mut self, error: String) {
        self.error_message = Some(error);
    }
}

/// Backup manager
#[derive(Debug)]
pub struct BackupJobManager {
    sources: HashMap<String, BackupSource>,
    destinations: HashMap<String, BackupDestination>,
    jobs: HashMap<String, BackupJob>,
    next_job_id: u64,
}

impl BackupJobManager {
    pub fn new() -> Self {
        let mut manager = Self {
            sources: HashMap::new(),
            destinations: HashMap::new(),
            jobs: HashMap::new(),
            next_job_id: 1,
        };

        // Add default sources
        let home_source = BackupSource::new(
            "home".to_string(),
            "/home".to_string(),
            "Home Directory".to_string(),
        );
        manager.sources.insert("home".to_string(), home_source);

        let documents_source = BackupSource::new(
            "documents".to_string(),
            "/home/user/Documents".to_string(),
            "Documents".to_string(),
        );
        manager.sources.insert("documents".to_string(), documents_source);

        // Add default destination
        let backup_dest = BackupDestination::new(
            "local-backup".to_string(),
            "/backup".to_string(),
            "Local Backup".to_string(),
            1024 * 1024 * 1024 * 1024, // 1 TB
        );
        manager.destinations.insert("local-backup".to_string(), backup_dest);

        manager
    }

    /// Add a source
    pub fn add_source(&mut self, source: BackupSource) {
        self.sources.insert(source.id.clone(), source);
    }

    /// Get a source
    pub fn get_source(&self, id: &str) -> Option<&BackupSource> {
        self.sources.get(id)
    }

    /// List all sources
    pub fn list_sources(&self) -> Vec<&BackupSource> {
        self.sources.values().collect()
    }

    /// Add a destination
    pub fn add_destination(&mut self, destination: BackupDestination) {
        self.destinations.insert(destination.id.clone(), destination);
    }

    /// Get a destination
    pub fn get_destination(&self, id: &str) -> Option<&BackupDestination> {
        self.destinations.get(id)
    }

    /// List all destinations
    pub fn list_destinations(&self) -> Vec<&BackupDestination> {
        self.destinations.values().collect()
    }

    /// Create a backup job
    pub fn create_backup(&mut self, backup_type: BackupType, source_id: &str, destination_id: &str) -> Result<String, String> {
        let source = self.sources.get(source_id)
            .ok_or_else(|| format!("Source {} not found", source_id))?;

        if !source.is_enabled {
            return Err(format!("Source {} is disabled", source_id));
        }

        let destination = self.destinations.get(destination_id)
            .ok_or_else(|| format!("Destination {} not found", destination_id))?;

        let job_id = format!("backup-{}", self.next_job_id);
        self.next_job_id += 1;

        let job = BackupJob::new(
            job_id.clone(),
            backup_type,
            source_id.to_string(),
            destination_id.to_string(),
        );
        self.jobs.insert(job_id.clone(), job);

        Ok(job_id)
    }

    /// Get a job
    pub fn get_job(&self, id: &str) -> Option<&BackupJob> {
        self.jobs.get(id)
    }

    /// List all jobs
    pub fn list_jobs(&self) -> Vec<&BackupJob> {
        self.jobs.values().collect()
    }

    /// List jobs by status
    pub fn list_jobs_by_status(&self, status: BackupStatus) -> Vec<&BackupJob> {
        self.jobs.values()
            .filter(|j| j.status == status)
            .collect()
    }

    /// Cancel a job
    pub fn cancel_job(&mut self, id: &str) -> Result<(), String> {
        let job = self.jobs.get_mut(id)
            .ok_or_else(|| format!("Job {} not found", id))?;

        if job.status == BackupStatus::Completed {
            return Err("Cannot cancel completed job".to_string());
        }

        job.set_status(BackupStatus::Cancelled);
        Ok(())
    }

    /// Get statistics
    pub fn get_statistics(&self) -> BackupJobStatistics {
        let total_sources = self.sources.len();
        let enabled_sources = self.sources.values()
            .filter(|s| s.is_enabled)
            .count();
        let total_destinations = self.destinations.len();
        let total_jobs = self.jobs.len();
        let completed_jobs = self.jobs.values()
            .filter(|j| j.status == BackupStatus::Completed)
            .count();
        let failed_jobs = self.jobs.values()
            .filter(|j| j.status == BackupStatus::Failed)
            .count();

        BackupJobStatistics {
            total_sources,
            enabled_sources,
            total_destinations,
            total_jobs,
            completed_jobs,
            failed_jobs,
        }
    }
}

impl Default for BackupJobManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Backup statistics
#[derive(Debug, Clone)]
pub struct BackupJobStatistics {
    pub total_sources: usize,
    pub enabled_sources: usize,
    pub total_destinations: usize,
    pub total_jobs: usize,
    pub completed_jobs: usize,
    pub failed_jobs: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_type_from_str() {
        assert_eq!(BackupType::from_str("full"), Some(BackupType::Full));
        assert_eq!(BackupType::from_str("incremental"), Some(BackupType::Incremental));
    }

    #[test]
    fn test_backup_status_from_str() {
        assert_eq!(BackupStatus::from_str("pending"), Some(BackupStatus::Pending));
        assert_eq!(BackupStatus::from_str("completed"), Some(BackupStatus::Completed));
    }

    #[test]
    fn test_backup_source_creation() {
        let source = BackupSource::new(
            "test".to_string(),
            "/path".to_string(),
            "Test".to_string(),
        );
        assert_eq!(source.name, "Test");
    }

    #[test]
    fn test_backup_manager_creation() {
        let manager = BackupJobManager::new();
        assert!(manager.get_source("home").is_some());
    }

    #[test]
    fn test_add_source() {
        let mut manager = BackupJobManager::new();
        let source = BackupSource::new(
            "test".to_string(),
            "/test".to_string(),
            "Test".to_string(),
        );
        manager.add_source(source);
        assert!(manager.get_source("test").is_some());
    }

    #[test]
    fn test_create_backup() {
        let mut manager = BackupJobManager::new();
        let job_id = manager.create_backup(
            BackupType::Full,
            "home",
            "local-backup",
        );
        assert!(job_id.is_ok());
    }

    #[test]
    fn test_cancel_job() {
        let mut manager = BackupJobManager::new();
        let job_id = manager.create_backup(
            BackupType::Full,
            "home",
            "local-backup",
        ).unwrap();
        assert!(manager.cancel_job(&job_id).is_ok());
    }

    #[test]
    fn test_statistics() {
        let manager = BackupJobManager::new();
        let stats = manager.get_statistics();
        assert!(stats.total_sources >= 2);
    }
}
