#![no_std]
#![no_main]

use core::mem;
/// OOP-based Scheduler for SigmaOS
/// Implements process/thread scheduling using OOP principles with traits and structs
/// No dependency on external scheduler frameworks
use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

/// Schedulable trait (OOP interface)
pub trait Schedulable {
    /// Get priority
    fn priority(&self) -> Priority;
    /// Set priority
    fn set_priority(&mut self, priority: Priority);
    /// Get state
    fn state(&self) -> TaskState;
    /// Set state
    fn set_state(&mut self, state: TaskState);
    /// Get CPU time used
    fn cpu_time(&self) -> u64;
    /// Increment CPU time
    fn increment_cpu_time(&mut self, time: u64);
    /// Get last run time
    fn last_run_time(&self) -> u64;
    /// Set last run time
    fn set_last_run_time(&mut self, time: u64);
    /// Get task ID
    fn task_id(&self) -> usize;
    /// Can yield
    fn can_yield(&self) -> bool { true }
    /// Can block
    fn can_block(&self) -> bool { true }
}

/// Priority levels
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Idle = 0,
    Low = 1,
    Normal = 2,
    High = 3,
    Realtime = 4,
}

/// Task state
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Ready = 0,
    Running = 1,
    Blocked = 2,
    Sleeping = 3,
    Terminated = 4,
}

/// Task (OOP: Schedulable object)
#[repr(C)]
pub struct Task {
    pub id: usize,
    pub priority: Priority,
    pub state: AtomicUsize, // TaskState as usize
    pub cpu_time: AtomicU64,
    pub last_run_time: AtomicU64,
    pub quantum: u64,
    pub capability: TaskCapability,
}

impl Task {
    pub fn new(id: usize, priority: Priority, quantum: u64, capability: TaskCapability) -> Self {
        Task {
            id,
            priority,
            state: AtomicUsize::new(TaskState::Ready as usize),
            cpu_time: AtomicU64::new(0),
            last_run_time: AtomicU64::new(0),
            quantum,
            capability,
        }
    }
}

impl Schedulable for Task {
    fn priority(&self) -> Priority {
        self.priority
    }

    fn set_priority(&mut self, priority: Priority) {
        self.priority = priority;
    }

    fn state(&self) -> TaskState {
        {
            let raw = self.state.load(Ordering::SeqCst) as u32;
            match raw {
                1 => TaskState::Running,
                2 => TaskState::Blocked,
                3 => TaskState::Sleeping,
                4 => TaskState::Terminated,
                _ => TaskState::Ready,
            }
        }
    }

    fn set_state(&mut self, state: TaskState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }

    fn cpu_time(&self) -> u64 {
        self.cpu_time.load(Ordering::SeqCst)
    }

    fn increment_cpu_time(&mut self, time: u64) {
        self.cpu_time.fetch_add(time, Ordering::SeqCst);
    }

    fn last_run_time(&self) -> u64 {
        self.last_run_time.load(Ordering::SeqCst)
    }

    fn set_last_run_time(&mut self, time: u64) {
        self.last_run_time.store(time, Ordering::SeqCst);
    }

    fn task_id(&self) -> usize {
        self.id
    }

    fn can_yield(&self) -> bool {
        self.capability.can_yield
    }

    fn can_block(&self) -> bool {
        self.capability.can_block
    }
}

/// Task capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TaskCapability {
    pub can_yield: bool,
    pub can_block: bool,
    pub can_terminate: bool,
    pub can_set_priority: bool,
}

impl TaskCapability {
    pub fn new() -> Self {
        TaskCapability {
            can_yield: false,
            can_block: false,
            can_terminate: false,
            can_set_priority: false,
        }
    }

    pub fn full() -> Self {
        TaskCapability {
            can_yield: true,
            can_block: true,
            can_terminate: true,
            can_set_priority: true,
        }
    }
}

/// Scheduler trait (OOP interface)
pub trait Scheduler {
    /// Add task to scheduler
    fn add_task(&mut self, task: Box<dyn Schedulable>) -> Result<(), SchedulerError>;
    /// Remove task from scheduler
    fn remove_task(&mut self, task_id: usize) -> Result<(), SchedulerError>;
    /// Get next task to run
    fn schedule(&mut self) -> Option<usize>;
    /// Yield current task
    fn yield_task(&mut self, task_id: usize) -> Result<(), SchedulerError>;
    /// Block task
    fn block_task(&mut self, task_id: usize) -> Result<(), SchedulerError>;
    /// Unblock task
    fn unblock_task(&mut self, task_id: usize) -> Result<(), SchedulerError>;
    /// Get scheduler statistics
    fn stats(&self) -> SchedulerStats;
}

