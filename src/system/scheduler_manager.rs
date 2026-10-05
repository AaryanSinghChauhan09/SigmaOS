//! Scheduler Manager
//!
//! Scheduler management inspired by Linux Mint's scheduler and Omarchy's
//! scheduler utilities, supporting task scheduling, cron jobs, and automation.

use std::collections::HashMap;

/// Schedule frequency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleFrequency {
    Once,
    Hourly,
    Daily,
    Weekly,
    Monthly,
}

impl ScheduleFrequency {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "once" => Some(ScheduleFrequency::Once),
            "hourly" => Some(ScheduleFrequency::Hourly),
            "daily" => Some(ScheduleFrequency::Daily),
            "weekly" => Some(ScheduleFrequency::Weekly),
            "monthly" => Some(ScheduleFrequency::Monthly),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ScheduleFrequency::Once => "Once",
            ScheduleFrequency::Hourly => "Hourly",
            ScheduleFrequency::Daily => "Daily",
            ScheduleFrequency::Weekly => "Weekly",
            ScheduleFrequency::Monthly => "Monthly",
        }
    }
}

/// Schedule status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Disabled,
}

impl ScheduleStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "pending" => Some(ScheduleStatus::Pending),
            "running" => Some(ScheduleStatus::Running),
            "completed" => Some(ScheduleStatus::Completed),
            "failed" => Some(ScheduleStatus::Failed),
            "disabled" => Some(ScheduleStatus::Disabled),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ScheduleStatus::Pending => "Pending",
            ScheduleStatus::Running => "Running",
            ScheduleStatus::Completed => "Completed",
            ScheduleStatus::Failed => "Failed",
            ScheduleStatus::Disabled => "Disabled",
        }
    }
}

/// Scheduled task
#[derive(Debug, Clone)]
pub struct ScheduledTask {
    pub id: String,
    pub name: String,
    pub command: String,
    pub frequency: ScheduleFrequency,
    pub status: ScheduleStatus,
    pub next_run: u64,
    pub last_run: Option<u64>,
    pub is_enabled: bool,
}

impl ScheduledTask {
    pub fn new(
        id: String,
        name: String,
        command: String,
        frequency: ScheduleFrequency,
    ) -> Self {
        Self {
            id,
            name,
            command,
            frequency,
            status: ScheduleStatus::Pending,
            next_run: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            last_run: None,
            is_enabled: true,
        }
    }

    pub fn set_status(&mut self, status: ScheduleStatus) {
        self.status = status;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.is_enabled = enabled;
    }
}

/// Scheduler manager
#[derive(Debug)]
pub struct SchedulerManager {
    tasks: HashMap<String, ScheduledTask>,
    next_task_id: u64,
}

impl SchedulerManager {
    pub fn new() -> Self {
        let mut manager = Self {
            tasks: HashMap::new(),
            next_task_id: 1,
        };

        // Add default tasks
        manager.add_default_tasks();

        manager
    }

    /// Add default tasks
    fn add_default_tasks(&mut self) {
        let tasks = vec![
            ("backup", "System Backup", "/usr/bin/sigma-backup", ScheduleFrequency::Daily),
            ("update", "System Update", "/usr/bin/sigma-update", ScheduleFrequency::Weekly),
            ("cleanup", "System Cleanup", "/usr/bin/sigma-cleanup", ScheduleFrequency::Weekly),
            ("logrotate", "Log Rotation", "/usr/sbin/logrotate", ScheduleFrequency::Daily),
        ];

        for (id, name, command, frequency) in tasks {
            let task = ScheduledTask::new(
                id.to_string(),
                name.to_string(),
                command.to_string(),
                frequency,
            );
            self.tasks.insert(id.to_string(), task);
        }
    }

    /// Add a task
    pub fn add_task(&mut self, task: ScheduledTask) {
        self.tasks.insert(task.id.clone(), task);
    }

    /// Get a task
    pub fn get_task(&self, id: &str) -> Option<&ScheduledTask> {
        self.tasks.get(id)
    }

    /// List all tasks
    pub fn list_tasks(&self) -> Vec<&ScheduledTask> {
        self.tasks.values().collect()
    }

    /// List by frequency
    pub fn list_by_frequency(&self, frequency: ScheduleFrequency) -> Vec<&ScheduledTask> {
        self.tasks.values()
            .filter(|t| t.frequency == frequency)
            .collect()
    }

    /// List by status
    pub fn list_by_status(&self, status: ScheduleStatus) -> Vec<&ScheduledTask> {
        self.tasks.values()
            .filter(|t| t.status == status)
            .collect()
    }

