//! SigmaOS io_uring — Asynchronous I/O Ring
//!
//! Sovereign implementation of Linux io_uring-inspired async I/O.
//! Provides zero-copy, batch submission, and kernel-polled I/O.
//!
//! # Design
//! Two lock-free ring buffers shared between kernel and userspace:
//! - **Submission Queue (SQ)**: userspace → kernel (work to do)
//! - **Completion Queue (CQ)**: kernel → userspace (completed work)
//!
//! Inspired by Linux io_uring (io_uring/io_uring.c, Jens Axboe 2019)

#![allow(dead_code)]
#![allow(clippy::new_without_default)]

extern crate alloc;
use alloc::collections::{BTreeMap, VecDeque};
use alloc::string::String;
use alloc::vec::Vec;

// ============================================================
// Operation Codes
// ============================================================

/// I/O operation type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoUringOp {
    /// No operation (probe)
    Nop,
    /// Read from file descriptor
    Read,
    /// Write to file descriptor
    Write,
    /// Read with scatter/gather (readv)
    Readv,
    /// Write with scatter/gather (writev)
    Writev,
    /// Synchronous file sync (fsync)
    Fsync,
    /// Read fixed buffer (registered buffer)
    ReadFixed,
    /// Write fixed buffer
    WriteFixed,
    /// Poll file descriptor for events
    PollAdd,
    /// Remove poll
    PollRemove,
    /// Sync file range
    SyncFileRange,
    /// Send on socket
    Send,
    /// Receive on socket
    Recv,
    /// Open file
    Openat,
    /// Close file descriptor
    Close,
    /// Wait for timeout
    Timeout,
    /// Cancel pending operation
    AsyncCancel,
    /// fallocate
    Fallocate,
    /// fstat
    Statx,
    /// madvise
    Madvise,
    /// splice
    Splice,
    /// tee
    Tee,
}

// ============================================================
// Submission Queue Entry (SQE)
// ============================================================

/// A single submission queue entry.
///
/// Describes one I/O operation for the kernel to execute.
/// Analogous to `struct io_uring_sqe` in Linux.
#[derive(Debug, Clone)]
pub struct IoUringSqe {
    /// Operation type
    pub opcode: IoUringOp,
    /// Operation flags
    pub flags: u8,
    /// File descriptor to operate on
    pub fd: i32,
    /// File offset (-1 for current position)
    pub off: i64,
    /// Buffer address (userspace pointer, stored as u64)
    pub addr: u64,
    /// Length of operation in bytes
    pub len: u32,
    /// User-supplied tag for matching with CQE
    pub user_data: u64,
    /// IO priority
    pub ioprio: u16,
    /// Personality (for privilege escalation)
    pub personality: u16,
    /// Buffer index (for fixed buffers)
    pub buf_index: u16,
}

impl IoUringSqe {
    /// Create a read SQE.
    pub fn read(fd: i32, buf_addr: u64, len: u32, off: i64, user_data: u64) -> Self {
        Self { opcode: IoUringOp::Read, flags: 0, fd, off, addr: buf_addr, len,
            user_data, ioprio: 0, personality: 0, buf_index: 0 }
    }

    /// Create a write SQE.
    pub fn write(fd: i32, buf_addr: u64, len: u32, off: i64, user_data: u64) -> Self {
        Self { opcode: IoUringOp::Write, flags: 0, fd, off, addr: buf_addr, len,
            user_data, ioprio: 0, personality: 0, buf_index: 0 }
    }

    /// Create a nop SQE (used for testing).
    pub fn nop(user_data: u64) -> Self {
        Self { opcode: IoUringOp::Nop, flags: 0, fd: -1, off: 0, addr: 0, len: 0,
            user_data, ioprio: 0, personality: 0, buf_index: 0 }
    }

    /// Create a timeout SQE.
    pub fn timeout(timeout_ns: u64, user_data: u64) -> Self {
        Self { opcode: IoUringOp::Timeout, flags: 0, fd: -1, off: 0,
            addr: timeout_ns, len: 0, user_data, ioprio: 0, personality: 0, buf_index: 0 }
    }
}

