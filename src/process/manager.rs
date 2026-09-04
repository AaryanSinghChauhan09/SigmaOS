// SPDX-License-Identifier: MIT
/// SigmaOS: Process Manager
/// Manages process creation, termination, scheduling, and inter-process communication

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;
use core::fmt;

/// Process State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Created,
    Ready,
    Running,
    Blocked,
    Stopped,
    Terminated,
}

/// Process Priority (for scheduling)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    RealTime = 3,
    High = 2,
    Normal = 1,
    Low = 0,
}

/// Exit Status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExitStatus {
    pub code: i32,
    pub signal: i32,
}

/// Resource Limits
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub max_memory: u64,
    pub max_open_files: u32,
    pub max_threads: u32,
    pub max_cpu_time_ms: u64,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory: 512 * 1024 * 1024, // 512MB
            max_open_files: 1024,
            max_threads: 256,
            max_cpu_time_ms: 3_600_000, // 1 hour
        }
    }
}

/// Process Information
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub ppid: u32, // Parent PID
    pub name: String,
    pub state: ProcessState,
    pub priority: Priority,
    pub uid: u32,
    pub gid: u32,
    pub exit_status: Option<ExitStatus>,
    pub memory_used: u64,
    pub cpu_time_ms: u64,
    pub thread_count: u32,
    pub open_files: u32,
    pub created_at: u64,
    pub resource_limits: ResourceLimits,
}

impl ProcessInfo {
    pub fn new(pid: u32, ppid: u32, name: String) -> Self {
        Self {
            pid,
            ppid,
            name,
            state: ProcessState::Created,
            priority: Priority::Normal,
            uid: 0,
            gid: 0,
            exit_status: None,
            memory_used: 0,
            cpu_time_ms: 0,
            thread_count: 1,
            open_files: 0,
            created_at: 0,
            resource_limits: ResourceLimits::default(),
        }
    }

    pub fn is_running(&self) -> bool {
        self.state == ProcessState::Running || self.state == ProcessState::Ready
    }

    pub fn is_alive(&self) -> bool {
        self.state != ProcessState::Terminated
    }
}

/// Process Error
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessError {
    ProcessNotFound,
    InvalidPid,
    OutOfMemory,
    TooManyProcesses,
    PermissionDenied,
    InvalidState,
    ResourceLimitExceeded,
    ForkFailed,
    ExecFailed,
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProcessNotFound => write!(f, "Process not found"),
            Self::InvalidPid => write!(f, "Invalid process ID"),
            Self::OutOfMemory => write!(f, "Out of memory"),
            Self::TooManyProcesses => write!(f, "Too many processes"),
            Self::PermissionDenied => write!(f, "Permission denied"),
            Self::InvalidState => write!(f, "Invalid process state"),
            Self::ResourceLimitExceeded => write!(f, "Resource limit exceeded"),
            Self::ForkFailed => write!(f, "Fork failed"),
            Self::ExecFailed => write!(f, "Exec failed"),
        }
    }
}

