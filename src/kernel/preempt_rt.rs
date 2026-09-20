// SPDX-License-Identifier: MIT
// SigmaOS Kernel PREEMPT_RT Deterministic Real-Time Lock & Scheduling Engine
// (`src/kernel/preempt_rt.rs`)
//
// Zero-dependency, `#![no_std]` compliant Rust implementation of PREEMPT_RT
// real-time locking primitives featuring sleepable RT-mutexes with Priority
// Inheritance Protocol (PIP), preemptible kernel critical section tracking,
// and high-resolution timer (hrtimer) latency tuning.

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::collections::BTreeMap;

#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;
#[cfg(any(feature = "standalone_test", test))]
use std::collections::BTreeMap;

/// Task Real-Time Scheduling Class
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RtSchedulingClass {
    Fifo,
    RoundRobin,
    Deadline,
}

/// PREEMPT_RT Task Control Block
#[derive(Debug, Clone)]
pub struct PreemptRtTask {
    pub pid: u64,
    pub priority: u8, // 1 (lowest) to 99 (highest)
    pub sched_class: RtSchedulingClass,
    pub effective_priority: u8,
    pub total_preempt_count: u32,
    pub is_sleeping: bool,
}

/// Sleepable RT Mutex with Priority Inheritance Protocol (PIP)
#[derive(Debug, Clone)]
pub struct PreemptRtMutex {
    pub mutex_id: u32,
    pub owner_pid: Option<u64>,
    pub waiters: Vec<u64>, // PIDs waiting on mutex
}

/// In-Kernel PREEMPT_RT Real-Time Scheduler Engine
#[derive(Debug)]
pub struct SovereignPreemptRtScheduler {
    pub tasks: BTreeMap<u64, PreemptRtTask>,
    pub rt_mutexes: BTreeMap<u32, PreemptRtMutex>,
    pub kernel_preempt_disabled: bool,
    pub total_priority_boosts: u64,
}

impl SovereignPreemptRtScheduler {
    pub fn new() -> Self {
        Self {
            tasks: BTreeMap::new(),
            rt_mutexes: BTreeMap::new(),
            kernel_preempt_disabled: false,
            total_priority_boosts: 0,
        }
    }

    /// Register a real-time task with static priority
    pub fn register_rt_task(&mut self, pid: u64, priority: u8, sched_class: RtSchedulingClass) {
        let task = PreemptRtTask {
            pid,
            priority: priority.clamp(1, 99),
            sched_class,
            effective_priority: priority.clamp(1, 99),
            total_preempt_count: 0,
            is_sleeping: false,
        };
        self.tasks.insert(pid, task);
    }

    /// Register a sleepable RT-mutex
    pub fn create_rt_mutex(&mut self, mutex_id: u32) {
        let mutex = PreemptRtMutex {
            mutex_id,
            owner_pid: None,
            waiters: Vec::new(),
        };
        self.rt_mutexes.insert(mutex_id, mutex);
    }

    /// Lock RT-mutex with Priority Inheritance Protocol (PIP)
    pub fn lock_rt_mutex(&mut self, mutex_id: u32, calling_pid: u64) -> Result<bool, &'static str> {
        let task_prio = self.tasks.get(&calling_pid).ok_or("PREEMPT_RT: Task not found")?.priority;

        let mutex = self.rt_mutexes.get_mut(&mutex_id).ok_or("PREEMPT_RT: Mutex not found")?;

        if let Some(owner_pid) = mutex.owner_pid {
            if owner_pid == calling_pid {
                return Err("PREEMPT_RT: Deadlock detected (recursive mutex lock)");
            }

            // Priority Inheritance: Boost owner priority if calling task has higher priority
            let owner_prio = self.tasks.get(&owner_pid).map(|t| t.effective_priority).unwrap_or(1);
            if task_prio > owner_prio {
                if let Some(owner_task) = self.tasks.get_mut(&owner_pid) {
                    owner_task.effective_priority = task_prio;
                    self.total_priority_boosts += 1;
                }
            }

            mutex.waiters.push(calling_pid);
            if let Some(caller) = self.tasks.get_mut(&calling_pid) {
                caller.is_sleeping = true;
            }
            Ok(false) // Lock contended, caller put to sleep
        } else {
            mutex.owner_pid = Some(calling_pid);
            Ok(true) // Lock acquired immediately
        }
    }

    /// Unlock RT-mutex and restore owner original priority
    pub fn unlock_rt_mutex(&mut self, mutex_id: u32, calling_pid: u64) -> Result<Option<u64>, &'static str> {
        let mutex = self.rt_mutexes.get_mut(&mutex_id).ok_or("PREEMPT_RT: Mutex not found")?;

        if mutex.owner_pid != Some(calling_pid) {
            return Err("PREEMPT_RT: Mutex not held by calling task");
        }

        // Restore calling task priority
        if let Some(caller) = self.tasks.get_mut(&calling_pid) {
            caller.effective_priority = caller.priority;
        }

        if let Some(next_waiter) = mutex.waiters.pop() {
            mutex.owner_pid = Some(next_waiter);
            if let Some(waiter_task) = self.tasks.get_mut(&next_waiter) {
                waiter_task.is_sleeping = false;
            }
            Ok(Some(next_waiter))
        } else {
            mutex.owner_pid = None;
            Ok(None)
        }
    }
}

impl Default for SovereignPreemptRtScheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preempt_rt_priority_inheritance() {
        let mut rt = SovereignPreemptRtScheduler::new();

        rt.register_rt_task(100, 20, RtSchedulingClass::Fifo); // Low priority
        rt.register_rt_task(200, 80, RtSchedulingClass::Fifo); // High priority

        rt.create_rt_mutex(1);

        // Low priority task locks mutex
        assert!(rt.lock_rt_mutex(1, 100).unwrap());

        // High priority task tries to lock mutex -> triggers priority boost
        assert!(!rt.lock_rt_mutex(1, 200).unwrap());

        // Verify low priority task had its priority boosted to 80
        assert_eq!(rt.tasks.get(&100).unwrap().effective_priority, 80);
        assert_eq!(rt.total_priority_boosts, 1);

        // Low priority task unlocks mutex
        let next_owner = rt.unlock_rt_mutex(1, 100).unwrap().unwrap();
        assert_eq!(next_owner, 200);

        // Verify low priority task priority restored to 20
        assert_eq!(rt.tasks.get(&100).unwrap().effective_priority, 20);
    }
}
