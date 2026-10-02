// Process Scheduler Enhancements
// Inspired by Linux CFS, RT scheduler, and energy-aware scheduling

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::Ordering as CmpOrdering;
use core::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};
use core::time::Duration;
use std::collections::BinaryHeap;

/// Process priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Priority {
    pub value: i32, // -20 to 19 (lower is higher priority)
}

impl Priority {
    pub const Realtime: Self = Self { value: -20 };
    pub const High: Self = Self { value: -10 };
    pub const Normal: Self = Self { value: 0 };
    pub const Low: Self = Self { value: 10 };
    pub const Idle: Self = Self { value: 19 };

    pub fn new(value: i32) -> Self {
        Self {
            value: value.max(-20).min(19),
        }
    }

    pub fn highest() -> Self {
        Self { value: -20 }
    }

    pub fn lowest() -> Self {
        Self { value: 19 }
    }

    // Common priority levels for compatibility
    pub fn realtime() -> Self {
        Self { value: -20 }
    }

    pub fn high() -> Self {
        Self { value: -10 }
    }

    pub fn normal() -> Self {
        Self { value: 0 }
    }

    pub fn low() -> Self {
        Self { value: 10 }
    }

    pub fn idle() -> Self {
        Self { value: 19 }
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
    // Compatibility aliases
    pub const Ready: Self = ProcessState::Runnable;
    pub const Blocked: Self = ProcessState::Sleeping;
}

/// Process task
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessTask {
    pub pid: u64,
    pub priority: Priority,
    pub policy: SchedulerPolicy,
    pub state: ProcessState,
    pub vruntime: u64,      // Virtual runtime for CFS
    pub exec_start: u64,    // Execution start time
    pub exec_duration: u64, // Total execution duration
    pub cpu_time: u64,      // CPU time used
    pub slice: u64,         // Time slice
}

impl Ord for ProcessTask {
    fn cmp(&self, other: &Self) -> CmpOrdering {
        // Lower vruntime has higher priority
        self.vruntime.cmp(&other.vruntime).reverse()
    }
}

/// CFS Scheduler implementation
pub struct CfsScheduler {
    runnable_tasks: BinaryHeap<ProcessTask>,
    current_task: Option<ProcessTask>,
    min_granularity: u64,
    latency: u64,
    next_pid: AtomicU64,
    pub task_count: usize,}

impl CfsScheduler {
    pub fn new(min_granularity: u64, latency: u64) -> Self {
        Self {
            runnable_tasks: BinaryHeap::new(),
            current_task: None,
            min_granularity,
            latency,
            next_pid: AtomicU64::new(1),
            task_count: 0,
        }
    }

    pub fn calculate_slice(&self, priority: Priority) -> u64 {
        let weight = (20 - priority.value).max(1) as u64;
        (self.latency * weight) / 20
    }

    /// Create a new process
    pub fn create_process(&mut self, priority: Priority, policy: SchedulerPolicy) -> ProcessTask {
        let pid = self.next_pid.fetch_add(1, AtomicOrdering::SeqCst);

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
            self.task_count += 1;
        }

        task
    }

    pub fn pick_next_task(&mut self) -> Option<ProcessTask> {
        if let Some(current) = self.current_task.take() {
            if current.state == ProcessState::Running {
                let mut updated = current;
                updated.state = ProcessState::Runnable;
                self.runnable_tasks.push(updated);
            }
        }

        if let Some(mut next) = self.runnable_tasks.pop() {
            next.state = ProcessState::Running;
            self.current_task = Some(next.clone());
            Some(next)
        } else {
            None
        }
    }

    /// Put task to sleep
    pub fn put_task_to_sleep(&mut self, pid: u64) -> Result<(), &'static str> {
        if let Some(ref current) = self.current_task {
            if current.pid == pid {
                if let Some(mut task) = self.current_task.take() {
                    task.state = ProcessState::Sleeping;
                    return Ok(());
                }
            }
        }
        Err("Task not found")
    }

    /// Wake up task
    pub fn wake_up_task(&mut self, pid: u64) -> Result<(), &'static str> {
        // In a real implementation, would find sleeping task and move to runnable
        Ok(())
    }

    /// Update task vruntime
    pub fn update_vruntime(&mut self, pid: u64, delta: u64) {
        if let Some(ref mut current) = self.current_task {
            if current.pid == pid {
                current.vruntime += delta;
                current.cpu_time += delta;
                current.exec_duration += delta;
            }
        }
    }

    /// Get runnable task count
    pub fn runnable_count(&self) -> usize {
        self.runnable_tasks.len()
    }

    /// Get current task
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

    /// Create a new RT process
    pub fn create_process(&mut self, priority: Priority) -> ProcessTask {
        let pid = self.next_pid.fetch_add(1, AtomicOrdering::SeqCst);

        let task = ProcessTask {
            pid,
            priority,
            policy: SchedulerPolicy::Realtime,
            state: ProcessState::Runnable,
            vruntime: 0,
            exec_start: 0,
            exec_duration: 0,
            cpu_time: 0,
            slice: 10000, // Fixed slice for RT
        };

        self.runnable_tasks.push(task.clone());
        task
    }

    /// Pick next RT task (highest priority first)
    pub fn pick_next_task(&mut self) -> Option<ProcessTask> {
        // Sort by priority (lower value = higher priority)
        self.runnable_tasks
            .sort_by(|a, b| a.priority.cmp(&b.priority));

        if let Some(current) = self.current_task.take() {
            if current.state == ProcessState::Running {
                let mut updated = current;
                updated.state = ProcessState::Runnable;
                self.runnable_tasks.push(updated);
            }
        }

        self.runnable_tasks.pop()
    }

    /// Get runnable task count
    pub fn runnable_count(&self) -> usize {
        self.runnable_tasks.len()
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
            cfs: CfsScheduler::new(1000000, 20000000), // 1ms min granularity, 20ms latency
            cpu_frequency: 2400,                       // 2.4 GHz default
            thermal_state: ThermalState::Normal,
            energy_budget: 10000, // 10 J default
        }
    }

    /// Create a process
    pub fn create_process(&mut self, priority: Priority, policy: SchedulerPolicy) -> ProcessTask {
        self.cfs.create_process(priority, policy)
    }

    /// Pick next task with energy awareness
    pub fn pick_next_task(&mut self) -> Option<ProcessTask> {
        match self.thermal_state {
            ThermalState::Normal => self.cfs.pick_next_task(),
            ThermalState::Throttling => {
                // Reduce frequency and pick lower priority tasks
                self.cpu_frequency = 1200;
                self.cfs.pick_next_task()
            }
            ThermalState::Critical => {
                // Only pick idle tasks
                self.cpu_frequency = 800;
                None // In real implementation, would pick only idle tasks
            }
        }
    }

    /// Update thermal state
    pub fn update_thermal_state(&mut self, temperature: u32) {
        self.thermal_state = if temperature > 90 {
            ThermalState::Critical
        } else if temperature > 80 {
            ThermalState::Throttling
        } else {
            ThermalState::Normal
        };
    }

    /// Get current CPU frequency
    pub fn cpu_frequency(&self) -> u32 {
        self.cpu_frequency
    }

    /// Get thermal state
    pub fn thermal_state(&self) -> ThermalState {
        self.thermal_state
    }

    /// Get runnable count
    pub fn runnable_count(&self) -> usize {
        self.cfs.runnable_count()
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

        scheduler.update_thermal_state(95);
        assert_eq!(scheduler.thermal_state(), ThermalState::Critical);
        assert_eq!(scheduler.cpu_frequency(), 800);
    }
}
