// SigmaOS Round-Robin Scheduler
// Enhanced priority-aware round-robin with process yielding and context tracking

use crate::kernel::scheduler::{Priority, Process, ProcessState};

/// CPU register context saved during a context switch
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CpuContext {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub rflags: u64,
}

impl CpuContext {
    pub fn new() -> Self {
        Self::default()
    }

    /// Simulate saving a context (in a real OS this would be done in assembly)
    pub fn save_from(&mut self, rsp: u64, rip: u64) {
        self.rsp = rsp;
        self.rip = rip;
    }
}

/// Extended process entry that includes context and yields tracking
#[derive(Debug, Clone)]
pub struct ScheduledProcess {
    pub process: Process,
    pub context: CpuContext,
    pub yield_requested: bool,
    pub cpu_time_used: u64,
    pub ticks_since_run: u64,
    pub original_priority: Priority,
}

impl ScheduledProcess {
    pub fn new(process: Process) -> Self {
        let original_priority = process.priority;
        Self {
            process,
            context: CpuContext::new(),
            yield_requested: false,
            cpu_time_used: 0,
            ticks_since_run: 0,
            original_priority,
        }
    }

    /// Request this process to yield the CPU voluntarily
    pub fn request_yield(&mut self) {
        self.yield_requested = true;
    }

    /// Priority-based weight: higher priority gets a larger time slice multiplier
    pub fn time_slice_ticks(&self, base_slice: u64) -> u64 {
        let mut multiplier: u64 = match self.process.priority {
            Priority::Realtime => 8,
            Priority::High => 4,
            Priority::Normal => 2,
            Priority::Low => 1,
            Priority::Idle => 1, // Idle still gets a minimal slice
        };
        // Dynamic boost if process has been starved (aged)
        if self.ticks_since_run > 50 {
            multiplier += 2;
        }
        base_slice * multiplier
    }
}

/// Round-robin scheduler configuration
pub struct RoundRobinConfig {
    pub time_slice: u64,
    pub max_processes: usize,
}

impl Default for RoundRobinConfig {
    fn default() -> Self {
        Self {
            time_slice: 10, // 10ms base time slice
            max_processes: 1024,
        }
    }
}

