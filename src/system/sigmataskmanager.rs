//! SigmaTaskManager - Task Management System
//!
//! Provides comprehensive task management inspired by Linux process management.
//! Supports task creation, scheduling, and resource monitoring.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Task state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Running,
    Sleeping,
    Stopped,
    Zombie,
    Unknown,
}

/// Task priority
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskPriority {
    Realtime,
    High,
    Normal,
    Low,
    Idle,
}

/// Task
#[derive(Debug, Clone)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub state: TaskState,
    pub priority: TaskPriority,
    pub pid: u32,
    pub parent_pid: u32,
    pub cpu_usage: u32,
    pub memory_usage: u64,
}

impl Task {
    pub fn new(id: String, name: String, pid: u32, parent_pid: u32) -> Self {
        Self {
            id,
            name,
            state: TaskState::Running,
            priority: TaskPriority::Normal,
            pid,
            parent_pid,
            cpu_usage: 0,
            memory_usage: 0,
        }
    }

    pub fn with_priority(mut self, priority: TaskPriority) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_cpu_usage(mut self, cpu_usage: u32) -> Self {
        self.cpu_usage = cpu_usage;
        self
    }

    pub fn with_memory_usage(mut self, memory_usage: u64) -> Self {
        self.memory_usage = memory_usage;
        self
    }

    pub fn stop(&mut self) {
        self.state = TaskState::Stopped;
    }

    pub fn resume(&mut self) {
        self.state = TaskState::Running;
    }
}

/// Task manager
pub struct SigmaTaskManager {
    tasks: BTreeMap<String, Task>,
    next_task_id: u64,
}

impl SigmaTaskManager {
    pub fn new() -> Self {
        Self {
            tasks: BTreeMap::new(),
            next_task_id: 1,
        }
    }

    pub fn add_task(&mut self, name: String, pid: u32, parent_pid: u32) -> String {
        let id = format!("task-{}", self.next_task_id);
        self.next_task_id += 1;
        let task = Task::new(id.clone(), name, pid, parent_pid);
        self.tasks.insert(id.clone(), task);
        id
    }

    pub fn get_task(&self, id: &str) -> Option<&Task> {
        self.tasks.get(id)
    }

    pub fn get_task_mut(&mut self, id: &str) -> Option<&mut Task> {
        self.tasks.get_mut(id)
    }

    pub fn list_tasks(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    pub fn list_tasks_by_state(&self, state: TaskState) -> Vec<&Task> {
        self.tasks.values().filter(|t| t.state == state).collect()
    }

    pub fn list_tasks_by_priority(&self, priority: TaskPriority) -> Vec<&Task> {
        self.tasks
            .values()
            .filter(|t| t.priority == priority)
            .collect()
    }

    pub fn set_task_priority(&mut self, id: &str, priority: TaskPriority) -> bool {
        if let Some(task) = self.tasks.get_mut(id) {
            task.priority = priority;
            true
        } else {
            false
        }
    }

    pub fn remove_task(&mut self, id: &str) -> bool {
        self.tasks.remove(id).is_some()
    }

    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }

    pub fn total_cpu_usage(&self) -> u32 {
        self.tasks.values().map(|t| t.cpu_usage).sum()
    }

    pub fn total_memory_usage(&self) -> u64 {
        self.tasks.values().map(|t| t.memory_usage).sum()
    }
}

impl Default for SigmaTaskManager {
    fn default() -> Self {
        Self::new()
    }
}

static mut SIGMATASKMANAGER_INSTANCE: Option<SigmaTaskManager> = None;

pub fn init_sigmataskmanager() {
    unsafe {
        SIGMATASKMANAGER_INSTANCE = Some(SigmaTaskManager::new());
    }
}

pub fn get_sigmataskmanager() -> &'static SigmaTaskManager {
    unsafe {
        SIGMATASKMANAGER_INSTANCE
            .as_ref()
            .expect("SigmaTaskManager not initialized")
    }
}

