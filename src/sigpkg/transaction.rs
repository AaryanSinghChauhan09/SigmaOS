#![allow(clippy::large_enum_variant)]

// Transaction Manager for SigmaPkg
// Atomic package installation and rollback

#[cfg(not(feature = "standalone_test"))]
use crate::sigpkg::{ContentAddressedStore, Package, SatSolver};

#[cfg(feature = "standalone_test")]
#[derive(Debug, Clone)]
pub struct ContentAddressedStore {
    pub path: std::path::PathBuf,
}
#[cfg(feature = "standalone_test")]
impl ContentAddressedStore {
    pub fn new(path: std::path::PathBuf) -> Self {
        Self { path }
    }
    pub fn get(&self, _name: &str) -> Option<Package> {
        None
    }
}

#[cfg(feature = "standalone_test")]
#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: (u64, u64, u64),
}

#[cfg(feature = "standalone_test")]
impl Package {
    pub fn new(
        name: String,
        version: (u64, u64, u64),
        _s1: String,
        _v: Vec<()>,
        _s2: String,
    ) -> Self {
        Self { name, version }
    }
}

#[cfg(feature = "standalone_test")]
pub struct SatSolver;
#[cfg(feature = "standalone_test")]
impl SatSolver {
    pub fn new() -> Self {
        Self
    }
    pub fn resolve(&self, _name: &str, _c: &Constraint) -> Result<Vec<Package>, ResolveError> {
        Err(ResolveError::PackageNotFound(_name.to_string()))
    }
}

#[cfg(feature = "standalone_test")]
#[derive(Debug, Clone)]
pub enum Constraint {
    Any,
}

#[cfg(feature = "standalone_test")]
#[derive(Debug, Clone)]
pub enum ResolveError {
    PackageNotFound(String),
}

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
        #[cfg(not(feature = "standalone_test"))]
        let constraint = &crate::sigpkg::VersionConstraint::Any;
        #[cfg(feature = "standalone_test")]
        let constraint = &Constraint::Any;

        let resolved = self
            .resolver
            .resolve(&package.name, constraint)
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
                    println!("Updating: {} -> {:?}", old.name, new.version);
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

/// Package Snapshot representing a point-in-time state for system rollback
#[derive(Debug, Clone)]
pub struct PackageSnapshot {
    pub snapshot_id: u64,
    pub description: String,
    pub installed_packages: Vec<(String, String)>, // (Package Name, Version String)
    pub timestamp_ms: u64,
}

/// Package Snapshot Rollback Engine (Btrfs / ZFS inspired atomic generation rollback)
pub struct PackageSnapshotRollbackEngine {
    pub snapshots: Vec<PackageSnapshot>,
    pub next_id: u64,
}

