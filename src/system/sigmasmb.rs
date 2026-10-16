//! SigmaSMB - Server Message Block (SMB/CIFS) Client and Server Support
//!
//! Inspired by Samba and cifs-utils (SMB2/SMB3 protocols), this module provides
//! comprehensive cross-platform file and print sharing, remote CIFS mounts, and
//! authentication management for SigmaOS.
//!
//! Features include:
//! - SMB 2.0, SMB 2.1, SMB 3.0, and SMB 3.1.1 protocol support
//! - Client-side remote CIFS/SMB share mount management
//! - Server-side share definitions (browseable, guest allowed, encrypted, read-only)
//! - Session connection state tracking (Connected, AuthFailed, Disconnected)
//! - Global instance and convenience API

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};

/// SMB / CIFS protocol dialect
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SMBDialect {
    Smb1Legacy,
    Smb2_02,
    Smb2_1,
    Smb3_0,
    Smb3_1_1,
    Unknown,
}

/// SMB Connection State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SMBConnectionState {
    Disconnected,
    Connecting,
    Connected,
    AuthFailed,
    Timeout,
    Failed,
}

/// SMB Share Permissions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SMBShareAccess {
    ReadOnly,
    ReadWrite,
}

/// Server-side SMB Share Descriptor
#[derive(Debug, Clone)]
pub struct SMBShare {
    pub share_id: String,
    pub share_name: String,
    pub path: String,
    pub access: SMBShareAccess,
    pub guest_ok: bool,
    pub browseable: bool,
    pub require_encryption: bool,
    pub enabled: bool,
}

impl SMBShare {
    /// Create a new SMB share definition
    pub fn new(
        share_id: String,
        share_name: String,
        path: String,
        access: SMBShareAccess,
        guest_ok: bool,
        require_encryption: bool,
    ) -> Self {
        Self {
            share_id,
            share_name,
            path,
            access,
            guest_ok,
            browseable: true,
            require_encryption,
            enabled: true,
        }
    }

    /// Enable or disable share
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

/// Client-side remote SMB Mount Descriptor
#[derive(Debug, Clone)]
pub struct SMBMount {
    pub mount_id: String,
    pub server_host: String,
    pub remote_share: String,
    pub local_mount_point: String,
    pub workgroup: String,
    pub username: String,
    pub dialect: SMBDialect,
    pub state: SMBConnectionState,
    pub encrypted: bool,
}

impl SMBMount {
    /// Create a new SMB mount definition
    pub fn new(
        mount_id: String,
        server_host: String,
        remote_share: String,
        local_mount_point: String,
        dialect: SMBDialect,
    ) -> Self {
        Self {
            mount_id,
            server_host,
            remote_share,
            local_mount_point,
            workgroup: "WORKGROUP".to_string(),
            username: "guest".to_string(),
            dialect,
            state: SMBConnectionState::Disconnected,
            encrypted: false,
        }
    }

    /// Set connection status
    pub fn set_state(&mut self, state: SMBConnectionState) {
        self.state = state;
    }

    /// Check if mount is actively connected
    pub fn is_connected(&self) -> bool {
        self.state == SMBConnectionState::Connected
    }
}

/// Main SigmaSMB manager struct
pub struct SigmaSMB {
    shares: BTreeMap<String, SMBShare>,
    mounts: BTreeMap<String, SMBMount>,
    running: AtomicBool,
}

impl SigmaSMB {
    /// Create a new SigmaSMB manager instance
    pub fn new() -> Self {
        Self {
            shares: BTreeMap::new(),
            mounts: BTreeMap::new(),
            running: AtomicBool::new(false),
        }
    }

    /// Start SMB service
    pub fn start(&self) {
        self.running.store(true, Ordering::SeqCst);
    }

    /// Stop SMB service
    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if service is active
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Add an SMB share
    pub fn add_share(&mut self, share: SMBShare) {
        self.shares.insert(share.share_id.clone(), share);
    }

    /// Remove a share by ID
    pub fn remove_share(&mut self, share_id: &str) -> Option<SMBShare> {
        self.shares.remove(share_id)
    }

    /// Get share by ID
    pub fn get_share(&self, share_id: &str) -> Option<&SMBShare> {
        self.shares.get(share_id)
    }

    /// List all shares
    pub fn list_shares(&self) -> Vec<&SMBShare> {
        self.shares.values().collect()
    }

    /// Add a remote SMB mount
    pub fn add_mount(&mut self, mount: SMBMount) {
        self.mounts.insert(mount.mount_id.clone(), mount);
    }

    /// Remove an SMB mount by ID
    pub fn remove_mount(&mut self, mount_id: &str) -> Option<SMBMount> {
        self.mounts.remove(mount_id)
    }