    /// Create a task
    pub fn create_task(
        &mut self,
        name: String,
        command: String,
        frequency: ScheduleFrequency,
    ) -> String {
        let id = format!("task-{}", self.next_task_id);
        self.next_task_id += 1;

        let task = ScheduledTask::new(id.clone(), name, command, frequency);
        self.tasks.insert(id.clone(), task);
        id
    }

    /// Enable a task
    pub fn enable_task(&mut self, id: &str) -> Result<(), String> {
        let task = self.tasks.get_mut(id)
            .ok_or_else(|| format!("Task {} not found", id))?;

        task.set_enabled(true);
        Ok(())
    }

    /// Disable a task
    pub fn disable_task(&mut self, id: &str) -> Result<(), String> {
        let task = self.tasks.get_mut(id)
            .ok_or_else(|| format!("Task {} not found", id))?;

        task.set_enabled(false);
        task.set_status(ScheduleStatus::Disabled);
        Ok(())
    }

    /// Run a task
    pub fn run_task(&mut self, id: &str) -> Result<(), String> {
        let task = self.tasks.get_mut(id)
            .ok_or_else(|| format!("Task {} not found", id))?;

        if !task.is_enabled {
            return Err(format!("Task {} is disabled", id));
        }

        task.set_status(ScheduleStatus::Running);

        // Simulate task execution
        task.set_status(ScheduleStatus::Completed);
        task.last_run = Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());

        Ok(())
    }

    /// Remove a task
    pub fn remove_task(&mut self, id: &str) -> Result<(), String> {
        self.tasks.remove(id)
            .ok_or_else(|| format!("Task {} not found", id))?;
        Ok(())
    }

    /// Get statistics
    pub fn get_statistics(&self) -> SchedulerStatistics {
        let total_tasks = self.tasks.len();
        let enabled_count = self.tasks.values()
            .filter(|t| t.is_enabled)
            .count();
        let pending_count = self.tasks.values()
            .filter(|t| t.status == ScheduleStatus::Pending)
            .count();
        let failed_count = self.tasks.values()
            .filter(|t| t.status == ScheduleStatus::Failed)
            .count();

        SchedulerStatistics {
            total_tasks,
            enabled_count,
            pending_count,
            failed_count,
        }
    }
}

impl Default for SchedulerManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Scheduler statistics
#[derive(Debug, Clone)]
pub struct SchedulerStatistics {
    pub total_tasks: usize,
    pub enabled_count: usize,
    pub pending_count: usize,
    pub failed_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schedule_frequency_from_str() {
        assert_eq!(ScheduleFrequency::from_str("daily"), Some(ScheduleFrequency::Daily));
        assert_eq!(ScheduleFrequency::from_str("weekly"), Some(ScheduleFrequency::Weekly));
    }

    #[test]
    fn test_schedule_status_from_str() {
        assert_eq!(ScheduleStatus::from_str("pending"), Some(ScheduleStatus::Pending));
        assert_eq!(ScheduleStatus::from_str("completed"), Some(ScheduleStatus::Completed));
    }

    #[test]
    fn test_scheduled_task_creation() {
        let task = ScheduledTask::new(
            "test".to_string(),
            "Test".to_string(),
            "/bin/test".to_string(),
            ScheduleFrequency::Daily,
        );
        assert_eq!(task.name, "Test");
    }

    #[test]
    fn test_scheduler_manager_creation() {
        let manager = SchedulerManager::new();
        assert!(manager.get_task("backup").is_some());
    }

    #[test]
    fn test_create_task() {
        let mut manager = SchedulerManager::new();
        let id = manager.create_task(
            "My Task".to_string(),
            "/bin/mytask".to_string(),
            ScheduleFrequency::Hourly,
        );
        assert!(manager.get_task(&id).is_some());
    }

    #[test]
    fn test_enable_disable() {
        let mut manager = SchedulerManager::new();
        manager.disable_task("backup").ok();
        assert!(!manager.get_task("backup").unwrap().is_enabled);
        manager.enable_task("backup").ok();
        assert!(manager.get_task("backup").unwrap().is_enabled);
    }

    #[test]
    fn test_run_task() {
        let mut manager = SchedulerManager::new();
        assert!(manager.run_task("backup").is_ok());
        assert_eq!(manager.get_task("backup").unwrap().status, ScheduleStatus::Completed);
    }

    #[test]
    fn test_list_by_frequency() {
        let manager = SchedulerManager::new();
        let daily = manager.list_by_frequency(ScheduleFrequency::Daily);
        assert!(daily.len() > 0);
    }

    #[test]
    fn test_statistics() {
        let manager = SchedulerManager::new();
        let stats = manager.get_statistics();
        assert!(stats.total_tasks >= 4);
    }
}
