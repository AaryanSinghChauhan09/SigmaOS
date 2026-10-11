// SPDX-License-Identifier: MIT
// SigmaOS - Task Scheduler Framework
// Inspired by cron, systemd timers, anacron
// Provides task scheduling capabilities

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Schedule frequency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleFrequency {
    Once,
    Minutely,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Custom,
}

/// Task status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Disabled,
}

/// Scheduled task
#[derive(Debug, Clone)]
pub struct ScheduledTask {
    pub id: u32,
    pub name: String,
    pub command: String,
    pub frequency: ScheduleFrequency,
    pub status: TaskStatus,
    pub last_run: Option<u64>,
    pub next_run: u64,
    pub enabled: AtomicBool,
}

/// Task scheduler
pub struct TaskScheduler {
    pub tasks: BTreeMap<u32, ScheduledTask>,
    pub next_task_id: AtomicU32,
    pub active: AtomicBool,
}

impl TaskScheduler {
    pub fn new() -> Self {
        Self {
            tasks: BTreeMap::new(),
            next_task_id: AtomicU32::new(1),
            active: AtomicBool::new(false),
        }
    }
    
    /// Initialize task scheduler
    pub fn init(&mut self) {
        self.active.store(true, Ordering::Relaxed);
    }
    
    /// Add task
    pub fn add_task(&mut self, name: String, command: String, frequency: ScheduleFrequency, next_run: u64) -> u32 {
        let id = self.next_task_id.fetch_add(1, Ordering::Relaxed);
        let task = ScheduledTask {
            id,
            name,
            command,
            frequency,
            status: TaskStatus::Pending,
            last_run: None,
            next_run,
            enabled: AtomicBool::new(true),
        };
        self.tasks.insert(id, task);
        id
    }
    
    /// Remove task
    pub fn remove_task(&mut self, task_id: u32) -> Result<(), &'static str> {
        self.tasks.remove(&task_id).ok_or("Task not found")?;
        Ok(())
    }
    
    /// Enable task
    pub fn enable_task(&mut self, task_id: u32) -> Result<(), &'static str> {
        let task = self.tasks.get_mut(&task_id).ok_or("Task not found")?;
        task.enabled.store(true, Ordering::Relaxed);
        Ok(())
    }
    
    /// Disable task
    pub fn disable_task(&mut self, task_id: u32) -> Result<(), &'static str> {
        let task = self.tasks.get_mut(&task_id).ok_or("Task not found")?;
        task.enabled.store(false, Ordering::Relaxed);
        Ok(())
    }
    
    /// Update task status
    pub fn update_task_status(&mut self, task_id: u32, status: TaskStatus) -> Result<(), &'static str> {
        let task = self.tasks.get_mut(&task_id).ok_or("Task not found")?;
        task.status = status;
        Ok(())
    }
    
    /// Get task
    pub fn get_task(&self, id: u32) -> Option<&ScheduledTask> {
        self.tasks.get(&id)
    }
    
    /// Get pending tasks
    pub fn get_pending_tasks(&self, current_time: u64) -> Vec<&ScheduledTask> {
        self.tasks
            .values()
            .filter(|t| t.enabled.load(Ordering::Relaxed) && t.next_run <= current_time && t.status == TaskStatus::Pending)
            .collect()
    }
    
    /// Get all tasks
    pub fn get_all_tasks(&self) -> Vec<&ScheduledTask> {
        self.tasks.values().collect()
    }
    
    /// Is active
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }
}

impl Default for TaskScheduler {
    fn default() -> Self {
        Self::new()
    }
}

/// Global task scheduler
static mut TASK_SCHEDULER: Option<TaskScheduler> = None;

/// Initialize task scheduler
pub fn init_task_scheduler() {
    unsafe {
        TASK_SCHEDULER = Some(TaskScheduler::new());
    }
}

/// Get global task scheduler
pub fn get_task_scheduler() -> Option<&'static mut TaskScheduler> {
    unsafe {
        TASK_SCHEDULER.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_task_scheduler_init() {
        let mut scheduler = TaskScheduler::new();
        scheduler.init();
        assert!(scheduler.is_active());
    }
    
    #[test]
    fn test_add_task() {
        let mut scheduler = TaskScheduler::new();
        let id = scheduler.add_task("backup".to_string(), "backup.sh".to_string(), ScheduleFrequency::Daily, 1000);
        assert!(scheduler.get_task(id).is_some());
    }
    
    #[test]
    fn test_enable_disable_task() {
        let mut scheduler = TaskScheduler::new();
        let id = scheduler.add_task("test".to_string(), "test.sh".to_string(), ScheduleFrequency::Hourly, 1000);
        scheduler.disable_task(id).unwrap();
        assert!(!scheduler.get_task(id).unwrap().enabled.load(Ordering::Relaxed));
        scheduler.enable_task(id).unwrap();
        assert!(scheduler.get_task(id).unwrap().enabled.load(Ordering::Relaxed));
    }
    
    #[test]
    fn test_get_pending_tasks() {
        let mut scheduler = TaskScheduler::new();
        scheduler.add_task("pending".to_string(), "pending.sh".to_string(), ScheduleFrequency::Daily, 500);
        scheduler.add_task("future".to_string(), "future.sh".to_string(), ScheduleFrequency::Daily, 2000);
        
        let pending = scheduler.get_pending_tasks(1000);
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].name, "pending");
    }
}
