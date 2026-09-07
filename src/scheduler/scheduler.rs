use core::default::Default;
use core::option::Option::{self, None, Some};
use core::result::Result::{self, Err, Ok};
/// OOP-based Scheduler for SigmaOS
/// Implements process/thread scheduling using Linux & BSD inspired task states and workload classifications.
use std::boxed::Box;
use std::vec::Vec;
use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

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
    /// Get workload type
    fn workload_type(&self) -> TaskWorkloadType;
    /// Set workload type
    fn set_workload_type(&mut self, workload: TaskWorkloadType);
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
    /// Get task capability
    fn capability(&self) -> TaskCapability;
    /// Check if task can yield
    fn can_yield(&self) -> bool;
    /// Check if task can block
    fn can_block(&self) -> bool;
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

/// Linux & BSD inspired Task State
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Running = 0,
    Ready = 1,
    WaitingBlocked = 2,
    SuspendedStopped = 3,
    TerminatedZombie = 4,
    // Compatibility aliases
    Blocked = 5,
    Sleeping = 6,
    Terminated = 7,
}

/// Task Workload Type classifications
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskWorkloadType {
    CpuBound,
    IoBound,
    Interactive,
    Batch,
    RealTimePeriodic { period_ms: u64, exec_time_ms: u64 },
    RealTimeAperiodic { deadline_ms: u64 },
    SystemKernelDaemon,
}

/// Task (OOP: Schedulable object)
pub struct Task {
    pub id: usize,
    pub priority: Priority,
    pub state: AtomicUsize, // TaskState as usize
    pub workload_type: TaskWorkloadType,
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
            workload_type: TaskWorkloadType::Interactive,
            cpu_time: AtomicU64::new(0),
            last_run_time: AtomicU64::new(0),
            quantum,
            capability,
        }
    }

    pub fn with_workload(mut self, workload: TaskWorkloadType) -> Self {
        self.workload_type = workload;
        self
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
        let raw = self.state.load(Ordering::SeqCst);
        match raw {
            0 => TaskState::Running,
            1 => TaskState::Ready,
            2 | 5 | 6 => TaskState::WaitingBlocked,
            3 => TaskState::SuspendedStopped,
            4 | 7 => TaskState::TerminatedZombie,
            _ => TaskState::Ready,
        }
    }

    fn set_state(&mut self, state: TaskState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }

    fn workload_type(&self) -> TaskWorkloadType {
        self.workload_type
    }

    fn set_workload_type(&mut self, workload: TaskWorkloadType) {
        self.workload_type = workload;
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

    fn capability(&self) -> TaskCapability {
        self.capability
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

impl Default for TaskCapability {
    fn default() -> Self {
        Self::new()
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy)]
pub struct SchedulerStats {
    pub total_tasks: usize,
    pub ready_tasks: usize,
    pub running_tasks: usize,
    pub blocked_tasks: usize,
    pub suspended_tasks: usize,
    pub zombie_tasks: usize,
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
            suspended_tasks: 0,
            zombie_tasks: 0,
            context_switches: 0,
            cpu_utilization: 0,
        }
    }
}

impl Default for SchedulerStats {
    fn default() -> Self {
        Self::new()
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

        for i in 0..self.ready_queue.len() {
            if let Some(ref mut task) = self.ready_queue[i] {
                if task.state() == TaskState::Ready {
                    let task_id = task.task_id();
                    task.set_state(TaskState::Running);
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
                    task.set_state(TaskState::WaitingBlocked);
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
                    TaskState::WaitingBlocked | TaskState::Blocked | TaskState::Sleeping => {
                        stats.blocked_tasks += 1
                    }
                    TaskState::SuspendedStopped => stats.suspended_tasks += 1,
                    TaskState::TerminatedZombie | TaskState::Terminated => stats.zombie_tasks += 1,
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

impl Default for PriorityScheduler {
    fn default() -> Self {
        Self::new()
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
        for i in (0..5).rev() {
            for task_option in &mut self.priority_queues[i] {
                if let Some(ref mut task) = *task_option {
                    if task.state() == TaskState::Ready {
                        let task_id = task.task_id();
                        task.set_state(TaskState::Running);
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
                        task.set_state(TaskState::WaitingBlocked);
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
                        TaskState::WaitingBlocked | TaskState::Blocked | TaskState::Sleeping => {
                            stats.blocked_tasks += 1
                        }
                        TaskState::SuspendedStopped => stats.suspended_tasks += 1,
                        TaskState::TerminatedZombie | TaskState::Terminated => {
                            stats.zombie_tasks += 1
                        }
                    }
                }
            }
        }

        stats.context_switches = self.context_switches.load(Ordering::SeqCst);
        stats
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_task_states_and_workload_classification() {
        let mut task = Task::new(1, Priority::High, 10, TaskCapability::full()).with_workload(
            TaskWorkloadType::RealTimePeriodic {
                period_ms: 10,
                exec_time_ms: 2,
            },
        );

        assert_eq!(task.state(), TaskState::Ready);
        assert_eq!(
            task.workload_type(),
            TaskWorkloadType::RealTimePeriodic {
                period_ms: 10,
                exec_time_ms: 2
            }
        );

        task.set_state(TaskState::Running);
        assert_eq!(task.state(), TaskState::Running);

        task.set_state(TaskState::WaitingBlocked);
        assert_eq!(task.state(), TaskState::WaitingBlocked);

        task.set_state(TaskState::SuspendedStopped);
        assert_eq!(task.state(), TaskState::SuspendedStopped);

        task.set_state(TaskState::TerminatedZombie);
        assert_eq!(task.state(), TaskState::TerminatedZombie);
    }

    #[test]
    fn test_priority_scheduler_workload() {
        let mut sched = PriorityScheduler::new();
        let task1 = Box::new(
            Task::new(101, Priority::High, 20, TaskCapability::full())
                .with_workload(TaskWorkloadType::Interactive),
        );
        let task2 = Box::new(
            Task::new(102, Priority::Realtime, 5, TaskCapability::full())
                .with_workload(TaskWorkloadType::SystemKernelDaemon),
        );

        sched.add_task(task1).unwrap();
        sched.add_task(task2).unwrap();

        // Realtime task should schedule first
        let next_id = sched.schedule().unwrap();
        assert_eq!(next_id, 102);

        let stats = sched.stats();
        assert_eq!(stats.total_tasks, 2);
        assert_eq!(stats.running_tasks, 1);
        assert_eq!(stats.ready_tasks, 1);
    }
}
