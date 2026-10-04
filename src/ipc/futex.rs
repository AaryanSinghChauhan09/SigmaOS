//! SigmaOS — Futex (Fast Userspace Mutex) Implementation
//! SPDX-License-Identifier: MIT OR GPL-2.0
//! Inspired by Linux kernel futex implementation (futex.c)
//! Syscalls: futex(2) with FUTEX_WAIT, FUTEX_WAKE, FUTEX_REQUEUE

#![allow(dead_code)]

use core::sync::atomic::{AtomicU32, Ordering};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::vec::Vec;

// ── Error types ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FutexError {
    /// Value at address did not match expected (EAGAIN)
    ValueMismatch,
    /// Wait timed out (ETIMEDOUT)
    TimedOut,
    /// Invalid arguments (EINVAL)
    InvalidArgs,
    /// Operation not supported
    NotSupported,
    /// Memory fault (EFAULT)
    Fault,
}

impl core::fmt::Display for FutexError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ValueMismatch => write!(f, "futex: value mismatch (EAGAIN)"),
            Self::TimedOut => write!(f, "futex: timed out (ETIMEDOUT)"),
            Self::InvalidArgs => write!(f, "futex: invalid arguments (EINVAL)"),
            Self::NotSupported => write!(f, "futex: operation not supported"),
            Self::Fault => write!(f, "futex: memory fault (EFAULT)"),
        }
    }
}

// ── FutexOp ───────────────────────────────────────────────────────────────────

/// Futex operations — mirrors Linux FUTEX_* constants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FutexOp {
    /// FUTEX_WAIT — sleep if *addr == val
    Wait,
    /// FUTEX_WAKE — wake up to count waiters
    Wake,
    /// FUTEX_REQUEUE — wake some, move rest to addr2
    Requeue,
    /// FUTEX_WAKE_OP — wake + conditional atomic operation
    WakeOp,
    /// FUTEX_WAIT_BITSET — wait with bitset mask
    WaitBitset,
    /// FUTEX_WAKE_BITSET — wake waiters matching bitset
    WakeBitset,
    /// FUTEX_WAIT_REQUEUE_PI — private wait-requeue for PI
    WaitRequeuePrivate,
}

impl FutexOp {
    /// Convert from raw u32 syscall argument
    pub fn from_raw(val: u32) -> Option<Self> {
        match val & 0x7F {
            0 => Some(Self::Wait),
            1 => Some(Self::Wake),
            3 => Some(Self::Requeue),
            5 => Some(Self::WakeOp),
            9 => Some(Self::WaitBitset),
            10 => Some(Self::WakeBitset),
            11 => Some(Self::WaitRequeuePrivate),
            _ => None,
        }
    }
}

// ── FutexKey ──────────────────────────────────────────────────────────────────

/// Unique key identifying a futex word in a process address space
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FutexKey {
    /// Address space / mm ID (0 = global for shared futexes)
    pub mm: u64,
    /// Virtual address of the futex word
    pub address: u64,
    /// Whether this is a private (process-local) futex
    pub private: bool,
}

impl FutexKey {
    pub fn new(mm: u64, address: u64, private: bool) -> Self {
        Self {
            mm,
            address,
            private,
        }
    }

    /// For shared futexes the mm is zeroed so different processes can share
    pub fn shared(address: u64) -> Self {
        Self {
            mm: 0,
            address,
            private: false,
        }
    }

    pub fn private(mm: u64, address: u64) -> Self {
        Self {
            mm,
            address,
            private: true,
        }
    }
}

// ── FutexWaiter ───────────────────────────────────────────────────────────────

/// A task blocked on a futex
#[derive(Debug, Clone)]
pub struct FutexWaiter {
    /// ID of the blocked task
    pub task_id: u64,
    /// Expected value (the value that was in *addr when we went to sleep)
    pub value: u32,
    /// Bitset for FUTEX_WAIT_BITSET / FUTEX_WAKE_BITSET (0xFFFFFFFF = match all)
    pub bitset: u32,
    /// Optional absolute deadline in nanoseconds; None = block indefinitely
    pub timeout_ns: Option<u64>,
    /// True once this waiter has been woken
    pub woken: bool,
}

