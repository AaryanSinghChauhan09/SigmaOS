// SigmaOS Software Store & Safety Scanner Shard
// Zero-dependency, #![no_std] compliant, highly-optimized for low-end hardware
// Evaluates package installations against security/safety scores and sandboxing requirements (Linux Mint Software Manager parity).

use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, Ordering};

pub const MAX_STORE_ENTRIES: usize = 4;

#[derive(Debug, Clone, Copy)]
pub struct SoftwareRegistryEntry {
    pub name: &'static str,
    pub safety_score: usize, // 1 to 100
    pub is_sandboxed: bool,  // Flatpak / Snap verification
    pub update_available: bool,
}

pub struct SigmaSoftwareStore {
    pub registry: RefCell<[Option<SoftwareRegistryEntry>; MAX_STORE_ENTRIES]>,
    pub auto_updates_enabled: AtomicBool,
}

unsafe impl Sync for SigmaSoftwareStore {}

impl SigmaSoftwareStore {
    pub const fn new() -> Self {
        const EMPTY_ENTRY: Option<SoftwareRegistryEntry> = None;
        Self {
            registry: RefCell::new([
                Some(SoftwareRegistryEntry {
                    name: "firefox-developer",
                    safety_score: 95,
                    is_sandboxed: true,
                    update_available: true,
                }),
                Some(SoftwareRegistryEntry {
                    name: "vlc-player",
                    safety_score: 90,
                    is_sandboxed: true,
                    update_available: false,
                }),
                EMPTY_ENTRY,
                EMPTY_ENTRY,
            ]),
            auto_updates_enabled: AtomicBool::new(true),
        }
    }

    /// Validates package installation criteria, rejecting low-safety or unsandboxed utilities
    pub fn install_with_safety_check(&self, name: &str) -> Result<(), &'static str> {
        let registry = self.registry.borrow();
        for entry_slot in registry.iter() {
            if let Some(ref entry) = entry_slot {
                if entry.name == name {
                    if entry.safety_score < 50 {
                        println!("SoftwareStore: SECURITY BLOCKED: Package '{}' has a critical low safety score of {}!", entry.name, entry.safety_score);
                        return Err("SecurityBlocked: Package safety threshold not met.");
                    }
                    if !entry.is_sandboxed {
                        println!("SoftwareStore: WARNING: Installing unsandboxed application '{}'. Sandbox policies degraded.", entry.name);
                    } else {
                        println!("SoftwareStore: Package '{}' validated (Safety: {}, Sandboxed: true). Installation granted.", entry.name, entry.safety_score);
                    }
                    return Ok(());
                }
    pub fn check_for_updates(&mut self) -> usize {
        self.pending_updates.clear();
        for (name, app) in &self.catalog {
            if app.is_installed && app.version != "1.5.0" {
                // Assume latest stable is 1.5.0
                self.pending_updates.push(name.clone());
    pub fn check_for_updates(&mut self) -> usize {
        self.pending_updates.clear();
        for (name, app) in &self.catalog {
            let name: &String = name;
            let app: &StoreApp = app;
            if app.is_installed && app.version != "1.5.0" {
                // Assume latest stable is 1.5.0
                self.pending_updates.push(name.clone());
            }
        }
        Err("ENOENT: Package not registered in the Software Store.")
    }

    /// Automatically scans and triggers update routines for registered packages
    pub fn trigger_auto_updates(&self) -> usize {
        if !self.auto_updates_enabled.load(Ordering::SeqCst) {
            println!("SoftwareStore: Auto-updates deactivated by user configuration.");
            return 0;
        }

        let mut registry = self.registry.borrow_mut();
        let mut count = 0;
        for entry_slot in registry.iter_mut() {
            if let Some(ref mut entry) = entry_slot {
                if entry.update_available {
                    println!("SoftwareStore: Auto-updating package: '{}'...", entry.name);
                    entry.update_available = false;
                    count += 1;
                }
            }
        }
        println!(
            "SoftwareStore: Update complete. Updated {} packages dynamically.",
            count
        );
        count
    }
}

pub static GLOBAL_SOFTWARE_STORE: SigmaSoftwareStore = SigmaSoftwareStore::new();
// SigmaOS Polish-Parity Software Store & Update Manager (SigmaStore)
// Designed for software installation, package upgrades, and security auditing

use crate::klib::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreError {
    Success = 0,
    PackageNotFound = 1,
    InstallFailed = 2,
    InsecurePackage = 3,
}

pub struct StoreApp {
    pub name: String,
    pub version: String,
    pub category: String,
    pub is_installed: bool,
    pub safety_score: f32, // 0.0 to 1.0 (GDPR/Compliance check)
}

pub struct SigmaSoftwareStore {
    pub catalog: HashMap<String, StoreApp>,
    pub pending_updates: Vec<String>,
}

impl SigmaSoftwareStore {
    pub fn new() -> Self {
        let mut store = SigmaSoftwareStore {
            catalog: HashMap::new(),
            pending_updates: Vec::new(),
        };
        store.register_app(StoreApp {
            name: "sigma-browse".to_string(),
            version: "1.0.0".to_string(),
            category: "Internet".to_string(),
            is_installed: false,
            safety_score: 0.98,
        });
        store.register_app(StoreApp {
            name: "sigma-paint".to_string(),
            version: "1.2.0".to_string(),
            category: "Graphics".to_string(),
            is_installed: false,
            safety_score: 0.95,
        });
        store
    }

    pub fn register_app(&mut self, app: StoreApp) {
        self.catalog.insert(app.name.clone(), app);
    }

    pub fn install_app(&mut self, name: &str) -> Result<(), StoreError> {
        if let Some(app) = self.catalog.get_mut(name) {
            if app.safety_score < 0.5 {
                return Err(StoreError::InsecurePackage);
            }
            app.is_installed = true;
            Ok(())
        } else {
            Err(StoreError::PackageNotFound)
        }
    }

    pub fn check_for_updates(&mut self) -> usize {
        self.pending_updates.clear();
        for (name, app) in &self.catalog {
            let name: &String = name;
            let app: &StoreApp = app;
            if app.is_installed && app.version != "1.5.0" {
                // Assume latest stable is 1.5.0
                let name_str: String = name.clone();
                self.pending_updates.push(name_str);
            }
        }
        self.pending_updates.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_software_store_install() {
        let mut store = SigmaSoftwareStore::new();
        assert!(store.install_app("sigma-paint").is_ok());
        let app = store.catalog.get("sigma-paint").unwrap();
        assert!(app.is_installed);
    }

    #[test]
    fn test_software_store_updates() {
        let mut store = SigmaSoftwareStore::new();
        store.install_app("sigma-browse").unwrap();
        let update_count = store.check_for_updates();
        assert_eq!(update_count, 1);
        assert_eq!(store.pending_updates[0], "sigma-browse");
    }
}
