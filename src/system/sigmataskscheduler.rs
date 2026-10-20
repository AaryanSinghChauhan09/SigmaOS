// SPDX-License-Identifier: MIT
// SigmaOS - Task Scheduler
// Task scheduling inspired by Arch (systemd timers), Debian (cron), and CachyOS

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Task priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// Task status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Scheduled task
#[derive(Debug, Clone)]
pub struct ScheduledTask {
    pub id: String,
    pub name: String,
    pub command: String,
    pub priority: TaskPriority,
    pub status: TaskStatus,
    pub scheduled_time: u64,
    pub retry_count: u32,
}

/// Task scheduler
pub struct SigmaTaskScheduler {
    tasks: Vec<ScheduledTask>,
    current_time: u64,
}

impl SigmaTaskScheduler {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            current_time: 0,
        }
    }

    pub fn add_task(&mut self, task: ScheduledTask) {
        self.tasks.push(task);
    }

    pub fn get_tasks(&self) -> &[ScheduledTask] {
        &self.tasks
    }

    pub fn get_task(&self, id: &str) -> Option<&ScheduledTask> {
        self.tasks.iter().find(|t| t.id == id)
    }

    pub fn get_task_mut(&mut self, id: &str) -> Option<&mut ScheduledTask> {
        self.tasks.iter_mut().find(|t| t.id == id)
    }

    pub fn schedule_task(&mut self, id: String, name: String, command: String, priority: TaskPriority, delay_seconds: u64) {
        let task = ScheduledTask {
            id,
            name,
            command,
            priority,
            status: TaskStatus::Pending,
            scheduled_time: self.current_time + delay_seconds,
            retry_count: 0,
        };
        self.tasks.push(task);
    }

    pub fn tick(&mut self) {
        self.current_time += 1;
        for task in &mut self.tasks {
            if task.status == TaskStatus::Pending && task.scheduled_time <= self.current_time {
                task.status = TaskStatus::Running;
            }
        }
    }

    pub fn get_pending_tasks(&self) -> Vec<&ScheduledTask> {
        self.tasks
            .iter()
            .filter(|t| t.status == TaskStatus::Pending)
            .collect()
    }

    pub fn get_running_tasks(&self) -> Vec<&ScheduledTask> {
        self.tasks
            .iter()
            .filter(|t| t.status == TaskStatus::Running)
            .collect()
    }

    pub fn complete_task(&mut self, id: String) -> Result<(), String> {
        if let Some(task) = self.get_task_mut(&id) {
            task.status = TaskStatus::Completed;
            Ok(())
        } else {
            Err(format!("Task {} not found", id))
        }
    }

    pub fn fail_task(&mut self, id: String) -> Result<(), String> {
        if let Some(task) = self.get_task_mut(&id) {
            task.status = TaskStatus::Failed;
            task.retry_count += 1;
            Ok(())
        } else {
            Err(format!("Task {} not found", id))
        }
    }

    pub fn cancel_task(&mut self, id: String) -> Result<(), String> {
        if let Some(task) = self.get_task_mut(&id) {
            task.status = TaskStatus::Cancelled;
            Ok(())
        } else {
            Err(format!("Task {} not found", id))
        }
    }

    pub fn get_high_priority_tasks(&self) -> Vec<&ScheduledTask> {
        self.tasks
            .iter()
            .filter(|t| t.priority >= TaskPriority::High)
            .collect()
    }
}

impl Default for SigmaTaskScheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_scheduler_creation() {
        let scheduler = SigmaTaskScheduler::new();
        assert_eq!(scheduler.get_tasks().len(), 0);
        assert_eq!(scheduler.current_time, 0);
    }

    #[test]
    fn test_add_task() {
        let mut scheduler = SigmaTaskScheduler::new();
        let task = ScheduledTask {
            id: "1".to_string(),
            name: "test".to_string(),
            command: "echo test".to_string(),
            priority: TaskPriority::Normal,
            status: TaskStatus::Pending,
            scheduled_time: 10,
            retry_count: 0,
        };
        scheduler.add_task(task);
        assert_eq!(scheduler.get_tasks().len(), 1);
    }

    #[test]
    fn test_schedule_task() {
        let mut scheduler = SigmaTaskScheduler::new();
        scheduler.schedule_task("1".to_string(), "test".to_string(), "echo test".to_string(), TaskPriority::Normal, 5);
        assert_eq!(scheduler.get_tasks().len(), 1);
        assert_eq!(scheduler.get_tasks()[0].scheduled_time, 5);
    }

    #[test]
    fn test_tick() {
        let mut scheduler = SigmaTaskScheduler::new();
        scheduler.schedule_task("1".to_string(), "test".to_string(), "echo test".to_string(), TaskPriority::Normal, 1);
        scheduler.tick();
        assert_eq!(scheduler.current_time, 1);
        assert_eq!(scheduler.get_tasks()[0].status, TaskStatus::Running);
    }

    #[test]
    fn test_complete_task() {
        let mut scheduler = SigmaTaskScheduler::new();
        scheduler.schedule_task("1".to_string(), "test".to_string(), "echo test".to_string(), TaskPriority::Normal, 0);
        scheduler.tick();
        scheduler.complete_task("1".to_string()).unwrap();
        assert_eq!(scheduler.get_tasks()[0].status, TaskStatus::Completed);
    }

    #[test]
    fn test_fail_task() {
        let mut scheduler = SigmaTaskScheduler::new();
        scheduler.schedule_task("1".to_string(), "test".to_string(), "echo test".to_string(), TaskPriority::Normal, 0);
        scheduler.tick();
        scheduler.fail_task("1".to_string()).unwrap();
        assert_eq!(scheduler.get_tasks()[0].status, TaskStatus::Failed);
        assert_eq!(scheduler.get_tasks()[0].retry_count, 1);
    }

    #[test]
    fn test_high_priority_tasks() {
        let mut scheduler = SigmaTaskScheduler::new();
        scheduler.schedule_task("1".to_string(), "low".to_string(), "echo low".to_string(), TaskPriority::Low, 0);
        scheduler.schedule_task("2".to_string(), "high".to_string(), "echo high".to_string(), TaskPriority::High, 0);
        assert_eq!(scheduler.get_high_priority_tasks().len(), 1);
    }
}
