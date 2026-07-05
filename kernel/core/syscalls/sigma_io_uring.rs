// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/syscalls/sigma_io_uring.rs — Async I/O Ring (io_uring-inspired)
//
// Implements a Linux io_uring-compatible asynchronous I/O subsystem for SigmaOS.
//
// Design overview:
//   - Submission Queue (SQ): ring buffer of SqEntry descriptors
//   - Completion Queue (CQ): ring buffer of CqEntry results
//   - Producer/consumer model: userspace submits, kernel completes
//   - Zero-copy: descriptor contains direct buffer pointers
//   - No heap: fixed-size static rings, single instance per instance
//
// Supports operations:
//   - Nop       (0): no-op, for benchmarking
//   - Read      (1): read from fd into buffer
//   - Write     (2): write buffer to fd
//   - Fsync     (3): flush fd to storage
//   - PollAdd   (4): wait for fd to become readable/writable
//   - Close     (5): close a file descriptor
//
// Language: Rust #![no_std] — no alloc, no external crates.
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU32, AtomicI32, Ordering};

// ── Types ─────────────────────────────────────────────────────────────────────
type SigmaU8    = u8;
type SigmaU16   = u16;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaI32   = i32;
type SigmaI64   = i64;
type SigmaUsize = usize;
type SigmaBool  = bool;

// ── Ring Sizes ────────────────────────────────────────────────────────────────
/// Submission queue depth (must be power of two).
const SQ_RING_SIZE: SigmaUsize = 256;
/// Completion queue depth (typically 2× SQ).
const CQ_RING_SIZE: SigmaUsize = 512;
const SQ_MASK: SigmaU32 = (SQ_RING_SIZE as SigmaU32) - 1;
const CQ_MASK: SigmaU32 = (CQ_RING_SIZE as SigmaU32) - 1;

// ── I/O Operation Codes ───────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum IoUringOp {
    Nop      = 0,
    Read     = 1,
    Write    = 2,
    Fsync    = 3,
    PollAdd  = 4,
    Close    = 5,
    Timeout  = 6,
    Accept   = 7,
    Connect  = 8,
    Recv     = 9,
    Send     = 10,
    Splice   = 11,
    OpenAt   = 12,
    Statx    = 13,
}

// ── Submission Queue Entry ────────────────────────────────────────────────────
/// Maps closely to Linux `struct io_uring_sqe` (64 bytes).
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SqEntry {
    /// Operation code (IoUringOp).
    pub opcode:      SigmaU8,
    /// Flags (IOSQE_* bitmask).
    pub flags:       SigmaU8,
    /// I/O priority (used by IO scheduler).
    pub ioprio:      SigmaU16,
    /// File descriptor.
    pub fd:          SigmaI32,
    /// Offset within file (for Read/Write).
    pub off:         SigmaU64,
    /// Buffer address (user virtual).
    pub addr:        SigmaU64,
    /// Length of buffer / data.
    pub len:         SigmaU32,
    /// Op-specific flags (e.g., O_SYNC for Fsync).
    pub op_flags:    SigmaU32,
    /// User data — echoed back in CqEntry (for request correlation).
    pub user_data:   SigmaU64,
    pub _pad:        [SigmaU8; 16],
}

impl SqEntry {
    pub const fn zeroed() -> Self {
        Self {
            opcode: 0, flags: 0, ioprio: 0, fd: -1,
            off: 0, addr: 0, len: 0, op_flags: 0,
            user_data: 0, _pad: [0u8; 16],
        }
    }
}

// ── Completion Queue Entry ────────────────────────────────────────────────────
/// Maps closely to Linux `struct io_uring_cqe` (16 bytes).
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CqEntry {
    /// User data from corresponding SqEntry — correlates response to request.
    pub user_data: SigmaU64,
    /// Result: bytes transferred, or negative errno on error.
    pub res:       SigmaI32,
    /// CQE flags (IORING_CQE_F_* bitmask).
    pub flags:     SigmaU32,
}

