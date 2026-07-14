// Atomic Upgrade System for SigmaOS
// Implements transactional system updates with rollback capability
// Inspired by Nix and openSUSE transactional updates

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: String,
    pub timestamp: i64,
    pub packages: Vec<String>,
    pub status: TransactionStatus,
    pub rollback_data: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TransactionStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    RolledBack,
}

pub struct AtomicUpgradeManager {
    store_path: PathBuf,
    transactions_path: PathBuf,
    current_transaction: Option<Transaction>,
}

impl AtomicUpgradeManager {
    pub fn new(store_path: PathBuf) -> Result<Self, std::io::Error> {
        let transactions_path = store_path.join("transactions");
        fs::create_dir_all(&transactions_path)?;
        
        Ok(AtomicUpgradeManager {
            store_path,
            transactions_path,
            current_transaction: None,
        })
    }

    /// Begin a new transactional upgrade
    pub fn begin_transaction(&mut self, packages: Vec<String>) -> Result<String, std::io::Error> {
        let transaction_id = format!("txn_{}", chrono::Utc::now().timestamp());
        let transaction_path = self.transactions_path.join(&transaction_id);
        fs::create_dir_all(&transaction_path)?;

        let transaction = Transaction {
            id: transaction_id.clone(),
            timestamp: chrono::Utc::now().timestamp(),
            packages: packages.clone(),
            status: TransactionStatus::Pending,
            rollback_data: transaction_path.join("rollback"),
        };

        // Save transaction metadata
        self.save_transaction(&transaction)?;
        self.current_transaction = Some(transaction);

        Ok(transaction_id)
    }

    /// Execute the transaction
    pub fn execute_transaction(&mut self) -> Result<(), std::io::Error> {
        if let Some(ref mut transaction) = self.current_transaction {
            transaction.status = TransactionStatus::InProgress;
            self.save_transaction(transaction)?;

            // Create rollback snapshot before applying changes
            self.create_rollback_snapshot(transaction)?;

            // Apply package updates
            for package in &transaction.packages {
                self.install_package(package)?;
            }

            transaction.status = TransactionStatus::Completed;
            self.save_transaction(transaction)?;

            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No active transaction",
            ))
        }
    }

    /// Rollback to previous system state
    pub fn rollback(&mut self, transaction_id: &str) -> Result<(), std::io::Error> {
        let transaction = self.load_transaction(transaction_id)?;
        
        if transaction.status != TransactionStatus::Completed {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Cannot rollback non-completed transaction",
            ));
        }

        // Restore from rollback snapshot
        self.restore_rollback_snapshot(&transaction)?;

        // Update transaction status
        let mut updated_transaction = transaction;
        updated_transaction.status = TransactionStatus::RolledBack;
        self.save_transaction(&updated_transaction)?;

        Ok(())
    }

    /// Get available rollback points
    pub fn list_rollback_points(&self) -> Result<Vec<Transaction>, std::io::Error> {
        let mut transactions = Vec::new();
        
        for entry in fs::read_dir(&self.transactions_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                if let Ok(transaction) = self.load_transaction_from_path(&path) {
                    if transaction.status == TransactionStatus::Completed {
                        transactions.push(transaction);
                    }
                }
            }
        }

        transactions.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        Ok(transactions)
    }

    fn create_rollback_snapshot(&self, transaction: &Transaction) -> Result<(), std::io::Error> {
        let rollback_path = &transaction.rollback_data;
        fs::create_dir_all(rollback_path)?;

        // Snapshot current system state
        let system_path = Path::new("/sigma/store/current");
        if system_path.exists() {
            let snapshot_path = rollback_path.join("system_snapshot");
            self.copy_directory(system_path, &snapshot_path)?;
        }

        // Snapshot configuration
        let config_path = Path::new("/etc/sigma");
        if config_path.exists() {
            let config_snapshot = rollback_path.join("config_snapshot");
            self.copy_directory(config_path, &config_snapshot)?;
        }

        Ok(())
    }

    fn restore_rollback_snapshot(&self, transaction: &Transaction) -> Result<(), std::io::Error> {
        let rollback_path = &transaction.rollback_data;

        // Restore system state
        let system_snapshot = rollback_path.join("system_snapshot");
        if system_snapshot.exists() {
            let system_path = Path::new("/sigma/store/current");
            self.copy_directory(&system_snapshot, system_path)?;
        }

        // Restore configuration
        let config_snapshot = rollback_path.join("config_snapshot");
        if config_snapshot.exists() {
            let config_path = Path::new("/etc/sigma");
            self.copy_directory(&config_snapshot, config_path)?;
        }

        Ok(())
    }

    fn install_package(&self, package: &str) -> Result<(), std::io::Error> {
        // Placeholder for actual package installation logic
        println!("Installing package: {}", package);
        Ok(())
    }

    fn save_transaction(&self, transaction: &Transaction) -> Result<(), std::io::Error> {
       let transaction_path = self.transactions_path.join(&transaction.id);
        let metadata_path = transaction_path.join("metadata.json");
        
        let metadata = serde_json::to_string_pretty(transaction)?;
        fs::write(metadata_path, metadata)?;
        
        Ok(())
    }

    fn load_transaction(&self, transaction_id: &str) -> Result<Transaction, std::io::Error> {
        let transaction_path = self.transactions_path.join(transaction_id);
        self.load_transaction_from_path(&transaction_path)
    }

    fn load_transaction_from_path(&self, path: &Path) -> Result<Transaction, std::io::Error> {
        let metadata_path = path.join("metadata.json");
        let metadata = fs::read_to_string(metadata_path)?;
        let transaction: Transaction = serde_json::from_str(&metadata)?;
        Ok(transaction)
    }

    fn copy_directory(&self, src: &Path, dst: &Path) -> Result<(), std::io::Error> {
        if dst.exists() {
            fs::remove_dir_all(dst)?;
        }
        
        let mut cmd = Command::new("cp");
        cmd.arg("-r")
           .arg(src)
           .arg(dst);
           
        cmd.status()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_transaction_creation() {
        let temp_dir = tempdir().unwrap();
        let store_path = temp_dir.path().to_path_buf();
        
        let mut manager = AtomicUpgradeManager::new(store_path).unwrap();
        let packages = vec!["test-package".to_string()];
        
        let txn_id = manager.begin_transaction(packages).unwrap();
        assert!(txn_id.starts_with("txn_"));
    }

    #[test]
    fn test_rollback_list() {
        let temp_dir = tempdir().unwrap();
        let store_path = temp_dir.path().to_path_buf();
        
        let manager = AtomicUpgradeManager::new(store_path).unwrap();
        let rollbacks = manager.list_rollback_points().unwrap();
        assert!(rollbacks.is_empty());
    }
}