/// Enhanced priority-aware round-robin scheduler
pub struct RoundRobinScheduler {
    processes: Vec<ScheduledProcess>,
    pub current_index: usize,
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
        self.processes.push(ScheduledProcess::new(process));
        Ok(())
    }

    pub fn schedule(&mut self) -> Option<&Process> {
        if self.processes.is_empty() {
            return None;
        }

        let start_index = self.current_index;
        loop {
            if self.processes[self.current_index].process.state == ProcessState::Ready {
                return Some(&self.processes[self.current_index].process);
            }
            self.current_index = (self.current_index + 1) % self.processes.len();
            if self.current_index == start_index {
                return None;
            }
        }
    }

    pub fn tick(&mut self) {
        self.current_time += 1;

        if self.processes.is_empty() {
            return;
        }

        // Safeguard current_index boundaries to prevent any out of bounds panic
        if self.current_index >= self.processes.len() {
            self.current_index = 0;
        }

        // Age other ready processes to prevent starvation (Linux/distro priority aging simulation)
        for (i, entry) in self.processes.iter_mut().enumerate() {
            if i != self.current_index && entry.process.state == ProcessState::Ready {
                entry.ticks_since_run += 1;
                // If extremely starved, temporarily promote priority to prevent starvation
                if entry.ticks_since_run > 100 && entry.process.priority == Priority::Low {
                    entry.process.priority = Priority::Normal;
                }
            }
        }

        let needs_switch = {
            let entry = &mut self.processes[self.current_index];
            entry.cpu_time_used += 1;
            entry.ticks_since_run = 0; // reset aging count
            // Demote back to original priority after getting its turn
            entry.process.priority = entry.original_priority;
            let slice = entry.time_slice_ticks(self.config.time_slice);
            let yielding = entry.yield_requested;
            entry.yield_requested = false;
            yielding || entry.cpu_time_used.is_multiple_of(slice)
        };

        if needs_switch {
            self.advance_to_next_ready();
        }
    }

    /// Move to the next ready process
    fn advance_to_next_ready(&mut self) {
        if self.processes.is_empty() {
            return;
        }
        let start = self.current_index;
        loop {
            self.current_index = (self.current_index + 1) % self.processes.len();
            if self.processes[self.current_index].process.state == ProcessState::Ready {
                break;
            }
            if self.current_index == start {
                break;
            }
        }
    }

    /// Request the current running process to yield on next tick
    pub fn yield_current(&mut self) {
        if self.processes.is_empty() {
            return;
        }
        self.processes[self.current_index].request_yield();
    }

    pub fn set_process_state(&mut self, pid: u64, state: ProcessState) {
        if let Some(entry) = self.processes.iter_mut().find(|e| e.process.pid == pid) {
            entry.process.state = state;
        }
    }

    pub fn remove_process(&mut self, pid: u64) {
        self.processes.retain(|e| e.process.pid != pid);
        if self.current_index >= self.processes.len() && !self.processes.is_empty() {
            self.current_index = 0;
        }
    }

    pub fn get_process_count(&self) -> usize {
        self.processes.len()
    }

    pub fn get_ready_process_count(&self) -> usize {
        self.processes
            .iter()
            .filter(|e| e.process.state == ProcessState::Ready)
            .count()
    }

    /// Save the context of the currently running process
    pub fn save_context(&mut self, rsp: u64, rip: u64) {
        if let Some(entry) = self.processes.get_mut(self.current_index) {
            entry.context.save_from(rsp, rip);
        }
    }

    /// Restore the context of the currently scheduled process
    pub fn restore_context(&self) -> Option<CpuContext> {
        self.processes.get(self.current_index).map(|e| e.context)
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
        assert!(scheduler.schedule().is_some());
    }

    #[test]
    fn test_tick_switches_process() {
        let mut scheduler = RoundRobinScheduler::new();
        let process1 = Process::new(1, "test1".to_string(), Priority::Normal);
        let process2 = Process::new(2, "test2".to_string(), Priority::Normal);
        scheduler.add_process(process1).unwrap();
        scheduler.add_process(process2).unwrap();

        for _ in 0..15 {
            scheduler.tick();
        }
        // After 15 ticks with 10ms time slice, index should change (and not cycle back to 0)
        let process1 = Process::new(1, "test1".to_string(), Priority::Normal);
        let process2 = Process::new(2, "test2".to_string(), Priority::Normal);
        scheduler.add_process(process1).unwrap();
        scheduler.add_process(process2).unwrap();

        let initial_index = scheduler.current_index;
        // Normal priority multiplier is 2x base 10 = 20 ticks per slice
        for _ in 0..20 {
            scheduler.tick();
        }
        assert_ne!(scheduler.current_index, initial_index);
    }

    #[test]
    fn test_yield_current() {
        let mut scheduler = RoundRobinScheduler::new();
        let p1 = Process::new(1, "test1".to_string(), Priority::High);
        let p2 = Process::new(2, "test2".to_string(), Priority::Normal);
        scheduler.add_process(p1).unwrap();
        scheduler.add_process(p2).unwrap();

        let initial_index = scheduler.current_index;
        scheduler.yield_current();
        scheduler.tick(); // triggers the switch
        assert_ne!(scheduler.current_index, initial_index);
    }

    #[test]
    fn test_context_save_restore() {
        let mut scheduler = RoundRobinScheduler::new();
        let process = Process::new(1, "test".to_string(), Priority::Normal);
        scheduler.add_process(process).unwrap();

        scheduler.save_context(0xDEADBEEF, 0xCAFEBABE);
        let ctx = scheduler.restore_context().unwrap();
        assert_eq!(ctx.rsp, 0xDEAD_BEEF);
        assert_eq!(ctx.rip, 0xCAFE_BABE);
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

        assert!(scheduler
            .add_process(Process::new(1, "test1".to_string(), Priority::Normal))
            .is_ok());
        assert!(scheduler
            .add_process(Process::new(2, "test2".to_string(), Priority::Normal))
            .is_ok());
        assert!(scheduler
            .add_process(Process::new(3, "test3".to_string(), Priority::Normal))
            .is_err());
    }

    #[test]
    fn test_priority_aging_and_demotion() {
        let mut scheduler = RoundRobinScheduler::new();
        scheduler.config.time_slice = 1000; // Large time slice so p1 does not switch automatically
        let p1 = Process::new(1, "p1".to_string(), Priority::Normal);
        let p2 = Process::new(2, "p2".to_string(), Priority::Low);
        scheduler.add_process(p1).unwrap();
        scheduler.add_process(p2).unwrap();

        // Let p1 run and p2 age
        for _ in 0..101 {
            scheduler.tick();
        }

        // p2 should be aged and temporarily promoted to Priority::Normal
        assert_eq!(scheduler.processes[1].process.priority, Priority::Normal);

        // Switch to p2 and tick once to let it run
        scheduler.current_index = 1;
        scheduler.tick();

        // After running, p2 should be demoted back to its original Priority::Low
        assert_eq!(scheduler.processes[1].process.priority, Priority::Low);
    }
}
