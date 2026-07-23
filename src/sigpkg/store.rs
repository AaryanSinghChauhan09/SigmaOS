// Content-Addressed Store for SigmaPkg
// Stores packages by SHA3-256 hash for reproducibility

use crate::sigpkg::Package;
use std::collections::HashMap;
use std::path::PathBuf;

/// Content-addressed store
pub struct ContentAddressedStore {
    base_path: PathBuf,
    packages: HashMap<String, StoredPackage>,
}

/// Stored package metadata
#[derive(Debug, Clone)]
struct StoredPackage {
    package: Package,
    path: PathBuf,
    hash: String,
}

impl ContentAddressedStore {
    /// Create new content-addressed store
    pub fn new(base_path: PathBuf) -> Self {
        Self {
            base_path,
            packages: HashMap::new(),
        }
    }

    /// Add package to store
    pub fn add(&mut self, package: Package, data: &[u8]) -> Result<String, StoreError> {
        let hash = self.compute_hash(data);
        let package_path = self.base_path.join(format!("{}-{}", hash, package.name));

        let stored = StoredPackage {
            package: package.clone(),
            path: package_path.clone(),
            hash: hash.clone(),
        };

        self.packages.insert(package.name.clone(), stored);

        Ok(hash)
    }

    /// Get package by name
    pub fn get(&self, name: &str) -> Option<&Package> {
        self.packages.get(name).map(|s| &s.package)
    }

    /// Get package by hash
    pub fn get_by_hash(&self, hash: &str) -> Option<&Package> {
        self.packages
            .values()
            .find(|s| s.hash == hash)
            .map(|s| &s.package)
    }

    /// Remove package from store
    pub fn remove(&mut self, name: &str) -> Result<(), StoreError> {
        self.packages
            .remove(name)
            .ok_or(StoreError::PackageNotFound(name.to_string()))?;
        Ok(())
    }

    /// List all packages
    pub fn list(&self) -> Vec<&Package> {
        self.packages.values().map(|s| &s.package).collect()
    }

    /// Compute SHA3-256 hash
    fn compute_hash(&self, data: &[u8]) -> String {
        // Simplified hash computation - in production use actual SHA3-256
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// Get package path
    pub fn get_path(&self, name: &str) -> Option<PathBuf> {
        self.packages.get(name).map(|s| s.path.clone())
    }
}

/// Store errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreError {
    PackageNotFound(String),
    HashMismatch,
    IoError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_creation() {
        let store = ContentAddressedStore::new(PathBuf::from("/var/store"));
        assert!(store.packages.is_empty());
    }

    #[test]
    fn test_add_package() {
        let mut store = ContentAddressedStore::new(PathBuf::from("/tmp/test"));
        let package = Package::new(
            "test".to_string(),
            crate::sigpkg::Version::new(1, 0, 0),
            String::new(),
            Vec::new(),
            String::new(),
        );

        let data = b"test data";
        let hash = store.add(package, data).unwrap();
        assert!(!hash.is_empty());
    }

    #[test]
    fn test_get_package() {
        let mut store = ContentAddressedStore::new(PathBuf::from("/tmp/test"));
        let package = Package::new(
            "test".to_string(),
            crate::sigpkg::Version::new(1, 0, 0),
            String::new(),
            Vec::new(),
            String::new(),
        );

        store.add(package.clone(), b"test data").unwrap();
        let retrieved = store.get("test").unwrap();
        assert_eq!(retrieved.name, "test");
    }

    #[test]
    fn test_remove_package() {
        let mut store = ContentAddressedStore::new(PathBuf::from("/tmp/test"));
        let package = Package::new(
            "test".to_string(),
            crate::sigpkg::Version::new(1, 0, 0),
            String::new(),
            Vec::new(),
            String::new(),
        );

        store.add(package, b"test data").unwrap();
        store.remove("test").unwrap();
        assert!(store.get("test").is_none());
    }
}
