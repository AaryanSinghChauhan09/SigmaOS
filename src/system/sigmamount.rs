//! SigmaMount - Mount Management
//!
//! Inspired by fstab, this module provides comprehensive mount management
//! including mount point configuration, mount options, automatic mounting,
//! and mount table management.

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, Ordering};

/// Mount state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MountState {
    Mounted,
    Unmounted,
    Failed,
    Unknown,
}

/// Mount option enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MountOption {
    ReadOnly,
    ReadWrite,
    NoExec,
    NoSuid,
    NoDev,
    Sync,
    Async,
    Auto,
    NoAuto,
    User,
    Users,
    Unknown,
}

/// Mount entry data structure
#[derive(Debug, Clone)]
pub struct MountEntry {
    pub entry_id: String,
    pub device: String,
    pub mount_point: String,
    pub filesystem_type: String,
    pub options: Vec<MountOption>,
    pub dump: u32,
    pub fsck_order: u32,
    pub state: MountState,
}

impl MountEntry {
    /// Create a new mount entry
    pub fn new(
        entry_id: String,
        device: String,
        mount_point: String,
        filesystem_type: String,
        dump: u32,
        fsck_order: u32,
    ) -> Self {
        Self {
            entry_id,
            device,
            mount_point,
            filesystem_type,
            options: Vec::new(),
            dump,
            fsck_order,
            state: MountState::Unknown,
        }
    }

    /// Set mount state
    pub fn set_state(&mut self, state: MountState) {
        self.state = state;
    }

    /// Add a mount option
    pub fn add_option(&mut self, option: MountOption) {
        if !self.options.contains(&option) {
            self.options.push(option);
        }
    }

    /// Remove a mount option
    pub fn remove_option(&mut self, option: MountOption) {
        self.options.retain(|o| *o != option);
    }

    /// Check if entry has an option
    pub fn has_option(&self, option: MountOption) -> bool {
        self.options.contains(&option)
    }

    /// Check if entry is mounted
    pub fn is_mounted(&self) -> bool {
        self.state == MountState::Mounted
    }

    /// Check if entry is auto-mounted
    pub fn is_auto(&self) -> bool {
        self.has_option(MountOption::Auto)
    }
}

/// Mount manager
pub struct SigmaMount {
    mount_entries: BTreeMap<String, MountEntry>,
    running: AtomicBool,
}

impl SigmaMount {
    /// Create a new mount manager
    pub fn new() -> Self {
        Self {
            mount_entries: BTreeMap::new(),
            running: AtomicBool::new(false),
        }
    }

    /// Start the mount manager
    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
    }

    /// Stop the mount manager
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Add a mount entry
    pub fn add_entry(&mut self, entry: MountEntry) {
        self.mount_entries.insert(entry.entry_id.clone(), entry);
    }

    /// Remove a mount entry
    pub fn remove_entry(&mut self, entry_id: &str) -> Option<MountEntry> {
        self.mount_entries.remove(entry_id)
    }

    /// Get a mount entry by ID
    pub fn get_entry(&self, entry_id: &str) -> Option<&MountEntry> {
        self.mount_entries.get(entry_id)
    }

    /// Get a mount entry by mount point
    pub fn get_entry_by_mount_point(&self, mount_point: &str) -> Option<&MountEntry> {
        self.mount_entries
            .values()
            .find(|e| e.mount_point == mount_point)
    }

    /// List all mount entries
    pub fn list_entries(&self) -> Vec<&MountEntry> {
        self.mount_entries.values().collect()
    }

    /// List mounted entries
    pub fn list_mounted(&self) -> Vec<&MountEntry> {
        self.mount_entries
            .values()
            .filter(|e| e.is_mounted())
            .collect()
    }

    /// List unmounted entries
    pub fn list_unmounted(&self) -> Vec<&MountEntry> {
        self.mount_entries
            .values()
            .filter(|e| !e.is_mounted())
            .collect()
    }

    /// List auto-mount entries
    pub fn list_auto(&self) -> Vec<&MountEntry> {
        self.mount_entries
            .values()
            .filter(|e| e.is_auto())
            .collect()
    }

    /// Mount an entry
    pub fn mount(&mut self, entry_id: &str) -> bool {
        if let Some(entry) = self.mount_entries.get_mut(entry_id) {
            entry.set_state(MountState::Mounted);
            true
        } else {
            false
        }
    }

    /// Unmount an entry
    pub fn unmount(&mut self, entry_id: &str) -> bool {
        if let Some(entry) = self.mount_entries.get_mut(entry_id) {
            entry.set_state(MountState::Unmounted);
            true
        } else {
            false
        }
    }

    /// Mount all auto entries
    pub fn mount_all_auto(&mut self) -> usize {
        let mut count = 0;
        for entry in self.mount_entries.values_mut() {
            if entry.is_auto() && !entry.is_mounted() {
                entry.set_state(MountState::Mounted);
                count += 1;
            }
        }
        count
    }

    /// Get entry count
    pub fn entry_count(&self) -> usize {
        self.mount_entries.len()
    }
}

