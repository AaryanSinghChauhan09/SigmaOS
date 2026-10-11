use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::boxed::Box;
use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

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
#[repr(u8)]
pub enum BlockError {
    Success = 0,
    NotFound = 1,
    ReadFailed = 2,
    WriteFailed = 3,
    WriteBlocked = 4,
    OutOfBounds = 5,
    InvalidBlockSize = 6,
    SequentialOnly = 7,
    RequestExhaustion = 8,
    LostCompletion = 9,
    DoubleCompletion = 10,
    QueueFull = 11,
    DeviceBusy = 12,
    InvalidRequest = 13,
    DeviceDetached = 14,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestState {
    Unallocated,
    Pending,
    Dispatched,
    Completed,
    Failed,
    Freed,
}

// ── 0. SCSI SENSE KEYS & ADDITIONAL SENSE CODE QUALIFIERS (ASC/ASCQ) ─────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ScsiSenseKey {
    NoSense = 0x00,
    RecoveredError = 0x01,
    NotReady = 0x02,
    MediumError = 0x03,
    HardwareError = 0x04,
    IllegalRequest = 0x05,
    UnitAttention = 0x06,
    DataProtect = 0x07,
    BlankCheck = 0x08,
    VendorSpecific = 0x09,
    CopyAborted = 0x0A,
    AbortedCommand = 0x0B,
    VolumeOverflow = 0x0D,
    Miscompare = 0x0E,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScsiAscQualifier {
    NoAdditionalSense = 0x0000,
    LogicalUnitNotReady = 0x0400,
    LogicalUnitNotReadyInitializing = 0x0401,
    LogicalUnitNotReadyManualIntervention = 0x0402,
    LogicalUnitNotReadyFormatInProgress = 0x0404,
    LbaOutOfRange = 0x2100,
    InvalidCommandOperationCode = 0x2000,
    WriteProtected = 0x2700,
    UnrecoveredReadError = 0x1100,
    WriteError = 0x0C00,
    MediumNotPresent = 0x3A00,
    PowerOnReset = 0x2900,
}

impl ScsiAscQualifier {
    pub fn asc(&self) -> u8 {
        ((*self as u16) >> 8) as u8
    }

    pub fn ascq(&self) -> u8 {
        ((*self as u16) & 0xFF) as u8
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScsiSenseData {
    pub sense_key: ScsiSenseKey,
    pub asc: u8,
    pub ascq: u8,
    pub qualifier: ScsiAscQualifier,
    pub valid_info: bool,
    pub information: u64,
}

impl ScsiSenseData {
    pub fn new(sense_key: ScsiSenseKey, qualifier: ScsiAscQualifier) -> Self {
        Self {
            sense_key,
            asc: qualifier.asc(),
            ascq: qualifier.ascq(),
            qualifier,
            valid_info: false,
            information: 0,
        }
    }

    pub fn with_info(mut self, info: u64) -> Self {
        self.valid_info = true;
        self.information = info;
        self
    }

    pub fn description(&self) -> String {
        format!(
            "SCSI SenseKey: {:?}, ASC/ASCQ: {:#04X}/{:#04X} ({:?})",
            self.sense_key, self.asc, self.ascq, self.qualifier
        )
    }
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
            erase_cycles: std::vec![0u32; total_blocks as usize],
            write_blocked: false,
            data: std::vec![0u8; total_bytes],
        }
    }
}

impl BlockOrientedDevice for SsdBlockDevice {
    fn device_id(&self) -> BlockDeviceID {
        self.id
    }
    fn device_class(&self) -> DeviceClass {
        DeviceClass::Ssd
    }
    fn block_size(&self) -> usize {
        self.block_size
    }
    fn total_blocks(&self) -> u64 {
        self.total_blocks
    }
    fn is_write_blocked(&self) -> bool {
        self.write_blocked
    }
    fn set_write_blocked(&mut self, blocked: bool) {
        self.write_blocked = blocked;
    }

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
            storage: std::vec![0u8; total_bytes],
        }
    }
}

