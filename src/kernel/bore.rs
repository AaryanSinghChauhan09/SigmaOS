// Burst-Oriented Response Enhancer (BORE) Scheduler for SigmaOS
// Inspired by CachyOS BORE, Linux EEVDF/CFS, and FreeBSD ULE schedulers.
// Implements starvation-avoidance (aging), nice-value weighting, and Real-Time priority lanes.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoreTaskType {
    RealTime,
    Interactive,
    Batch,
}

/// A process controlled by the BORE scheduler
#[derive(Debug, Clone)]
pub struct BoreTask {
    pub pid: u64,
    pub name: String,
    pub cpu_runtime_ms: u64,
    pub sleep_time_ms: u64,
    pub virtual_deadline: u64,
    // Open-Source Algorithmic Enhancements:
    pub nice: i8,           // Nice value: -20 (highest priority) to 19 (lowest priority)
    pub wait_ticks: u64,    // For FreeBSD-style starvation-avoidance aging
    pub is_real_time: bool, // Real-Time task bypass lane
}

impl BoreTask {
    pub fn new(pid: u64, name: &str) -> Self {
        Self {
            pid,
            name: String::from(name),
            cpu_runtime_ms: 0,
            sleep_time_ms: 0,
            virtual_deadline: 0,
            nice: 0,
            wait_ticks: 0,
            is_real_time: false,
        }
    }

    /// Sets task nice weight value (validated between -20 and 19)
    pub fn with_nice(mut self, nice: i8) -> Self {
        self.nice = nice.clamp(-20, 19);
        self
    }

    /// Sets task as a strict Real-Time task
    pub fn with_real_time(mut self, rt: bool) -> Self {
        self.is_real_time = rt;
        self
    }

    /// Calculates task "burstiness" score:
    /// High burstiness (CPU-hogs) gets scaled high, while interactive (sleepy) tasks stay low.
    pub fn calculate_burst_score(&self) -> u64 {
        let divisor = self.sleep_time_ms + 1;
        self.cpu_runtime_ms / divisor
    }

    /// Dynamically scales virtual deadline based on burst score and nice weight to prioritize interactive tasks.
    /// Incorporates starvation aging factor to maintain fairness.
    pub fn update_deadline(&mut self, current_time: u64) {
        if self.is_real_time {
            // Real-Time tasks always run on the earliest possible deadline
            self.virtual_deadline = current_time;
            return;
        }

        let burst = self.calculate_burst_score();

        // BORE multiplier: penalize high-burst tasks by throwing their deadline further out
        let mut penalty = if burst > 50 { burst * 5 } else { burst };

        // Nice-value weight adjustment (Linux CFS inspired):
        // Shift penalty higher for positive nice values (lower priority)
        // Shift penalty lower for negative nice values (higher priority)
        let nice_factor = (self.nice as i32 + 20) as u64; // Map -20..19 to 0..39
        penalty = penalty * (nice_factor + 1) / 20;

        // Apply starvation-aging offset (FreeBSD ULE inspired):
        // Highly starved tasks get deadline boosts so they are guaranteed execution
        let starvation_boost = self.wait_ticks * 4;
        let base_deadline = current_time + penalty + 10;

        self.virtual_deadline = base_deadline.saturating_sub(starvation_boost);
    }
}

/// BORE Responsive Scheduler
pub struct BoreScheduler {
    pub tasks: Vec<BoreTask>,
    pub current_time: AtomicU64,
}

impl BoreScheduler {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            current_time: AtomicU64::new(0),
        }
    }

    pub fn add_task(&mut self, mut task: BoreTask) {
        task.update_deadline(self.current_time.load(Ordering::SeqCst));
        self.tasks.push(task);
    }

    /// Dispatches the next task with the earliest virtual deadline.
    /// Real-time tasks are always dispatched first.
    pub fn schedule(&mut self) -> Option<&BoreTask> {
        if self.tasks.is_empty() {
            return None;
        }

        // First, check if there are any active Real-Time tasks
        let mut best_rt_idx: Option<usize> = None;
        for (i, task) in self.tasks.iter().enumerate() {
            if task.is_real_time {
                best_rt_idx = Some(i);
                break; // Real-Time simple bypass
            }
        }

        if let Some(idx) = best_rt_idx {
            return Some(&self.tasks[idx]);
        }

        // Otherwise, find the standard standard task with the earliest virtual deadline
        let mut best_idx = 0;
        let mut min_deadline = self.tasks[0].virtual_deadline;

        for (i, task) in self.tasks.iter().enumerate().skip(1) {
            if task.virtual_deadline < min_deadline {
                min_deadline = task.virtual_deadline;
                best_idx = i;
            }
        }

        // Increment wait_ticks for all other waiting standard tasks to prevent starvation (Aging)
        let curr_time = self.current_time.load(Ordering::SeqCst);
        for (i, task) in self.tasks.iter_mut().enumerate() {
            if i != best_idx && !task.is_real_time {
                task.wait_ticks += 1;
                task.update_deadline(curr_time);
            } else if i == best_idx {
                // Reset waiting ticks for the scheduled task
                task.wait_ticks = 0;
            }
        }

        Some(&self.tasks[best_idx])
    }

    /// Increments the global scheduler tick clock
    pub fn tick(&self) {
        self.current_time.fetch_add(1, Ordering::SeqCst);
    }

    pub fn update_task_metrics(&mut self, pid: u64, cpu_runtime: u64, sleep_time: u64) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.pid == pid) {
            task.cpu_runtime_ms += cpu_runtime;
            task.sleep_time_ms += sleep_time;
            task.update_deadline(self.current_time.load(Ordering::SeqCst));
        }
    }
}

