// SPDX-License-Identifier: MIT
/// SigmaOS: Process Scheduler (EEVDF - Earliest Eligible Virtual Deadline First)
/// Implements fair, preemptible scheduling with time-slice management
use super::manager::{Priority, ProcessInfo, ProcessState};
use core::fmt;
use std::collections::VecDeque;
use std::vec::Vec;

/// Scheduling Statistics
#[derive(Debug, Clone, Default)]
pub struct SchedulingStats {
    pub total_context_switches: u64,
    pub total_preemptions: u64,
    pub average_wait_time_ms: u64,
    pub average_run_time_ms: u64,
}

/// Time Slice (quantum) in milliseconds
pub const DEFAULT_TIME_SLICE_MS: u64 = 10;
pub const MIN_TIME_SLICE_MS: u64 = 1;
pub const MAX_TIME_SLICE_MS: u64 = 100;

/// Virtual Runtime (VRT) - used for fair scheduling
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct VirtualRuntime {
    pub value: u64, // in nanoseconds
}

impl VirtualRuntime {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn add_ms(&mut self, ms: u64) {
        self.value += ms * 1_000_000; // Convert ms to ns
    }

    pub fn add_ns(&mut self, ns: u64) {
        self.value += ns;
    }
}

impl Default for VirtualRuntime {
    fn default() -> Self {
        Self::new()
    }
}

/// Scheduling Queue Entry
#[derive(Debug, Clone)]
pub struct QueueEntry {
    pub pid: u32,
    pub vruntime: VirtualRuntime,
    pub time_slice_remaining: u64,
    pub priority: Priority,
}