/// Scheduler error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SchedulerError {
    Success = 0,
    TaskNotFound = 1,
    TaskAlreadyExists = 2,
    InvalidState = 3,
    PermissionDenied = 4,
    QueueFull = 5,
}

/// Scheduler statistics
#[repr(C)]
pub struct SchedulerStats {
    pub total_tasks: usize,
    pub ready_tasks: usize,
    pub running_tasks: usize,
    pub blocked_tasks: usize,
    pub context_switches: u64,
    pub cpu_utilization: u32,
}

impl SchedulerStats {
    pub fn new() -> Self {
        SchedulerStats {
            total_tasks: 0,
            ready_tasks: 0,
            running_tasks: 0,
            blocked_tasks: 0,
            context_switches: 0,
            cpu_utilization: 0,
        }
    }
}

/// Round-robin scheduler (OOP: Concrete scheduler class)
pub struct RoundRobinScheduler {
    ready_queue: Vec<Option<Box<dyn Schedulable>>>,
    current_task: AtomicUsize,
    context_switches: AtomicU64,
    quantum: u64,
}

impl RoundRobinScheduler {
    pub fn new(quantum: u64) -> Self {
        RoundRobinScheduler {
            ready_queue: Vec::new(),
            current_task: AtomicUsize::new(0),
            context_switches: AtomicU64::new(0),
            quantum,
        }
    }
}

impl Scheduler for RoundRobinScheduler {
    fn add_task(&mut self, task: Box<dyn Schedulable>) -> Result<(), SchedulerError> {
        self.ready_queue.push(Some(task));
        Ok(())
    }

    fn remove_task(&mut self, task_id: usize) -> Result<(), SchedulerError> {
        for i in 0..self.ready_queue.len() {
            if let Some(ref task) = self.ready_queue[i] {
                if task.task_id() == task_id {
                    self.ready_queue.remove(i);
                    return Ok(());
                }
            }
        }
        Err(SchedulerError::TaskNotFound)
    }

    fn schedule(&mut self) -> Option<usize> {
        if self.ready_queue.is_empty() {
            return None;
        }

        // Find next ready task
        for i in 0..self.ready_queue.len() {
            if let Some(ref task) = self.ready_queue[i] {
                if task.state() == TaskState::Ready {
                    let task_id = task.task_id();
                    self.current_task.store(task_id, Ordering::SeqCst);
                    self.context_switches.fetch_add(1, Ordering::SeqCst);
                    return Some(task_id);
                }
            }
        }

        None
    }

    fn yield_task(&mut self, task_id: usize) -> Result<(), SchedulerError> {
        for task_option in &mut self.ready_queue {
            if let Some(ref mut task) = *task_option {
                if task.task_id() == task_id {
                    if !task.can_yield() {
                        return Err(SchedulerError::PermissionDenied);
                    }
                    task.set_state(TaskState::Ready);
                    return Ok(());
                }
            }
        }
        Err(SchedulerError::TaskNotFound)
    }

    fn block_task(&mut self, task_id: usize) -> Result<(), SchedulerError> {
        for task_option in &mut self.ready_queue {
            if let Some(ref mut task) = *task_option {
                if task.task_id() == task_id {
                    if !task.can_block() {
                        return Err(SchedulerError::PermissionDenied);
                    }
                    task.set_state(TaskState::Blocked);
                    return Ok(());
                }
            }
        }
        Err(SchedulerError::TaskNotFound)
    }

    fn unblock_task(&mut self, task_id: usize) -> Result<(), SchedulerError> {
        for task_option in &mut self.ready_queue {
            if let Some(ref mut task) = *task_option {
                if task.task_id() == task_id {
                    task.set_state(TaskState::Ready);
                    return Ok(());
                }
            }
        }
        Err(SchedulerError::TaskNotFound)
    }