impl BlockOrientedDevice for NvmeBlockDevice {
    fn device_id(&self) -> BlockDeviceID {
        self.id
    }
    fn device_class(&self) -> DeviceClass {
        DeviceClass::Nvme
    }
    fn block_size(&self) -> usize {
        self.block_size
    }
    fn total_blocks(&self) -> u64 {
        self.total_blocks
    }
    fn is_write_blocked(&self) -> bool {
        self.write_blocked
    }
    fn set_write_blocked(&mut self, blocked: bool) {
        self.write_blocked = blocked;
    }

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
                    let mut b = std::vec![0u8; dev.block_size()];
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
                        dev.write_block(
                            req.block_num + i as u64,
                            &req.buffer[start..start + blk_sz],
                        )?;
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
                let zero_buf = std::vec![0u8; dev.block_size()];
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

// ── 2B. BOUNDED BLOCK DEVICE MANAGER & EXACT-ONCE COMPLETION ENGINE ──────

#[derive(Debug, Clone)]
pub struct BoundedBlockRequest {
    pub req_id: u64,
    pub device_id: BlockDeviceID,
    pub op: BlockOpCode,
    pub block_num: BlockNumber,
    pub count: usize,
    pub buffer: Vec<u8>,
    pub state: RequestState,
    pub submission_time_ms: u64,
}

pub struct BoundedBlockDeviceManager {
    pub max_requests: usize,
    pub active_requests: BTreeMap<u64, BoundedBlockRequest>,
    pub pending_queue: Vec<u64>,
    pub next_req_id: AtomicU64,
    pub devices: BTreeMap<BlockDeviceID, Box<dyn BlockOrientedDevice>>,
}

impl BoundedBlockDeviceManager {
    pub fn new(max_requests: usize) -> Self {
        Self {
            max_requests,
            active_requests: BTreeMap::new(),
            pending_queue: Vec::new(),
            next_req_id: AtomicU64::new(100),
            devices: BTreeMap::new(),
        }
    }

    pub fn register_device(&mut self, device: Box<dyn BlockOrientedDevice>) {
        self.devices.insert(device.device_id(), device);
    }

    pub fn unregister_device(&mut self, device_id: BlockDeviceID) -> Result<(), BlockError> {
        if self.devices.remove(&device_id).is_some() {
            Ok(())
        } else {
            Err(BlockError::NotFound)
        }
    }

    pub fn submit_request(
        &mut self,
        device_id: BlockDeviceID,
        op: BlockOpCode,
        block_num: BlockNumber,
        count: usize,
        buffer: Vec<u8>,
        timestamp_ms: u64,
    ) -> Result<u64, BlockError> {
        if !self.devices.contains_key(&device_id) {
            return Err(BlockError::NotFound);
        }
        if self.active_requests.len() >= self.max_requests {
            return Err(BlockError::RequestExhaustion);
        }

        let req_id = self.next_req_id.fetch_add(1, Ordering::SeqCst);
        let req = BoundedBlockRequest {
            req_id,
            device_id,
            op,
            block_num,
            count,
            buffer,
            state: RequestState::Pending,
            submission_time_ms: timestamp_ms,
        };

        self.active_requests.insert(req_id, req);
        self.pending_queue.push(req_id);
        Ok(req_id)
    }

    pub fn dispatch_next(&mut self) -> Result<Option<u64>, BlockError> {
        if self.pending_queue.is_empty() {
            return Ok(None);
        }

        let req_id = self.pending_queue.remove(0);
        if let Some(req) = self.active_requests.get_mut(&req_id) {
            req.state = RequestState::Dispatched;
            if let Some(dev) = self.devices.get_mut(&req.device_id) {
                let mut op_req = BlockOperationRequest {
                    req_id,
                    device_id: req.device_id,
                    op: req.op,
                    block_num: req.block_num,
                    count: req.count,
                    buffer: req.buffer.clone(),
                    direct_io: false,
                };
                match BlockOperationEngine::execute_op(dev.as_mut(), &mut op_req) {
                    Ok(_) => {
                        req.buffer = op_req.buffer;
                        Ok(Some(req_id))
                    }
                    Err(e) => {
                        req.state = RequestState::Failed;
                        Err(e)
                    }
                }
            } else {
                req.state = RequestState::Failed;
                Err(BlockError::DeviceDetached)
            }
        } else {
            Err(BlockError::NotFound)
        }
    }

