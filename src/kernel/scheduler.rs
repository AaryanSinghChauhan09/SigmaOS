// Process Scheduler Enhancements
// Inspired by Linux CFS, RT scheduler, and energy-aware scheduling

use std::collections::{BinaryHeap, HashMap};
use std::sync::atomic::{AtomicU64, Ordering};

/// Process priority — encapsulated with invariant validation.
///
/// Invariant: priority value must be in range [-20, 19] where lower values
/// indicate higher priority (real-time -20 > normal 0 > idle 19).
///
/// NOTE: This struct uses const fn for associated constants to support
/// constant evaluation, while the `new()` constructor uses `assert!` to
/// validate the invariant at runtime for non-const contexts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Priority {
    value: i32, // -20 to 19 (lower is higher priority)
}

impl Priority {
    /// Realtime priority (-20, highest)
    pub const High: Self = Self::new_const(-20);

    /// Normal priority (0, default)
    pub const Normal: Self = Self::new_const(0);

    /// Low priority (10)
    pub const Low: Self = Self::new_const(10);

    /// Idle priority (19, lowest)
    pub const Idle: Self = Self::new_const(19);

    /// Realtime priority (-20, highest)
    #[doc(hidden)]
    pub const Realtime: Self = Self::new_const(-20);

    /// Create a priority with invariant validation for const context.
    ///
    /// Safe for const evaluation: no assertions, just direct assignment.
    /// The caller must ensure the value is in [-20, 19].
    const fn new_const(value: i32) -> Self {
        Self { value }
    }

    /// Create a priority with runtime invariant validation.
    ///
    /// # Panics
    ///
    /// Panics if `value` is outside the valid range [-20, 19].
    /// This ensures the priority invariant is always upheld.
    pub fn new(value: i32) -> Self {
        assert!(
            (-20..=19).contains(&value),
            "Priority value must be in range [-20, 19], got {}",
            value
        );
        Self { value }
    }

    /// Return the internal priority value (query).
    ///
    /// Follows command-query separation: this function only reads state,
    /// it does not modify any scheduler state.
    pub fn value(&self) -> i32 {
        self.value
    }

    /// Return the highest possible priority.
    pub fn highest() -> Self {
        Self::new(-20)
    }

    /// Return the lowest possible priority.
    pub fn lowest() -> Self {
        Self::new(19)
    }

    /// Return the realtime priority constant.
    pub fn realtime() -> Self {
        Self::new(-20)
    }

    /// Return the high priority constant.
    pub fn high() -> Self {
        Self::new(-10)
    }

    /// Return the normal/default priority constant.
    pub fn normal() -> Self {
        Self::new(0)
    }

    /// Return the low priority constant.
    pub fn low() -> Self {
        Self::new(10)
    }

    /// Return the idle/lowest priority constant.
    pub fn idle() -> Self {
        Self::new(19)
    }
}

/// Scheduler policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulerPolicy {
    Normal, // CFS
    Realtime,
    Idle,
    Batch,
}

/// Process state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    New,
    Running,
    Runnable,
    Sleeping,
    Stopped,
    Zombie,
    BlockedWaiting,
    BlockedSuspended,
    Blocked, // For compatibility
}

impl ProcessState {
    pub const Ready: Self = ProcessState::Runnable;
}

/// Process task
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessTask {
    pid: u64,
    pub priority: Priority,
    pub policy: SchedulerPolicy,
    pub state: ProcessState,
    vruntime: u64,      // Virtual runtime for CFS
    exec_start: u64,    // Execution start time
    exec_duration: u64, // Total execution duration
    cpu_time: u64,      // CPU time used
    slice: u64,         // Time slice
}

impl ProcessTask {
    pub fn new(pid: u64, _name: String, priority: Priority, policy: SchedulerPolicy) -> Self {
        Self {
            pid,
            priority,
            policy,
            state: ProcessState::Runnable,
            vruntime: 0,
            exec_start: 0,
            exec_duration: 0,
            cpu_time: 0,
            slice: 10000,
        }
    }

    pub fn new_simple(pid: u64, priority: Priority) -> Self {
        Self {
            pid,
            priority,
            policy: SchedulerPolicy::Normal,
            state: ProcessState::Runnable,
            vruntime: 0,
            exec_start: 0,
            exec_duration: 0,
            cpu_time: 0,
            slice: 10,
        }
    }