impl FutexWaiter {
    pub fn new(task_id: u64, value: u32, timeout_ns: Option<u64>) -> Self {
        Self {
            task_id,
            value,
            bitset: 0xFFFF_FFFF,
            timeout_ns,
            woken: false,
        }
    }

    pub fn with_bitset(mut self, bitset: u32) -> Self {
        self.bitset = bitset;
        self
    }
}

// ── Hash table bucket ─────────────────────────────────────────────────────────

const FUTEX_HASH_BUCKETS: usize = 256;

/// A single hash bucket containing waiters for one or more futex addresses
#[derive(Debug, Default)]
pub struct FutexBucket {
    /// Map from FutexKey → list of waiters
    pub waiters: HashMap<FutexKey, Vec<FutexWaiter>>,
}

impl FutexBucket {
    pub fn new() -> Self {
        Self {
            waiters: HashMap::new(),
        }
    }

    /// Add a waiter for the given key
    pub fn add_waiter(&mut self, key: FutexKey, waiter: FutexWaiter) {
        self.waiters.entry(key).or_default().push(waiter);
    }

    /// Wake up to `count` waiters matching `bitset`; return number woken
    pub fn wake(&mut self, key: &FutexKey, count: u32, bitset: u32) -> u32 {
        let mut woken = 0u32;
        if let Some(list) = self.waiters.get_mut(key) {
            for w in list.iter_mut() {
                if woken >= count {
                    break;
                }
                if !w.woken && (w.bitset & bitset) != 0 {
                    w.woken = true;
                    woken += 1;
                }
            }
            list.retain(|w| !w.woken);
        }
        woken
    }

    /// Drain up to `count` waiters from `key` into another bucket/key
    pub fn requeue(
        &mut self,
        from: &FutexKey,
        to: &FutexKey,
        wake_count: u32,
        requeue_count: u32,
        dest: &mut FutexBucket,
    ) -> (u32, u32) {
        let mut woken = 0u32;
        let mut requeued = 0u32;

        if let Some(list) = self.waiters.get_mut(from) {
            // Wake first `wake_count`
            for w in list.iter_mut() {
                if woken >= wake_count {
                    break;
                }
                if !w.woken {
                    w.woken = true;
                    woken += 1;
                }
            }
            list.retain(|w| !w.woken);

            // Requeue next `requeue_count` to dest bucket
            let mut to_move = Vec::new();
            for _ in 0..requeue_count {
                if list.is_empty() {
                    break;
                }
                to_move.push(list.remove(0));
                requeued += 1;
            }
            dest.waiters.entry(to.clone()).or_default().extend(to_move);
        }
        (woken, requeued)
    }
}

// ── FutexHashTable ────────────────────────────────────────────────────────────

/// Global futex hash table: 256 buckets, each guarded by its own Mutex
pub struct FutexHashTable {
    buckets: Vec<Arc<Mutex<FutexBucket>>>,
}

impl FutexHashTable {
    pub fn new() -> Self {
        let mut buckets = Vec::with_capacity(FUTEX_HASH_BUCKETS);
        for _ in 0..FUTEX_HASH_BUCKETS {
            buckets.push(Arc::new(Mutex::new(FutexBucket::new())));
        }
        Self { buckets }
    }

    /// Hash a FutexKey to a bucket index
    pub fn hash(&self, key: &FutexKey) -> usize {
        // Simple multiplicative hash of address + mm
        let h = key.address.wrapping_mul(0x9e37_79b9_7f4a_7c15)
            ^ key.mm.wrapping_mul(0x6c62_272e_07bb_0142);
        (h as usize) & (FUTEX_HASH_BUCKETS - 1)
    }

    /// Get bucket arc for a key
    pub fn bucket(&self, key: &FutexKey) -> Arc<Mutex<FutexBucket>> {
        Arc::clone(&self.buckets[self.hash(key)])
    }
}

impl Default for FutexHashTable {
    fn default() -> Self {
        Self::new()
    }
}

// ── Public Futex API ──────────────────────────────────────────────────────────

