#![no_std]

/// Volume Manager for SigmaOS
/// Implements storage volume management and mounting
/// Based on 100-Improvement-Ideas.md storage management concepts

use core::sync::atomic::{AtomicU64, Ordering};

/// Volume ID type
pub type VolumeID = u64;

/// Volume types
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VolumeType {
    Root = 0,
    Home = 1,
    Data = 2,
    Backup = 3,
    Swap = 4,
    Custom = 5,
}

/// Volume status
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VolumeStatus {
    Unmounted = 0,
    Mounted = 1,
    Error = 2,
}

/// Filesystem types
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemType {
    SigmaFS = 0,
    Ext4 = 1,
    Btrfs = 2,
    ZFS = 3,
    XFS = 4,
}

/// Volume metadata
#[repr(C)]
pub struct VolumeMetadata {
    pub id: VolumeID,
    pub name: [u8; 64],
    pub volume_type: VolumeType,
    pub filesystem: FilesystemType,
    pub size_bytes: u64,
    pub used_bytes: u64,
    pub mount_point: [u8; 256],
    pub status: VolumeStatus,
}

impl VolumeMetadata {
    pub fn new(id: VolumeID, name: &str, volume_type: VolumeType, filesystem: FilesystemType, size_bytes: u64) -> Self {
        let mut name_array = [0u8; 64];
        let name_bytes = name.as_bytes();
        let len = name_bytes.len().min(63);
        
        unsafe {
            core::ptr::copy_nonoverlapping(name_bytes.as_ptr(), name_array.as_mut_ptr(), len);
        }
        
        VolumeMetadata {
            id,
            name: name_array,
            volume_type,
            filesystem,
            size_bytes,
            used_bytes: 0,
            mount_point: [0u8; 256],
            status: VolumeStatus::Unmounted,
        }
    }
    
    pub fn name_str(&self) -> &str {
        unsafe {
            let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
            core::str::from_utf8_unchecked(&self.name[..len])
        }
    }
    
    pub fn mount_point_str(&self) -> &str {
        unsafe {
            let len = self.mount_point.iter().position(|&b| b == 0).unwrap_or(256);
            core::str::from_utf8_unchecked(&self.mount_point[..len])
        }
    }
    
    pub fn usage_percent(&self) -> f32 {
        if self.size_bytes == 0 {
            0.0
        } else {
            (self.used_bytes as f32 / self.size_bytes as f32) * 100.0
        }
    }
}

/// Volume manager
pub struct VolumeManager {
    volumes: Vec<Option<VolumeMetadata>>,
    next_volume_id: AtomicU64,
}

impl VolumeManager {
    pub fn new() -> Self {
        VolumeManager {
            volumes: Vec::new(),
            next_volume_id: AtomicU64::new(1),
        }
    }
    
    /// Create new volume
    pub fn create_volume(&mut self, name: &str, volume_type: VolumeType, filesystem: FilesystemType, size_bytes: u64) -> VolumeID {
        let id = self.next_volume_id.fetch_add(1, Ordering::SeqCst);
        let volume = VolumeMetadata::new(id, name, volume_type, filesystem, size_bytes);
        self.volumes.push(Some(volume));
        id
    }
    
    /// Mount volume
    pub fn mount_volume(&mut self, volume_id: VolumeID, mount_point: &str) -> Result<(), VolumeError> {
        for volume_option in &mut self.volumes {
            if let Some(ref mut volume) = *volume_option {
                if volume.id == volume_id {
                    let mount_bytes = mount_point.as_bytes();
                    let len = mount_bytes.len().min(255);
                    
                    unsafe {
                        core::ptr::copy_nonoverlapping(mount_bytes.as_ptr(), volume.mount_point.as_mut_ptr(), len);
                    }
                    
                    volume.status = VolumeStatus::Mounted;
                    return Ok(());
                }
            }
        }
        Err(VolumeError::VolumeNotFound)
    }
    
    /// Unmount volume
    pub fn unmount_volume(&mut self, volume_id: VolumeID) -> Result<(), VolumeError> {
        for volume_option in &mut self.volumes {
            if let Some(ref mut volume) = *volume_option {
                if volume.id == volume_id {
                    volume.status = VolumeStatus::Unmounted;
                    volume.mount_point = [0u8; 256];
                    return Ok(());
                }
            }
        }
        Err(VolumeError::VolumeNotFound)
    }
    
    /// Delete volume
    pub fn delete_volume(&mut self, volume_id: VolumeID) -> Result<(), VolumeError> {
        for volume_option in &mut self.volumes {
            if let Some(ref volume) = *volume_option {
                if volume.id == volume_id {
                    if volume.status == VolumeStatus::Mounted {
                        return Err(VolumeError::VolumeMounted);
                    }
                }
            }
        }
        
        for (i, volume_option) in self.volumes.iter().enumerate() {
            if let Some(ref volume) = *volume_option {
                if volume.id == volume_id {
                    self.volumes[i] = None;
                    return Ok(());
                }
            }
        }
        
        Err(VolumeError::VolumeNotFound)
    }
    
    /// Get volume by ID
    pub fn get_volume(&self, volume_id: VolumeID) -> Option<&VolumeMetadata> {
        for volume_option in &self.volumes {
            if let Some(ref volume) = *volume_option {
                if volume.id == volume_id {
                    return Some(volume);
                }
            }
        }
        None
    }
    
    /// List all volumes
    pub fn list_volumes(&self) -> Vec<VolumeID> {
        let mut ids = Vec::new();
        for volume_option in &self.volumes {
            if let Some(ref volume) = *volume_option {
                ids.push(volume.id);
            }
        }
        ids
    }
    
    /// Update volume usage
    pub fn update_usage(&mut self, volume_id: VolumeID, used_bytes: u64) -> Result<(), VolumeError> {
        for volume_option in &mut self.volumes {
            if let Some(ref mut volume) = *volume_option {
                if volume.id == volume_id {
                    volume.used_bytes = used_bytes;
                    return Ok(());
                }
            }
        }
        Err(VolumeError::VolumeNotFound)
    }
}

/// Volume error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum VolumeError {
    Success = 0,
    VolumeNotFound = 1,
    VolumeMounted = 2,
    MountFailed = 3,
    UnmountFailed = 4,
    InvalidSize = 5,
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
