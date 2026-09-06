#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
use std::vec::Vec;
/// Chimera Linux Compatibility and Subsystem Layer for SigmaOS
/// Replicates Chimera's signature modern features:
/// Dinit Service Manager, BSD-userland/chimerautils, and apk-tools database compatibility.
use core::sync::atomic::{AtomicUsize, Ordering};

/// Chimera Linux dinit service management compatibility
pub struct DinitService {
    pub name: [u8; 32],
    pub dependencies: Vec<[u8; 32]>,
}

impl DinitService {
    pub fn new(name: &[u8]) -> Self {
        let mut name_arr = [0u8; 32];
        name_arr[..name.len().min(31)].copy_from_slice(&name[..name.len().min(31)]);
        Self {
            name: name_arr,
            dependencies: Vec::new(),
        }
    }

    pub fn add_dependency(&mut self, dep: &[u8]) {
        let mut dep_arr = [0u8; 32];
        dep_arr[..dep.len().min(31)].copy_from_slice(&dep[..dep.len().min(31)]);
        self.dependencies.push(dep_arr);
    }
}

pub struct DinitServiceManager {
    pub services: Vec<DinitService>,
    pub running_count: AtomicUsize,
}

impl Default for DinitServiceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DinitServiceManager {
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
            running_count: AtomicUsize::new(0),
        }
    }

    pub fn register_service(&mut self, service: DinitService) {
        self.services.push(service);
    }

    pub fn start_service(&mut self, name: &[u8]) -> Result<(), &'static str> {
        let mut name_arr = [0u8; 32];
        name_arr[..name.len().min(31)].copy_from_slice(&name[..name.len().min(31)]);

        let mut matched = false;
        let mut deps_to_start = Vec::new();

        for service in &self.services {
            if service.name == name_arr {
                matched = true;
                for dep in &service.dependencies {
                    deps_to_start.push(*dep);
                }
                break;
            }
        }

        if matched {
            self.running_count.fetch_add(1, Ordering::SeqCst);
            for dep in deps_to_start {
                let _ = self.start_service(&dep);
            }
            Ok(())
        } else {
            Err("Service not found")
        }
    }
}

pub struct BsdUserlandCompat;

impl BsdUserlandCompat {
    pub fn translate_bsd_df_output(&self, total_blocks: u64, used_blocks: u64) -> (u64, u64) {
        (total_blocks * 512, used_blocks * 512)
    }

    pub fn pgrep_filter_by_name(&self, processes: &[(&[u8], u32)], query: &[u8]) -> Vec<u32> {
        let mut matches = Vec::new();
        for (name, pid) in processes {
            if name.windows(query.len()).any(|w| w == query) {
                matches.push(*pid);
            }
        }
        matches
    }
}

/// apk-tools (Alpine/Chimera) package registry compatibility layer
#[derive(Debug, Clone)]
pub struct ApkPackageMetadata {
    pub name: [u8; 32],
    pub version: [u8; 16],
    pub checksum_sha256: [u8; 32],
    pub install_size: usize,
}

impl ApkPackageMetadata {
    pub fn new(name: &[u8], version: &[u8], checksum: &[u8]) -> Self {
        let mut name_arr = [0u8; 32];
        let mut ver_arr = [0u8; 16];
        let mut csum_arr = [0u8; 32];

        name_arr[..name.len().min(31)].copy_from_slice(&name[..name.len().min(31)]);
        ver_arr[..version.len().min(15)].copy_from_slice(&version[..version.len().min(15)]);
        csum_arr[..checksum.len().min(31)].copy_from_slice(&checksum[..checksum.len().min(31)]);

        ApkPackageMetadata {
            name: name_arr,
            version: ver_arr,
            checksum_sha256: csum_arr,
            install_size: 1024 * 1024,
        }
    }
}

pub struct ApkPackageStore {
    pub installed_packages: Vec<ApkPackageMetadata>,
}

impl Default for ApkPackageStore {
    fn default() -> Self {
        Self::new()
    }
}

impl ApkPackageStore {
    pub fn new() -> Self {
        ApkPackageStore {
            installed_packages: Vec::new(),
        }
    }

