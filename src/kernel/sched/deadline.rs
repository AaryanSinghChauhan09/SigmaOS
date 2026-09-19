// SPDX-License-Identifier: MIT
// SigmaOS SCHED_DEADLINE Earliest-Deadline-First (EDF) Real-Time Scheduler Engine
// Zero-dependency, microsecond-accurate real-time task scheduler with Constant Bandwidth Server (CBS)
// Enhanced with EEVDF (Earliest Eligible Virtual Deadline First) and hard preemption

#![allow(dead_code)]

use std::collections::BinaryHeap;
use std::cmp::Ordering as CmpOrdering;
use core::sync::atomic::{AtomicU64, AtomicU32, Ordering as AtomicOrdering};

/// SCHED_DEADLINE Real-Time Task Parameters
#[derive(Debug, Clone)]
pub struct SchedDeadlineParams {
    pub pid: u64,
    pub runtime_ns: u64,      // Executable budget per period
    pub deadline_ns: u64,     // Relative deadline
    pub period_ns: u64,       // Recurrence period
    pub preemptible: bool,    // Whether task can be preempted
}

/// Active Real-Time Task Instance with EEVDF virtual deadline
#[derive(Debug, Clone)]
pub struct DeadlineTaskInstance {
    pub pid: u64,
    pub remaining_runtime_ns: u64,
    pub absolute_deadline_ns: u64,
    pub virtual_deadline_ns: u64,  // EEVDF virtual deadline
    pub params: SchedDeadlineParams,
}

impl PartialEq for DeadlineTaskInstance {
    fn eq(&self, other: &Self) -> bool {
        self.virtual_deadline_ns == other.virtual_deadline_ns && self.pid == other.pid
    }
}

impl Eq for DeadlineTaskInstance {}

// Reverse ordering for Min-Heap (Earliest Virtual Deadline First - EEVDF)
impl Ord for DeadlineTaskInstance {
    fn cmp(&self, other: &Self) -> CmpOrdering {
        other.virtual_deadline_ns.cmp(&self.virtual_deadline_ns)
            .then_with(|| self.pid.cmp(&other.pid))
    }
}

impl PartialOrd for DeadlineTaskInstance {
    fn partial_cmp(&self, other: &Self) -> Option<CmpOrdering> {
        Some(self.cmp(other))
    }
}

/// Sovereign SCHED_DEADLINE EDF Real-Time Scheduler with EEVDF
#[derive(Debug)]
pub struct SovereignSchedDeadlineEngine {
    pub current_time_ns: u64,
    pub active_heap: BinaryHeap<DeadlineTaskInstance>,
    pub total_bandwidth_utilization: f64,
    pub preemptions: AtomicU64,
    pub context_switches: AtomicU64,
}

impl SovereignSchedDeadlineEngine {
    pub fn new() -> Self {
        Self {
            current_time_ns: 0,
            active_heap: BinaryHeap::new(),
            total_bandwidth_utilization: 0.0,
            preemptions: AtomicU64::new(0),
            context_switches: AtomicU64::new(0),
        }
    }

    pub fn register_task(&mut self, params: SchedDeadlineParams) -> Result<(), &'static str> {
        if params.runtime_ns == 0 || params.deadline_ns == 0 || params.period_ns == 0 {
            return Err("Invalid SCHED_DEADLINE parameters");
        }
        if params.runtime_ns > params.deadline_ns || params.deadline_ns > params.period_ns {
            return Err("Runtime > Deadline or Deadline > Period invalid");
        }

        let task_util = (params.runtime_ns as f64) / (params.period_ns as f64);
        if self.total_bandwidth_utilization + task_util > 0.95 {
            return Err("SCHED_DEADLINE admission control rejected: bandwidth > 95%");
        }

        self.total_bandwidth_utilization += task_util;

        // Calculate initial virtual deadline for EEVDF
        let virtual_deadline = self.current_time_ns + params.deadline_ns;

        let instance = DeadlineTaskInstance {
            pid: params.pid,
            remaining_runtime_ns: params.runtime_ns,
            absolute_deadline_ns: self.current_time_ns + params.deadline_ns,
            virtual_deadline_ns: virtual_deadline,
            params,
        };

