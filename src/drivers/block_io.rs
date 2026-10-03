//! # Block I/O Layer
//!
//! Linux-inspired block I/O subsystem for disk operations.
//! Provides request queue, I/O scheduler, and bio abstraction.

#![no_std]

extern crate alloc;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

/// Block size (512 bytes - standard sector size)
pub const BLOCK_SIZE: usize = 512;

/// Block I/O operation type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BioOp {
    /// Read operation
    Read = 0,
    /// Write operation
    Write = 1,
    /// Flush operation
    Flush = 2,
    /// Discard operation (TRIM)
    Discard = 3,
    /// Secure erase
    SecureErase = 4,
}

/// Block I/O flags (inspired by Linux bio flags)
#[derive(Debug, Clone, Copy)]
pub struct BioFlags {
    /// High priority I/O
    pub high_priority: bool,
    /// Synchronous I/O
    pub sync: bool,
    /// Force unit access (write-through)
    pub fua: bool,
    /// Read-ahead hint
    pub readahead: bool,
    /// Don't retry on error
    pub no_retry: bool,
}

impl BioFlags {
    pub const DEFAULT: Self = Self {
        high_priority: false,
        sync: false,
        fua: false,
        readahead: false,
        no_retry: false,
    };

    pub const SYNC: Self = Self {
        high_priority: false,
        sync: true,
        fua: true,
        readahead: false,
        no_retry: false,
    };
}

/// Block I/O vector (bio_vec in Linux)
#[derive(Debug, Clone)]
pub struct BioVec {
    /// Physical page address
    pub page: u64,
    /// Length in bytes
    pub len: usize,
    /// Offset within page
    pub offset: usize,
}

impl BioVec {
    pub fn new(page: u64, len: usize, offset: usize) -> Self {
        Self { page, len, offset }
    }
}

/// Block I/O request (bio in Linux)
pub struct BlockIoRequest {
    /// Request ID
    pub id: u64,
    /// Operation type
    pub op: BioOp,
    /// Starting block number
    pub sector: u64,
    /// Number of blocks
    pub num_sectors: u32,
    /// I/O vectors
    pub bio_vecs: Vec<BioVec>,
    /// Flags
    pub flags: BioFlags,
    /// Request state
    pub state: AtomicU32,
    /// Completion callback
    pub completion: Option<fn(u64, Result<(), BlockIoError>)>,
}

/// Block I/O request state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum BioState {
    /// Request pending
    Pending = 0,
    /// Request in progress
    InProgress = 1,
    /// Request completed
    Completed = 2,
    /// Request failed
    Failed = 3,
}

impl BlockIoRequest {
    pub fn new(id: u64, op: BioOp, sector: u64, num_sectors: u32, flags: BioFlags) -> Self {
        Self {
            id,
            op,
            sector,
            num_sectors,
            bio_vecs: Vec::new(),
            flags,
            state: AtomicU32::new(BioState::Pending as u32),
            completion: None,
        }
    }

    /// Add bio vector to request
    pub fn add_bio_vec(&mut self, vec: BioVec) {
        self.bio_vecs.push(vec);
    }

    /// Get request state
    pub fn state(&self) -> BioState {
        match self.state.load(Ordering::Acquire) {
            0 => BioState::Pending,
            1 => BioState::InProgress,
            2 => BioState::Completed,
            3 => BioState::Failed,
            _ => BioState::Pending,
        }
    }

    /// Set request state
    pub fn set_state(&self, state: BioState) {
        self.state.store(state as u32, Ordering::Release);
    }

    /// Complete request
    pub fn complete(&mut self, result: Result<(), BlockIoError>) {
        self.set_state(if result.is_ok() {
            BioState::Completed
        } else {
            BioState::Failed
        });

        if let Some(callback) = self.completion {
            callback(self.id, result);
        }
    }

    /// Get total size in bytes
    pub fn size_bytes(&self) -> usize {
        self.num_sectors as usize * BLOCK_SIZE
    }
}

