// SigmaOS Kernel Perf - Scheduler Profiler
// Measures scheduling latency, context switch overhead, and priority inheritance for blocked I/O threads.

use std::string::String;
use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct SchedulerLatencyEntry {
    pub tid: u32,
    pub task_name: String,
    pub wakeup_timestamp_ns: u64,
    pub dispatch_timestamp_ns: u64,
    pub latency_ns: u64,
}

#[derive(Debug, Clone)]
pub struct PriorityInheritanceRecord {
    pub blocked_thread_id: u32,
    pub original_priority: i8,
    pub lock_owner_thread_id: u32,
    pub boosted_priority: i8,
}

pub struct SchedulerProfiler {
    pub latencies: Vec<SchedulerLatencyEntry>,
    pub pi_records: Vec<PriorityInheritanceRecord>,
    pub total_context_switches: u64,
}

impl SchedulerProfiler {
    pub fn new() -> Self {
        Self {
            latencies: Vec::new(),
            pi_records: Vec::new(),
            total_context_switches: 0,
        }
    }

    pub fn record_latency(&mut self, tid: u32, task_name: &str, wakeup_ns: u64, dispatch_ns: u64) {
        let latency_ns = dispatch_ns.saturating_sub(wakeup_ns);
        self.latencies.push(SchedulerLatencyEntry {
            tid,
            task_name: String::from(task_name),
            wakeup_timestamp_ns: wakeup_ns,
            dispatch_timestamp_ns: dispatch_ns,
            latency_ns,
        });
        self.total_context_switches += 1;
    }

    pub fn record_priority_inheritance(&mut self, blocked_tid: u32, orig_prio: i8, owner_tid: u32, boosted_prio: i8) {
        self.pi_records.push(PriorityInheritanceRecord {
            blocked_thread_id: blocked_tid,
            original_priority: orig_prio,
            lock_owner_thread_id: owner_tid,
            boosted_priority: boosted_prio,
        });
    }

    pub fn max_latency_ns(&self) -> u64 {
        self.latencies.iter().map(|l| l.latency_ns).max().unwrap_or(0)
    }

    pub fn avg_latency_ns(&self) -> u64 {
        if self.latencies.is_empty() {
            return 0;
        }
        let sum: u64 = self.latencies.iter().map(|l| l.latency_ns).sum();
        sum / (self.latencies.len() as u64)
    }
}

impl Default for SchedulerProfiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler_profiler_latencies_and_pi() {
        let mut profiler = SchedulerProfiler::new();

        profiler.record_latency(101, "audio_server", 1000, 1050);
        profiler.record_latency(102, "compositor", 2000, 2100);

        assert_eq!(profiler.total_context_switches, 2);
        assert_eq!(profiler.max_latency_ns(), 100);
        assert_eq!(profiler.avg_latency_ns(), 75);

        profiler.record_priority_inheritance(103, 10, 101, 20);
        assert_eq!(profiler.pi_records.len(), 1);
        assert_eq!(profiler.pi_records[0].boosted_priority, 20);
    }
}
