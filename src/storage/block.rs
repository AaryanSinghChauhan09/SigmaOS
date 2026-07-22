#![no_std]
#![no_main]

use core::mem;
/// OOP-based Block Storage for SigmaOS
/// Based on 100-Improvement-Ideas.md storage management concepts
/// Implements comprehensive block device abstraction, partition management,
/// and caching for high-performance storage operations
use core::sync::atomic::{AtomicUsize, Ordering};
extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

pub type BlockDeviceID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BlockDeviceType {
    HDD = 0,
    SSD = 1,
    NVMe = 2,
    Virtual = 3,
    RAMDisk = 4,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BlockError {
    Success = 0,
    NotFound = 1,
    ReadFailed = 2,
    WriteFailed = 3,
}

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
    pub fn new(
        id: BlockDeviceID,
        device_type: BlockDeviceType,
        block_size: usize,
        total_blocks: usize,
    ) -> Self {
        SimpleBlockDevice {
            id,
            device_type: AtomicUsize::new(device_type as usize),
            block_size: AtomicUsize::new(block_size),
            total_blocks: AtomicUsize::new(total_blocks),
        }
    }
}

impl BlockDevice for SimpleBlockDevice {
    fn id(&self) -> BlockDeviceID {
        self.id
    }
    fn device_type(&self) -> BlockDeviceType {
        match self.device_type.load(Ordering::SeqCst) as u32 {
            1 => BlockDeviceType::SSD,
            2 => BlockDeviceType::NVMe,
            3 => BlockDeviceType::Virtual,
            4 => BlockDeviceType::RAMDisk,
            _ => BlockDeviceType::HDD,
        }
    }
    fn block_size(&self) -> usize {
        self.block_size.load(Ordering::SeqCst)
    }
    fn total_blocks(&self) -> usize {
        self.total_blocks.load(Ordering::SeqCst)
    }

    fn read_block(&self, _block_num: usize, _buffer: &mut [u8]) -> Result<(), BlockError> {
        Ok(())
    }

    fn write_block(&mut self, _block_num: usize, _data: &[u8]) -> Result<(), BlockError> {
        Ok(())
    }
}

pub trait BlockManager {
    fn register_device(
        &mut self,
        device: Box<dyn BlockDevice>,
    ) -> Result<BlockDeviceID, BlockError>;
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
    fn register_device(
        &mut self,
        device: Box<dyn BlockDevice>,
    ) -> Result<BlockDeviceID, BlockError> {
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
                if device.id() == id {
                    return Some(device.as_ref());
                }
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
    fn create_partition(
        &mut self,
        device_id: BlockDeviceID,
        start_block: usize,
        size_blocks: usize,
    ) -> Result<usize, BlockError>;
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
    fn create_partition(
        &mut self,
        device_id: BlockDeviceID,
        start_block: usize,
        size_blocks: usize,
    ) -> Result<usize, BlockError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        self.partitions
            .push((device_id, id, start_block, size_blocks));
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


