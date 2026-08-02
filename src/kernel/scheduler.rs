#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Kernel Scheduler
// Implements EEVDF (Earliest Eligible Virtual Deadline First) scheduler

use core::time::Duration;

/// Process priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Idle = 0,
    Low = 1,
    Normal = 2,
    High = 3,
    Realtime = 4,
}

/// Process state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Running,
    Ready,
    Blocked,
    Terminated,
}

/// Process control block
#[derive(Debug, Clone)]
pub struct Process {
    pub pid: u64,
    pub name: String,
    pub priority: Priority,
    pub state: ProcessState,
    pub runtime: Duration,
    pub virtual_deadline: u64,
    pub time_slice: Duration,
    pub core_affinity: Option<usize>, // Core affinity for multi-core/HPC setups
}

impl Process {
    pub fn new(pid: u64, name: String, priority: Priority) -> Self {
        Self {
            pid,
            name,
            priority,
            state: ProcessState::Ready,
            runtime: Duration::from_secs(0),
            virtual_deadline: 0,
            time_slice: Duration::from_millis(10),
            core_affinity: None,
        }
    }

    pub fn update_virtual_deadline(&mut self, current_time: u64) {
        // EEVDF virtual deadline calculation
        let weight = match self.priority {
            Priority::Idle => 1024,
            Priority::Low => 512,
            Priority::Normal => 256,
            Priority::High => 128,
            Priority::Realtime => 64,
        };
        self.virtual_deadline = current_time + (1000 / weight);
    }
}

/// EEVDF Scheduler
pub struct Scheduler {
    pub processes: Vec<Process>,
    pub current_time: u64,
    pub is_realtime_profile: bool,
    pub is_hpc_profile: bool,
}

impl Scheduler {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            current_time: 0,
            is_realtime_profile: false,
            is_hpc_profile: false,
        }
    }

    pub fn enable_realtime_profile(&mut self, enabled: bool) {
        self.is_realtime_profile = enabled;
    }

    pub fn enable_hpc_profile(&mut self, enabled: bool) {
        self.is_hpc_profile = enabled;
    }

    pub fn add_process(&mut self, mut process: Process) {
        process.update_virtual_deadline(self.current_time);
        self.processes.push(process);
    }

    pub fn schedule(&mut self) -> Option<&Process> {
        self.schedule_on_core(0)
    }

    pub fn schedule_on_core(&self, target_core: usize) -> Option<&Process> {
        // Find process with earliest eligible virtual deadline matching core affinity
        let now = self.current_time;

        if self.is_realtime_profile {
            // Prioritize Realtime priority tasks immediately under realtime profile
            let rt_proc = self
                .processes
                .iter()
                .filter(|p| {
                    p.state == ProcessState::Ready
                        && p.priority == Priority::Realtime
                        && p.core_affinity.map_or(true, |c| c == target_core)
                })
                .min_by_key(|p| p.virtual_deadline);
            if rt_proc.is_some() {
                return rt_proc;
            }
        }

        self.processes
            .iter()
            .filter(|p| {
                p.state == ProcessState::Ready
                    && p.virtual_deadline <= now
                    && p.core_affinity.map_or(true, |c| c == target_core)
            })
            .min_by_key(|p| p.virtual_deadline)
    }

    /// Check if a higher-priority task can preempt the currently running process
    pub fn check_preemption(&self, running_pid: u64) -> bool {
        let running_proc = self.processes.iter().find(|p| p.pid == running_pid);
        let highest_ready = self
            .processes
            .iter()
            .filter(|p| p.state == ProcessState::Ready)
            .max_by_key(|p| p.priority);

        match (running_proc, highest_ready) {
            (Some(run), Some(ready)) => ready.priority > run.priority,
            (None, Some(_)) => true,
            _ => false,
        }
    }

    pub fn tick(&mut self) {
        self.current_time += 1;
    }

    pub fn set_process_state(&mut self, pid: u64, state: ProcessState) {
        if let Some(process) = self.processes.iter_mut().find(|p| p.pid == pid) {
            process.state = state;
            if state == ProcessState::Ready {
                process.update_virtual_deadline(self.current_time);
            }
        }
    }

    pub fn remove_process(&mut self, pid: u64) {
        self.processes.retain(|p| p.pid != pid);
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
        assert!(scheduler.processes.is_empty());
    }

    #[test]
    fn test_add_process() {
        let mut scheduler = Scheduler::new();
        let process = Process::new(1, "test".to_string(), Priority::Normal);
        scheduler.add_process(process);
        assert_eq!(scheduler.processes.len(), 1);
    }

    #[test]
    fn test_schedule() {
        let mut scheduler = Scheduler::new();
        let process = Process::new(1, "test".to_string(), Priority::Normal);
        scheduler.add_process(process);

        for _ in 0..5 {
            scheduler.tick();
        }

        let scheduled = scheduler.schedule();
        assert!(scheduled.is_some());
    }

    #[test]
    fn test_priority_ordering() {
        let p1 = Priority::Low;
        let p2 = Priority::High;
        assert!(p2 > p1);
    }

    #[test]
    fn test_realtime_preemption_and_core_affinity() {
        let mut scheduler = Scheduler::new();
        scheduler.enable_realtime_profile(true);

        // Process 1: Normal task on Core 0
        let mut p1 = Process::new(1, "normal_task".to_string(), Priority::Normal);
        p1.core_affinity = Some(0);
        scheduler.add_process(p1);

        // Process 2: Realtime task on Core 1
        let mut p2 = Process::new(2, "rt_task".to_string(), Priority::Realtime);
        p2.core_affinity = Some(1);
        scheduler.add_process(p2);

        // schedule on Core 1 should pick the Realtime task (Process 2)
        let scheduled_core1 = scheduler.schedule_on_core(1);
        assert!(scheduled_core1.is_some());
        assert_eq!(scheduled_core1.unwrap().pid, 2);

        // Preemption check: Higher priority (Realtime) should preempt Normal
        assert!(scheduler.check_preemption(1));
    }
}
