#![no_std]
#![no_main]

/// OOP-based Process Scheduler for SigmaOS
/// Implements process scheduling using OOP principles with traits and structs
/// No dependency on external scheduling frameworks
/// Based on Roadmap Item 5: Process scheduler

use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

/// Process ID
pub type ProcessID = usize;

/// Process state
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ProcessState {
    Ready = 0,
    Running = 1,
    Blocked = 2,
    Terminated = 3,
}

/// Process priority
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProcessPriority {
    Idle = 0,
    Low = 1,
    Normal = 2,
    High = 3,
    Critical = 4,
}

/// Process trait (OOP interface)
pub trait Process {
    /// Get process ID
    fn id(&self) -> ProcessID;
    /// Get process name
    fn name(&self) -> &[u8];
    /// Get process state
    fn state(&self) -> ProcessState;
    /// Get process priority
    fn priority(&self) -> ProcessPriority;
    /// Set process state
    fn set_state(&mut self, state: ProcessState);
    /// Set process priority
    fn set_priority(&mut self, priority: ProcessPriority);
    /// Get process info
    fn info(&self) -> ProcessInfo;
}

/// Process info
#[repr(C)]
pub struct ProcessInfo {
    pub id: ProcessID,
    pub name: [u8; 64],
    pub state: ProcessState,
    pub priority: ProcessPriority,
    pub cpu_time: u64,
    pub capability: ProcessCapability,
}

impl ProcessInfo {
    pub fn new(id: ProcessID) -> Self {
        ProcessInfo {
            id,
            name: [0; 64],
            state: ProcessState::Ready,
            priority: ProcessPriority::Normal,
            cpu_time: 0,
            capability: ProcessCapability::new(),
        }
    }
}

/// Process capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ProcessCapability {
    pub can_change_state: bool,
    pub can_change_priority: bool,
}

impl ProcessCapability {
    pub fn new() -> Self {
        ProcessCapability {
            can_change_state: false,
            can_change_priority: false,
        }
    }

    pub fn full() -> Self {
        ProcessCapability {
            can_change_state: true,
            can_change_priority: true,
        }
    }
}

/// Simple process (OOP: Concrete process class)
#[repr(C)]
pub struct SimpleProcess {
    pub id: ProcessID,
    pub name: [u8; 64],
    pub state: AtomicUsize, // ProcessState as usize
    pub priority: AtomicUsize, // ProcessPriority as usize
    pub cpu_time: AtomicUsize,
    pub capability: ProcessCapability,
}

impl SimpleProcess {
    pub fn new(id: ProcessID, name: &[u8], priority: ProcessPriority, capability: ProcessCapability) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);

        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }

        SimpleProcess {
            id,
            name: name_array,
            state: AtomicUsize::new(ProcessState::Ready as usize),
            priority: AtomicUsize::new(priority as usize),
            cpu_time: AtomicUsize::new(0),
            capability,
        }
    }

    pub fn get_state(&self) -> ProcessState {
        unsafe {
            core::mem::transmute(self.state.load(Ordering::SeqCst))
        }
    }

    pub fn set_state_atomic(&self, state: ProcessState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }

    pub fn get_priority(&self) -> ProcessPriority {
        unsafe {
            core::mem::transmute(self.priority.load(Ordering::SeqCst))
        }
    }

    pub fn set_priority_atomic(&self, priority: ProcessPriority) {
        self.priority.store(priority as usize, Ordering::SeqCst);
    }
}

impl Process for SimpleProcess {
    fn id(&self) -> ProcessID {
        self.id
    }

    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }

    fn state(&self) -> ProcessState {
        self.get_state()
    }

    fn priority(&self) -> ProcessPriority {
        self.get_priority()
    }

    fn set_state(&mut self, state: ProcessState) {
        if !self.capability.can_change_state {
            return;
        }
        self.set_state_atomic(state);
    }

    fn set_priority(&mut self, priority: ProcessPriority) {
        if !self.capability.can_change_priority {
            return;
        }
        self.set_priority_atomic(priority);
    }

    fn info(&self) -> ProcessInfo {
        ProcessInfo {
            id: self.id,
            name: self.name,
            state: self.get_state(),
            priority: self.get_priority(),
            cpu_time: self.cpu_time.load(Ordering::SeqCst) as u64,
            capability: self.capability,
        }
    }
}

/// Process scheduler trait (OOP interface)
pub trait ProcessScheduler {
    /// Create process
    fn create_process(&mut self, name: &[u8], priority: ProcessPriority) -> Result<ProcessID, SchedulerError>;
    /// Destroy process
    fn destroy_process(&mut self, id: ProcessID) -> Result<(), SchedulerError>;
    /// Schedule next process
    fn schedule(&mut self) -> Option<ProcessID>;
    /// Get process
    fn get_process(&self, id: ProcessID) -> Option<&dyn Process>;
    /// List processes
    fn list_processes(&self) -> Vec<ProcessID>;
    /// Get scheduler statistics
    fn stats(&self) -> SchedulerStats;
}

