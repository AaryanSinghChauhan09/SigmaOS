// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// OOP-based scheduler traits for SigmaOS
// Zero-allocation, performance-optimized scheduling interfaces

/// Core scheduler trait - all schedulers must implement this
pub trait Scheduler {
    /// Initialize the scheduler
    fn init(&mut self) -> Result<(), SchedulerError>;
    
    /// Get scheduler name
    fn name(&self) -> &str;
    
    /// Get scheduler type
    fn scheduler_type(&self) -> SchedulerType;
    
    /// Add task to scheduler
    fn add_task(&mut self, task: Box<dyn Task>) -> Result<TaskId, SchedulerError>;
    
    /// Remove task from scheduler
    fn remove_task(&mut self, task_id: TaskId) -> Result<(), SchedulerError>;
    
    /// Get next task to run
    fn next_task(&mut self) -> Option<Box<dyn Task>>;
    
    /// Yield current task
    fn yield_task(&mut self, task_id: TaskId) -> Result<(), SchedulerError>;
    
    /// Block current task
    fn block_task(&mut self, task_id: TaskId) -> Result<(), SchedulerError>;
    
    /// Unblock task
    fn unblock_task(&mut self, task_id: TaskId) -> Result<(), SchedulerError>;
    
    /// Get task count
    fn task_count(&self) -> usize;
    
    /// Get scheduler statistics
    fn stats(&self) -> SchedulerStats;
}

/// Task trait for schedulable entities
pub trait Task {
    /// Get task ID
    fn task_id(&self) -> TaskId;
    
    /// Get task priority
    fn priority(&self) -> TaskPriority;
    
    /// Get task state
    fn state(&self) -> TaskState;
    
    /// Set task state
    fn set_state(&mut self, state: TaskState);
    
    /// Execute task
    fn execute(&mut self) -> TaskResult;
    
    /// Get task name
    fn name(&self) -> &str;
    
    /// Get CPU affinity
    fn cpu_affinity(&self) -> u32;
    
    /// Set CPU affinity
    fn set_cpu_affinity(&mut self, cpu: u32);
    
    /// Get time slice
    fn time_slice(&self) -> u64;
    
    /// Set time slice
    fn set_time_slice(&mut self, slice: u64);
    
    /// Get execution time
    fn execution_time(&self) -> u64;
    
    /// Get wait time
    fn wait_time(&self) -> u64;
}

/// Priority scheduler trait
pub trait PriorityScheduler: Scheduler {
    /// Set task priority
    fn set_priority(&mut self, task_id: TaskId, priority: TaskPriority) -> Result<(), SchedulerError>;
    
    /// Get task priority
    fn get_priority(&self, task_id: TaskId) -> Option<TaskPriority>;
    
    /// Get highest priority task
    fn highest_priority_task(&self) -> Option<TaskId>;
}

/// Real-time scheduler trait
pub trait RealTimeScheduler: Scheduler {
    /// Set task deadline
    fn set_deadline(&mut self, task_id: TaskId, deadline: u64) -> Result<(), SchedulerError>;
    
    /// Get task deadline
    fn get_deadline(&self, task_id: TaskId) -> Option<u64>;
    
    /// Get task with earliest deadline
    fn earliest_deadline_task(&self) -> Option<TaskId>;
}

/// Fair scheduler trait
pub trait FairScheduler: Scheduler {
    /// Get task CPU usage
    fn get_cpu_usage(&self, task_id: TaskId) -> Option<f64>;
    
    /// Get task runtime
    fn get_runtime(&self, task_id: TaskId) -> Option<u64>;
    
    /// Balance load across CPUs
    fn balance_load(&mut self) -> Result<(), SchedulerError>;
}

/// CPU affinity scheduler trait
pub trait AffinityScheduler: Scheduler {
    /// Set task CPU affinity
    fn set_affinity(&mut self, task_id: TaskId, cpu: u32) -> Result<(), SchedulerError>;
    
    /// Get task CPU affinity
    fn get_affinity(&self, task_id: TaskId) -> Option<u32>;
    
    /// Get tasks for specific CPU
    fn get_tasks_for_cpu(&self, cpu: u32) -> Vec<TaskId>;
}

/// Scheduler error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulerError {
    TaskNotFound,
    InvalidTask,
    InvalidPriority,
    InvalidState,
    SchedulerFull,
    TaskAlreadyExists,
    NotSupported,
    Other,
}

/// Scheduler types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulerType {
    RoundRobin,
    Priority,
    RealTime,
    Fair,
    CFS,
    Other,
}

/// Task ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(pub u64);

impl TaskId {
    pub const fn new(id: u64) -> Self {
        Self(id)
    }
}

/// Task priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    Idle = 0,
    Low = 1,
    Normal = 2,
    High = 3,
    RealTime = 4,
}

/// Task state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
    Sleeping,
    Zombie,
    Stopped,
}

/// Task execution result
#[derive(Debug, Clone, Copy)]
pub enum TaskResult {
    Completed,
    Yielded,
    Blocked,
    Error(TaskError),
}

/// Task error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskError {
    InvalidOperation,
    ResourceUnavailable,
    Timeout,
    Cancelled,
    Other,
}

/// Scheduler statistics
#[derive(Debug, Clone, Copy)]
pub struct SchedulerStats {
    pub total_tasks: usize,
    pub ready_tasks: usize,
    pub running_tasks: usize,
    pub blocked_tasks: usize,
    pub total_switches: u64,
    pub total_runtime: u64,
    pub average_wait_time: u64,
    pub cpu_utilization: f64,
}

impl SchedulerStats {
    pub const fn new() -> Self {
        Self {
            total_tasks: 0,
            ready_tasks: 0,
            running_tasks: 0,
            blocked_tasks: 0,
            total_switches: 0,
            total_runtime: 0,
            average_wait_time: 0,
            cpu_utilization: 0.0,
        }
    }
}

/// Scheduler policy trait
pub trait SchedulerPolicy {
    /// Should preempt current task?
    fn should_preempt(&self, current: &dyn Task, next: &dyn Task) -> bool;
    
    /// Calculate task score for scheduling
    fn calculate_score(&self, task: &dyn Task) -> i64;
    
    /// Compare two tasks
    fn compare_tasks(&self, a: &dyn Task, b: &dyn Task) -> std::cmp::Ordering;
}

/// Time slice manager trait
pub trait TimeSliceManager {
    /// Get default time slice
    fn default_time_slice(&self) -> u64;
    
    /// Calculate time slice for task
    fn calculate_time_slice(&self, task: &dyn Task) -> u64;
    
    /// Adjust time slice based on behavior
    fn adjust_time_slice(&mut self, task: &dyn Task, behavior: TaskBehavior) -> u64;
}

/// Task behavior for time slice adjustment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskBehavior {
    CPUIntensive,
    IOIntensive,
    Interactive,
    Batch,
}

/// Load balancer trait for multi-CPU scheduling
pub trait LoadBalancer {
    /// Find least loaded CPU
    fn find_least_loaded_cpu(&self) -> Option<u32>;
    
    /// Migrate task to different CPU
    fn migrate_task(&mut self, task_id: TaskId, target_cpu: u32) -> Result<(), SchedulerError>;
    
    /// Get CPU load
    fn get_cpu_load(&self, cpu: u32) -> f64;
    
    /// Balance all CPUs
    fn balance_all(&mut self) -> Result<(), SchedulerError>;
}
