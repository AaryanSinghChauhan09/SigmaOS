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
    processes: Vec<Process>,
    current_time: u64,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            current_time: 0,
        }
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
}