pub fn get_sigmataskmanager_mut() -> &'static mut SigmaTaskManager {
    unsafe {
        SIGMATASKMANAGER_INSTANCE
            .as_mut()
            .expect("SigmaTaskManager not initialized")
    }
}

pub fn add_task(name: String, pid: u32, parent_pid: u32) -> String {
    let manager = get_sigmataskmanager_mut();
    manager.add_task(name, pid, parent_pid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() {
        let task = Task::new("task-1".to_string(), "bash".to_string(), 1000, 999);
        assert_eq!(task.id, "task-1");
        assert_eq!(task.name, "bash");
        assert_eq!(task.pid, 1000);
        assert_eq!(task.parent_pid, 999);
    }

    #[test]
    fn test_task_state() {
        let mut task = Task::new("task-1".to_string(), "bash".to_string(), 1000, 999);
        assert_eq!(task.state, TaskState::Running);

        task.stop();
        assert_eq!(task.state, TaskState::Stopped);

        task.resume();
        assert_eq!(task.state, TaskState::Running);
    }

    #[test]
    fn test_task_with_priority() {
        let task = Task::new("task-1".to_string(), "bash".to_string(), 1000, 999)
            .with_priority(TaskPriority::High)
            .with_cpu_usage(50)
            .with_memory_usage(1000000);
        assert_eq!(task.priority, TaskPriority::High);
        assert_eq!(task.cpu_usage, 50);
        assert_eq!(task.memory_usage, 1000000);
    }

    #[test]
    fn test_task_manager() {
        let manager = SigmaTaskManager::new();
        assert_eq!(manager.task_count(), 0);
    }

    #[test]
    fn test_add_task() {
        let mut manager = SigmaTaskManager::new();
        let id = manager.add_task("bash".to_string(), 1000, 999);
        assert_eq!(id, "task-1");
        assert_eq!(manager.task_count(), 1);
    }

    #[test]
    fn test_list_tasks_by_state() {
        let mut manager = SigmaTaskManager::new();
        let id1 = manager.add_task("bash".to_string(), 1000, 999);
        let id2 = manager.add_task("vim".to_string(), 1001, 1000);

        manager.get_task_mut(&id2).unwrap().stop();

        let running = manager.list_tasks_by_state(TaskState::Running);
        assert_eq!(running.len(), 1);
    }

    #[test]
    fn test_list_tasks_by_priority() {
        let mut manager = SigmaTaskManager::new();
        manager.add_task("bash".to_string(), 1000, 999);
        let vim_id = manager.add_task("vim".to_string(), 1001, 1000);
        manager.set_task_priority(&vim_id, TaskPriority::High);

        let high = manager.list_tasks_by_priority(TaskPriority::High);
        assert_eq!(high.len(), 1);
    }

    #[test]
    fn test_total_cpu_usage() {
        let mut manager = SigmaTaskManager::new();
        let id1 = manager.add_task("bash".to_string(), 1000, 999);
        let id2 = manager.add_task("vim".to_string(), 1001, 1000);

        manager.get_task_mut(&id1).unwrap().with_cpu_usage(50);
        manager.get_task_mut(&id2).unwrap().with_cpu_usage(30);

        assert_eq!(manager.total_cpu_usage(), 80);
    }

    #[test]
    fn test_global_instance() {
        init_sigmataskmanager();
        let manager = get_sigmataskmanager();
        assert_eq!(manager.task_count(), 0);

        let manager_mut = get_sigmataskmanager_mut();
        manager_mut.add_task("bash".to_string(), 1000, 999);
        assert_eq!(manager.task_count(), 1);
    }

    #[test]
    fn test_add_task_convenience() {
        init_sigmataskmanager();
        let id = add_task("bash".to_string(), 1000, 999);
        assert_eq!(id, "task-1");

        let manager = get_sigmataskmanager();
        assert!(manager.get_task(&id).is_some());
    }
}