// ============================================================
// Completion Queue Entry (CQE)
// ============================================================

/// A single completion queue entry.
///
/// Returned by kernel when an SQE has been processed.
/// Analogous to `struct io_uring_cqe` in Linux.
#[derive(Debug, Clone)]
pub struct IoUringCqe {
    /// Matches the `user_data` of the corresponding SQE
    pub user_data: u64,
    /// Result: bytes transferred (>= 0) or error code (< 0)
    pub res: i32,
    /// Flags
    pub flags: u32,
}

impl IoUringCqe {
    /// Create a successful CQE.
    pub fn ok(user_data: u64, bytes: i32) -> Self {
        Self { user_data, res: bytes, flags: 0 }
    }

    /// Create an error CQE.
    pub fn err(user_data: u64, errno: i32) -> Self {
        Self { user_data, res: -errno.abs(), flags: 0 }
    }

    /// Returns true if operation succeeded.
    pub fn is_ok(&self) -> bool { self.res >= 0 }
    /// Returns the error code if failed.
    pub fn error(&self) -> Option<i32> { if self.res < 0 { Some(-self.res) } else { None } }
}

// ============================================================
// Ring Buffer
// ============================================================

/// Fixed-size ring buffer for SQE/CQE.
struct RingBuffer<T> {
    entries: Vec<Option<T>>,
    head: usize,
    tail: usize,
    capacity: usize,
}

impl<T: Clone> RingBuffer<T> {
    fn new(capacity: usize) -> Self {
        // Capacity must be power of 2
        let cap = capacity.next_power_of_two();
        Self { entries: (0..cap).map(|_| None).collect(), head: 0, tail: 0, capacity: cap }
    }

    fn push(&mut self, item: T) -> bool {
        let next_tail = (self.tail + 1) & (self.capacity - 1);
        if next_tail == self.head { return false; } // full
        self.entries[self.tail] = Some(item);
        self.tail = next_tail;
        true
    }

    fn pop(&mut self) -> Option<T> {
        if self.head == self.tail { return None; } // empty
        let item = self.entries[self.head].take();
        self.head = (self.head + 1) & (self.capacity - 1);
        item
    }

    fn len(&self) -> usize {
        if self.tail >= self.head { self.tail - self.head }
        else { self.capacity - self.head + self.tail }
    }

    fn is_empty(&self) -> bool { self.head == self.tail }
    fn is_full(&self) -> bool { ((self.tail + 1) & (self.capacity - 1)) == self.head }
}

// ============================================================
// IoUring Instance
// ============================================================

/// Statistics for an io_uring instance.
#[derive(Debug, Default, Clone)]
pub struct IoUringStats {
    pub submitted: u64,
    pub completed: u64,
    pub errors: u64,
    pub bytes_read: u64,
    pub bytes_written: u64,
}

/// A single io_uring instance (analogous to an io_uring file descriptor).
pub struct IoUring {
    /// Instance identifier
    id: u32,
    /// Submission ring
    sq: RingBuffer<IoUringSqe>,
    /// Completion ring
    cq: RingBuffer<IoUringCqe>,
    /// Queue depth (power of 2)
    depth: u32,
    /// Flags
    flags: u32,
    /// Stats
    stats: IoUringStats,
    /// Simulated file data store (fd → Vec<u8>)
    /// In a real system this would delegate to the VFS
    file_store: BTreeMap<i32, Vec<u8>>,
}

/// io_uring setup flags.
pub const IORING_SETUP_SQPOLL:    u32 = 1 << 1; // Kernel polling thread
pub const IORING_SETUP_IOPOLL:    u32 = 1 << 0; // I/O polling (no IRQ)
pub const IORING_SETUP_SINGLE_ISSUER: u32 = 1 << 12; // Single submitter

impl IoUring {
    /// Create a new io_uring instance with the given queue depth.
    ///
    /// # Arguments
    /// * `id` — Instance identifier
    /// * `depth` — Queue depth (rounded up to next power of 2, max 32768)
    /// * `flags` — Setup flags (IORING_SETUP_*)
    pub fn new(id: u32, depth: u32, flags: u32) -> Self {
        let depth = depth.next_power_of_two().min(32768);
        Self {
            id, depth, flags,
            sq: RingBuffer::new(depth as usize),
            cq: RingBuffer::new(depth as usize * 2),
            stats: IoUringStats::default(),
            file_store: BTreeMap::new(),
        }
    }

