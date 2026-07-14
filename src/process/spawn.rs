#![no_std]
#![no_main]

/// OOP-based Process Spawning for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 121
/// Implements process creation, fork, and exec

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ProcessID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ProcessState { Created = 0, Running = 1, Sleeping = 2, Zombie = 3, Terminated = 4 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ProcessError { Success = 0, NotFound = 1, InvalidArgs = 2, SpawnFailed = 3 }

pub trait Process {
    fn id(&self) -> ProcessID;
    fn parent_id(&self) -> ProcessID;
    fn state(&self) -> ProcessState;
    fn set_state(&mut self, state: ProcessState);
    fn exit_code(&self) -> i32;
}

#[repr(C)]
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
    fn id(&self) -> ProcessID { self.id }
    fn parent_id(&self) -> ProcessID { self.parent_id }
    fn state(&self) -> ProcessState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }

    fn set_state(&mut self, state: ProcessState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }

    fn exit_code(&self) -> i32 { self.exit_code.load(Ordering::SeqCst) as i32 }
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

    fn kill(&mut self, process_id: ProcessID, _signal: u8) -> Result<(), ProcessError> {
        for process_option in &mut self.processes {
            if let Some(ref mut process) = *process_option {
                if process.id() == process_id {
                    process.set_state(ProcessState::Terminated);
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

    fn waitpid(&mut self, process_id: ProcessID, _options: u32) -> Result<(ProcessID, i32), ProcessError> {
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
        for group in &mut self.groups {
            if group.0 == group_id {
                return Ok(());
            }
        }
        Err(ProcessError::NotFound)
    }
}

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
