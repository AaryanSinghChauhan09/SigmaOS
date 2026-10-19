//! SigmaAutomation - Task Automation and Scheduling System
//!
//! Provides comprehensive task automation and scheduling inspired by Linux automation tools.
//! Supports task automation, scheduling, and workflow management.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
}

/// Task automation
#[derive(Debug, Clone)]
pub struct AutomationTask {
    pub id: String,
    pub name: String,
    pub command: String,
    pub enabled: bool,
    pub run_count: u32,
    pub last_run: u64,
}

impl AutomationTask {
    pub fn new(id: String, name: String, command: String) -> Self {
        Self {
            id,
            name,
            command,
            enabled: true,
            run_count: 0,
            last_run: 0,
        }
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn increment_run_count(&mut self) {
        self.run_count += 1;
    }

    pub fn set_last_run(&mut self, timestamp: u64) {
        self.last_run = timestamp;
    }
}

/// Automation manager
pub struct SigmaAutomation {
    tasks: BTreeMap<String, AutomationTask>,
    next_task_id: u64,
}

impl SigmaAutomation {
    pub fn new() -> Self {
        Self {
            tasks: BTreeMap::new(),
            next_task_id: 1,
        }
    }

    pub fn add_task(&mut self, name: String, command: String) -> String {
        let id = format!("task-{}", self.next_task_id);
        self.next_task_id += 1;
        let task = AutomationTask::new(id.clone(), name, command);
        self.tasks.insert(id.clone(), task);
        id
    }

    pub fn get_task(&self, id: &str) -> Option<&AutomationTask> {
        self.tasks.get(id)
    }

    pub fn get_task_mut(&mut self, id: &str) -> Option<&mut AutomationTask> {
        self.tasks.get_mut(id)
    }

    pub fn list_tasks(&self) -> Vec<&AutomationTask> {
        self.tasks.values().collect()
    }

    pub fn list_enabled_tasks(&self) -> Vec<&AutomationTask> {
        self.tasks.values().filter(|t| t.enabled).collect()
    }

    pub fn remove_task(&mut self, id: &str) -> bool {
        self.tasks.remove(id).is_some()
    }

    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }

    pub fn enabled_task_count(&self) -> usize {
        self.tasks.values().filter(|t| t.enabled).count()
    }

    pub fn total_run_count(&self) -> u32 {
        self.tasks.values().map(|t| t.run_count).sum()
    }
}

impl Default for SigmaAutomation {
    fn default() -> Self {
        Self::new()
    }
}

static mut SIGMAAUTOMATION_INSTANCE: Option<SigmaAutomation> = None;

pub fn init_sigmaautomation() {
    unsafe {
        SIGMAAUTOMATION_INSTANCE = Some(SigmaAutomation::new());
    }
}

pub fn get_sigmaautomation() -> &'static SigmaAutomation {
    unsafe {
        SIGMAAUTOMATION_INSTANCE
            .as_ref()
            .expect("SigmaAutomation not initialized")
    }
}

pub fn get_sigmaautomation_mut() -> &'static mut SigmaAutomation {
    unsafe {
        SIGMAAUTOMATION_INSTANCE
            .as_mut()
            .expect("SigmaAutomation not initialized")
    }
}

pub fn add_task(name: String, command: String) -> String {
    let manager = get_sigmaautomation_mut();
    manager.add_task(name, command)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_automation_task_creation() {
        let task = AutomationTask::new("task-1".to_string(), "Backup".to_string(), "rsync -av /home /backup".to_string());
        assert_eq!(task.id, "task-1");
        assert_eq!(task.name, "Backup");
        assert!(task.enabled);
    }

    #[test]
    fn test_automation_task_enable_disable() {
        let mut task = AutomationTask::new("task-1".to_string(), "Backup".to_string(), "rsync".to_string());
        task.disable();
        assert!(!task.enabled);

        task.enable();
        assert!(task.enabled);
    }

    #[test]
    fn test_automation_task_run_count() {
        let mut task = AutomationTask::new("task-1".to_string(), "Backup".to_string(), "rsync".to_string());
        task.increment_run_count();
        task.increment_run_count();
        assert_eq!(task.run_count, 2);
    }

    #[test]
    fn test_automation_manager() {
        let manager = SigmaAutomation::new();
        assert_eq!(manager.task_count(), 0);
    }

    #[test]
    fn test_add_task() {
        let mut manager = SigmaAutomation::new();
        let id = manager.add_task("Backup".to_string(), "rsync -av /home /backup".to_string());
        assert_eq!(id, "task-1");
        assert_eq!(manager.task_count(), 1);
    }

    #[test]
    fn test_list_enabled_tasks() {
        let mut manager = SigmaAutomation::new();
        let id1 = manager.add_task("Backup".to_string(), "rsync".to_string());
        let id2 = manager.add_task("Cleanup".to_string(), "rm -rf /tmp/*".to_string());

        manager.get_task_mut(&id2).unwrap().disable();

        let enabled = manager.list_enabled_tasks();
        assert_eq!(enabled.len(), 1);
    }

    #[test]
    fn test_total_run_count() {
        let mut manager = SigmaAutomation::new();
        let id1 = manager.add_task("Backup".to_string(), "rsync".to_string());
        let id2 = manager.add_task("Cleanup".to_string(), "rm".to_string());

        manager.get_task_mut(&id1).unwrap().increment_run_count();
        manager.get_task_mut(&id1).unwrap().increment_run_count();
        manager.get_task_mut(&id2).unwrap().increment_run_count();

        assert_eq!(manager.total_run_count(), 3);
    }

    #[test]
    fn test_global_instance() {
        init_sigmaautomation();
        let manager = get_sigmaautomation();
        assert_eq!(manager.task_count(), 0);

        let manager_mut = get_sigmaautomation_mut();
        manager_mut.add_task("Backup".to_string(), "rsync".to_string());
        assert_eq!(manager.task_count(), 1);
    }

    #[test]
    fn test_add_task_convenience() {
        init_sigmaautomation();
        let id = add_task("Backup".to_string(), "rsync".to_string());
        assert_eq!(id, "task-1");

        let manager = get_sigmaautomation();
        assert!(manager.get_task(&id).is_some());
    }
}