impl PackageSnapshotRollbackEngine {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
            next_id: 1,
        }
    }

    pub fn create_pre_transaction_snapshot(
        &mut self,
        description: &str,
        current_packages: &[(&str, &str)],
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let installed_packages = current_packages
            .iter()
            .map(|(n, v)| (n.to_string(), v.to_string()))
            .collect();

        let snapshot = PackageSnapshot {
            snapshot_id: id,
            description: description.to_string(),
            installed_packages,
            timestamp_ms: 1000 + id * 100,
        };

        self.snapshots.push(snapshot);
        id
    }

    pub fn get_snapshot(&self, snapshot_id: u64) -> Option<&PackageSnapshot> {
        self.snapshots.iter().find(|s| s.snapshot_id == snapshot_id)
    }

    pub fn compute_rollback_diff(
        &self,
        current_packages: &[(&str, &str)],
        target_snapshot_id: u64,
    ) -> Result<(Vec<(String, String)>, Vec<String>), TransactionError> {
        let snapshot = self
            .get_snapshot(target_snapshot_id)
            .ok_or(TransactionError::RollbackFailed)?;

        let mut to_restore = Vec::new(); // Packages to install/revert
        let mut to_remove = Vec::new(); // Packages installed after snapshot to remove

        // Find packages in snapshot that are missing or mismatched in current
        for (snap_name, snap_ver) in &snapshot.installed_packages {
            let found = current_packages.iter().find(|(cn, _)| cn == snap_name);
            match found {
                Some((_, cv)) if cv != snap_ver => {
                    to_restore.push((snap_name.clone(), snap_ver.clone()));
                }
                None => {
                    to_restore.push((snap_name.clone(), snap_ver.clone()));
                }
                _ => {}
            }
        }

        // Find packages in current that were not in snapshot
        for (cur_name, _) in current_packages {
            if !snapshot
                .installed_packages
                .iter()
                .any(|(sn, _)| sn == cur_name)
            {
                to_remove.push(cur_name.to_string());
            }
        }

        Ok((to_restore, to_remove))
    }

    pub fn rollback_to_snapshot(
        &mut self,
        current_packages: &mut Vec<(String, String)>,
        target_snapshot_id: u64,
    ) -> Result<(), TransactionError> {
        let (to_restore, to_remove) = self.compute_rollback_diff(
            &current_packages
                .iter()
                .map(|(n, v)| (n.as_str(), v.as_str()))
                .collect::<Vec<_>>(),
            target_snapshot_id,
        )?;

        // Apply removals
        current_packages.retain(|(n, _)| !to_remove.contains(n));

        // Apply restorations/reverts
        for (res_name, res_ver) in to_restore {
            if let Some(existing) = current_packages.iter_mut().find(|(n, _)| n == &res_name) {
                existing.1 = res_ver;
            } else {
                current_packages.push((res_name, res_ver));
            }
        }

        Ok(())
    }
}

impl Default for PackageSnapshotRollbackEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Transaction errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionError {
    PackageNotFound(String),
    DependencyConflict(String),
    RollbackFailed,
}

#[cfg(not(feature = "standalone_test"))]
impl From<crate::sigpkg::resolver::ResolveError> for TransactionError {
    fn from(err: crate::sigpkg::resolver::ResolveError) -> Self {
        match err {
            crate::sigpkg::resolver::ResolveError::PackageNotFound(name) => {
                TransactionError::PackageNotFound(name)
            }
            crate::sigpkg::resolver::ResolveError::NoMatchingVersion(name) => {
                TransactionError::DependencyConflict(name)
            }
            crate::sigpkg::resolver::ResolveError::CircularDependency(name) => {
                TransactionError::DependencyConflict(name)
            }
            crate::sigpkg::resolver::ResolveError::Conflict(name) => {
                TransactionError::DependencyConflict(name)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::klib::custom_string::SigmaString;
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

        #[cfg(not(feature = "standalone_test"))]
        let version = crate::sigpkg::Version::new(1, 0, 0);
        #[cfg(feature = "standalone_test")]
        let version = (1, 0, 0);

        let package = Package::new(
            "test".to_string(),
            version,
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

    #[test]
    fn test_package_snapshot_rollback_engine() {
        let mut rollback_engine = PackageSnapshotRollbackEngine::new();

        // System initial state
        let mut current_pkgs = vec![
            ("glibc".to_string(), "2.38".to_string()),
            ("bash".to_string(), "5.2".to_string()),
        ];

        // Create pre-transaction snapshot #1
        let snap_id = rollback_engine.create_pre_transaction_snapshot(
            "Before installing nginx",
            &[("glibc", "2.38"), ("bash", "5.2")],
        );
        assert_eq!(snap_id, 1);

        // Modify system: update bash to 5.3, add nginx 1.24
        current_pkgs[1].1 = "5.3".to_string();
        current_pkgs.push(("nginx".to_string(), "1.24".to_string()));

        // Perform atomic rollback to snapshot #1
        assert!(rollback_engine
            .rollback_to_snapshot(&mut current_pkgs, snap_id)
            .is_ok());

        // Verify state is restored to pre-transaction snapshot exactly
        assert_eq!(current_pkgs.len(), 2);
        assert_eq!(current_pkgs[0], ("glibc".to_string(), "2.38".to_string()));
        assert_eq!(current_pkgs[1], ("bash".to_string(), "5.2".to_string()));
    }
}