impl CqEntry {
    pub const fn zeroed() -> Self {
        Self { user_data: 0, res: 0, flags: 0 }
    }
}

// ── IOSQE flags ───────────────────────────────────────────────────────────────
const IOSQE_FIXED_FILE:    SigmaU8 = 1 << 0;
const IOSQE_IO_DRAIN:      SigmaU8 = 1 << 1;
const IOSQE_IO_LINK:       SigmaU8 = 1 << 2;
const IOSQE_IO_HARDLINK:   SigmaU8 = 1 << 3;

// ── IoUringRing — the full ring state ────────────────────────────────────────
pub struct IoUringRing {
    // Submission queue
    sq:      [SqEntry; SQ_RING_SIZE],
    sq_head: AtomicU32, // kernel consumer head
    sq_tail: AtomicU32, // userspace producer tail
    // Completion queue
    cq:      [CqEntry; CQ_RING_SIZE],
    cq_head: AtomicU32, // userspace consumer head
    cq_tail: AtomicU32, // kernel producer tail
    // Ring metadata
    initialized: SigmaBool,
    pending:     AtomicU32,
    completed:   AtomicU32,
    dropped:     AtomicU32,
    
    // Fixed file table
    fixed_files: [SigmaI32; 64],
}

impl IoUringRing {
    pub const fn new() -> Self {
        Self {
            sq:      [SqEntry::zeroed(); SQ_RING_SIZE],
            sq_head: AtomicU32::new(0),
            sq_tail: AtomicU32::new(0),
            cq:      [CqEntry::zeroed(); CQ_RING_SIZE],
            cq_head: AtomicU32::new(0),
            cq_tail: AtomicU32::new(0),
            initialized: false,
            pending:   AtomicU32::new(0),
            completed: AtomicU32::new(0),
            dropped:   AtomicU32::new(0),
            fixed_files: [-1; 64],
        }
    }

    pub fn init(&mut self) { self.initialized = true; }

    // ── Submission API ────────────────────────────────────────────────────────

    /// Submit an SQE to the ring. Returns 0 on success, -1 if full.
    pub fn submit(&mut self, sqe: &SqEntry) -> SigmaI32 {
        if !self.initialized { return -1; }
        let tail = self.sq_tail.load(Ordering::Acquire);
        let head = self.sq_head.load(Ordering::Acquire);
        // Full check.
        if (tail - head) as SigmaUsize >= SQ_RING_SIZE {
            self.dropped.fetch_add(1, Ordering::Relaxed);
            return -1;
        }
        let idx = (tail & SQ_MASK) as SigmaUsize;
        unsafe { core::ptr::write_volatile(&mut self.sq[idx] as *mut SqEntry, *sqe); }
        self.sq_tail.fetch_add(1, Ordering::Release);
        self.pending.fetch_add(1, Ordering::Relaxed);
        0
    }

    // ── Dispatch Loop (kernel side) ───────────────────────────────────────────

    /// Process all pending SQEs and write CQEs.
    /// In the real kernel this runs in a dedicated kthread.
    /// Here it is called synchronously on `submit_and_wait()`.
    pub unsafe fn dispatch(&mut self) {
        loop {
            let head = self.sq_head.load(Ordering::Acquire);
            let tail = self.sq_tail.load(Ordering::Acquire);
            if head == tail { break; }

            let idx = (head & SQ_MASK) as SigmaUsize;
            let sqe = core::ptr::read_volatile(&self.sq[idx] as *const SqEntry);
            self.sq_head.fetch_add(1, Ordering::Release);

            let res = self.execute_op(&sqe);
            self.push_cqe(sqe.user_data, res, 0);
        }
    }