/// Process Manager - central process management system
pub struct ProcessManager {
    processes: BTreeMap<u32, ProcessInfo>,
    next_pid: u32,
    max_processes: u32,
    init_pid: u32,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            processes: BTreeMap::new(),
            next_pid: 1,
            max_processes: 32768,
            init_pid: 0,
        }
    }

    /// Create initial init process (PID 1)
    pub fn init(&mut self) -> Result<u32, ProcessError> {
        let init_process = ProcessInfo::new(1, 0, "init".to_string());
        self.processes.insert(1, init_process);
        self.init_pid = 1;
        self.next_pid = 2;
        Ok(1)
    }

    /// Fork a process (create child process)
    pub fn fork(&mut self, ppid: u32) -> Result<u32, ProcessError> {
        // Verify parent exists
        if !self.processes.contains_key(&ppid) {
            return Err(ProcessError::ProcessNotFound);
        }

        if self.processes.len() >= self.max_processes as usize {
            return Err(ProcessError::TooManyProcesses);
        }

        let parent = self.processes.get(&ppid).ok_or(ProcessError::ProcessNotFound)?;
        let new_pid = self.next_pid;
        self.next_pid += 1;

        let mut child = ProcessInfo::new(new_pid, ppid, format!("{}-fork", parent.name));
        child.uid = parent.uid;
        child.gid = parent.gid;
        child.priority = parent.priority;
        child.resource_limits = parent.resource_limits.clone();

        self.processes.insert(new_pid, child);
        Ok(new_pid)
    }

    /// Execute a program (replace process image)
    pub fn exec(&mut self, pid: u32, _executable: &str) -> Result<(), ProcessError> {
        if let Some(process) = self.processes.get_mut(&pid) {
            process.state = ProcessState::Ready;
            Ok(())
        } else {
            Err(ProcessError::ProcessNotFound)
        }
    }

    /// Exit a process
    pub fn exit(&mut self, pid: u32, code: i32) -> Result<(), ProcessError> {
        if let Some(process) = self.processes.get_mut(&pid) {
            process.state = ProcessState::Terminated;
            process.exit_status = Some(ExitStatus { code, signal: 0 });
            Ok(())
        } else {
            Err(ProcessError::ProcessNotFound)
        }
    }

    /// Wait for a child process
    pub fn wait(&self, ppid: u32, wpid: i32) -> Result<(u32, ExitStatus), ProcessError> {
        // Find children of parent
        for (_pid, process) in &self.processes {
            if process.ppid == ppid && process.state == ProcessState::Terminated {
                let status = process.exit_status.ok_or(ProcessError::InvalidState)?;
                return Ok((process.pid, status));
            }
        }

        Err(ProcessError::InvalidState)
    }

    /// Get process information
    pub fn get_process(&self, pid: u32) -> Result<ProcessInfo, ProcessError> {
        self.processes
            .get(&pid)
            .cloned()
            .ok_or(ProcessError::ProcessNotFound)
    }

    /// List all processes
    pub fn list_processes(&self) -> Vec<ProcessInfo> {
        self.processes.values().cloned().collect()
    }

    /// Change process priority
    pub fn set_priority(&mut self, pid: u32, priority: Priority) -> Result<(), ProcessError> {
        if let Some(process) = self.processes.get_mut(&pid) {
            process.priority = priority;
            Ok(())
        } else {
            Err(ProcessError::ProcessNotFound)
        }
    }

    /// Set process state
    pub fn set_state(&mut self, pid: u32, state: ProcessState) -> Result<(), ProcessError> {
        if let Some(process) = self.processes.get_mut(&pid) {
            process.state = state;
            Ok(())
        } else {
            Err(ProcessError::ProcessNotFound)
        }
    }

    /// Update resource usage
    pub fn update_resources(
        &mut self,
        pid: u32,
        memory: u64,
        cpu_time: u64,
        open_files: u32,
    ) -> Result<(), ProcessError> {
        if let Some(process) = self.processes.get_mut(&pid) {
            // Check limits
            if memory > process.resource_limits.max_memory {
                return Err(ProcessError::ResourceLimitExceeded);
            }
            if open_files > process.resource_limits.max_open_files {
                return Err(ProcessError::ResourceLimitExceeded);
            }

            process.memory_used = memory;
            process.cpu_time_ms = cpu_time;
            process.open_files = open_files;
            Ok(())
        } else {
            Err(ProcessError::ProcessNotFound)
        }
    }

    /// Get runnable processes (for scheduler)
    pub fn get_runnable(&self) -> Vec<ProcessInfo> {
        self.processes
            .values()
            .filter(|p| p.state == ProcessState::Ready || p.state == ProcessState::Running)
            .cloned()
            .collect()
    }

    /// Get process count
    pub fn process_count(&self) -> usize {
        self.processes.len()
    }
}

impl Default for ProcessManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_manager_creation() {
        let pm = ProcessManager::new();
        assert_eq!(pm.process_count(), 0);
        assert_eq!(pm.next_pid, 1);
    }

    #[test]
    fn test_init_process() {
        let mut pm = ProcessManager::new();
        let pid = pm.init().unwrap();
        assert_eq!(pid, 1);
        assert_eq!(pm.process_count(), 1);
        assert_eq!(pm.init_pid, 1);
    }

    #[test]
    fn test_fork_process() {
        let mut pm = ProcessManager::new();
        pm.init().unwrap();

        let child_pid = pm.fork(1).unwrap();
        assert_eq!(child_pid, 2);
        assert_eq!(pm.process_count(), 2);

        let child = pm.get_process(child_pid).unwrap();
        assert_eq!(child.ppid, 1);
    }

    #[test]
    fn test_exit_process() {
        let mut pm = ProcessManager::new();
        pm.init().unwrap();
        let child_pid = pm.fork(1).unwrap();

        pm.exit(child_pid, 42).unwrap();
        let child = pm.get_process(child_pid).unwrap();
        assert_eq!(child.state, ProcessState::Terminated);
        assert_eq!(child.exit_status.unwrap().code, 42);
    }

    #[test]
    fn test_resource_limits() {
        let mut pm = ProcessManager::new();
        pm.init().unwrap();

        let result = pm.update_resources(1, 1024 * 1024 * 1024, 0, 0);
        assert!(result.is_err()); // Exceeds default memory limit
    }

    #[test]
    fn test_priority_change() {
        let mut pm = ProcessManager::new();
        pm.init().unwrap();

        pm.set_priority(1, Priority::High).unwrap();
        let process = pm.get_process(1).unwrap();
        assert_eq!(process.priority, Priority::High);
    }
}
