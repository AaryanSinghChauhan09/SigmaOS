// SigmaOS Kernel Scheduler
// Implements EEVDF (Earliest Eligible Virtual Deadline First) scheduler
// Enhanced with CachyOS-style BORE (Burst-Oriented Response Enhancer) responsiveness tuning

use core::time::Duration;
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

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
    pub sleep_count: u64, // BORE tracking for interactive sleep cycles
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
            sleep_count: 0,
        }
    }

    pub fn update_virtual_deadline(&mut self, current_time: u64) {
        // EEVDF virtual deadline calculation
        let weight: u64 = match self.priority {
            Priority::Idle => 1024,
            Priority::Low => 512,
            Priority::Normal => 256,
            Priority::High => 128,
            Priority::Realtime => 64,
        };

        // BORE-style burstiness responsive deadline bonus
        // Bursty tasks (high sleep count) get a scheduling bonus (up to 50% deadline reduction)
        let bore_bonus = (self.sleep_count * 5).min(50);
        let base_delay = 1000 / weight;
        let adjusted_delay = base_delay.saturating_sub(bore_bonus);

        self.virtual_deadline = current_time + adjusted_delay;
    }
}

/// EEVDF Scheduler
pub struct Scheduler {
    processes: Vec<Process>,
    current_time: u64,
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
        // Find process with earliest eligible virtual deadline
        let now = self.current_time;
        self.processes
            .iter()
            .filter(|p| p.state == ProcessState::Ready && p.virtual_deadline <= now)
            .min_by_key(|p| p.virtual_deadline)
    }

    pub fn tick(&mut self) {
        self.current_time += 1;
    }

    pub fn set_process_state(&mut self, pid: u64, state: ProcessState) {
        if let Some(process) = self.processes.iter_mut().find(|p| p.pid == pid) {
            process.state = state;
            if state == ProcessState::Blocked {
                // Task yielded or blocked (interactive behavior) -> increment sleep_count for BORE bonus
                process.sleep_count = process.sleep_count.saturating_add(1);
            }
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
    fn test_bore_responsiveness_bonus() {
        let mut scheduler = Scheduler::new();

        // 1. Create a CPU-bound process (never sleeps, sleep_count = 0)
        let cpu_process = Process::new(1, "cpu_worker".to_string(), Priority::Normal);
        scheduler.add_process(cpu_process);

        // 2. Create an interactive process (sleeps frequently)
        let mut interactive_process = Process::new(2, "compositor".to_string(), Priority::Normal);
        // Simulate multiple sleep/yield events
        interactive_process.sleep_count = 10;
        scheduler.add_process(interactive_process);

        // Interactive process should have a closer/earlier virtual deadline due to BORE bonus
        let p_cpu = scheduler.processes.iter().find(|p| p.pid == 1).unwrap();
        let p_inter = scheduler.processes.iter().find(|p| p.pid == 2).unwrap();

        assert!(p_inter.virtual_deadline < p_cpu.virtual_deadline);
    }
}
