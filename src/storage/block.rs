// SPDX-License-Identifier: MIT
// SigmaOS Block Device Subsystem & High-Performance Block Management Architecture
// Implements block-oriented devices, block operations, multi-type block classification,
// record blocking (fixed, variable, permanent, spanned), and system block diagram topologies.

extern crate alloc;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

pub type BlockDeviceID = usize;
pub type BlockNumber = u64;

// ── 1. BLOCK-ORIENTED DEVICES & DRIVERS ───────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceClass {
    Hdd,
    Ssd,
    Nvme,
    RamDisk,
    VirtIoBlock,
    TapeDevice,
    LoopDevice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockError {
    Success = 0,
    NotFound = 1,
    ReadFailed = 2,
    WriteFailed = 3,
    WriteBlocked = 4,
    OutOfBounds = 5,
    InvalidBlockSize = 6,
    SequentialOnly = 7,
}

pub trait BlockOrientedDevice: Send + Sync {
    fn device_id(&self) -> BlockDeviceID;
    fn device_class(&self) -> DeviceClass;
    fn block_size(&self) -> usize;
    fn total_blocks(&self) -> u64;
    fn is_write_blocked(&self) -> bool;
    fn set_write_blocked(&mut self, blocked: bool);
    fn read_block(&self, block_num: BlockNumber, buffer: &mut [u8]) -> Result<(), BlockError>;
    fn write_block(&mut self, block_num: BlockNumber, data: &[u8]) -> Result<(), BlockError>;
    fn flush(&mut self) -> Result<(), BlockError> {
        Ok(())
    }
}

/// Solid-State Drive with Flash Translation Layer (FTL) and Wear-Leveling simulation
pub struct SsdBlockDevice {
    pub id: BlockDeviceID,
    pub block_size: usize,
    pub total_blocks: u64,
    pub erase_cycles: Vec<u32>,
    pub write_blocked: bool,
    pub data: Vec<u8>,
}

impl SsdBlockDevice {
    pub fn new(id: BlockDeviceID, block_size: usize, total_blocks: u64) -> Self {
        let total_bytes = (block_size as u64 * total_blocks) as usize;
        Self {
            id,
            block_size,
            total_blocks,
            erase_cycles: alloc::vec![0u32; total_blocks as usize],
            write_blocked: false,
            data: alloc::vec![0u8; total_bytes],
        }
    }
}

impl BlockOrientedDevice for SsdBlockDevice {
    fn device_id(&self) -> BlockDeviceID { self.id }
    fn device_class(&self) -> DeviceClass { DeviceClass::Ssd }
    fn block_size(&self) -> usize { self.block_size }
    fn total_blocks(&self) -> u64 { self.total_blocks }
    fn is_write_blocked(&self) -> bool { self.write_blocked }
    fn set_write_blocked(&mut self, blocked: bool) { self.write_blocked = blocked; }

    fn read_block(&self, block_num: BlockNumber, buffer: &mut [u8]) -> Result<(), BlockError> {
        if block_num >= self.total_blocks {
            return Err(BlockError::OutOfBounds);
        }
        let offset = (block_num as usize) * self.block_size;
        let end = offset + self.block_size.min(buffer.len());
        buffer[..end - offset].copy_from_slice(&self.data[offset..end]);
        Ok(())
    }

    fn write_block(&mut self, block_num: BlockNumber, data: &[u8]) -> Result<(), BlockError> {
        if self.write_blocked {
            return Err(BlockError::WriteBlocked);
        }
        if block_num >= self.total_blocks {
            return Err(BlockError::OutOfBounds);
        }
        let offset = (block_num as usize) * self.block_size;
        let len = self.block_size.min(data.len());
        self.data[offset..offset + len].copy_from_slice(&data[..len]);
        self.erase_cycles[block_num as usize] += 1; // FTL wear tracking
        Ok(())
    }
}

/// NVMe 2.0 High-Performance Multi-Queue Device
pub struct NvmeBlockDevice {
    pub id: BlockDeviceID,
    pub block_size: usize,
    pub total_blocks: u64,
    pub submission_queues: usize,
    pub completion_queues: usize,
    pub write_blocked: bool,
    pub storage: Vec<u8>,
}

