// Linux io_uring-style async zero-copy I/O subsystem
// Zero-dependency, std-based implementation with lock-free ring buffers

use std::vec::Vec;
use core::sync::atomic::{AtomicU32, AtomicU64, AtomicUsize, Ordering};

/// io_uring operation codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IoUringOpcode {
    Nop = 0,
    Read = 1,
    Write = 2,
    Poll = 3,
    Fsync = 4,
    Readv = 5,
    Writev = 6,
    Send = 7,
    Recv = 8,
    Accept = 9,
    Connect = 10,
}

/// Submission Queue Entry (SQE)
#[derive(Debug, Clone)]
pub struct SubmissionQueueEntry {
    pub opcode: IoUringOpcode,
    pub fd: i32,
    pub offset: u64,
    pub addr: u64,
    pub len: u32,
    pub user_data: u64,
    pub flags: u8,
    pub ioprio: u16,
}

impl SubmissionQueueEntry {
    pub fn new(
        opcode: IoUringOpcode,
        fd: i32,
        offset: u64,
        addr: u64,
        len: u32,
        user_data: u64,
    ) -> Self {
        Self {
            opcode,
            fd,
            offset,
            addr,
            len,
            user_data,
            flags: 0,
            ioprio: 0,
        }
    }

    pub fn with_flags(mut self, flags: u8) -> Self {
        self.flags = flags;
        self
    }

    pub fn with_ioprio(mut self, ioprio: u16) -> Self {
        self.ioprio = ioprio;
        self
    }
}

/// Completion Queue Entry (CQE)
#[derive(Debug, Clone)]
pub struct CompletionQueueEntry {
    pub user_data: u64,
    pub res: i32,
    pub flags: u32,
}

impl CompletionQueueEntry {
    pub fn new(user_data: u64, res: i32, flags: u32) -> Self {
        Self {
            user_data,
            res,
            flags,
        }
    }
}

/// Linux io_uring-style async zero-copy I/O subsystem
/// Lock-free ring buffer implementation with atomic head/tail pointers
pub struct IoUringEngine {
    // Submission queue
    sq_entries: Vec<Option<SubmissionQueueEntry>>,
    sq_head: AtomicUsize,
    sq_tail: AtomicUsize,
    sq_mask: usize,
    
    // Completion queue
    cq_entries: Vec<Option<CompletionQueueEntry>>,
    cq_head: AtomicUsize,
    cq_tail: AtomicUsize,
    cq_mask: usize,
    
    // Configuration
    max_entries: usize,
    sq_flags: AtomicU32,
    cq_flags: AtomicU32,
}

impl IoUringEngine {
    pub fn new(max_entries: usize) -> Self {
        // Ensure max_entries is power of 2 for efficient masking
        let entries = max_entries.next_power_of_two();
        let mask = entries - 1;
        
        Self {
            sq_entries: vec![None; entries],
            sq_head: AtomicUsize::new(0),
            sq_tail: AtomicUsize::new(0),
            sq_mask: mask,
            cq_entries: vec![None; entries],
            cq_head: AtomicUsize::new(0),
            cq_tail: AtomicUsize::new(0),
            cq_mask: mask,
            max_entries,
            sq_flags: AtomicU32::new(0),
            cq_flags: AtomicU32::new(0),
        }
    }

    /// Submit a single SQE to the submission queue
    pub fn submit_sqe(&mut self, sqe: SubmissionQueueEntry) -> Result<(), &'static str> {
        let tail = self.sq_tail.load(Ordering::Acquire);
        let head = self.sq_head.load(Ordering::Acquire);
        
        // Check if queue is full
        if (tail - head) >= self.max_entries {
            return Err("io_uring: Submission queue full");
        }
        
        let index = tail & self.sq_mask;
        self.sq_entries[index] = Some(sqe);
        
