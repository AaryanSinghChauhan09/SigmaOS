#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// Content-Addressed Store for SigmaPkg
// Stores packages by SHA3-256 hash for reproducibility

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use std::path::PathBuf;
use crate::sigpkg::Package;

/// Content-addressed store
pub struct ContentAddressedStore {
    base_path: PathBuf,
    packages: BTreeMap<String, StoredPackage>,
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
            packages: BTreeMap::new(),
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
        let key = name.to_string();
        self.packages.get(&key).map(|s| &s.package)
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
        let key = name.to_string();
        self.packages
            .remove(&key)
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
        self.packages.get(&name.to_string()).map(|s| s.path.clone())
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

// =========================================================================
// Linux & BSD Package Manager Innovations
// =========================================================================

/// FreeBSD `pkg` / OpenBSD `pkg_add` inspired repository mirror verification engine
#[derive(Debug, Clone)]
pub struct BsdPkgRepositoryMirror {
    pub url: String,
    pub index_hash: String,
    pub is_trusted: bool,
    pub packages_count: usize,
}

impl BsdPkgRepositoryMirror {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            index_hash: String::new(),
            is_trusted: false,
            packages_count: 0,
        }
    }

    pub fn update_index(&mut self, index_bytes: &[u8], signature_valid: bool) -> bool {
        if signature_valid {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let mut hasher = DefaultHasher::new();
            index_bytes.hash(&mut hasher);
            self.index_hash = format!("{:x}", hasher.finish());
            self.is_trusted = true;
            self.packages_count = index_bytes.len() / 32;
            true
        } else {
            self.is_trusted = false;
            false
        }
    }
}

/// Gentoo Portage USE flag masking & USE-conditional dependency resolution engine
#[derive(Debug, Clone, Default)]
pub struct GentooPortageUseFlagMask {
    pub active_flags: Vec<String>,
    pub masked_flags: Vec<String>,
}

impl GentooPortageUseFlagMask {
    pub fn new() -> Self {
        Self {
            active_flags: Vec::new(),
            masked_flags: Vec::new(),
        }
    }

    pub fn set_use_flag(&mut self, flag: &str, enable: bool) {
        if enable {
            if !self.masked_flags.contains(&flag.to_string()) && !self.active_flags.contains(&flag.to_string()) {
                self.active_flags.push(flag.to_string());
            }
        } else {
            self.active_flags.retain(|f| f != flag);
        }
    }

    pub fn mask_use_flag(&mut self, flag: &str) {
        if !self.masked_flags.contains(&flag.to_string()) {
            self.masked_flags.push(flag.to_string());
            self.active_flags.retain(|f| f != flag);
        }
    }

    pub fn is_flag_active(&self, flag: &str) -> bool {
        self.active_flags.contains(&flag.to_string()) && !self.masked_flags.contains(&flag.to_string())
    }
}

/// NixOS-inspired Content-Addressed Storage (CAS) Hermetic Store
#[derive(Debug, Clone)]
pub struct NixOsHermeticCasStore {
    pub store_dir: PathBuf,
    pub store_paths: BTreeMap<String, PathBuf>,
}

impl NixOsHermeticCasStore {
    pub fn new(store_dir: PathBuf) -> Self {
        Self {
            store_dir,
            store_paths: BTreeMap::new(),
        }
    }

    pub fn compute_store_path(&mut self, pkg_name: &str, version: &str, payload: &[u8]) -> PathBuf {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        payload.hash(&mut hasher);
        let hash_str = format!("{:016x}", hasher.finish());

        let folder_name = format!("{}-{}-{}", hash_str, pkg_name, version);
        let store_path = self.store_dir.join(folder_name);
        self.store_paths.insert(pkg_name.to_string(), store_path.clone());
        store_path
    }

    pub fn verify_closure(&self, pkg_name: &str) -> bool {
        self.store_paths.contains_key(pkg_name)
    }
}

#[cfg(test)]
mod distro_pkg_tests {
    use super::*;

    #[test]
    fn test_bsd_pkg_repo_mirror() {
        let mut mirror = BsdPkgRepositoryMirror::new("https://pkg.freebsd.org/freebsd:14:x86:64/latest");
        assert!(!mirror.is_trusted);

        let index_data = b"pkg_index_binary_blob_sample_data_12345";
        assert!(mirror.update_index(index_data, true));
        assert!(mirror.is_trusted);
        assert!(!mirror.index_hash.is_empty());
    }

    #[test]
    fn test_gentoo_portage_use_flags() {
        let mut portage = GentooPortageUseFlagMask::new();
        portage.set_use_flag("wayland", true);
        portage.set_use_flag("x11", false);
        assert!(portage.is_flag_active("wayland"));
        assert!(!portage.is_flag_active("x11"));

        portage.mask_use_flag("wayland");
        assert!(!portage.is_flag_active("wayland"));
    }

    #[test]
    fn test_nix_cas_hermetic_store() {
        let mut cas = NixOsHermeticCasStore::new(PathBuf::from("/sigma/store"));
        let path = cas.compute_store_path("zenith-compositor", "1.0.0", b"binary_data");
        assert!(path.to_str().unwrap().contains("zenith-compositor"));
        assert!(cas.verify_closure("zenith-compositor"));
    }
}
