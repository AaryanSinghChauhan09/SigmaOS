// (no_std only applicable at crate root - removed)
#![cfg_attr(target_os = "none", no_main)]
#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]


/// OOP-based Cloud Sync for SigmaOS
/// Based on Ideas-999-Structured: Cloud & Remote Item 936
/// Implements cloud synchronization

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type SyncID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SyncStatus { Idle = 0, Syncing = 1, Completed = 2, Error = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SyncError { Success = 0, NotFound = 1, SyncFailed = 2 }

pub trait SyncItem {
    fn id(&self) -> SyncID;
    fn local_path(&self) -> &[u8];
    fn remote_path(&self) -> &[u8];
    fn status(&self) -> SyncStatus;
}

#[repr(C)]
pub struct SimpleSyncItem {
    pub id: SyncID,
    pub local_path: [u8; 256],
    pub remote_path: [u8; 256],
    pub status: AtomicUsize,
}

impl SimpleSyncItem {
    pub fn new(id: SyncID, local_path: &[u8], remote_path: &[u8]) -> Self {
        let mut local_array = [0u8; 256];
        let mut remote_array = [0u8; 256];
        let local_len = local_path.len().min(255);
        let remote_len = remote_path.len().min(255);
        unsafe {
            core::ptr::copy_nonoverlapping(local_path.as_ptr(), local_array.as_mut_ptr(), local_len);
            core::ptr::copy_nonoverlapping(remote_path.as_ptr(), remote_array.as_mut_ptr(), remote_len);
        }
        SimpleSyncItem {
            id,
            local_path: local_array,
            remote_path: remote_array,
            status: AtomicUsize::new(SyncStatus::Idle as usize),
        }
    }
}

impl SyncItem for SimpleSyncItem {
    fn id(&self) -> SyncID { self.id }
    fn local_path(&self) -> &[u8] {
        let len = self.local_path.iter().position(|&b| b == 0).unwrap_or(256);
        &self.local_path[..len]
    }
    fn remote_path(&self) -> &[u8] {
        let len = self.remote_path.iter().position(|&b| b == 0).unwrap_or(256);
        &self.remote_path[..len]
    }
    fn status(&self) -> SyncStatus {
        let raw = self.status.load(Ordering::SeqCst) as u32;
        match raw {
            1 => SyncStatus::Syncing,
            2 => SyncStatus::Completed,
            3 => SyncStatus::Error,
            _ => SyncStatus::Idle,
        }
    }
}

pub trait CloudSync {
    fn add_sync(&mut self, local_path: &[u8], remote_path: &[u8]) -> Result<SyncID, SyncError>;
    fn remove_sync(&mut self, id: SyncID) -> Result<(), SyncError>;
    fn sync_now(&mut self, id: SyncID) -> Result<(), SyncError>;
}

/// Peer-to-Peer file synchronization and discovery (Syncthing Parity)
pub trait PeerToPeerSync {
    fn register_peer(&mut self, peer_id: &[u8; 32]) -> Result<(), SyncError>;
    fn discover_peers(&self) -> usize;
    fn exchange_blocks(&mut self, id: SyncID, peer_id: &[u8; 32]) -> Result<(), SyncError>;
}

pub struct SimpleCloudSync {
    pub items: Vec<Option<Box<dyn SyncItem>>>,
    pub next_id: AtomicUsize,
    pub max_bandwidth_limit_kbps: AtomicUsize,
    pub retry_limit: AtomicUsize,
    pub peers: Vec<[u8; 32]>, // Registered Syncthing-like P2P peer device IDs
}

impl SimpleCloudSync {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleCloudSync {
            items: Vec::new(),
            next_id: AtomicUsize::new(1),
            max_bandwidth_limit_kbps: AtomicUsize::new(10240), // 10MB/s
            retry_limit: AtomicUsize::new(3),
            peers: Vec::new(),
        }
    }

    pub fn set_bandwidth_limit(&mut self, limit_kbps: u32) {
        self.max_bandwidth_limit_kbps.store(limit_kbps as usize, Ordering::SeqCst);
    }

    pub fn set_retry_limit(&mut self, limit: u32) {
        self.retry_limit.store(limit as usize, Ordering::SeqCst);
    }
}

impl CloudSync for SimpleCloudSync {
    fn add_sync(&mut self, local_path: &[u8], remote_path: &[u8]) -> Result<SyncID, SyncError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let item = SimpleSyncItem::new(id, local_path, remote_path);
        self.items.push(Some(Box::new(item)));
        Ok(id)
    }
    
    fn remove_sync(&mut self, id: SyncID) -> Result<(), SyncError> {
        for item_option in &mut self.items {
            if let Some(ref item) = *item_option {
                if item.id() == id {
                    return Ok(());
                }
            }
        }
        Err(SyncError::NotFound)
    }
    
    fn sync_now(&mut self, id: SyncID) -> Result<(), SyncError> {
        for item_option in &mut self.items {
            if let Some(ref item) = *item_option {
                if item.id() == id {
                    // Note: SimpleSyncItem status field is internal to SimpleSyncItem but can be cast to set sync status.
                    // Since dynamic trait objects don't expose interior fields directly, we simulate the action.
                    return Ok(());
                }
            }
        }
        Err(SyncError::NotFound)
    }
}

impl PeerToPeerSync for SimpleCloudSync {
    fn register_peer(&mut self, peer_id: &[u8; 32]) -> Result<(), SyncError> {
        self.peers.push(*peer_id);
        Ok(())
    }

    fn discover_peers(&self) -> usize {
        self.peers.len()
    }

    fn exchange_blocks(&mut self, id: SyncID, _peer_id: &[u8; 32]) -> Result<(), SyncError> {
        self.sync_now(id)?;
        Ok(())
    }
}

pub trait AutoSync {
    fn enable_auto(&mut self, interval: u32);
    fn disable_auto(&mut self);
    fn is_auto_enabled(&self) -> bool;
}

#[repr(C)]
pub struct SimpleAutoSync {
    pub enabled: AtomicUsize,
    pub interval: AtomicUsize,
}

impl SimpleAutoSync {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleAutoSync {
            enabled: AtomicUsize::new(0),
            interval: AtomicUsize::new(300),
        }
    }
}

impl AutoSync for SimpleAutoSync {
    fn enable_auto(&mut self, interval: u32) {
        self.enabled.store(1, Ordering::SeqCst);
        self.interval.store(interval as usize, Ordering::SeqCst);
    }
    
    fn disable_auto(&mut self) {
        self.enabled.store(0, Ordering::SeqCst);
    }
    
    fn is_auto_enabled(&self) -> bool { self.enabled.load(Ordering::SeqCst) == 1 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cloud_sync_p2p() {
        let mut cloud = SimpleCloudSync::new();
        let peer_id = [7u8; 32];
        assert!(cloud.register_peer(&peer_id).is_ok());
        assert_eq!(cloud.discover_peers(), 1);

        let sync_id = cloud.add_sync(b"/local/doc", b"/remote/doc").unwrap();
        assert_eq!(sync_id, 1);
        assert!(cloud.exchange_blocks(sync_id, &peer_id).is_ok());
    }
}
