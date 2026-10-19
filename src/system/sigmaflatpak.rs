//! SigmaFlatpak - Flatpak Package Management
//!
//! Inspired by Flatpak, this module provides Flatpak package management including
//! sandboxed applications, runtime management, and Flatpak repository support.

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, Ordering};

/// Flatpak type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlatpakType {
    App,
    Runtime,
    Sdk,
    Extension,
    Unknown,
}

/// Flatpak state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlatpakState {
    Installed,
    NotInstalled,
    UpdateAvailable,
    Unknown,
}

/// Flatpak data structure
#[derive(Debug, Clone)]
pub struct Flatpak {
    pub flatpak_id: String,
    pub name: String,
    pub app_id: String,
    pub branch: String,
    pub arch: String,
    pub flatpak_type: FlatpakType,
    pub state: FlatpakState,
    pub size: u64,
}

impl Flatpak {
    /// Create a new Flatpak
    pub fn new(
        flatpak_id: String,
        name: String,
        app_id: String,
        branch: String,
        arch: String,
        flatpak_type: FlatpakType,
    ) -> Self {
        Self {
            flatpak_id,
            name,
            app_id,
            branch,
            arch,
            flatpak_type,
            state: FlatpakState::NotInstalled,
            size: 0,
        }
    }

    /// Set state
    pub fn set_state(&mut self, state: FlatpakState) {
        self.state = state;
    }

    /// Set size
    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }

    /// Check if Flatpak is installed
    pub fn is_installed(&self) -> bool {
        self.state == FlatpakState::Installed
    }

    /// Check if update is available
    pub fn has_update(&self) -> bool {
        self.state == FlatpakState::UpdateAvailable
    }
}

/// Flatpak remote repository
#[derive(Debug, Clone)]
pub struct FlatpakRemote {
    pub remote_id: String,
    pub name: String,
    pub url: String,
    pub enabled: bool,
}

impl FlatpakRemote {
    /// Create a new Flatpak remote
    pub fn new(remote_id: String, name: String, url: String) -> Self {
        Self {
            remote_id,
            name,
            url,
            enabled: true,
        }
    }

    /// Enable remote
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable remote
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Check if remote is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Flatpak manager
pub struct SigmaFlatpak {
    flatpaks: BTreeMap<String, Flatpak>,
    remotes: BTreeMap<String, FlatpakRemote>,
    running: AtomicBool,
}

impl SigmaFlatpak {
    /// Create a new Flatpak manager
    pub fn new() -> Self {
        Self {
            flatpaks: BTreeMap::new(),
            remotes: BTreeMap::new(),
            running: AtomicBool::new(false),
        }
    }

    /// Start the Flatpak manager
    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
    }

    /// Stop the Flatpak manager
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Add a Flatpak
    pub fn add_flatpak(&mut self, flatpak: Flatpak) -> bool {
        let fp_id = flatpak.flatpak_id.clone();
        self.flatpaks.insert(fp_id, flatpak);
        true
    }

    /// Remove a Flatpak
    pub fn remove_flatpak(&mut self, flatpak_id: &str) -> Option<Flatpak> {
        self.flatpaks.remove(flatpak_id)
    }

    /// Get a Flatpak by ID
    pub fn get_flatpak(&self, flatpak_id: &str) -> Option<&Flatpak> {
        self.flatpaks.get(flatpak_id)
    }

    /// List all Flatpaks
    pub fn list_flatpaks(&self) -> Vec<&Flatpak> {
        self.flatpaks.values().collect()
    }

    /// List Flatpaks by type
    pub fn list_flatpaks_by_type(&self, flatpak_type: FlatpakType) -> Vec<&Flatpak> {
        self.flatpaks
            .values()
            .filter(|f| f.flatpak_type == flatpak_type)
            .collect()
    }

    /// List installed Flatpaks
    pub fn list_installed_flatpaks(&self) -> Vec<&Flatpak> {
        self.flatpaks
            .values()
            .filter(|f| f.is_installed())
            .collect()
    }

    /// Install a Flatpak
    pub fn install_flatpak(&mut self, flatpak_id: &str) -> bool {
        if let Some(flatpak) = self.flatpaks.get_mut(flatpak_id) {
            flatpak.set_state(FlatpakState::Installed);
            true
        } else {
            false
        }
    }

    /// Remove a Flatpak
    pub fn remove_installed_flatpak(&mut self, flatpak_id: &str) -> bool {
        if let Some(flatpak) = self.flatpaks.get_mut(flatpak_id) {
            flatpak.set_state(FlatpakState::NotInstalled);
            true
        } else {
            false
        }
    }

    /// Update a Flatpak
    pub fn update_flatpak(&mut self, flatpak_id: &str) -> bool {
        if let Some(flatpak) = self.flatpaks.get_mut(flatpak_id) {
            flatpak.set_state(FlatpakState::Installed);
            true
        } else {
            false
        }
    }

    /// Add a remote
    pub fn add_remote(&mut self, remote: FlatpakRemote) -> bool {
        let remote_id = remote.remote_id.clone();
        self.remotes.insert(remote_id, remote);
        true
    }

    /// Remove a remote
    pub fn remove_remote(&mut self, remote_id: &str) -> Option<FlatpakRemote> {
        self.remotes.remove(remote_id)
    }

