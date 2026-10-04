//! # Futex (Fast Userspace Mutex) Subsystem
//!
//! Linux-inspired futex implementation for efficient userspace synchronization.
//! Provides wait/wake primitives for implementing mutexes, condition variables, and semaphores.

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

/// Maximum number of waiters per futex
const MAX_WAITERS: usize = 1024;

/// Futex operation types (inspired by Linux FUTEX_* constants)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum FutexOp {
    /// Wait if *futex == val
    Wait = 0,
    /// Wake up to val waiters
    Wake = 1,
    /// Atomic compare and requeue
    CmpRequeue = 4,
    /// Wake op (combined wake and modify)
    WakeOp = 5,
    /// Lock private futex
    LockPi = 6,
    /// Unlock private futex
    UnlockPi = 7,
    /// Trylock private futex
    TrylockPi = 8,
    /// Wait with bitset mask
    WaitBitset = 9,
    /// Wake with bitset mask
    WakeBitset = 10,
}

/// Futex flags
#[derive(Debug, Clone, Copy)]
pub struct FutexFlags {
    /// Private futex (process-local)
    pub private: bool,
    /// Use realtime clock for timeout
    pub clock_realtime: bool,
}

impl FutexFlags {
    pub const PRIVATE: Self = Self {
        private: true,
        clock_realtime: false,
    };

    pub const SHARED: Self = Self {
        private: false,
        clock_realtime: false,
    };
}

/// Futex error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FutexError {
    /// Invalid argument
    Invalid,
    /// Timeout expired
    Timeout,
    /// Value mismatch (futex changed before wait)
    Again,
    /// Address fault
    Fault,
    /// Operation would block
    WouldBlock,
    /// Permission denied
    PermissionDenied,
}

/// Futex key for identifying unique futex locations
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FutexKey {
    /// Process ID (for private futexes)
    pid: u32,
    /// Virtual address or shared memory offset
    address: usize,
}

impl FutexKey {
    pub const fn new(pid: u32, address: usize) -> Self {
        Self { pid, address }
    }

    pub const fn shared(address: usize) -> Self {
        Self { pid: 0, address }
    }
}

/// Futex waiter entry
#[derive(Debug, Clone)]
pub struct FutexWaiter {
    /// Thread ID waiting
    pub tid: u32,
    /// Bitset mask for selective wakeup
    pub bitset: u32,
    /// Requeue target (for FUTEX_CMP_REQUEUE)
    pub requeue_key: Option<FutexKey>,
}

impl FutexWaiter {
    pub const fn new(tid: u32) -> Self {
        Self {
            tid,
            bitset: 0xFFFFFFFF, // All bits set by default
            requeue_key: None,
        }
    }

    pub const fn with_bitset(tid: u32, bitset: u32) -> Self {
        Self {
            tid,
            bitset,
            requeue_key: None,
        }
    }
}

/// Futex wait queue
pub struct FutexQueue {
    /// Waiters for this futex
    waiters: Vec<FutexWaiter>,
    /// Spinlock for queue access
    lock: AtomicU32,
}

impl FutexQueue {
    pub const fn new() -> Self {
        Self {
            waiters: Vec::new(),
            lock: AtomicU32::new(0),
        }
    }

    /// Add waiter to queue
    pub fn enqueue(&mut self, waiter: FutexWaiter) -> Result<(), FutexError> {
        if self.waiters.len() >= MAX_WAITERS {
            return Err(FutexError::WouldBlock);
        }
        self.waiters.push(waiter);
        Ok(())
    }

    /// Wake up to count waiters matching bitset
    pub fn wake(&mut self, count: usize, bitset: u32) -> usize {
        let mut woken = 0;
        self.waiters.retain(|waiter| {
            if woken < count && (waiter.bitset & bitset) != 0 {
                woken += 1;
                false // Remove from queue (woken)
            } else {
                true // Keep in queue
            }
        });
        woken
    }

    /// Remove specific waiter
    pub fn remove_waiter(&mut self, tid: u32) -> bool {
        if let Some(pos) = self.waiters.iter().position(|w| w.tid == tid) {
            self.waiters.remove(pos);
            true
        } else {
            false
        }
    }

    /// Get number of waiters
    pub fn len(&self) -> usize {
        self.waiters.len()
    }

