// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Aaryan Singh Chauhan

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Task status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Task priority
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Scheduled task
#[derive(Debug, Clone)]
pub struct ScheduledTask {
    pub id: String,
    pub name: String,
    pub command: String,
    pub schedule: String,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub last_run: Option<String>,
    pub next_run: Option<String>,
}

/// Schedule manager for managing scheduled tasks
/// Inspired by Linux cron and systemd timers
#[derive(Debug, Clone)]
pub struct SigmaScheduleManager {
    tasks: Vec<ScheduledTask>,
}

impl SigmaScheduleManager {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
        }
    }

    /// Add a scheduled task
    pub fn add_task(&mut self, task: ScheduledTask) {
        self.tasks.push(task);
    }

    /// Get task by ID
    pub fn get_task(&self, id: &str) -> Option<&ScheduledTask> {
        self.tasks.iter().find(|t| t.id == id)
    }

    /// Get all tasks
    pub fn get_tasks(&self) -> &[ScheduledTask] {
        &self.tasks
    }

    /// Remove a task
    pub fn remove_task(&mut self, id: &str) -> Result<String, String> {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            let task = self.tasks.remove(pos);
            Ok(format!("Removed task: {}", task.name))
        } else {
            Err(format!("Task not found: {}", id))
        }
    }

    /// Run a task now
    pub fn run_task(&mut self, id: &str) -> Result<String, String> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.status = TaskStatus::Running;
            Ok(format!("Running task: {}", task.name))
        } else {
            Err(format!("Task not found: {}", id))
        }
    }

    /// Complete a task
    pub fn complete_task(&mut self, id: &str) -> Result<String, String> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.status = TaskStatus::Completed;
            Ok(format!("Completed task: {}", task.name))
        } else {
            Err(format!("Task not found: {}", id))
        }
    }

    /// Fail a task
    pub fn fail_task(&mut self, id: &str) -> Result<String, String> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.status = TaskStatus::Failed;
            Ok(format!("Task failed: {}", task.name))
        } else {
            Err(format!("Task not found: {}", id))
        }
    }

    /// Cancel a task
    pub fn cancel_task(&mut self, id: &str) -> Result<String, String> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.status = TaskStatus::Cancelled;
            Ok(format!("Cancelled task: {}", task.name))
        } else {
            Err(format!("Task not found: {}", id))
        }
    }

    /// Get pending tasks
    pub fn get_pending_tasks(&self) -> Vec<&ScheduledTask> {
        self.tasks
            .iter()
            .filter(|t| t.status == TaskStatus::Pending)
            .collect()
    }

    /// Get running tasks
    pub fn get_running_tasks(&self) -> Vec<&ScheduledTask> {
        self.tasks
            .iter()
            .filter(|t| t.status == TaskStatus::Running)
            .collect()
    }

    /// Get high priority tasks
    pub fn get_high_priority_tasks(&self) -> Vec<&ScheduledTask> {
        self.tasks
            .iter()
            .filter(|t| t.priority == TaskPriority::High || t.priority == TaskPriority::Critical)
            .collect()
    }

    /// Get task count
    pub fn get_task_count(&self) -> usize {
        self.tasks.len()
    }
}

impl Default for SigmaScheduleManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task() {
        let mut manager = SigmaScheduleManager::new();
        let task = ScheduledTask {
            id: "task-1".to_string(),
            name: "Backup".to_string(),
            command: "/usr/bin/backup".to_string(),
            schedule: "0 2 * * *".to_string(),
            status: TaskStatus::Pending,
            priority: TaskPriority::Normal,
            last_run: None,
            next_run: None,
        };
        manager.add_task(task);
        assert_eq!(manager.get_task_count(), 1);
    }

    #[test]
    fn test_run_task() {
        let mut manager = SigmaScheduleManager::new();
        let task = ScheduledTask {
            id: "task-1".to_string(),
            name: "Backup".to_string(),
            command: "/usr/bin/backup".to_string(),
            schedule: "0 2 * * *".to_string(),
            status: TaskStatus::Pending,
            priority: TaskPriority::Normal,
            last_run: None,
            next_run: None,
        };
        manager.add_task(task);
        manager.run_task("task-1").unwrap();
        assert_eq!(
            manager.get_task("task-1").unwrap().status,
            TaskStatus::Running
        );
    }

    #[test]
    fn test_complete_task() {
        let mut manager = SigmaScheduleManager::new();
        let task = ScheduledTask {
            id: "task-1".to_string(),
            name: "Backup".to_string(),
            command: "/usr/bin/backup".to_string(),
            schedule: "0 2 * * *".to_string(),
            status: TaskStatus::Running,
            priority: TaskPriority::Normal,
            last_run: None,
            next_run: None,
        };
        manager.add_task(task);
        manager.complete_task("task-1").unwrap();
        assert_eq!(
            manager.get_task("task-1").unwrap().status,
            TaskStatus::Completed
        );
    }

    #[test]
    fn test_remove_task() {
        let mut manager = SigmaScheduleManager::new();
        let task = ScheduledTask {
            id: "task-1".to_string(),
            name: "Backup".to_string(),
            command: "/usr/bin/backup".to_string(),
            schedule: "0 2 * * *".to_string(),
            status: TaskStatus::Pending,
            priority: TaskPriority::Normal,
            last_run: None,
            next_run: None,
        };
        manager.add_task(task);
        manager.remove_task("task-1").unwrap();
        assert_eq!(manager.get_task_count(), 0);
    }

    #[test]
    fn test_get_pending_tasks() {
        let mut manager = SigmaScheduleManager::new();
        let task1 = ScheduledTask {
            id: "task-1".to_string(),
            name: "Backup".to_string(),
            command: "/usr/bin/backup".to_string(),
            schedule: "0 2 * * *".to_string(),
            status: TaskStatus::Pending,
            priority: TaskPriority::Normal,
            last_run: None,
            next_run: None,
        };
        let task2 = ScheduledTask {
            id: "task-2".to_string(),
            name: "Update".to_string(),
            command: "/usr/bin/update".to_string(),
            schedule: "0 3 * * *".to_string(),
            status: TaskStatus::Completed,
            priority: TaskPriority::Normal,
            last_run: None,
            next_run: None,
        };
        manager.add_task(task1);
        manager.add_task(task2);
        assert_eq!(manager.get_pending_tasks().len(), 1);
    }

    #[test]
    fn test_get_high_priority_tasks() {
        let mut manager = SigmaScheduleManager::new();
        let task1 = ScheduledTask {
            id: "task-1".to_string(),
            name: "Backup".to_string(),
            command: "/usr/bin/backup".to_string(),
            schedule: "0 2 * * *".to_string(),
            status: TaskStatus::Pending,
            priority: TaskPriority::High,
            last_run: None,
            next_run: None,
        };
        let task2 = ScheduledTask {
            id: "task-2".to_string(),
            name: "Update".to_string(),
            command: "/usr/bin/update".to_string(),
            schedule: "0 3 * * *".to_string(),
            status: TaskStatus::Pending,
            priority: TaskPriority::Normal,
            last_run: None,
            next_run: None,
        };
        manager.add_task(task1);
        manager.add_task(task2);
        assert_eq!(manager.get_high_priority_tasks().len(), 1);
    }
}
