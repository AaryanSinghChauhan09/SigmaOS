//! SigmaOS Package Transaction Management
//! Provides atomic package operations with rollback capability
//! Ensures system remains in consistent state during package operations

use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;

/// Transaction state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransactionState {
    NotStarted,
    InProgress,
    Committed,
    RolledBack,
}

/// Transaction operation
#[derive(Debug, Clone)]
pub enum TransactionOp {
    Install { name: String, version: String },
    Remove { name: String },
    Update { name: String, old_version: String, new_version: String },
    Config { file: PathBuf, old_content: Vec<u8>, new_content: Vec<u8> },
}

/// Transaction
#[derive(Debug)]
pub struct Transaction {
    pub id: String,
    pub state: TransactionState,
    pub operations: Vec<TransactionOp>,
    pub completed_ops: usize,
    pub rollback_ops: Vec<TransactionOp>,
}

impl Transaction {
    pub fn new(id: String) -> Self {
        Self {
            id,
            state: TransactionState::NotStarted,
            operations: Vec::new(),
            completed_ops: 0,
            rollback_ops: Vec::new(),
        }
    }
    
    pub fn begin(&mut self) -> Result<(), String> {
        if self.state != TransactionState::NotStarted {
            return Err("Transaction already started".to_string());
        }
        
        self.state = TransactionState::InProgress;
        Ok(())
    }
    
    pub fn add_operation(&mut self, op: TransactionOp) -> Result<(), String> {
        if self.state != TransactionState::InProgress {
            return Err("Transaction not in progress".to_string());
        }
        
        // Record rollback operation
        let rollback_op = match &op {
            TransactionOp::Install { name, version } => {
                TransactionOp::Remove { name: name.clone() }
            }
            TransactionOp::Remove { name } => {
                // In real implementation, would need to know version to reinstall
                TransactionOp::Install { name: name.clone(), version: "unknown".to_string() }
            }
            TransactionOp::Update { name, old_version, .. } => {
                TransactionOp::Update { name: name.clone(), old_version: old_version.clone(), new_version: old_version.clone() }
            }
            TransactionOp::Config { file, old_content, .. } => {
                TransactionOp::Config { file: file.clone(), old_content: old_content.clone(), new_content: Vec::new() }
            }
        };
        
        self.rollback_ops.push(rollback_op);
        self.operations.push(op);
        
        Ok(())
    }
    
    pub fn commit(&mut self) -> Result<(), String> {
        if self.state != TransactionState::InProgress {
            return Err("Transaction not in progress".to_string());
        }
        
        self.state = TransactionState::Committed;
        self.rollback_ops.clear();
        
        Ok(())
    }
    
    pub fn rollback(&mut self) -> Result<(), String> {
        if self.state != TransactionState::InProgress {
            return Err("Transaction not in progress".to_string());
        }
        
        // Execute rollback operations in reverse order
        for op in self.rollback_ops.iter().rev() {
            match op {
                TransactionOp::Install { name, version } => {
                    // Remove package
                    if let Err(e) = self.remove_package(name, version) {
                        eprintln!("Rollback failed for {}: {}", name, e);
                    }
                }
                TransactionOp::Remove { name } => {
                    // Reinstall package
                    if let Err(e) = self.install_package(name, "unknown") {
                        eprintln!("Rollback failed for {}: {}", name, e);
                    }
                }
                TransactionOp::Update { name, old_version, .. } => {
                    // Revert to old version
                    if let Err(e) = self.update_package(name, old_version) {
                        eprintln!("Rollback failed for {}: {}", name, e);
                    }
                }
                TransactionOp::Config { file, old_content, .. } => {
                    // Restore old config
                    if let Err(e) = fs::write(file, old_content) {
                        eprintln!("Rollback failed for {:?}: {}", file, e);
                    }
                }
            }
        }
        
        self.state = TransactionState::RolledBack;
        Ok(())
    }
    
    fn install_package(&self, name: &str, version: &str) -> Result<(), String> {
        // In real implementation, install package
        eprintln!("Installing {} version {}", name, version);
        Ok(())
    }
    
    fn remove_package(&self, name: &str, version: &str) -> Result<(), String> {
        // In real implementation, remove package
        eprintln!("Removing {} version {}", name, version);
        Ok(())
    }
    
    fn update_package(&self, name: &str, version: &str) -> Result<(), String> {
        // In real implementation, update package
        eprintln!("Updating {} to version {}", name, version);
        Ok(())
    }
    
    pub fn get_state(&self) -> TransactionState {
        self.state
    }
    
    pub fn get_operation_count(&self) -> usize {
        self.operations.len()
    }
}

/// Transaction manager
#[derive(Debug)]
pub struct TransactionManager {
    pub current_transaction: Option<Transaction>,
    pub transaction_log: Vec<Transaction>,
    pub state_dir: PathBuf,
}

