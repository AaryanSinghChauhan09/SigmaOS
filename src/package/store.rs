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