    /// Execute a single SQE and return the result (bytes done / errno).
    unsafe fn execute_op(&self, sqe: &SqEntry) -> SigmaI32 {
        let fd = if (sqe.flags & IOSQE_FIXED_FILE) != 0 {
            if sqe.fd >= 0 && (sqe.fd as usize) < self.fixed_files.len() {
                self.fixed_files[sqe.fd as usize]
            } else {
                return -9; // EBADF
            }
        } else {
            sqe.fd
        };

        match sqe.opcode {
            0 => 0, // Nop: always success
            1 => {  // Read
                extern "C" {
                    fn sigma_fd_read(fd: SigmaI32, buf: *mut SigmaU8, len: SigmaU32, off: SigmaU64) -> SigmaI32;
                }
                sigma_fd_read(fd, sqe.addr as *mut SigmaU8, sqe.len, sqe.off)
            }
            2 => {  // Write
                extern "C" {
                    fn sigma_fd_write(fd: SigmaI32, buf: *const SigmaU8, len: SigmaU32, off: SigmaU64) -> SigmaI32;
                }
                sigma_fd_write(fd, sqe.addr as *const SigmaU8, sqe.len, sqe.off)
            }
            3 => {  // Fsync
                extern "C" { fn sigma_fd_fsync(fd: SigmaI32) -> SigmaI32; }
                sigma_fd_fsync(fd)
            }
            5 => {  // Close
                extern "C" { fn sigma_fd_close(fd: SigmaI32) -> SigmaI32; }
                sigma_fd_close(fd)
            }
            7 => {  // Accept
                extern "C" { fn sigma_net_accept(fd: SigmaI32) -> SigmaI32; }
                sigma_net_accept(fd)
            }
            8 => {  // Connect
                extern "C" { fn sigma_net_connect(fd: SigmaI32, addr: *const SigmaU8, addrlen: SigmaU32) -> SigmaI32; }
                sigma_net_connect(fd, sqe.addr as *const SigmaU8, sqe.len)
            }
            9 => {  // Recv
                extern "C" { fn sigma_net_recv(fd: SigmaI32, buf: *mut SigmaU8, len: SigmaU32, flags: SigmaI32) -> SigmaI32; }
                sigma_net_recv(fd, sqe.addr as *mut SigmaU8, sqe.len, sqe.op_flags as SigmaI32)
            }
            10 => { // Send
                extern "C" { fn sigma_net_send(fd: SigmaI32, buf: *const SigmaU8, len: SigmaU32, flags: SigmaI32) -> SigmaI32; }
                sigma_net_send(fd, sqe.addr as *const SigmaU8, sqe.len, sqe.op_flags as SigmaI32)
            }
            _ => -38, // ENOSYS
        }
    }

    /// Push a completion event into the CQ.
    fn push_cqe(&mut self, user_data: SigmaU64, res: SigmaI32, flags: SigmaU32) {
        let tail = self.cq_tail.load(Ordering::Acquire);
        let head = self.cq_head.load(Ordering::Acquire);
        if (tail - head) as SigmaUsize >= CQ_RING_SIZE {
            // CQ overflow — drop.
            self.dropped.fetch_add(1, Ordering::Relaxed);
            return;
        }
        let idx = (tail & CQ_MASK) as SigmaUsize;
        let cqe = CqEntry { user_data, res, flags };
        unsafe { core::ptr::write_volatile(&mut self.cq[idx] as *mut CqEntry, cqe); }
        self.cq_tail.fetch_add(1, Ordering::Release);
        self.completed.fetch_add(1, Ordering::Relaxed);
        self.pending.fetch_sub(1, Ordering::Relaxed);
    }

    // ── Completion API ────────────────────────────────────────────────────────

