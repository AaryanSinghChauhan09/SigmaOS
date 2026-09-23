//! Lock-Free High-Performance io_uring Simulation Subsystem
//! Implements Submission Queue (SQ), Completion Queue (CQ) with atomic indexes and file descriptor pre-registration.

use core::sync::atomic::{AtomicUsize, Ordering};

pub const SQ_RING_SIZE: usize = 16;
pub const CQ_RING_SIZE: usize = 16;
pub const MAX_REGISTERED_FILES: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoOpcode {
    Nop = 0,
    Read = 1,
    Write = 2,
    Fsync = 3,
}

#[derive(Debug, Clone, Copy)]
pub struct SubmissionQueueEntry {
    pub opcode: IoOpcode,
    pub fd: i32,
    pub offset: u64,
    pub buf_addr: usize,
    pub len: usize,
    pub user_data: u64,
    pub use_registered_file: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompletionQueueEntry {
    pub user_data: u64,
    pub result: i32, // bytes read/written, or 0 for success, negative for error
    pub flags: u32,
}

pub struct IoUring {
    pub sqes: [Option<SubmissionQueueEntry>; SQ_RING_SIZE],
    pub cqes: [Option<CompletionQueueEntry>; CQ_RING_SIZE],

    // SQ pointers
    pub sq_head: AtomicUsize,
    pub sq_tail: AtomicUsize,

    // CQ pointers
    pub cq_head: AtomicUsize,
    pub cq_tail: AtomicUsize,

    // File pre-registration table to avoid expensive fd lookups on hot paths
    pub registered_files: [Option<i32>; MAX_REGISTERED_FILES],
}

impl IoUring {
    pub fn new() -> Self {
        Self {
            sqes: [None; SQ_RING_SIZE],
            cqes: [None; CQ_RING_SIZE],
            sq_head: AtomicUsize::new(0),
            sq_tail: AtomicUsize::new(0),
            cq_head: AtomicUsize::new(0),
            cq_tail: AtomicUsize::new(0),
            registered_files: [None; MAX_REGISTERED_FILES],
        }
    }

    /// Register file descriptors to skip runtime descriptor translation overhead
    pub fn register_files(&mut self, fds: &[i32]) -> Result<usize, &'static str> {
        let mut count = 0;
        for &fd in fds {
            if count >= MAX_REGISTERED_FILES {
                return Err("Registered files list full");
            }
            self.registered_files[count] = Some(fd);
            count += 1;
        }
        Ok(count)
    }

    /// Submit a task asynchronously to the SQ
    pub fn submit_entry(&mut self, sqe: SubmissionQueueEntry) -> Result<(), &'static str> {
        let head = self.sq_head.load(Ordering::Relaxed);
        let tail = self.sq_tail.load(Ordering::Acquire);

        if tail.wrapping_sub(head) >= SQ_RING_SIZE {
            return Err("Submission Queue is full");
        }

        let idx = tail % SQ_RING_SIZE;
        self.sqes[idx] = Some(sqe);
        self.sq_tail.store(tail.wrapping_add(1), Ordering::Release);
        Ok(())
    }

    /// Kernel side: Process SQ entries and append completion results to CQ
    pub fn process_completions(&mut self) -> usize {
        let mut processed = 0;
        let mut head = self.sq_head.load(Ordering::Acquire);
        let tail = self.sq_tail.load(Ordering::Relaxed);

        while head != tail {
            let idx = head % SQ_RING_SIZE;
            if let Some(sqe) = self.sqes[idx].take() {
                // Verify file registration if specified
                let mut fd_valid = true;
                if sqe.use_registered_file {
                    let reg_idx = sqe.fd as usize;
                    if reg_idx >= MAX_REGISTERED_FILES || self.registered_files[reg_idx].is_none() {
                        fd_valid = false;
                    }
                }

                let result = if !fd_valid {
                    -9 // EBADF (Bad File Descriptor)
                } else {
                    match sqe.opcode {
                        IoOpcode::Nop => 0,
                        IoOpcode::Read => sqe.len as i32, // simulated read bytes
                        IoOpcode::Write => sqe.len as i32, // simulated write bytes
                        IoOpcode::Fsync => 0,
                    }
                };

                let cqe = CompletionQueueEntry {
                    user_data: sqe.user_data,
                    result,
                    flags: 0,
                };

                let _ = self.push_cqe(cqe);
                processed += 1;
            }
            head = head.wrapping_add(1);
            self.sq_head.store(head, Ordering::Release);
        }

        processed
    }

    /// Helper to push completion event to CQ
    fn push_cqe(&mut self, cqe: CompletionQueueEntry) -> Result<(), &'static str> {
        let head = self.cq_head.load(Ordering::Relaxed);
        let tail = self.cq_tail.load(Ordering::Acquire);

        if tail.wrapping_sub(head) >= CQ_RING_SIZE {
            return Err("Completion Queue is full");
        }

        let idx = tail % CQ_RING_SIZE;
        self.cqes[idx] = Some(cqe);
        self.cq_tail.store(tail.wrapping_add(1), Ordering::Release);
        Ok(())
    }

    /// Dequeue a completion event from CQ
    pub fn pop_completion(&mut self) -> Option<CompletionQueueEntry> {
        let head = self.cq_head.load(Ordering::Acquire);
        let tail = self.cq_tail.load(Ordering::Relaxed);

        if head == tail {
            return None;
        }

        let idx = head % CQ_RING_SIZE;
        let cqe = self.cqes[idx].take();
        self.cq_head.store(head.wrapping_add(1), Ordering::Release);
        cqe
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_io_uring_basic_submission_completion() {
        let mut ring = IoUring::new();

        // Register file descriptors
        let registered = ring.register_files(&[100, 101]).unwrap();
        assert_eq!(registered, 2);

        // Submit a read operation using a pre-registered file descriptor (index 0 maps to 100)
        let sqe = SubmissionQueueEntry {
            opcode: IoOpcode::Read,
            fd: 0, // Registered index 0
            offset: 0,
            buf_addr: 0x4000,
            len: 512,
            user_data: 0x9999,
            use_registered_file: true,
        };

        ring.submit_entry(sqe).unwrap();

        // Process submission queue
        let processed = ring.process_completions();
        assert_eq!(processed, 1);

        // Retrieve from completion queue
        let cqe = ring.pop_completion().unwrap();
        assert_eq!(cqe.user_data, 0x9999);
        assert_eq!(cqe.result, 512); // successfully simulated read of 512 bytes
    }
}
