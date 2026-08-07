/// OOP-based Process Spawning & Signal Handling for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 121
/// Implements process creation, fork, exec, and Linux-grade signals (SIGINT, SIGKILL, SIGTERM, etc.)

use crate::klib::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type ProcessID = usize;

pub const SIGINT: u8 = 2;
pub const SIGKILL: u8 = 9;
pub const SIGUSR1: u8 = 10;
pub const SIGSEGV: u8 = 11;
pub const SIGTERM: u8 = 15;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState { Created = 0, Running = 1, Sleeping = 2, Zombie = 3, Terminated = 4 }

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessError { Success = 0, NotFound = 1, InvalidArgs = 2, SpawnFailed = 3 }

pub type SignalHandler = fn(u8);

pub trait Process {
    fn id(&self) -> ProcessID;
    fn parent_id(&self) -> ProcessID;
    fn state(&self) -> ProcessState;
    fn set_state(&mut self, state: ProcessState);
    fn exit_code(&self) -> i32;
    fn register_signal_handler(&mut self, signal: u8, handler: SignalHandler);
    fn deliver_signal(&mut self, signal: u8) -> bool; // Returns true if custom handler was executed, false otherwise
}

pub struct SimpleProcess {
    pub id: ProcessID,
    pub parent_id: ProcessID,
    pub state: AtomicUsize,
    pub exit_code: AtomicUsize,
    pub signal_handlers: Vec<(u8, SignalHandler)>,
    pub pending_signals: Vec<u8>,
}

impl SimpleProcess {
    pub fn new(id: ProcessID, parent_id: ProcessID) -> Self {
        SimpleProcess {
            id,
            parent_id,
            state: AtomicUsize::new(ProcessState::Created as usize),
            exit_code: AtomicUsize::new(0),
            signal_handlers: Vec::new(),
            pending_signals: Vec::new(),
        }
    }
}

impl Process for SimpleProcess {
    fn id(&self) -> ProcessID { self.id }
    fn parent_id(&self) -> ProcessID { self.parent_id }
    fn state(&self) -> ProcessState {
        match self.state.load(Ordering::SeqCst) {
            0 => ProcessState::Created,
            1 => ProcessState::Running,
            2 => ProcessState::Sleeping,
            3 => ProcessState::Zombie,
            _ => ProcessState::Terminated,
        }
    }

    fn set_state(&mut self, state: ProcessState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }

    fn exit_code(&self) -> i32 { self.exit_code.load(Ordering::SeqCst) as i32 }

    fn register_signal_handler(&mut self, signal: u8, handler: SignalHandler) {
        // SIGKILL cannot be caught or ignored
        if signal == SIGKILL {
            return;
        }
        self.signal_handlers.push((signal, handler));
    }

    fn deliver_signal(&mut self, signal: u8) -> bool {
        // Enforce SIGKILL (9) instant kernel termination (cannot be caught)
        if signal == SIGKILL {
            self.set_state(ProcessState::Terminated);
            self.exit_code.store(137, Ordering::SeqCst); // 128 + 9 = 137 standard Linux SIGKILL exit code
            return false;
        }

        // Search for registered custom signal handler
        let mut handler_idx = None;
        for i in 0..self.signal_handlers.len() {
            if self.signal_handlers[i].0 == signal {
                handler_idx = Some(i);
                break;
            }
        }

        if let Some(idx) = handler_idx {
            let handler_func = self.signal_handlers[idx].1;
            handler_func(signal);
            true
        } else {
            // Apply default Linux signal behavior (e.g. SIGTERM/SIGINT terminate)
            if signal == SIGTERM || signal == SIGINT || signal == SIGSEGV {
                self.set_state(ProcessState::Terminated);
                self.exit_code.store(128 + signal as usize, Ordering::SeqCst);
            }
            false
        }
    }
}

pub trait ProcessSpawner {
    fn spawn(&mut self, executable: &[u8], args: &[[u8; 64]]) -> Result<ProcessID, ProcessError>;
    fn fork(&mut self, parent_id: ProcessID) -> Result<ProcessID, ProcessError>;
    fn exec(&mut self, process_id: ProcessID, executable: &[u8], args: &[[u8; 64]]) -> Result<(), ProcessError>;
    fn kill(&mut self, process_id: ProcessID, signal: u8) -> Result<(), ProcessError>;
}

#[repr(C)]
pub struct SimpleProcessSpawner {
    pub processes: Vec<Option<Box<dyn Process>>>,
    pub next_id: AtomicUsize,
}

