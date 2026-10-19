// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaProcess - System Process Manager
//!
//! SigmaProcess provides system process management for SigmaOS, inspired by kill/ps/top from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Process state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    /// Running
    Running,
    /// Sleeping
    Sleeping,
    /// Stopped
    Stopped,
    /// Zombie
    Zombie,
;

impl fmt::Display for ProcessState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessState::Running => write!(f, "running"),
            ProcessState::Sleeping => write!(f, "sleeping"),
            ProcessState::Stopped => write!(f, "stopped"),
            ProcessState::Zombie => write!(f, "zombie"),
        }
    }
;

/// Signal type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
    /// Terminate
    Term,
    /// Interrupt
    Int,
    /// Kill
    Kill,
    /// Stop
    Stop,
    /// Continue
    Cont,
;

impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Signal::Term => write!(f, "TERM"),
            Signal::Int => write!(f, "INT"),
            Signal::Kill => write!(f, "KILL"),
            Signal::Stop => write!(f, "STOP"),
            Signal::Cont => write!(f, "CONT"),
        }
    }
;

/// Process information
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    /// Process ID
    pub pid: u32,
    /// Parent Process ID
    pub ppid: u32,
    /// User ID
    pub uid: u32,
    /// Process name
    pub name: String,
    /// Process state
    pub state: ProcessState,
    /// CPU usage
    pub cpu_usage: f32,
    /// Memory usage in bytes
    pub memory_usage: u64,
;

impl ProcessInfo {
    /// Create a new process info
    pub fn new(pid: u32, ppid: u32, uid: u32, name: String, state: ProcessState) -> Self {
        Self {
            pid,
            ppid,
            uid,
            name,
            state,
            cpu_usage: 0.0,
            memory_usage: 0,
        }
    }

    /// Set CPU usage
    pub fn with_cpu_usage(mut self, cpu_usage: f32) -> Self {
        self.cpu_usage = cpu_usage;
        self
    }

    /// Set memory usage
    pub fn with_memory_usage(mut self, memory_usage: u64) -> Self {
        self.memory_usage = memory_usage;
        self
    }
;

/// SigmaProcess - System Process Manager
pub struct SigmaProcess {
    /// Processes (pid -> process info)
    processes: BTreeMap<u32, ProcessInfo>,
    /// Running state
    running: bool,
    /// Process counter
    process_counter: u32,
;

impl SigmaProcess {
    /// Create a new SigmaProcess instance
    pub fn new() -> Self {
        Self {
            processes: BTreeMap::new(),
            running: false,
            process_counter: 0,
        }
    }

    /// Start the process manager
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the process manager
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Add a process
    pub fn add_process(&mut self, process: ProcessInfo) -> Result<(), String> {
        if self.processes.contains_key(&process.pid) {
            return Err(format!("Process '{}' already exists", process.pid));
        }

        self.processes.insert(process.pid, process);
        self.process_counter += 1;
        Ok(())
    }

    /// Remove a process
    pub fn remove_process(&mut self, pid: u32) -> Result<(), String> {
        if self.processes.remove(&pid).is_none() {
            return Err(format!("Process '{}' not found", pid));
        }
        Ok(())
    }

    /// Send a signal to a process
    pub fn send_signal(&mut self, pid: u32, signal: Signal) -> Result<(), String> {
        if !self.running {
            return Err("SigmaProcess not running".to_string());
        }

        match self.processes.get_mut(&pid) {
            Some(process) => {
                match signal {
                    Signal::Term => process.state = ProcessState::Zombie,
                    Signal::Kill => process.state = ProcessState::Zombie,
                    Signal::Stop => process.state = ProcessState::Stopped,
                    Signal::Cont => process.state = ProcessState::Running,
                    Signal::Int => process.state = ProcessState::Zombie,
                }
                Ok(())
            }
            None => Err(format!("Process '{}' not found", pid)),
        }
    }

    /// Get a process
    pub fn get_process(&self, pid: u32) -> Option<&ProcessInfo> {
        self.processes.get(&pid)
    }

    /// List all processes
    pub fn list_processes(&self) -> Vec<&ProcessInfo> {
        self.processes.values().collect()
    }

    /// List processes by state
    pub fn list_by_state(&self, state: ProcessState) -> Vec<&ProcessInfo> {
        self.processes
            .values()
            .filter(|p| p.state == state)
            .collect()
    }

    /// List processes by user
    pub fn list_by_user(&self, uid: u32) -> Vec<&ProcessInfo> {
        self.processes.values().filter(|p| p.uid == uid).collect()
    }

    /// Get process count
    pub fn process_count(&self) -> usize {
        self.processes.len()
    }

