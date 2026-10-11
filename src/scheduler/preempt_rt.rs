// SigmaOS PREEMPT_RT Real-Time Preemption Engine
// Provides priority inheritance mutexes and preemption disabling guards for deterministic latencies.

#![allow(dead_code)]
extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Per-CPU Preemption Counter
static PREEMPT_COUNT: AtomicUsize = AtomicUsize::new(0);

/// RAII Guard that temporarily disables kernel preemption
#[derive(Debug)]
pub struct PreemptGuard;

impl PreemptGuard {
    pub fn new() -> Self {
        PREEMPT_COUNT.fetch_add(1, Ordering::SeqCst);
        Self
    }

    pub fn is_preemption_disabled() -> bool {
        PREEMPT_COUNT.load(Ordering::Relaxed) > 0
    }
}

impl Default for PreemptGuard {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for PreemptGuard {
    fn drop(&mut self) {
        PREEMPT_COUNT.fetch_sub(1, Ordering::SeqCst);
    }
}

/// Waiter for an RtMutex
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RtWaiter {
    pub pid: u32,
    pub original_priority: i32,
}

/// Real-time Mutex with Priority Inheritance (PI)
#[derive(Debug, Clone)]
pub struct RtMutex {
    pub owner_pid: Option<u32>,
    pub owner_effective_priority: i32,
    pub owner_original_priority: i32,
    pub waiters: Vec<RtWaiter>,
}

impl RtMutex {
    pub fn new() -> Self {
        Self {
            owner_pid: None,
            owner_effective_priority: 0,
            owner_original_priority: 0,
            waiters: Vec::new(),
        }
    }

    /// Attempt to acquire lock. If already locked, boosts owner priority if caller is higher priority (lower number).
    pub fn lock(&mut self, caller_pid: u32, caller_priority: i32) -> Result<(), &'static str> {
        let _guard = PreemptGuard::new();

        match self.owner_pid {
            None => {
                // Free: acquire immediately
                self.owner_pid = Some(caller_pid);
                self.owner_original_priority = caller_priority;
                self.owner_effective_priority = caller_priority;
                Ok(())
            }
            Some(owner) => {
                if owner == caller_pid {
                    return Err("Deadlock: Recursive RtMutex acquisition");
                }

                // Add caller to wait queue
                self.waiters.push(RtWaiter {
                    pid: caller_pid,
                    original_priority: caller_priority,
                });

                // Priority inheritance: if caller has higher priority (smaller value), boost owner!
                if caller_priority < self.owner_effective_priority {
                    self.owner_effective_priority = caller_priority;
                }

                Err("Contended: Task placed on PI wait list")
            }
        }
    }

    /// Release mutex and transfer ownership to highest priority waiting task
    pub fn unlock(&mut self, caller_pid: u32) -> Option<u32> {
        let _guard = PreemptGuard::new();

        if self.owner_pid != Some(caller_pid) {
            return None;
        }

        if self.waiters.is_empty() {
            self.owner_pid = None;
            self.owner_effective_priority = 0;
            self.owner_original_priority = 0;
            None
        } else {
            // Find highest priority waiter (minimum priority value in standard RT numbering)
            let mut highest_idx = 0;
            let mut min_prio = self.waiters[0].original_priority;
            for (idx, waiter) in self.waiters.iter().enumerate().skip(1) {
                if waiter.original_priority < min_prio {
                    min_prio = waiter.original_priority;
                    highest_idx = idx;
                }
            }

            let next_owner = self.waiters.remove(highest_idx);
            self.owner_pid = Some(next_owner.pid);
            self.owner_original_priority = next_owner.original_priority;

            // Recalculate boosted priority among remaining waiters
            let boosted = self
                .waiters
                .iter()
                .map(|w| w.original_priority)
                .min()
                .unwrap_or(next_owner.original_priority)
                .min(next_owner.original_priority);

            self.owner_effective_priority = boosted;
            Some(next_owner.pid)
        }
    }
}

impl Default for RtMutex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preempt_rt_mutex_priority_inheritance() {
        let mut mutex = RtMutex::new();

        // Low priority task (prio 80) acquires lock
        assert!(mutex.lock(100, 80).is_ok());
        assert_eq!(mutex.owner_effective_priority, 80);

        // High priority task (prio 10) attempts lock -> fails and boosts owner to 10
        assert!(mutex.lock(200, 10).is_err());
        assert_eq!(mutex.owner_effective_priority, 10); // Priority inheritance active!

        // Low priority task unlocks -> lock handed over to high priority task
        let next = mutex.unlock(100);
        assert_eq!(next, Some(200));
        assert_eq!(mutex.owner_pid, Some(200));
        assert_eq!(mutex.owner_effective_priority, 10);
    }
}