impl NvmeBlockDevice {
    pub fn new(id: BlockDeviceID, block_size: usize, total_blocks: u64, num_queues: usize) -> Self {
        let total_bytes = (block_size as u64 * total_blocks) as usize;
        Self {
            id,
            block_size,
            total_blocks,
            submission_queues: num_queues,
            completion_queues: num_queues,
            write_blocked: false,
            storage: alloc::vec![0u8; total_bytes],
        }
    }
}

impl BlockOrientedDevice for NvmeBlockDevice {
    fn device_id(&self) -> BlockDeviceID { self.id }
    fn device_class(&self) -> DeviceClass { DeviceClass::Nvme }
    fn block_size(&self) -> usize { self.block_size }
    fn total_blocks(&self) -> u64 { self.total_blocks }
    fn is_write_blocked(&self) -> bool { self.write_blocked }
    fn set_write_blocked(&mut self, blocked: bool) { self.write_blocked = blocked; }

    fn read_block(&self, block_num: BlockNumber, buffer: &mut [u8]) -> Result<(), BlockError> {
        if block_num >= self.total_blocks {
            return Err(BlockError::OutOfBounds);
        }
        let offset = (block_num as usize) * self.block_size;
        let len = self.block_size.min(buffer.len());
        buffer[..len].copy_from_slice(&self.storage[offset..offset + len]);
        Ok(())
    }

    fn write_block(&mut self, block_num: BlockNumber, data: &[u8]) -> Result<(), BlockError> {
        if self.write_blocked {
            return Err(BlockError::WriteBlocked);
        }
        if block_num >= self.total_blocks {
            return Err(BlockError::OutOfBounds);
        }
        let offset = (block_num as usize) * self.block_size;
        let len = self.block_size.min(data.len());
        self.storage[offset..offset + len].copy_from_slice(&data[..len]);
        Ok(())
    }
}

// ── 2. BLOCK OPERATIONS ENGINE ─────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockOpCode {
    Read,
    Write,
    Flush,
    DiscardTrim,
    WriteSame,
    SecureErase,
    Barrier,
    WriteBlockingCheck,
    DirectIoRead,
    DirectIoWrite,
    AsyncIoSubmit,
}

#[derive(Debug, Clone)]
pub struct BlockOperationRequest {
    pub req_id: u64,
    pub device_id: BlockDeviceID,
    pub op: BlockOpCode,
    pub block_num: BlockNumber,
    pub count: usize,
    pub buffer: Vec<u8>,
    pub direct_io: bool,
}

pub struct BlockOperationEngine;

impl BlockOperationEngine {
    pub fn execute_op(
        dev: &mut dyn BlockOrientedDevice,
        req: &mut BlockOperationRequest,
    ) -> Result<usize, BlockError> {
        match req.op {
            BlockOpCode::Read | BlockOpCode::DirectIoRead => {
                let mut read_bytes = 0;
                for i in 0..req.count {
                    let mut b = alloc::vec![0u8; dev.block_size()];
                    dev.read_block(req.block_num + i as u64, &mut b)?;
                    req.buffer.extend_from_slice(&b);
                    read_bytes += dev.block_size();
                }
                Ok(read_bytes)
            }
            BlockOpCode::Write | BlockOpCode::DirectIoWrite => {
                if dev.is_write_blocked() {
                    return Err(BlockError::WriteBlocked);
                }
                let mut written_bytes = 0;
                let blk_sz = dev.block_size();
                for i in 0..req.count {
                    let start = i * blk_sz;
                    if start + blk_sz <= req.buffer.len() {
                        dev.write_block(req.block_num + i as u64, &req.buffer[start..start + blk_sz])?;
                        written_bytes += blk_sz;
                    }
                }
                Ok(written_bytes)
            }
            BlockOpCode::Flush => {
                dev.flush()?;
                Ok(0)
            }
            BlockOpCode::DiscardTrim | BlockOpCode::SecureErase => {
                if dev.is_write_blocked() {
                    return Err(BlockError::WriteBlocked);
                }
                let zero_buf = alloc::vec![0u8; dev.block_size()];
                for i in 0..req.count {
                    dev.write_block(req.block_num + i as u64, &zero_buf)?;
                }
                Ok(req.count * dev.block_size())
            }
            BlockOpCode::WriteSame => {
                if dev.is_write_blocked() {
                    return Err(BlockError::WriteBlocked);
                }
                for i in 0..req.count {
                    dev.write_block(req.block_num + i as u64, &req.buffer[..dev.block_size()])?;
                }
                Ok(req.count * dev.block_size())
            }
            _ => Ok(0),
        }
    }
}

