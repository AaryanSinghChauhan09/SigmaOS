//! io_uring — Async I/O Interface
//! Inspired by Linux io_uring (Jens Axboe, kernel 5.1).
//! Uses shared ring buffers between userspace and kernel to eliminate
//! per-syscall overhead. Achieves near-zero-copy async I/O.
//!
//! References:
//! - Linux io_uring: https://kernel.dk/io_uring.pdf
//! - liburing: https://github.com/axboe/liburing
//! - FreeBSD: kqueue/kevent as inspiration for event notification

use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

/// Maximum entries in a single ring (must be power of two, max 32768).
pub const IORING_MAX_ENTRIES: u32 = 4096;

/// Submission Queue Entry opcodes (matches Linux io_uring opcodes)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoUringOp {
    Nop       = 0,
    Readv     = 1,
    Writev    = 2,
    FSync     = 3,
    ReadFixed = 4,
    WriteFixed = 5,
    PollAdd   = 6,
    PollRemove = 7,
    SyncFileRange = 8,
    SendMsg   = 9,
    RecvMsg   = 10,
    Timeout   = 11,
    TimeoutRemove = 12,
    Accept    = 13,
    AsyncCancel = 14,
    LinkTimeout = 15,
    Connect   = 16,
    Fallocate = 17,
    OpenAt    = 18,
    Close     = 19,
    Read      = 22,
    Write     = 23,
    Statx     = 24,
    Splice    = 25,
    ProvideBuffers = 31,
    RemoveBuffers = 32,
}

/// Submission Queue Entry (SQE) — 64 bytes, matches Linux struct io_uring_sqe
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy, Default)]
pub struct IoUringSqe {
    pub opcode: u8,
    pub flags: u8,
    pub ioprio: u16,
    pub fd: i32,
    pub off_or_addr2: u64,  // file offset or address
    pub addr_or_splice_off_in: u64, // buffer pointer
    pub len: u32,           // buffer length
    pub op_flags: u32,      // opcode-specific flags
    pub user_data: u64,     // caller-defined identifier
    pub buf_index_or_group: u16,
    pub personality: u16,
    pub splice_fd_in_or_file_index: i32,
    pub addr3_or_optval: u64,
    pub _pad: u64,
}

/// Completion Queue Entry (CQE) — 16 bytes, matches Linux struct io_uring_cqe
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, Default)]
pub struct IoUringCqe {
    pub user_data: u64,   // matches SQE user_data
    pub res: i32,         // result: bytes transferred or negative errno
    pub flags: u32,       // CQE flags
}

/// Shared ring state (kernel-side)
pub struct IoUringRing {
    /// Submission queue entries
    sqes: Vec<IoUringSqe>,
    /// Completion queue entries
    cqes: Vec<IoUringCqe>,
    /// SQ head (read by kernel, advanced by kernel after processing)
    sq_head: AtomicU32,
    /// SQ tail (written by userspace to submit new entries)
    sq_tail: AtomicU32,
    /// CQ head (read/advanced by userspace after consuming completions)
    cq_head: AtomicU32,
    /// CQ tail (written by kernel after completing I/O)
    cq_tail: AtomicU32,
    /// Total completions processed
    total_completions: AtomicU64,
    /// Ring size (number of entries)
    ring_size: u32,
}

impl IoUringRing {
    /// Create a new io_uring ring with `entries` slots.
    pub fn new(entries: u32) -> Self {
        let entries = entries.next_power_of_two().min(IORING_MAX_ENTRIES);
        Self {
            sqes: vec![IoUringSqe::default(); entries as usize],
            cqes: vec![IoUringCqe::default(); (entries * 2) as usize], // CQ is 2x SQ
            sq_head: AtomicU32::new(0),
            sq_tail: AtomicU32::new(0),
            cq_head: AtomicU32::new(0),
            cq_tail: AtomicU32::new(0),
            total_completions: AtomicU64::new(0),
            ring_size: entries,
        }
    }