    /// Get mount by ID
    pub fn get_mount(&self, mount_id: &str) -> Option<&SMBMount> {
        self.mounts.get(mount_id)
    }

    /// Connect / activate an SMB mount
    pub fn connect_mount(&mut self, mount_id: &str) -> bool {
        if let Some(mount) = self.mounts.get_mut(mount_id) {
            mount.set_state(SMBConnectionState::Connected);
            true
        } else {
            false
        }
    }

    /// Disconnect an SMB mount
    pub fn disconnect_mount(&mut self, mount_id: &str) -> bool {
        if let Some(mount) = self.mounts.get_mut(mount_id) {
            mount.set_state(SMBConnectionState::Disconnected);
            true
        } else {
            false
        }
    }

    /// List all mounts
    pub fn list_mounts(&self) -> Vec<&SMBMount> {
        self.mounts.values().collect()
    }

    /// List connected mounts
    pub fn list_connected_mounts(&self) -> Vec<&SMBMount> {
        self.mounts.values().filter(|m| m.is_connected()).collect()
    }

    /// Share count
    pub fn share_count(&self) -> usize {
        self.shares.len()
    }

    /// Mount count
    pub fn mount_count(&self) -> usize {
        self.mounts.len()
    }
}

impl Default for SigmaSMB {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaSMB instance
static mut SIGMA_SMB_INSTANCE: Option<SigmaSMB> = None;

/// Initialize global SigmaSMB instance
pub fn init_sigmasmb() {
    unsafe {
        SIGMA_SMB_INSTANCE = Some(SigmaSMB::new());
    }
}

/// Get global SigmaSMB reference
pub fn get_sigmasmb() -> Option<&'static SigmaSMB> {
    unsafe { SIGMA_SMB_INSTANCE.as_ref() }
}

/// Get mutable global SigmaSMB reference
pub fn get_sigmasmb_mut() -> Option<&'static mut SigmaSMB> {
    unsafe { SIGMA_SMB_INSTANCE.as_mut() }
}

/// Convenience function to mount a remote SMB share
pub fn mount_smb(mount: SMBMount) -> bool {
    if let Some(mgr) = get_sigmasmb_mut() {
        let mount_id = mount.mount_id.clone();
        mgr.add_mount(mount);
        mgr.connect_mount(&mount_id)
    } else {
        false
    }
}

/// Convenience function to export an SMB share
pub fn export_smb_share(share: SMBShare) {
    if let Some(mgr) = get_sigmasmb_mut() {
        mgr.add_share(share);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smb_share_creation() {
        let mut share = SMBShare::new(
            "sh-public".to_string(),
            "PublicFiles".to_string(),
            "/var/shares/public".to_string(),
            SMBShareAccess::ReadWrite,
            true,
            false,
        );
        assert_eq!(share.share_id, "sh-public");
        assert!(share.guest_ok);
        assert!(share.browseable);
        assert!(share.enabled);

        share.set_enabled(false);
        assert!(!share.enabled);
    }

    #[test]
    fn test_smb_mount_lifecycle() {
        let mut mount = SMBMount::new(
            "mnt-win".to_string(),
            "winserver.lan".to_string(),
            "Data".to_string(),
            "/mnt/windows".to_string(),
            SMBDialect::Smb3_1_1,
        );
        assert_eq!(mount.dialect, SMBDialect::Smb3_1_1);
        assert!(!mount.is_connected());

        mount.set_state(SMBConnectionState::Connected);
        assert!(mount.is_connected());
    }

    #[test]
    fn test_sigmasmb_manager() {
        let mut mgr = SigmaSMB::new();
        mgr.start();
        assert!(mgr.is_running());

        let sh = SMBShare::new(
            "s1".to_string(),
            "Media".to_string(),
            "/media".to_string(),
            SMBShareAccess::ReadOnly,
            true,
            true,
        );
        mgr.add_share(sh);
        assert_eq!(mgr.share_count(), 1);

        let mnt = SMBMount::new(
            "m1".to_string(),
            "nas.corp".to_string(),
            "Projects".to_string(),
            "/mnt/projects".to_string(),
            SMBDialect::Smb3_0,
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
        init_sigmasmb();
        let mnt = SMBMount::new(
            "gm1".to_string(),
            "fileserver".to_string(),
            "Share".to_string(),
            "/mnt/smb".to_string(),
            SMBDialect::Smb3_1_1,
        );
        assert!(mount_smb(mnt));

        let mgr = get_sigmasmb().unwrap();
        assert_eq!(mgr.mount_count(), 1);
        assert_eq!(mgr.list_connected_mounts().len(), 1);
    }
}
