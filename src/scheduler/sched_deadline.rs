// SigmaOS SCHED_DEADLINE with Constant Bandwidth Server (CBS) Algorithm
// Guarantees hard/soft real-time task deadlines with budget enforcement and over-utilization throttling.

#![allow(dead_code)]
extern crate alloc;

use alloc::vec::Vec;

/// Task scheduled under SCHED_DEADLINE policy
#[derive(Debug, Clone)]
pub struct DeadlineTask {
    pub pid: u32,
    pub runtime_ns: u64,  // Worst-case execution time (WCET) C_i
    pub deadline_ns: u64, // Relative deadline D_i
    pub period_ns: u64,   // Period P_i
    pub remaining_budget_ns: u64,
    pub current_absolute_deadline_ns: u64,
    pub is_throttled: bool,
}

/// Constant Bandwidth Server (CBS) Scheduler Engine
#[derive(Debug, Clone, Default)]
pub struct DeadlineScheduler {
    pub tasks: Vec<DeadlineTask>,
    pub max_bandwidth_percent: u32, // Default 95% (leaving 5% for non-RT kernel tasks)
    pub current_time_ns: u64,
}

impl DeadlineScheduler {
    pub fn new(max_bandwidth_percent: u32) -> Self {
        Self {
            tasks: Vec::new(),
            max_bandwidth_percent,
            current_time_ns: 0,
        }
    }

    /// Calculate total bandwidth consumption as percentage (sum of C_i / P_i)
    pub fn current_bandwidth_percent(&self) -> u32 {
        let mut total = 0u64;
        for t in &self.tasks {
            if t.period_ns > 0 {
                total += (t.runtime_ns * 100) / t.period_ns;
            }
        }
        total as u32
    }

    /// Perform CBS schedulability admission control test
    pub fn admit_task(&mut self, mut task: DeadlineTask) -> Result<(), &'static str> {
        if task.runtime_ns > task.deadline_ns
            || task.deadline_ns > task.period_ns
            || task.period_ns == 0
        {
            return Err("Invalid deadline parameters: runtime <= deadline <= period required");
        }

        let task_bw = (task.runtime_ns * 100) / task.period_ns;
        if self.current_bandwidth_percent() + (task_bw as u32) > self.max_bandwidth_percent {
            return Err("SCHED_DEADLINE admission rejected: Bandwidth exceeds quota");
        }

        task.remaining_budget_ns = task.runtime_ns;
        task.current_absolute_deadline_ns = self.current_time_ns + task.deadline_ns;
        task.is_throttled = false;

        self.tasks.push(task);
        Ok(())
    }

    /// Advance simulation clock and replenish tasks whose periods have expired
    pub fn advance_time(&mut self, elapsed_ns: u64) {
        self.current_time_ns += elapsed_ns;
        for t in &mut self.tasks {
            if t.current_time_ns_expired(self.current_time_ns) {
                // Period expired: replenish budget and advance absolute deadline
                t.remaining_budget_ns = t.runtime_ns;
                t.current_absolute_deadline_ns = self.current_time_ns + t.deadline_ns;
                t.is_throttled = false;
            }
        }
    }

    /// Pick task with earliest absolute deadline that has remaining runtime budget (Earliest Deadline First)
    pub fn pick_next_task(&mut self) -> Option<u32> {
        let mut earliest_idx: Option<usize> = None;
        let mut earliest_deadline = u64::MAX;

        for (idx, t) in self.tasks.iter().enumerate() {
            if !t.is_throttled && t.remaining_budget_ns > 0 {
                if t.current_absolute_deadline_ns < earliest_deadline {
                    earliest_deadline = t.current_absolute_deadline_ns;
                    earliest_idx = Some(idx);
                }
            }
        }

        earliest_idx.map(|idx| self.tasks[idx].pid)
    }

    /// Consume execution budget for currently active task
    pub fn consume_budget(&mut self, pid: u32, runtime_used_ns: u64) {
        for t in &mut self.tasks {
            if t.pid == pid {
                t.remaining_budget_ns = t.remaining_budget_ns.saturating_sub(runtime_used_ns);
                if t.remaining_budget_ns == 0 {
                    t.is_throttled = true;
                }
                break;
            }
        }
    }
}

impl DeadlineTask {
    fn current_time_ns_expired(&self, current_time: u64) -> bool {
        current_time >= self.current_absolute_deadline_ns
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deadline_scheduler_edf_cbs() {
        let mut sched = DeadlineScheduler::new(95);

        // Task A: 10ms runtime every 30ms (33% bw)
        sched
            .admit_task(DeadlineTask {
                pid: 10,
                runtime_ns: 10_000_000,
                deadline_ns: 30_000_000,
                period_ns: 30_000_000,
                remaining_budget_ns: 0,
                current_absolute_deadline_ns: 0,
                is_throttled: false,
            })
            .expect("Admit Task A");

        // Task B: 5ms runtime every 20ms (25% bw)
        sched
            .admit_task(DeadlineTask {
                pid: 20,
                runtime_ns: 5_000_000,
                deadline_ns: 20_000_000,
                period_ns: 20_000_000,
                remaining_budget_ns: 0,
                current_absolute_deadline_ns: 0,
                is_throttled: false,
            })
            .expect("Admit Task B");

        // Earliest deadline should pick Task B (deadline +20ms vs +30ms)
        assert_eq!(sched.pick_next_task(), Some(20));

        // Consume Task B budget completely -> should throttle and pick Task A
        sched.consume_budget(20, 5_000_000);
        assert_eq!(sched.pick_next_task(), Some(10));
    }
}
