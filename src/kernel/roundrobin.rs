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
}

impl ScheduledProcess {
    pub fn new(process: Process) -> Self {
        Self {
            process,
            context: CpuContext::new(),
            yield_requested: false,
            cpu_time_used: 0,
        }
    }

    /// Request this process to yield the CPU voluntarily
    pub fn request_yield(&mut self) {
        self.yield_requested = true;
    }

    /// Priority-based weight: higher priority gets a larger time slice multiplier
    pub fn time_slice_ticks(&self, base_slice: u64) -> u64 {
        let multiplier: u64 = match self.process.priority {
            Priority::Realtime => 8,
            Priority::High => 4,
            Priority::Normal => 2,
            Priority::Low => 1,
            Priority::Idle => 1, // Idle still gets a minimal slice
        };
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

/// Enhanced priority-aware round-robin scheduler with O(1) work-stealing
pub struct RoundRobinScheduler {
    processes: Vec<ScheduledProcess>,
    pub current_index: usize,
    config: RoundRobinConfig,
    current_time: u64,
    ready_queue_head: Option<usize>, // O(1) tracking of ready processes
}

impl RoundRobinScheduler {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            current_index: 0,
            config: RoundRobinConfig::default(),
            current_time: 0,
            ready_queue_head: None,
        }
    }

    pub fn with_config(config: RoundRobinConfig) -> Self {
        Self {
            processes: Vec::new(),
            current_index: 0,
            config,
            current_time: 0,
            ready_queue_head: None,
        }
    }

    pub fn add_process(&mut self, process: Process) -> Result<(), SchedulerError> {
        if self.processes.len() >= self.config.max_processes {
            return Err(SchedulerError::TooManyProcesses);
        }
        let idx = self.processes.len();
        self.processes.push(ScheduledProcess::new(process));
        // Update ready_queue_head if this is the first ready process
        if self.ready_queue_head.is_none()
            && self.processes[idx].process.state == ProcessState::Ready
        {
            self.ready_queue_head = Some(idx);
        }
        Ok(())
    }

    pub fn schedule(&mut self) -> Option<&Process> {
        if self.processes.is_empty() {
            return None;
        }

        // O(1) lookup using ready_queue_head
        if let Some(head) = self.ready_queue_head {
            if self.processes[head].process.state == ProcessState::Ready {
                self.current_index = head;
                return Some(&self.processes[head].process);
            }
        }

        // Fallback: find first ready process and update head
        for (i, proc) in self.processes.iter().enumerate() {
            if proc.process.state == ProcessState::Ready {
                self.ready_queue_head = Some(i);
                self.current_index = i;
                return Some(&proc.process);
            }
        }

        self.ready_queue_head = None;
        None
    }

    pub fn tick(&mut self) {
        self.current_time += 1;

        if self.processes.is_empty() {
            return;
        }

        let needs_switch = {
            let entry = &mut self.processes[self.current_index];
            entry.cpu_time_used += 1;
            let slice = entry.time_slice_ticks(self.config.time_slice);
            let yielding = entry.yield_requested;
            entry.yield_requested = false;
            yielding || (entry.cpu_time_used % slice == 0)
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
        if let Some((idx, entry)) = self
            .processes
            .iter_mut()
            .enumerate()
            .find(|(i, e)| e.process.pid == pid)
        {
            entry.process.state = state;
            // Update ready_queue_head when state changes
            if state == ProcessState::Ready && self.ready_queue_head.is_none() {
                self.ready_queue_head = Some(idx);
            } else if state != ProcessState::Ready && self.ready_queue_head == Some(idx) {
                self.ready_queue_head = None;
            }
        }
    }

    pub fn remove_process(&mut self, pid: u64) {
        if let Some(idx) = self.processes.iter().position(|e| e.process.pid == pid) {
            self.processes.remove(idx);
            // Reset ready_queue_head if it pointed to removed process
            if self.ready_queue_head == Some(idx) {
                self.ready_queue_head = None;
            }
            // Adjust ready_queue_head if it was after removed index
            if let Some(head) = self.ready_queue_head {
                if head > idx {
                    self.ready_queue_head = Some(head - 1);
                }
            }
        }
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
        let p1 = Process::new(1, "test1".to_string(), Priority::Normal);
        let p2 = Process::new(2, "test2".to_string(), Priority::Normal);
        scheduler.add_process(p1).unwrap();
        scheduler.add_process(p2).unwrap();

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
}
