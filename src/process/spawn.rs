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

pub const WNOHANG: u32 = 1;
pub const WUNTRACED: u32 = 2;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState { Created = 0, Running = 1, Sleeping = 2, Zombie = 3, Terminated = 4 }
||||||| 43be3a7e8
#[derive(Debug, Clone, Copy)]
pub enum ProcessState { Created = 0, Running = 1, Sleeping = 2, Zombie = 3, Terminated = 4 }
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
pub enum ProcessError { Success = 0, NotFound = 1, InvalidArgs = 2, SpawnFailed = 3 }
||||||| 43be3a7e8
#[derive(Debug, Clone, Copy)]
pub enum ProcessError { Success = 0, NotFound = 1, InvalidArgs = 2, SpawnFailed = 3 }
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
    fn set_parent_id(&mut self, parent_id: ProcessID);
    fn state(&self) -> ProcessState;
    fn set_state(&mut self, state: ProcessState);
    fn exit_code(&self) -> i32;

    // Nice values (-20 to 19) mimicking Linux process scheduling priority
    fn nice(&self) -> i32 { 0 }
    fn set_nice(&mut self, nice: i32) {}
||||||| 43be3a7e8
    fn set_exit_code(&mut self, code: i32);
}

pub struct SimpleProcess {
    pub id: ProcessID,
    pub parent_id: ProcessID,
    pub state: AtomicUsize,
    pub exit_code: AtomicUsize,
    pub nice: i32,
}

impl SimpleProcess {
    pub fn new(id: ProcessID, parent_id: ProcessID) -> Self {
        SimpleProcess {
            id,
            parent_id,
            state: AtomicUsize::new(ProcessState::Created as usize),
            exit_code: AtomicUsize::new(0),
            nice: 0,
        }
    }
}

impl Process for SimpleProcess {
    fn id(&self) -> ProcessID { self.id }
    fn parent_id(&self) -> ProcessID { self.parent_id }
    fn set_parent_id(&mut self, parent_id: ProcessID) { self.parent_id = parent_id; }
    fn state(&self) -> ProcessState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }
||||||| 43be3a7e8
    fn id(&self) -> ProcessID { self.id }
    fn parent_id(&self) -> ProcessID { self.parent_id }
    fn state(&self) -> ProcessState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }
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

    fn exit_code(&self) -> i32 { self.exit_code.load(Ordering::SeqCst) as i32 }

    fn nice(&self) -> i32 {
        self.nice
    }

    fn set_nice(&mut self, nice: i32) {
        self.nice = nice.clamp(-20, 19);
    }
||||||| 43be3a7e8
    fn exit_code(&self) -> i32 { self.exit_code.load(Ordering::SeqCst) as i32 }
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

    /// Linux-style reparenting: Orphan children are re-parented to init (PID 1) upon parent exit.
    pub fn reparent_orphans(&mut self, dead_parent_id: ProcessID) {
        for i in 0..self.processes.len() {
            if let Some(ref mut process) = self.processes[i] {
                if process.parent_id() == dead_parent_id {
                    process.set_parent_id(1);
                }
            }
        }
    }
