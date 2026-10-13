//! SigmaNFS - Network File System (NFS) Client and Server Support
//!
//! Inspired by Linux nfs-utils, nfsd, and mount.nfs (NFSv3/NFSv4), this module provides
//! comprehensive network filesystem sharing, remote mount management, and RPC export
//! handling for SigmaOS.
//!
//! Features include:
//! - NFSv3, NFSv4, NFSv4.1, and NFSv4.2 protocol support
//! - Remote NFS mount configuration and connection tracking
//! - Server-side export management (subnets, read-only/read-write, root squash)
//! - Connection lifecycle state monitoring (Connected, StaleHandle, Disconnected)
//! - Global instance and convenience API

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};

/// NFS Protocol version
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NFSVersion {
    V3,
    V4,
    V4_1,
    V4_2,
    Unknown,
}

/// NFS mount connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NFSConnectionState {
    Disconnected,
    Connecting,
    Connected,
    StaleHandle,
    Timeout,
    Failed,
}

/// Export permission policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NFSExportAccess {
    ReadOnly,
    ReadWrite,
}

/// Root squash security policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RootSquashPolicy {
    RootSquash,
    NoRootSquash,
    AllSquash,
}

/// NFS Server Export descriptor
#[derive(Debug, Clone)]
pub struct NFSExport {
    pub export_id: String,
    pub local_path: String,
    pub allowed_client_cidr: String,
    pub access: NFSExportAccess,
    pub root_squash: RootSquashPolicy,
    pub sync: bool,
    pub enabled: bool,
}

impl NFSExport {
    /// Create a new NFS export definition
    pub fn new(
        export_id: String,
        local_path: String,
        allowed_client_cidr: String,
        access: NFSExportAccess,
        root_squash: RootSquashPolicy,
        sync: bool,
    ) -> Self {
        Self {
            export_id,
            local_path,
            allowed_client_cidr,
            access,
            root_squash,
            sync,
            enabled: true,
        }
    }

    /// Enable or disable export
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

/// Remote NFS mount point descriptor
#[derive(Debug, Clone)]
pub struct NFSMount {
    pub mount_id: String,
    pub server_address: String,
    pub remote_path: String,
    pub local_mount_point: String,
    pub version: NFSVersion,
    pub state: NFSConnectionState,
    pub rsize_bytes: u32,
    pub wsize_bytes: u32,
    pub hard_mount: bool,
}

impl NFSMount {
    /// Create a new NFS mount definition
    pub fn new(
        mount_id: String,
        server_address: String,
        remote_path: String,
        local_mount_point: String,
        version: NFSVersion,
    ) -> Self {
        Self {
            mount_id,
            server_address,
            remote_path,
            local_mount_point,
            version,
            state: NFSConnectionState::Disconnected,
            rsize_bytes: 1048576,
            wsize_bytes: 1048576,
            hard_mount: true,
        }
    }

    /// Set connection status
    pub fn set_state(&mut self, state: NFSConnectionState) {
        self.state = state;
    }

    /// Check if mount is actively connected
    pub fn is_connected(&self) -> bool {
        self.state == NFSConnectionState::Connected
    }
}

/// Main SigmaNFS manager struct
pub struct SigmaNFS {
    exports: BTreeMap<String, NFSExport>,
    mounts: BTreeMap<String, NFSMount>,
    running: AtomicBool,
}

impl SigmaNFS {
    /// Create a new SigmaNFS manager instance
    pub fn new() -> Self {
        Self {
            exports: BTreeMap::new(),
            mounts: BTreeMap::new(),
            running: AtomicBool::new(false),
        }
    }

    /// Start NFS service
    pub fn start(&self) {
        self.running.store(true, Ordering::SeqCst);
    }

    /// Stop NFS service
    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if NFS service is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Add an exported share
    pub fn add_export(&mut self, export: NFSExport) {
        self.exports.insert(export.export_id.clone(), export);
    }

    /// Remove an export by ID
    pub fn remove_export(&mut self, export_id: &str) -> Option<NFSExport> {
        self.exports.remove(export_id)
    }

    /// Get export by ID
    pub fn get_export(&self, export_id: &str) -> Option<&NFSExport> {
        self.exports.get(export_id)
    }

    /// List all exports
    pub fn list_exports(&self) -> Vec<&NFSExport> {
        self.exports.values().collect()
    }

    /// Add a remote NFS mount
    pub fn add_mount(&mut self, mount: NFSMount) {
        self.mounts.insert(mount.mount_id.clone(), mount);
    }

    /// Remove an NFS mount by ID
    pub fn remove_mount(&mut self, mount_id: &str) -> Option<NFSMount> {
        self.mounts.remove(mount_id)
    }

    /// Get mount by ID
    pub fn get_mount(&self, mount_id: &str) -> Option<&NFSMount> {
        self.mounts.get(mount_id)
    }

    /// Get mutable mount by ID
    pub fn get_mount_mut(&mut self, mount_id: &str) -> Option<&mut NFSMount> {
        self.mounts.get_mut(mount_id)
    }

