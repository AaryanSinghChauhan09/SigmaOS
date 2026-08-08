<<<<<<< HEAD
#![no_std]
#![no_main]

/// OOP-based Process Spawning for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 121
/// Implements process creation, fork, and exec
||||||| 23ef22a4a
/// OOP-based Process Spawning for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 121
/// Implements process creation, fork, exec, namespace isolations, and nice priority levels
=======
// OOP-based Process Spawning and POSIX Signals Framework for SigmaOS
// Implements process lifecycles, fork, exec, and signals (SIGKILL, SIGTERM, SIGINT) under `#![no_std]`.
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e

<<<<<<< HEAD
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;
||||||| 23ef22a4a
extern crate alloc;

use alloc::boxed::Box;
use core::sync::atomic::{AtomicUsize, Ordering, AtomicI32};
use core::mem;
=======
extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e

pub type ProcessID = usize;
pub type SignalHandlerFn = fn(ProcessID, u8);

<<<<<<< HEAD
||||||| 23ef22a4a
pub const SIGINT: u8 = 2;
pub const SIGKILL: u8 = 9;
pub const SIGUSR1: u8 = 10;
pub const SIGSEGV: u8 = 11;
pub const SIGTERM: u8 = 15;

=======
/// Standard POSIX Signals
pub const SIGINT: u8 = 2; // Interrupt (graceful / catchable)
pub const SIGKILL: u8 = 9; // Force Kill (un-catchable, immediate)
pub const SIGUSR1: u8 = 10; // User defined 1 (catchable)
pub const SIGTERM: u8 = 15; // Terminate (graceful / catchable)

>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
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
<<<<<<< HEAD
#[derive(Debug, Clone, Copy)]
pub enum ProcessError { Success = 0, NotFound = 1, InvalidArgs = 2, SpawnFailed = 3 }
||||||| 23ef22a4a
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessError { Success = 0, NotFound = 1, InvalidArgs = 2, SpawnFailed = 3, InvalidNiceValue = 4 }

// Linux namespace isolation flags representation
pub const CLONE_NEWNS: u32 = 0x00020000;  // Mount namespace
pub const CLONE_NEWNET: u32 = 0x40000000; // Network namespace
pub const CLONE_NEWPID: u32 = 0x20000000; // PID namespace

pub type SignalHandler = fn(u8);
=======
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessError {
    Success = 0,
    NotFound = 1,
    InvalidArgs = 2,
    SpawnFailed = 3,
}
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e

pub trait Process {
    fn id(&self) -> ProcessID;
    fn parent_id(&self) -> ProcessID;
    fn state(&self) -> ProcessState;
    fn set_state(&mut self, state: ProcessState);
    fn exit_code(&self) -> i32;
<<<<<<< HEAD
||||||| 23ef22a4a
    fn nice(&self) -> i32;
    fn set_nice(&mut self, value: i32) -> Result<(), ProcessError>;
    fn namespace_flags(&self) -> u32;
    fn set_namespace_flags(&mut self, flags: u32);
=======
    fn set_exit_code(&mut self, code: i32);
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
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
<<<<<<< HEAD
    fn id(&self) -> ProcessID { self.id }
    fn parent_id(&self) -> ProcessID { self.parent_id }
||||||| 23ef22a4a
    fn id(&self) -> ProcessID { self.id }
    fn parent_id(&self) -> ProcessID { self.parent_id }

=======
    fn id(&self) -> ProcessID {
        self.id
    }
    fn parent_id(&self) -> ProcessID {
        self.parent_id
    }
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
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

<<<<<<< HEAD
    fn exit_code(&self) -> i32 { self.exit_code.load(Ordering::SeqCst) as i32 }
||||||| 23ef22a4a
    fn exit_code(&self) -> i32 { self.exit_code.load(Ordering::SeqCst) as i32 }

    fn nice(&self) -> i32 {
        self.nice_val.load(Ordering::SeqCst)
    }

