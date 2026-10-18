// SPDX-License-Identifier: MIT
// SigmaOS - Task Manager
// Inspired by GNOME System Monitor and Task Manager
// Provides process and task management

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Process state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Running,
    Sleeping,
    Waiting,
    Stopped,
    Zombie,
}

/// Process
#[derive(Debug, Clone)]
pub struct Process {
    pub pid: u32,
    pub name: String,
    pub state: ProcessState,
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub user: String,
    pub command: String,
    pub priority: i32,
}

/// Task manager
pub struct TaskManager {
    pub processes: Vec<Process>,
    pub sort_by: String,
    pub sort_ascending: bool,
    pub filter: String,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            sort_by: "name".to_string(),
            sort_ascending: true,
            filter: String::new(),
        }
    }

    /// Refresh process list
    pub fn refresh(&mut self) {
        // Simplified - in real implementation would read from /proc
        self.processes.clear();

        // Add sample processes
        self.processes.push(Process {
            pid: 1,
            name: "init".to_string(),
            state: ProcessState::Sleeping,
            cpu_usage: 0.1,
            memory_usage: 1024 * 1024,
            user: "root".to_string(),
            command: "/sbin/init".to_string(),
            priority: 0,
        });
    }

    /// Kill process
    pub fn kill_process(&mut self, pid: u32, signal: i32) -> Result<(), &'static str> {
        let index = self
            .processes
            .iter()
            .position(|p| p.pid == pid)
            .ok_or("Process not found")?;
        self.processes.remove(index);
        Ok(())
    }

    /// Set sort by
    pub fn set_sort_by(&mut self, field: String) {
        self.sort_by = field;
    }

    /// Set filter
    pub fn set_filter(&mut self, filter: String) {
        self.filter = filter;
    }

    /// Get filtered processes
    pub fn get_filtered_processes(&self) -> Vec<&Process> {
        if self.filter.is_empty() {
            self.processes.iter().collect()
        } else {
            let filter_lower = self.filter.to_lowercase();
            self.processes
                .iter()
                .filter(|p| {
                    p.name.to_lowercase().contains(&filter_lower)
                        || p.command.to_lowercase().contains(&filter_lower)
                })
                .collect()
        }
    }

    /// Process count
    pub fn process_count(&self) -> usize {
        self.processes.len()
    }
}

/// Global task manager
static mut TASK_MANAGER: Option<TaskManager> = None;

/// Initialize task manager
pub fn init_task_manager() {
    unsafe {
        TASK_MANAGER = Some(TaskManager::new());
    }
}

/// Get task manager
pub fn get_task_manager() -> &'static mut TaskManager {
    unsafe { TASK_MANAGER.as_mut().expect("Task manager not initialized") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_manager_creation() {
        let tm = TaskManager::new();
        assert_eq!(tm.process_count(), 0);
    }

    #[test]
    fn test_refresh() {
        let mut tm = TaskManager::new();
        tm.refresh();
        assert_eq!(tm.process_count(), 1);
    }

    #[test]
    fn test_kill_process() {
        let mut tm = TaskManager::new();
        tm.refresh();
        tm.kill_process(1, 9).unwrap();
        assert_eq!(tm.process_count(), 0);
    }

    #[test]
    fn test_filter() {
        let mut tm = TaskManager::new();
        tm.refresh();
        tm.set_filter("init".to_string());
        let filtered = tm.get_filtered_processes();
        assert_eq!(filtered.len(), 1);
    }
}
