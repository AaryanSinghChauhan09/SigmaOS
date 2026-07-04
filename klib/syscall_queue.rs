// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: syscall_queue - io_uring-style async syscall queue
//! Hand-rolled zero-dependency implementation, no_std, no pre-defined libraries/functions
//! =========================================================================

#![no_std]

/// Syscall operation type
#[derive(Debug, Clone, Copy)]
pub enum SyscallOp {
    Read,
    Write,
    Open,
    Close,
}

/// Syscall submission queue entry (SQE)
#[derive(Debug, Clone, Copy)]
pub struct SyscallSQE {
    op: SyscallOp,
    user_data: u64,
    arg1: u64,
    arg2: u64,
    arg3: u64,
    completed: bool,
}

impl SyscallSQE {
    pub const fn new(op: SyscallOp, user_data: u64, arg1: u64, arg2: u64, arg3: u64) -> Self {
        Self {
            op,
            user_data,
            arg1,
            arg2,
            arg3,
            completed: false,
        }
    }
}

/// Completion queue entry (CQE)
#[derive(Debug, Clone, Copy)]
pub struct SyscallCQE {
    user_data: u64,
    result: i64,
}

/// Async syscall queue
pub struct SyscallQueue {
    sq: [Option<SyscallSQE>; 64],
    cq: [Option<SyscallCQE>; 64],
    sq_tail: usize,
    sq_head: usize,
    cq_tail: usize,
    cq_head: usize,
}

impl SyscallQueue {
    pub const fn new() -> Self {
        Self {
            sq: [None; 64],
            cq: [None; 64],
            sq_tail: 0,
            sq_head: 0,
            cq_tail: 0,
            cq_head: 0,
        }
    }

    /// Submit a syscall to the queue
    pub fn submit(&mut self, sqe: SyscallSQE) -> bool {
        let next_tail = (self.sq_tail + 1) % 64;
        if next_tail == self.sq_head {
            return false; // Queue full
        }
        self.sq[self.sq_tail] = Some(sqe);
        self.sq_tail = next_tail;
        true
    }

    /// Dequeue a syscall to process
    pub fn dequeue(&mut self) -> Option<SyscallSQE> {
        if self.sq_head == self.sq_tail {
            return None;
        }
        let sqe = self.sq[self.sq_head].take();
        self.sq_head = (self.sq_head + 1) % 64;
        sqe
    }

    /// Post a completion
    pub fn complete(&mut self, cqe: SyscallCQE) -> bool {
        let next_tail = (self.cq_tail + 1) % 64;
        if next_tail == self.cq_head {
            return false;
        }
        self.cq[self.cq_tail] = Some(cqe);
        self.cq_tail = next_tail;
        true
    }

    /// Read a completion
    pub fn read_completion(&mut self) -> Option<SyscallCQE> {
        if self.cq_head == self.cq_tail {
            return None;
        }
        let cqe = self.cq[self.cq_head].take();
        self.cq_head = (self.cq_head + 1) % 64;
        cqe
    }
}