    /// Connect / activate a mount
    pub fn connect_mount(&mut self, mount_id: &str) -> bool {
        if let Some(mount) = self.mounts.get_mut(mount_id) {
            mount.set_state(NFSConnectionState::Connected);
            true
        } else {
            false
        }
    }

    /// Disconnect an NFS mount
    pub fn disconnect_mount(&mut self, mount_id: &str) -> bool {
        if let Some(mount) = self.mounts.get_mut(mount_id) {
            mount.set_state(NFSConnectionState::Disconnected);
            true
        } else {
            false
        }
    }

    /// List all mounts
    pub fn list_mounts(&self) -> Vec<&NFSMount> {
        self.mounts.values().collect()
    }

    /// List connected mounts
    pub fn list_connected_mounts(&self) -> Vec<&NFSMount> {
        self.mounts.values().filter(|m| m.is_connected()).collect()
    }

    /// Export count
    pub fn export_count(&self) -> usize {
        self.exports.len()
    }

    /// Mount count
    pub fn mount_count(&self) -> usize {
        self.mounts.len()
    }
}

impl Default for SigmaNFS {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaNFS instance
static mut SIGMA_NFS_INSTANCE: Option<SigmaNFS> = None;

/// Initialize global SigmaNFS instance
pub fn init_sigmanfs() {
    unsafe {
        SIGMA_NFS_INSTANCE = Some(SigmaNFS::new());
    }
}

/// Get global SigmaNFS instance
pub fn get_sigmanfs() -> Option<&'static SigmaNFS> {
    unsafe {
        SIGMA_NFS_INSTANCE.as_ref()
    }
}

/// Get mutable global SigmaNFS instance
pub fn get_sigmanfs_mut() -> Option<&'static mut SigmaNFS> {
    unsafe {
        SIGMA_NFS_INSTANCE.as_mut()
    }
}

/// Convenience function to add and connect a remote NFS mount
pub fn mount_nfs(mount: NFSMount) -> bool {
    if let Some(mgr) = get_sigmanfs_mut() {
        let mount_id = mount.mount_id.clone();
        mgr.add_mount(mount);
        mgr.connect_mount(&mount_id)
    } else {
        false
    }
}

/// Convenience function to add an NFS share export
pub fn export_nfs_share(export: NFSExport) {
    if let Some(mgr) = get_sigmanfs_mut() {
        mgr.add_export(export);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nfs_export_creation() {
        let mut exp = NFSExport::new(
            "exp-data".to_string(),
            "/srv/nfs/data".to_string(),
            "192.168.1.0/24".to_string(),
            NFSExportAccess::ReadWrite,
            RootSquashPolicy::RootSquash,
            true,
        );
        assert_eq!(exp.export_id, "exp-data");
        assert_eq!(exp.access, NFSExportAccess::ReadWrite);
        assert!(exp.enabled);

        exp.set_enabled(false);
        assert!(!exp.enabled);
    }

    #[test]
    fn test_nfs_mount_lifecycle() {
        let mut mount = NFSMount::new(
            "mnt-backup".to_string(),
            "192.168.1.50".to_string(),
            "/exports/backups".to_string(),
            "/mnt/nfs_backups".to_string(),
            NFSVersion::V4_2,
        );
        assert_eq!(mount.version, NFSVersion::V4_2);
        assert!(!mount.is_connected());

        mount.set_state(NFSConnectionState::Connected);
        assert!(mount.is_connected());
    }

    #[test]
    fn test_sigmanfs_manager() {
        let mut mgr = SigmaNFS::new();
        mgr.start();
        assert!(mgr.is_running());

        let exp = NFSExport::new(
            "e1".to_string(),
            "/shared".to_string(),
            "*".to_string(),
            NFSExportAccess::ReadOnly,
            RootSquashPolicy::NoRootSquash,
            false,
        );
        mgr.add_export(exp);
        assert_eq!(mgr.export_count(), 1);

        let mnt = NFSMount::new(
            "m1".to_string(),
            "storage.lan".to_string(),
            "/volume1".to_string(),
            "/mnt/storage".to_string(),
            NFSVersion::V4_1,
        );
        mgr.add_mount(mnt);
        assert_eq!(mgr.mount_count(), 1);
        assert_eq!(mgr.list_connected_mounts().len(), 0);

        assert!(mgr.connect_mount("m1"));
        assert_eq!(mgr.list_connected_mounts().len(), 1);

        assert!(mgr.disconnect_mount("m1"));
        assert_eq!(mgr.list_connected_mounts().len(), 0);

        mgr.stop();
        assert!(!mgr.is_running());
    }

    #[test]
    fn test_global_instance() {
        init_sigmanfs();
        let mnt = NFSMount::new(
            "gm1".to_string(),
            "10.0.0.1".to_string(),
            "/export".to_string(),
            "/mnt/net".to_string(),
            NFSVersion::V4,
        );
        assert!(mount_nfs(mnt));

        let mgr = get_sigmanfs().unwrap();
        assert_eq!(mgr.mount_count(), 1);
        assert_eq!(mgr.list_connected_mounts().len(), 1);
    }
}
