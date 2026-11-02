//! # Mount Manager
//!
//! Race-free mount/unmount lifecycle with failure rollback semantics.
//! Implements the `MountManager` struct and exposes `mount`/`unmount` functions.
//!
//! ## Principles
//! - Race-free design: state machine guards against double-mount/unmount.
//! - Fail-fast: any failed initialisation rolls back immediately.
//! - Single responsibility: this module owns only mount lifecycle.

use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;

use thiserror::Error;

// ---------------------------------------------------------------------------
// Errors and state
// ---------------------------------------------------------------------------

/// Errors that can occur during mount / unmount operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum MountError {
    #[error("target is already mounted")]
    AlreadyMounted,
    #[error("target is not mounted")]
    NotMounted,
    #[error("double unmount attempted")]
    DoubleUnmount,
    #[error("device or resource busy")]
    ResourceBusy,
    #[error("invalid source device")]
    InvalidSource,
    #[error("invalid mount target path")]
    InvalidTarget,
    #[error("filesystem initialisation failed")]
    InitializationFailed,
    #[error("mount rolled back after failure")]
    RollbackSuccessful,
}

/// Lifecycle state of a single mount point.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MountState {
    Unmounted,
    Mounting,
    Mounted,
    Unmounting,
}

// ---------------------------------------------------------------------------
// Mount flags
// ---------------------------------------------------------------------------

/// Per-mount option flags.
#[derive(Debug, Clone, Copy)]
pub struct MountFlags {
    pub read_only: bool,
    pub no_exec:   bool,
    pub no_suid:   bool,
    pub sync:      bool,
    pub bind:      bool,
}

impl Default for MountFlags {
    fn default() -> Self {
        Self { read_only: false, no_exec: false, no_suid: false, sync: true, bind: false }
    }
}

// ---------------------------------------------------------------------------
// Mount entry
// ---------------------------------------------------------------------------

/// Record for one active (or recently unmounted) mount point.
#[derive(Debug, Clone)]
pub struct MountEntry {
    pub mount_point:    String,
    pub device_source:  String,
    pub fs_type:        String,
    pub flags:          MountFlags,
    pub state:          MountState,
    pub open_ref_count: usize,
}

// ---------------------------------------------------------------------------
// MountManager
// ---------------------------------------------------------------------------

/// Manages the set of active mount points for the system.
pub struct MountManager {
    pub mounts: BTreeMap<String, MountEntry>,
}

impl MountManager {
    /// Create an empty mount manager.
    pub fn new() -> Self {
        Self { mounts: BTreeMap::new() }
    }

    /// Mount `source` at `target` with the given `fs_type` and `flags`.
    pub fn mount(
        &mut self,
        source:  &str,
        target:  &str,
        fs_type: &str,
        flags:   MountFlags,
    ) -> Result<(), MountError> {
        let key = target.to_string();
        if let Some(e) = self.mounts.get(&key) {
            if e.state == MountState::Mounted {
                return Err(MountError::AlreadyMounted);
            }
        }
        self.mounts.insert(key.clone(), MountEntry {
            mount_point:    key,
            device_source:  source.to_string(),
            fs_type:        fs_type.to_string(),
            flags,
            state:          MountState::Mounted,
            open_ref_count: 0,
        });
        Ok(())
    }

    /// Mount with an init closure; rolls back if the closure returns `Err`.
    pub fn mount_with_rollback<F>(
        &mut self,
        source:  &str,
        target:  &str,
        fs_type: &str,
        flags:   MountFlags,
        init_fn: F,
    ) -> Result<(), MountError>
    where
        F: FnOnce() -> Result<(), ()>,
    {
        let key = target.to_string();
        if self.mounts.contains_key(&key) {
            return Err(MountError::AlreadyMounted);
        }

        // Stage the entry in Mounting state.
        let entry = MountEntry {
            mount_point:    key.clone(),
            device_source:  source.to_string(),
            fs_type:        fs_type.to_string(),
            flags,
            state:          MountState::Mounting,
            open_ref_count: 0,
        };
        self.mounts.insert(key.clone(), entry);

        if init_fn().is_err() {
            // Rollback: remove the partial entry.
            self.mounts.remove(&key);
            return Err(MountError::RollbackSuccessful);
        }

        if let Some(e) = self.mounts.get_mut(&key) {
            e.state = MountState::Mounted;
        }
        Ok(())
    }

