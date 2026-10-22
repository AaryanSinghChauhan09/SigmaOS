// SPDX-License-Identifier: MIT
// SigmaOS - Backup Manager
// Backup management inspired by Arch (borg), Debian (restic), and CachyOS

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

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
}

/// Backup entry
#[derive(Debug, Clone)]
pub struct BackupEntry {
    pub id: String,
    pub name: String,
    pub backup_type: BackupType,
    pub status: BackupStatus,
    pub size_bytes: u64,
    pub timestamp: u64,
    pub source_path: String,
    pub destination_path: String,
}

/// Backup manager
pub struct SigmaBackupManager {
    backups: Vec<BackupEntry>,
}

impl SigmaBackupManager {
    pub fn new() -> Self {
        Self {
            backups: Vec::new(),
        }
    }

    pub fn create_backup(
        &mut self,
        name: String,
        backup_type: BackupType,
        source_path: String,
        destination_path: String,
    ) -> BackupEntry {
        let backup = BackupEntry {
            id: alloc::format!("backup_{}", self.backups.len()),
            name,
            backup_type,
            status: BackupStatus::Pending,
            size_bytes: 0,
            timestamp: 0,
            source_path,
            destination_path,
        };
        self.backups.push(backup.clone());
        backup
    }

    pub fn get_backups(&self) -> &[BackupEntry] {
        &self.backups
    }

    pub fn get_backup(&self, id: &str) -> Option<&BackupEntry> {
        self.backups.iter().find(|b| b.id == id)
    }

    pub fn get_backup_mut(&mut self, id: &str) -> Option<&mut BackupEntry> {
        self.backups.iter_mut().find(|b| b.id == id)
    }

    pub fn start_backup(&mut self, id: String) -> Result<(), String> {
        if let Some(backup) = self.get_backup_mut(&id) {
            backup.status = BackupStatus::Running;
            Ok(())
        } else {
            Err(format!("Backup {} not found", id))
        }
    }

    pub fn complete_backup(&mut self, id: String, size_bytes: u64) -> Result<(), String> {
        if let Some(backup) = self.get_backup_mut(&id) {
            backup.status = BackupStatus::Completed;
            backup.size_bytes = size_bytes;
            Ok(())
        } else {
            Err(format!("Backup {} not found", id))
        }
    }

    pub fn fail_backup(&mut self, id: String) -> Result<(), String> {
        if let Some(backup) = self.get_backup_mut(&id) {
            backup.status = BackupStatus::Failed;
            Ok(())
        } else {
            Err(format!("Backup {} not found", id))
        }
    }

    pub fn delete_backup(&mut self, id: String) -> Result<(), String> {
        if let Some(idx) = self.backups.iter().position(|b| b.id == id) {
            self.backups.remove(idx);
            Ok(())
        } else {
            Err(format!("Backup {} not found", id))
        }
    }

    pub fn get_backups_by_type(&self, backup_type: BackupType) -> Vec<&BackupEntry> {
        self.backups
            .iter()
            .filter(|b| b.backup_type == backup_type)
            .collect()
    }

    pub fn get_total_backup_size(&self) -> u64 {
        self.backups.iter().map(|b| b.size_bytes).sum()
    }
}

impl Default for SigmaBackupManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_manager_creation() {
        let manager = SigmaBackupManager::new();
        assert_eq!(manager.get_backups().len(), 0);
    }

    #[test]
    fn test_create_backup() {
        let mut manager = SigmaBackupManager::new();
        manager.create_backup(
            "test".to_string(),
            BackupType::Full,
            "/source".to_string(),
            "/dest".to_string(),
        );
        assert_eq!(manager.get_backups().len(), 1);
    }

    #[test]
    fn test_start_backup() {
        let mut manager = SigmaBackupManager::new();
        let backup = manager.create_backup(
            "test".to_string(),
            BackupType::Full,
            "/source".to_string(),
            "/dest".to_string(),
        );
        manager.start_backup(backup.id.clone()).unwrap();
        assert_eq!(manager.get_backups()[0].status, BackupStatus::Running);
    }

    #[test]
    fn test_complete_backup() {
        let mut manager = SigmaBackupManager::new();
        let backup = manager.create_backup(
            "test".to_string(),
            BackupType::Full,
            "/source".to_string(),
            "/dest".to_string(),
        );
        manager.start_backup(backup.id).unwrap();
        manager.complete_backup(backup.id.clone(), 1000).unwrap();
        assert_eq!(manager.get_backups()[0].status, BackupStatus::Completed);
        assert_eq!(manager.get_backups()[0].size_bytes, 1000);
    }

    #[test]
    fn test_delete_backup() {
        let mut manager = SigmaBackupManager::new();
        let backup = manager.create_backup(
            "test".to_string(),
            BackupType::Full,
            "/source".to_string(),
            "/dest".to_string(),
        );
        manager.delete_backup(backup.id).unwrap();
        assert_eq!(manager.get_backups().len(), 0);
    }

    #[test]
    fn test_backups_by_type() {
        let mut manager = SigmaBackupManager::new();
        manager.create_backup(
            "full".to_string(),
            BackupType::Full,
            "/source".to_string(),
            "/dest".to_string(),
        );
        manager.create_backup(
            "incremental".to_string(),
            BackupType::Incremental,
            "/source".to_string(),
            "/dest".to_string(),
        );
        assert_eq!(manager.get_backups_by_type(BackupType::Full).len(), 1);
    }

    #[test]
    fn test_total_backup_size() {
        let mut manager = SigmaBackupManager::new();
        let backup1 = manager.create_backup(
            "test1".to_string(),
            BackupType::Full,
            "/source".to_string(),
            "/dest".to_string(),
        );
        let backup2 = manager.create_backup(
            "test2".to_string(),
            BackupType::Full,
            "/source".to_string(),
            "/dest".to_string(),
        );
        manager.complete_backup(backup1.id.clone(), 1000).unwrap();
        manager.complete_backup(backup2.id.clone(), 2000).unwrap();
        assert_eq!(manager.get_total_backup_size(), 3000);
    }
}