||||||| 43be3a7e8

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

    fn exec(&mut self, process_id: ProcessID, _executable: &[u8], _args: &[[u8; 64]]) -> Result<(), ProcessError> {
        for i in 0..self.processes.len() {
            if let Some(ref mut process) = self.processes[i] {
||||||| 43be3a7e8
    fn exec(&mut self, process_id: ProcessID, _executable: &[u8], _args: &[[u8; 64]]) -> Result<(), ProcessError> {
        for process_option in &mut self.processes {
            if let Some(ref mut process) = *process_option {
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

    fn kill(&mut self, process_id: ProcessID, _signal: u8) -> Result<(), ProcessError> {
        for i in 0..self.processes.len() {
            if let Some(ref mut process) = self.processes[i] {
||||||| 43be3a7e8
    fn kill(&mut self, process_id: ProcessID, _signal: u8) -> Result<(), ProcessError> {
        for process_option in &mut self.processes {
            if let Some(ref mut process) = *process_option {
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
        self.waitpid(process_id, 0).map(|(_, code)| code)
    }

    fn waitpid(&mut self, process_id: ProcessID, options: u32) -> Result<(ProcessID, i32), ProcessError> {
        for i in 0..self.spawner.processes.len() {
            if let Some(ref process) = self.spawner.processes[i] {
||||||| 43be3a7e8
    fn waitpid(&mut self, process_id: ProcessID, _options: u32) -> Result<(ProcessID, i32), ProcessError> {
        for process_option in &self.spawner.processes {
            if let Some(ref process) = *process_option {
    fn waitpid(
        &mut self,
        process_id: ProcessID,
        _options: u32,
    ) -> Result<(ProcessID, i32), ProcessError> {
        for process_option in &self.spawner.processes {
            if let Some(ref process) = *process_option {
                if process.id() == process_id {
                    let is_terminated = process.state() == ProcessState::Terminated;
                    if is_terminated {
                        return Ok((process.id(), process.exit_code()));
                    } else {
                        // Standard non-blocking wait (WNOHANG)
                        if (options & WNOHANG) != 0 {
                            return Ok((0, 0));
                        }
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
    fn signal_group(&mut self, group_id: usize, signal: u8, spawner: &mut SimpleProcessSpawner) -> Result<(), ProcessError>;
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
        for i in 0..self.groups.len() {
            if self.groups[i].0 == group_id {
                self.groups[i].1.push(process_id);
                return Ok(());
            }
        }
        Err(ProcessError::NotFound)
    }

    fn signal_group(&mut self, group_id: usize, signal: u8, spawner: &mut SimpleProcessSpawner) -> Result<(), ProcessError> {
        let mut members = Vec::new();
        for i in 0..self.groups.len() {
            if self.groups[i].0 == group_id {
                for j in 0..self.groups[i].1.len() {
                    members.push(self.groups[i].1[j]);
                }
                break;
||||||| 43be3a7e8
    fn signal_group(&mut self, group_id: usize, _signal: u8) -> Result<(), ProcessError> {
        for group in &mut self.groups {
            if group.0 == group_id {
                return Ok(());
    fn signal_group(&mut self, group_id: usize, _signal: u8) -> Result<(), ProcessError> {
        for group in &self.groups {
            if group.0 == group_id {
                return Ok(());
            }
        }

        if members.is_empty() {
            return Err(ProcessError::NotFound);
        }

        for i in 0..members.len() {
            let pid = members[i];
            let _ = spawner.kill(pid, signal);
        }
        Ok(())
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

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }


impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::Deref;
        self.deref().iter()
    }
}


impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::DerefMut;
        self.deref_mut().iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_lifecycle_spawn_and_kill() {
        let mut spawner = SimpleProcessSpawner::new();
        let pid = spawner.spawn(b"/bin/ls", &[]).unwrap();
        assert_eq!(pid, 1);

        spawner.exec(pid, b"", &[]).unwrap();
        assert_eq!(spawner.processes[0].as_ref().unwrap().state(), ProcessState::Running);

        spawner.kill(pid, 9).unwrap();
        assert_eq!(spawner.processes[0].as_ref().unwrap().state(), ProcessState::Terminated);
    }

    #[test]
    fn test_linux_process_parity() {
        let mut spawner = SimpleProcessSpawner::new();

        // Spawn parents & children
        let parent_id = spawner.spawn(b"parent", &[]).unwrap();
        let child_id = spawner.fork(parent_id).unwrap();

        let child = spawner.processes[1].as_ref().unwrap();
        assert_eq!(child.parent_id(), parent_id);

        // Reparent orphans check
        spawner.reparent_orphans(parent_id);
        assert_eq!(spawner.processes[1].as_ref().unwrap().parent_id(), 1);

        // Nice value priority tests
        let mut child_mut = spawner.processes[1].as_mut().unwrap();
        child_mut.set_nice(10);
        assert_eq!(child_mut.nice(), 10);
        child_mut.set_nice(-25); // clamping to -20
        assert_eq!(child_mut.nice(), -20);

        // Non-blocking WNOHANG waitpid
        let mut waiter = SimpleProcessWaiter::new(spawner);
        // child_id is not terminated -> WNOHANG returns (0, 0)
        let (wait_pid, code) = waiter.waitpid(child_id, WNOHANG).unwrap();
        assert_eq!(wait_pid, 0);
        assert_eq!(code, 0);

        // Signal groups testing
        let mut spawner2 = SimpleProcessSpawner::new();
        let lead_pid = spawner2.spawn(b"leader", &[]).unwrap();
        let member_pid = spawner2.spawn(b"member", &[]).unwrap();

        let mut grp = SimpleProcessGroup::new();
        let gid = grp.create_group(lead_pid).unwrap();
        grp.add_to_group(gid, member_pid).unwrap();

        grp.signal_group(gid, 15, &mut spawner2).unwrap();
        assert_eq!(spawner2.processes[0].as_ref().unwrap().state(), ProcessState::Terminated);
        assert_eq!(spawner2.processes[1].as_ref().unwrap().state(), ProcessState::Terminated);
    }
}
||||||| 43be3a7e8

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
