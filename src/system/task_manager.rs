//! Task Manager
//!
//! Task management inspired by Linux Mint's task manager and Omarchy's
//! task utilities, supporting process listing, filtering, and termination.

use std::collections::HashMap;

/// Process state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskProcessState {
    Running,
    Sleeping,
    Stopped,
    Zombie,
}

impl TaskProcessState {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "running" => Some(TaskProcessState::Running),
            "sleeping" => Some(TaskProcessState::Sleeping),
            "stopped" => Some(TaskProcessState::Stopped),
            "zombie" => Some(TaskProcessState::Zombie),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            TaskProcessState::Running => "Running",
            TaskProcessState::Sleeping => "Sleeping",
            TaskProcessState::Stopped => "Stopped",
            TaskProcessState::Zombie => "Zombie",
        }
    }
}

/// Process entry
#[derive(Debug, Clone)]
pub struct TaskProcessEntry {
    pub pid: u32,
    pub name: String,
    pub command: String,
    pub state: TaskProcessState,
    pub cpu_percent: f32,
    pub memory_mb: f32,
    pub user: String,
    pub threads: u32,
}

impl TaskProcessEntry {
    pub fn new(pid: u32, name: String, command: String) -> Self {
        Self {
            pid,
            name,
            command,
            state: TaskProcessState::Running,
            cpu_percent: 0.0,
            memory_mb: 0.0,
            user: "root".to_string(),
            threads: 1,
        }
    }

    pub fn set_state(&mut self, state: TaskProcessState) {
        self.state = state;
    }

    pub fn set_cpu(&mut self, cpu: f32) {
        self.cpu_percent = cpu;
    }

    pub fn set_memory(&mut self, memory: f32) {
        self.memory_mb = memory;
    }
}

/// Task manager
#[derive(Debug)]
pub struct ProcessTaskManager {
    processes: HashMap<u32, TaskProcessEntry>,
    next_pid: u32,
}

impl ProcessTaskManager {
    pub fn new() -> Self {
        let mut manager = Self {
            processes: HashMap::new(),
            next_pid: 1,
        };

        // Add default processes
        manager.add_default_processes();

        manager
    }

    /// Add default processes
    fn add_default_processes(&mut self) {
        let processes = vec![
            (1, "init", "/sbin/init", TaskProcessState::Running, 0.1, 10.0, "root"),
            (2, "kthreadd", "[kthreadd]", TaskProcessState::Sleeping, 0.0, 0.0, "root"),
            (100, "systemd", "/usr/lib/systemd/systemd", TaskProcessState::Running, 0.5, 20.0, "root"),
            (200, "sigma-shell", "/usr/bin/sigma-shell", TaskProcessState::Running, 1.0, 50.0, "user"),
            (300, "sigma-wm", "/usr/bin/sigma-wm", TaskProcessState::Running, 2.0, 100.0, "user"),
        ];

        for (pid, name, command, state, cpu, memory, user) in processes {
            let mut proc = TaskProcessEntry::new(pid, name.to_string(), command.to_string());
            proc.set_state(state);
            proc.set_cpu(cpu);
            proc.set_memory(memory);
            proc.user = user.to_string();
            self.processes.insert(pid, proc);
            self.next_pid = pid + 1;
        }
    }

    /// Add a process
    pub fn add_process(&mut self, process: TaskProcessEntry) {
        self.processes.insert(process.pid, process);
    }

    /// Get a process
    pub fn get_process(&self, pid: u32) -> Option<&TaskProcessEntry> {
        self.processes.get(&pid)
    }

    /// List all processes
    pub fn list_processes(&self) -> Vec<&TaskProcessEntry> {
        self.processes.values().collect()
    }

    /// List by state
    pub fn list_by_state(&self, state: TaskProcessState) -> Vec<&TaskProcessEntry> {
        self.processes.values()
            .filter(|p| p.state == state)
            .collect()
    }

    /// List by user
    pub fn list_by_user(&self, user: &str) -> Vec<&TaskProcessEntry> {
        self.processes.values()
            .filter(|p| p.user == user)
            .collect()
    }