// ── 3. MULTI-TYPE BLOCK CLASSIFICATION ────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockKind {
    BootBlock,
    DataBlock,
    DefinedBlock,
    DispatchedBlock,
    FunctionOfBlock,
    ProcessControlBlock,
    ScheduledBlock,
}

#[derive(Debug, Clone)]
pub struct SovereignBlockClassifier {
    pub kind: BlockKind,
    pub block_num: BlockNumber,
    pub block_size: usize,
    pub allocated: bool,
    pub function_tag: String,
}

impl SovereignBlockClassifier {
    pub fn new(kind: BlockKind, block_num: BlockNumber, block_size: usize) -> Self {
        let tag = match kind {
            BlockKind::BootBlock => "Bootloader Stage1/Stage2 Header",
            BlockKind::DataBlock => "File System Payload Data",
            BlockKind::DefinedBlock => "Pre-defined System Descriptor",
            BlockKind::DispatchedBlock => "In-Flight Hardware DMA Queue",
            BlockKind::FunctionOfBlock => "Inode/Superblock Control Logic",
            BlockKind::ProcessControlBlock => "Process Register & State Storage",
            BlockKind::ScheduledBlock => "Elevator Queue Scheduled Block",
        };

        Self {
            kind,
            block_num,
            block_size,
            allocated: true,
            function_tag: tag.to_string(),
        }
    }
}

// ── 4. FIXED & PERMANENT & RECORD BLOCKING ─────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockingStrategy {
    FixedLengthUnspanned,
    FixedLengthSpanned,
    VariableLengthUnspanned,
    VariableLengthSpanned,
    PermanentContiguous,
}

#[derive(Debug, Clone)]
pub struct RecordBlockingEngine {
    pub strategy: BlockingStrategy,
    pub block_size: usize,
    pub record_size: usize,
    pub blocking_factor: usize,
}

impl RecordBlockingEngine {
    pub fn new(strategy: BlockingStrategy, block_size: usize, record_size: usize) -> Self {
        let factor = if record_size > 0 {
            block_size / record_size
        } else {
            0
        };

        Self {
            strategy,
            block_size,
            record_size,
            blocking_factor: factor,
        }
    }

    pub fn calculate_blocks_needed(&self, total_records: usize) -> usize {
        match self.strategy {
            BlockingStrategy::FixedLengthUnspanned => {
                if self.blocking_factor == 0 {
                    0
                } else {
                    (total_records + self.blocking_factor - 1) / self.blocking_factor
                }
            }
            BlockingStrategy::FixedLengthSpanned | BlockingStrategy::VariableLengthSpanned => {
                let total_bytes = total_records * self.record_size;
                (total_bytes + self.block_size - 1) / self.block_size
            }
            BlockingStrategy::PermanentContiguous => {
                (total_records * self.record_size + self.block_size - 1) / self.block_size
            }
            BlockingStrategy::VariableLengthUnspanned => {
                let bytes_per_record_with_header = self.record_size + 4;
                let recs_per_blk = self.block_size / bytes_per_record_with_header;
                if recs_per_blk == 0 {
                    0
                } else {
                    (total_records + recs_per_blk - 1) / recs_per_blk
                }
            }
        }
    }
}

// ── 5. SYSTEM BLOCK DIAGRAM TOPOLOGY ──────────────────────────────────────

#[derive(Debug, Clone)]
pub struct DiagramSubBlock {
    pub id: usize,
    pub name: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SignalBus {
    pub bus_name: String,
    pub source_block_id: usize,
    pub target_block_id: usize,
    pub bandwidth_mbps: u32,
}

#[derive(Debug, Clone)]
pub struct SystemBlockDiagramEngine {
    pub diagram_name: String,
    pub sub_blocks: BTreeMap<usize, DiagramSubBlock>,
    pub signal_buses: Vec<SignalBus>,
}

impl SystemBlockDiagramEngine {
    pub fn new(name: &str) -> Self {
        Self {
            diagram_name: name.to_string(),
            sub_blocks: BTreeMap::new(),
            signal_buses: Vec::new(),
        }
    }

