// SigmaOS Round-Robin Scheduler
// Simple round-robin scheduler for time-sliced execution

use crate::kernel::scheduler::{Process, Priority, ProcessState};

/// Round-robin scheduler configuration
pub struct RoundRobinConfig {
    pub time_slice: u64,
    pub max_processes: usize,
}

impl Default for RoundRobinConfig {
    fn default() -> Self {
        Self {
            time_slice: 10, // 10ms time slice
            max_processes: 1024,
        }
    }
}

/// Round-robin scheduler
pub struct RoundRobinScheduler {
    processes: Vec<Process>,
    current_index: usize,
    config: RoundRobinConfig,
    current_time: u64,
}

impl RoundRobinScheduler {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            current_index: 0,
            config: RoundRobinConfig::default(),
            current_time: 0,
        }
    }

    pub fn with_config(config: RoundRobinConfig) -> Self {
        Self {
            processes: Vec::new(),
            current_index: 0,
            config,
            current_time: 0,
        }
    }

    pub fn add_process(&mut self, process: Process) -> Result<(), SchedulerError> {
        if self.processes.len() >= self.config.max_processes {
            return Err(SchedulerError::TooManyProcesses);
        }
        self.processes.push(process);
        Ok(())
    }

    pub fn schedule(&mut self) -> Option<&Process> {
        if self.processes.is_empty() {
            return None;
        }

        // Find next ready process
        let start_index = self.current_index;
        loop {
            if self.processes[self.current_index].state == ProcessState::Ready {
                return Some(&self.processes[self.current_index]);
            }
            
            self.current_index = (self.current_index + 1) % self.processes.len();
            
            // If we've looped through all processes
            if self.current_index == start_index {
                return None;
            }
        }
    }

    pub fn tick(&mut self) {
        self.current_time += 1;
        
        // Time slice expired, move to next process
        if self.current_time % self.config.time_slice == 0 {
            self.current_index = (self.current_index + 1) % self.processes.len();
        }
    }

    pub fn set_process_state(&mut self, pid: u64, state: ProcessState) {
        if let Some(process) = self.processes.iter_mut().find(|p| p.pid == pid) {
            process.state = state;
        }
    }

    pub fn remove_process(&mut self, pid: u64) {
        self.processes.retain(|p| p.pid != pid);
        
        // Adjust current index if necessary
        if self.current_index >= self.processes.len() && !self.processes.is_empty() {
            self.current_index = 0;
        }
    }

    pub fn get_process_count(&self) -> usize {
        self.processes.len()
    }

    pub fn get_ready_process_count(&self) -> usize {
        self.processes.iter().filter(|p| p.state == ProcessState::Ready).count()
    }
}

impl Default for RoundRobinScheduler {
    fn default() -> Self {
        Self::new()
    }
}

/// Scheduler errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchedulerError {
    TooManyProcesses,
    ProcessNotFound,
    InvalidState,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundrobin_creation() {
        let scheduler = RoundRobinScheduler::new();
        assert_eq!(scheduler.get_process_count(), 0);
    }

    #[test]
    fn test_add_process() {
        let mut scheduler = RoundRobinScheduler::new();
        let process = Process::new(1, "test".to_string(), Priority::Normal);
        assert!(scheduler.add_process(process).is_ok());
        assert_eq!(scheduler.get_process_count(), 1);
    }

    #[test]
    fn test_schedule() {
        let mut scheduler = RoundRobinScheduler::new();
        let process = Process::new(1, "test".to_string(), Priority::Normal);
        scheduler.add_process(process).unwrap();
        
        let scheduled = scheduler.schedule();
        assert!(scheduled.is_some());
    }

    #[test]
    fn test_tick() {
        let mut scheduler = RoundRobinScheduler::new();
        let process1 = Process::new(1, "test1".to_string(), Priority::Normal);
        let process2 = Process::new(2, "test2".to_string(), Priority::Normal);
        scheduler.add_process(process1).unwrap();
        scheduler.add_process(process2).unwrap();
        
        let initial_index = scheduler.current_index;
        for _ in 0..15 {
            scheduler.tick();
        }
        // After 15 ticks with 10ms time slice, index should change (and not cycle back to 0)
        assert_ne!(scheduler.current_index, initial_index);
    }

    #[test]
    fn test_remove_process() {
        let mut scheduler = RoundRobinScheduler::new();
        let process = Process::new(1, "test".to_string(), Priority::Normal);
        scheduler.add_process(process).unwrap();
        scheduler.remove_process(1);
        assert_eq!(scheduler.get_process_count(), 0);
    }

    #[test]
    fn test_max_processes() {
        let config = RoundRobinConfig {
            time_slice: 10,
            max_processes: 2,
        };
        let mut scheduler = RoundRobinScheduler::with_config(config);
        
        let process1 = Process::new(1, "test1".to_string(), Priority::Normal);
        let process2 = Process::new(2, "test2".to_string(), Priority::Normal);
        let process3 = Process::new(3, "test3".to_string(), Priority::Normal);
        
        assert!(scheduler.add_process(process1).is_ok());
        assert!(scheduler.add_process(process2).is_ok());
        assert!(scheduler.add_process(process3).is_err());
    }
}