    fn stats(&self) -> SchedulerStats {
        let mut stats = SchedulerStats::new();
        stats.total_tasks = self.ready_queue.len();

        for task_option in &self.ready_queue {
            if let Some(ref task) = *task_option {
                match task.state() {
                    TaskState::Ready => stats.ready_tasks += 1,
                    TaskState::Running => stats.running_tasks += 1,
                    TaskState::Blocked => stats.blocked_tasks += 1,
                    _ => {}
                }
            }
        }

        stats.context_switches = self.context_switches.load(Ordering::SeqCst);
        stats
    }
}

/// Priority scheduler (OOP: Concrete scheduler class)
pub struct PriorityScheduler {
    priority_queues: [Vec<Option<Box<dyn Schedulable>>>; 5], // One queue per priority
    current_task: AtomicUsize,
    context_switches: AtomicU64,
}

impl PriorityScheduler {
    pub fn new() -> Self {
        PriorityScheduler {
            priority_queues: [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()],
            current_task: AtomicUsize::new(0),
            context_switches: AtomicU64::new(0),
        }
    }

    fn get_priority_index(priority: Priority) -> usize {
        priority as usize
    }
}

impl Scheduler for PriorityScheduler {
    fn add_task(&mut self, task: Box<dyn Schedulable>) -> Result<(), SchedulerError> {
        let priority_index = Self::get_priority_index(task.priority());
        if priority_index < 5 {
            self.priority_queues[priority_index].push(Some(task));
            Ok(())
        } else {
            Err(SchedulerError::InvalidState)
        }
    }

    fn remove_task(&mut self, task_id: usize) -> Result<(), SchedulerError> {
        for queue in &mut self.priority_queues {
            for i in 0..queue.len() {
                if let Some(ref task) = queue[i] {
                    if task.task_id() == task_id {
                        queue.remove(i);
                        return Ok(());
                    }
                }
            }
        }
        Err(SchedulerError::TaskNotFound)
    }

    fn schedule(&mut self) -> Option<usize> {
        // Check queues from highest to lowest priority
        for i in (0..5).rev() {
            for task_option in &mut self.priority_queues[i] {
                if let Some(ref task) = *task_option {
                    if task.state() == TaskState::Ready {
                        let task_id = task.task_id();
                        self.current_task.store(task_id, Ordering::SeqCst);
                        self.context_switches.fetch_add(1, Ordering::SeqCst);
                        return Some(task_id);
                    }
                }
            }
        }

        None
    }

    fn yield_task(&mut self, task_id: usize) -> Result<(), SchedulerError> {
        for queue in &mut self.priority_queues {
            for task_option in queue {
                if let Some(ref mut task) = *task_option {
                    if task.task_id() == task_id {
                        if !task.can_yield() {
                            return Err(SchedulerError::PermissionDenied);
                        }
                        task.set_state(TaskState::Ready);
                        return Ok(());
                    }
                }
            }
        }
        Err(SchedulerError::TaskNotFound)
    }

    fn block_task(&mut self, task_id: usize) -> Result<(), SchedulerError> {
        for queue in &mut self.priority_queues {
            for task_option in queue {
                if let Some(ref mut task) = *task_option {
                    if task.task_id() == task_id {
                        if !task.can_block() {
                            return Err(SchedulerError::PermissionDenied);
                        }
                        task.set_state(TaskState::Blocked);
                        return Ok(());
                    }
                }
            }
        }
        Err(SchedulerError::TaskNotFound)
    }

    fn unblock_task(&mut self, task_id: usize) -> Result<(), SchedulerError> {
        for queue in &mut self.priority_queues {
            for task_option in queue {
                if let Some(ref mut task) = *task_option {
                    if task.task_id() == task_id {
                        task.set_state(TaskState::Ready);
                        return Ok(());
                    }
                }
            }
        }
        Err(SchedulerError::TaskNotFound)
    }

    fn stats(&self) -> SchedulerStats {
        let mut stats = SchedulerStats::new();

        for queue in &self.priority_queues {
            stats.total_tasks += queue.len();
            for task_option in queue {
                if let Some(ref task) = *task_option {
                    match task.state() {
                        TaskState::Ready => stats.ready_tasks += 1,
                        TaskState::Running => stats.running_tasks += 1,
                        TaskState::Blocked => stats.blocked_tasks += 1,
                        _ => {}
                    }
                }
            }
        }

        stats.context_switches = self.context_switches.load(Ordering::SeqCst);
        stats
    }
}