    fn set_nice(&mut self, value: i32) -> Result<(), ProcessError> {
        if value < -20 || value > 19 {
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
=======
    fn exit_code(&self) -> i32 {
        self.exit_code.load(Ordering::SeqCst) as i32
    }

    fn set_exit_code(&mut self, code: i32) {
        self.exit_code.store(code as usize, Ordering::SeqCst);
    }
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
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

<<<<<<< HEAD
    fn exec(&mut self, process_id: ProcessID, _executable: &[u8], _args: &[[u8; 64]]) -> Result<(), ProcessError> {
        for i in 0..self.processes.len() {
            if let Some(ref mut process) = &mut self.processes.data_mut()[i] {
||||||| 23ef22a4a
    fn exec(&mut self, process_id: ProcessID, _executable: &[u8], _args: &[[u8; 64]]) -> Result<(), ProcessError> {
        for process_option in self.processes.as_slice_mut() {
            if let Some(ref mut process) = *process_option {
=======
    fn exec(
        &mut self,
        process_id: ProcessID,
        _executable: &[u8],
        _args: &[[u8; 64]],
    ) -> Result<(), ProcessError> {
        for process_option in &mut self.processes {
            if let Some(ref mut process) = *process_option {
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
                if process.id() == process_id {
                    process.set_state(ProcessState::Running);
                    return Ok(());
                }
            }
        }
        Err(ProcessError::NotFound)
    }

<<<<<<< HEAD
    fn kill(&mut self, process_id: ProcessID, _signal: u8) -> Result<(), ProcessError> {
        for i in 0..self.processes.len() {
            if let Some(ref mut process) = &mut self.processes.data_mut()[i] {
||||||| 23ef22a4a
    fn kill(&mut self, process_id: ProcessID, _signal: u8) -> Result<(), ProcessError> {
        for process_option in self.processes.as_slice_mut() {
            if let Some(ref mut process) = *process_option {
=======
    /// Dispatches POSIX signals. SIGKILL forces instant termination. Graceful signals trigger handlers or exit.
    fn kill(&mut self, process_id: ProcessID, signal: u8) -> Result<(), ProcessError> {
        let mut process_found = false;
        let mut exit_code_to_set = 0;

        for process_option in &mut self.processes {
            if let Some(ref mut process) = *process_option {
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
                if process.id() == process_id {
<<<<<<< HEAD
                    process.set_state(ProcessState::Terminated);
                    return Ok(());
||||||| 23ef22a4a
                    process.deliver_signal(signal);
                    return Ok(());
=======
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
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
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
        for i in 0..self.spawner.processes.len() {
            if let Some(ref process) = self.spawner.processes.data()[i] {
                if process.id() == process_id {
                    if process.state() == ProcessState::Terminated {
                        return Ok(process.exit_code());
                    }
                }
            }
        }
        Err(ProcessError::NotFound)
    }

<<<<<<< HEAD
    fn waitpid(&mut self, process_id: ProcessID, _options: u32) -> Result<(ProcessID, i32), ProcessError> {
        for i in 0..self.spawner.processes.len() {
            if let Some(ref process) = self.spawner.processes.data()[i] {
||||||| 23ef22a4a
    fn waitpid(&mut self, process_id: ProcessID, _options: u32) -> Result<(ProcessID, i32), ProcessError> {
        for process_option in self.spawner.processes.as_slice() {
            if let Some(ref process) = *process_option {
=======
    fn waitpid(
        &mut self,
        process_id: ProcessID,
        _options: u32,
    ) -> Result<(ProcessID, i32), ProcessError> {
        for process_option in &self.spawner.processes {
            if let Some(ref process) = *process_option {
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
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
            let group = &mut self.groups.data_mut()[i];
            if group.0 == group_id {
                group.1.push(process_id);
                return Ok(());
            }
        }
        Err(ProcessError::NotFound)
    }

    fn signal_group(&mut self, group_id: usize, _signal: u8) -> Result<(), ProcessError> {
<<<<<<< HEAD
        for i in 0..self.groups.len() {
            let group = &self.groups.data()[i];
||||||| 23ef22a4a
        for group in self.groups.as_slice_mut() {
=======
        for group in &self.groups {
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
            if group.0 == group_id {
                return Ok(());
            }
        }
        Err(ProcessError::NotFound)
    }
}

<<<<<<< HEAD
pub struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    pub fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    pub fn len(&self) -> usize { self.len }
    pub fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    pub fn data(&self) -> &[T] {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
    pub fn data_mut(&mut self) -> &mut [T] {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
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

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if !self.data.is_null() {
            unsafe {
                for i in 0..self.len {
                    core::ptr::drop_in_place(self.data.add(i));
                }
                free(self.data as *mut u8);
            }
        }
||||||| 23ef22a4a
pub struct Vec<T> { data: *mut T, len: usize, capacity: usize }

    static RECEIVED_SIGNAL: AtomicUsize = AtomicUsize::new(0);

    fn custom_handler(sig: u8) {
        RECEIVED_SIGNAL.store(sig as usize, Ordering::SeqCst);
    }
    fn as_slice(&self) -> &[T] {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
    fn as_slice_mut(&mut self) -> &mut [T] {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
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

#[cfg(not(test))]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[cfg(test)]
extern crate std;

#[cfg(test)]
unsafe fn alloc(size: usize) -> *mut u8 {
    std::alloc::alloc(std::alloc::Layout::from_size_align_unchecked(size, 8))
}

#[cfg(test)]
unsafe fn free(_ptr: *mut u8) {
    // In standard shims, we can just let OS reclaim heap on test exit or perform simple dummy dealloc
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
        assert!(process.set_nice(25).is_err());  // invalid nice level

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

        let mut waiter = SimpleProcessWaiter::new(spawner);
        // Wait on non-terminated process should not succeed with termination exit code
        assert!(waiter.wait(pid).is_err());
=======
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
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
