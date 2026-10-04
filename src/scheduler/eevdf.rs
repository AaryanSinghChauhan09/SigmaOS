#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

// EEVDF Scheduler - Earliest Eligible Virtual Deadline First
// Linux 6.6+ inspired implementation for SigmaOS

/// Nice value to scheduling weight mapping
/// Lower nice = higher weight = more CPU time
/// Linux CFS nice-to-weight table (kernel/sched/core.c)
pub static NICE_TO_WEIGHT: [u32; 40] = [
    /* -20 */ 88761, 71755, 56483, 46273, 36291, /* -15 */ 29154, 23254, 18705, 14949,
    11916, /* -10 */ 9548, 7620, 6100, 4904, 3906, /*  -5 */ 3121, 2501, 1991, 1586,
    1277, /*   0 */ 1024, 820, 655, 526, 423, /*   5 */ 335, 272, 215, 172, 137,
    /*  10 */ 110, 87, 70, 56, 45, /*  15 */ 36, 29, 23, 18, 15,
];

/// Convert nice value (-20 to +19) to weight
pub fn nice_to_weight(nice: i8) -> u32 {
    let idx = (nice.clamp(-20, 19) + 20) as usize;
    NICE_TO_WEIGHT[idx]
}

/// Compute unit types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComputeUnit {
    CpuCore(u32),
    GpuStream(u32),
    NpuAccelerator(u32),
}

/// Task state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
    Completed,
}

/// EEVDF Task with Linux 6.6+ scheduling parameters
#[derive(Debug, Clone)]
pub struct EevdfTask {
    pub pid: u64,
    pub vruntime: u64,    // Virtual runtime (nanoseconds)
    pub deadline: u64,    // Virtual deadline
    pub weight: u32,      // Nice-to-weight mapped value
    pub slice: u64,       // Time slice per period (ns)
    pub lag: i64,         // Service debt (positive = owed CPU time)
    pub eligible_at: u64, // Earliest eligible vruntime
    pub latency_nice: i8, // Latency sensitivity (-20 to +19)
    pub state: TaskState,
    pub compute_unit: Option<ComputeUnit>,
    pub cpu_affinity: u64,             // CPU affinity bitmask
    pub numa_node: u32,                // NUMA node preference
    pub boosted_priority: Option<u32>, // Priority inheritance boost
}

impl EevdfTask {
    pub fn new(pid: u64, deadline: u64, priority: u8) -> Self {
        let nice = priority as i8;
        let weight = nice_to_weight(nice);
        Self {
            pid,
            vruntime: 0,
            deadline,
            weight,
            slice: 6_000_000, // 6ms default
            lag: 0,
            eligible_at: 0,
            latency_nice: 0,
            state: TaskState::Ready,
            compute_unit: None,
            cpu_affinity: u64::MAX, // All CPUs by default
            numa_node: 0,
            boosted_priority: None,
        }
    }

    /// Check if task is eligible to run
    pub fn is_eligible(&self, min_vruntime: u64) -> bool {
        // Keep the signed lag signed: casting a negative lag to u64 wraps and
        // can make a task with service debt appear eligible.
        self.state == TaskState::Ready
            && (self.vruntime as i128 - self.lag as i128) <= min_vruntime as i128
    }

    /// Boost priority for priority inheritance
    pub fn boost_priority(&mut self, new_priority: u32) {
        if self.boosted_priority.is_none() {
            self.boosted_priority = Some(self.weight);
        }
        self.weight = new_priority;
    }

    /// Restore original priority after boost
    pub fn restore_priority(&mut self) {
        if let Some(orig_weight) = self.boosted_priority {
            self.weight = orig_weight;
            self.boosted_priority = None;
        }
    }

    /// Assign compute unit
    pub fn assign_compute_unit(&mut self, unit: ComputeUnit) {
        self.compute_unit = Some(unit);
    }
}

/// EEVDF Runqueue
pub struct EevdfRunqueue {
    /// Tasks sorted by virtual deadline (eligible tasks)
    tasks: Vec<EevdfTask>,
    /// Global minimum vruntime (monotonic)
    min_vruntime: u64,
}