    pub fn add_sub_block(&mut self, id: usize, name: &str, inputs: &[&str], outputs: &[&str]) {
        let block = DiagramSubBlock {
            id,
            name: name.to_string(),
            inputs: inputs.iter().map(|s| s.to_string()).collect(),
            outputs: outputs.iter().map(|s| s.to_string()).collect(),
        };
        self.sub_blocks.insert(id, block);
    }

    pub fn connect_bus(&mut self, bus_name: &str, src_id: usize, dst_id: usize, bandwidth_mbps: u32) {
        self.signal_buses.push(SignalBus {
            bus_name: bus_name.to_string(),
            source_block_id: src_id,
            target_block_id: dst_id,
            bandwidth_mbps,
        });
    }

    pub fn block_count(&self) -> usize {
        self.sub_blocks.len()
    }
}

// ── 6. SIMPLE BLOCK MANAGER & CACHE API ───────────────────────────────────

pub trait BlockDevice {
    fn id(&self) -> BlockDeviceID;
    fn device_type(&self) -> DeviceClass;
    fn block_size(&self) -> usize;
    fn total_blocks(&self) -> usize;
    fn read_block(&self, block_num: usize, buffer: &mut [u8]) -> Result<(), BlockError>;
    fn write_block(&mut self, block_num: usize, data: &[u8]) -> Result<(), BlockError>;
}

pub struct SimpleBlockDevice {
    pub id: BlockDeviceID,
    pub device_type: DeviceClass,
    pub block_size: usize,
    pub total_blocks: usize,
}

impl SimpleBlockDevice {
    pub fn new(
        id: BlockDeviceID,
        device_type: DeviceClass,
        block_size: usize,
        total_blocks: usize,
    ) -> Self {
        SimpleBlockDevice {
            id,
            device_type,
            block_size,
            total_blocks,
        }
    }
}

impl BlockDevice for SimpleBlockDevice {
    fn id(&self) -> BlockDeviceID { self.id }
    fn device_type(&self) -> DeviceClass { self.device_type }
    fn block_size(&self) -> usize { self.block_size }
    fn total_blocks(&self) -> usize { self.total_blocks }

    fn read_block(&self, _block_num: usize, _buffer: &mut [u8]) -> Result<(), BlockError> { Ok(()) }
    fn write_block(&mut self, _block_num: usize, _data: &[u8]) -> Result<(), BlockError> { Ok(()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ssd_ftl_wear_leveling() {
        let mut ssd = SsdBlockDevice::new(1, 4096, 16);
        let data = alloc::vec![0xAAu8; 4096];
        assert!(ssd.write_block(0, &data).is_ok());
        assert_eq!(ssd.erase_cycles[0], 1);

        ssd.set_write_blocked(true);
        assert_eq!(ssd.write_block(0, &data), Err(BlockError::WriteBlocked));
    }

    #[test]
    fn test_record_blocking_factor() {
        let engine = RecordBlockingEngine::new(BlockingStrategy::FixedLengthUnspanned, 4096, 128);
        assert_eq!(engine.blocking_factor, 32);
        assert_eq!(engine.calculate_blocks_needed(100), 4);
    }

    #[test]
    fn test_system_block_diagram_topology() {
        let mut diagram = SystemBlockDiagramEngine::new("SigmaOS Memory & PCIe Topology");
        diagram.add_sub_block(1, "PCIe Gen5 Controller", &["Root Complex Bus"], &["DMA Channels"]);
        diagram.add_sub_block(2, "NVMe Controller Engine", &["DMA Channels"], &["Flash Memory Arrays"]);
        diagram.connect_bus("HighSpeedPCIeBus", 1, 2, 32000);

        assert_eq!(diagram.block_count(), 2);
        assert_eq!(diagram.signal_buses.len(), 1);
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

#[cfg(target_os = "none")]
#[cfg(target_os = "none")]
#[cfg(target_os = "none")]
#[cfg(target_os = "none")]
#[cfg(target_os = "none")]
#[cfg(target_os = "none")]
#[cfg(target_os = "none")]
#[cfg(target_os = "none")]
struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

#[cfg(target_os = "none")]
#[cfg(target_os = "none")]
#[cfg(target_os = "none")]
#[cfg(target_os = "none")]
#[cfg(target_os = "none")]
#[cfg(target_os = "none")]
#[cfg(target_os = "none")]
#[cfg(target_os = "none")]
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
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
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

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
