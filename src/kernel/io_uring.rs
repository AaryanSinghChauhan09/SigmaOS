use std::vec::Vec;

/// io_uring operation codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IoUringOpcode {
    Nop = 0,
    Read = 1,
    Write = 2,
    Poll = 3,
    Fsync = 4,
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
    pub buf_index: u16, // Fixed registered buffer index
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
            buf_index: 0,
        }
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

/// Pre-registered fixed buffer descriptor for zero-copy I/O
#[derive(Debug, Clone)]
pub struct RegisteredBuffer {
    pub virt_addr: u64,
    pub len: usize,
}

/// Kernel SQPOLL (Submission Queue Polling) Daemon Thread Simulation
pub struct IoUringSqpollDaemon {
    pub active: bool,
    pub polls_executed: usize,
}

impl IoUringSqpollDaemon {
    pub fn new() -> Self {
        Self {
            active: true,
            polls_executed: 0,
        }
    }

    /// Continuously poll submission ring and submit SQEs without userland syscall overhead
    pub fn poll_and_submit(&mut self, engine: &mut IoUringEngine) -> usize {
        if !self.active || engine.sq_entries.is_empty() {
            return 0;
        }
        self.polls_executed += 1;
        engine.enter_submit_and_wait()
    }
}

impl Default for IoUringSqpollDaemon {
    fn default() -> Self {
        Self::new()
    }
}

/// Linux io_uring-style async zero-copy I/O subsystem
pub struct IoUringEngine {
    pub sq_entries: Vec<SubmissionQueueEntry>,
    pub cq_entries: Vec<CompletionQueueEntry>,
    pub registered_files: Vec<i32>,
    pub registered_buffers: Vec<RegisteredBuffer>,
    pub sqpoll_daemon: IoUringSqpollDaemon,
    pub max_entries: usize,
}

impl IoUringEngine {
    pub fn new(max_entries: usize) -> Self {
        Self {
            sq_entries: Vec::new(),
            cq_entries: Vec::new(),
            registered_files: Vec::new(),
            registered_buffers: Vec::new(),
            sqpoll_daemon: IoUringSqpollDaemon::new(),
            max_entries,
        }
    }

    /// Register fixed file descriptors to bypass VFS file table lookups on every I/O
    pub fn register_files(&mut self, fds: &[i32]) {
        self.registered_files.extend_from_slice(fds);
    }

    /// Register persistent memory buffers for zero-copy DMA I/O
    pub fn register_buffers(&mut self, buffers: &[(u64, usize)]) {
        for &(addr, len) in buffers {
            self.registered_buffers.push(RegisteredBuffer { virt_addr: addr, len });
        }
    }

    pub fn submit_sqe(&mut self, sqe: SubmissionQueueEntry) -> Result<(), &'static str> {
        if self.sq_entries.len() >= self.max_entries {
            return Err("io_uring: Submission queue full");
        }
        self.sq_entries.push(sqe);
        Ok(())
    }

    pub fn enter_submit_and_wait(&mut self) -> usize {
        let mut processed = 0;
        let entries = self.sq_entries.clone();
        self.sq_entries.clear();

        for sqe in entries {
            let res = match sqe.opcode {
                IoUringOpcode::Nop => 0,
                IoUringOpcode::Read => sqe.len as i32,
                IoUringOpcode::Write => sqe.len as i32,
                IoUringOpcode::Poll => 0,
                IoUringOpcode::Fsync => 0,
            };

            self.cq_entries
                .push(CompletionQueueEntry::new(sqe.user_data, res, 0));
            processed += 1;
        }

        processed
    }

    pub fn pop_cqe(&mut self) -> Option<CompletionQueueEntry> {
        if self.cq_entries.is_empty() {
            None
        } else {
            Some(self.cq_entries.remove(0))
        }
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

        let processed = ring.enter_submit_and_wait();
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
    fn test_sqpoll_kernel_daemon() {
        let mut ring = IoUringEngine::new(8);
        ring.register_files(&[3, 4, 5]);
        ring.register_buffers(&[(0x1000, 4096), (0x2000, 4096)]);

        let sqe = SubmissionQueueEntry::new(IoUringOpcode::Read, 3, 0, 0x1000, 1024, 2001);
        ring.submit_sqe(sqe).unwrap();

        let processed = ring.sqpoll_daemon.poll_and_submit(&mut ring);
        assert_eq!(processed, 1);
        assert_eq!(ring.sqpoll_daemon.polls_executed, 1);
        assert_eq!(ring.registered_files.len(), 3);
        assert_eq!(ring.registered_buffers.len(), 2);
    }
}