    /// Return the process ID (query).
    pub fn pid(&self) -> u64 {
        self.pid
    }

    /// Return the process priority (query).
    pub fn priority(&self) -> Priority {
        self.priority
    }

    /// Return the scheduler policy (query).
    pub fn policy(&self) -> SchedulerPolicy {
        self.policy
    }

    /// Return the process state (query).
    pub fn state(&self) -> ProcessState {
        self.state
    }

    /// Return the virtual runtime (query).
    pub fn vruntime(&self) -> u64 {
        self.vruntime
    }

    /// Return the execution start time (query).
    pub fn exec_start(&self) -> u64 {
        self.exec_start
    }

    /// Return the total execution duration (query).
    pub fn exec_duration(&self) -> u64 {
        self.exec_duration
    }

    /// Return the CPU time used (query).
    pub fn cpu_time(&self) -> u64 {
        self.cpu_time
    }

    /// Return the time slice (query).
    pub fn slice(&self) -> u64 {
        self.slice
    }

    /// Update the time slice (command).
    ///
    /// Follows command-query separation: this function modifies state
    /// but does not return any meaningful value beyond success/failure.
    pub fn set_slice(&mut self, new_slice: u64) {
        self.slice = new_slice;
    }
}

impl Ord for ProcessTask {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.vruntime.cmp(&other.vruntime).reverse()
    }
}

impl PartialOrd for ProcessTask {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// CFS scheduler
pub struct CfsScheduler {
    runnable_tasks: BinaryHeap<ProcessTask>,
    current_task: Option<ProcessTask>,
    sleeping_tasks: HashMap<u64, ProcessTask>,
    min_granularity: u64,
    latency: u64,
    next_pid: AtomicU64,
}

impl CfsScheduler {
    pub fn new(min_granularity: u64, latency: u64) -> Self {
        Self {
            runnable_tasks: BinaryHeap::new(),
            current_task: None,
            sleeping_tasks: HashMap::new(),
            min_granularity,
            latency,
            next_pid: AtomicU64::new(1),
        }
    }

    /// Create a new process and add it to the scheduler (command).
    ///
    /// Follows command-query separation: this function modifies scheduler state
    /// (adds a task) but returns the created task as a read-only view.
    pub fn create_process(&mut self, priority: Priority, policy: SchedulerPolicy) -> ProcessTask {
        let pid = self.next_pid.fetch_add(1, Ordering::SeqCst);

        let task = ProcessTask {
            pid,
            priority,
            policy,
            state: ProcessState::Runnable,
            vruntime: 0,
            exec_start: 0,
            exec_duration: 0,
            cpu_time: 0,
            slice: self.calculate_slice(priority),
        };

        if policy == SchedulerPolicy::Normal {
            self.runnable_tasks.push(task.clone());
        }

        task
    }

    /// Calculate the time slice for a given priority (query).
    ///
    /// Pure function: output depends only on input, no mutable state.
    /// Follows command-query separation.
    fn calculate_slice(&self, priority: Priority) -> u64 {
        // Nice values range from -20 (largest slice) to 19 (smallest slice).
        // Keep the arithmetic signed until after clamping; casting a negative
        // factor to u64 used to overflow for normal and low priorities.
        let weight = (20 - priority.value().clamp(-20, 19)) as u64;
        (self.latency / 10)
            .saturating_mul(weight)
            .max(self.min_granularity)
    }

    /// Return the next runnable task and set it as current (command).
    ///
    /// Returns the task that was removed from the run queue and is now
    /// the running task. State is modified: the task's state changes to Running.
    pub fn pick_next_task(&mut self) -> Option<ProcessTask> {
        if let Some(mut current) = self.current_task.take() {
            if current.state == ProcessState::Running {
                current.state = ProcessState::Runnable;
                self.runnable_tasks.push(current);
            }
        }

        let mut next = self.runnable_tasks.pop()?;
        next.state = ProcessState::Running;
        self.current_task = Some(next.clone());
        Some(next)
    }

    /// Put the current running task to sleep (command).
    ///
    /// Returns `Ok(())` if the task was found and put to sleep, or `Err`
    /// if the task was not found in the scheduler.
    pub fn put_task_to_sleep(&mut self, pid: u64) -> Result<(), &'static str> {
        if let Some(ref current) = self.current_task {
            if current.pid == pid {
                if let Some(mut task) = self.current_task.take() {
                    task.state = ProcessState::Sleeping;
                    // Put sleeping task into the sleeping map
                    self.sleeping_tasks.insert(pid, task);
                    return Ok(());
                }
            }
        }
        Err("Task not found")
    }