    pub fn is_empty(&self) -> bool {
        self.waiters.is_empty()
    }

    /// Requeue waiters to another futex
    pub fn requeue(&mut self, count: usize, target_key: FutexKey) -> Vec<FutexWaiter> {
        let mut requeued = Vec::new();
        let mut remaining = Vec::new();

        for (i, mut waiter) in self.waiters.drain(..).enumerate() {
            if i < count {
                waiter.requeue_key = Some(target_key);
                requeued.push(waiter);
            } else {
                remaining.push(waiter);
            }
        }

        self.waiters = remaining;
        requeued
    }
}

/// Global futex hash table (buckets for futex wait queues)
pub struct FutexHashTable {
    /// Hash buckets (using BTreeMap for now, real implementation would use hash table)
    buckets: BTreeMap<FutexKey, FutexQueue>,
    /// Number of active waiters
    active_waiters: AtomicUsize,
}

impl FutexHashTable {
    pub const fn new() -> Self {
        Self {
            buckets: BTreeMap::new(),
            active_waiters: AtomicUsize::new(0),
        }
    }

    /// Get or create queue for futex key
    fn get_or_create_queue(&mut self, key: FutexKey) -> &mut FutexQueue {
        self.buckets.entry(key).or_insert_with(FutexQueue::new)
    }

    /// FUTEX_WAIT operation
    pub fn wait(
        &mut self,
        key: FutexKey,
        expected: u32,
        current: u32,
        tid: u32,
        bitset: u32,
    ) -> Result<(), FutexError> {
        // Check if value matches expected
        if current != expected {
            return Err(FutexError::Again);
        }

        let queue = self.get_or_create_queue(key);
        let waiter = FutexWaiter::with_bitset(tid, bitset);
        queue.enqueue(waiter)?;
        self.active_waiters.fetch_add(1, Ordering::SeqCst);

        Ok(())
    }

    /// FUTEX_WAKE operation
    pub fn wake(&mut self, key: FutexKey, count: usize, bitset: u32) -> usize {
        if let Some(queue) = self.buckets.get_mut(&key) {
            let woken = queue.wake(count, bitset);
            self.active_waiters.fetch_sub(woken, Ordering::SeqCst);

            // Remove empty queues
            if queue.is_empty() {
                self.buckets.remove(&key);
            }

            woken
        } else {
            0
        }
    }

    /// FUTEX_CMP_REQUEUE operation
    pub fn cmp_requeue(
        &mut self,
        key: FutexKey,
        expected: u32,
        current: u32,
        wake_count: usize,
        requeue_count: usize,
        target_key: FutexKey,
    ) -> Result<usize, FutexError> {
        // Check if value matches expected
        if current != expected {
            return Err(FutexError::Again);
        }

        // Wake some waiters
        let woken = if let Some(queue) = self.buckets.get_mut(&key) {
            queue.wake(wake_count, 0xFFFFFFFF)
        } else {
            return Ok(0);
        };

        // Requeue remaining waiters
        let requeued = if let Some(queue) = self.buckets.get_mut(&key) {
            queue.requeue(requeue_count, target_key)
        } else {
            Vec::new()
        };

        let requeued_count = requeued.len();

        // Add requeued waiters to target queue
        let requeued_count = requeued.len();
        if !requeued.is_empty() {
            let target_queue = self.get_or_create_queue(target_key);
            for waiter in requeued {
                let _ = target_queue.enqueue(waiter);
            }
        }

        Ok(woken + requeued_count)
    }

    /// Remove waiter (called on thread cancellation)
    pub fn remove_waiter(&mut self, key: FutexKey, tid: u32) -> bool {
        if let Some(queue) = self.buckets.get_mut(&key) {
            let removed = queue.remove_waiter(tid);
            if removed {
                self.active_waiters.fetch_sub(1, Ordering::SeqCst);
                if queue.is_empty() {
                    self.buckets.remove(&key);
                }
            }
            removed
        } else {
            false
        }
    }

    /// Get number of active waiters
    pub fn active_waiters(&self) -> usize {
        self.active_waiters.load(Ordering::SeqCst)
    }

    /// Get number of queues
    pub fn queue_count(&self) -> usize {
        self.buckets.len()
    }
}

/// Futex manager (singleton for the system)
pub struct FutexManager {
    /// Global hash table
    hash_table: FutexHashTable,
}