/// EEVDF Scheduler
pub struct Scheduler {
    ready_queue: VecDeque<QueueEntry>,
    current_process: Option<u32>,
    virtual_time: VirtualRuntime,
    stats: SchedulingStats,
    total_weight: f64, // For weighted fair queueing
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
            current_process: None,
            virtual_time: VirtualRuntime::new(),
            stats: SchedulingStats::default(),
            total_weight: 0.0,
        }
    }

    /// Get weight for priority level
    fn get_weight(priority: Priority) -> f64 {
        match priority {
            Priority::RealTime => 4.0,
            Priority::High => 2.0,
            Priority::Normal => 1.0,
            Priority::Low => 0.5,
        }
    }

    /// Add process to ready queue
    pub fn add_process(&mut self, pid: u32, priority: Priority) {
        let time_slice = match priority {
            Priority::RealTime => 50,
            Priority::High => 20,
            Priority::Normal => DEFAULT_TIME_SLICE_MS,
            Priority::Low => 5,
        };

        let entry = QueueEntry {
            pid,
            vruntime: VirtualRuntime::new(),
            time_slice_remaining: time_slice,
            priority,
        };

        self.total_weight += Self::get_weight(priority);
        self.ready_queue.push_back(entry);
    }

    /// Remove process from ready queue
    pub fn remove_process(&mut self, pid: u32) -> Option<QueueEntry> {
        if let Some(pos) = self.ready_queue.iter().position(|e| e.pid == pid) {
            let entry = self.ready_queue.remove(pos).unwrap();
            self.total_weight -= Self::get_weight(entry.priority);
            Some(entry)
        } else {
            None
        }
    }

    /// Schedule next process (EEVDF algorithm)
    pub fn schedule_next(&mut self) -> Option<u32> {
        if self.ready_queue.is_empty() {
            self.current_process = None;
            return None;
        }

        // Find process with earliest virtual deadline
        let mut earliest_idx = 0;
        let mut earliest_deadline = self.ready_queue[0].vruntime;

        for (i, entry) in self.ready_queue.iter().enumerate() {
            if entry.vruntime < earliest_deadline {
                earliest_idx = i;
                earliest_deadline = entry.vruntime;
            }
        }

        // Move selected process to front (simulate removal and reinsertion at back after time slice)
        let mut entry = self.ready_queue.remove(earliest_idx).unwrap();

        // Update virtual runtime and assign time slice
        let weight = Self::get_weight(entry.priority);
        entry.time_slice_remaining = (DEFAULT_TIME_SLICE_MS as f64 * weight / 1.0) as u64;

        self.current_process = Some(entry.pid);
        let pid = entry.pid;

        // Re-enqueue if time slice remaining
        self.ready_queue.push_back(entry);

        Some(pid)
    }

    /// Perform context switch
    pub fn context_switch(&mut self, elapsed_ms: u64) -> Option<u32> {
        // Update virtual time
        self.virtual_time.add_ms(elapsed_ms);

        // Update current process virtual runtime
        if let Some(pid) = self.current_process {
            if let Some(pos) = self.ready_queue.iter().position(|e| e.pid == pid) {
                let weight = Self::get_weight(self.ready_queue[pos].priority);
                let weight_factor = 1.0 / weight; // Inverse weight for fair queueing
                self.ready_queue[pos]
                    .vruntime
                    .add_ns((elapsed_ms * 1_000_000) as u64 / weight as u64);
                self.ready_queue[pos].time_slice_remaining = self.ready_queue[pos]
                    .time_slice_remaining
                    .saturating_sub(elapsed_ms);
            }
        }

        self.stats.total_context_switches += 1;

        // Schedule next process
        self.schedule_next()
    }

    /// Preempt current process and schedule next
    pub fn preempt(&mut self) -> Option<u32> {
        self.stats.total_preemptions += 1;
        self.schedule_next()
    }

    /// Get statistics
    pub fn get_stats(&self) -> SchedulingStats {
        self.stats.clone()
    }

    /// Get queue length
    pub fn queue_length(&self) -> usize {
        self.ready_queue.len()
    }

    /// Get current process
    pub fn current(&self) -> Option<u32> {
        self.current_process
    }

    /// Get ready queue (for inspection)
    pub fn get_ready_queue(&self) -> Vec<(u32, Priority, u64)> {
        self.ready_queue
            .iter()
            .map(|e| (e.pid, e.priority, e.time_slice_remaining))
            .collect()
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler_creation() {
        let scheduler = Scheduler::new();
        assert_eq!(scheduler.queue_length(), 0);
        assert_eq!(scheduler.current(), None);
    }

    #[test]
    fn test_add_process() {
        let mut scheduler = Scheduler::new();
        scheduler.add_process(1, Priority::Normal);

        assert_eq!(scheduler.queue_length(), 1);
        let next = scheduler.schedule_next();
        assert_eq!(next, Some(1));
    }

    #[test]
    fn test_multiple_processes() {
        let mut scheduler = Scheduler::new();
        scheduler.add_process(1, Priority::Normal);
        scheduler.add_process(2, Priority::High);
        scheduler.add_process(3, Priority::Low);

        assert_eq!(scheduler.queue_length(), 3);

        // First schedule should pick one
        let first = scheduler.schedule_next();
        assert!(first.is_some());
    }

    #[test]
    fn test_context_switch() {
        let mut scheduler = Scheduler::new();
        scheduler.add_process(1, Priority::Normal);
        scheduler.add_process(2, Priority::Normal);

        let first = scheduler.schedule_next().unwrap();
        let second = scheduler.context_switch(10).unwrap();

        // Should alternate between processes
        assert_ne!(first, second);
    }

    #[test]
    fn test_remove_process() {
        let mut scheduler = Scheduler::new();
        scheduler.add_process(1, Priority::Normal);
        scheduler.add_process(2, Priority::Normal);

        let removed = scheduler.remove_process(1);
        assert!(removed.is_some());
        assert_eq!(scheduler.queue_length(), 1);
    }

    #[test]
    fn test_priority_weight() {
        assert!(Scheduler::get_weight(Priority::RealTime) > Scheduler::get_weight(Priority::High));
        assert!(Scheduler::get_weight(Priority::High) > Scheduler::get_weight(Priority::Normal));
        assert!(Scheduler::get_weight(Priority::Normal) > Scheduler::get_weight(Priority::Low));
    }

    #[test]
    fn test_statistics() {
        let mut scheduler = Scheduler::new();
        scheduler.add_process(1, Priority::Normal);

        scheduler.schedule_next();
        assert_eq!(scheduler.get_stats().total_context_switches, 0);

        scheduler.context_switch(10);
        assert_eq!(scheduler.get_stats().total_context_switches, 1);

        scheduler.preempt();
        assert_eq!(scheduler.get_stats().total_preemptions, 1);
    }
}
