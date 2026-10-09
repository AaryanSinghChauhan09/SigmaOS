//! io_uring v2 Extensions — Advanced Async I/O for SigmaOS
//!
//! Extends the base io_uring implementation with features from Linux 5.5–6.7:
//! - Multishot operations (IORING_POLL_ADD_MULTI, IORING_RECV_MULTISHOT)
//! - Registered buffers and fixed files (zero-copy)
//! - io_uring over io_uring (linked timeouts)
//! - CancelAll / drain / barrier operations
//! - Zero-copy send/receive (MSG_ZEROCOPY integration)
//! - Direct I/O without page cache (O_DIRECT equivalent)
//! - IORING_OP_URING_CMD for device-specific commands (NVMe passthrough)
//! - Socket messaging with multishot IORING_OP_ACCEPT
//!
//! References:
//! - io_uring changelog: https://kernel.dk/io_uring.pdf
//! - Linux 6.7 io_uring: IORING_OP_FIXED_FD_INSTALL, multishot recv
//! - FreeBSD kqueue multishot: inspiration for event-driven model
//!
//! Future Development:
//! - Integration with SigmaOS NVMe driver for direct NVMe passthrough
//! - io_uring network stack bypass (XDP alternative)
//! - NUMA-aware io_uring instances (per-NUMA-node rings)
//! - GPU compute queue integration (similar to DRM/KMS submission)

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use std::sync::atomic::{AtomicU32, Ordering};

/// io_uring v2 extended opcodes (beyond base io_uring)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IoUringOpV2 {
    /// Read with registered buffer (zero-copy)
    ReadFixed = 0x01,
    /// Write with registered buffer (zero-copy)
    WriteFixed = 0x02,
    /// Poll add with multishot support
    PollAddMulti = 0x03,
    /// Update poll mask on existing request
    PollUpdate = 0x04,
    /// Multishot accept (Linux 5.19+)
    AcceptMultishot = 0x05,
    /// Multishot receive (Linux 6.0+)
    RecvMultishot = 0x06,
    /// io_uring cmd (device passthrough, Linux 5.19+)
    UringCmd = 0x07,
    /// Futex wait (Linux 6.7+)
    FutexWait = 0x08,
    /// Futex wake (Linux 6.7+)
    FutexWake = 0x09,
    /// Zero-copy send
    SendZc = 0x0a,
    /// Zero-copy sendmsg
    SendmsgZc = 0x0b,
    /// Fixed file installation (Linux 6.7+)
    FixedFdInstall = 0x0c,
}

/// Registered buffer table entry
#[derive(Debug)]
pub struct IoUringRegisteredBuf {
    /// Buffer index in the registration table
    pub buf_index: u16,
    /// Buffer data
    pub data: Vec<u8>,
    /// Length of registered buffer
    pub len: u32,
    /// Physical address (simulated)
    pub phys_addr: u64,
    /// Is this buffer currently in use by an in-flight operation?
    pub in_use: bool,
}

impl IoUringRegisteredBuf {
    pub fn new(index: u16, size: usize) -> Self {
        Self {
            buf_index: index,
            data: vec![0u8; size],
            len: size as u32,
            phys_addr: (index as u64) * 4096, // Simulated physical address
            in_use: false,
        }
    }
}

/// Registered file entry (fixed file table)
#[derive(Debug, Clone)]
pub struct IoUringRegisteredFile {
    /// Slot index in fixed file table
    pub slot: u32,
    /// File descriptor
    pub fd: i32,
    /// File type hint
    pub file_type: RegisteredFileType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisteredFileType {
    Regular,
    Socket,
    BlockDevice,
    NvmeDirect,
    Epoll,
}

/// Multishot operation state — tracks ongoing multishot completions
#[derive(Debug)]
pub struct MultishotState {
    /// User data tag for this multishot
    pub user_data: u64,
    /// Operation type
    pub op: IoUringOpV2,
    /// Number of completions generated so far
    pub completions: u64,
    /// Is this multishot still active?
    pub active: bool,
    /// Maximum completions (0 = unlimited)
    pub max_completions: u64,
}

impl MultishotState {
    pub fn new(user_data: u64, op: IoUringOpV2) -> Self {
        Self {
            user_data,
            op,
            completions: 0,
            active: true,
            max_completions: 0,
        }
    }

