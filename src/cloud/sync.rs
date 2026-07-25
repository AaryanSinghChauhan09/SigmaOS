#![no_std]
#![no_main]

/// OOP-based Cloud Sync for SigmaOS
/// Based on Ideas-999-Structured: Cloud & Remote Item 936
/// Implements cloud synchronization

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

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
    fn status(&self) -> SyncStatus { unsafe { core::mem::transmute(self.status.load(Ordering::SeqCst)) } }
}

pub trait CloudSync {
    fn add_sync(&mut self, local_path: &[u8], remote_path: &[u8]) -> Result<SyncID, SyncError>;
    fn remove_sync(&mut self, id: SyncID) -> Result<(), SyncError>;
    def sync_now(&mut self, id: SyncID) -> Result<(), SyncError>;
}

#[repr(C)]
pub struct SimpleCloudSync {
    pub items: Vec<Option<Box<dyn SyncItem>>>,
    pub next_id: AtomicUsize,
}

impl SimpleCloudSync {
    pub fn new() -> Self {
        SimpleCloudSync {
            items: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
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
            if let Some(ref mut item) = *item_option {
                if item.id() == id {
                    item.status.store(SyncStatus::Syncing as usize, Ordering::SeqCst);
                    item.status.store(SyncStatus::Completed as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(SyncError::NotFound)
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

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
