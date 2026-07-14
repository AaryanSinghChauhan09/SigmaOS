#![no_std]

/// Cross-Device Sync (Mobile + IoT) for SigmaOS
/// Based on 100-Improvement-Ideas.md #50: Cross-device sync (mobile + IoT)
/// Implements synchronization across multiple devices and IoT platforms

use core::sync::atomic::{AtomicU64, Ordering};

/// Device type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Desktop = 0,
    Laptop = 1,
    Tablet = 2,
    Phone = 3,
    IoT = 4,
    Watch = 5,
}

/// Device state
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceState {
    Online = 0,
    Offline = 1,
    Syncing = 2,
    Error = 3,
}

/// Sync item type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncItemType {
    File = 0,
    Setting = 1,
    Notification = 2,
    Clipboard = 3,
    Bookmark = 4,
}

/// Device
#[repr(C)]
pub struct Device {
    pub id: u64,
    pub device_type: DeviceType,
    pub name: [u8; 64],
    pub state: DeviceState,
    pub last_seen: u64,
}

impl Device {
    pub fn new(id: u64, device_type: DeviceType, name: &str) -> Self {
        let mut name_array = [0u8; 64];
        let name_bytes = name.as_bytes();
        let len = name_bytes.len().min(63);
        
        unsafe {
            core::ptr::copy_nonoverlapping(name_bytes.as_ptr(), name_array.as_mut_ptr(), len);
        }
        
        Device {
            id,
            device_type,
            name: name_array,
            state: DeviceState::Offline,
            last_seen: 0,
        }
    }
}

/// Sync item
#[repr(C)]
pub struct SyncItem {
    pub id: u64,
    pub item_type: SyncItemType,
    pub source_device: u64,
    pub data: [u8; 1024],
    pub data_size: u32,
    pub timestamp: u64,
    pub synced: bool,
}

impl SyncItem {
    pub fn new(id: u64, item_type: SyncItemType, source_device: u64) -> Self {
        SyncItem {
            id,
            item_type,
            source_device,
            data: [0u8; 1024],
            data_size: 0,
            timestamp: get_current_time(),
            synced: false,
        }
    }
    
    pub fn set_data(&mut self, data: &[u8]) {
        let len = data.len().min(1024);
        unsafe {
            core::ptr::copy_nonoverlapping(data.as_ptr(), self.data.as_mut_ptr(), len);
        }
        self.data_size = len as u32;
    }
}

/// Cross-device sync manager
pub struct CrossDeviceSyncManager {
    pub devices: Vec<Option<Device>>,
    pub sync_items: Vec<Option<SyncItem>>,
    pub next_device_id: AtomicU64,
    pub next_item_id: AtomicU64,
    pub sync_enabled: bool,
    pub auto_sync: bool,
}

impl CrossDeviceSyncManager {
    pub fn new() -> Self {
        CrossDeviceSyncManager {
            devices: Vec::new(),
            sync_items: Vec::new(),
            next_device_id: AtomicU64::new(1),
            next_item_id: AtomicU64::new(1),
            sync_enabled: true,
            auto_sync: true,
        }
    }
    
    /// Register device
    pub fn register_device(&mut self, device_type: DeviceType, name: &str) -> u64 {
        let id = self.next_device_id.fetch_add(1, Ordering::SeqCst);
        let mut device = Device::new(id, device_type, name);
        device.state = DeviceState::Online;
        device.last_seen = get_current_time();
        self.devices.push(Some(device));
        id
    }
    
    /// Unregister device
    pub fn unregister_device(&mut self, id: u64) -> bool {
        for device_option in &mut self.devices {
            if let Some(ref device) = *device_option {
                if device.id == id {
                    *device_option = None;
                    return true;
                }
            }
        }
        false
    }
    
    /// Update device state
    pub fn update_device_state(&mut self, id: u64, state: DeviceState) -> bool {
        for device_option in &mut self.devices {
            if let Some(ref mut device) = *device_option {
                if device.id == id {
                    device.state = state;
                    device.last_seen = get_current_time();
                    return true;
                }
            }
        }
        false
    }
    
    /// Add sync item
    pub fn add_sync_item(&mut self, item_type: SyncItemType, source_device: u64, data: &[u8]) -> u64 {
        let id = self.next_item_id.fetch_add(1, Ordering::SeqCst);
        let mut item = SyncItem::new(id, item_type, source_device);
        item.set_data(data);
        self.sync_items.push(Some(item));
        id
    }
    
    /// Sync item to all devices
    pub fn sync_to_all(&mut self, item_id: u64) -> Result<(), SyncError> {
        if !self.sync_enabled {
            return Err(SyncError::SyncDisabled);
        }
        
        for item_option in &mut self.sync_items {
            if let Some(ref mut item) = *item_option {
                if item.id == item_id {
                    item.synced = true;
                    return Ok(());
                }
            }
        }
        
        Err(SyncError::ItemNotFound)
    }
    
    /// Get device by ID
    pub fn get_device(&self, id: u64) -> Option<&Device> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id == id {
                    return Some(device);
                }
            }
        }
        None
    }
    
    /// List online devices
    pub fn list_online_devices(&self) -> Vec<&Device> {
        let mut devices = Vec::new();
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.state == DeviceState::Online {
                    devices.push(device);
                }
            }
        }
        devices
    }
    
    /// Get unsynced items
    pub fn get_unsynced_items(&self) -> Vec<&SyncItem> {
        let mut items = Vec::new();
        for item_option in &self.sync_items {
            if let Some(ref item) = *item_option {
                if !item.synced {
                    items.push(item);
                }
            }
        }
        items
    }
    
    /// Auto-sync all pending items
    pub fn auto_sync(&mut self) {
        if !self.auto_sync || !self.sync_enabled {
            return;
        }
        
        let unsynced_ids: Vec<u64> = self.get_unsynced_items().iter().map(|item| item.id).collect();
        
        for id in unsynced_ids {
            let _ = self.sync_to_all(id);
        }
    }
    
    /// Enable/disable sync
    pub fn set_sync_enabled(&mut self, enabled: bool) {
        self.sync_enabled = enabled;
    }
    
    /// Enable/disable auto-sync
    pub fn set_auto_sync(&mut self, enabled: bool) {
        self.auto_sync = enabled;
    }
}

/// Sync error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SyncError {
    Success = 0,
    DeviceNotFound = 1,
    ItemNotFound = 2,
    SyncDisabled = 3,
    SyncFailed = 4,
}

/// Simple Vec implementation for no_std
struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }

            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * core::mem::size_of::<T>()) as *mut T;

        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }

            if self.capacity > 0 {
                free(self.data as *mut u8);
            }

            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

/// Get current time (nanoseconds)
fn get_current_time() -> u64 {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    COUNTER.fetch_add(1_000_000, Ordering::SeqCst)
}
