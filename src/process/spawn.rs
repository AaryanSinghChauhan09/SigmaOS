// OOP-based Process Spawning and POSIX Signals Framework for SigmaOS
// Implements process lifecycles, fork, exec, and signals (SIGKILL, SIGTERM, SIGINT) under `#![no_std]`.

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type ProcessID = usize;
pub type SignalHandlerFn = fn(ProcessID, u8);

/// Standard POSIX Signals
pub const SIGINT: u8 = 2; // Interrupt (graceful / catchable)
pub const SIGKILL: u8 = 9; // Force Kill (un-catchable, immediate)
pub const SIGUSR1: u8 = 10; // User defined 1 (catchable)
pub const SIGTERM: u8 = 15; // Terminate (graceful / catchable)

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Created = 0,
    Running = 1,
    Sleeping = 2,
    Zombie = 3,
    Terminated = 4,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessError {
    Success = 0,
    NotFound = 1,
    InvalidArgs = 2,
    SpawnFailed = 3,
}

pub trait Process {
    fn id(&self) -> ProcessID;
    fn parent_id(&self) -> ProcessID;
    fn state(&self) -> ProcessState;
    fn set_state(&mut self, state: ProcessState);
    fn exit_code(&self) -> i32;
    fn set_exit_code(&mut self, code: i32);
}

pub struct SimpleProcess {
    pub id: ProcessID,
    pub parent_id: ProcessID,
    pub state: AtomicUsize,
    pub exit_code: AtomicUsize,
}

impl SimpleProcess {
    pub fn new(id: ProcessID, parent_id: ProcessID) -> Self {
        SimpleProcess {
            id,
            parent_id,
            state: AtomicUsize::new(ProcessState::Created as usize),
            exit_code: AtomicUsize::new(0),
        }
    }
}

impl Process for SimpleProcess {
    fn id(&self) -> ProcessID {
        self.id
    }
    fn parent_id(&self) -> ProcessID {
        self.parent_id
    }
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

    fn exit_code(&self) -> i32 {
        self.exit_code.load(Ordering::SeqCst) as i32
    }

    fn set_exit_code(&mut self, code: i32) {
        self.exit_code.store(code as usize, Ordering::SeqCst);
    }
}

pub trait ProcessSpawner {
    fn spawn(&mut self, executable: &[u8], args: &[[u8; 64]]) -> Result<ProcessID, ProcessError>;
    fn fork(&mut self, parent_id: ProcessID) -> Result<ProcessID, ProcessError>;
    fn exec(
        &mut self,
        process_id: ProcessID,
        executable: &[u8],
        args: &[[u8; 64]],
    ) -> Result<(), ProcessError>;
    fn kill(&mut self, process_id: ProcessID, signal: u8) -> Result<(), ProcessError>;
}

/// Simple Process Spawner with custom signal handlers database
pub struct SimpleProcessSpawner {
    pub processes: Vec<Option<Box<dyn Process>>>,
    pub next_id: AtomicUsize,
    pub signal_handlers: Vec<(ProcessID, u8, SignalHandlerFn)>,
}

impl SimpleProcessSpawner {
    pub fn new() -> Self {
        SimpleProcessSpawner {
            processes: Vec::new(),
            next_id: AtomicUsize::new(1),
            signal_handlers: Vec::new(),
        }
    }