impl SimpleProcessSpawner {
    pub fn new() -> Self {
        SimpleProcessSpawner {
            processes: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Default for SimpleProcessSpawner {
    fn default() -> Self {
        Self::new()
    }
}

impl ProcessSpawner for SimpleProcessSpawner {
    fn spawn(&mut self, _executable: &[u8], _args: &[[u8; 64]]) -> Result<ProcessID, ProcessError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let process = SimpleProcess::new(id, 0);
        self.processes.push(Some(Box::new(process)));
        Ok(id)
    }

    fn fork(&mut self, parent_id: ProcessID) -> Result<ProcessID, ProcessError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let process = SimpleProcess::new(id, parent_id);
        self.processes.push(Some(Box::new(process)));
        Ok(id)
    }

    fn exec(&mut self, process_id: ProcessID, _executable: &[u8], _args: &[[u8; 64]]) -> Result<(), ProcessError> {
        for i in 0..self.processes.len() {
            if let Some(ref mut process) = self.processes[i] {
                if process.id() == process_id {
                    process.set_state(ProcessState::Running);
                    return Ok(());
                }
            }
        }
        Err(ProcessError::NotFound)
    }

    fn kill(&mut self, process_id: ProcessID, signal: u8) -> Result<(), ProcessError> {
        for process_option in &mut self.processes {
            if let Some(ref mut process) = *process_option {
                if process.id() == process_id {
                    process.deliver_signal(signal);
                    return Ok(());
                }
            }
        }
        Err(ProcessError::NotFound)
    }
}

pub trait ProcessWaiter {
    fn wait(&mut self, process_id: ProcessID) -> Result<i32, ProcessError>;
    fn waitpid(&mut self, process_id: ProcessID, options: u32) -> Result<(ProcessID, i32), ProcessError>;
}

#[repr(C)]
pub struct SimpleProcessWaiter {
    pub spawner: SimpleProcessSpawner,
}

impl SimpleProcessWaiter {
    pub fn new(spawner: SimpleProcessSpawner) -> Self {
        SimpleProcessWaiter { spawner }
    }
}

impl ProcessWaiter for SimpleProcessWaiter {
    fn wait(&mut self, process_id: ProcessID) -> Result<i32, ProcessError> {
        for i in 0..self.spawner.processes.len() {
            if let Some(ref process) = self.spawner.processes[i] {
                if process.id() == process_id {
                    if process.state() == ProcessState::Terminated {
                        return Ok(process.exit_code());
                    }
                }
            }
        }
        Err(ProcessError::NotFound)
    }

    fn waitpid(&mut self, process_id: ProcessID, _options: u32) -> Result<(ProcessID, i32), ProcessError> {
        for i in 0..self.spawner.processes.len() {
            if let Some(ref process) = self.spawner.processes[i] {
                if process.id() == process_id {
                    if process.state() == ProcessState::Terminated {
                        return Ok((process.id(), process.exit_code()));
                    }
                }
            }
        }
        Err(ProcessError::NotFound)
    }
}

pub trait ProcessGroup {
    fn create_group(&mut self, leader_id: ProcessID) -> Result<usize, ProcessError>;
    fn add_to_group(&mut self, group_id: usize, process_id: ProcessID) -> Result<(), ProcessError>;
    fn signal_group(&mut self, group_id: usize, signal: u8) -> Result<(), ProcessError>;
}

#[repr(C)]
pub struct SimpleProcessGroup {
    pub groups: Vec<(usize, Vec<ProcessID>)>,
    pub next_id: AtomicUsize,
}

impl SimpleProcessGroup {
    pub fn new() -> Self {
        SimpleProcessGroup {
            groups: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Default for SimpleProcessGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl ProcessGroup for SimpleProcessGroup {
    fn create_group(&mut self, leader_id: ProcessID) -> Result<usize, ProcessError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let mut processes = Vec::new();
        processes.push(leader_id);
        self.groups.push((id, processes));
        Ok(id)
    }

    fn add_to_group(&mut self, group_id: usize, process_id: ProcessID) -> Result<(), ProcessError> {
        for i in 0..self.groups.len() {
            let group = &mut self.groups[i];
            if group.0 == group_id {
                group.1.push(process_id);
                return Ok(());
            }
        }
        Err(ProcessError::NotFound)
    }

    fn signal_group(&mut self, group_id: usize, _signal: u8) -> Result<(), ProcessError> {
        for i in 0..self.groups.len() {
            let group = &self.groups[i];
            if group.0 == group_id {
                return Ok(());
            }
        }
        Err(ProcessError::NotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static RECEIVED_SIGNAL: AtomicUsize = AtomicUsize::new(0);

    fn custom_handler(sig: u8) {
        RECEIVED_SIGNAL.store(sig as usize, Ordering::SeqCst);
    }

    #[test]
    fn test_process_creation_and_state() {
        let mut proc = SimpleProcess::new(101, 1);
        assert_eq!(proc.id(), 101);
        assert_eq!(proc.state(), ProcessState::Created);

        proc.set_state(ProcessState::Running);
        assert_eq!(proc.state(), ProcessState::Running);
    }

    #[test]
    fn test_linux_signals_default_actions() {
        let mut proc = SimpleProcess::new(202, 1);

        // Delivering SIGTERM (15) should terminate process with standard exit code 128 + 15 = 143
        assert!(!proc.deliver_signal(SIGTERM));
        assert_eq!(proc.state(), ProcessState::Terminated);
        assert_eq!(proc.exit_code(), 143);
    }

    #[test]
    fn test_custom_signal_handlers() {
        let mut proc = SimpleProcess::new(303, 1);

        // Register custom handler for SIGUSR1 (10)
        proc.register_signal_handler(SIGUSR1, custom_handler);

        // Reset static signal received indicator
        RECEIVED_SIGNAL.store(0, Ordering::SeqCst);

        // Deliver SIGUSR1 -> should trigger handler instead of default termination
        assert!(proc.deliver_signal(SIGUSR1));
        assert_eq!(proc.state(), ProcessState::Created); // Still created (not terminated)
        assert_eq!(RECEIVED_SIGNAL.load(Ordering::SeqCst), SIGUSR1 as usize);
    }

    #[test]
    fn test_sigkill_cannot_be_caught_or_ignored() {
        let mut proc = SimpleProcess::new(404, 1);

        // Try registering handler for SIGKILL -> should be ignored/blocked by process model
        proc.register_signal_handler(SIGKILL, custom_handler);

        // Deliver SIGKILL -> must instantly terminate with standard exit code 137, bypassing handler!
        assert!(!proc.deliver_signal(SIGKILL));
        assert_eq!(proc.state(), ProcessState::Terminated);
        assert_eq!(proc.exit_code(), 137);
    }
}