    /// Search processes
    pub fn search(&self, query: &str) -> Vec<&TaskProcessEntry> {
        let query_lower = query.to_lowercase();
        self.processes.values()
            .filter(|p| {
                p.name.to_lowercase().contains(&query_lower) ||
                p.command.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Terminate a process
    pub fn terminate(&mut self, pid: u32) -> Result<(), String> {
        let process = self.processes.get(&pid)
            .ok_or_else(|| format!("Process {} not found", pid))?;

        if process.pid == 1 {
            return Err("Cannot terminate init process".to_string());
        }

        self.processes.remove(&pid);
        Ok(())
    }

    /// Kill a process
    pub fn kill(&mut self, pid: u32) -> Result<(), String> {
        self.terminate(pid)
    }

    /// Get process count
    pub fn get_process_count(&self) -> usize {
        self.processes.len()
    }

    /// Get total CPU usage
    pub fn get_total_cpu(&self) -> f32 {
        self.processes.values()
            .map(|p| p.cpu_percent)
            .sum()
    }

    /// Get total memory usage
    pub fn get_total_memory(&self) -> f32 {
        self.processes.values()
            .map(|p| p.memory_mb)
            .sum()
    }

    /// Get statistics
    pub fn get_statistics(&self) -> ProcessTaskStatistics {
        let total_processes = self.processes.len();
        let running_count = self.processes.values()
            .filter(|p| p.state == TaskProcessState::Running)
            .count();
        let sleeping_count = self.processes.values()
            .filter(|p| p.state == TaskProcessState::Sleeping)
            .count();
        let zombie_count = self.processes.values()
            .filter(|p| p.state == TaskProcessState::Zombie)
            .count();
        let total_cpu = self.get_total_cpu();
        let total_memory = self.get_total_memory();

        ProcessTaskStatistics {
            total_processes,
            running_count,
            sleeping_count,
            zombie_count,
            total_cpu,
            total_memory,
        }
    }
}

impl Default for ProcessTaskManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Task manager statistics
#[derive(Debug, Clone)]
pub struct ProcessTaskStatistics {
    pub total_processes: usize,
    pub running_count: usize,
    pub sleeping_count: usize,
    pub zombie_count: usize,
    pub total_cpu: f32,
    pub total_memory: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_state_from_str() {
        assert_eq!(TaskProcessState::from_str("running"), Some(TaskProcessState::Running));
        assert_eq!(TaskProcessState::from_str("sleeping"), Some(TaskProcessState::Sleeping));
    }

    #[test]
    fn test_process_entry_creation() {
        let proc = TaskProcessEntry::new(
            1,
            "test".to_string(),
            "/bin/test".to_string(),
        );
        assert_eq!(proc.name, "test");
    }

    #[test]
    fn test_task_manager_creation() {
        let manager = ProcessTaskManager::new();
        assert!(manager.get_process(1).is_some());
    }

    #[test]
    fn test_add_process() {
        let mut manager = ProcessTaskManager::new();
        let proc = TaskProcessEntry::new(
            999,
            "test".to_string(),
            "/bin/test".to_string(),
        );
        manager.add_process(proc);
        assert!(manager.get_process(999).is_some());
    }

    #[test]
    fn test_list_by_state() {
        let manager = ProcessTaskManager::new();
        let running = manager.list_by_state(TaskProcessState::Running);
        assert!(running.len() > 0);
    }

    #[test]
    fn test_list_by_user() {
        let manager = ProcessTaskManager::new();
        let user = manager.list_by_user("user");
        assert!(user.len() > 0);
    }

    #[test]
    fn test_search() {
        let manager = ProcessTaskManager::new();
        let results = manager.search("sigma");
        assert!(results.len() > 0);
    }

    #[test]
    fn test_terminate() {
        let mut manager = ProcessTaskManager::new();
        assert!(manager.terminate(200).is_ok());
        assert!(manager.get_process(200).is_none());
    }

    #[test]
    fn test_terminate_init_fails() {
        let mut manager = ProcessTaskManager::new();
        assert!(manager.terminate(1).is_err());
    }

    #[test]
    fn test_statistics() {
        let manager = ProcessTaskManager::new();
        let stats = manager.get_statistics();
        assert!(stats.total_processes >= 5);
    }
}
