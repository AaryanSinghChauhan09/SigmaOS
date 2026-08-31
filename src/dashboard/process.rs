// SigmaOS Process Manager
// OOP-based process management with monitoring and control

use crate::klib::HashMap;
use std::time::{Duration, Instant};

/// Process info
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub command: String,
    pub user: String,
    pub cpu_percent: f64,
    pub memory_mb: u64,
    pub memory_percent: f64,
    pub state: ProcessState,
    pub priority: ProcessPriority,
    pub threads: u32,
    pub created_at: u64,
}

/// Process state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Running,
    Sleeping,
    Stopped,
    Zombie,
    Dead,
}

/// Process priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProcessPriority {
    Low,
    Normal,
    High,
    Realtime,
}

/// Process action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessAction {
    Suspend,
    Resume,
    Terminate,
    Kill,
    ChangePriority(ProcessPriority),
}

/// Process filter
#[derive(Debug, Clone)]
pub struct ProcessFilter {
    pub name_contains: Option<String>,
    pub user: Option<String>,
    pub state: Option<ProcessState>,
    pub min_cpu_percent: Option<f64>,
    pub min_memory_mb: Option<u64>,
}

/// OOP trait for process monitoring strategies
pub trait ProcessMonitorStrategy {
    /// Get all processes
    fn get_processes(&self) -> Result<Vec<ProcessInfo>, ProcessError>;
    /// Get process by PID
    fn get_process(&self, pid: u32) -> Result<ProcessInfo, ProcessError>;
    /// Get strategy name
    fn name(&self) -> &str;
}

/// System process monitor
pub struct SystemProcessMonitor {
    processes: HashMap<u32, ProcessInfo>,
}

impl SystemProcessMonitor {
    pub fn new() -> Self {
        let mut processes = HashMap::new();

        // Simulated system processes
        processes.insert(
            1,
            ProcessInfo {
                pid: 1,
                name: "init".to_string(),
                command: "/sbin/init".to_string(),
                user: "root".to_string(),
                cpu_percent: 0.1,
                memory_mb: 8,
                memory_percent: 0.1,
                state: ProcessState::Running,
                priority: ProcessPriority::High,
                threads: 1,
                created_at: 0,
            },
        );

        processes.insert(
            100,
            ProcessInfo {
                pid: 100,
                name: "sigma-kernel".to_string(),
                command: "/boot/sigma-kernel".to_string(),
                user: "root".to_string(),
                cpu_percent: 5.0,
                memory_mb: 256,
                memory_percent: 2.5,
                state: ProcessState::Running,
                priority: ProcessPriority::Realtime,
                threads: 8,
                created_at: 1,
            },
        );

        processes.insert(
            500,
            ProcessInfo {
                pid: 500,
                name: "sigma-ui".to_string(),
                command: "/usr/bin/sigma-ui".to_string(),
                user: "user".to_string(),
                cpu_percent: 15.0,
                memory_mb: 512,
                memory_percent: 5.0,
                state: ProcessState::Running,
                priority: ProcessPriority::Normal,
                threads: 4,
                created_at: 10,
            },
        );

        Self { processes }
    }
}

impl ProcessMonitorStrategy for SystemProcessMonitor {
    fn get_processes(&self) -> Result<Vec<ProcessInfo>, ProcessError> {
        Ok(self.processes.values().cloned().collect())
    }

    fn get_process(&self, pid: u32) -> Result<ProcessInfo, ProcessError> {
        self.processes
            .get(&pid)
            .cloned()
            .ok_or_else(|| ProcessError::ProcessNotFound(pid))
    }

    fn name(&self) -> &str {
        "SystemProcessMonitor"
    }
}

/// OOP-based Process Manager
pub struct ProcessManager {
    monitor: Box<dyn ProcessMonitorStrategy>,
    process_history: HashMap<u32, Vec<ProcessInfo>>,
    auto_refresh_enabled: bool,
    refresh_interval: Duration,
    last_refresh: Option<Instant>,
}

impl ProcessManager {
    pub fn new(monitor: Box<dyn ProcessMonitorStrategy>) -> Self {
        Self {
            monitor,
            process_history: HashMap::new(),
            auto_refresh_enabled: false,
            refresh_interval: Duration::from_secs(5),
            last_refresh: None,
        }
    }

    /// Enable auto-refresh
    pub fn with_auto_refresh(mut self, enabled: bool, interval: Duration) -> Self {
        self.auto_refresh_enabled = enabled;
        self.refresh_interval = interval;
        self
    }

    /// Get all processes
    pub fn get_processes(&mut self) -> Result<Vec<ProcessInfo>, ProcessError> {
        if self.auto_refresh_enabled {
            self.auto_refresh_if_needed();
        }

        let processes = self.monitor.get_processes()?;

        // Update history
        for process in &processes {
            self.process_history
                .entry(process.pid)
                .or_insert_with(Vec::new)
                .push(process.clone());
        }

        Ok(processes)
    }

    /// Get process by PID
    pub fn get_process(&mut self, pid: u32) -> Result<ProcessInfo, ProcessError> {
        if self.auto_refresh_enabled {
            self.auto_refresh_if_needed();
        }

        self.monitor.get_process(pid)
    }

