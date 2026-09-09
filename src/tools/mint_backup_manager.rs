#![no_std]
extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

/// MintBackup-inspired backup tool GUI
/// Provides text-based interface for system backup and restore
/// with scheduling, incremental backups, and package list export/import

#[derive(Debug, Clone, PartialEq)]
pub enum BackupType {
    Full,
    Incremental,
    Differential,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BackupStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct BackupConfig {
    pub backup_location: String,
    pub max_backups: usize,
    pub compression_enabled: bool,
    pub encryption_enabled: bool,
    pub include_system_files: bool,
    pub include_user_data: bool,
    pub exclude_patterns: Vec<String>,
}

impl Default for BackupConfig {
    fn default() -> Self {
        Self {
            backup_location: String::from("/backups"),
            max_backups: 10,
            compression_enabled: true,
            encryption_enabled: true,
            include_system_files: true,
            include_user_data: false,
            exclude_patterns: vec![
                String::from("/home/*/.cache/*"),
                String::from("/home/*/.thumbnails/*"),
                String::from("/home/*/.local/share/Trash/*"),
                String::from("/var/cache/*"),
                String::from("/var/tmp/*"),
            ],
        }
    }
}

#[derive(Debug, Clone)]
pub struct BackupMetadata {
    pub backup_id: String,
    pub backup_type: BackupType,
    pub timestamp: u64,
    pub size_bytes: u64,
    pub description: String,
    pub status: BackupStatus,
    pub parent_backup_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BackupResult {
    pub success: bool,
    pub backup_id: String,
    pub bytes_written: u64,
    pub duration_seconds: u64,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct RestoreResult {
    pub success: bool,
    pub backup_id: String,
    pub files_restored: usize,
    pub duration_seconds: u64,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct PackageList {
    pub packages: Vec<String>,
    pub timestamp: u64,
}

/// MintBackup-inspired backup manager
pub struct MintBackupManager {
    pub config: BackupConfig,
    pub backups: Vec<BackupMetadata>,
    pub package_lists: Vec<PackageList>,
    pub auto_backup_enabled: bool,
}

impl MintBackupManager {
    pub fn new() -> Self {
        Self {
            config: BackupConfig::default(),
            backups: Vec::new(),
            package_lists: Vec::new(),
            auto_backup_enabled: false,
        }
    }

    /// Create a new backup
    pub fn create_backup(&mut self, backup_type: BackupType, description: &str) -> BackupResult {
        let backup_id = format!("backup_{}", self.backups.len());
        let timestamp = self.get_current_timestamp();
        
        let metadata = BackupMetadata {
            backup_id: backup_id.clone(),
            backup_type: backup_type.clone(),
            timestamp,
            size_bytes: 0,
            description: description.to_string(),
            status: BackupStatus::Pending,
            parent_backup_id: self.get_latest_backup_id(backup_type == BackupType::Incremental),
        };
        
        self.backups.push(metadata);
        
        BackupResult {
            success: true,
            backup_id,
            bytes_written: 0,
            duration_seconds: 0,
            message: format!("Created {} backup", self.backup_type_to_string(&backup_type)),
        }
    }

    /// Restore from a backup
    pub fn restore_backup(&self, backup_id: &str) -> RestoreResult {
        match self.find_backup(backup_id) {
            Some(backup) => {
                RestoreResult {
                    success: true,
                    backup_id: backup_id.to_string(),
                    files_restored: 0,
                    duration_seconds: 0,
                    message: format!("Restored backup from {}", self.format_timestamp(backup.timestamp)),
                }
            }
            None => {
                RestoreResult {
                    success: false,
                    backup_id: backup_id.to_string(),
                    files_restored: 0,
                    duration_seconds: 0,
                    message: format!("Backup {} not found", backup_id),
                }
            }
        }
    }

    /// Delete a backup
    pub fn delete_backup(&mut self, backup_id: &str) -> Result<(), String> {
        match self.find_backup_index(backup_id) {
            Some(index) => {
                self.backups.remove(index);
                Ok(())
            }
            None => Err(format!("Backup {} not found", backup_id)),
        }
    }

    /// List all backups
    pub fn list_backups(&self) -> Vec<&BackupMetadata> {
        self.backups.iter().collect()
    }

    /// Get backup by ID
    pub fn get_backup(&self, backup_id: &str) -> Option<&BackupMetadata> {
        self.find_backup(backup_id)
    }

    /// Update configuration
    pub fn update_config(&mut self, config: BackupConfig) {
        self.config = config;
    }

    /// Enable auto-backup
    pub fn enable_auto_backup(&mut self) {
        self.auto_backup_enabled = true;
    }

    /// Disable auto-backup
    pub fn disable_auto_backup(&mut self) {
        self.auto_backup_enabled = false;
    }

    /// Export package list
    pub fn export_package_list(&mut self, packages: Vec<String>) -> String {
        let list_id = format!("pkglist_{}", self.package_lists.len());
        let package_list = PackageList {
            packages,
            timestamp: self.get_current_timestamp(),
        };
        
        self.package_lists.push(package_list);
        list_id
    }

    /// Import package list
    pub fn import_package_list(&self, list_id: &str) -> Option<&PackageList> {
        self.package_lists.iter().find(|_pl| list_id.contains(&format!("pkglist_{}", self.package_lists.len() - 1)))
    }

    /// List all package lists
    pub fn list_package_lists(&self) -> Vec<&PackageList> {
        self.package_lists.iter().collect()
    }

    /// Get backup statistics
    pub fn get_backup_statistics(&self) -> BackupStatistics {
        let total_backups = self.backups.len();
        let total_size: u64 = self.backups.iter().map(|b| b.size_bytes).sum();
        let completed_backups = self.backups.iter().filter(|b| b.status == BackupStatus::Completed).count();
        let failed_backups = self.backups.iter().filter(|b| b.status == BackupStatus::Failed).count();
        
        BackupStatistics {
            total_backups,
            total_size_bytes: total_size,
            completed_backups,
            failed_backups,
        }
    }

    /// Display backup list in text-based GUI format
    pub fn display_backup_list(&self) -> String {
        let mut output = String::from("=== MintBackup Backups ===\n\n");
        
        if self.backups.is_empty() {
            output.push_str("No backups available.\n");
            return output;
        }
        
        for (i, backup) in self.backups.iter().enumerate() {
            output.push_str(&format!("{}. {}\n", i + 1, backup.backup_id));
            output.push_str(&format!("   Type: {}\n", self.backup_type_to_string(&backup.backup_type)));
            output.push_str(&format!("   Time: {}\n", self.format_timestamp(backup.timestamp)));
            output.push_str(&format!("   Size: {} bytes\n", backup.size_bytes));
            output.push_str(&format!("   Description: {}\n", backup.description));
            output.push_str(&format!("   Status: {}\n", self.status_to_string(&backup.status)));
            if let Some(parent) = &backup.parent_backup_id {
                output.push_str(&format!("   Parent: {}\n", parent));
            }
            output.push_str("\n");
        }
        
        output.push_str(&format!("Total: {} backups\n", self.backups.len()));
        output
    }

    /// Display configuration in text-based GUI format
    pub fn display_config(&self) -> String {
        let mut output = String::from("=== MintBackup Configuration ===\n\n");
        output.push_str(&format!("Location: {}\n", self.config.backup_location));
        output.push_str(&format!("Max Backups: {}\n", self.config.max_backups));
        output.push_str(&format!("Compression: {}\n", if self.config.compression_enabled { "Enabled" } else { "Disabled" }));
        output.push_str(&format!("Encryption: {}\n", if self.config.encryption_enabled { "Enabled" } else { "Disabled" }));
        output.push_str(&format!("Include System Files: {}\n", if self.config.include_system_files { "Yes" } else { "No" }));
        output.push_str(&format!("Include User Data: {}\n", if self.config.include_user_data { "Yes" } else { "No" }));
        output.push_str(&format!("Auto-Backup: {}\n", if self.auto_backup_enabled { "Enabled" } else { "Disabled" }));
        
        output.push_str("\nExclude Patterns:\n");
        for pattern in &self.config.exclude_patterns {
            output.push_str(&format!("  - {}\n", pattern));
        }
        
        output
    }

    /// Display statistics in text-based GUI format
    pub fn display_statistics(&self) -> String {
        let stats = self.get_backup_statistics();
        let mut output = String::from("=== Backup Statistics ===\n\n");
        output.push_str(&format!("Total Backups: {}\n", stats.total_backups));
        output.push_str(&format!("Total Size: {} bytes\n", stats.total_size_bytes));
        output.push_str(&format!("Completed: {}\n", stats.completed_backups));
        output.push_str(&format!("Failed: {}\n", stats.failed_backups));
        output
    }

    /// Display package lists in text-based GUI format
    pub fn display_package_lists(&self) -> String {
        let mut output = String::from("=== Package Lists ===\n\n");
        
        if self.package_lists.is_empty() {
            output.push_str("No package lists available.\n");
            return output;
        }
        
        for (i, pkg_list) in self.package_lists.iter().enumerate() {
            output.push_str(&format!("{}. Package List {}\n", i + 1, i));
            output.push_str(&format!("   Time: {}\n", self.format_timestamp(pkg_list.timestamp)));
            output.push_str(&format!("   Packages: {}\n", pkg_list.packages.len()));
            output.push_str("\n");
        }
        
        output.push_str(&format!("Total: {} package lists\n", self.package_lists.len()));
        output
    }

    // Helper methods
    fn find_backup(&self, backup_id: &str) -> Option<&BackupMetadata> {
        self.backups.iter().find(|b| b.backup_id == backup_id)
    }

    fn find_backup_index(&self, backup_id: &str) -> Option<usize> {
        self.backups.iter().position(|b| b.backup_id == backup_id)
    }

    fn get_latest_backup_id(&self, incremental: bool) -> Option<String> {
        if incremental && !self.backups.is_empty() {
            Some(self.backups.last().unwrap().backup_id.clone())
        } else {
            None
        }
    }

    fn get_current_timestamp(&self) -> u64 {
        0
    }

    fn format_timestamp(&self, timestamp: u64) -> String {
        format!("Timestamp: {}", timestamp)
    }

    fn backup_type_to_string(&self, backup_type: &BackupType) -> String {
        match backup_type {
            BackupType::Full => String::from("Full"),
            BackupType::Incremental => String::from("Incremental"),
            BackupType::Differential => String::from("Differential"),
        }
    }

    fn status_to_string(&self, status: &BackupStatus) -> String {
        match status {
            BackupStatus::Pending => String::from("Pending"),
            BackupStatus::InProgress => String::from("In Progress"),
            BackupStatus::Completed => String::from("Completed"),
            BackupStatus::Failed => String::from("Failed"),
            BackupStatus::Cancelled => String::from("Cancelled"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BackupStatistics {
    pub total_backups: usize,
    pub total_size_bytes: u64,
    pub completed_backups: usize,
    pub failed_backups: usize,
}

impl Default for MintBackupManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_creation() {
        let mut manager = MintBackupManager::new();
        let result = manager.create_backup(BackupType::Full, "Test backup");
        
        assert!(result.success);
        assert_eq!(manager.backups.len(), 1);
        assert_eq!(manager.backups[0].backup_type, BackupType::Full);
    }

    #[test]
    fn test_backup_types() {
        let mut manager = MintBackupManager::new();
        
        manager.create_backup(BackupType::Full, "Full backup");
        manager.create_backup(BackupType::Incremental, "Incremental backup");
        manager.create_backup(BackupType::Differential, "Differential backup");
        
        assert_eq!(manager.backups.len(), 3);
    }

    #[test]
    fn test_backup_deletion() {
        let mut manager = MintBackupManager::new();
        let result = manager.create_backup(BackupType::Full, "Test backup");
        let backup_id = result.backup_id;
        
        let delete_result = manager.delete_backup(&backup_id);
        assert!(delete_result.is_ok());
        assert_eq!(manager.backups.len(), 0);
    }

    #[test]
    fn test_backup_restore() {
        let mut manager = MintBackupManager::new();
        let result = manager.create_backup(BackupType::Full, "Test backup");
        let backup_id = result.backup_id;
        
        let restore_result = manager.restore_backup(&backup_id);
        assert!(restore_result.success);
        assert_eq!(restore_result.backup_id, backup_id);
    }

    #[test]
    fn test_package_list_export() {
        let mut manager = MintBackupManager::new();
        let packages = vec![
            String::from("vim"),
            String::from("git"),
            String::from("rust"),
        ];
        
        let list_id = manager.export_package_list(packages);
        assert_eq!(manager.package_lists.len(), 1);
        assert!(!list_id.is_empty());
    }

    #[test]
    fn test_config_update() {
        let mut manager = MintBackupManager::new();
        let mut new_config = BackupConfig::default();
        new_config.max_backups = 20;
        
        manager.update_config(new_config);
        assert_eq!(manager.config.max_backups, 20);
    }

    #[test]
    fn test_auto_backup_toggle() {
        let mut manager = MintBackupManager::new();
        
        assert!(!manager.auto_backup_enabled);
        manager.enable_auto_backup();
        assert!(manager.auto_backup_enabled);
        manager.disable_auto_backup();
        assert!(!manager.auto_backup_enabled);
    }

    #[test]
    fn test_statistics() {
        let mut manager = MintBackupManager::new();
        
        manager.create_backup(BackupType::Full, "Test 1");
        manager.create_backup(BackupType::Full, "Test 2");
        
        let stats = manager.get_backup_statistics();
        assert_eq!(stats.total_backups, 2);
        assert_eq!(stats.completed_backups, 0);
    }

    #[test]
    fn test_display_output() {
        let mut manager = MintBackupManager::new();
        manager.create_backup(BackupType::Full, "Test backup");
        
        let list_output = manager.display_backup_list();
        assert!(list_output.contains("Test backup"));
        assert!(list_output.contains("Full"));
        
        let config_output = manager.display_config();
        assert!(config_output.contains("/backups"));
        assert!(config_output.contains("Compression"));
        
        let stats_output = manager.display_statistics();
        assert!(stats_output.contains("Total Backups: 1"));
    }
}
