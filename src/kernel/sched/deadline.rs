// SPDX-License-Identifier: MIT
// SigmaOS SCHED_DEADLINE Earliest-Deadline-First (EDF) Real-Time Scheduler Engine
// Zero-dependency, microsecond-accurate real-time task scheduler with Constant Bandwidth Server (CBS)

#![allow(dead_code)]

use std::collections::BinaryHeap;
use std::cmp::Ordering;

/// SCHED_DEADLINE Real-Time Task Parameters
#[derive(Debug, Clone)]
pub struct SchedDeadlineParams {
    pub pid: u64,
    pub runtime_ns: u64,  // Executable budget per period
    pub deadline_ns: u64, // Relative deadline
    pub period_ns: u64,   // Recurrence period
}

/// Active Real-Time Task Instance
#[derive(Debug, Clone)]
pub struct DeadlineTaskInstance {
    pub pid: u64,
    pub remaining_runtime_ns: u64,
    pub absolute_deadline_ns: u64,
    pub params: SchedDeadlineParams,
}

impl PartialEq for DeadlineTaskInstance {
    fn eq(&self, other: &Self) -> bool {
        self.absolute_deadline_ns == other.absolute_deadline_ns && self.pid == other.pid
    }
}

impl Eq for DeadlineTaskInstance {}

// Reverse ordering for Min-Heap (Earliest Deadline First)
impl Ord for DeadlineTaskInstance {
    fn cmp(&self, other: &Self) -> Ordering {
        other.absolute_deadline_ns.cmp(&self.absolute_deadline_ns)
            .then_with(|| self.pid.cmp(&other.pid))
    }
}

impl PartialOrd for DeadlineTaskInstance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Sovereign SCHED_DEADLINE EDF Real-Time Scheduler
#[derive(Debug)]
pub struct SovereignSchedDeadlineEngine {
    pub current_time_ns: u64,
    pub active_heap: BinaryHeap<DeadlineTaskInstance>,
    pub total_bandwidth_utilization: f64,
}

impl SovereignSchedDeadlineEngine {
    pub fn new() -> Self {
        Self {
            current_time_ns: 0,
            active_heap: BinaryHeap::new(),
            total_bandwidth_utilization: 0.0,
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

        let instance = DeadlineTaskInstance {
            pid: params.pid,
            remaining_runtime_ns: params.runtime_ns,
            absolute_deadline_ns: self.current_time_ns + params.deadline_ns,
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
                let pid = task.pid;
                self.active_heap.push(task);
                Some(pid)
            }
        } else {
            None
        }
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
        }).unwrap();

        sched.register_task(SchedDeadlineParams {
            pid: 200,
            runtime_ns: 1000,
            deadline_ns: 5000,
            period_ns: 5000,
        }).unwrap();

        // Earliest deadline (PID 200) must be scheduled first
        assert_eq!(sched.pick_next_task(), Some(200));

        let ran = sched.tick(1000);
        assert_eq!(ran, Some(200));
    }
}