    /// Consume one CQE. Returns true if a CQE was available, false if empty.
    pub fn consume_cqe(&mut self, out: *mut CqEntry) -> SigmaBool {
        let head = self.cq_head.load(Ordering::Acquire);
        let tail = self.cq_tail.load(Ordering::Acquire);
        if head == tail { return false; }
        let idx = (head & CQ_MASK) as SigmaUsize;
        unsafe {
            core::ptr::write(out, core::ptr::read_volatile(&self.cq[idx] as *const CqEntry));
        }
        self.cq_head.fetch_add(1, Ordering::Release);
        true
    }

    // ── Stats ─────────────────────────────────────────────────────────────────
    pub fn pending_count(&self)   -> SigmaU32 { self.pending.load(Ordering::Relaxed) }
    pub fn completed_count(&self) -> SigmaU32 { self.completed.load(Ordering::Relaxed) }
    pub fn dropped_count(&self)   -> SigmaU32 { self.dropped.load(Ordering::Relaxed) }
}

// ── Global Instance ───────────────────────────────────────────────────────────
static mut G_IO_RING: IoUringRing = IoUringRing::new();

// ── C-ABI Exports ─────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_io_uring_init() {
    G_IO_RING.init();
}

/// Submit one SQE. Returns 0 on success, -1 if queue is full.
#[no_mangle]
pub unsafe extern "C" fn sigma_io_uring_submit(sqe: *const SqEntry) -> SigmaI32 {
    if sqe.is_null() { return -22; } // EINVAL
    G_IO_RING.submit(&*sqe)
}

/// Submit and synchronously dispatch all pending SQEs.
/// Returns number of SQEs processed.
#[no_mangle]
pub unsafe extern "C" fn sigma_io_uring_enter(to_submit: SigmaU32, _min_complete: SigmaU32) -> SigmaU32 {
    for _ in 0..to_submit {
        G_IO_RING.dispatch();
    }
    G_IO_RING.completed_count()
}

/// Consume one CQE into `out`. Returns 1 if got one, 0 if empty.
#[no_mangle]
pub unsafe extern "C" fn sigma_io_uring_cqe_peek(out: *mut CqEntry) -> SigmaU32 {
    if G_IO_RING.consume_cqe(out) { 1 } else { 0 }
}

/// Returns count of pending (submitted but not yet completed) SQEs.
#[no_mangle]
pub unsafe extern "C" fn sigma_io_uring_pending() -> SigmaU32 {
    G_IO_RING.pending_count()
}

/// Returns total completed operations count.
#[no_mangle]
pub unsafe extern "C" fn sigma_io_uring_completed() -> SigmaU32 {
    G_IO_RING.completed_count()
}

/// Returns total dropped (overflow) count.
#[no_mangle]
pub unsafe extern "C" fn sigma_io_uring_dropped() -> SigmaU32 {
    G_IO_RING.dropped_count()
}

/// Pointer to the SQ entries array (for zero-copy userspace mapping).
#[no_mangle]
pub unsafe extern "C" fn sigma_io_uring_sq_ptr() -> *const SqEntry {
    G_IO_RING.sq.as_ptr()
}

/// Pointer to the CQ entries array (for zero-copy userspace mapping).
#[no_mangle]
pub unsafe extern "C" fn sigma_io_uring_cq_ptr() -> *const CqEntry {
    G_IO_RING.cq.as_ptr()
}

/// Register an array of fixed file descriptors
#[no_mangle]
pub unsafe extern "C" fn sigma_io_uring_register_files(fds: *const SigmaI32, count: SigmaU32) -> SigmaI32 {
    if fds.is_null() || count > 64 { return -22; } // EINVAL
    let slice = core::slice::from_raw_parts(fds, count as usize);
    for i in 0..count as usize {
        G_IO_RING.fixed_files[i] = slice[i];
    }
    0
}

/// Trigger SQPOLL (Submit without enter)
#[no_mangle]
pub unsafe extern "C" fn sigma_io_uring_sqpoll_wakeup() {
    // In a real kernel this would wake up the SQPOLL kthread.
    // Here we just synchronously dispatch pending.
    G_IO_RING.dispatch();
}