    /// Get a remote by ID
    pub fn get_remote(&self, remote_id: &str) -> Option<&FlatpakRemote> {
        self.remotes.get(remote_id)
    }

    /// List all remotes
    pub fn list_remotes(&self) -> Vec<&FlatpakRemote> {
        self.remotes.values().collect()
    }

    /// List enabled remotes
    pub fn list_enabled_remotes(&self) -> Vec<&FlatpakRemote> {
        self.remotes.values().filter(|r| r.is_enabled()).collect()
    }

    /// Get Flatpak count
    pub fn flatpak_count(&self) -> usize {
        self.flatpaks.len()
    }

    /// Get remote count
    pub fn remote_count(&self) -> usize {
        self.remotes.len()
    }
}

impl Default for SigmaFlatpak {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaFlatpak instance
static mut SIGMAFLATPAK_INSTANCE: Option<SigmaFlatpak> = None;

/// Initialize the global SigmaFlatpak instance
pub fn init_sigmaflatpak() {
    unsafe {
        SIGMAFLATPAK_INSTANCE = Some(SigmaFlatpak::new());
    }
}

/// Get the global SigmaFlatpak instance
pub fn get_sigmaflatpak() -> Option<&'static SigmaFlatpak> {
    unsafe { SIGMAFLATPAK_INSTANCE.as_ref() }
}

/// Get mutable global SigmaFlatpak instance
pub fn get_sigmaflatpak_mut() -> Option<&'static mut SigmaFlatpak> {
    unsafe { SIGMAFLATPAK_INSTANCE.as_mut() }
}

/// Convenience function to add a Flatpak
pub fn add_flatpak(flatpak: Flatpak) -> bool {
    if let Some(flatpak_mgr) = get_sigmaflatpak_mut() {
        flatpak_mgr.add_flatpak(flatpak)
    } else {
        false
    }
}

/// Convenience function to install a Flatpak
pub fn install_flatpak(flatpak_id: &str) -> bool {
    if let Some(flatpak_mgr) = get_sigmaflatpak_mut() {
        flatpak_mgr.install_flatpak(flatpak_id)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatpak_creation() {
        let flatpak = Flatpak::new(
            "fp1".to_string(),
            "Firefox".to_string(),
            "org.mozilla.Firefox".to_string(),
            "stable".to_string(),
            "x86_64".to_string(),
            FlatpakType::App,
        );

        assert_eq!(flatpak.flatpak_id, "fp1");
        assert_eq!(flatpak.flatpak_type, FlatpakType::App);
        assert!(!flatpak.is_installed());
    }

    #[test]
    fn test_flatpak_state() {
        let mut flatpak = Flatpak::new(
            "fp1".to_string(),
            "Firefox".to_string(),
            "org.mozilla.Firefox".to_string(),
            "stable".to_string(),
            "x86_64".to_string(),
            FlatpakType::App,
        );

        flatpak.set_state(FlatpakState::Installed);
        assert!(flatpak.is_installed());
    }

    #[test]
    fn test_sigmaflatpak_creation() {
        let flatpak_mgr = SigmaFlatpak::new();
        assert!(!flatpak_mgr.is_running());
        assert_eq!(flatpak_mgr.flatpak_count(), 0);
    }

    #[test]
    fn test_sigmaflatpak_start_stop() {
        let mut flatpak_mgr = SigmaFlatpak::new();
        flatpak_mgr.start();
        assert!(flatpak_mgr.is_running());
        flatpak_mgr.stop();
        assert!(!flatpak_mgr.is_running());
    }

    #[test]
    fn test_sigmaflatpak_add_flatpak() {
        let mut flatpak_mgr = SigmaFlatpak::new();
        let flatpak = Flatpak::new(
            "fp1".to_string(),
            "Firefox".to_string(),
            "org.mozilla.Firefox".to_string(),
            "stable".to_string(),
            "x86_64".to_string(),
            FlatpakType::App,
        );

        let added = flatpak_mgr.add_flatpak(flatpak);
        assert!(added);
        assert_eq!(flatpak_mgr.flatpak_count(), 1);
    }

    #[test]
    fn test_global_instance() {
        init_sigmaflatpak();

        let flatpak_mgr = get_sigmaflatpak();
        assert!(flatpak_mgr.is_some());

        let mut flatpak_mgr_mut = get_sigmaflatpak_mut();
        assert!(flatpak_mgr_mut.is_some());

        if let Some(flatpak_mgr) = flatpak_mgr_mut {
            flatpak_mgr.start();
            assert!(flatpak_mgr.is_running());
        }
    }

    #[test]
    fn test_convenience_add_flatpak() {
        init_sigmaflatpak();

        let flatpak = Flatpak::new(
            "fp1".to_string(),
            "Firefox".to_string(),
            "org.mozilla.Firefox".to_string(),
            "stable".to_string(),
            "x86_64".to_string(),
            FlatpakType::App,
        );

        let added = add_flatpak(flatpak);
        assert!(added);

        let flatpak_mgr = get_sigmaflatpak().unwrap();
        assert_eq!(flatpak_mgr.flatpak_count(), 1);
    }
}
