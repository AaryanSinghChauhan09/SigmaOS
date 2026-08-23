extern crate alloc;
use alloc::vec::Vec;
/// Chimera Linux Compatibility and Subsystem Layer for SigmaOS
/// Replicates Chimera's signature modern features:
/// Dinit Service Manager, BSD-userland/chimerautils, and apk-tools database compatibility.
extern crate alloc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DinitServiceState {
    Stopped,
    Starting,
    Started,
    Stopping,
    Failed,
}

#[derive(Debug, Clone)]
pub struct DinitService {
    pub name: [u8; 32],
    pub state: DinitServiceState,
    pub dependencies: Vec<[u8; 32]>,
}

impl DinitService {
    pub fn new(name: &[u8]) -> Self {
        let mut name_arr = [0u8; 32];
        name_arr[..name.len().min(31)].copy_from_slice(&name[..name.len().min(31)]);
        DinitService {
            name: name_arr,
            state: DinitServiceState::Stopped,
            dependencies: Vec::new(),
        }
    }

    pub fn add_dependency(&mut self, dep: &[u8]) {
        let mut dep_arr = [0u8; 32];
        dep_arr[..dep.len().min(31)].copy_from_slice(&dep[..dep.len().min(31)]);
        self.dependencies.push(dep_arr);
    }
}

/// dinit-chimera service manager simulation
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
        DinitServiceManager {
            services: Vec::new(),
            running_count: AtomicUsize::new(0),
        }
    }

    pub fn register_service(&mut self, svc: DinitService) {
        self.services.push(svc);
    }

    pub fn start_service(&mut self, name: &[u8]) -> Result<(), &'static str> {
        let mut name_arr = [0u8; 32];
        name_arr[..name.len().min(31)].copy_from_slice(&name[..name.len().min(31)]);

        let mut found_idx = None;
        for (i, svc) in self.services.iter().enumerate() {
            if svc.name == name_arr {
                found_idx = Some(i);
                break;
            }
        }

        let idx = found_idx.ok_or("Service not found in dinit database")?;

        if self.services[idx].state == DinitServiceState::Started {
            return Ok(());
        }

        self.services[idx].state = DinitServiceState::Starting;

        // Recursively start dependencies first (Dinit logic)
        let deps = self.services[idx].dependencies.clone();
        for dep in &deps {
            let dep_name = &dep[..dep.iter().position(|b: &u8| *b == 0).unwrap_or(32)];
            self.start_service(dep_name)?;
        }

        self.services[idx].state = DinitServiceState::Started;
        self.running_count.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
}

/// BSD chimerautils / userland core utilities compatibility layer
pub struct BsdUserlandCompat;

impl BsdUserlandCompat {
    pub fn translate_bsd_df_output(&self, total: usize, used: usize) -> (usize, usize) {
        // BSD df reports blocks, we translate to standardized byte structures
        let block_size = 512;
        (total * block_size, used * block_size)
    }

    pub fn pgrep_filter_by_name(&self, processes: &[(&[u8], usize)], needle: &[u8]) -> Vec<usize> {
        let mut matched_pids = Vec::new();
        for &(name, pid) in processes {
            if name.windows(needle.len()).any(|w| w == needle) {
                matched_pids.push(pid);
            }
        }
        matched_pids
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

#[cfg(test)]
mod tests {
    use super::*;

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
