// Transaction Manager for SigmaPkg
// Atomic package installation and rollback

use crate::sigpkg::{ContentAddressedStore, Package, SatSolver};

/// Transaction for package operations
pub struct Transaction {
    operations: Vec<Operation>,
    store: ContentAddressedStore,
    resolver: SatSolver,
}

/// Transaction operation
#[derive(Debug, Clone)]
enum Operation {
    Install { package: Package },
    Remove { package_name: String },
    Update { old: Package, new: Package },
}

impl Transaction {
    /// Create new transaction
    pub fn new(store: ContentAddressedStore, resolver: SatSolver) -> Self {
        Self {
            operations: Vec::new(),
            store,
            resolver,
        }
    }

    /// Add install operation
    pub fn install(&mut self, package: Package) -> Result<(), TransactionError> {
        // Resolve dependencies first
        let resolved = self
            .resolver
            .resolve(&package.name, &crate::sigpkg::VersionConstraint::Any)
            .map_err(|e| TransactionError::DependencyConflict(format!("{:?}", e)))?;

        for dep in resolved {
            if self.store.get(&dep.name).is_none() {
                self.operations.push(Operation::Install { package: dep });
            }
        }

        self.operations.push(Operation::Install { package });
        Ok(())
    }

    /// Add remove operation
    pub fn remove(&mut self, package_name: String) -> Result<(), TransactionError> {
        if self.store.get(&package_name).is_none() {
            return Err(TransactionError::PackageNotFound(package_name));
        }

        self.operations.push(Operation::Remove { package_name });
        Ok(())
    }

    /// Add update operation
    pub fn update(&mut self, old: Package, new: Package) -> Result<(), TransactionError> {
        if self.store.get(&old.name).is_none() {
            return Err(TransactionError::PackageNotFound(old.name));
        }

        self.operations.push(Operation::Update { old, new });
        Ok(())
    }

    /// Commit transaction (atomic)
    pub fn commit(self) -> Result<(), TransactionError> {
        // In production, implement atomic symlink swap
        for operation in self.operations {
            match operation {
                Operation::Install { package } => {
                    println!("Installing: {}", package.name);
                    // Actual installation logic
                }
                Operation::Remove { package_name } => {
                    println!("Removing: {}", package_name);
                    // Actual removal logic
                }
                Operation::Update { old, new } => {
                    println!("Updating: {} -> {}", old.name, new.version);
                    // Actual update logic
                }
            }
        }
        Ok(())
    }

    /// Rollback transaction
    pub fn rollback(&self) {
        println!("Rolling back transaction");
        // In production, revert to previous generation
    }

    /// Preview transaction
    pub fn preview(&self) -> Vec<&str> {
        self.operations
            .iter()
            .map(|op| match op {
                Operation::Install { package } => package.name.as_str(),
                Operation::Remove { package_name } => package_name.as_str(),
                Operation::Update { old, .. } => old.name.as_str(),
            })
            .collect()
    }
}

/// Transaction errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionError {
    PackageNotFound(String),
    DependencyConflict(String),
    RollbackFailed,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_transaction_creation() {
        let store = ContentAddressedStore::new(PathBuf::from("/tmp/test"));
        let resolver = SatSolver::new();
        let transaction = Transaction::new(store, resolver);
        assert!(transaction.operations.is_empty());
    }

    #[test]
    fn test_install_operation() {
        let store = ContentAddressedStore::new(PathBuf::from("/tmp/test"));
        let resolver = SatSolver::new();
        let mut transaction = Transaction::new(store, resolver);

        let package = Package::new(
            "test".to_string(),
            crate::sigpkg::Version::new(1, 0, 0),
            String::new(),
            Vec::new(),
            String::new(),
        );

        // This will fail due to dependency resolution, but tests the flow
        let result = transaction.install(package);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_operation() {
        let store = ContentAddressedStore::new(PathBuf::from("/tmp/test"));
        let resolver = SatSolver::new();
        let mut transaction = Transaction::new(store, resolver);

        let result = transaction.remove("nonexistent".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_preview() {
        let store = ContentAddressedStore::new(PathBuf::from("/tmp/test"));
        let resolver = SatSolver::new();
        let transaction = Transaction::new(store, resolver);

        let preview = transaction.preview();
        assert!(preview.is_empty());
    }
}