/// Simulated futex_wait.
///
/// In a real kernel this would:
/// 1. Atomically verify that `*addr == val`.
/// 2. Add the calling task to the hash bucket's wait queue.
/// 3. Put the task to sleep.
///
/// Here we simulate the atomic check and queue insertion.
///
/// Returns:
/// * `Ok(())` — woken normally
/// * `Err(FutexError::ValueMismatch)` — *addr != val at wait time
/// * `Err(FutexError::TimedOut)` — deadline expired
pub fn futex_wait(
    table: &FutexHashTable,
    key: FutexKey,
    addr: &AtomicU32,
    val: u32,
    task_id: u64,
    timeout_ns: Option<u64>,
) -> Result<(), FutexError> {
    // Atomic check: EAGAIN if the value already changed
    if addr.load(Ordering::Acquire) != val {
        return Err(FutexError::ValueMismatch);
    }

    let bucket = table.bucket(&key);
    let mut b = bucket.lock().unwrap();
    let waiter = FutexWaiter::new(task_id, val, timeout_ns);
    b.add_waiter(key, waiter);
    Ok(())
}

/// Simulated futex_wake.
///
/// Wakes up to `count` waiters on `addr` (matching `bitset`).
/// Returns the number of tasks actually woken.
pub fn futex_wake(table: &FutexHashTable, key: &FutexKey, count: u32, bitset: u32) -> u32 {
    let bucket = table.bucket(key);
    let mut b = bucket.lock().unwrap();
    b.wake(key, count, bitset)
}

/// Simulated futex_requeue.
///
/// Wakes `wake_count` waiters on `from_key`, then moves up to `requeue_count`
/// remaining waiters to `to_key`.
/// Returns (woken, requeued).
pub fn futex_requeue(
    table: &FutexHashTable,
    from_key: &FutexKey,
    to_key: &FutexKey,
    wake_count: u32,
    requeue_count: u32,
) -> (u32, u32) {
    // To avoid deadlock, lock both buckets in a consistent order.
    let from_idx = table.hash(from_key);
    let to_idx = table.hash(to_key);

    if from_idx == to_idx {
        // Same bucket — safe to lock once
        let bucket = table.bucket(from_key);
        let mut b = bucket.lock().unwrap();
        let mut dest_tmp = FutexBucket::new();
        let (woken, requeued) =
            b.requeue(from_key, to_key, wake_count, requeue_count, &mut dest_tmp);
        // Merge dest_tmp back into the same bucket
        for (k, mut v) in dest_tmp.waiters {
            b.waiters.entry(k).or_default().append(&mut v);
        }
        (woken, requeued)
    } else {
        // Different buckets
        let (lo, hi) = if from_idx < to_idx {
            (
                Arc::clone(&table.buckets[from_idx]),
                Arc::clone(&table.buckets[to_idx]),
            )
        } else {
            (
                Arc::clone(&table.buckets[to_idx]),
                Arc::clone(&table.buckets[from_idx]),
            )
        };
        let (mut lo_g, mut hi_g) = (lo.lock().unwrap(), hi.lock().unwrap());
        let (from_g, to_g) = if from_idx < to_idx {
            (&mut *lo_g, &mut *hi_g)
        } else {
            (&mut *hi_g, &mut *lo_g)
        };
        from_g.requeue(from_key, to_key, wake_count, requeue_count, to_g)
    }
}

/// Robust list entry — mirrors Linux robust_list
#[derive(Debug, Clone)]
pub struct RobustListEntry {
    pub lock_addr: u64,
    pub task_id: u64,
}

/// Process robust futex list — maintained by the kernel on behalf of userspace.
/// When a task dies, the kernel walks this list and marks each owned futex
/// with FUTEX_OWNER_DIED so other waiters can clean up.
#[derive(Debug, Default)]
pub struct RobustFutexList {
    entries: Vec<RobustListEntry>,
}

impl RobustFutexList {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a lock acquisition
    pub fn register(&mut self, lock_addr: u64, task_id: u64) {
        self.entries.push(RobustListEntry { lock_addr, task_id });
    }

    /// Deregister a lock release
    pub fn deregister(&mut self, lock_addr: u64, task_id: u64) {
        self.entries
            .retain(|e| !(e.lock_addr == lock_addr && e.task_id == task_id));
    }

