// OOP-based Process Spawning and POSIX Signals Framework for SigmaOS
// Implements process lifecycles, fork, exec, and signals (SIGKILL, SIGTERM, SIGINT).

use std::boxed::Box;
use std::vec::Vec;
use core::sync::atomic::{AtomicI32, AtomicUsize, Ordering};

pub type ProcessID = usize;

pub const SIGINT: u8 = 2;
pub const SIGKILL: u8 = 9;
pub const SIGTERM: u8 = 15;

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
    InvalidNiceValue = 4,
}

// Linux namespace isolation flags representation
pub const CLONE_NEWNS: u32 = 0x00020000; // Mount namespace
pub const CLONE_NEWNET: u32 = 0x40000000; // Network namespace
pub const CLONE_NEWPID: u32 = 0x20000000; // PID namespace

pub trait Process {
    fn id(&self) -> ProcessID;
    fn parent_id(&self) -> ProcessID;
    fn state(&self) -> ProcessState;
    fn set_state(&mut self, state: ProcessState);
    fn exit_code(&self) -> i32;
    fn set_exit_code(&mut self, code: i32);
    fn nice(&self) -> i32;
    fn set_nice(&mut self, value: i32) -> Result<(), ProcessError>;
    fn namespace_flags(&self) -> u32;
    fn set_namespace_flags(&mut self, flags: u32);
}

pub struct SimpleProcess {
    pub id: ProcessID,
    pub parent_id: ProcessID,
    pub state: AtomicUsize,
    pub exit_code: AtomicUsize,
    pub nice_val: AtomicI32,
    pub ns_flags: AtomicUsize,
}

impl SimpleProcess {
    pub fn new(id: ProcessID, parent_id: ProcessID) -> Self {
        SimpleProcess {
            id,
            parent_id,
            state: AtomicUsize::new(ProcessState::Created as usize),
            exit_code: AtomicUsize::new(0),
            nice_val: AtomicI32::new(0),   // Default Nice level = 0
            ns_flags: AtomicUsize::new(0), // No isolation flags by default
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
            1 => ProcessState::Running,
            2 => ProcessState::Sleeping,
            3 => ProcessState::Zombie,
            4 => ProcessState::Terminated,
            _ => ProcessState::Created,
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

    fn nice(&self) -> i32 {
        self.nice_val.load(Ordering::SeqCst)
    }

    fn set_nice(&mut self, value: i32) -> Result<(), ProcessError> {
        if !(-20..=19).contains(&value) {
            return Err(ProcessError::InvalidNiceValue);
        }
        self.nice_val.store(value, Ordering::SeqCst);
        Ok(())
    }

    fn namespace_flags(&self) -> u32 {
        self.ns_flags.load(Ordering::SeqCst) as u32
    }

    fn set_namespace_flags(&mut self, flags: u32) {
        self.ns_flags.store(flags as usize, Ordering::SeqCst);
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

    fn kill(&mut self, process_id: ProcessID, signal: u8) -> Result<(), ProcessError> {
        for process_option in &mut self.processes {
            if let Some(ref mut process) = *process_option {
                if process.id() == process_id {
                    if signal == SIGKILL {
                        process.set_state(ProcessState::Terminated);
                        process.set_exit_code(137);
                    } else {
                        process.set_state(ProcessState::Terminated);
                        process.set_exit_code(143);
                    }
                    return Ok(());
                }
            }
        }
        Err(ProcessError::NotFound)
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
                if process.id() == process_id && process.state() == ProcessState::Terminated {
                    return Ok(process.exit_code());
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
                if process.id() == process_id && process.state() == ProcessState::Terminated {
                    return Ok((process.id(), process.exit_code()));
                }
            }
        }
        Err(ProcessError::NotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_nice_and_namespaces() {
        let mut process = SimpleProcess::new(101, 10);
        assert_eq!(process.id(), 101);
        assert_eq!(process.parent_id(), 10);
        assert_eq!(process.state(), ProcessState::Created);
        assert_eq!(process.nice(), 0);
        assert_eq!(process.namespace_flags(), 0);

        // Modify nice priority level with boundary check
        assert!(process.set_nice(15).is_ok());
        assert_eq!(process.nice(), 15);
        assert!(process.set_nice(-25).is_err()); // invalid nice level
        assert!(process.set_nice(25).is_err()); // invalid nice level

        // Modify namespace isolation flags
        process.set_namespace_flags(CLONE_NEWPID | CLONE_NEWNET);
        assert_eq!(process.namespace_flags(), CLONE_NEWPID | CLONE_NEWNET);
    }

    #[test]
    fn test_process_spawner_and_waiter() {
        let mut spawner = SimpleProcessSpawner::new();
        let pid = spawner.spawn(b"/bin/ls", &[]).unwrap();
        assert_eq!(pid, 1);

        spawner.exec(pid, b"/bin/ls", &[]).unwrap();

        spawner.kill(pid, SIGKILL).unwrap();

        let mut waiter = SimpleProcessWaiter::new(spawner);
        let (exit_pid, code) = waiter.waitpid(pid, 0).unwrap();
        assert_eq!(exit_pid, pid);
        assert_eq!(code, 137);
    }
}
