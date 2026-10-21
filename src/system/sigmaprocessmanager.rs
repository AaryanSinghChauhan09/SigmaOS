// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Aaryan Singh Chauhan

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessPriority {
    Low,
    Normal,
    High,
    Realtime,
}

/// Process information
#[derive(Debug, Clone)]
pub struct Process {
    pub pid: u32,
    pub name: String,
    pub command: String,
    pub state: ProcessState,
    pub priority: ProcessPriority,
    pub parent_pid: Option<u32>,
    pub memory_usage: u64,
    pub cpu_usage: f32,
}

/// Process manager for managing system processes
/// Inspired by Linux process management and Omarchy monitoring
#[derive(Debug, Clone)]
pub struct SigmaProcessManager {
    processes: Vec<Process>,
    next_pid: u32,
}

impl SigmaProcessManager {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            next_pid: 1,
        }
    }

    /// Spawn a new process
    pub fn spawn_process(&mut self, name: String, command: String) -> u32 {
        let pid = self.next_pid;
        self.next_pid += 1;
        let process = Process {
            pid,
            name,
            command,
            state: ProcessState::Running,
            priority: ProcessPriority::Normal,
            parent_pid: None,
            memory_usage: 0,
            cpu_usage: 0.0,
        };
        self.processes.push(process);
        pid
    }

    /// Get process by PID
    pub fn get_process(&self, pid: u32) -> Option<&Process> {
        self.processes.iter().find(|p| p.pid == pid)
    }

    /// Get all processes
    pub fn get_processes(&self) -> &[Process] {
        &self.processes
    }

    /// Kill a process
    pub fn kill_process(&mut self, pid: u32) -> Result<String, String> {
        if let Some(process) = self.processes.iter_mut().find(|p| p.pid == pid) {
            process.state = ProcessState::Dead;
            Ok(format!("Killed process: {} (PID: {})", process.name, pid))
        } else {
            Err(format!("Process not found: {}", pid))
        }
    }

    /// Stop a process
    pub fn stop_process(&mut self, pid: u32) -> Result<String, String> {
        if let Some(process) = self.processes.iter_mut().find(|p| p.pid == pid) {
            process.state = ProcessState::Stopped;
            Ok(format!("Stopped process: {} (PID: {})", process.name, pid))
        } else {
            Err(format!("Process not found: {}", pid))
        }
    }

    /// Resume a stopped process
    pub fn resume_process(&mut self, pid: u32) -> Result<String, String> {
        if let Some(process) = self.processes.iter_mut().find(|p| p.pid == pid) {
            process.state = ProcessState::Running;
            Ok(format!("Resumed process: {} (PID: {})", process.name, pid))
        } else {
            Err(format!("Process not found: {}", pid))
        }
    }

    /// Set process priority
    pub fn set_priority(&mut self, pid: u32, priority: ProcessPriority) -> Result<String, String> {
        if let Some(process) = self.processes.iter_mut().find(|p| p.pid == pid) {
            process.priority = priority;
            Ok(format!("Set priority for process: {} (PID: {})", process.name, pid))
        } else {
            Err(format!("Process not found: {}", pid))
        }
    }

    /// Get running processes
    pub fn get_running_processes(&self) -> Vec<&Process> {
        self.processes
            .iter()
            .filter(|p| p.state == ProcessState::Running)
            .collect()
    }

    /// Get processes by name
    pub fn get_processes_by_name(&self, name: &str) -> Vec<&Process> {
        self.processes
            .iter()
            .filter(|p| p.name == name)
            .collect()
    }

    /// Get child processes
    pub fn get_child_processes(&self, parent_pid: u32) -> Vec<&Process> {
        self.processes
            .iter()
            .filter(|p| p.parent_pid == Some(parent_pid))
            .collect()
    }

    /// Get process count
    pub fn get_process_count(&self) -> usize {
        self.processes.len()
    }

    /// Clean up zombie processes
    pub fn cleanup_zombies(&mut self) -> usize {
        let initial_count = self.processes.len();
        self.processes.retain(|p| p.state != ProcessState::Zombie);
        initial_count - self.processes.len()
    }
}

impl Default for SigmaProcessManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_process() {
        let mut manager = SigmaProcessManager::new();
        let pid = manager.spawn_process("test".to_string(), "/bin/test".to_string());
        assert_eq!(pid, 1);
        assert_eq!(manager.get_process_count(), 1);
    }

    #[test]
    fn test_kill_process() {
        let mut manager = SigmaProcessManager::new();
        let pid = manager.spawn_process("test".to_string(), "/bin/test".to_string());
        manager.kill_process(pid).unwrap();
        assert_eq!(
            manager.get_process(pid).unwrap().state,
            ProcessState::Dead
        );
    }

    #[test]
    fn test_stop_process() {
        let mut manager = SigmaProcessManager::new();
        let pid = manager.spawn_process("test".to_string(), "/bin/test".to_string());
        manager.stop_process(pid).unwrap();
        assert_eq!(
            manager.get_process(pid).unwrap().state,
            ProcessState::Stopped
        );
    }

    #[test]
    fn test_resume_process() {
        let mut manager = SigmaProcessManager::new();
        let pid = manager.spawn_process("test".to_string(), "/bin/test".to_string());
        manager.stop_process(pid).unwrap();
        manager.resume_process(pid).unwrap();
        assert_eq!(
            manager.get_process(pid).unwrap().state,
            ProcessState::Running
        );
    }

    #[test]
    fn test_set_priority() {
        let mut manager = SigmaProcessManager::new();
        let pid = manager.spawn_process("test".to_string(), "/bin/test".to_string());
        manager.set_priority(pid, ProcessPriority::High).unwrap();
        assert_eq!(
            manager.get_process(pid).unwrap().priority,
            ProcessPriority::High
        );
    }

    #[test]
    fn test_get_running_processes() {
        let mut manager = SigmaProcessManager::new();
        let pid1 = manager.spawn_process("test1".to_string(), "/bin/test1".to_string());
        let pid2 = manager.spawn_process("test2".to_string(), "/bin/test2".to_string());
        manager.stop_process(pid2).unwrap();
        assert_eq!(manager.get_running_processes().len(), 1);
    }

    #[test]
    fn test_cleanup_zombies() {
        let mut manager = SigmaProcessManager::new();
        let pid = manager.spawn_process("test".to_string(), "/bin/test".to_string());
        manager.processes[0].state = ProcessState::Zombie;
        let cleaned = manager.cleanup_zombies();
        assert_eq!(cleaned, 1);
        assert_eq!(manager.get_process_count(), 0);
    }
}