    /// Register a simulated file for testing.
    pub fn register_file(&mut self, fd: i32, data: Vec<u8>) {
        self.file_store.insert(fd, data);
    }

    /// Submit a batch of SQEs for processing.
    ///
    /// # Returns
    /// Number of SQEs successfully submitted.
    pub fn submit_batch(&mut self, sqes: &[IoUringSqe]) -> usize {
        let mut count = 0;
        for sqe in sqes {
            if self.sq.push(sqe.clone()) { count += 1; }
        }
        self.stats.submitted += count as u64;
        count
    }

    /// Submit a single SQE.
    pub fn submit_one(&mut self, sqe: IoUringSqe) -> bool {
        let ok = self.sq.push(sqe);
        if ok { self.stats.submitted += 1; }
        ok
    }

    /// Process all pending SQEs and place CQEs into the completion ring.
    ///
    /// In a real system this runs in kernel context (potentially in SQPOLL thread).
    /// Here we simulate the I/O synchronously.
    ///
    /// # Returns
    /// Number of operations completed.
    pub fn process(&mut self) -> usize {
        let mut count = 0;
        let mut pending: Vec<IoUringSqe> = Vec::new();
        while let Some(sqe) = self.sq.pop() { pending.push(sqe); }

        for sqe in pending {
            let cqe = self.execute_sqe(&sqe);
            if let Some(ref c) = cqe {
                if c.res < 0 { self.stats.errors += 1; }
            }
            if let Some(cqe) = cqe {
                self.cq.push(cqe);
                count += 1;
            }
        }
        self.stats.completed += count as u64;
        count
    }

    fn execute_sqe(&mut self, sqe: &IoUringSqe) -> Option<IoUringCqe> {
        match sqe.opcode {
            IoUringOp::Nop => Some(IoUringCqe::ok(sqe.user_data, 0)),

            IoUringOp::Read | IoUringOp::Readv | IoUringOp::ReadFixed => {
                if let Some(data) = self.file_store.get(&sqe.fd) {
                    let off = sqe.off.max(0) as usize;
                    let end = (off + sqe.len as usize).min(data.len());
                    let bytes = if end > off { (end - off) as i32 } else { 0 };
                    self.stats.bytes_read += bytes as u64;
                    Some(IoUringCqe::ok(sqe.user_data, bytes))
                } else {
                    Some(IoUringCqe::err(sqe.user_data, 9)) // EBADF
                }
            }

            IoUringOp::Write | IoUringOp::Writev | IoUringOp::WriteFixed => {
                if self.file_store.contains_key(&sqe.fd) {
                    self.stats.bytes_written += sqe.len as u64;
                    Some(IoUringCqe::ok(sqe.user_data, sqe.len as i32))
                } else {
                    Some(IoUringCqe::err(sqe.user_data, 9)) // EBADF
                }
            }

            IoUringOp::Close => {
                self.file_store.remove(&sqe.fd);
                Some(IoUringCqe::ok(sqe.user_data, 0))
            }

            IoUringOp::Fsync => Some(IoUringCqe::ok(sqe.user_data, 0)),
            IoUringOp::Timeout => Some(IoUringCqe::ok(sqe.user_data, 0)),

            IoUringOp::PollAdd | IoUringOp::PollRemove => {
                Some(IoUringCqe::ok(sqe.user_data, 0))
            }

            _ => Some(IoUringCqe::err(sqe.user_data, 38)), // ENOSYS
        }
    }

    /// Collect all available CQEs.
    pub fn collect_completions(&mut self) -> Vec<IoUringCqe> {
        let mut cqes = Vec::new();
        while let Some(cqe) = self.cq.pop() { cqes.push(cqe); }
        cqes
    }