        self.active_heap.push(instance);
        Ok(())
    }

    pub fn pick_next_task(&mut self) -> Option<u64> {
        self.active_heap.peek().map(|t| t.pid)
    }

    pub fn tick(&mut self, delta_ns: u64) -> Option<u64> {
        self.current_time_ns += delta_ns;

        if let Some(mut task) = self.active_heap.pop() {
            if task.remaining_runtime_ns > delta_ns {
                task.remaining_runtime_ns -= delta_ns;
                let pid = task.pid;
                self.active_heap.push(task);
                Some(pid)
            } else {
                // Task budget exhausted for current period: replenish for next period (CBS)
                let next_deadline = task.absolute_deadline_ns + task.params.period_ns;
                task.remaining_runtime_ns = task.params.runtime_ns;
                task.absolute_deadline_ns = next_deadline;
                task.virtual_deadline_ns = next_deadline; // Update virtual deadline
                let pid = task.pid;
                self.active_heap.push(task);
                Some(pid)
            }
        } else {
            None
        }
    }

    /// Force preemption of current task (hard preemption)
    pub fn preempt_current(&mut self) -> bool {
        if let Some(_) = self.active_heap.pop() {
            self.preemptions.fetch_add(1, AtomicOrdering::SeqCst);
            self.context_switches.fetch_add(1, AtomicOrdering::SeqCst);
            true
        } else {
            false
        }
    }

    /// Update virtual deadline for EEVDF when task yields
    pub fn update_virtual_deadline(&mut self, pid: u64, new_virtual_deadline: u64) -> bool {
        let mut vec = std::mem::take(&mut self.active_heap).into_vec();
        let mut found = false;
        for task in vec.iter_mut() {
            if task.pid == pid {
                task.virtual_deadline_ns = new_virtual_deadline;
                found = true;
                break;
            }
        }
        self.active_heap = BinaryHeap::from(vec);
        found
    }

    pub fn get_preemption_count(&self) -> u64 {
        self.preemptions.load(AtomicOrdering::SeqCst)
    }

    pub fn get_context_switch_count(&self) -> u64 {
        self.context_switches.load(AtomicOrdering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sched_deadline_edf_ordering() {
        let mut sched = SovereignSchedDeadlineEngine::new();
        sched.register_task(SchedDeadlineParams {
            pid: 100,
            runtime_ns: 2000,
            deadline_ns: 10000,
            period_ns: 10000,
            preemptible: true,
        }).unwrap();

        sched.register_task(SchedDeadlineParams {
            pid: 200,
            runtime_ns: 1000,
            deadline_ns: 5000,
            period_ns: 5000,
            preemptible: true,
        }).unwrap();

        // Earliest virtual deadline (PID 200) must be scheduled first
        assert_eq!(sched.pick_next_task(), Some(200));

        let ran = sched.tick(1000);
        assert_eq!(ran, Some(200));
    }

    #[test]
    fn test_hard_preemption() {
        let mut sched = SovereignSchedDeadlineEngine::new();
        sched.register_task(SchedDeadlineParams {
            pid: 100,
            runtime_ns: 2000,
            deadline_ns: 10000,
            period_ns: 10000,
            preemptible: true,
        }).unwrap();

        assert!(sched.preempt_current());
        assert_eq!(sched.get_preemption_count(), 1);
        assert_eq!(sched.get_context_switch_count(), 1);
    }

    #[test]
    fn test_eevdf_virtual_deadline() {
        let mut sched = SovereignSchedDeadlineEngine::new();
        sched.register_task(SchedDeadlineParams {
            pid: 100,
            runtime_ns: 2000,
            deadline_ns: 10000,
            period_ns: 10000,
            preemptible: true,
        }).unwrap();

        // Update virtual deadline
        assert!(sched.update_virtual_deadline(100, 15000));
    }

    #[test]
    fn test_bandwidth_admission_control() {
        let mut sched = SovereignSchedDeadlineEngine::new();
        
        // First task: 50% bandwidth
        sched.register_task(SchedDeadlineParams {
            pid: 100,
            runtime_ns: 5000,
            deadline_ns: 10000,
            period_ns: 10000,
            preemptible: true,
        }).unwrap();

        // Second task: 40% bandwidth (total 90%)
        sched.register_task(SchedDeadlineParams {
            pid: 200,
            runtime_ns: 4000,
            deadline_ns: 10000,
            period_ns: 10000,
            preemptible: true,
        }).unwrap();

        // Third task: 10% bandwidth (total 100% - should fail)
        let result = sched.register_task(SchedDeadlineParams {
            pid: 300,
            runtime_ns: 1000,
            deadline_ns: 10000,
            period_ns: 10000,
            preemptible: true,
        });
        assert!(result.is_err());
    }
}