    /// Register a custom signal handler for a process
    pub fn register_signal_handler(
        &mut self,
        pid: ProcessID,
        signal: u8,
        handler: SignalHandlerFn,
    ) {
        if signal == SIGKILL {
            return; // SIGKILL cannot be caught or ignored!
        }
        self.signal_handlers.push((pid, signal, handler));
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

    fn exec(
        &mut self,
        process_id: ProcessID,
        _executable: &[u8],
        _args: &[[u8; 64]],
    ) -> Result<(), ProcessError> {
        for process_option in &mut self.processes {
            if let Some(ref mut process) = *process_option {
                if process.id() == process_id {
                    process.set_state(ProcessState::Running);
                    return Ok(());
                }
            }
        }
        Err(ProcessError::NotFound)
    }

    /// Dispatches POSIX signals. SIGKILL forces instant termination. Graceful signals trigger handlers or exit.
    fn kill(&mut self, process_id: ProcessID, signal: u8) -> Result<(), ProcessError> {
        let mut process_found = false;
        let mut exit_code_to_set = 0;

        for process_option in &mut self.processes {
            if let Some(ref mut process) = *process_option {
                if process.id() == process_id {
                    process_found = true;

                    if signal == SIGKILL {
                        // SIGKILL (9) is immediate and un-catchable
                        process.set_state(ProcessState::Terminated);
                        process.set_exit_code(137); // Standard 128 + 9 exit status for SIGKILL
                        return Ok(());
                    }

                    // Check for custom registered catchable signal handler
                    let mut handler_dispatched = false;
                    for &(pid, sig, handler) in &self.signal_handlers {
                        if pid == process_id && sig == signal {
                            handler(process_id, signal);
                            handler_dispatched = true;
                            break;
                        }
                    }

                    if !handler_dispatched {
                        // Default signal action is termination
                        process.set_state(ProcessState::Terminated);
                        exit_code_to_set = match signal {
                            SIGTERM => 143, // 128 + 15
                            SIGINT => 130,  // 128 + 2
                            _ => 1,
                        };
                    }
                    break;
                }
            }
        }

        if process_found {
            if exit_code_to_set > 0 {
                for process_option in &mut self.processes {
                    if let Some(ref mut process) = *process_option {
                        if process.id() == process_id {
                            process.set_exit_code(exit_code_to_set);
                        }
                    }
                }
            }
            Ok(())
        } else {
            Err(ProcessError::NotFound)
        }
    }
}

pub trait ProcessWaiter {
    fn wait(&mut self, process_id: ProcessID) -> Result<i32, ProcessError>;
    fn waitpid(
        &mut self,
        process_id: ProcessID,
        options: u32,
    ) -> Result<(ProcessID, i32), ProcessError>;
}

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
        for process_option in &self.spawner.processes {
            if let Some(ref process) = *process_option {
                if process.id() == process_id {
                    if process.state() == ProcessState::Terminated {
                        return Ok(process.exit_code());
                    }
                }
            }
        }
        Err(ProcessError::NotFound)
    }

    fn waitpid(
        &mut self,
        process_id: ProcessID,
        _options: u32,
    ) -> Result<(ProcessID, i32), ProcessError> {
        for process_option in &self.spawner.processes {
            if let Some(ref process) = *process_option {
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
        for group in &mut self.groups {
            if group.0 == group_id {
                group.1.push(process_id);
                return Ok(());
            }
        }
        Err(ProcessError::NotFound)
    }

    fn signal_group(&mut self, group_id: usize, _signal: u8) -> Result<(), ProcessError> {
        for group in &self.groups {
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
    use core::sync::atomic::AtomicUsize;

    static CUSTOM_SIGNAL_DISPATCH_COUNT: AtomicUsize = AtomicUsize::new(0);

    fn custom_sigterm_handler(_pid: ProcessID, _sig: u8) {
        CUSTOM_SIGNAL_DISPATCH_COUNT.fetch_add(1, Ordering::SeqCst);
    }

    #[test]
    fn test_process_spawning_and_sigkill() {
        let mut spawner = SimpleProcessSpawner::new();
        let pid = spawner.spawn(b"/bin/shell", &[]).unwrap();

        // 1. Send un-catchable SIGKILL -> Process should be instantly terminated with exit status 137
        spawner.kill(pid, SIGKILL).unwrap();

        let mut waiter = SimpleProcessWaiter::new(spawner);
        let exit_code = waiter.wait(pid).unwrap();
        assert_eq!(exit_code, 137);
    }

    #[test]
    fn test_custom_signal_handler_and_sigterm() {
        let mut spawner = SimpleProcessSpawner::new();
        let pid = spawner.spawn(b"/bin/logger", &[]).unwrap();
        spawner.exec(pid, b"/bin/logger", &[]).unwrap();

        // Register custom SIGTERM (15) handler
        spawner.register_signal_handler(pid, SIGTERM, custom_sigterm_handler);

        // Send SIGTERM -> Custom handler should be dispatched instead of default termination
        spawner.kill(pid, SIGTERM).unwrap();
        assert_eq!(CUSTOM_SIGNAL_DISPATCH_COUNT.load(Ordering::SeqCst), 1);

        // Standard processes state is unchanged since handler didn't call exit
        let mut waiter = SimpleProcessWaiter::new(spawner);
        assert!(waiter.wait(pid).is_err()); // Not terminated yet!
    }
}