impl EevdfRunqueue {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            min_vruntime: 0,
        }
    }

    /// Add task to runqueue
    pub fn enqueue(&mut self, task: EevdfTask) {
        self.tasks.push(task);
        self.sort_by_deadline();
    }

    /// Remove task from runqueue
    pub fn dequeue(&mut self, pid: u64) -> Option<EevdfTask> {
        if let Some(idx) = self.tasks.iter().position(|t| t.pid == pid) {
            Some(self.tasks.remove(idx))
        } else {
            None
        }
    }

    /// Sort tasks by virtual deadline
    fn sort_by_deadline(&mut self) {
        self.tasks.sort_by_key(|t| t.deadline);
    }

    /// Pick earliest eligible virtual deadline
    pub fn pick_eevdf(&mut self, min_vruntime: u64) -> Option<EevdfTask> {
        // Among eligible tasks, pick the one with earliest virtual deadline
        let eligible_idx = self
            .tasks
            .iter()
            .position(|t| t.is_eligible(min_vruntime))?;
        Some(self.tasks.remove(eligible_idx))
    }

    /// Update minimum vruntime (monotonically increasing)
    pub fn update_min_vruntime(&mut self) {
        if let Some(leftmost) = self.tasks.first() {
            self.min_vruntime = self.min_vruntime.max(leftmost.vruntime);
        }
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn min_vruntime(&self) -> u64 {
        self.min_vruntime
    }
}

/// Calculate delta fair: scale real time by task weight
/// delta_fair = delta_ns * NICE_0_WEIGHT / weight
pub fn calc_delta_fair(delta_ns: u64, weight: u32, _inv_weight: u32) -> u64 {
    const NICE_0_LOAD: u32 = 1024;
    if weight == NICE_0_LOAD {
        return delta_ns;
    }
    (delta_ns * NICE_0_LOAD as u64) / weight as u64
}

/// Update task virtual runtime
pub fn update_curr(task: &mut EevdfTask, delta_ns: u64) {
    let delta_vruntime = calc_delta_fair(delta_ns, task.weight, 0);
    task.vruntime += delta_vruntime;
    task.lag -= delta_ns as i64;
}

/// Place entity on wakeup
pub fn place_entity(task: &mut EevdfTask, min_vruntime: u64, initial: bool) {
    if initial {
        task.vruntime = min_vruntime;
    } else {
        let lag_adjusted = (task.vruntime as i64 + task.lag).max(0) as u64;
        task.vruntime = lag_adjusted.max(min_vruntime);
    }
}

/// EEVDF Scheduler
pub struct EevdfScheduler {
    ready_queue: EevdfRunqueue,
    running_tasks: BTreeMap<u64, EevdfTask>,
    current_time: u64,
    compute_units: Vec<ComputeUnit>,
    sched_latency_ns: u64,   // Target scheduling latency (6ms)
    min_granularity_ns: u64, // Minimum time slice (0.75ms)
}

impl EevdfScheduler {
    pub fn new() -> Self {
        Self {
            ready_queue: EevdfRunqueue::new(),
            running_tasks: BTreeMap::new(),
            current_time: 0,
            compute_units: Vec::new(),
            sched_latency_ns: 6_000_000, // 6ms
            min_granularity_ns: 750_000, // 0.75ms
        }
    }

    /// Add compute unit
    pub fn add_compute_unit(&mut self, unit: ComputeUnit) {
        self.compute_units.push(unit);
    }

    /// Add task to scheduler
    pub fn add_task(&mut self, mut task: EevdfTask) {
        place_entity(&mut task, self.ready_queue.min_vruntime(), true);
        self.ready_queue.enqueue(task);
    }

    /// Calculate time slice for task
    pub fn calculate_slice(&self, task: &EevdfTask) -> u64 {
        let nr_running = (self.ready_queue.len() + self.running_tasks.len()).max(1);
        let period = self
            .sched_latency_ns
            .max(self.min_granularity_ns * nr_running as u64);
        (period * task.weight as u64) / (nr_running as u64 * 1024)
    }

    /// Schedule next task using EEVDF algorithm
    pub fn schedule(&mut self) -> Option<u64> {
        self.ready_queue.update_min_vruntime();
        let min_vruntime = self.ready_queue.min_vruntime();

        let mut task = self.ready_queue.pick_eevdf(min_vruntime)?;
        task.state = TaskState::Running;

        // Assign available compute unit
        if let Some(unit) = self.get_available_unit() {
            task.assign_compute_unit(unit);
        }

        let pid = task.pid;
        self.running_tasks.insert(pid, task);
        Some(pid)
    }

    /// Complete a task
    pub fn complete_task(&mut self, pid: u64) -> Result<(), &'static str> {
        let mut task = self.running_tasks.remove(&pid).ok_or("Task not found")?;
        task.state = TaskState::Completed;
        Ok(())
    }

    /// Get available compute unit
    fn get_available_unit(&self) -> Option<ComputeUnit> {
        let used_units: Vec<ComputeUnit> = self
            .running_tasks
            .values()
            .filter_map(|t| t.compute_unit)
            .collect();

        for unit in &self.compute_units {
            if !used_units.contains(unit) {
                return Some(*unit);
            }
        }
        None
    }

    /// Advance time and update running tasks
    pub fn advance_time(&mut self, delta: u64) {
        self.current_time += delta;
        for task in self.running_tasks.values_mut() {
            update_curr(task, delta);
        }
    }

    /// Update minimum vruntime
    pub fn update_min_vruntime(&mut self) {
        self.ready_queue.update_min_vruntime();
    }

    pub fn current_time(&self) -> u64 {
        self.current_time
    }

    pub fn ready_count(&self) -> usize {
        self.ready_queue.len()
    }

    pub fn running_count(&self) -> usize {
        self.running_tasks.len()
    }

    pub fn compute_unit_count(&self) -> usize {
        self.compute_units.len()
    }
}