    pub fn complete_request(&mut self, req_id: u64, success: bool) -> Result<(), BlockError> {
        let req = match self.active_requests.get_mut(&req_id) {
            Some(r) => r,
            None => return Err(BlockError::LostCompletion),
        };

        match req.state {
            RequestState::Completed | RequestState::Failed => Err(BlockError::DoubleCompletion),
            RequestState::Dispatched | RequestState::Pending => {
                req.state = if success {
                    RequestState::Completed
                } else {
                    RequestState::Failed
                };
                Ok(())
            }
            RequestState::Unallocated | RequestState::Freed => Err(BlockError::InvalidRequest),
        }
    }

    pub fn reap_completed(&mut self, req_id: u64) -> Result<BoundedBlockRequest, BlockError> {
        if let Some(req) = self.active_requests.get(&req_id) {
            if req.state != RequestState::Completed && req.state != RequestState::Failed {
                return Err(BlockError::DeviceBusy);
            }
        } else {
            return Err(BlockError::NotFound);
        }

        Ok(self.active_requests.remove(&req_id).unwrap())
    }

    pub fn detect_lost_completions(&mut self, current_time_ms: u64, timeout_ms: u64) -> Vec<u64> {
        let mut timed_out = Vec::new();
        for (req_id, req) in &mut self.active_requests {
            if req.state == RequestState::Dispatched
                && current_time_ms >= req.submission_time_ms + timeout_ms
            {
                req.state = RequestState::Failed;
                timed_out.push(*req_id);
            }
        }
        timed_out
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

    pub fn connect_bus(
        &mut self,
        bus_name: &str,
        src_id: usize,
        dst_id: usize,
        bandwidth_mbps: u32,
    ) {
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
    fn id(&self) -> BlockDeviceID {
        self.id
    }
    fn device_type(&self) -> DeviceClass {
        self.device_type
    }
    fn block_size(&self) -> usize {
        self.block_size
    }
    fn total_blocks(&self) -> usize {
        self.total_blocks
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ssd_ftl_wear_leveling() {
        let mut ssd = SsdBlockDevice::new(1, 4096, 16);
        let data = std::vec![0xAAu8; 4096];
        assert!(ssd.write_block(0, &data).is_ok());
        assert_eq!(ssd.erase_cycles[0], 1);

        ssd.set_write_blocked(true);
        assert_eq!(ssd.write_block(0, &data), Err(BlockError::WriteBlocked));
    }

    #[test]
    fn test_bounded_block_manager_exhaustion_and_exact_once_completion() {
        let mut mgr = BoundedBlockDeviceManager::new(2);
        let ssd = SsdBlockDevice::new(1, 512, 100);
        mgr.register_device(Box::new(ssd));

        let req1 = mgr
            .submit_request(1, BlockOpCode::Read, 0, 1, Vec::new(), 1000)
            .unwrap();
        let req2 = mgr
            .submit_request(1, BlockOpCode::Write, 1, 1, std::vec![0xFF; 512], 1005)
            .unwrap();

        // Exhaustion check
        let err = mgr.submit_request(1, BlockOpCode::Read, 2, 1, Vec::new(), 1010);
        assert_eq!(err, Err(BlockError::RequestExhaustion));

        // Dispatch req1
        let dispatched = mgr.dispatch_next().unwrap();
        assert_eq!(dispatched, Some(req1));

        // Complete req1 once -> Success
        assert_eq!(mgr.complete_request(req1, true), Ok(()));

        // Complete req1 second time -> DoubleCompletion Error
        assert_eq!(
            mgr.complete_request(req1, true),
            Err(BlockError::DoubleCompletion)
        );

        // Lost completion check for non-existent req
        assert_eq!(
            mgr.complete_request(999, true),
            Err(BlockError::LostCompletion)
        );

        // Reap completed req1
        let reaped = mgr.reap_completed(req1).unwrap();
        assert_eq!(reaped.state, RequestState::Completed);

        // Dispatch req2 and test lost completion timeout
        let dispatched2 = mgr.dispatch_next().unwrap().unwrap();
        assert_eq!(dispatched2, req2);
        let timed_out = mgr.detect_lost_completions(2000, 500);
        assert_eq!(timed_out, std::vec![req2]);
    }
}