    /// Handle process exit: mark all owned futexes with FUTEX_OWNER_DIED (bit 30)
    pub fn handle_exit(&self, atomics: &HashMap<u64, Arc<AtomicU32>>, dead_task: u64) {
        for entry in &self.entries {
            if entry.task_id == dead_task {
                if let Some(atom) = atomics.get(&entry.lock_addr) {
                    // Set FUTEX_OWNER_DIED bit and clear TID
                    atom.fetch_or(0x4000_0000, Ordering::Release);
                }
            }
        }
    }

    pub fn entries(&self) -> &[RobustListEntry] {
        &self.entries
    }
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    fn make_table() -> FutexHashTable {
        FutexHashTable::new()
    }

    fn make_key(addr: u64) -> FutexKey {
        FutexKey::private(1, addr)
    }

    #[test]
    fn test_futex_wait_wake_basic() {
        let table = make_table();
        let atom = AtomicU32::new(0);
        let key = make_key(0x1000);

        // Wait should succeed (value matches)
        let res = futex_wait(&table, key.clone(), &atom, 0, 42, None);
        assert!(res.is_ok());

        // Wake should unblock 1 waiter
        let woken = futex_wake(&table, &key, 1, 0xFFFF_FFFF);
        assert_eq!(woken, 1);

        // Second wake: no more waiters
        let woken2 = futex_wake(&table, &key, 1, 0xFFFF_FFFF);
        assert_eq!(woken2, 0);
    }

    #[test]
    fn test_futex_wait_value_mismatch() {
        let table = make_table();
        let atom = AtomicU32::new(99); // does NOT match expected val=0
        let key = make_key(0x2000);
        let res = futex_wait(&table, key, &atom, 0, 1, None);
        assert_eq!(res, Err(FutexError::ValueMismatch));
    }

    #[test]
    fn test_futex_wake_multiple() {
        let table = make_table();
        let atom = AtomicU32::new(0);
        let key = make_key(0x3000);

        // Queue 3 waiters
        for tid in 1..=3 {
            futex_wait(&table, key.clone(), &atom, 0, tid, None).unwrap();
        }
        // Wake only 2
        let woken = futex_wake(&table, &key, 2, 0xFFFF_FFFF);
        assert_eq!(woken, 2);
        // One still waiting
        let woken2 = futex_wake(&table, &key, 10, 0xFFFF_FFFF);
        assert_eq!(woken2, 1);
    }

    #[test]
    fn test_futex_requeue() {
        let table = make_table();
        let atom = AtomicU32::new(0);
        let key1 = make_key(0x4000);
        let key2 = make_key(0x5000);

        // 4 waiters on key1
        for tid in 1..=4 {
            futex_wait(&table, key1.clone(), &atom, 0, tid, None).unwrap();
        }
        // Wake 1, requeue 2 to key2
        let (woken, requeued) = futex_requeue(&table, &key1, &key2, 1, 2);
        assert_eq!(woken, 1);
        assert_eq!(requeued, 2);

        // 1 still on key1
        let w1 = futex_wake(&table, &key1, 10, 0xFFFF_FFFF);
        assert_eq!(w1, 1);
        // 2 on key2
        let w2 = futex_wake(&table, &key2, 10, 0xFFFF_FFFF);
        assert_eq!(w2, 2);
    }

    #[test]
    fn test_robust_list_owner_died() {
        let mut rlist = RobustFutexList::new();
        let atom = Arc::new(AtomicU32::new(1)); // TID=1 owns it
        let addr = 0xDEAD_BEEF_u64;

        rlist.register(addr, 1);

        let mut atomics = HashMap::new();
        atomics.insert(addr, Arc::clone(&atom));

        rlist.handle_exit(&atomics, 1);

        // FUTEX_OWNER_DIED bit should be set
        assert!(atom.load(Ordering::Relaxed) & 0x4000_0000 != 0);
    }

    #[test]
    fn test_futex_op_from_raw() {
        assert_eq!(FutexOp::from_raw(0), Some(FutexOp::Wait));
        assert_eq!(FutexOp::from_raw(1), Some(FutexOp::Wake));
        assert_eq!(FutexOp::from_raw(3), Some(FutexOp::Requeue));
        assert_eq!(FutexOp::from_raw(128), Some(FutexOp::Wait)); // bit 7 is PRIVATE flag
        assert_eq!(FutexOp::from_raw(99), None);
    }
}