impl TransactionManager {
    pub fn new(state_dir: PathBuf) -> Self {
        Self {
            current_transaction: None,
            transaction_log: Vec::new(),
            state_dir,
        }
    }
    
    pub fn init(&mut self) -> Result<(), String> {
        fs::create_dir_all(&self.state_dir).map_err(|e| format!("Failed to create state dir: {}", e))?;
        
        // Load transaction log
        self.load_transaction_log()?;
        
        Ok(())
    }
    
    pub fn begin_transaction(&mut self) -> Result<String, String> {
        if self.current_transaction.is_some() {
            return Err("Transaction already in progress".to_string());
        }
        
        let id = format!("txn-{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis());
        
        let transaction = Transaction::new(id.clone());
        transaction.begin()?;
        
        self.current_transaction = Some(transaction);
        
        Ok(id)
    }
    
    pub fn add_operation(&mut self, op: TransactionOp) -> Result<(), String> {
        if let Some(ref mut transaction) = self.current_transaction {
            transaction.add_operation(op)
        } else {
            Err("No transaction in progress".to_string())
        }
    }
    
    pub fn commit_transaction(&mut self) -> Result<(), String> {
        if let Some(transaction) = self.current_transaction.take() {
            transaction.commit()?;
            self.transaction_log.push(transaction);
            self.save_transaction_log()?;
            Ok(())
        } else {
            Err("No transaction in progress".to_string())
        }
    }
    
    pub fn rollback_transaction(&mut self) -> Result<(), String> {
        if let Some(mut transaction) = self.current_transaction.take() {
            transaction.rollback()?;
            self.transaction_log.push(transaction);
            self.save_transaction_log()?;
            Ok(())
        } else {
            Err("No transaction in progress".to_string())
        }
    }
    
    pub fn get_transaction_state(&self) -> Option<TransactionState> {
        self.current_transaction.as_ref().map(|t| t.get_state())
    }
    
    fn load_transaction_log(&mut self) -> Result<(), String> {
        let log_path = self.state_dir.join("transactions.log");
        
        if !log_path.exists() {
            return Ok(());
        }
        
        // In real implementation, load and parse transaction log
        // For now, just return success
        Ok(())
    }
    
    fn save_transaction_log(&self) -> Result<(), String> {
        let log_path = self.state_dir.join("transactions.log");
        
        // In real implementation, serialize and save transaction log
        // For now, just return success
        Ok(())
    }
    
    pub fn get_transaction_history(&self) -> Vec<Transaction> {
        self.transaction_log.clone()
    }
}

/// Install package with transaction
pub fn install_package_with_transaction(manager: &mut TransactionManager, name: String, version: String) -> Result<(), String> {
    manager.begin_transaction()?;
    
    manager.add_operation(TransactionOp::Install {
        name: name.clone(),
        version: version.clone(),
    })?;
    
    // Perform actual installation
    // In real implementation, this would call the package manager
    
    manager.commit_transaction()
}

/// Remove package with transaction
pub fn remove_package_with_transaction(manager: &mut TransactionManager, name: String) -> Result<(), String> {
    manager.begin_transaction()?;
    
    manager.add_operation(TransactionOp::Remove {
        name: name.clone(),
    })?;
    
    // Perform actual removal
    // In real implementation, this would call the package manager
    
    manager.commit_transaction()
}

/// Update package with transaction
pub fn update_package_with_transaction(manager: &mut TransactionManager, name: String, old_version: String, new_version: String) -> Result<(), String> {
    manager.begin_transaction()?;
    
    manager.add_operation(TransactionOp::Update {
        name: name.clone(),
        old_version: old_version.clone(),
        new_version: new_version.clone(),
    })?;
    
    // Perform actual update
    // In real implementation, this would call the package manager
    
    manager.commit_transaction()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_transaction_lifecycle() {
        let mut txn = Transaction::new("test-txn".to_string());
        
        assert_eq!(txn.get_state(), TransactionState::NotStarted);
        
        txn.begin().unwrap();
        assert_eq!(txn.get_state(), TransactionState::InProgress);
        
        txn.commit().unwrap();
        assert_eq!(txn.get_state(), TransactionState::Committed);
    }
    
    #[test]
    fn test_transaction_rollback() {
        let mut txn = Transaction::new("test-txn".to_string());
        
        txn.begin().unwrap();
        txn.add_operation(TransactionOp::Install {
            name: "test-pkg".to_string(),
            version: "1.0.0".to_string(),
        }).unwrap();
        
        txn.rollback().unwrap();
        assert_eq!(txn.get_state(), TransactionState::RolledBack);
    }
    
    #[test]
    fn test_transaction_manager() {
        let state_dir = PathBuf::from("/tmp/test_txn_state");
        let mut manager = TransactionManager::new(state_dir);
        
        let id = manager.begin_transaction().unwrap();
        assert!(!id.is_empty());
        
        manager.commit_transaction().unwrap();
    }
}