impl Default for SigmaMount {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaMount instance (placeholder for actual global state)
static mut SIGMAMOUNT_INSTANCE: Option<SigmaMount> = None;

/// Initialize the global SigmaMount instance
pub fn init_sigmamount() {
    unsafe {
        SIGMAMOUNT_INSTANCE = Some(SigmaMount::new());
    }
}

/// Get the global SigmaMount instance
pub fn get_sigmamount() -> Option<&'static SigmaMount> {
    unsafe { SIGMAMOUNT_INSTANCE.as_ref() }
}

/// Get mutable global SigmaMount instance
pub fn get_sigmamount_mut() -> Option<&'static mut SigmaMount> {
    unsafe { SIGMAMOUNT_INSTANCE.as_mut() }
}

/// Convenience function to add a mount entry
pub fn add_mount_entry(entry: MountEntry) {
    if let Some(mount) = get_sigmamount_mut() {
        mount.add_entry(entry);
    }
}

/// Convenience function to mount an entry
pub fn mount_entry(entry_id: &str) -> bool {
    if let Some(mount) = get_sigmamount_mut() {
        mount.mount(entry_id)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mount_entry_creation() {
        let entry = MountEntry::new(
            "entry1".to_string(),
            "/dev/sda1".to_string(),
            "/".to_string(),
            "ext4".to_string(),
            1,
            1,
        );

        assert_eq!(entry.entry_id, "entry1");
        assert_eq!(entry.device, "/dev/sda1");
        assert_eq!(entry.mount_point, "/");
        assert_eq!(entry.filesystem_type, "ext4");
    }

    #[test]
    fn test_mount_entry_options() {
        let mut entry = MountEntry::new(
            "entry1".to_string(),
            "/dev/sda1".to_string(),
            "/".to_string(),
            "ext4".to_string(),
            1,
            1,
        );

        entry.add_option(MountOption::ReadOnly);
        entry.add_option(MountOption::Auto);
        assert!(entry.has_option(MountOption::ReadOnly));
        assert!(entry.is_auto());

        entry.remove_option(MountOption::ReadOnly);
        assert!(!entry.has_option(MountOption::ReadOnly));
    }

    #[test]
    fn test_mount_entry_state() {
        let mut entry = MountEntry::new(
            "entry1".to_string(),
            "/dev/sda1".to_string(),
            "/".to_string(),
            "ext4".to_string(),
            1,
            1,
        );

        entry.set_state(MountState::Mounted);
        assert!(entry.is_mounted());

        entry.set_state(MountState::Unmounted);
        assert!(!entry.is_mounted());
    }

    #[test]
    fn test_sigmamount_creation() {
        let mount = SigmaMount::new();
        assert!(!mount.is_running());
        assert_eq!(mount.entry_count(), 0);
    }

    #[test]
    fn test_sigmamount_start_stop() {
        let mut mount = SigmaMount::new();
        mount.start();
        assert!(mount.is_running());
        mount.stop();
        assert!(!mount.is_running());
    }

    #[test]
    fn test_sigmamount_add_entry() {
        let mut mount = SigmaMount::new();
        let entry = MountEntry::new(
            "entry1".to_string(),
            "/dev/sda1".to_string(),
            "/".to_string(),
            "ext4".to_string(),
            1,
            1,
        );

        mount.add_entry(entry);
        assert_eq!(mount.entry_count(), 1);
        assert!(mount.get_entry("entry1").is_some());
    }

    #[test]
    fn test_sigmamount_remove_entry() {
        let mut mount = SigmaMount::new();
        let entry = MountEntry::new(
            "entry1".to_string(),
            "/dev/sda1".to_string(),
            "/".to_string(),
            "ext4".to_string(),
            1,
            1,
        );

        mount.add_entry(entry);
        assert_eq!(mount.entry_count(), 1);

        let removed = mount.remove_entry("entry1");
        assert!(removed.is_some());
        assert_eq!(mount.entry_count(), 0);
    }

    #[test]
    fn test_sigmamount_mount_unmount() {
        let mut mount = SigmaMount::new();
        let entry = MountEntry::new(
            "entry1".to_string(),
            "/dev/sda1".to_string(),
            "/".to_string(),
            "ext4".to_string(),
            1,
            1,
        );

        mount.add_entry(entry);
        let mounted = mount.mount("entry1");
        assert!(mounted);

        let entry = mount.get_entry("entry1").unwrap();
        assert!(entry.is_mounted());

        let unmounted = mount.unmount("entry1");
        assert!(unmounted);

        let entry = mount.get_entry("entry1").unwrap();
        assert!(!entry.is_mounted());
    }

    #[test]
    fn test_sigmamount_mount_all_auto() {
        let mut mount = SigmaMount::new();

        let mut entry1 = MountEntry::new(
            "entry1".to_string(),
            "/dev/sda1".to_string(),
            "/".to_string(),
            "ext4".to_string(),
            1,
            1,
        );
        entry1.add_option(MountOption::Auto);
        mount.add_entry(entry1);

        let entry2 = MountEntry::new(
            "entry2".to_string(),
            "/dev/sda2".to_string(),
            "/home".to_string(),
            "ext4".to_string(),
            1,
            2,
        );
        entry2.add_option(MountOption::Auto);
        mount.add_entry(entry2);

        let entry3 = MountEntry::new(
            "entry3".to_string(),
            "/dev/sda3".to_string(),
            "/data".to_string(),
            "ext4".to_string(),
            1,
            3,
        );
        entry3.add_option(MountOption::NoAuto);
        mount.add_entry(entry3);

        let count = mount.mount_all_auto();
        assert_eq!(count, 2);

        let mounted = mount.list_mounted();
        assert_eq!(mounted.len(), 2);
    }

    #[test]
    fn test_sigmamount_list_mounted() {
        let mut mount = SigmaMount::new();

        let mut entry1 = MountEntry::new(
            "entry1".to_string(),
            "/dev/sda1".to_string(),
            "/".to_string(),
            "ext4".to_string(),
            1,
            1,
        );
        entry1.set_state(MountState::Mounted);
        mount.add_entry(entry1);

        let entry2 = MountEntry::new(
            "entry2".to_string(),
            "/dev/sda2".to_string(),
            "/home".to_string(),
            "ext4".to_string(),
            1,
            2,
        );
        mount.add_entry(entry2);

        let mounted = mount.list_mounted();
        assert_eq!(mounted.len(), 1);
    }

    #[test]
    fn test_sigmamount_get_entry_by_mount_point() {
        let mut mount = SigmaMount::new();
        let entry = MountEntry::new(
            "entry1".to_string(),
            "/dev/sda1".to_string(),
            "/".to_string(),
            "ext4".to_string(),
            1,
            1,
        );

        mount.add_entry(entry);
        let found = mount.get_entry_by_mount_point("/");
        assert!(found.is_some());

        let not_found = mount.get_entry_by_mount_point("/home");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_global_instance() {
        init_sigmamount();

        let mount = get_sigmamount();
        assert!(mount.is_some());

        let mut mount_mut = get_sigmamount_mut();
        assert!(mount_mut.is_some());

        if let Some(mount) = mount_mut {
            mount.start();
            assert!(mount.is_running());
        }
    }

    #[test]
    fn test_convenience_add_entry() {
        init_sigmamount();

        let entry = MountEntry::new(
            "entry1".to_string(),
            "/dev/sda1".to_string(),
            "/".to_string(),
            "ext4".to_string(),
            1,
            1,
        );

        add_mount_entry(entry);

        let mount = get_sigmamount().unwrap();
        assert_eq!(mount.entry_count(), 1);
    }

    #[test]
    fn test_convenience_mount_entry() {
        init_sigmamount();

        let entry = MountEntry::new(
            "entry1".to_string(),
            "/dev/sda1".to_string(),
            "/".to_string(),
            "ext4".to_string(),
            1,
            1,
        );

        add_mount_entry(entry);

        let mounted = mount_entry("entry1");
        assert!(mounted);

        let mount = get_sigmamount().unwrap();
        let entry = mount.get_entry("entry1").unwrap();
        assert!(entry.is_mounted());
    }
}