impl FutexManager {
    pub const fn new() -> Self {
        Self {
            hash_table: FutexHashTable::new(),
        }
    }

    /// Execute futex operation
    pub fn futex_op(
        &mut self,
        uaddr: usize,
        op: FutexOp,
        val: u32,
        val2: u32,
        uaddr2: usize,
        val3: u32,
        pid: u32,
        tid: u32,
        flags: FutexFlags,
    ) -> Result<usize, FutexError> {
        let key = if flags.private {
            FutexKey::new(pid, uaddr)
        } else {
            FutexKey::shared(uaddr)
        };

        match op {
            FutexOp::Wait => {
                // val = expected value, val3 = bitset
                let bitset = if val3 == 0 { 0xFFFFFFFF } else { val3 };
                self.hash_table.wait(key, val, val2, tid, bitset).map(|_| 0)
            }
            FutexOp::Wake => {
                // val = max waiters to wake, val3 = bitset
                let bitset = if val3 == 0 { 0xFFFFFFFF } else { val3 };
                Ok(self.hash_table.wake(key, val as usize, bitset))
            }
            FutexOp::WaitBitset => {
                // Same as WAIT but with explicit bitset
                self.hash_table.wait(key, val, val2, tid, val3).map(|_| 0)
            }
            FutexOp::WakeBitset => {
                // Same as WAKE but with explicit bitset
                Ok(self.hash_table.wake(key, val as usize, val3))
            }
            FutexOp::CmpRequeue => {
                // val = wake_count, val2 = expected value, val3 = requeue_count
                let target_key = if flags.private {
                    FutexKey::new(pid, uaddr2)
                } else {
                    FutexKey::shared(uaddr2)
                };
                self.hash_table.cmp_requeue(
                    key,
                    val2,
                    val, // Current value at uaddr
                    val as usize,
                    val3 as usize,
                    target_key,
                )
            }
            _ => Err(FutexError::Invalid),
        }
    }

    pub fn stats(&self) -> FutexStats {
        FutexStats {
            active_waiters: self.hash_table.active_waiters(),
            queue_count: self.hash_table.queue_count(),
        }
    }
}

/// Futex statistics
#[derive(Debug, Clone, Copy)]
pub struct FutexStats {
    pub active_waiters: usize,
    pub queue_count: usize,
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_futex_key() {
        let key1 = FutexKey::new(100, 0x1000);
        let key2 = FutexKey::new(100, 0x1000);
        let key3 = FutexKey::shared(0x1000);

        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_futex_queue() {
        let mut queue = FutexQueue::new();
        let waiter = FutexWaiter::new(1);
        queue.enqueue(waiter).unwrap();
        assert_eq!(queue.len(), 1);

        let woken = queue.wake(1, 0xFFFFFFFF);
        assert_eq!(woken, 1);
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_futex_wait_wake() {
        let mut manager = FutexManager::new();
        let key = FutexKey::new(100, 0x1000);

        // Wait on futex
        manager.hash_table.wait(key, 42, 42, 1, 0xFFFFFFFF).unwrap();
        assert_eq!(manager.hash_table.active_waiters(), 1);

        // Wake futex
        let woken = manager.hash_table.wake(key, 1, 0xFFFFFFFF);
        assert_eq!(woken, 1);
        assert_eq!(manager.hash_table.active_waiters(), 0);
    }

    #[test]
    fn test_futex_bitset() {
        let mut queue = FutexQueue::new();
        queue.enqueue(FutexWaiter::with_bitset(1, 0x01)).unwrap();
        queue.enqueue(FutexWaiter::with_bitset(2, 0x02)).unwrap();
        queue.enqueue(FutexWaiter::with_bitset(3, 0x04)).unwrap();

        // Wake only waiters with bit 0x02 set
        let woken = queue.wake(10, 0x02);
        assert_eq!(woken, 1);
        assert_eq!(queue.len(), 2);
    }

    #[test]
    fn test_futex_requeue() {
        let mut queue = FutexQueue::new();
        for i in 0..5 {
            queue.enqueue(FutexWaiter::new(i)).unwrap();
        }

        let target_key = FutexKey::new(100, 0x2000);
        let requeued = queue.requeue(3, target_key);

        assert_eq!(requeued.len(), 3);
        assert_eq!(queue.len(), 2);
    }
}
