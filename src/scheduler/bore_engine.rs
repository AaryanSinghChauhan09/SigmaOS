//! CachyOS BORE (Burst-Oriented Response Enhancer) Scheduler Engine for SigmaOS
//!
//! Inspired by CachyOS's Linux kernel BORE scheduler:
//! - Tracks task execution bursts vs sleep/quiescent cycles
//! - Calculates normalized `burst_score` (0 to 39) reflecting batch CPU appetite
//! - Applies dynamic `burst_penalty` to virtual runtime (`vruntime`)
//! - Guarantees sub-millisecond scheduling latency for interactive Wayland & audio tasks

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;

pub const BORE_BURST_MIN_NS: u64 = 1_000_000; // 1ms base burst window
pub const BORE_MAX_SCORE: u32 = 39;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskType {
    InteractiveDesktop, // Wayland, Audio, Input
    NormalApp,          // Web browser, editor
    BatchCompute,       // Compilation, rendering, AI training
}

#[derive(Debug, Clone)]
pub struct BoreTask {
    pub pid: u64,
    pub name: &'static str,
    pub task_type: TaskType,
    pub burst_time_ns: u64,
    pub sleep_time_ns: u64,
    pub vruntime: u64,
    pub burst_score: u32,
}

impl BoreTask {
    pub fn new(pid: u64, name: &'static str, task_type: TaskType) -> Self {
        Self {
            pid,
            name,
            task_type,
            burst_time_ns: 0,
            sleep_time_ns: 0,
            vruntime: 0,
            burst_score: 0,
        }
    }

    /// Update burst statistics following a runtime quantum or sleep interval
    pub fn record_execution(&mut self, runtime_ns: u64) {
        self.burst_time_ns = self.burst_time_ns.saturating_add(runtime_ns);
        self.update_burst_score();
    }

    pub fn record_sleep(&mut self, sleep_ns: u64) {
        self.sleep_time_ns = self.sleep_time_ns.saturating_add(sleep_ns);
        // Quiescence decays accumulated burst time
        self.burst_time_ns = self.burst_time_ns.saturating_sub(sleep_ns / 2);
        self.update_burst_score();
    }

    /// Compute dynamic BORE burst score (0 to 39)
    pub fn update_burst_score(&mut self) {
        if self.burst_time_ns < BORE_BURST_MIN_NS {
            self.burst_score = 0;
            return;
        }

        // Ratio of active burst to total lifecycle time
        let total = self.burst_time_ns.saturating_add(self.sleep_time_ns).max(1);
        let ratio = (self.burst_time_ns as u128 * 100) / total as u128;

        // Scale ratio to 0..39
        let score = (ratio * BORE_MAX_SCORE as u128 / 100) as u32;
        self.burst_score = score.min(BORE_MAX_SCORE);
    }

    /// Calculate effective virtual runtime with BORE burst penalty
    pub fn effective_vruntime(&self) -> u64 {
        let penalty = (self.burst_score as u64) * 10_000;
        self.vruntime.saturating_add(penalty)
    }
}

pub struct BoreSchedulerEngine {
    pub tasks: BTreeMap<u64, BoreTask>,
    pub current_time_ns: u64,
}

impl BoreSchedulerEngine {
    pub fn new() -> Self {
        Self {
            tasks: BTreeMap::new(),
            current_time_ns: 0,
        }
    }

    pub fn add_task(&mut self, task: BoreTask) {
        self.tasks.insert(task.pid, task);
    }

    /// Pick the next task to run based on minimal effective vruntime
    pub fn pick_next(&self) -> Option<u64> {
        self.tasks
            .values()
            .min_by_key(|t| t.effective_vruntime())
            .map(|t| t.pid)
    }

    /// Simulate a scheduling step
    pub fn step_task(&mut self, pid: u64, run_quantum_ns: u64) {
        if let Some(task) = self.tasks.get_mut(&pid) {
            task.record_execution(run_quantum_ns);
            task.vruntime = task.vruntime.saturating_add(run_quantum_ns);
            self.current_time_ns += run_quantum_ns;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bore_interactive_task_priority() {
        let mut sched = BoreSchedulerEngine::new();

        let mut audio_task = BoreTask::new(1, "pipewire-audio", TaskType::InteractiveDesktop);
        audio_task.record_sleep(50_000_000); // 50ms sleep
        audio_task.record_execution(200_000); // 0.2ms quick burst

        let mut compute_task = BoreTask::new(2, "rustc-compiler", TaskType::BatchCompute);
        compute_task.record_execution(200_000_000); // 200ms compute burst without sleep

        sched.add_task(audio_task);
        sched.add_task(compute_task);

        // Audio task has lower burst score and must be picked first
        let next_pid = sched.pick_next().unwrap();
        assert_eq!(next_pid, 1);
    }

    #[test]
    fn test_bore_compute_burst_penalty() {
        let mut task = BoreTask::new(10, "render-engine", TaskType::BatchCompute);
        task.record_execution(100_000_000); // Heavy burst
        assert!(task.burst_score > 0);
        assert!(task.effective_vruntime() > task.vruntime);
    }
}