    /// Filter processes
    pub fn filter_processes(
        &mut self,
        filter: &ProcessFilter,
    ) -> Result<Vec<ProcessInfo>, ProcessError> {
        let processes = self.get_processes()?;

        Ok(processes
            .into_iter()
            .filter(|p| {
                if let Some(ref name) = filter.name_contains {
                    if !p.name.to_lowercase().contains(name) {
                        return false;
                    }
                }
                if let Some(ref user) = filter.user {
                    if &p.user != user {
                        return false;
                    }
                }
                if let Some(state) = filter.state {
                    if p.state != state {
                        return false;
                    }
                }
                if let Some(min_cpu) = filter.min_cpu_percent {
                    if p.cpu_percent < min_cpu {
                        return false;
                    }
                }
                if let Some(min_mem) = filter.min_memory_mb {
                    if p.memory_mb < min_mem {
                        return false;
                    }
                }
                true
            })
            .collect())
    }

    /// Search processes by name
    pub fn search_by_name(&mut self, name: &str) -> Result<Vec<ProcessInfo>, ProcessError> {
        let filter = ProcessFilter {
            name_contains: Some(name.to_lowercase()),
            user: None,
            state: None,
            min_cpu_percent: None,
            min_memory_mb: None,
        };
        self.filter_processes(&filter)
    }

    /// Get processes by user
    pub fn get_by_user(&mut self, user: &str) -> Result<Vec<ProcessInfo>, ProcessError> {
        let filter = ProcessFilter {
            name_contains: None,
            user: Some(user.to_string()),
            state: None,
            min_cpu_percent: None,
            min_memory_mb: None,
        };
        self.filter_processes(&filter)
    }

    /// Get top CPU consumers
    pub fn get_top_cpu(&mut self, count: usize) -> Result<Vec<ProcessInfo>, ProcessError> {
        let mut processes = self.get_processes()?;
        processes.sort_by(|a, b| b.cpu_percent.partial_cmp(&a.cpu_percent).unwrap());
        Ok(processes.into_iter().take(count).collect())
    }

    /// Get top memory consumers
    pub fn get_top_memory(&mut self, count: usize) -> Result<Vec<ProcessInfo>, ProcessError> {
        let mut processes = self.get_processes()?;
        processes.sort_by(|a, b| b.memory_mb.cmp(&a.memory_mb));
        Ok(processes.into_iter().take(count).collect())
    }

    /// Execute action on process
    pub fn execute_action(&mut self, _pid: u32, action: ProcessAction) -> Result<(), ProcessError> {
        // Simulated action execution
        match action {
            ProcessAction::Terminate => {
                // In real implementation, would send SIGTERM
            }
            ProcessAction::Kill => {
                // In real implementation, would send SIGKILL
            }
            ProcessAction::Suspend => {
                // In real implementation, would send SIGSTOP
            }
            ProcessAction::Resume => {
                // In real implementation, would send SIGCONT
            }
            ProcessAction::ChangePriority(_) => {
                // In real implementation, would change nice value
            }
        }
        Ok(())
    }

    /// Get process history
    pub fn get_process_history(&self, pid: u32) -> Option<&[ProcessInfo]> {
        self.process_history.get(&pid).map(|v| v.as_slice())
    }

    /// Auto-refresh if needed
    fn auto_refresh_if_needed(&mut self) {
        if let Some(last) = self.last_refresh {
            if last.elapsed() < self.refresh_interval {
                return;
            }
        }
        self.last_refresh = Some(Instant::now());
    }

    /// Get total CPU usage
    pub fn get_total_cpu(&mut self) -> Result<f64, ProcessError> {
        let processes = self.get_processes()?;
        Ok(processes.iter().map(|p| p.cpu_percent).sum())
    }

    /// Get total memory usage
    pub fn get_total_memory(&mut self) -> Result<u64, ProcessError> {
        let processes = self.get_processes()?;
        Ok(processes.iter().map(|p| p.memory_mb).sum())
    }

    /// Get process count
    pub fn get_process_count(&mut self) -> Result<usize, ProcessError> {
        let processes = self.get_processes()?;
        Ok(processes.len())
    }

    /// Is auto-refresh enabled
    pub fn is_auto_refresh_enabled(&self) -> bool {
        self.auto_refresh_enabled
    }

    /// Enable auto-refresh
    pub fn enable_auto_refresh(&mut self, enabled: bool) {
        self.auto_refresh_enabled = enabled;
    }
}

impl Default for ProcessManager {
    fn default() -> Self {
        Self::new(Box::new(SystemProcessMonitor::new()))
            .with_auto_refresh(false, Duration::from_secs(5))
    }
}

/// Process errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessError {
    ProcessNotFound(u32),
    PermissionDenied(u32),
    ActionFailed(String),
    MonitorError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_info() {
        let info = ProcessInfo {
            pid: 1,
            name: "test".to_string(),
            command: "/bin/test".to_string(),
            user: "root".to_string(),
            cpu_percent: 5.0,
            memory_mb: 100,
            memory_percent: 1.0,
            state: ProcessState::Running,
            priority: ProcessPriority::Normal,
            threads: 1,
            created_at: 0,
        };
        assert_eq!(info.name, "test");
    }

    #[test]
    fn test_system_process_monitor() {
        let monitor = SystemProcessMonitor::new();
        assert_eq!(monitor.name(), "SystemProcessMonitor");
    }

    #[test]
    fn test_process_manager() {
        let manager = ProcessManager::default();
        assert!(!manager.is_auto_refresh_enabled());
    }

    #[test]
    fn test_get_processes() {
        let mut manager = ProcessManager::default();
        let processes = manager.get_processes().unwrap();
        assert!(!processes.is_empty());
    }

    #[test]
    fn test_search_by_name() {
        let mut manager = ProcessManager::default();
        let results = manager.search_by_name("sigma").unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_get_top_cpu() {
        let mut manager = ProcessManager::default();
        let top = manager.get_top_cpu(3).unwrap();
        assert!(top.len() <= 3);
    }
}