impl Default for BoreScheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bore_task_burst_scoring() {
        let mut task_hog = BoreTask::new(1, "batch-render");
        task_hog.cpu_runtime_ms = 1000;
        task_hog.sleep_time_ms = 0; // CPU hog

        let mut task_ui = BoreTask::new(2, "mouse-trackball");
        task_ui.cpu_runtime_ms = 10;
        task_ui.sleep_time_ms = 500; // Interactive

        assert!(task_hog.calculate_burst_score() > task_ui.calculate_burst_score());

        // Hogs should have a further out deadline compared to UI/interactive tasks
        task_hog.update_deadline(100);
        task_ui.update_deadline(100);
        assert!(task_hog.virtual_deadline > task_ui.virtual_deadline);
    }

    #[test]
    fn test_bore_scheduler_dispatch() {
        let mut scheduler = BoreScheduler::new();

        let mut t1 = BoreTask::new(1, "batch-job");
        t1.cpu_runtime_ms = 200;

        let mut t2 = BoreTask::new(2, "user-input");
        t2.cpu_runtime_ms = 5;
        t2.sleep_time_ms = 100;

        scheduler.add_task(t1);
        scheduler.add_task(t2);

        // Interactive task (t2) should be scheduled first due to low burstiness & early deadline
        let scheduled = scheduler.schedule().unwrap();
        assert_eq!(scheduled.pid, 2);
    }

    #[test]
    fn test_bore_nice_values() {
        let mut scheduler = BoreScheduler::new();

        // High priority nice task (-20)
        let mut t1 = BoreTask::new(1, "high-priority").with_nice(-20);
        t1.cpu_runtime_ms = 100;
        // Low priority nice task (19)
        let mut t2 = BoreTask::new(2, "low-priority").with_nice(19);
        t2.cpu_runtime_ms = 100;

        scheduler.add_task(t1);
        scheduler.add_task(t2);

        assert!(scheduler.tasks[0].virtual_deadline < scheduler.tasks[1].virtual_deadline);
    }

    #[test]
    fn test_bore_real_time_override() {
        let mut scheduler = BoreScheduler::new();

        let t1 = BoreTask::new(1, "batch-hog").with_nice(10);
        let t2 = BoreTask::new(2, "realtime-audio").with_real_time(true);

        scheduler.add_task(t1);
        scheduler.add_task(t2);

        // The Real-Time task must be scheduled first even if standard task has smaller PID or order
        let scheduled = scheduler.schedule().unwrap();
        assert_eq!(scheduled.pid, 2);
    }

    #[test]
    fn test_bore_starvation_aging_avoidance() {
        let mut scheduler = BoreScheduler::new();

        let mut t1 = BoreTask::new(1, "batch-hog");
        t1.cpu_runtime_ms = 1000; // High penalty

        let mut t2 = BoreTask::new(2, "starved-job");
        t2.cpu_runtime_ms = 500;

        scheduler.add_task(t1);
        scheduler.add_task(t2);

        // Initially t2 scheduled because of lower cpu_runtime
        assert_eq!(scheduler.schedule().unwrap().pid, 2);

        // Run several scheduling rounds. The other task waiting gets wait_ticks increased.
        for _ in 0..10 {
            let _ = scheduler.schedule();
        }

        // Task 1's wait_ticks should have accumulated, decreasing its virtual deadline
        assert!(scheduler.tasks[0].wait_ticks > 0);
        assert_eq!(scheduler.tasks[1].wait_ticks, 0); // Task 2 ran, wait_ticks reset
    }
}
