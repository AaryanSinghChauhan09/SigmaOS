//! # Wait Queue Subsystem
//!
//! Linux-inspired wait queues for blocking/waking processes.
//! Used throughout kernel for synchronization and event notification.

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Wait queue entry flags
#[derive(Debug, Clone, Copy)]
pub struct WaitFlags {
    /// Exclusive wait (only one waiter woken)
    pub exclusive: bool,
    /// Interruptible wait (can be interrupted by signals)
    pub interruptible: bool,
    /// Non-blocking (return immediately if condition not met)
    pub non_blocking: bool,
}

impl WaitFlags {
    pub const DEFAULT: Self = Self {
        exclusive: false,
        interruptible: true,
        non_blocking: false,
    };

    pub const EXCLUSIVE: Self = Self {
        exclusive: true,
        interruptible: true,
        non_blocking: false,
    };

    pub const UNINTERRUPTIBLE: Self = Self {
        exclusive: false,
        interruptible: false,
        non_blocking: false,
    };
}

/// Wait queue entry state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WaitState {
    /// Waiting for condition
    Waiting = 0,
    /// Woken up
    Woken = 1,
    /// Timed out
    Timeout = 2,
    /// Interrupted by signal
    Interrupted = 3,
}

/// Wait queue entry (inspired by Linux wait_queue_entry)
pub struct WaitQueueEntry {
    /// Task/thread ID
    pub tid: u32,
    /// Wait flags
    pub flags: WaitFlags,
    /// Current state
    pub state: AtomicU32,
    /// Is this entry on the queue
    pub on_queue: AtomicBool,
}

impl WaitQueueEntry {
    pub fn new(tid: u32, flags: WaitFlags) -> Self {
        Self {
            tid,
            flags,
            state: AtomicU32::new(WaitState::Waiting as u32),
            on_queue: AtomicBool::new(false),
        }
    }

    /// Get wait state
    pub fn state(&self) -> WaitState {
        match self.state.load(Ordering::Acquire) {
            0 => WaitState::Waiting,
            1 => WaitState::Woken,
            2 => WaitState::Timeout,
            3 => WaitState::Interrupted,
            _ => WaitState::Waiting,
        }
    }

    /// Set wait state
    pub fn set_state(&self, state: WaitState) {
        self.state.store(state as u32, Ordering::Release);
    }

    /// Wake this entry
    pub fn wake(&self) {
        self.set_state(WaitState::Woken);
    }

    /// Check if woken
    pub fn is_woken(&self) -> bool {
        self.state() == WaitState::Woken
    }
}

/// Wait queue head (inspired by Linux wait_queue_head_t)
pub struct WaitQueue {
    /// List of waiting entries
    entries: Vec<WaitQueueEntry>,
    /// Spinlock for queue access
    lock: AtomicU32,
}

impl WaitQueue {
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
            lock: AtomicU32::new(0),
        }
    }

    /// Add entry to wait queue
    pub fn add(&mut self, entry: WaitQueueEntry) {
        entry.on_queue.store(true, Ordering::Release);
        self.entries.push(entry);
    }

    /// Remove entry from wait queue
    pub fn remove(&mut self, tid: u32) -> bool {
        if let Some(pos) = self.entries.iter().position(|e| e.tid == tid) {
            let entry = self.entries.remove(pos);
            entry.on_queue.store(false, Ordering::Release);
            true
        } else {
            false
        }
    }

    /// Wake one waiter
    pub fn wake_one(&mut self) -> bool {
        for entry in &self.entries {
            if entry.state() == WaitState::Waiting {
                entry.wake();
                return true;
            }
        }
        false
    }

    /// Wake all waiters
    pub fn wake_all(&mut self) -> usize {
        let mut woken = 0;
        for entry in &self.entries {
            if entry.state() == WaitState::Waiting {
                entry.wake();
                woken += 1;
            }
        }
        woken
    }

    /// Wake exclusive waiters (wake one if any exclusive, otherwise all)
    pub fn wake_exclusive(&mut self) -> usize {
        // Wake one exclusive waiter
        for entry in &self.entries {
            if entry.flags.exclusive && entry.state() == WaitState::Waiting {
                entry.wake();
                return 1;
            }
        }

        // No exclusive waiters, wake all non-exclusive
        let mut woken = 0;
        for entry in &self.entries {
            if !entry.flags.exclusive && entry.state() == WaitState::Waiting {
                entry.wake();
                woken += 1;
            }
        }
        woken
    }

    /// Wake up to N waiters
    pub fn wake_up_nr(&mut self, n: usize) -> usize {
        let mut woken = 0;
        for entry in &self.entries {
            if woken >= n {
                break;
            }
            if entry.state() == WaitState::Waiting {
                entry.wake();
                woken += 1;
            }
        }
        woken
    }

    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get number of waiters
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Get number of active waiters (not woken yet)
    pub fn active_waiters(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| e.state() == WaitState::Waiting)
            .count()
    }
}

/// Wait event with condition checking
pub struct WaitEvent {
    /// Wait queue
    queue: WaitQueue,
}