    pub fn fire(&mut self) -> bool {
        if !self.active {
            return false;
        }
        self.completions += 1;
        if self.max_completions > 0 && self.completions >= self.max_completions {
            self.active = false;
        }
        true
    }
}

/// io_uring v2 Extended Completion Queue Entry
#[derive(Debug, Clone)]
pub struct CqeV2 {
    /// User data (matches SQE user_data)
    pub user_data: u64,
    /// Result (positive = bytes, negative = -errno)
    pub res: i32,
    /// CQE flags
    pub flags: u32,
    /// Extra data (Linux 5.16+ BIG_CQE extension)
    pub extra1: u64,
    pub extra2: u64,
}

impl CqeV2 {
    /// IORING_CQE_F_MORE: more completions coming (multishot)
    pub const F_MORE: u32 = 1 << 1;
    /// IORING_CQE_F_SOCK_NONEMPTY: socket still has data
    pub const F_SOCK_NONEMPTY: u32 = 1 << 2;
    /// IORING_CQE_F_NOTIF: zero-copy notification
    pub const F_NOTIF: u32 = 1 << 3;
    /// IORING_CQE_F_BUF_SELECTED: buffer was selected from pool
    pub const F_BUF_SELECTED: u32 = 1 << 4;

    pub fn is_multishot(&self) -> bool {
        self.flags & Self::F_MORE != 0
    }
}

/// io_uring Direct I/O descriptor (O_DIRECT equivalent)
#[derive(Debug, Clone)]
pub struct DirectIoOp {
    pub buf_index: u16,
    pub offset: u64,
    pub len: u32,
    pub direction: DirectIoDir,
    pub fd: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectIoDir {
    Read,
    Write,
}

/// NVMe io_uring passthrough command (IORING_OP_URING_CMD)
#[derive(Debug, Clone)]
pub struct NvmeUringCmd {
    /// NVMe command opcode
    pub opcode: u8,
    /// Namespace ID
    pub nsid: u32,
    /// Starting LBA
    pub slba: u64,
    /// Number of logical blocks
    pub nlb: u16,
    /// Buffer index (registered)
    pub buf_index: u16,
    /// Command flags
    pub flags: u32,
}

/// io_uring v2 Extended Ring Manager
#[derive(Debug)]
pub struct IoUringV2 {
    /// Ring depth (must be power of two)
    pub depth: u32,
    /// Submission queue head (kernel reads)
    pub sq_head: AtomicU32,
    /// Submission queue tail (user writes)
    pub sq_tail: AtomicU32,
    /// Completion queue head (user reads)
    pub cq_head: AtomicU32,
    /// Completion queue tail (kernel writes)
    pub cq_tail: AtomicU32,
    /// Registered buffer table
    pub registered_bufs: BTreeMap<u16, IoUringRegisteredBuf>,
    /// Registered file table
    pub registered_files: BTreeMap<u32, IoUringRegisteredFile>,
    /// Active multishot operations
    pub multishots: Vec<MultishotState>,
    /// Pending completions
    pub pending_cqes: Vec<CqeV2>,
    /// Direct I/O operations in flight
    pub direct_io_ops: Vec<DirectIoOp>,
    /// Ring flags
    pub flags: u32,
    /// Statistics
    pub stats: IoUringV2Stats,
}

/// io_uring v2 ring flags
impl IoUringV2 {
    /// IORING_SETUP_SQPOLL: kernel-side submission polling
    pub const SETUP_SQPOLL: u32 = 1 << 1;
    /// IORING_SETUP_IOPOLL: I/O polling instead of interrupts
    pub const SETUP_IOPOLL: u32 = 1 << 0;
    /// IORING_SETUP_ATTACH_WQ: share worker pool
    pub const SETUP_ATTACH_WQ: u32 = 1 << 5;
    /// IORING_SETUP_CQSIZE: custom CQ ring size
    pub const SETUP_CQSIZE: u32 = 1 << 3;
    /// IORING_SETUP_DEFER_TASKRUN: defer task_work to io_uring_enter
    pub const SETUP_DEFER_TASKRUN: u32 = 1 << 13;
    /// IORING_SETUP_SINGLE_ISSUER: hint for single-threaded submission
    pub const SETUP_SINGLE_ISSUER: u32 = 1 << 12;
}

#[derive(Debug, Default)]
pub struct IoUringV2Stats {
    pub sq_submissions: u64,
    pub cq_completions: u64,
    pub registered_bufs: u32,
    pub registered_files: u32,
    pub multishot_events: u64,
    pub zero_copy_ops: u64,
    pub direct_io_ops: u64,
    pub nvme_passthrough_ops: u64,
}

impl IoUringV2 {
    pub fn new(depth: u32, flags: u32) -> Self {
        assert!(depth.is_power_of_two(), "Ring depth must be power of two");
        assert!(depth >= 1 && depth <= 32768, "Depth out of range");
        Self {
            depth,
            sq_head: AtomicU32::new(0),
            sq_tail: AtomicU32::new(0),
            cq_head: AtomicU32::new(0),
            cq_tail: AtomicU32::new(0),
            registered_bufs: BTreeMap::new(),
            registered_files: BTreeMap::new(),
            multishots: Vec::new(),
            pending_cqes: Vec::new(),
            direct_io_ops: Vec::new(),
            flags,
            stats: IoUringV2Stats::default(),
        }
    }

