#![no_std]
#![no_main]

/// OOP-based Volume Management for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 241
/// Implements logical volume management

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type VolumeID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum VolumeType { Linear = 0, Stripe = 1, Mirror = 2, RAID5 = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum VolumeError { Success = 0, NotFound = 1, CreationFailed = 2 }

pub trait Volume {
    fn id(&self) -> VolumeID;
    fn name(&self) -> &[u8];
    fn volume_type(&self) -> VolumeType;
    fn size(&self) -> u64;
    fn is_mounted(&self) -> bool;
}

#[repr(C)]
pub struct SimpleVolume {
    pub id: VolumeID,
    pub name: [u8; 64],
    pub volume_type: AtomicUsize,
    pub size: AtomicUsize,
    pub mounted: AtomicUsize,
}

impl SimpleVolume {
    pub fn new(id: VolumeID, name: &[u8], volume_type: VolumeType, size: u64) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        SimpleVolume {
            id,
            name: name_array,
            volume_type: AtomicUsize::new(volume_type as usize),
            size: AtomicUsize::new(size as usize),
            mounted: AtomicUsize::new(0),
        }
    }
}

impl Volume for SimpleVolume {
    fn id(&self) -> VolumeID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn volume_type(&self) -> VolumeType { unsafe { core::mem::transmute(self.volume_type.load(Ordering::SeqCst)) } }
    fn size(&self) -> u64 { self.size.load(Ordering::SeqCst) as u64 }
    fn is_mounted(&self) -> bool { self.mounted.load(Ordering::SeqCst) == 1 }
}

pub trait VolumeManager {
    fn create_volume(&mut self, name: &[u8], volume_type: VolumeType, size: u64) -> Result<VolumeID, VolumeError>;
    fn delete_volume(&mut self, id: VolumeID) -> Result<(), VolumeError>;
    fn get_volume(&self, id: VolumeID) -> Option<&dyn Volume>;
    fn mount_volume(&mut self, id: VolumeID) -> Result<(), VolumeError>;
    fn unmount_volume(&mut self, id: VolumeID) -> Result<(), VolumeError>;
}

#[repr(C)]
pub struct SimpleVolumeManager {
    pub volumes: Vec<Option<Box<dyn Volume>>>,
    pub next_id: AtomicUsize,
}

impl SimpleVolumeManager {
    pub fn new() -> Self {
        SimpleVolumeManager {
            volumes: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl VolumeManager for SimpleVolumeManager {
    fn create_volume(&mut self, name: &[u8], volume_type: VolumeType, size: u64) -> Result<VolumeID, VolumeError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let volume = SimpleVolume::new(id, name, volume_type, size);
        self.volumes.push(Some(Box::new(volume)));
        Ok(id)
    }
    
    fn delete_volume(&mut self, id: VolumeID) -> Result<(), VolumeError> {
        for volume_option in &mut self.volumes {
            if let Some(ref volume) = *volume_option {
                if volume.id() == id {
                    return Ok(());
                }
            }
        }
        Err(VolumeError::NotFound)
    }
    
    fn get_volume(&self, id: VolumeID) -> Option<&dyn Volume> {
        for volume_option in &self.volumes {
            if let Some(ref volume) = *volume_option {
                if volume.id() == id { return Some(volume.as_ref()); }
            }
        }
        None
    }
    
    fn mount_volume(&mut self, id: VolumeID) -> Result<(), VolumeError> {
        for volume_option in &mut self.volumes {
            if let Some(ref mut volume) = *volume_option {
                if volume.id() == id {
                    volume.mounted.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(VolumeError::NotFound)
    }
    
    fn unmount_volume(&mut self, id: VolumeID) -> Result<(), VolumeError> {
        for volume_option in &mut self.volumes {
            if let Some(ref mut volume) = *volume_option {
                if volume.id() == id {
                    volume.mounted.store(0, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(VolumeError::NotFound)
    }
}

pub trait SnapshotManager {
    fn create_snapshot(&mut self, volume_id: VolumeID) -> Result<VolumeID, VolumeError>;
    fn delete_snapshot(&mut self, snapshot_id: VolumeID) -> Result<(), VolumeError>;
    def restore_snapshot(&mut self, volume_id: VolumeID, snapshot_id: VolumeID) -> Result<(), VolumeError>;
}

#[repr(C)]
pub struct SimpleSnapshotManager {
    pub snapshots: Vec<(VolumeID, VolumeID)>,
}

impl SimpleSnapshotManager {
    pub fn new() -> Self {
        SimpleSnapshotManager {
            snapshots: Vec::new(),
        }
    }
}

impl SnapshotManager for SimpleSnapshotManager {
    fn create_snapshot(&mut self, volume_id: VolumeID) -> Result<VolumeID, VolumeError> {
        let snapshot_id = volume_id + 1000;
        self.snapshots.push((volume_id, snapshot_id));
        Ok(snapshot_id)
    }
    
    fn delete_snapshot(&mut self, snapshot_id: VolumeID) -> Result<(), VolumeError> {
        for i in 0..self.snapshots.len() {
            if self.snapshots[i].1 == snapshot_id {
                self.snapshots.remove(i);
                return Ok(());
            }
        }
        Err(VolumeError::NotFound)
    }
    
    fn restore_snapshot(&mut self, _volume_id: VolumeID, _snapshot_id: VolumeID) -> Result<(), VolumeError> {
        Ok(())
    }
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
    fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
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
