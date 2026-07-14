#![no_std]
#![no_main]

/// OOP-based Block Storage for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 101
/// Implements block device abstraction and storage management

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type BlockDeviceID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BlockDeviceType { HDD = 0, SSD = 1, NVMe = 2, Virtual = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BlockError { Success = 0, NotFound = 1, ReadFailed = 2, WriteFailed = 3 }

pub trait BlockDevice {
    fn id(&self) -> BlockDeviceID;
    fn device_type(&self) -> BlockDeviceType;
    fn block_size(&self) -> usize;
    fn total_blocks(&self) -> usize;
    fn read_block(&self, block_num: usize, buffer: &mut [u8]) -> Result<(), BlockError>;
    fn write_block(&mut self, block_num: usize, data: &[u8]) -> Result<(), BlockError>;
}

#[repr(C)]
pub struct SimpleBlockDevice {
    pub id: BlockDeviceID,
    pub device_type: AtomicUsize,
    pub block_size: AtomicUsize,
    pub total_blocks: AtomicUsize,
}

impl SimpleBlockDevice {
    pub fn new(id: BlockDeviceID, device_type: BlockDeviceType, block_size: usize, total_blocks: usize) -> Self {
        SimpleBlockDevice {
            id,
            device_type: AtomicUsize::new(device_type as usize),
            block_size: AtomicUsize::new(block_size),
            total_blocks: AtomicUsize::new(total_blocks),
        }
    }
}

impl BlockDevice for SimpleBlockDevice {
    fn id(&self) -> BlockDeviceID { self.id }
    fn device_type(&self) -> BlockDeviceType { unsafe { core::mem::transmute(self.device_type.load(Ordering::SeqCst)) } }
    fn block_size(&self) -> usize { self.block_size.load(Ordering::SeqCst) }
    fn total_blocks(&self) -> usize { self.total_blocks.load(Ordering::SeqCst) }
    
    fn read_block(&self, _block_num: usize, _buffer: &mut [u8]) -> Result<(), BlockError> {
        Ok(())
    }
    
    fn write_block(&mut self, _block_num: usize, _data: &[u8]) -> Result<(), BlockError> {
        Ok(())
    }
}

pub trait BlockManager {
    fn register_device(&mut self, device: Box<dyn BlockDevice>) -> Result<BlockDeviceID, BlockError>;
    fn unregister_device(&mut self, id: BlockDeviceID) -> Result<(), BlockError>;
    fn get_device(&self, id: BlockDeviceID) -> Option<&dyn BlockDevice>;
    fn list_devices(&self) -> Vec<BlockDeviceID>;
}

#[repr(C)]
pub struct SimpleBlockManager {
    pub devices: Vec<Option<Box<dyn BlockDevice>>>,
    pub next_id: AtomicUsize,
}

impl SimpleBlockManager {
    pub fn new() -> Self {
        SimpleBlockManager {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl BlockManager for SimpleBlockManager {
    fn register_device(&mut self, device: Box<dyn BlockDevice>) -> Result<BlockDeviceID, BlockError> {
        let id = device.id();
        self.devices.push(Some(device));
        Ok(id)
    }
    
    fn unregister_device(&mut self, id: BlockDeviceID) -> Result<(), BlockError> {
        for device_option in &mut self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id {
                    return Ok(());
                }
            }
        }
        Err(BlockError::NotFound)
    }
    
    fn get_device(&self, id: BlockDeviceID) -> Option<&dyn BlockDevice> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
    }
    
    fn list_devices(&self) -> Vec<BlockDeviceID> {
        let mut ids = Vec::new();
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                ids.push(device.id());
            }
        }
        ids
    }
}

pub trait PartitionTable {
    fn create_partition(&mut self, device_id: BlockDeviceID, start_block: usize, size_blocks: usize) -> Result<usize, BlockError>;
    fn delete_partition(&mut self, partition_id: usize) -> Result<(), BlockError>;
    fn list_partitions(&self, device_id: BlockDeviceID) -> Vec<(usize, usize, usize)>;
}

#[repr(C)]
pub struct SimplePartitionTable {
    pub partitions: Vec<(BlockDeviceID, usize, usize, usize)>,
    pub next_id: AtomicUsize,
}

impl SimplePartitionTable {
    pub fn new() -> Self {
        SimplePartitionTable {
            partitions: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl PartitionTable for SimplePartitionTable {
    fn create_partition(&mut self, device_id: BlockDeviceID, start_block: usize, size_blocks: usize) -> Result<usize, BlockError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        self.partitions.push((device_id, id, start_block, size_blocks));
        Ok(id)
    }
    
    fn delete_partition(&mut self, partition_id: usize) -> Result<(), BlockError> {
        for i in 0..self.partitions.len() {
            if self.partitions[i].1 == partition_id {
                self.partitions.remove(i);
                return Ok(());
            }
        }
        Err(BlockError::NotFound)
    }
    
    fn list_partitions(&self, device_id: BlockDeviceID) -> Vec<(usize, usize, usize)> {
        let mut result = Vec::new();
        for &(dev_id, part_id, start, size) in &self.partitions {
            if dev_id == device_id {
                result.push((part_id, start, size));
            }
        }
        result
    }
}

pub trait BlockCache {
    fn read_cached(&mut self, device_id: BlockDeviceID, block_num: usize) -> Option<&[u8]>;
    fn write_cache(&mut self, device_id: BlockDeviceID, block_num: usize, data: &[u8]);
    fn invalidate(&mut self, device_id: BlockDeviceID, block_num: usize);
}

#[repr(C)]
pub struct SimpleBlockCache {
    pub cache: Vec<(BlockDeviceID, usize, [u8; 4096])>,
    pub max_entries: AtomicUsize,
}

impl SimpleBlockCache {
    pub fn new(max_entries: usize) -> Self {
        SimpleBlockCache {
            cache: Vec::new(),
            max_entries: AtomicUsize::new(max_entries),
        }
    }
}

impl BlockCache for SimpleBlockCache {
    fn read_cached(&mut self, device_id: BlockDeviceID, block_num: usize) -> Option<&[u8]> {
        for &(dev_id, blk_num, ref data) in &self.cache {
            if dev_id == device_id && blk_num == block_num {
                return Some(data);
            }
        }
        None
    }
    
    fn write_cache(&mut self, device_id: BlockDeviceID, block_num: usize, data: &[u8]) {
        let max = self.max_entries.load(Ordering::SeqCst);
        if self.cache.len() >= max {
            self.cache.remove(0);
        }
        
        let mut data_array = [0u8; 4096];
        let data_len = data.len().min(4095);
        for i in 0..data_len {
            data_array[i] = data[i];
        }
        
        self.cache.push((device_id, block_num, data_array));
    }
    
    fn invalidate(&mut self, device_id: BlockDeviceID, block_num: usize) {
        for i in 0..self.cache.len() {
            if self.cache[i].0 == device_id && self.cache[i].1 == block_num {
                self.cache.remove(i);
                return;
            }
        }
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