/// I/O scheduler types (inspired by Linux)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IoScheduler {
    /// No-op scheduler (FIFO)
    Noop = 0,
    /// Deadline scheduler
    Deadline = 1,
    /// Complete Fair Queuing (CFQ)
    Cfq = 2,
    /// Multi-queue Block I/O (BFQ)
    Bfq = 3,
    /// Kyber I/O scheduler
    Kyber = 4,
}

/// Block request queue
pub struct BlockRequestQueue {
    /// Queue name
    pub name: &'static str,
    /// Pending requests
    pending: VecDeque<BlockIoRequest>,
    /// Active requests
    active: Vec<BlockIoRequest>,
    /// I/O scheduler type
    pub scheduler: IoScheduler,
    /// Maximum queue depth
    max_depth: usize,
    /// Next request ID
    next_id: AtomicU64,
    /// Total requests submitted
    total_requests: AtomicU64,
    /// Total requests completed
    total_completed: AtomicU64,
    /// Total bytes read
    bytes_read: AtomicU64,
    /// Total bytes written
    bytes_written: AtomicU64,
}

impl BlockRequestQueue {
    pub fn new(name: &'static str, max_depth: usize, scheduler: IoScheduler) -> Self {
        Self {
            name,
            pending: VecDeque::with_capacity(max_depth),
            active: Vec::with_capacity(max_depth),
            scheduler,
            max_depth,
            next_id: AtomicU64::new(1),
            total_requests: AtomicU64::new(0),
            total_completed: AtomicU64::new(0),
            bytes_read: AtomicU64::new(0),
            bytes_written: AtomicU64::new(0),
        }
    }

    /// Allocate request ID
    fn allocate_id(&self) -> u64 {
        self.next_id.fetch_add(1, Ordering::SeqCst)
    }

    /// Submit I/O request
    pub fn submit(&mut self, mut request: BlockIoRequest) -> Result<u64, BlockIoError> {
        if self.pending.len() >= self.max_depth {
            return Err(BlockIoError::QueueFull);
        }

        request.id = self.allocate_id();
        let id = request.id;

        self.pending.push_back(request);
        self.total_requests.fetch_add(1, Ordering::Relaxed);

        Ok(id)
    }

    /// Get next request to process
    pub fn get_next_request(&mut self) -> Option<BlockIoRequest> {
        // Apply I/O scheduling policy
        match self.scheduler {
            IoScheduler::Noop => self.pending.pop_front(),
            IoScheduler::Deadline => self.schedule_deadline(),
            IoScheduler::Cfq => self.schedule_cfq(),
            _ => self.pending.pop_front(),
        }
    }

    /// Deadline scheduler (prioritize reads, respect deadlines)
    fn schedule_deadline(&mut self) -> Option<BlockIoRequest> {
        // Prioritize read requests
        if let Some(pos) = self.pending.iter().position(|r| r.op == BioOp::Read) {
            return self.pending.remove(pos);
        }
        self.pending.pop_front()
    }

    /// CFQ scheduler (fair queuing)
    fn schedule_cfq(&mut self) -> Option<BlockIoRequest> {
        // Simple round-robin for now
        self.pending.pop_front()
    }

    /// Complete request
    pub fn complete_request(&mut self, id: u64, result: Result<(), BlockIoError>) {
        if let Some(pos) = self.active.iter().position(|r| r.id == id) {
            let mut request = self.active.remove(pos);

            // Update statistics
            self.total_completed.fetch_add(1, Ordering::Relaxed);
            if result.is_ok() {
                let bytes = request.size_bytes() as u64;
                match request.op {
                    BioOp::Read => self.bytes_read.fetch_add(bytes, Ordering::Relaxed),
                    BioOp::Write => self.bytes_written.fetch_add(bytes, Ordering::Relaxed),
                    _ => 0,
                };
            }

            request.complete(result);
        }
    }

