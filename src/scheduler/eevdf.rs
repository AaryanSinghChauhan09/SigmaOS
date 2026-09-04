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
use std::string::{String, ToString};

// EEVDF Scheduler - Earliest Eligible Virtual Deadline First
// Asymmetric Multi-Processing scheduler for SigmaOS

// (no_std only applicable at crate root - removed)

use std::collections::BTreeMap;
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComputeUnit {
    CpuCore(u32),
    GpuStream(u32),
    NpuAccelerator(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
    Completed,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: u64,
    pub state: TaskState,
    pub virtual_deadline: u64,
    pub execution_time: u64,
    pub expected_service_time: u64, // Linux EEVDF service expectation
    pub compute_unit: Option<ComputeUnit>,
    pub priority: u8,
}

impl Task {
    pub fn new(id: u64, virtual_deadline: u64, priority: u8) -> Self {
        Self {
            id,
            state: TaskState::Ready,
            virtual_deadline,
            execution_time: 0,
            expected_service_time: 0,
            compute_unit: None,
            priority,
        }
    }

    pub fn assign_compute_unit(&mut self, unit: ComputeUnit) {
        self.compute_unit = Some(unit);
    }

    /// Calculate task lag: positive if task received less service than expected
    pub fn calculate_lag(&self) -> i64 {
        self.expected_service_time as i64 - self.execution_time as i64
    }

    pub fn is_eligible(&self, current_time: u64) -> bool {
        self.state == TaskState::Ready
            && (self.virtual_deadline >= current_time || self.calculate_lag() >= 0)
    }
}

/// EEVDF Scheduler
pub struct EevdfScheduler {
    ready_queue: Vec<Task>,
    running_tasks: BTreeMap<u64, Task>,
    current_time: u64,
    compute_units: Vec<ComputeUnit>,
}

impl EevdfScheduler {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            ready_queue: Vec::new(),
            running_tasks: BTreeMap::new(),
            current_time: 0,
            compute_units: Vec::new(),
        }
    }

    /// Add a compute unit (CPU core, GPU stream, NPU)
    pub fn add_compute_unit(&mut self, unit: ComputeUnit) {
        self.compute_units.push(unit);
    }

    /// Add a task to the ready queue
    pub fn add_task(&mut self, task: Task) {
        self.ready_queue.push(task);
        self.ready_queue.sort_by_key(|t| t.virtual_deadline);
    }

    /// Schedule the next eligible task
    pub fn schedule(&mut self) -> Option<u64> {
        // Find earliest eligible task
        let eligible_idx = self
            .ready_queue
            .iter()
            .position(|t| t.is_eligible(self.current_time));

        if let Some(idx) = eligible_idx {
            let mut task = self.ready_queue.remove(idx);
            task.state = TaskState::Running;

            // Assign to available compute unit
            if let Some(unit) = self.get_available_unit() {
                task.assign_compute_unit(unit);
            }

            let task_id = task.id;
            self.running_tasks.insert(task_id, task);
            Some(task_id)
        } else {
            None
        }
    }

    /// Complete a task
    pub fn complete_task(&mut self, task_id: u64) -> Result<(), &'static str> {
        let mut task = self
            .running_tasks
            .remove(&task_id)
            .ok_or("Task not found")?;

        task.state = TaskState::Completed;

        // Release compute unit
        if let Some(unit) = task.compute_unit {
            self.release_unit(unit);
        }

        Ok(())
    }

    /// Get available compute unit
    fn get_available_unit(&self) -> Option<ComputeUnit> {
        // Check which units are not in use by running tasks
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

    /// Release a compute unit
    fn release_unit(&mut self, _unit: ComputeUnit) {
        // Unit is now available (tracked by get_available_unit)
    }

    /// Advance time
    pub fn advance_time(&mut self, delta: u64) {
        self.current_time += delta;
    }

    /// Get current time
    pub fn current_time(&self) -> u64 {
        self.current_time
    }

    /// Get ready queue count
    pub fn ready_count(&self) -> usize {
        self.ready_queue.len()
    }

    /// Get running task count
    pub fn running_count(&self) -> usize {
        self.running_tasks.len()
    }

    /// Get compute unit count
    pub fn compute_unit_count(&self) -> usize {
        self.compute_units.len()
    }
}

impl Default for EevdfScheduler {
    fn default() -> Self {
        Self::new()
    }
}

/// S-INIT Service Supervisor
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
        self.pid = Some(1000 + self.restart_count); // Simulated PID
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
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            services: BTreeMap::new(),
            supervision_tree: Vec::new(),
        }
    }

    /// Add a service
    pub fn add_service(&mut self, service: Service) {
        let name = service.name.clone();
        self.services.insert(name, service);
    }

    /// Start a service
    pub fn start_service(&mut self, name: &str) -> Result<(), &'static str> {
        let service = self.services.get_mut(name).ok_or("Service not found")?;
        service.start()
    }

    /// Stop a service
    pub fn stop_service(&mut self, name: &str) -> Result<(), &'static str> {
        let service = self.services.get_mut(name).ok_or("Service not found")?;
        service.stop()
    }

    /// Restart a crashed service
    pub fn restart_service(&mut self, name: &str) -> Result<(), &'static str> {
        let service = self.services.get_mut(name).ok_or("Service not found")?;
        service.restart()
    }

    /// Get service state
    pub fn get_service_state(&self, name: &str) -> Option<ServiceState> {
        self.services.get(name).map(|s| s.state)
    }

    /// Check for crashed services and restart them
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

    /// Get service count
    pub fn service_count(&self) -> usize {
        self.services.len()
    }

    /// List all services
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
mod tests {
    use super::*;

    #[test]
    fn test_eevdf_scheduler() {
        let mut scheduler = EevdfScheduler::new();
        scheduler.add_compute_unit(ComputeUnit::CpuCore(0));
        scheduler.add_compute_unit(ComputeUnit::CpuCore(1));

        let task1 = Task::new(1, 100, 1);
        let task2 = Task::new(2, 50, 2);
        let task3 = Task::new(3, 150, 1);

        scheduler.add_task(task1);
        scheduler.add_task(task2);
        scheduler.add_task(task3);

        assert_eq!(scheduler.ready_count(), 3);

        let scheduled = scheduler.schedule();
        assert_eq!(scheduled, Some(2)); // Task with earliest deadline
    }

    #[test]
    fn test_task_eligibility() {
        let mut task = Task::new(1, 100, 1);
        assert!(task.is_eligible(50));

        task.execution_time = 120;
        task.expected_service_time = 100; // lag = -20
        assert!(!task.is_eligible(150));

        task.expected_service_time = 130; // lag = +10
        assert!(task.is_eligible(150));
    }

    #[test]
    fn test_compute_unit_assignment() {
        let mut scheduler = EevdfScheduler::new();
        scheduler.add_compute_unit(ComputeUnit::GpuStream(0));

        let mut task = Task::new(1, 100, 1);
        task.assign_compute_unit(ComputeUnit::GpuStream(0));

        assert!(task.compute_unit.is_some());
    }

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

    #[test]
    fn test_time_advancement() {
        let mut scheduler = EevdfScheduler::new();
        assert_eq!(scheduler.current_time(), 0);

        scheduler.advance_time(100);
        assert_eq!(scheduler.current_time(), 100);
    }
}