    /// Wake up a sleeping task (command).
    ///
    /// Moves the task from the sleeping map back to the run queue.
    pub fn wake_up_task(&mut self, pid: u64) -> Result<(), &'static str> {
        if let Some(mut task) = self.sleeping_tasks.remove(&pid) {
            task.state = ProcessState::Runnable;
            self.runnable_tasks.push(task);
            Ok(())
        } else {
            Err("Task not found in sleeping list")
        }
    }

    /// Update the virtual runtime of the current task (command).
    ///
    /// Adjusts vruntime, cpu_time, and exec_duration by the given delta.
    pub fn update_vruntime(&mut self, pid: u64, delta: u64) {
        if let Some(ref mut current) = self.current_task {
            if current.pid == pid {
                current.vruntime = current.vruntime.saturating_add(delta);
                current.cpu_time = current.cpu_time.saturating_add(delta);
                current.exec_duration = current.exec_duration.saturating_add(delta);
            }
        }
    }

    /// Return the number of runnable tasks (query).
    ///
    /// Follows command-query separation: this function only reads state,
    /// it does not modify any scheduler state.
    pub fn runnable_count(&self) -> usize {
        self.runnable_tasks.len()
    }

    /// Return the current running task reference (query).
    ///
    /// Follows command-query separation: this function only reads state,
    /// it does not modify any scheduler state.
    pub fn current_task(&self) -> Option<&ProcessTask> {
        self.current_task.as_ref()
    }
}

/// RT scheduler
pub struct RtScheduler {
    runnable_tasks: Vec<ProcessTask>,
    current_task: Option<ProcessTask>,
    next_pid: AtomicU64,
}

impl RtScheduler {
    pub fn new() -> Self {
        Self {
            runnable_tasks: Vec::new(),
            current_task: None,
            next_pid: AtomicU64::new(1),
        }
    }

    /// Create a real-time process and add it to the scheduler (command).
    ///
    /// Follows command-query separation: this function modifies the scheduler
    /// state by adding a task, then returns the created task.
    pub fn create_process(&mut self, priority: Priority) -> ProcessTask {
        let pid = self.next_pid.fetch_add(1, Ordering::SeqCst);

        let task = ProcessTask {
            pid,
            priority,
            policy: SchedulerPolicy::Realtime,
            state: ProcessState::Runnable,
            vruntime: 0,
            exec_start: 0,
            exec_duration: 0,
            cpu_time: 0,
            slice: 10000,
        };

        self.runnable_tasks.push(task.clone());
        task
    }

    /// Return the next ready real-time task and make it current (command).
    ///
    /// Sorts tasks by priority (highest first), switches the current running
    /// task to runnable state, and returns the highest-priority task.
    pub fn pick_next_task(&mut self) -> Option<ProcessTask> {
        self.runnable_tasks
            .sort_by(|a, b| a.priority().value().cmp(&b.priority().value()).reverse());

        if let Some(current) = self.current_task.take() {
            if current.state == ProcessState::Running {
                let mut updated = current;
                updated.state = ProcessState::Runnable;
                self.runnable_tasks.push(updated);
            }
        }

        self.runnable_tasks.pop()
    }

    /// Return the number of runnable tasks (query).
    ///
    /// Follows command-query separation: this function only reads state,
    /// it does not modify any scheduler state.
    pub fn runnable_count(&self) -> usize {
        self.runnable_tasks.len()
    }

    /// Return the current running task reference (query).
    ///
    /// Follows command-query separation: this function only reads state,
    /// it does not modify any scheduler state.
    pub fn current_task(&self) -> Option<&ProcessTask> {
        self.current_task.as_ref()
    }
}

impl Default for RtScheduler {
    fn default() -> Self {
        Self::new()
    }
}

/// Energy-aware scheduler
pub struct EnergyAwareScheduler {
    cfs: CfsScheduler,
    cpu_frequency: u32, // MHz
    thermal_state: ThermalState,
    energy_budget: u64, // mJ
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalState {
    Normal,
    Throttling,
    Critical,
}

impl EnergyAwareScheduler {
    pub fn new() -> Self {
        Self {
            cfs: CfsScheduler::new(1000000, 20000000),
            cpu_frequency: 2400,
            thermal_state: ThermalState::Normal,
            energy_budget: 10000,
        }
    }

