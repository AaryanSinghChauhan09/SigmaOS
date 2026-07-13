#![no_std]
#![no_main]

/// OOP-based Kernel Core for SigmaOS
/// Implements kernel core using OOP principles with traits and structs
/// No dependency on external kernel frameworks
/// Based on Roadmap Item 3: Kernel core

use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

/// Task ID
pub type TaskID = usize;

/// Task state
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum TaskState {
    Ready = 0,
    Running = 1,
    Blocked = 2,
    Terminated = 3,
}

/// Kernel task trait (OOP interface)
pub trait KernelTask {
    /// Get task ID
    fn id(&self) -> TaskID;
    /// Get task name
    fn name(&self) -> &[u8];
    /// Get task state
    fn state(&self) -> TaskState;
    /// Execute task
    fn execute(&mut self) -> Result<(), KernelError>;
    /// Get task info
    fn info(&self) -> TaskInfo;
}

/// Kernel error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum KernelError {
    Success = 0,
    TaskNotFound = 1,
    ExecutionFailed = 2,
    PermissionDenied = 3,
}

/// Task info
#[repr(C)]
pub struct TaskInfo {
    pub id: TaskID,
    pub name: [u8; 64],
    pub state: TaskState,
    pub capability: TaskCapability,
}

impl TaskInfo {
    pub fn new(id: TaskID) -> Self {
        TaskInfo {
            id,
            name: [0; 64],
            state: TaskState::Ready,
            capability: TaskCapability::new(),
        }
    }
}

/// Task capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TaskCapability {
    pub can_execute: bool,
    pub can_terminate: bool,
}

impl TaskCapability {
    pub fn new() -> Self {
        TaskCapability {
            can_execute: false,
            can_terminate: false,
        }
    }

    pub fn full() -> Self {
        TaskCapability {
            can_execute: true,
            can_terminate: true,
        }
    }
}

/// Simple kernel task (OOP: Concrete task class)
#[repr(C)]
pub struct SimpleKernelTask {
    pub id: TaskID,
    pub name: [u8; 64],
    pub state: AtomicUsize, // TaskState as usize
    pub capability: TaskCapability,
}

impl SimpleKernelTask {
    pub fn new(id: TaskID, name: &[u8], capability: TaskCapability) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);

        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }

        SimpleKernelTask {
            id,
            name: name_array,
            state: AtomicUsize::new(TaskState::Ready as usize),
            capability,
        }
    }

    pub fn get_state(&self) -> TaskState {
        unsafe {
            core::mem::transmute(self.state.load(Ordering::SeqCst))
        }
    }

    pub fn set_state(&self, state: TaskState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }
}

impl KernelTask for SimpleKernelTask {
    fn id(&self) -> TaskID {
        self.id
    }

    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }

    fn state(&self) -> TaskState {
        self.get_state()
    }

    fn execute(&mut self) -> Result<(), KernelError> {
        if !self.capability.can_execute {
            return Err(KernelError::PermissionDenied);
        }

        self.set_state(TaskState::Running);

        // In a real implementation, this would execute the task
        self.set_state(TaskState::Ready);
        Ok(())
    }

    fn info(&self) -> TaskInfo {
        TaskInfo {
            id: self.id,
            name: self.name,
            state: self.get_state(),
            capability: self.capability,
        }
    }
}

/// Kernel core trait (OOP interface)
pub trait KernelCore {
    /// Register task
    fn register_task(&mut self, task: Box<dyn KernelTask>) -> Result<TaskID, KernelError>;
    /// Unregister task
    fn unregister_task(&mut self, id: TaskID) -> Result<(), KernelError>;
    /// Execute task
    fn execute_task(&mut self, id: TaskID) -> Result<(), KernelError>;
    /// Get task
    fn get_task(&self, id: TaskID) -> Option<&dyn KernelTask>;
    /// List tasks
    fn list_tasks(&self) -> Vec<TaskID>;
    /// Get kernel statistics
    fn stats(&self) -> KernelStats;
}

/// Kernel statistics
#[repr(C)]
pub struct KernelStats {
    pub total_tasks: usize,
    pub running_tasks: usize,
    pub ready_tasks: usize,
}

impl KernelStats {
    pub fn new() -> Self {
        KernelStats {
            total_tasks: 0,
            running_tasks: 0,
            ready_tasks: 0,
        }
    }
}

/// Simple kernel core (OOP: Concrete kernel class)
pub struct SimpleKernelCore {
    tasks: Vec<Option<Box<dyn KernelTask>>>,
    next_id: AtomicUsize,
    stats: KernelStats,
    capability: CoreCapability,
}

/// Core capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CoreCapability {
    pub can_register: bool,
    pub can_execute: bool,
}

impl CoreCapability {
    pub fn new() -> Self {
        CoreCapability {
            can_register: false,
            can_execute: false,
        }
    }

    pub fn full() -> Self {
        CoreCapability {
            can_register: true,
            can_execute: true,
        }
    }
}

impl SimpleKernelCore {
    pub fn new(capability: CoreCapability) -> Self {
        SimpleKernelCore {
            tasks: Vec::new(),
            next_id: AtomicUsize::new(1),
            stats: KernelStats::new(),
            capability,
        }
    }
}

impl KernelCore for SimpleKernelCore {
    fn register_task(&mut self, task: Box<dyn KernelTask>) -> Result<TaskID, KernelError> {
        if !self.capability.can_register {
            return Err(KernelError::PermissionDenied);
        }

        let id = task.id();
        self.tasks.push(Some(task));
        self.stats.total_tasks += 1;
        self.stats.ready_tasks += 1;
        Ok(id)
    }

    fn unregister_task(&mut self, id: TaskID) -> Result<(), KernelError> {
        if !self.capability.can_register {
            return Err(KernelError::PermissionDenied);
        }

        let mut index = None;
        for (i, task_option) in self.tasks.iter().enumerate() {
            if let Some(ref task) = *task_option {
                if task.id() == id {
                    index = Some(i);
                    break;
                }
            }
        }

        if let Some(i) = index {
            self.tasks[i] = None;
            self.stats.total_tasks -= 1;
            Ok(())
        } else {
            Err(KernelError::TaskNotFound)
        }
    }

    fn execute_task(&mut self, id: TaskID) -> Result<(), KernelError> {
        if !self.capability.can_execute {
            return Err(KernelError::PermissionDenied);
        }

        for task_option in &mut self.tasks {
            if let Some(ref mut task) = *task_option {
                if task.id() == id {
                    let result = task.execute();
                    if result.is_ok() {
                        self.stats.ready_tasks += 1;
                    }
                    return result;
                }
            }
        }
        Err(KernelError::TaskNotFound)
    }

    fn get_task(&self, id: TaskID) -> Option<&dyn KernelTask> {
        for task_option in &self.tasks {
            if let Some(ref task) = *task_option {
                if task.id() == id {
                    return Some(task.as_ref());
                }
            }
        }
        None
    }

    fn list_tasks(&self) -> Vec<TaskID> {
        let mut ids = Vec::new();
        for task_option in &self.tasks {
            if let Some(ref task) = *task_option {
                ids.push(task.id());
            }
        }
        ids
    }

    fn stats(&self) -> KernelStats {
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
