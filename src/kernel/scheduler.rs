// SigmaOS Kernel Scheduler
// Implements EEVDF (Earliest Eligible Virtual Deadline First) scheduler

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
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
        // Single-pass scan to locate the process with the earliest eligible virtual deadline,
        // eliminating multi-pass vector iterations when evaluating realtime vs standard processes.
        let now = self.current_time;
        let mut best_rt: Option<&Process> = None;
        let mut best_eligible: Option<&Process> = None;

        for proc in &self.processes {
            if proc.state != ProcessState::Ready {
                continue;
            }
            if !proc.core_affinity.map_or(true, |c| c == target_core) {
                continue;
            }

            if self.is_realtime_profile && proc.priority == Priority::Realtime {
                if best_rt.map_or(true, |best| proc.virtual_deadline < best.virtual_deadline) {
                    best_rt = Some(proc);
                }
            }

            if proc.virtual_deadline <= now {
                if best_eligible.map_or(true, |best| proc.virtual_deadline < best.virtual_deadline) {
                    best_eligible = Some(proc);
                }
            }
        }

        if self.is_realtime_profile && best_rt.is_some() {
            best_rt
        } else {
            best_eligible
        }
    }

    /// Check if a higher-priority task can preempt the currently running process
    pub fn check_preemption(&self, running_pid: u64) -> bool {
        // Single-pass scan to locate the running process and highest priority ready process simultaneously
        let mut running_proc: Option<&Process> = None;
        let mut highest_ready: Option<&Process> = None;

        for proc in &self.processes {
            if proc.pid == running_pid {
                running_proc = Some(proc);
            }
            if proc.state == ProcessState::Ready {
                if highest_ready.map_or(true, |best| proc.priority > best.priority) {
                    highest_ready = Some(proc);
                }
            }
        }

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