/// Scheduler error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SchedulerError {
    Success = 0,
    ProcessNotFound = 1,
    PermissionDenied = 2,
}

/// Scheduler statistics
#[repr(C)]
pub struct SchedulerStats {
    pub total_processes: usize,
    pub ready_processes: usize,
    pub running_processes: usize,
    pub blocked_processes: usize,
}

impl SchedulerStats {
    pub fn new() -> Self {
        SchedulerStats {
            total_processes: 0,
            ready_processes: 0,
            running_processes: 0,
            blocked_processes: 0,
        }
    }
}

/// Simple process scheduler (OOP: Concrete scheduler class)
pub struct SimpleProcessScheduler {
    processes: Vec<Option<Box<dyn Process>>>,
    next_id: AtomicUsize,
    stats: SchedulerStats,
    capability: SchedulerCapability,
}

/// Scheduler capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SchedulerCapability {
    pub can_create: bool,
    pub can_destroy: bool,
    pub can_schedule: bool,
}

impl SchedulerCapability {
    pub fn new() -> Self {
        SchedulerCapability {
            can_create: false,
            can_destroy: false,
            can_schedule: false,
        }
    }

    pub fn full() -> Self {
        SchedulerCapability {
            can_create: true,
            can_destroy: true,
            can_schedule: true,
        }
    }
}

impl SimpleProcessScheduler {
    pub fn new(capability: SchedulerCapability) -> Self {
        SimpleProcessScheduler {
            processes: Vec::new(),
            next_id: AtomicUsize::new(1),
            stats: SchedulerStats::new(),
            capability,
        }
    }
}

impl ProcessScheduler for SimpleProcessScheduler {
    fn create_process(&mut self, name: &[u8], priority: ProcessPriority) -> Result<ProcessID, SchedulerError> {
        if !self.capability.can_create {
            return Err(SchedulerError::PermissionDenied);
        }

        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let process = SimpleProcess::new(id, name, priority, ProcessCapability::full());
        self.processes.push(Some(Box::new(process)));
        self.stats.total_processes += 1;
        self.stats.ready_processes += 1;
        Ok(id)
    }

    fn destroy_process(&mut self, id: ProcessID) -> Result<(), SchedulerError> {
        if !self.capability.can_destroy {
            return Err(SchedulerError::PermissionDenied);
        }

        let mut index = None;
        let mut state = ProcessState::Ready;

        for (i, process_option) in self.processes.iter().enumerate() {
            if let Some(ref process) = *process_option {
                if process.id() == id {
                    index = Some(i);
                    state = process.state();
                    break;
                }
            }
        }

        if let Some(i) = index {
            self.processes[i] = None;
            self.stats.total_processes -= 1;

            match state {
                ProcessState::Ready => self.stats.ready_processes -= 1,
                ProcessState::Running => self.stats.running_processes -= 1,
                ProcessState::Blocked => self.stats.blocked_processes -= 1,
                ProcessState::Terminated => {}
            }

            Ok(())
        } else {
            Err(SchedulerError::ProcessNotFound)
        }
    }

    fn schedule(&mut self) -> Option<ProcessID> {
        if !self.capability.can_schedule {
            return None;
        }

        let mut highest_priority_process: Option<ProcessID> = None;
        let mut highest_priority = ProcessPriority::Idle;

        for process_option in &mut self.processes {
            if let Some(ref mut process) = *process_option {
                if process.state() == ProcessState::Ready {
                    let priority = process.priority();
                    if priority > highest_priority {
                        highest_priority = priority;
                        highest_priority_process = Some(process.id());
                    }
                }
            }
        }

        if let Some(id) = highest_priority_process {
            for process_option in &mut self.processes {
                if let Some(ref mut process) = *process_option {
                    if process.id() == id {
                        process.set_state(ProcessState::Running);
                        self.stats.ready_processes -= 1;
                        self.stats.running_processes += 1;
                        break;
                    }
                }
            }
        }

        highest_priority_process
    }

    fn get_process(&self, id: ProcessID) -> Option<&dyn Process> {
        for process_option in &self.processes {
            if let Some(ref process) = *process_option {
                if process.id() == id {
                    return Some(process.as_ref());
                }
            }
        }
        None
    }

    fn list_processes(&self) -> Vec<ProcessID> {
        let mut ids = Vec::new();
        for process_option in &self.processes {
            if let Some(ref process) = *process_option {
                ids.push(process.id());
            }
        }
        ids
    }

    fn stats(&self) -> SchedulerStats {
        self.stats
    }
}

/// Simple Vec implementation for no_std
struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }

            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;

        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }

            if self.capacity > 0 {
                free(self.data as *mut u8);
            }

            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
