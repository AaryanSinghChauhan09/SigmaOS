use std::collections::VecDeque;

/// Represents a single captured syscall event, displacing `strace`.
#[derive(Debug, Clone)]
pub struct SyscallEvent {
    pub pid: u32,
    pub syscall_nr: u64,
    pub args: [u64; 6],
    pub return_value: i64,
    pub timestamp_ns: u64,
}

/// TraceCollector captures syscall-level events from processes
/// using the kernel's ptrace interface natively.
pub struct TraceCollector {
    events: VecDeque<SyscallEvent>,
    max_buffer: usize,
}

impl Default for TraceCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl TraceCollector {
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
            max_buffer: 65536,
        }
    }

    /// Attach to a process and begin tracing its syscalls.
    pub fn attach(&mut self, _pid: u32) -> Result<(), String> {
        // Real implementation: ptrace(PTRACE_ATTACH, pid, ...)
        Ok(())
    }

    /// Record a syscall event into the ring buffer.
    pub fn record(&mut self, event: SyscallEvent) {
        if self.events.len() >= self.max_buffer {
            self.events.pop_front();
        }
        self.events.push_back(event);
    }

    /// Drain all captured events.
    pub fn drain(&mut self) -> Vec<SyscallEvent> {
        self.events.drain(..).collect()
    }
}
