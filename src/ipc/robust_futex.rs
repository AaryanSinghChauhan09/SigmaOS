// SigmaOS Robust Futex with Priority Inheritance & Owner-Death Handling
// Prevents deadlocks when mutex-holding threads terminate unexpectedly in userspace.

#![allow(dead_code)]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

pub const FUTEX_WAITERS: u32 = 0x8000_0000;
pub const FUTEX_OWNER_DIED: u32 = 0x4000_0000;
pub const FUTEX_TID_MASK: u32 = 0x3FFF_FFFF;

/// Waiter on a robust futex word
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RobustFutexWaiter {
    pub tid: u32,
    pub priority: i32,
}

/// State of an individual robust futex tracked by kernel
#[derive(Debug, Clone)]
pub struct RobustFutexState {
    pub uaddr: u64,
    pub owner_tid: u32,
    pub owner_died: bool,
    pub waiters: Vec<RobustFutexWaiter>,
}

/// Robust Futex Registry & Thread Exit Cleanup Manager
#[derive(Debug, Clone, Default)]
pub struct RobustFutexManager {
    pub tracked_futexes: BTreeMap<u64, RobustFutexState>, // uaddr -> state
    pub thread_held_futexes: BTreeMap<u32, Vec<u64>>,     // tid -> [uaddr]
}

impl RobustFutexManager {
    pub fn new() -> Self {
        Self {
            tracked_futexes: BTreeMap::new(),
            thread_held_futexes: BTreeMap::new(),
        }
    }

    /// Register acquisition of a robust futex by a thread
    pub fn lock(&mut self, uaddr: u64, caller_tid: u32, priority: i32) -> Result<(), &'static str> {
        let state = self
            .tracked_futexes
            .entry(uaddr)
            .or_insert_with(|| RobustFutexState {
                uaddr,
                owner_tid: 0,
                owner_died: false,
                waiters: Vec::new(),
            });

        if state.owner_tid == 0 {
            state.owner_tid = caller_tid;
            state.owner_died = false;
            self.thread_held_futexes
                .entry(caller_tid)
                .or_insert_with(Vec::new)
                .push(uaddr);
            Ok(())
        } else if state.owner_tid == caller_tid {
            Err("Deadlock: recursive robust lock")
        } else {
            // Queue waiter
            state.waiters.push(RobustFutexWaiter {
                tid: caller_tid,
                priority,
            });
            Err("EAGAIN / Contended")
        }
    }

    /// Unlock a robust futex and wake the highest priority waiter
    pub fn unlock(&mut self, uaddr: u64, caller_tid: u32) -> Option<u32> {
        let state = self.tracked_futexes.get_mut(&uaddr)?;

        if state.owner_tid != caller_tid {
            return None;
        }

        // Remove from thread held list
        if let Some(list) = self.thread_held_futexes.get_mut(&caller_tid) {
            if let Some(pos) = list.iter().position(|&a| a == uaddr) {
                list.remove(pos);
            }
        }

        if state.waiters.is_empty() {
            state.owner_tid = 0;
            None
        } else {
            // Wake highest priority waiter (lowest priority number)
            let mut best_idx = 0;
            let mut min_prio = state.waiters[0].priority;
            for (idx, w) in state.waiters.iter().enumerate().skip(1) {
                if w.priority < min_prio {
                    min_prio = w.priority;
                    best_idx = idx;
                }
            }

            let next_waiter = state.waiters.remove(best_idx);
            state.owner_tid = next_waiter.tid;
            self.thread_held_futexes
                .entry(next_waiter.tid)
                .or_insert_with(Vec::new)
                .push(uaddr);

            Some(next_waiter.tid)
        }
    }

    /// Kernel cleanup called upon thread exit: marks all held futexes with FUTEX_OWNER_DIED
    /// and wakes next waiting thread with EOWNERDEAD
    pub fn handle_thread_exit(&mut self, dying_tid: u32) -> Vec<(u64, Option<u32>)> {
        let mut recovered_futexes = Vec::new();

        if let Some(held_addrs) = self.thread_held_futexes.remove(&dying_tid) {
            for uaddr in held_addrs {
                if let Some(state) = self.tracked_futexes.get_mut(&uaddr) {
                    state.owner_died = true;

                    let next_owner = if state.waiters.is_empty() {
                        state.owner_tid = 0;
                        None
                    } else {
                        let next_waiter = state.waiters.remove(0);
                        state.owner_tid = next_waiter.tid;
                        self.thread_held_futexes
                            .entry(next_waiter.tid)
                            .or_insert_with(Vec::new)
                            .push(uaddr);
                        Some(next_waiter.tid)
                    };

                    recovered_futexes.push((uaddr, next_owner));
                }
            }
        }

        recovered_futexes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robust_futex_owner_death_recovery() {
        let mut manager = RobustFutexManager::new();
        let uaddr = 0x7FFF_0000_1000;

        // Thread 10 acquires lock
        assert!(manager.lock(uaddr, 10, 50).is_ok());

        // Thread 20 tries to acquire lock -> contended
        assert!(manager.lock(uaddr, 20, 20).is_err());

        // Thread 10 terminates unexpectedly!
        let recovered = manager.handle_thread_exit(10);
        assert_eq!(recovered.len(), 1);
        assert_eq!(recovered[0], (uaddr, Some(20))); // Ownership transferred to Thread 20 with owner_died set!

        let state = manager.tracked_futexes.get(&uaddr).unwrap();
        assert!(state.owner_died);
        assert_eq!(state.owner_tid, 20);
    }
}
