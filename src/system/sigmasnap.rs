//! SigmaSnap - Snap Package Management
//!
//! Inspired by Snap, this module provides Snap package management including
//! sandboxed applications, snap channels, and Snap store integration.

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, Ordering};

/// Snap type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapType {
    App,
    Base,
    Gadget,
    Kernel,
    Unknown,
;

/// Snap state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapState {
    Installed,
    NotInstalled,
    UpdateAvailable,
    Unknown,
;

/// Snap data structure
#[derive(Debug, Clone)]
pub struct Snap {
    pub snap_id: String,
    pub name: String,
    pub publisher: String,
    pub snap_type: SnapType,
    pub state: SnapState,
    pub channel: String,
    pub size: u64,
;

impl Snap {
    /// Create a new Snap
    pub fn new(
        snap_id: String,
        name: String,
        publisher: String,
        snap_type: SnapType,
        channel: String,
    ) -> Self {
        Self {
            snap_id,
            name,
            publisher,
            snap_type,
            state: SnapState::NotInstalled,
            channel,
            size: 0,
        }
    }

    /// Set state
    pub fn set_state(&mut self, state: SnapState) {
        self.state = state;
    }

    /// Set size
    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }

    /// Check if Snap is installed
    pub fn is_installed(&self) -> bool {
        self.state == SnapState::Installed
    }
;

/// Snap manager
pub struct SigmaSnap {
    snaps: BTreeMap<String, Snap>,
    running: AtomicBool,
;

impl SigmaSnap {
    /// Create a new Snap manager
    pub fn new() -> Self {
        Self {
            snaps: BTreeMap::new(),
            running: AtomicBool::new(false),
        }
    }

    /// Start the Snap manager
    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
    }

    /// Stop the Snap manager
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Add a Snap
    pub fn add_snap(&mut self, snap: Snap) -> bool {
        let snap_id = snap.snap_id.clone();
        self.snaps.insert(snap_id, snap);
        true
    }

    /// Remove a Snap
    pub fn remove_snap(&mut self, snap_id: &str) -> Option<Snap> {
        self.snaps.remove(snap_id)
    }

    /// Get a Snap by ID
    pub fn get_snap(&self, snap_id: &str) -> Option<&Snap> {
        self.snaps.get(snap_id)
    }

    /// List all Snaps
    pub fn list_snaps(&self) -> Vec<&Snap> {
        self.snaps.values().collect()
    }

    /// List Snaps by type
    pub fn list_snaps_by_type(&self, snap_type: SnapType) -> Vec<&Snap> {
        self.snaps
            .values()
            .filter(|s| s.snap_type == snap_type)
            .collect()
    }

    /// List installed Snaps
    pub fn list_installed_snaps(&self) -> Vec<&Snap> {
        self.snaps.values().filter(|s| s.is_installed()).collect()
    }

    /// Install a Snap
    pub fn install_snap(&mut self, snap_id: &str) -> bool {
        if let Some(snap) = self.snaps.get_mut(snap_id) {
            snap.set_state(SnapState::Installed);
            true
        } else {
            false
        }
    }

    /// Remove a Snap
    pub fn remove_installed_snap(&mut self, snap_id: &str) -> bool {
        if let Some(snap) = self.snaps.get_mut(snap_id) {
            snap.set_state(SnapState::NotInstalled);
            true
        } else {
            false
        }
    }

    /// Get snap count
    pub fn snap_count(&self) -> usize {
        self.snaps.len()
    }
;

impl Default for SigmaSnap {
    fn default() -> Self {
        Self::new()
    }
;

/// Global SigmaSnap instance
static mut SIGMASNAP_INSTANCE: Option<SigmaSnap> = None;

/// Initialize the global SigmaSnap instance
pub fn init_sigmasnap() {
    unsafe {
        SIGMASNAP_INSTANCE = Some(SigmaSnap::new());
    }
;

/// Get the global SigmaSnap instance
pub fn get_sigmasnap() -> Option<&'static SigmaSnap> {
    unsafe { SIGMASNAP_INSTANCE.as_ref() }
;

/// Get mutable global SigmaSnap instance
pub fn get_sigmasnap_mut() -> Option<&'static mut SigmaSnap> {
    unsafe { SIGMASNAP_INSTANCE.as_mut() }
;

/// Convenience function to add a Snap
pub fn add_snap(snap: Snap) -> bool {
    if let Some(snap_mgr) = get_sigmasnap_mut() {
        snap_mgr.add_snap(snap)
    } else {
        false
    }
;

/// Convenience function to install a Snap
pub fn install_snap(snap_id: &str) -> bool {
    if let Some(snap_mgr) = get_sigmasnap_mut() {
        snap_mgr.install_snap(snap_id)
    } else {
        false
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snap_creation() {
        let snap = Snap::new(
            "snap1".to_string(),
            "Firefox".to_string(),
            "Mozilla".to_string(),
            SnapType::App,
            "stable".to_string(),
        );

        assert_eq!(snap.snap_id, "snap1");
        assert_eq!(snap.snap_type, SnapType::App);
        assert!(!snap.is_installed());
    }

    #[test]
    fn test_sigmasnap_creation() {
        let snap_mgr = SigmaSnap::new();
        assert!(!snap_mgr.is_running());
        assert_eq!(snap_mgr.snap_count(), 0);
    }

    #[test]
    fn test_sigmasnap_start_stop() {
        let mut snap_mgr = SigmaSnap::new();
        snap_mgr.start();
        assert!(snap_mgr.is_running());
        snap_mgr.stop();
        assert!(!snap_mgr.is_running());
    }

    #[test]
    fn test_sigmasnap_add_snap() {
        let mut snap_mgr = SigmaSnap::new();
        let snap = Snap::new(
            "snap1".to_string(),
            "Firefox".to_string(),
            "Mozilla".to_string(),
            SnapType::App,
            "stable".to_string(),
        );

        let added = snap_mgr.add_snap(snap);
        assert!(added);
        assert_eq!(snap_mgr.snap_count(), 1);
    }

    #[test]
    fn test_global_instance() {
        init_sigmasnap();

        let snap_mgr = get_sigmasnap();
        assert!(snap_mgr.is_some());

        let mut snap_mgr_mut = get_sigmasnap_mut();
        assert!(snap_mgr_mut.is_some());

        if let Some(snap_mgr) = snap_mgr_mut {
            snap_mgr.start();
            assert!(snap_mgr.is_running());
        }
    }

    #[test]
    fn test_convenience_add_snap() {
        init_sigmasnap();

        let snap = Snap::new(
            "snap1".to_string(),
            "Firefox".to_string(),
            "Mozilla".to_string(),
            SnapType::App,
            "stable".to_string(),
        );

        let added = add_snap(snap);
        assert!(added);

        let snap_mgr = get_sigmasnap().unwrap();
        assert_eq!(snap_mgr.snap_count(), 1);
    }
;