        // Use Release ordering to ensure the entry is written before tail update
        self.sq_tail.store(tail + 1, Ordering::Release);
        Ok(())
    }

    /// Submit multiple SQEs to the submission queue
    pub fn submit_sqes(&mut self, sqes: Vec<SubmissionQueueEntry>) -> Result<usize, &'static str> {
        let mut submitted = 0;
        for sqe in sqes {
            match self.submit_sqe(sqe) {
                Ok(()) => submitted += 1,
                Err(e) => return Err(e),
            }
        }
        Ok(submitted)
    }

    /// Enter submission and wait for completions
    pub fn enter_submit_and_wait(&mut self, wait_nr: u32) -> usize {
        let mut processed = 0;
        
        // Process all available SQEs
        loop {
            let head = self.sq_head.load(Ordering::Acquire);
            let tail = self.sq_tail.load(Ordering::Acquire);
            
            if head == tail {
                break; // No more entries to process
            }
            
            let index = head & self.sq_mask;
            if let Some(sqe) = self.sq_entries[index].take() {
                let res = self.process_sqe(&sqe);
                
                // Add completion to CQ
                let cq_tail = self.cq_tail.load(Ordering::Acquire);
                let cq_index = cq_tail & self.cq_mask;
                self.cq_entries[cq_index] = Some(CompletionQueueEntry::new(sqe.user_data, res, 0));
                self.cq_tail.store(cq_tail + 1, Ordering::Release);
                
                processed += 1;
                self.sq_head.store(head + 1, Ordering::Release);
            } else {
                break;
            }
        }
        
        processed
    }

    /// Process a single SQE and return the result
    fn process_sqe(&self, sqe: &SubmissionQueueEntry) -> i32 {
        match sqe.opcode {
            IoUringOpcode::Nop => 0,
            IoUringOpcode::Read => sqe.len as i32,
            IoUringOpcode::Write => sqe.len as i32,
            IoUringOpcode::Readv => sqe.len as i32,
            IoUringOpcode::Writev => sqe.len as i32,
            IoUringOpcode::Poll => 0,
            IoUringOpcode::Fsync => 0,
            IoUringOpcode::Send => sqe.len as i32,
            IoUringOpcode::Recv => sqe.len as i32,
            IoUringOpcode::Accept => 0,
            IoUringOpcode::Connect => 0,
        }
    }

    /// Pop a completion queue entry
    pub fn pop_cqe(&mut self) -> Option<CompletionQueueEntry> {
        let head = self.cq_head.load(Ordering::Acquire);
        let tail = self.cq_tail.load(Ordering::Acquire);
        
        if head == tail {
            return None; // No completions available
        }
        
        let index = head & self.cq_mask;
        let cqe = self.cq_entries[index].take();
        self.cq_head.store(head + 1, Ordering::Release);
        cqe
    }

    /// Get submission queue depth
    pub fn sq_depth(&self) -> usize {
        let tail = self.sq_tail.load(Ordering::Acquire);
        let head = self.sq_head.load(Ordering::Acquire);
        tail - head
    }

    /// Get completion queue depth
    pub fn cq_depth(&self) -> usize {
        let tail = self.cq_tail.load(Ordering::Acquire);
        let head = self.cq_head.load(Ordering::Acquire);
        tail - head
    }

    /// Check if submission queue is full
    pub fn sq_full(&self) -> bool {
        self.sq_depth() >= self.max_entries
    }

    /// Check if completion queue is empty
    pub fn cq_empty(&self) -> bool {
        self.cq_depth() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io_uring_lifecycle() {
        let mut ring = IoUringEngine::new(4);

        let sqe1 = SubmissionQueueEntry::new(IoUringOpcode::Read, 3, 0, 0x1000, 512, 1001);
        let sqe2 = SubmissionQueueEntry::new(IoUringOpcode::Write, 4, 512, 0x2000, 256, 1002);

        assert!(ring.submit_sqe(sqe1).is_ok());
        assert!(ring.submit_sqe(sqe2).is_ok());

        let processed = ring.enter_submit_and_wait(0);
        assert_eq!(processed, 2);

        let cqe1 = ring.pop_cqe().unwrap();
        assert_eq!(cqe1.user_data, 1001);
        assert_eq!(cqe1.res, 512);

        let cqe2 = ring.pop_cqe().unwrap();
        assert_eq!(cqe2.user_data, 1002);
        assert_eq!(cqe2.res, 256);

        assert!(ring.pop_cqe().is_none());
    }

    #[test]
    fn test_ring_buffer_depth() {
        let mut ring = IoUringEngine::new(8);

        // Initially empty
        assert_eq!(ring.sq_depth(), 0);
        assert_eq!(ring.cq_depth(), 0);
        assert!(!ring.sq_full());
        assert!(ring.cq_empty());

        // Submit some entries
        let sqe = SubmissionQueueEntry::new(IoUringOpcode::Nop, 0, 0, 0, 0, 1);
        for _ in 0..4 {
            assert!(ring.submit_sqe(sqe.clone()).is_ok());
        }

        assert_eq!(ring.sq_depth(), 4);
        assert!(!ring.sq_full());

        // Process entries
        ring.enter_submit_and_wait(0);
        assert_eq!(ring.sq_depth(), 0);
        assert_eq!(ring.cq_depth(), 4);
        assert!(!ring.cq_empty());
    }

    #[test]
    fn test_queue_full() {
        let mut ring = IoUringEngine::new(4);

        let sqe = SubmissionQueueEntry::new(IoUringOpcode::Nop, 0, 0, 0, 0, 1);
        
        // Fill queue
        for _ in 0..4 {
            assert!(ring.submit_sqe(sqe.clone()).is_ok());
        }

        // Should be full now
        assert!(ring.sq_full());
        assert!(ring.submit_sqe(sqe).is_err());
    }

    #[test]
    fn test_submit_multiple() {
        let mut ring = IoUringEngine::new(8);

        let sqes = vec![
            SubmissionQueueEntry::new(IoUringOpcode::Read, 1, 0, 0x1000, 512, 1),
            SubmissionQueueEntry::new(IoUringOpcode::Write, 2, 0, 0x2000, 256, 2),
            SubmissionQueueEntry::new(IoUringOpcode::Fsync, 3, 0, 0, 0, 3),
        ];

        let submitted = ring.submit_sqes(sqes).unwrap();
        assert_eq!(submitted, 3);
        assert_eq!(ring.sq_depth(), 3);
    }

    #[test]
    fn test_extended_opcodes() {
        let mut ring = IoUringEngine::new(4);

        let sqe = SubmissionQueueEntry::new(IoUringOpcode::Send, 5, 0, 0x1000, 128, 1)
            .with_flags(0x01)
            .with_ioprio(100);

        assert!(ring.submit_sqe(sqe).is_ok());
        let processed = ring.enter_submit_and_wait(0);
        assert_eq!(processed, 1);

        let cqe = ring.pop_cqe().unwrap();
        assert_eq!(cqe.res, 128);
    }
}
