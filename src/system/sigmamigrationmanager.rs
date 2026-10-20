// SPDX-License-Identifier: MIT
// SigmaOS - Migration Manager
// Migration management inspired by Omarchy migrations and Linux database migrations

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Migration status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MigrationStatus {
    Pending,
    Applied,
    Failed,
    RolledBack,
}

/// Migration
#[derive(Debug, Clone)]
pub struct Migration {
    pub id: String,
    pub name: String,
    pub version: String,
    pub status: MigrationStatus,
}

/// Migration manager
pub struct SigmaMigrationManager {
    migrations: Vec<Migration>,
}

impl SigmaMigrationManager {
    pub fn new() -> Self {
        Self {
            migrations: Vec::new(),
        }
    }

    pub fn add_migration(&mut self, migration: Migration) {
        self.migrations.push(migration);
    }

    pub fn get_migrations(&self) -> &[Migration] {
        &self.migrations
    }

    pub fn get_migration(&self, id: &str) -> Option<&Migration> {
        self.migrations.iter().find(|m| m.id == id)
    }

    pub fn apply_migration(&mut self, id: String) -> Result<(), String> {
        if let Some(migration) = self.migrations.iter_mut().find(|m| m.id == id) {
            migration.status = MigrationStatus::Applied;
            Ok(())
        } else {
            Err(format!("Migration {} not found", id))
        }
    }

    pub fn rollback_migration(&mut self, id: String) -> Result<(), String> {
        if let Some(migration) = self.migrations.iter_mut().find(|m| m.id == id) {
            migration.status = MigrationStatus::RolledBack;
            Ok(())
        } else {
            Err(format!("Migration {} not found", id))
        }
    }

    pub fn get_pending_migrations(&self) -> Vec<&Migration> {
        self.migrations.iter().filter(|m| m.status == MigrationStatus::Pending).collect()
    }
}

impl Default for SigmaMigrationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migration_manager_creation() {
        let manager = SigmaMigrationManager::new();
        assert_eq!(manager.get_migrations().len(), 0);
    }

    #[test]
    fn test_add_migration() {
        let mut manager = SigmaMigrationManager::new();
        let migration = Migration {
            id: "migrate-001".to_string(),
            name: "Initial Migration".to_string(),
            version: "1.0".to_string(),
            status: MigrationStatus::Pending,
        };
        manager.add_migration(migration);
        assert_eq!(manager.get_migrations().len(), 1);
    }

    #[test]
    fn test_apply_migration() {
        let mut manager = SigmaMigrationManager::new();
        let migration = Migration {
            id: "migrate-001".to_string(),
            name: "Initial Migration".to_string(),
            version: "1.0".to_string(),
            status: MigrationStatus::Pending,
        };
        manager.add_migration(migration);
        manager.apply_migration("migrate-001".to_string()).unwrap();
        assert_eq!(manager.get_migrations()[0].status, MigrationStatus::Applied);
    }

    #[test]
    fn test_pending_migrations() {
        let mut manager = SigmaMigrationManager::new();
        let migration1 = Migration {
            id: "migrate-001".to_string(),
            name: "Migration 1".to_string(),
            version: "1.0".to_string(),
            status: MigrationStatus::Applied,
        };
        let migration2 = Migration {
            id: "migrate-002".to_string(),
            name: "Migration 2".to_string(),
            version: "1.1".to_string(),
            status: MigrationStatus::Pending,
        };
        manager.add_migration(migration1);
        manager.add_migration(migration2);
        assert_eq!(manager.get_pending_migrations().len(), 1);
    }
}
