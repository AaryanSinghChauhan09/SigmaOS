#![no_std]
extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Cleaner manager for system cleanup and maintenance
/// Inspired by Linux cleanup tools (bleachbit, ncdu, systemd-tmpfiles)
#[derive(Debug, Clone)]
pub struct CleanerManager {
    tasks: Vec<CleanupTask>,
}

/// A cleanup task
#[derive(Debug, Clone)]
pub struct CleanupTask {
    id: String,
    name: String,
    target_path: String,
    task_type: CleanupType,
    state: TaskState,
    size_bytes: u64,
}

/// Cleanup type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CleanupType {
    Cache,
    TempFiles,
    Logs,
    Thumbnails,
    PackageCache,
    OldKernels,
    Trash,
}

/// Task state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Pending,
    Running,
    Completed,
    Failed,
}

impl CleanerManager {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
        }
    }

    /// Add a cleanup task
    pub fn add_task(&mut self, id: String, name: String, target_path: String, task_type: CleanupType, size_bytes: u64) -> Result<String, String> {
        if self.tasks.iter().any(|t| t.id == id) {
            return Err(format!("Task with id {} already exists", id));
        }
        
        let task = CleanupTask {
            id: id.clone(),
            name,
            target_path,
            task_type,
            state: TaskState::Pending,
            size_bytes,
        };
        self.tasks.push(task);
        Ok(id)
    }

    /// Remove a task
    pub fn remove_task(&mut self, id: &str) -> Result<(), String> {
        let pos = self.tasks.iter().position(|t| t.id == id)
            .ok_or_else(|| format!("Task with id {} not found", id))?;
        self.tasks.remove(pos);
        Ok(())
    }

    /// Execute a cleanup task
    pub fn execute_task(&mut self, id: &str) -> Result<(), String> {
        let task = self.tasks.iter_mut().find(|t| t.id == id)
            .ok_or_else(|| format!("Task with id {} not found", id))?;
        task.state = TaskState::Running;
        // In a real implementation, this would perform the cleanup
        task.state = TaskState::Completed;
        task.size_bytes = 0;
        Ok(())
    }

    /// Mark task as failed
    pub fn mark_failed(&mut self, id: &str) -> Result<(), String> {
        let task = self.tasks.iter_mut().find(|t| t.id == id)
            .ok_or_else(|| format!("Task with id {} not found", id))?;
        task.state = TaskState::Failed;
        Ok(())
    }

    /// Get all tasks
    pub fn get_tasks(&self) -> &[CleanupTask] {
        &self.tasks
    }

    /// Get pending tasks
    pub fn get_pending_tasks(&self) -> Vec<&CleanupTask> {
        self.tasks.iter().filter(|t| t.state == TaskState::Pending).collect()
    }

    /// Get task by ID
    pub fn get_task(&self, id: &str) -> Option<&CleanupTask> {
        self.tasks.iter().find(|t| t.id == id)
    }

    /// Get tasks by type
    pub fn get_tasks_by_type(&self, task_type: CleanupType) -> Vec<&CleanupTask> {
        self.tasks.iter().filter(|t| t.task_type == task_type).collect()
    }

    /// Get total cleanup size
    pub fn get_total_size(&self) -> u64 {
        self.tasks.iter().map(|t| t.size_bytes).sum()
    }
}

impl Default for CleanerManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task() {
        let mut manager = CleanerManager::new();
        let result = manager.add_task("task1".to_string(), "Clean Cache".to_string(), "/var/cache".to_string(), CleanupType::Cache, 1024 * 1024 * 100);
        assert!(result.is_ok());
        assert_eq!(manager.get_tasks().len(), 1);
    }

    #[test]
    fn test_duplicate_task() {
        let mut manager = CleanerManager::new();
        manager.add_task("task1".to_string(), "Clean Cache".to_string(), "/var/cache".to_string(), CleanupType::Cache, 1024 * 1024 * 100).unwrap();
        let result = manager.add_task("task1".to_string(), "Clean Temp".to_string(), "/tmp".to_string(), CleanupType::TempFiles, 1024 * 1024 * 50);
        assert!(result.is_err());
    }

    #[test]
    fn test_execute_task() {
        let mut manager = CleanerManager::new();
        manager.add_task("task1".to_string(), "Clean Cache".to_string(), "/var/cache".to_string(), CleanupType::Cache, 1024 * 1024 * 100).unwrap();
        
        manager.execute_task("task1").unwrap();
        assert_eq!(manager.get_task("task1").unwrap().state, TaskState::Completed);
        assert_eq!(manager.get_task("task1").unwrap().size_bytes, 0);
    }

    #[test]
    fn test_get_tasks_by_type() {
        let mut manager = CleanerManager::new();
        manager.add_task("task1".to_string(), "Clean Cache".to_string(), "/var/cache".to_string(), CleanupType::Cache, 1024 * 1024 * 100).unwrap();
        manager.add_task("task2".to_string(), "Clean Temp".to_string(), "/tmp".to_string(), CleanupType::TempFiles, 1024 * 1024 * 50).unwrap();
        
        let cache_tasks = manager.get_tasks_by_type(CleanupType::Cache);
        assert_eq!(cache_tasks.len(), 1);
        assert_eq!(cache_tasks[0].id, "task1");
    }
}