    /// Unmount the filesystem at `target`.
    pub fn unmount(&mut self, target: &str) -> Result<(), MountError> {
        let entry = self.mounts.get_mut(target).ok_or(MountError::NotMounted)?;

        if entry.state == MountState::Unmounted {
            return Err(MountError::DoubleUnmount);
        }
        if entry.open_ref_count > 0 {
            return Err(MountError::ResourceBusy);
        }

        entry.state = MountState::Unmounted;
        self.mounts.remove(target);
        Ok(())
    }

    /// Increment the open-file reference count for `target`.
    pub fn acquire_ref(&mut self, target: &str) -> Result<(), MountError> {
        let entry = self.mounts.get_mut(target).ok_or(MountError::NotMounted)?;
        if entry.state != MountState::Mounted {
            return Err(MountError::NotMounted);
        }
        entry.open_ref_count += 1;
        Ok(())
    }

    /// Decrement the open-file reference count for `target`.
    pub fn release_ref(&mut self, target: &str) -> Result<(), MountError> {
        let entry = self.mounts.get_mut(target).ok_or(MountError::NotMounted)?;
        if entry.open_ref_count > 0 {
            entry.open_ref_count -= 1;
        }
        Ok(())
    }

    /// Return the list of currently mounted targets.
    pub fn list_mounts(&self) -> Vec<&str> {
        self.mounts
            .iter()
            .filter(|(_, e)| e.state == MountState::Mounted)
            .map(|(k, _)| k.as_str())
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Free-standing helpers
// ---------------------------------------------------------------------------

/// Convenience wrapper: mount without a rollback hook.
pub fn mount(
    mgr:     &mut MountManager,
    source:  &str,
    target:  &str,
    fs_type: &str,
    flags:   MountFlags,
) -> Result<(), MountError> {
    mgr.mount(source, target, fs_type, flags)
}

/// Convenience wrapper: unmount.
pub fn unmount(mgr: &mut MountManager, target: &str) -> Result<(), MountError> {
    mgr.unmount(target)
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mount_lifecycle_and_double_unmount_prevention() {
        let mut mgr = MountManager::new();
        let flags = MountFlags::default();

        mgr.mount("/dev/nvme0n1p1", "/mnt/data", "ext4", flags).unwrap();
        mgr.acquire_ref("/mnt/data").unwrap();

        // Busy — cannot unmount.
        assert_eq!(mgr.unmount("/mnt/data"), Err(MountError::ResourceBusy));

        mgr.release_ref("/mnt/data").unwrap();
        mgr.unmount("/mnt/data").unwrap();

        // Double unmount.
        assert_eq!(mgr.unmount("/mnt/data"), Err(MountError::NotMounted));
    }

    #[test]
    fn mount_rollback_on_failure() {
        let mut mgr = MountManager::new();
        let flags = MountFlags::default();

        let res = mgr.mount_with_rollback("/dev/sdb1", "/mnt/fail", "zfs", flags, || Err(()));
        assert_eq!(res, Err(MountError::RollbackSuccessful));
        assert!(!mgr.mounts.contains_key("/mnt/fail"));
    }

    #[test]
    fn double_mount_rejected() {
        let mut mgr = MountManager::new();
        let flags = MountFlags::default();
        mgr.mount("/dev/sda1", "/", "ext4", flags).unwrap();
        assert_eq!(mgr.mount("/dev/sda2", "/", "ext4", flags), Err(MountError::AlreadyMounted));
    }

    #[test]
    fn list_mounts_shows_active() {
        let mut mgr = MountManager::new();
        let flags = MountFlags::default();
        mgr.mount("/dev/sda1", "/", "ext4", flags).unwrap();
        mgr.mount("/dev/sda2", "/home", "ext4", flags).unwrap();
        let mounts = mgr.list_mounts();
        assert_eq!(mounts.len(), 2);
    }
}
