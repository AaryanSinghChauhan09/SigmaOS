// CachyOS BORE (Burst-Oriented Response Enhancer) Scheduler
// Interactive CPU scheduling tuner inspired by CachyOS BORE EEVDF kernel improvements

use std::collections::HashMap;

/// BORE Task Burstiness State tracking CPU burst vs I/O sleep duration
#[derive(Debug, Clone)]
pub struct BoreTaskMetrics {
    pub pid: usize,
    pub name: String,
    pub burst_time_ns: u64,
    pub sleep_time_ns: u64,
    pub interactivity_score: u32, // 0 (heavy background batch) to 100 (ultra-interactive UI)
}

impl BoreTaskMetrics {
    pub fn new(pid: usize, name: &str) -> Self {
        Self {
            pid,
            name: name.to_string(),
            burst_time_ns: 0,
            sleep_time_ns: 1_000_000,
            interactivity_score: 50,
        }
    }

    /// Calculate dynamic interactivity score based on ratio of sleep to burst time
    pub fn update_score(&mut self, cpu_burst_ns: u64, sleep_ns: u64) {
        self.burst_time_ns = cpu_burst_ns;
        self.sleep_time_ns = sleep_ns;

        if cpu_burst_ns == 0 {
            self.interactivity_score = 100;
            return;
        }

        // Higher sleep ratio relative to CPU burst indicates interactive user application
        let ratio = (sleep_ns as f64) / (cpu_burst_ns as f64);
        let score = (ratio * 20.0).clamp(0.0, 100.0) as u32;
        self.interactivity_score = score;
    }

    /// Calculate BORE deadline adjustment factor (lower latency multiplier for interactive tasks)
    pub fn deadline_multiplier(&self) -> f64 {
        if self.interactivity_score >= 80 {
            0.5 // Ultra-low latency deadline priority boost
        } else if self.interactivity_score >= 50 {
            1.0 // Standard CFS/EEVDF deadline
        } else {
            2.0 // Background batch task (penalized latency to prevent UI stuttering)
        }
    }
}

/// CachyOS BORE Scheduler Governor
#[derive(Debug, Clone)]
pub struct BoreSchedulerGovernor {
    pub tasks: HashMap<usize, BoreTaskMetrics>,
    pub base_granularity_ms: u64,
    pub is_enabled: bool,
}

impl BoreSchedulerGovernor {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            base_granularity_ms: 3,
            is_enabled: true,
        }
    }

    /// Register a task under BORE scheduling
    pub fn register_task(&mut self, pid: usize, name: &str) {
        self.tasks.insert(pid, BoreTaskMetrics::new(pid, name));
    }

    /// Record task CPU burst execution and sleep intervals
    pub fn record_burst(&mut self, pid: usize, cpu_burst_ns: u64, sleep_ns: u64) {
        if let Some(task) = self.tasks.get_mut(&pid) {
            task.update_score(cpu_burst_ns, sleep_ns);
        }
    }

    /// Get adjusted scheduling quantum for task based on BORE interactivity score
    pub fn get_adjusted_quantum_ms(&self, pid: usize) -> u64 {
        if !self.is_enabled {
            return self.base_granularity_ms;
        }

        if let Some(task) = self.tasks.get(&pid) {
            let mult = task.deadline_multiplier();
            ((self.base_granularity_ms as f64) * mult).max(1.0) as u64
        } else {
            self.base_granularity_ms
        }
    }

    /// Summary of current BORE scheduler interactivity scores
    pub fn summary(&self) -> String {
        let count = self.tasks.len();
        let mut interactive_count = 0;
        for t in self.tasks.values() {
            if t.interactivity_score >= 70 {
                interactive_count += 1;
            }
        }
        format!(
            "CachyOS BORE Scheduler: Active ({}/{} interactive tasks prioritized)",
            interactive_count, count
        )
    }
}

impl Default for BoreSchedulerGovernor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bore_interactivity_scoring() {
        let mut governor = BoreSchedulerGovernor::new();
        governor.register_task(100, "firefox");
        governor.register_task(200, "ffmpeg");

        // Firefox: frequent short CPU bursts (5ms) and long I/O sleeps (100ms)
        governor.record_burst(100, 5_000_000, 100_000_000);

        // FFmpeg: long continuous CPU bursts (200ms) and minimal sleep (2ms)
        governor.record_burst(200, 200_000_000, 2_000_000);

        let firefox_quantum = governor.get_adjusted_quantum_ms(100);
        let ffmpeg_quantum = governor.get_adjusted_quantum_ms(200);

        assert!(firefox_quantum < ffmpeg_quantum);
        assert!(governor.summary().contains("active"));
    }
}
