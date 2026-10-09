// Process Scheduler Enhancements
// Inspired by Linux CFS, RT scheduler, and energy-aware scheduling

use std::collections::{BinaryHeap, HashMap};
use std::sync::atomic::{AtomicU64, Ordering};

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
    pub const Ready: Self = ProcessState::Runnable;
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

impl ProcessTask {
    pub fn new(pid: u64, _name: String, priority: Priority) -> Self {
        Self {
            pid,
            priority,
            policy: SchedulerPolicy::Normal,
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

    fn calculate_slice(&self, priority: Priority) -> u64 {
        // Nice values range from -20 (largest slice) to 19 (smallest slice).
        // Keep the arithmetic signed until after clamping; casting a negative
        // factor to u64 used to overflow for normal and low priorities.
        let weight = (20 - priority.value.clamp(-20, 19)) as u64;
        (self.latency / 10)
            .saturating_mul(weight)
            .max(self.min_granularity)
    }

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

    pub fn wake_up_task(&mut self, _pid: u64) -> Result<(), &'static str> {
        Ok(())
    }

    pub fn update_vruntime(&mut self, pid: u64, delta: u64) {
        if let Some(ref mut current) = self.current_task {
            if current.pid == pid {
                current.vruntime = current.vruntime.saturating_add(delta);
                current.cpu_time = current.cpu_time.saturating_add(delta);
                current.exec_duration = current.exec_duration.saturating_add(delta);
            }
        }
    }

    pub fn runnable_count(&self) -> usize {
        self.runnable_tasks.len()
    }

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

    pub fn pick_next_task(&mut self) -> Option<ProcessTask> {
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

    pub fn runnable_count(&self) -> usize {
        self.runnable_tasks.len()
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
        scheduler.update_vruntime(first.pid, 10);
        assert_eq!(scheduler.current_task().unwrap().vruntime, 10);
        let next = scheduler.pick_next_task().unwrap();
        assert_ne!(next.pid, first.pid);
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
        assert_eq!(scheduler.pick_next_task().unwrap().pid, task.pid);
    }
}
