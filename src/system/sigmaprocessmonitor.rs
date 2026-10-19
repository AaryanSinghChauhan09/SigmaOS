//! SigmaProcessMonitor - Process Monitoring System
//!
//! Provides comprehensive process monitoring inspired by Linux process tools.
//! Supports process discovery, resource tracking, and performance analysis.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Process state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Running,
    Sleeping,
    Stopped,
    Zombie,
    Dead,
    Unknown,
;

/// Process
#[derive(Debug, Clone)]
pub struct Process {
    pub pid: u32,
    pub name: String,
    pub state: ProcessState,
    pub parent_pid: u32,
    pub cpu_percent: f32,
    pub memory_mb: f32,
    pub threads: u32,
    pub start_time: u64,
;

impl Process {
    pub fn new(pid: u32, name: String, parent_pid: u32) -> Self {
        Self {
            pid,
            name,
            state: ProcessState::Running,
            parent_pid,
            cpu_percent: 0.0,
            memory_mb: 0.0,
            threads: 1,
            start_time: 0,
        }
    }

    pub fn with_state(mut self, state: ProcessState) -> Self {
        self.state = state;
        self
    }

    pub fn with_cpu(mut self, cpu_percent: f32) -> Self {
        self.cpu_percent = cpu_percent;
        self
    }

    pub fn with_memory(mut self, memory_mb: f32) -> Self {
        self.memory_mb = memory_mb;
        self
    }

    pub fn with_threads(mut self, threads: u32) -> Self {
        self.threads = threads;
        self
    }
;

/// Process monitor
pub struct SigmaProcessMonitor {
    processes: BTreeMap<u32, Process>,
;

impl SigmaProcessMonitor {
    pub fn new() -> Self {
        Self {
            processes: BTreeMap::new(),
        }
    }

    pub fn add_process(&mut self, process: Process) {
        self.processes.insert(process.pid, process);
    }

    pub fn get_process(&self, pid: u32) -> Option<&Process> {
        self.processes.get(&pid)
    }

    pub fn get_process_mut(&mut self, pid: u32) -> Option<&mut Process> {
        self.processes.get_mut(&pid)
    }

    pub fn list_processes(&self) -> Vec<&Process> {
        self.processes.values().collect()
    }

    pub fn list_processes_by_state(&self, state: ProcessState) -> Vec<&Process> {
        self.processes
            .values()
            .filter(|p| p.state == state)
            .collect()
    }

    pub fn remove_process(&mut self, pid: u32) -> bool {
        self.processes.remove(&pid).is_some()
    }

    pub fn process_count(&self) -> usize {
        self.processes.len()
    }

    pub fn total_cpu_usage(&self) -> f32 {
        self.processes.values().map(|p| p.cpu_percent).sum()
    }

    pub fn total_memory_usage(&self) -> f32 {
        self.processes.values().map(|p| p.memory_mb).sum()
    }

    pub fn find_by_name(&self, name: &str) -> Vec<&Process> {
        self.processes.values().filter(|p| p.name == name).collect()
    }
;

impl Default for SigmaProcessMonitor {
    fn default() -> Self {
        Self::new()
    }
;

static mut SIGMAPROCESSMONITOR_INSTANCE: Option<SigmaProcessMonitor> = None;

pub fn init_sigmaprocessmonitor() {
    unsafe {
        SIGMAPROCESSMONITOR_INSTANCE = Some(SigmaProcessMonitor::new());
    }
;

pub fn get_sigmaprocessmonitor() -> &'static SigmaProcessMonitor {
    unsafe {
        SIGMAPROCESSMONITOR_INSTANCE
            .as_ref()
            .expect("SigmaProcessMonitor not initialized")
    }
;

pub fn get_sigmaprocessmonitor_mut() -> &'static mut SigmaProcessMonitor {
    unsafe {
        SIGMAPROCESSMONITOR_INSTANCE
            .as_mut()
            .expect("SigmaProcessMonitor not initialized")
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_creation() {
        let process = Process::new(1000, "bash".to_string(), 999);
        assert_eq!(process.pid, 1000);
        assert_eq!(process.name, "bash");
        assert_eq!(process.parent_pid, 999);
    }

    #[test]
    fn test_process_with_state() {
        let process = Process::new(1000, "bash".to_string(), 999)
            .with_state(ProcessState::Sleeping)
            .with_cpu(5.5)
            .with_memory(10.0)
            .with_threads(2);
        assert_eq!(process.state, ProcessState::Sleeping);
        assert_eq!(process.cpu_percent, 5.5);
        assert_eq!(process.memory_mb, 10.0);
        assert_eq!(process.threads, 2);
    }

    #[test]
    fn test_process_monitor() {
        let monitor = SigmaProcessMonitor::new();
        assert_eq!(monitor.process_count(), 0);
    }

    #[test]
    fn test_add_process() {
        let mut monitor = SigmaProcessMonitor::new();
        let process = Process::new(1000, "bash".to_string(), 999);
        monitor.add_process(process);
        assert_eq!(monitor.process_count(), 1);
    }

    #[test]
    fn test_list_processes_by_state() {
        let mut monitor = SigmaProcessMonitor::new();
        monitor.add_process(
            Process::new(1000, "bash".to_string(), 999).with_state(ProcessState::Running),
        );
        monitor.add_process(
            Process::new(1001, "vim".to_string(), 1000).with_state(ProcessState::Sleeping),
        );

        let running = monitor.list_processes_by_state(ProcessState::Running);
        assert_eq!(running.len(), 1);
    }

    #[test]
    fn test_find_by_name() {
        let mut monitor = SigmaProcessMonitor::new();
        monitor.add_process(Process::new(1000, "bash".to_string(), 999));
        monitor.add_process(Process::new(1001, "bash".to_string(), 1000));
        monitor.add_process(Process::new(1002, "vim".to_string(), 1000));

        let bash_processes = monitor.find_by_name("bash");
        assert_eq!(bash_processes.len(), 2);
    }

    #[test]
    fn test_total_cpu_usage() {
        let mut monitor = SigmaProcessMonitor::new();
        monitor.add_process(Process::new(1000, "bash".to_string(), 999).with_cpu(5.5));
        monitor.add_process(Process::new(1001, "vim".to_string(), 1000).with_cpu(3.2));

        assert_eq!(monitor.total_cpu_usage(), 8.7);
    }

    #[test]
    fn test_global_instance() {
        init_sigmaprocessmonitor();
        let monitor = get_sigmaprocessmonitor();
        assert_eq!(monitor.process_count(), 0);

        let monitor_mut = get_sigmaprocessmonitor_mut();
        monitor_mut.add_process(Process::new(1000, "bash".to_string(), 999));
        assert_eq!(monitor.process_count(), 1);
    }
;
