// SigmaOS POSIX Real-Time Signals Subsystem
// Implements queued real-time signal delivery (SIGRTMIN..SIGRTMAX) with sigval payloads.

#![allow(dead_code)]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

pub const SIGRTMIN: i32 = 34;
pub const SIGRTMAX: i32 = 64;
pub const MAX_QUEUED_RT_SIGNALS: usize = 64;

/// Associated sigval union payload
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SigVal {
    Int(i32),
    Ptr(u64),
}

/// An individual queued real-time signal instance
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueuedRtSignal {
    pub signo: i32,
    pub sender_pid: u32,
    pub sender_uid: u32,
    pub value: SigVal,
    pub timestamp_us: u64,
}

/// Process Real-Time Signal Queue
#[derive(Debug, Clone, Default)]
pub struct RtSignalQueue {
    pub pending: Vec<QueuedRtSignal>,
    pub max_capacity: usize,
}

impl RtSignalQueue {
    pub fn new() -> Self {
        Self {
            pending: Vec::new(),
            max_capacity: MAX_QUEUED_RT_SIGNALS,
        }
    }

    /// Enqueue a real-time signal into the process queue in FIFO order for same signal number,
    /// and ascending signal number order (lower numbered RT signals delivered first)
    pub fn enqueue(
        &mut self,
        signo: i32,
        sender_pid: u32,
        sender_uid: u32,
        value: SigVal,
        now_us: u64,
    ) -> Result<(), &'static str> {
        if signo < SIGRTMIN || signo > SIGRTMAX {
            return Err("Signal number outside POSIX real-time signal range [SIGRTMIN..SIGRTMAX]");
        }

        if self.pending.len() >= self.max_capacity {
            return Err("Process real-time signal queue is full (EAGAIN)");
        }

        let sig = QueuedRtSignal {
            signo,
            sender_pid,
            sender_uid,
            value,
            timestamp_us: now_us,
        };

        // Insert maintaining signal priority (lower signo delivered first)
        let insert_idx = self
            .pending
            .iter()
            .position(|s| s.signo > signo)
            .unwrap_or(self.pending.len());

        self.pending.insert(insert_idx, sig);
        Ok(())
    }

    /// Dequeue the highest priority pending real-time signal (lowest signal number, FIFO)
    pub fn dequeue(&mut self) -> Option<QueuedRtSignal> {
        if self.pending.is_empty() {
            None
        } else {
            Some(self.pending.remove(0))
        }
    }

    /// Peek at the next pending real-time signal without removing it
    pub fn peek(&self) -> Option<&QueuedRtSignal> {
        self.pending.first()
    }

    /// Number of pending queued signals
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
}

/// System-wide registry of per-process RT signal queues
#[derive(Debug, Clone, Default)]
pub struct ProcessRtSignalManager {
    pub queues: BTreeMap<u32, RtSignalQueue>,
}

impl ProcessRtSignalManager {
    pub fn new() -> Self {
        Self {
            queues: BTreeMap::new(),
        }
    }

    /// Send a real-time signal to a target process
    pub fn sigqueue(
        &mut self,
        target_pid: u32,
        signo: i32,
        sender_pid: u32,
        sender_uid: u32,
        value: SigVal,
        now_us: u64,
    ) -> Result<(), &'static str> {
        let queue = self
            .queues
            .entry(target_pid)
            .or_insert_with(RtSignalQueue::new);
        queue.enqueue(signo, sender_pid, sender_uid, value, now_us)
    }

    /// Consume next signal for a target process
    pub fn deliver_next(&mut self, target_pid: u32) -> Option<QueuedRtSignal> {
        let queue = self.queues.get_mut(&target_pid)?;
        queue.dequeue()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_posix_rt_signal_queueing_and_priority() {
        let mut queue = RtSignalQueue::new();

        // Enqueue RT signal 40
        assert!(queue.enqueue(40, 10, 0, SigVal::Int(100), 1000).is_ok());

        // Enqueue RT signal 35 (higher priority than 40)
        assert!(queue.enqueue(35, 11, 0, SigVal::Int(200), 1001).is_ok());

        // Enqueue another RT signal 35 (FIFO with respect to first signal 35)
        assert!(queue.enqueue(35, 12, 0, SigVal::Int(300), 1002).is_ok());

        assert_eq!(queue.pending_count(), 3);

        // First dequeued must be the first signal 35
        let first = queue.dequeue().expect("Signal 35-A");
        assert_eq!(first.signo, 35);
        assert_eq!(first.value, SigVal::Int(200));

        // Second dequeued must be the second signal 35
        let second = queue.dequeue().expect("Signal 35-B");
        assert_eq!(second.signo, 35);
        assert_eq!(second.value, SigVal::Int(300));

        // Third dequeued must be signal 40
        let third = queue.dequeue().expect("Signal 40");
        assert_eq!(third.signo, 40);
        assert_eq!(third.value, SigVal::Int(100));
    }
}