    /// Submit and immediately process (synchronous path).
    pub fn submit_and_wait(&mut self, sqes: &[IoUringSqe]) -> Vec<IoUringCqe> {
        self.submit_batch(sqes);
        self.process();
        self.collect_completions()
    }

    pub fn id(&self) -> u32 { self.id }
    pub fn depth(&self) -> u32 { self.depth }
    pub fn sq_ready(&self) -> usize { self.sq.len() }
    pub fn cq_ready(&self) -> usize { self.cq.len() }
    pub fn stats(&self) -> &IoUringStats { &self.stats }
}

// ============================================================
// IoUringManager — System-Wide Registry
// ============================================================

/// System-wide io_uring instance manager.
pub struct IoUringManager {
    instances: BTreeMap<u32, IoUring>,
    next_id: u32,
}

impl IoUringManager {
    pub fn new() -> Self { Self { instances: BTreeMap::new(), next_id: 1 } }

    /// Create a new io_uring instance. Returns the instance ID.
    pub fn setup(&mut self, depth: u32, flags: u32) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.instances.insert(id, IoUring::new(id, depth, flags));
        id
    }

    /// Destroy an io_uring instance.
    pub fn destroy(&mut self, id: u32) { self.instances.remove(&id); }

    /// Get mutable reference to an instance.
    pub fn get_mut(&mut self, id: u32) -> Option<&mut IoUring> { self.instances.get_mut(&id) }
    pub fn get(&self, id: u32) -> Option<&IoUring> { self.instances.get(&id) }
    pub fn count(&self) -> usize { self.instances.len() }
}

impl Default for IoUringManager { fn default() -> Self { Self::new() } }

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nop_completion() {
        let mut ring = IoUring::new(1, 64, 0);
        let cqes = ring.submit_and_wait(&[IoUringSqe::nop(42)]);
        assert_eq!(cqes.len(), 1);
        assert_eq!(cqes[0].user_data, 42);
        assert!(cqes[0].is_ok());
    }

    #[test]
    fn test_read_operation() {
        let mut ring = IoUring::new(1, 64, 0);
        ring.register_file(3, b"Hello SigmaOS".to_vec());
        let sqe = IoUringSqe::read(3, 0, 13, 0, 100);
        let cqes = ring.submit_and_wait(&[sqe]);
        assert_eq!(cqes[0].res, 13);
        assert_eq!(ring.stats().bytes_read, 13);
    }

    #[test]
    fn test_bad_fd_returns_ebadf() {
        let mut ring = IoUring::new(1, 64, 0);
        let sqe = IoUringSqe::read(99, 0, 100, 0, 200);
        let cqes = ring.submit_and_wait(&[sqe]);
        assert!(!cqes[0].is_ok());
        assert_eq!(cqes[0].error(), Some(9)); // EBADF
    }

    #[test]
    fn test_batch_submission() {
        let mut ring = IoUring::new(1, 128, 0);
        ring.register_file(1, vec![0u8; 1024]);
        ring.register_file(2, vec![0u8; 1024]);
        let sqes = vec![
            IoUringSqe::write(1, 0, 512, 0, 1),
            IoUringSqe::write(2, 0, 256, 0, 2),
            IoUringSqe::nop(3),
        ];
        let cqes = ring.submit_and_wait(&sqes);
        assert_eq!(cqes.len(), 3);
        assert_eq!(ring.stats().bytes_written, 768);
        assert_eq!(ring.stats().completed, 3);
    }

    #[test]
    fn test_ring_wrap() {
        let mut ring = IoUring::new(1, 8, 0); // depth=8
        // Submit 8 nops (fill the ring)
        for i in 0..8u64 {
            ring.submit_one(IoUringSqe::nop(i));
        }
        ring.process();
        let cqes = ring.collect_completions();
        assert_eq!(cqes.len(), 8);
    }

    #[test]
    fn test_manager() {
        let mut mgr = IoUringManager::new();
        let id1 = mgr.setup(256, 0);
        let id2 = mgr.setup(64, IORING_SETUP_SQPOLL);
        assert_eq!(mgr.count(), 2);
        mgr.destroy(id1);
        assert_eq!(mgr.count(), 1);
        assert!(mgr.get(id2).is_some());
    }
}