    /// Get queue statistics
    pub fn stats(&self) -> BlockQueueStats {
        BlockQueueStats {
            name: self.name,
            pending: self.pending.len(),
            active: self.active.len(),
            total_requests: self.total_requests.load(Ordering::Relaxed),
            total_completed: self.total_completed.load(Ordering::Relaxed),
            bytes_read: self.bytes_read.load(Ordering::Relaxed),
            bytes_written: self.bytes_written.load(Ordering::Relaxed),
        }
    }
}

/// Block queue statistics
#[derive(Debug, Clone, Copy)]
pub struct BlockQueueStats {
    pub name: &'static str,
    pub pending: usize,
    pub active: usize,
    pub total_requests: u64,
    pub total_completed: u64,
    pub bytes_read: u64,
    pub bytes_written: u64,
}

/// Block device interface
pub trait BlockDevice {
    /// Get device name
    fn name(&self) -> &str;

    /// Get device capacity (in blocks)
    fn capacity(&self) -> u64;

    /// Get block size
    fn block_size(&self) -> usize {
        BLOCK_SIZE
    }

    /// Submit I/O request
    fn submit_io(&mut self, request: BlockIoRequest) -> Result<u64, BlockIoError>;

    /// Flush pending writes
    fn flush(&mut self) -> Result<(), BlockIoError>;
}

/// Block I/O errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockIoError {
    /// Queue is full
    QueueFull,
    /// Invalid sector number
    InvalidSector,
    /// I/O error
    IoError,
    /// Device not ready
    NotReady,
    /// Permission denied
    PermissionDenied,
    /// Read-only device
    ReadOnly,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bio_request() {
        let req = BlockIoRequest::new(1, BioOp::Read, 0, 8, BioFlags::DEFAULT);
        assert_eq!(req.op, BioOp::Read);
        assert_eq!(req.sector, 0);
        assert_eq!(req.num_sectors, 32);
        assert_eq!(req.size_bytes(), 4096);
    }

    #[test]
    fn test_bio_vec() {
        let vec = BioVec::new(0x1000, 512, 0);
        assert_eq!(vec.page, 0x1000);
        assert_eq!(vec.len, 512);
    }

    #[test]
    fn test_block_queue() {
        let mut queue = BlockRequestQueue::new("test", 16, IoScheduler::Noop);
        let req = BlockIoRequest::new(0, BioOp::Read, 0, 8, BioFlags::DEFAULT);
        let id = queue.submit(req).unwrap();
        assert!(id > 0);
        assert_eq!(queue.stats().pending, 1);
    }

    #[test]
    fn test_queue_full() {
        let mut queue = BlockRequestQueue::new("test", 2, IoScheduler::Noop);
        queue
            .submit(BlockIoRequest::new(0, BioOp::Read, 0, 8, BioFlags::DEFAULT))
            .unwrap();
        queue
            .submit(BlockIoRequest::new(0, BioOp::Read, 8, 8, BioFlags::DEFAULT))
            .unwrap();
        let result = queue.submit(BlockIoRequest::new(
            0,
            BioOp::Read,
            16,
            8,
            BioFlags::DEFAULT,
        ));
        assert_eq!(result, Err(BlockIoError::QueueFull));
    }

    #[test]
    fn test_scheduler_deadline() {
        let mut queue = BlockRequestQueue::new("test", 16, IoScheduler::Deadline);
        // Add write first
        queue
            .submit(BlockIoRequest::new(
                0,
                BioOp::Write,
                0,
                8,
                BioFlags::DEFAULT,
            ))
            .unwrap();
        // Add read second
        queue
            .submit(BlockIoRequest::new(0, BioOp::Read, 8, 8, BioFlags::DEFAULT))
            .unwrap();

        // Read should be scheduled first
        let next = queue.get_next_request().unwrap();
        assert_eq!(next.op, BioOp::Read);
    }
}