    /// Submit an SQE to the ring. Returns the index of the submission.
    pub fn submit(&mut self, sqe: IoUringSqe) -> Option<u32> {
        let tail = self.sq_tail.load(Ordering::Acquire);
        let head = self.sq_head.load(Ordering::Acquire);
        if tail.wrapping_sub(head) >= self.ring_size {
            return None; // ring full
        }
        let idx = (tail & (self.ring_size - 1)) as usize;
        self.sqes[idx] = sqe;
        self.sq_tail.fetch_add(1, Ordering::Release);
        Some(idx as u32)
    }

    /// Process pending submissions (kernel-side). Returns number processed.
    pub fn process_submissions(&mut self) -> u32 {
        let head = self.sq_head.load(Ordering::Acquire);
        let tail = self.sq_tail.load(Ordering::Acquire);
        let mut count = 0u32;
        let mut h = head;
        while h != tail {
            let sqe_idx = (h & (self.ring_size - 1)) as usize;
            let sqe = self.sqes[sqe_idx];
            // Simulate completion: result = 0 (success) for NOP, otherwise bytes from len
            let res = match sqe.opcode {
                0 => 0, // NOP
                _ => sqe.len as i32,
            };
            self.post_completion(sqe.user_data, res, 0);
            h = h.wrapping_add(1);
            count += 1;
        }
        self.sq_head.store(tail, Ordering::Release);
        count
    }

    /// Post a completion to the CQ.
    fn post_completion(&mut self, user_data: u64, res: i32, flags: u32) {
        let tail = self.cq_tail.load(Ordering::Acquire);
        let idx = (tail & (self.ring_size * 2 - 1)) as usize;
        self.cqes[idx] = IoUringCqe { user_data, res, flags };
        self.cq_tail.fetch_add(1, Ordering::Release);
        self.total_completions.fetch_add(1, Ordering::Relaxed);
    }

    /// Consume a completion from the CQ. Returns None if empty.
    pub fn consume_completion(&mut self) -> Option<IoUringCqe> {
        let head = self.cq_head.load(Ordering::Acquire);
        let tail = self.cq_tail.load(Ordering::Acquire);
        if head == tail { return None; }
        let idx = (head & (self.ring_size * 2 - 1)) as usize;
        let cqe = self.cqes[idx];
        self.cq_head.fetch_add(1, Ordering::Release);
        Some(cqe)
    }

    /// Total completions posted since ring creation.
    pub fn total_completions(&self) -> u64 {
        self.total_completions.load(Ordering::Relaxed)
    }

    /// Number of pending submissions awaiting processing.
    pub fn pending_submissions(&self) -> u32 {
        self.sq_tail.load(Ordering::Relaxed).wrapping_sub(self.sq_head.load(Ordering::Relaxed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nop_roundtrip() {
        let mut ring = IoUringRing::new(64);
        let sqe = IoUringSqe { opcode: IoUringOp::Nop as u8, user_data: 42, ..Default::default() };
        assert!(ring.submit(sqe).is_some());
        assert_eq!(ring.pending_submissions(), 1);
        let processed = ring.process_submissions();
        assert_eq!(processed, 1);
        let cqe = ring.consume_completion().expect("should have completion");
        assert_eq!(cqe.user_data, 42);
        assert_eq!(cqe.res, 0);
    }

    #[test]
    fn test_ring_full() {
        let mut ring = IoUringRing::new(4);
        for i in 0..4 {
            let sqe = IoUringSqe { opcode: IoUringOp::Nop as u8, user_data: i, ..Default::default() };
            assert!(ring.submit(sqe).is_some());
        }
        let sqe = IoUringSqe { opcode: IoUringOp::Nop as u8, user_data: 99, ..Default::default() };
        assert!(ring.submit(sqe).is_none(), "ring should be full");
    }

    #[test]
    fn test_write_op() {
        let mut ring = IoUringRing::new(64);
        let sqe = IoUringSqe {
            opcode: IoUringOp::Write as u8,
            fd: 3,
            len: 1024,
            user_data: 100,
            ..Default::default()
        };
        ring.submit(sqe);
        ring.process_submissions();
        let cqe = ring.consume_completion().unwrap();
        assert_eq!(cqe.user_data, 100);
        assert_eq!(cqe.res, 1024); // simulated bytes written
    }
}