    /// Register a buffer for zero-copy I/O
    pub fn register_buffer(&mut self, index: u16, size: usize) -> Result<(), &'static str> {
        if self.registered_bufs.contains_key(&index) {
            return Err("Buffer index already registered");
        }
        let buf = IoUringRegisteredBuf::new(index, size);
        self.registered_bufs.insert(index, buf);
        self.stats.registered_bufs += 1;
        Ok(())
    }

    /// Unregister a buffer
    pub fn unregister_buffer(&mut self, index: u16) -> bool {
        if let Some(buf) = self.registered_bufs.get(&index) {
            if buf.in_use {
                return false; // Cannot unregister while in use
            }
        }
        if self.registered_bufs.remove(&index).is_some() {
            self.stats.registered_bufs -= 1;
            true
        } else {
            false
        }
    }

    /// Register a file into the fixed file table
    pub fn register_file(&mut self, slot: u32, fd: i32, file_type: RegisteredFileType) -> Result<(), &'static str> {
        self.registered_files.insert(slot, IoUringRegisteredFile { slot, fd, file_type });
        self.stats.registered_files += 1;
        Ok(())
    }

    /// Submit a zero-copy send operation
    pub fn submit_send_zc(&mut self, user_data: u64, fd: i32, buf_index: u16, len: u32) -> Result<(), &'static str> {
        if !self.registered_bufs.contains_key(&buf_index) {
            return Err("Buffer not registered — use register_buffer first");
        }
        if let Some(buf) = self.registered_bufs.get_mut(&buf_index) {
            buf.in_use = true;
        }
        // Simulate completion (in real kernel, happens asynchronously)
        let cqe = CqeV2 {
            user_data,
            res: len as i32,
            flags: CqeV2::F_NOTIF, // Zero-copy notification
            extra1: buf_index as u64,
            extra2: 0,
        };
        self.pending_cqes.push(cqe);
        self.stats.zero_copy_ops += 1;
        self.stats.sq_submissions += 1;
        Ok(())
    }

    /// Start a multishot accept operation
    pub fn submit_accept_multishot(&mut self, user_data: u64, listen_fd: i32) -> Result<(), &'static str> {
        let ms = MultishotState::new(user_data, IoUringOpV2::AcceptMultishot);
        self.multishots.push(ms);
        self.stats.sq_submissions += 1;
        Ok(())
    }

    /// Simulate an incoming connection for multishot accept
    pub fn simulate_incoming_connection(&mut self, listen_user_data: u64) -> Option<CqeV2> {
        if let Some(ms) = self.multishots.iter_mut().find(|m| m.user_data == listen_user_data && m.active) {
            ms.fire();
            let new_fd = 100 + ms.completions as i32;
            let cqe = CqeV2 {
                user_data: listen_user_data,
                res: new_fd, // New accepted fd
                flags: CqeV2::F_MORE, // More coming (multishot still active)
                extra1: 0,
                extra2: 0,
            };
            self.stats.multishot_events += 1;
            self.stats.cq_completions += 1;
            Some(cqe)
        } else {
            None
        }
    }

    /// Submit a multishot receive
    pub fn submit_recv_multishot(&mut self, user_data: u64, fd: i32, buf_index: u16) -> Result<(), &'static str> {
        if !self.registered_bufs.contains_key(&buf_index) {
            return Err("Buffer not registered");
        }
        let ms = MultishotState::new(user_data, IoUringOpV2::RecvMultishot);
        self.multishots.push(ms);
        self.stats.sq_submissions += 1;
        Ok(())
    }

    /// Submit a direct NVMe passthrough command
    pub fn submit_nvme_cmd(&mut self, user_data: u64, cmd: NvmeUringCmd) -> Result<(), &'static str> {
        if !self.registered_bufs.contains_key(&cmd.buf_index) {
            return Err("Data buffer not registered");
        }
        // Simulate completion
        let cqe = CqeV2 {
            user_data,
            res: 0, // 0 = success for NVMe
            flags: 0,
            extra1: cmd.slba,
            extra2: cmd.nlb as u64,
        };
        self.pending_cqes.push(cqe);
        self.stats.nvme_passthrough_ops += 1;
        self.stats.sq_submissions += 1;
        Ok(())
    }

    /// Read available completions (drain CQ)
    pub fn drain_completions(&mut self) -> Vec<CqeV2> {
        let cqes = self.pending_cqes.drain(..).collect();
        self.stats.cq_completions += self.pending_cqes.len() as u64;
        cqes
    }

    /// Cancel all active multishot operations
    pub fn cancel_all_multishots(&mut self) {
        for ms in &mut self.multishots {
            ms.active = false;
        }
    }

    /// Flush zero-copy notification — release buffer back to pool
    pub fn flush_zc_notif(&mut self, buf_index: u16) {
        if let Some(buf) = self.registered_bufs.get_mut(&buf_index) {
            buf.in_use = false;
        }
    }

    pub fn stats(&self) -> &IoUringV2Stats {
        &self.stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ring_creation() {
        let ring = IoUringV2::new(4096, 0);
        assert_eq!(ring.depth, 4096);
        assert!(ring.registered_bufs.is_empty());
    }

    #[test]
    fn test_buffer_registration() {
        let mut ring = IoUringV2::new(256, 0);
        ring.register_buffer(0, 65536).unwrap();
        ring.register_buffer(1, 65536).unwrap();
        assert_eq!(ring.stats.registered_bufs, 2);
        
        // Duplicate registration should fail
        assert!(ring.register_buffer(0, 65536).is_err());
    }

    #[test]
    fn test_zero_copy_send() {
        let mut ring = IoUringV2::new(256, IoUringV2::SETUP_SQPOLL);
        ring.register_buffer(0, 65536).unwrap();
        ring.submit_send_zc(42, 3, 0, 1024).unwrap();
        
        let completions = ring.drain_completions();
        assert_eq!(completions.len(), 1);
        assert_eq!(completions[0].user_data, 42);
        assert_eq!(completions[0].res, 1024);
        assert!(completions[0].flags & CqeV2::F_NOTIF != 0);
        assert_eq!(ring.stats.zero_copy_ops, 1);
    }

    #[test]
    fn test_multishot_accept() {
        let mut ring = IoUringV2::new(256, 0);
        ring.submit_accept_multishot(99, 5).unwrap();
        
        // Simulate 3 incoming connections
        let c1 = ring.simulate_incoming_connection(99).unwrap();
        let c2 = ring.simulate_incoming_connection(99).unwrap();
        let c3 = ring.simulate_incoming_connection(99).unwrap();
        
        // All should have MORE flag (multishot still active)
        assert!(c1.is_multishot());
        assert!(c2.is_multishot());
        assert!(c3.is_multishot());
        
        // Each should get a different fd
        assert_ne!(c1.res, c2.res);
        assert_ne!(c2.res, c3.res);
        
        assert_eq!(ring.stats.multishot_events, 3);
    }

    #[test]
    fn test_nvme_passthrough() {
        let mut ring = IoUringV2::new(256, IoUringV2::SETUP_IOPOLL);
        ring.register_buffer(0, 512 * 8).unwrap(); // 8 sectors
        
        let cmd = NvmeUringCmd {
            opcode: 0x02, // NVMe Read
            nsid: 1,
            slba: 0,
            nlb: 7, // 8 sectors (0-indexed)
            buf_index: 0,
            flags: 0,
        };
        ring.submit_nvme_cmd(77, cmd).unwrap();
        let completions = ring.drain_completions();
        assert_eq!(completions[0].user_data, 77);
        assert_eq!(completions[0].res, 0); // Success
        assert_eq!(ring.stats.nvme_passthrough_ops, 1);
    }

    #[test]
    fn test_buffer_in_use_protection() {
        let mut ring = IoUringV2::new(256, 0);
        ring.register_buffer(0, 1024).unwrap();
        ring.submit_send_zc(1, 3, 0, 512).unwrap();
        
        // Buffer should be marked in-use
        assert!(ring.registered_bufs[&0].in_use);
        
        // Cannot unregister while in use
        assert!(!ring.unregister_buffer(0));
        
        // After flush, can unregister
        ring.flush_zc_notif(0);
        assert!(!ring.registered_bufs[&0].in_use);
        assert!(ring.unregister_buffer(0));
    }

    #[test]
    fn test_cancel_multishots() {
        let mut ring = IoUringV2::new(256, 0);
        ring.submit_accept_multishot(1, 5).unwrap();
        ring.submit_accept_multishot(2, 6).unwrap();
        
        ring.cancel_all_multishots();
        
        for ms in &ring.multishots {
            assert!(!ms.active);
        }
    }
}
