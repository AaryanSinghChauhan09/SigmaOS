// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS io_uring-style Async I/O Ring
//! Submission Queue (SQ) + Completion Queue (CQ) ring buffers.
//! Single-producer single-consumer. no_std, no alloc.

#![no_std]
#![allow(dead_code)]

type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;

pub const URING_RING_SIZE: usize = 256;  // must be power of 2
const RING_MASK: usize = URING_RING_SIZE - 1;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum UringOp {
    Nop    = 0,
    Read   = 1,
    Write  = 2,
    Fsync  = 3,
    Accept = 4,
    Recv   = 5,
    Send   = 6,
}

/// Submission Queue Entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Sqe {
    pub op:      UringOp,
    pub fd:      SigmaI32,
    pub buf:     *mut u8,
    pub len:     SigmaU32,
    pub offset:  SigmaU64,
    pub user_data: SigmaU64,  // caller-defined tag echoed in CQE
}

/// Completion Queue Entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Cqe {
    pub user_data: SigmaU64,
    pub result:    SigmaI32,  // bytes transferred or negative errno
    pub flags:     SigmaU32,
}

static mut SQ_RING: [Sqe; URING_RING_SIZE] = [Sqe {
    op: UringOp::Nop, fd: 0, buf: core::ptr::null_mut(), len: 0, offset: 0, user_data: 0,
}; URING_RING_SIZE];
static mut SQ_HEAD: usize = 0;  // kernel consumes from head
static mut SQ_TAIL: usize = 0;  // user pushes to tail

static mut CQ_RING: [Cqe; URING_RING_SIZE] = [Cqe { user_data: 0, result: 0, flags: 0 }; URING_RING_SIZE];
static mut CQ_HEAD: usize = 0;  // user reads from head
static mut CQ_TAIL: usize = 0;  // kernel posts to tail

#[no_mangle]
pub unsafe extern "C" fn sigma_uring_init() {
    SQ_HEAD = 0; SQ_TAIL = 0;
    CQ_HEAD = 0; CQ_TAIL = 0;
}

/// Submit an SQE. Returns 0 on success, -1 if the ring is full.
#[no_mangle]
pub unsafe extern "C" fn sigma_uring_submit(sqe: *const Sqe) -> SigmaI32 {
    if sqe.is_null() { return -1; }
    let next = (SQ_TAIL + 1) & RING_MASK;
    if next == SQ_HEAD { return -1; }  // ring full
    SQ_RING[SQ_TAIL & RING_MASK] = *sqe;
    SQ_TAIL = next;
    0
}

/// Process all pending SQEs and post CQEs.
/// In real implementation this runs in the kernel I/O thread.
#[no_mangle]
pub unsafe extern "C" fn sigma_uring_process() {
    while SQ_HEAD != SQ_TAIL {
        let sqe = &SQ_RING[SQ_HEAD & RING_MASK];
        // Simulate I/O dispatch — real impl calls VFS read/write here
        let result: SigmaI32 = match sqe.op {
            UringOp::Read  => sqe.len as SigmaI32,   // pretend we read all bytes
            UringOp::Write => sqe.len as SigmaI32,   // pretend we wrote all bytes
            UringOp::Fsync => 0,
            _ => -38,  // ENOSYS
        };
        // Post CQE
        let cq_next = (CQ_TAIL + 1) & RING_MASK;
        if cq_next != CQ_HEAD {
            CQ_RING[CQ_TAIL & RING_MASK] = Cqe { user_data: sqe.user_data, result, flags: 0 };
            CQ_TAIL = cq_next;
        }
        SQ_HEAD = (SQ_HEAD + 1) & RING_MASK;
    }
}

/// Dequeue one CQE. Returns 1 if a CQE was available, 0 otherwise.
#[no_mangle]
pub unsafe extern "C" fn sigma_uring_cq_dequeue(out: *mut Cqe) -> SigmaI32 {
    if CQ_HEAD == CQ_TAIL { return 0; }
    if !out.is_null() { *out = CQ_RING[CQ_HEAD & RING_MASK]; }
    CQ_HEAD = (CQ_HEAD + 1) & RING_MASK;
    1
}

/// Pending SQE count.
#[no_mangle]
pub unsafe extern "C" fn sigma_uring_sq_pending() -> SigmaU32 {
    ((SQ_TAIL.wrapping_sub(SQ_HEAD)) & RING_MASK) as SigmaU32
}