impl Default for EevdfScheduler {
    fn default() -> Self {
        Self::new()
    }
}

// Legacy compatibility types
pub type Task = EevdfTask;

/// S-INIT Service Supervisor (legacy compatibility)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Crashed,
}

pub struct Service {
    pub name: String,
    pub state: ServiceState,
    pub pid: Option<u32>,
    pub restart_count: u32,
}

impl Service {
    pub fn new(name: String) -> Self {
        Self {
            name,
            state: ServiceState::Stopped,
            pid: None,
            restart_count: 0,
        }
    }

    pub fn start(&mut self) -> Result<(), &'static str> {
        if self.state == ServiceState::Running {
            return Err("Service already running");
        }
        self.state = ServiceState::Starting;
        self.pid = Some(1000 + self.restart_count);
        self.state = ServiceState::Running;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), &'static str> {
        if self.state != ServiceState::Running {
            return Err("Service not running");
        }
        self.state = ServiceState::Stopping;
        self.pid = None;
        self.state = ServiceState::Stopped;
        Ok(())
    }

    pub fn crash(&mut self) {
        self.state = ServiceState::Crashed;
        self.pid = None;
    }

    pub fn restart(&mut self) -> Result<(), &'static str> {
        self.crash();
        self.restart_count += 1;
        self.start()
    }
}

pub struct SInitSupervisor {
    services: BTreeMap<String, Service>,
    supervision_tree: Vec<Vec<String>>,
}

impl SInitSupervisor {
    pub fn new() -> Self {
        Self {
            services: BTreeMap::new(),
            supervision_tree: Vec::new(),
        }
    }

    pub fn add_service(&mut self, service: Service) {
        let name = service.name.clone();
        self.services.insert(name, service);
    }

    pub fn start_service(&mut self, name: &str) -> Result<(), &'static str> {
        let service = self.services.get_mut(name).ok_or("Service not found")?;
        service.start()
    }

    pub fn stop_service(&mut self, name: &str) -> Result<(), &'static str> {
        let service = self.services.get_mut(name).ok_or("Service not found")?;
        service.stop()
    }

    pub fn restart_service(&mut self, name: &str) -> Result<(), &'static str> {
        let service = self.services.get_mut(name).ok_or("Service not found")?;
        service.restart()
    }

    pub fn get_service_state(&self, name: &str) -> Option<ServiceState> {
        self.services.get(name).map(|s| s.state)
    }

    pub fn supervise(&mut self) -> Vec<String> {
        let mut restarted = Vec::new();
        for (name, service) in self.services.iter_mut() {
            if service.state == ServiceState::Crashed {
                if service.restart().is_ok() {
                    restarted.push(name.clone());
                }
            }
        }
        restarted
    }

    pub fn service_count(&self) -> usize {
        self.services.len()
    }

    pub fn list_services(&self) -> Vec<&Service> {
        self.services.values().collect()
    }
}

