#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
//! EEVDF (Earliest Eligible Virtual Deadline First) Task Scheduler Subsystem
//! Implements modern Linux-inspired virtual runtime tracking, lag computation, and latency-nice weighting.

pub const MAX_SCHED_TASKS: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EevdfTask {
    pub id: usize,
    pub weight: u32,
    pub virtual_runtime: u64,
    pub actual_runtime: u64,
    pub lag: i64,
    pub latency_nice: i8, // -20 to 19 (lower means smaller, more frequent slices)
    pub virtual_deadline: u64,
    pub eligible: bool,
}

pub struct EevdfScheduler {
    pub tasks: [Option<EevdfTask>; MAX_SCHED_TASKS],
    pub avg_virtual_runtime: u64,
}

impl EevdfScheduler {
    pub fn new() -> Self {
        Self {
            tasks: [None; MAX_SCHED_TASKS],
            avg_virtual_runtime: 0,
        }
    }

    /// Add a new task to the scheduler queue
    pub fn add_task(
        &mut self,
        id: usize,
        weight: u32,
        latency_nice: i8,
    ) -> Result<(), &'static str> {
        for slot in &mut self.tasks {
            if slot.is_none() {
                *slot = Some(EevdfTask {
                    id,
                    weight: weight.max(1),
                    virtual_runtime: self.avg_virtual_runtime,
                    actual_runtime: 0,
                    lag: 0,
                    latency_nice,
                    virtual_deadline: 0,
                    eligible: true,
                });
                return Ok(());
            }
        }
        Err("Scheduler task table full")
    }

    /// Update lag and determine eligibility based on system average virtual runtime
    pub fn update_lag_and_eligibility(&mut self) {
        // Calculate average virtual runtime among all active tasks
        let mut total_vruntime = 0u64;
        let mut count = 0;
        for slot in &self.tasks {
            if let Some(ref task) = slot {
                total_vruntime += task.virtual_runtime;
                count += 1;
            }
        }

        if count > 0 {
            self.avg_virtual_runtime = total_vruntime / count;
        }

        // Lag = expected - actual (in virtual runtime space)
        // Task is eligible to run if its virtual runtime is less than or equal to the average (lag >= 0)
        for slot in &mut self.tasks {
            if let Some(ref mut task) = slot {
                task.lag = self.avg_virtual_runtime as i64 - task.virtual_runtime as i64;
                task.eligible = task.lag >= 0;
            }
        }
    }

    /// Compute virtual deadlines for eligible tasks
    /// virtual_deadline = virtual_runtime + (time_slice / weight)
    /// Time-slice is scaled by latency_nice values (lower latency_nice -> smaller time_slice)
    pub fn calculate_deadlines(&mut self) {
        for slot in &mut self.tasks {
            if let Some(ref mut task) = slot {
                if task.eligible {
                    // Baseline time_slice = 10ms (10000 microseconds)
                    let base_slice = 10000u32;
                    // Scale time slice based on latency_nice
                    let nice_multiplier = if task.latency_nice < 0 {
                        // Interactive tasks get smaller slices to preempt quickly and reduce latency
                        1.0 + (task.latency_nice as f32 / 20.0) // ranges from 0.05 to 1.0
                    } else {
                        1.0 + (task.latency_nice as f32 * 0.5) // ranges from 1.0 to 10.5
                    };

                    let allocated_slice = (base_slice as f32 * nice_multiplier) as u32;
                    let virtual_duration = (allocated_slice / task.weight) as u64;

                    task.virtual_deadline = task.virtual_runtime + virtual_duration;
                }
            }
        }
    }

    /// Select the next task with the earliest virtual deadline among eligible tasks
    pub fn select_next_task(&mut self) -> Option<usize> {
        self.update_lag_and_eligibility();
        self.calculate_deadlines();

        let mut earliest_deadline = u64::MAX;
        let mut best_task_idx = None;

        for (idx, slot) in self.tasks.iter().enumerate() {
            if let Some(ref task) = slot {
                if task.eligible && task.virtual_deadline < earliest_deadline {
                    earliest_deadline = task.virtual_deadline;
                    best_task_idx = Some(idx);
                }
            }
        }

        // If no tasks are eligible (all are ahead of average), pick the one with the highest lag (least runtime)
        if best_task_idx.is_none() {
            let mut highest_lag = i64::MIN;
            for (idx, slot) in self.tasks.iter().enumerate() {
                if let Some(ref task) = slot {
                    if task.lag > highest_lag {
                        highest_lag = task.lag;
                        best_task_idx = Some(idx);
                    }
                }
            }
        }

        best_task_idx.map(|idx| self.tasks[idx].unwrap().id)
    }

    /// Simulate execution of a task
    pub fn execute_task(&mut self, id: usize, duration: u32) -> bool {
        for slot in &mut self.tasks {
            if let Some(ref mut task) = slot {
                if task.id == id {
                    task.actual_runtime += duration as u64;
                    // virtual_runtime advances scaled inversely by task weight
                    task.virtual_runtime += (duration / task.weight) as u64;
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_eevdf_scheduler_eligibility_and_deadlines() {
        let mut sched = EevdfScheduler::new();

        // Add two tasks: Task 1 has weight 1, Task 2 has weight 4 (high priority)
        sched.add_task(1, 1, 0).unwrap();
        sched.add_task(2, 4, 0).unwrap();

        // Initially both tasks are eligible as virtual_runtime is 0
        let next = sched.select_next_task().unwrap();
        // Task 2 should be selected because it has higher weight (larger virtual deadline priority)
        assert_eq!(next, 2);

        // Simulate executing Task 2
        sched.execute_task(2, 100);

        // Re-evaluate: Task 2 has executed, so its virtual runtime advanced.
        // Task 1 now has less virtual runtime, so it becomes the earliest deadline.
        let next_again = sched.select_next_task().unwrap();
        assert_eq!(next_again, 1);
    }

    #[test]
    fn test_eevdf_latency_nice_scaling() {
        let mut sched = EevdfScheduler::new();

        // Add an interactive task (low nice) and a batch task (high nice) with equal weight
        sched.add_task(10, 2, -10).unwrap(); // interactive
        sched.add_task(20, 2, 10).unwrap(); // batch

        sched.update_lag_and_eligibility();
        sched.calculate_deadlines();

        let task_interactive = sched.tasks[0].unwrap();
        let task_batch = sched.tasks[1].unwrap();

        // The interactive task gets a smaller deadline offset because of its latency_nice value (-10)
        assert!(task_interactive.virtual_deadline < task_batch.virtual_deadline);
    }
}
