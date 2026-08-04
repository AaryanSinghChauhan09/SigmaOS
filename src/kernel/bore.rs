// Burst-Oriented Response Enhancer (BORE) Scheduler for SigmaOS
// Inspired by CachyOS BORE, providing extreme responsiveness and low latency for interactive workloads.

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

/// A process controlled by the BORE scheduler
#[derive(Debug, Clone)]
pub struct BoreTask {
    pub pid: u64,
    pub name: String,
    pub cpu_runtime_ms: u64,
    pub sleep_time_ms: u64,
    pub virtual_deadline: u64,
}

impl BoreTask {
    pub fn new(pid: u64, name: &str) -> Self {
        Self {
            pid,
            name: String::from(name),
            cpu_runtime_ms: 0,
            sleep_time_ms: 0,
            virtual_deadline: 0,
        }
    }

    /// Calculates task "burstiness" score:
    /// High burstiness (CPU-hogs) gets scaled high, while interactive (sleepy) tasks stay low.
    pub fn calculate_burst_score(&self) -> u64 {
        let divisor = self.sleep_time_ms + 1;
        self.cpu_runtime_ms / divisor
    }

    /// Dynamically scales virtual deadline based on burst score to prioritize interactive tasks
    pub fn update_deadline(&mut self, current_time: u64) {
        let burst = self.calculate_burst_score();
        // BORE multiplier: penalize high-burst tasks by throwing their deadline further out
        let penalty = if burst > 50 { burst * 5 } else { burst };
        self.virtual_deadline = current_time + penalty + 10;
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

    /// Dispatches the next task with the earliest virtual deadline
    pub fn schedule(&mut self) -> Option<&BoreTask> {
        self.tasks.iter().min_by_key(|t| t.virtual_deadline)
    }

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
}