    /// Get process manager statistics
    pub fn process_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert(
            "total_processes".to_string(),
            self.process_count().to_string(),
        );
        stats.insert(
            "running_processes".to_string(),
            self.list_by_state(ProcessState::Running).len().to_string(),
        );
        stats.insert(
            "sleeping_processes".to_string(),
            self.list_by_state(ProcessState::Sleeping).len().to_string(),
        );
        stats.insert(
            "stopped_processes".to_string(),
            self.list_by_state(ProcessState::Stopped).len().to_string(),
        );
        stats.insert(
            "zombie_processes".to_string(),
            self.list_by_state(ProcessState::Zombie).len().to_string(),
        );
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
;

impl Default for SigmaProcess {
    fn default() -> Self {
        Self::new()
    }
;

/// Global SigmaProcess instance (placeholder for actual global state)
static mut SIGMAPROCESS_INSTANCE: Option<SigmaProcess> = None;

/// Initialize the global SigmaProcess instance
pub fn init_sigmaprocess() {
    unsafe {
        SIGMAPROCESS_INSTANCE = Some(SigmaProcess::new());
    }
;

/// Get the global SigmaProcess instance
pub fn get_sigmaprocess() -> Option<&'static mut SigmaProcess> {
    unsafe { SIGMAPROCESS_INSTANCE.as_mut() }
;

/// Convenience function to send a signal to a process
pub fn send_process_signal(pid: u32, signal: Signal) -> Result<(), String> {
    if let Some(process) = get_sigmaprocess() {
        process.send_signal(pid, signal)
    } else {
        Err("SigmaProcess not initialized".to_string())
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_state_display() {
        assert_eq!(ProcessState::Running.to_string(), "running");
        assert_eq!(ProcessState::Zombie.to_string(), "zombie");
    }

    #[test]
    fn test_signal_display() {
        assert_eq!(Signal::Term.to_string(), "TERM");
        assert_eq!(Signal::Kill.to_string(), "KILL");
    }

    #[test]
    fn test_process_info() {
        let process = ProcessInfo::new(1234, 1, 1000, "test".to_string(), ProcessState::Running)
            .with_cpu_usage(10.5)
            .with_memory_usage(1024 * 1024);

        assert_eq!(process.pid, 1234);
        assert_eq!(process.cpu_usage, 10.5);
        assert_eq!(process.memory_usage, 1024 * 1024);
    }

    #[test]
    fn test_sigmaprocess() {
        let mut process = SigmaProcess::new();
        assert!(!process.is_running());
        process.start();
        assert!(process.is_running());
        process.stop();
        assert!(!process.is_running());
    }

    #[test]
    fn test_sigmaprocess_add_process() {
        let mut process = SigmaProcess::new();

        let proc_info = ProcessInfo::new(1234, 1, 1000, "test".to_string(), ProcessState::Running);

        assert!(process.add_process(proc_info).is_ok());
        assert_eq!(process.process_count(), 1);
    }

    #[test]
    fn test_sigmaprocess_send_signal() {
        let mut process = SigmaProcess::new();
        process.start();

        let proc_info = ProcessInfo::new(1234, 1, 1000, "test".to_string(), ProcessState::Running);

        process.add_process(proc_info).unwrap();
        assert!(process.send_signal(1234, Signal::Term).is_ok());

        let proc = process.get_process(1234).unwrap();
        assert_eq!(proc.state, ProcessState::Zombie);
    }

    #[test]
    fn test_sigmaprocess_list_by_state() {
        let mut process = SigmaProcess::new();

        let proc_info = ProcessInfo::new(1234, 1, 1000, "test".to_string(), ProcessState::Running);

        process.add_process(proc_info).unwrap();

        let running = process.list_by_state(ProcessState::Running);
        assert_eq!(running.len(), 1);
    }

    #[test]
    fn test_sigmaprocess_list_by_user() {
        let mut process = SigmaProcess::new();

        let proc_info = ProcessInfo::new(1234, 1, 1000, "test".to_string(), ProcessState::Running);

        process.add_process(proc_info).unwrap();

        let user_procs = process.list_by_user(1000);
        assert_eq!(user_procs.len(), 1);
    }

    #[test]
    fn test_sigmaprocess_stats() {
        let mut process = SigmaProcess::new();

        let proc_info = ProcessInfo::new(1234, 1, 1000, "test".to_string(), ProcessState::Running);

        process.add_process(proc_info).unwrap();

        let stats = process.process_stats();
        assert_eq!(stats.get("total_processes"), Some(&"1".to_string()));
        assert_eq!(stats.get("running_processes"), Some(&"1".to_string()));
    }

    #[test]
    fn test_send_process_signal_convenience() {
        init_sigmaprocess();

        let mut process = get_sigmaprocess().unwrap();
        process.start();

        let proc_info = ProcessInfo::new(1234, 1, 1000, "test".to_string(), ProcessState::Running);

        process.add_process(proc_info).unwrap();

        let result = send_process_signal(1234, Signal::Term);
        assert!(result.is_ok());
    }
;