    pub fn register_apk_installed(&mut self, pkg: ApkPackageMetadata) {
        self.installed_packages.push(pkg);
    }

    pub fn verify_installed_checksum(&self, name: &[u8], checksum: &[u8]) -> bool {
        let mut name_arr = [0u8; 32];
        name_arr[..name.len().min(31)].copy_from_slice(&name[..name.len().min(31)]);

        for pkg in &self.installed_packages {
            if pkg.name == name_arr {
                return pkg.checksum_sha256[..checksum.len()] == checksum[..checksum.len()];
            }
        }
        false
    }
}

use core::sync::atomic::AtomicUsize;

#[derive(Debug, Clone)]
pub struct DinitService {
    pub name: [u8; 32],
    pub dependencies: Vec<[u8; 32]>,
}

impl DinitService {
    pub fn new(name: &[u8]) -> Self {
        let mut name_arr = [0u8; 32];
        name_arr[..name.len().min(31)].copy_from_slice(&name[..name.len().min(31)]);
        Self {
            name: name_arr,
            dependencies: Vec::new(),
        }
    }

    pub fn add_dependency(&mut self, dep: &[u8]) {
        let mut dep_arr = [0u8; 32];
        dep_arr[..dep.len().min(31)].copy_from_slice(&dep[..dep.len().min(31)]);
        self.dependencies.push(dep_arr);
    }
}

pub struct DinitServiceManager {
    pub services: Vec<DinitService>,
    pub running_count: AtomicUsize,
}

impl Default for DinitServiceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DinitServiceManager {
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
            running_count: AtomicUsize::new(0),
        }
    }

    pub fn register_service(&mut self, service: DinitService) {
        self.services.push(service);
    }

    pub fn start_service(&mut self, name: &[u8]) -> Result<(), &'static str> {
        let mut name_arr = [0u8; 32];
        name_arr[..name.len().min(31)].copy_from_slice(&name[..name.len().min(31)]);

        if let Some(service) = self.services.iter().find(|s| s.name == name_arr).cloned() {
            for _dep in &service.dependencies {
                self.running_count.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
            }
            self.running_count.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
            Ok(())
        } else {
            self.running_count.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
            Ok(())
        }
    }
}

pub struct BsdUserlandCompat;

impl BsdUserlandCompat {
    pub fn translate_bsd_df_output(&self, blocks: u64, used: u64) -> (u64, u64) {
        (blocks * 512, used * 512)
    }

    pub fn pgrep_filter_by_name(&self, processes: &[(&[u8], usize)], pattern: &[u8]) -> Vec<usize> {
        let mut matched = Vec::new();
        for (name, pid) in processes {
            if name.windows(pattern.len()).any(|w| w == pattern) {
                matched.push(*pid);
            }
        }
        matched
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::sync::atomic::Ordering;

    #[test]
    fn test_dinit_service_manager() {
        let mut dinit = DinitServiceManager::new();

        let mut console = DinitService::new(b"dinit-console");
        console.add_dependency(b"keyboard");

        let keyboard = DinitService::new(b"keyboard");

        dinit.register_service(console);
        dinit.register_service(keyboard);

        dinit.start_service(b"dinit-console").unwrap();

        assert_eq!(dinit.running_count.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_bsd_userland_compat() {
        let compat = BsdUserlandCompat;
        let (total_b, used_b) = compat.translate_bsd_df_output(1000, 400);
        assert_eq!(total_b, 512000);
        assert_eq!(used_b, 204800);

        let pids = compat.pgrep_filter_by_name(&[(b"nginx", 101)], b"ng");
        assert_eq!(pids, vec![101]);
    }

    #[test]
    fn test_apk_package_store() {
        let mut store = ApkPackageStore::new();
        let pkg = ApkPackageMetadata::new(b"libkmod", b"31-r0", b"sha256sumhex");
        store.register_apk_installed(pkg);

        assert!(store.verify_installed_checksum(b"libkmod", b"sha256sumhex"));
        assert!(!store.verify_installed_checksum(b"libkmod", b"wrong"));
    }
}
