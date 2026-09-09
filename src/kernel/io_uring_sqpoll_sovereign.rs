#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unexpected_cfgs)]
#![allow(clippy::new_without_default)]

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

// ─── io_uring SQE Opcode ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoUringOp {
    Nop,
    Readv,
    Writev,
    Fsync,
    PollAdd,
    Sendmsg,
    Recvmsg,
}

// ─── Submission Queue Entry (SQE) ─────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct SovereignSqe {
    pub opcode: IoUringOp,
    pub fd: i32,
    pub addr: u64,
    pub len: u32,
    pub user_data: u64,
}

// ─── Completion Queue Entry (CQE) ─────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct SovereignCqe {
    pub user_data: u64,
    pub res: i32, // Result code (bytes processed or -errno)
    pub flags: u32,
}

// ─── Sovereign SQPOLL Controller ──────────────────────────────────────────────

pub struct SovereignIoUringSqpoll {
    pub ring_entries: usize,
    pub sq_entries: Vec<SovereignSqe>,
    pub cq_entries: Vec<SovereignCqe>,
    pub sq_thread_idle_ticks: u32,
    pub idle_timeout_ticks: u32,
    pub thread_is_sleeping: bool,
    pub total_sqes_polled: u64,
    pub total_zero_syscall_ops: u64,
}

impl SovereignIoUringSqpoll {
    pub fn new(ring_entries: usize, idle_timeout_ticks: u32) -> Self {
        SovereignIoUringSqpoll {
            ring_entries,
            sq_entries: Vec::new(),
            cq_entries: Vec::new(),
            sq_thread_idle_ticks: 0,
            idle_timeout_ticks,
            thread_is_sleeping: false,
            total_sqes_polled: 0,
            total_zero_syscall_ops: 0,
        }
    }

    /// User writes SQE to ring buffer without system call
    pub fn submit_sqe(&mut self, sqe: SovereignSqe) -> bool {
        if self.sq_entries.len() >= self.ring_entries {
            return false;
        }
        self.sq_entries.push(sqe);
        true
    }

    /// Check if user must issue wakeup
    pub fn needs_wakeup(&self) -> bool {
        self.thread_is_sleeping
    }

    /// Wake up sleeping SQPOLL thread
    pub fn wakeup_thread(&mut self) {
        self.thread_is_sleeping = false;
        self.sq_thread_idle_ticks = 0;
    }

    /// Kernel SQPOLL worker thread polling cycle
    pub fn poll_cycle(&mut self) -> usize {
        if self.thread_is_sleeping {
            return 0;
        }

        if self.sq_entries.is_empty() {
            self.sq_thread_idle_ticks = self.sq_thread_idle_ticks.saturating_add(1);
            if self.sq_thread_idle_ticks >= self.idle_timeout_ticks {
                self.thread_is_sleeping = true;
            }
            return 0;
        }

        // Reset idle counter on active work
        self.sq_thread_idle_ticks = 0;
        let batch: Vec<SovereignSqe> = self.sq_entries.drain(..).collect();
        let batch_size = batch.len();

        for sqe in batch {
            self.total_sqes_polled = self.total_sqes_polled.saturating_add(1);
            self.total_zero_syscall_ops = self.total_zero_syscall_ops.saturating_add(1);

            // Execute SQE operation
            let res = match sqe.opcode {
                IoUringOp::Nop => 0,
                IoUringOp::Readv => sqe.len as i32,
                IoUringOp::Writev => sqe.len as i32,
                IoUringOp::Fsync => 0,
                IoUringOp::PollAdd => 1,
                IoUringOp::Sendmsg => sqe.len as i32,
                IoUringOp::Recvmsg => sqe.len as i32,
            };

            // Post CQE
            self.cq_entries.push(SovereignCqe {
                user_data: sqe.user_data,
                res,
                flags: 0,
            });
        }

        batch_size
    }

    /// User reaps CQE without system call
    pub fn reap_cqe(&mut self) -> Option<SovereignCqe> {
        if self.cq_entries.is_empty() {
            None
        } else {
            Some(self.cq_entries.remove(0))
        }
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqpoll_submission_and_completion() {
        let mut ring = SovereignIoUringSqpoll::new(16, 5);
        let sqe = SovereignSqe {
            opcode: IoUringOp::Readv,
            fd: 3,
            addr: 0x1000,
            len: 4096,
            user_data: 0xCAFE,
        };
        assert!(ring.submit_sqe(sqe));

        let processed = ring.poll_cycle();
        assert_eq!(processed, 1);
        assert_eq!(ring.total_zero_syscall_ops, 1);

        let cqe = ring.reap_cqe().unwrap();
        assert_eq!(cqe.user_data, 0xCAFE);
        assert_eq!(cqe.res, 4096);
    }

    #[test]
    fn test_sqpoll_idle_sleep_transition() {
        let mut ring = SovereignIoUringSqpoll::new(8, 3);
        assert!(!ring.needs_wakeup());

        // 3 empty poll cycles
        ring.poll_cycle();
        ring.poll_cycle();
        ring.poll_cycle();

        assert!(ring.needs_wakeup()); // Thread went to sleep
        assert_eq!(ring.poll_cycle(), 0);

        ring.wakeup_thread();
        assert!(!ring.needs_wakeup());
    }

    #[test]
    fn test_sqpoll_ring_capacity_limit() {
        let mut ring = SovereignIoUringSqpoll::new(2, 5);
        let sqe = SovereignSqe { opcode: IoUringOp::Nop, fd: 0, addr: 0, len: 0, user_data: 1 };
        assert!(ring.submit_sqe(sqe.clone()));
        assert!(ring.submit_sqe(sqe.clone()));
        assert!(!ring.submit_sqe(sqe)); // Exceeds ring capacity
    }

    #[test]
    fn test_sqpoll_batch_processing() {
        let mut ring = SovereignIoUringSqpoll::new(16, 10);
        for i in 0..5 {
            ring.submit_sqe(SovereignSqe {
                opcode: IoUringOp::Writev,
                fd: 1,
                addr: 0,
                len: 128,
                user_data: i,
            });
        }
        let processed = ring.poll_cycle();
        assert_eq!(processed, 5);
        assert_eq!(ring.cq_entries.len(), 5);
    }

    #[test]
    fn test_sqpoll_reap_empty() {
        let mut ring = SovereignIoUringSqpoll::new(8, 5);
        assert!(ring.reap_cqe().is_none());
    }

    #[test]
    fn test_sqpoll_nop_operation() {
        let mut ring = SovereignIoUringSqpoll::new(8, 5);
        ring.submit_sqe(SovereignSqe { opcode: IoUringOp::Nop, fd: 0, addr: 0, len: 0, user_data: 42 });
        ring.poll_cycle();
        let cqe = ring.reap_cqe().unwrap();
        assert_eq!(cqe.user_data, 42);
        assert_eq!(cqe.res, 0);
    }
}