impl Default for SInitSupervisor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nice_to_weight_mapping() {
        assert_eq!(nice_to_weight(0), 1024); // Nice 0 = 1024 weight
        assert!(nice_to_weight(-10) > nice_to_weight(0)); // Lower nice = higher weight
        assert!(nice_to_weight(0) > nice_to_weight(10)); // Higher nice = lower weight
    }

    #[test]
    fn test_eevdf_task_eligibility() {
        let mut task = EevdfTask::new(1, 100, 0);
        task.vruntime = 50;
        task.lag = 10;

        // Task is eligible if vruntime - lag <= min_vruntime
        assert!(task.is_eligible(60)); // 50 - 10 = 40 <= 60
        assert!(!task.is_eligible(30)); // 50 - 10 = 40 > 30
    }

    #[test]
    fn negative_lag_requires_a_later_virtual_time_to_be_eligible() {
        let mut task = EevdfTask::new(1, 100, 0);
        task.vruntime = 50;
        task.lag = -10;

        assert!(!task.is_eligible(59)); // 50 - (-10) = 60
        assert!(task.is_eligible(60));
    }

    #[test]
    fn eligibility_rejects_non_ready_tasks_even_with_positive_lag() {
        let mut task = EevdfTask::new(1, 100, 0);
        task.lag = 10;
        task.state = TaskState::Blocked;

        assert!(!task.is_eligible(100));
    }

    #[test]
    fn test_priority_inheritance() {
        let mut task = EevdfTask::new(1, 100, 5);
        let orig_weight = task.weight;

        task.boost_priority(2048);
        assert_eq!(task.weight, 2048);
        assert_eq!(task.boosted_priority, Some(orig_weight));

        task.restore_priority();
        assert_eq!(task.weight, orig_weight);
        assert_eq!(task.boosted_priority, None);
    }

    #[test]
    fn test_calc_delta_fair() {
        let delta = calc_delta_fair(1000, 1024, 0);
        assert_eq!(delta, 1000); // Nice 0: no scaling

        let delta_high = calc_delta_fair(1000, 2048, 0);
        assert!(delta_high < 1000); // Higher weight = slower vruntime growth
    }

    #[test]
    fn test_eevdf_scheduler() {
        let mut scheduler = EevdfScheduler::new();
        scheduler.add_compute_unit(ComputeUnit::CpuCore(0));

        let task1 = EevdfTask::new(1, 100, 1);
        let task2 = EevdfTask::new(2, 50, 1); // Earlier deadline

        scheduler.add_task(task1);
        scheduler.add_task(task2);

        assert_eq!(scheduler.ready_count(), 2);

        let scheduled = scheduler.schedule();
        assert_eq!(scheduled, Some(2)); // Task with earliest deadline
    }

    #[test]
    fn test_weight_based_time_slice() {
        let scheduler = EevdfScheduler::new();

        let task_high = EevdfTask::new(1, 100, 0); // Nice 0
        let task_low = EevdfTask::new(2, 100, 10); // Nice 10

        let slice_high = scheduler.calculate_slice(&task_high);
        let slice_low = scheduler.calculate_slice(&task_low);

        assert!(slice_high >= slice_low); // Higher weight = more CPU time
    }

    #[test]
    fn test_min_vruntime_monotonic() {
        let mut scheduler = EevdfScheduler::new();

        let task1 = EevdfTask::new(1, 50, 1);
        let task2 = EevdfTask::new(2, 100, 1);

        scheduler.add_task(task1);
        scheduler.add_task(task2);

        let vruntime_before = scheduler.ready_queue.min_vruntime();
        scheduler.update_min_vruntime();
        let vruntime_after = scheduler.ready_queue.min_vruntime();

        assert!(vruntime_after >= vruntime_before); // Never decreases
    }

    #[test]
    fn test_eligible_task_ordering() {
        let mut scheduler = EevdfScheduler::new();
        scheduler.add_compute_unit(ComputeUnit::CpuCore(0));

        let mut task1 = EevdfTask::new(1, 200, 1);
        task1.vruntime = 50;
        task1.lag = -10; // Not owed CPU time

        let mut task2 = EevdfTask::new(2, 100, 1); // Earlier deadline
        task2.vruntime = 20;
        task2.lag = 10; // Owed CPU time

        scheduler.add_task(task1);
        scheduler.add_task(task2);

        let scheduled = scheduler.schedule();
        assert_eq!(scheduled, Some(2)); // Eligible with earlier deadline
    }

    #[test]
    fn test_latency_nice_field_initialization() {
        let task = EevdfTask::new(1, 100, 5);
        assert_eq!(task.latency_nice, 0); // Default latency_nice
        assert_eq!(task.weight, nice_to_weight(5));
    }

    // Legacy compatibility tests
    #[test]
    fn test_service_start_stop() {
        let mut service = Service::new("test".to_string());

        service.start().unwrap();
        assert_eq!(service.state, ServiceState::Running);

        service.stop().unwrap();
        assert_eq!(service.state, ServiceState::Stopped);
    }

    #[test]
    fn test_service_restart() {
        let mut service = Service::new("test".to_string());

        service.start().unwrap();
        service.crash();
        assert_eq!(service.state, ServiceState::Crashed);

        service.restart().unwrap();
        assert_eq!(service.state, ServiceState::Running);
        assert_eq!(service.restart_count, 1);
    }

    #[test]
    fn test_sinit_supervisor() {
        let mut supervisor = SInitSupervisor::new();

        let service = Service::new("web".to_string());
        supervisor.add_service(service);

        supervisor.start_service("web").unwrap();
        assert_eq!(
            supervisor.get_service_state("web"),
            Some(ServiceState::Running)
        );
    }

    #[test]
    fn test_supervision_restart() {
        let mut supervisor = SInitSupervisor::new();

        let mut service = Service::new("database".to_string());
        service.start().unwrap();
        service.crash();
        supervisor.add_service(service);

        let restarted = supervisor.supervise();
        assert_eq!(restarted.len(), 1);
        assert_eq!(
            supervisor.get_service_state("database"),
            Some(ServiceState::Running)
        );
    }
}