    pub fn create_process(&mut self, priority: Priority, policy: SchedulerPolicy) -> ProcessTask {
        self.cfs.create_process(priority, policy)
    }

    pub fn pick_next_task(&mut self) -> Option<ProcessTask> {
        // Stop dispatching under critical heat without discarding queued work.
        if self.thermal_state == ThermalState::Critical {
            None
        } else {
            self.cfs.pick_next_task()
        }
    }

    pub fn update_thermal_state(&mut self, temperature: u32) {
        self.thermal_state = if temperature > 90 {
            ThermalState::Critical
        } else if temperature > 80 {
            ThermalState::Throttling
        } else {
            ThermalState::Normal
        };
        self.cpu_frequency = match self.thermal_state {
            ThermalState::Normal => 2400,
            ThermalState::Throttling => 1200,
            ThermalState::Critical => 800,
        };
    }

    pub fn cpu_frequency(&self) -> u32 {
        self.cpu_frequency
    }

    pub fn thermal_state(&self) -> ThermalState {
        self.thermal_state
    }

    pub fn runnable_count(&self) -> usize {
        self.cfs.runnable_count()
    }
}

impl Default for EnergyAwareScheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        let high = Priority::new(-20);
        let low = Priority::new(19);

        assert!(high < low);
    }

    #[test]
    fn test_cfs_create_process() {
        let mut scheduler = CfsScheduler::new(1000000, 20000000);

        let task = scheduler.create_process(Priority::new(0), SchedulerPolicy::Normal);
        assert_eq!(task.policy, SchedulerPolicy::Normal);
        assert_eq!(scheduler.runnable_count(), 1);
    }

    #[test]
    fn test_cfs_pick_next_task() {
        let mut scheduler = CfsScheduler::new(1000000, 20000000);

        scheduler.create_process(Priority::new(0), SchedulerPolicy::Normal);
        let task = scheduler.pick_next_task();

        assert!(task.is_some());
        assert_eq!(scheduler.runnable_count(), 0);
    }

    #[test]
    fn test_cfs_priority_slices_and_runtime_order() {
        let mut scheduler = CfsScheduler::new(1_000_000, 20_000_000);
        let high = scheduler.create_process(Priority::high(), SchedulerPolicy::Normal);
        let normal = scheduler.create_process(Priority::normal(), SchedulerPolicy::Normal);
        let low = scheduler.create_process(Priority::low(), SchedulerPolicy::Normal);
        assert!(high.slice > normal.slice && normal.slice > low.slice);
        assert!(low.slice >= 1_000_000);

        let first = scheduler.pick_next_task().unwrap();
        scheduler.update_vruntime(first.pid(), 10);
        assert_eq!(scheduler.current_task().unwrap().vruntime(), 10);
        let next = scheduler.pick_next_task().unwrap();
        assert_ne!(next.pid(), first.pid());
        assert_eq!(scheduler.runnable_count(), 2);
    }

    #[test]
    fn test_rt_scheduler() {
        let mut scheduler = RtScheduler::new();

        let task = scheduler.create_process(Priority::new(-10));
        assert_eq!(task.policy, SchedulerPolicy::Realtime);
        assert_eq!(scheduler.runnable_count(), 1);
    }

    #[test]
    fn test_energy_aware_scheduler() {
        let mut scheduler = EnergyAwareScheduler::new();

        scheduler.update_thermal_state(85);
        assert_eq!(scheduler.thermal_state(), ThermalState::Throttling);
        assert_eq!(scheduler.cpu_frequency(), 1200);
    }

    #[test]
    fn test_thermal_critical() {
        let mut scheduler = EnergyAwareScheduler::new();
        let task = scheduler.create_process(Priority::normal(), SchedulerPolicy::Normal);

        scheduler.update_thermal_state(95);
        assert_eq!(scheduler.thermal_state(), ThermalState::Critical);
        assert_eq!(scheduler.cpu_frequency(), 800);
        assert!(scheduler.pick_next_task().is_none());
        assert_eq!(scheduler.runnable_count(), 1);

        scheduler.update_thermal_state(70);
        assert_eq!(scheduler.cpu_frequency(), 2400);
        assert_eq!(scheduler.pick_next_task().unwrap().pid(), task.pid());
    }
}