impl WaitEvent {
    pub const fn new() -> Self {
        Self {
            queue: WaitQueue::new(),
        }
    }

    /// Wait until condition is true
    pub fn wait<F>(&mut self, tid: u32, flags: WaitFlags, mut condition: F) -> WaitState
    where
        F: FnMut() -> bool,
    {
        // Check condition first
        if condition() {
            return WaitState::Woken;
        }

        // Add to wait queue
        let entry = WaitQueueEntry::new(tid, flags);
        self.queue.add(entry);

        // Single-pass simulation of the kernel wait loop: a real kernel would
        // schedule out and re-evaluate on wake events; the hosted model checks
        // the waiter state once and reports the result. (The previous
        // `loop { ...; break; }` shape tripped clippy::never_loop while
        // adding no iteration semantics.)
        if let Some(e) = self.queue.entries.iter().find(|e| e.tid == tid) {
            if e.is_woken() {
                self.queue.remove(tid);
                return WaitState::Woken;
            }

            // Re-check condition
            if condition() {
                e.wake();
                self.queue.remove(tid);
                return WaitState::Woken;
            }

            // In real kernel, would yield CPU here
        }

        WaitState::Waiting
    }

    /// Wake all waiters
    pub fn wake_all(&mut self) -> usize {
        self.queue.wake_all()
    }

    /// Wake one waiter
    pub fn wake_one(&mut self) -> bool {
        self.queue.wake_one()
    }
}

/// Completion primitive (inspired by Linux completion)
pub struct Completion {
    /// Is completion done
    done: AtomicBool,
    /// Wait queue for waiters
    wait_queue: WaitQueue,
}

impl Completion {
    pub const fn new() -> Self {
        Self {
            done: AtomicBool::new(false),
            wait_queue: WaitQueue::new(),
        }
    }

    /// Wait for completion
    pub fn wait(&mut self, tid: u32) {
        if self.done.load(Ordering::Acquire) {
            return;
        }

        let entry = WaitQueueEntry::new(tid, WaitFlags::UNINTERRUPTIBLE);
        self.wait_queue.add(entry);

        // Wait loop (in real kernel, would schedule)
        while !self.done.load(Ordering::Acquire) {
            // Would yield here
        }

        self.wait_queue.remove(tid);
    }

    /// Mark completion as done
    pub fn complete(&mut self) {
        self.done.store(true, Ordering::Release);
        self.wait_queue.wake_all();
    }

    /// Mark completion as done and wake all
    pub fn complete_all(&mut self) {
        self.complete();
    }

    /// Check if completed
    pub fn is_done(&self) -> bool {
        self.done.load(Ordering::Acquire)
    }

    /// Reset completion
    pub fn reinit(&mut self) {
        self.done.store(false, Ordering::Release);
    }
}

/// Wait queue timeout
pub struct WaitTimeout {
    /// Timeout in ticks
    pub timeout_ticks: u64,
    /// Start time
    pub start_tick: u64,
}

impl WaitTimeout {
    pub fn new(timeout_ticks: u64, current_tick: u64) -> Self {
        Self {
            timeout_ticks,
            start_tick: current_tick,
        }
    }

    /// Check if timed out
    pub fn is_expired(&self, current_tick: u64) -> bool {
        current_tick >= self.start_tick + self.timeout_ticks
    }

    /// Get remaining ticks
    pub fn remaining(&self, current_tick: u64) -> u64 {
        let elapsed = current_tick.saturating_sub(self.start_tick);
        self.timeout_ticks.saturating_sub(elapsed)
    }
}

#[cfg(test)]
#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_wait_queue_entry() {
        let entry = WaitQueueEntry::new(100, WaitFlags::DEFAULT);
        assert_eq!(entry.state(), WaitState::Waiting);
        entry.wake();
        assert!(entry.is_woken());
    }

    #[test]
    fn test_wait_queue_wake() {
        let mut wq = WaitQueue::new();
        wq.add(WaitQueueEntry::new(1, WaitFlags::DEFAULT));
        wq.add(WaitQueueEntry::new(2, WaitFlags::DEFAULT));

        assert!(wq.wake_one());
        assert_eq!(wq.active_waiters(), 1);

        let woken = wq.wake_all();
        assert_eq!(woken, 1);
    }

    #[test]
    fn test_wait_queue_exclusive() {
        let mut wq = WaitQueue::new();
        wq.add(WaitQueueEntry::new(1, WaitFlags::EXCLUSIVE));
        wq.add(WaitQueueEntry::new(2, WaitFlags::DEFAULT));

        let woken = wq.wake_exclusive();
        assert_eq!(woken, 1); // Only exclusive waiter woken
    }

    #[test]
    fn test_completion() {
        let mut comp = Completion::new();
        assert!(!comp.is_done());

        comp.complete();
        assert!(comp.is_done());

        comp.reinit();
        assert!(!comp.is_done());
    }

    #[test]
    fn test_wait_timeout() {
        let timeout = WaitTimeout::new(100, 0);
        assert!(!timeout.is_expired(50));
        assert!(timeout.is_expired(100));
        assert_eq!(timeout.remaining(50), 50);
    }
}
