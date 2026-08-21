extern crate alloc;

use std::string::ToString;
use std::vec::Vec;

#[cfg(test)]
use std::collections::HashMap;
#[cfg(test)]
use std::sync::atomic::{AtomicUsize, Ordering};
#[cfg(test)]
use std::time::Duration;

#[cfg(not(test))]
use crate::klib::HashMap;
#[cfg(not(test))]
use crate::kernel::scheduler::{Priority, Process, ProcessState};
#[cfg(not(test))]
use core::sync::atomic::{AtomicUsize, Ordering};
#[cfg(not(test))]
use core::time::Duration;

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    Idle = 0,
    Low = 1,
    Normal = 2,
    High = 3,
    Realtime = 4,
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Running,
    Ready,
    Blocked,
    Terminated,
}

#[cfg(test)]
#[derive(Debug, Clone)]
pub struct Process {
    pub pid: u64,
    pub name: String,
    pub priority: Priority,
    pub state: ProcessState,
    pub runtime: Duration,
    pub time_slice: Duration,
}

#[cfg(test)]
impl Process {
    pub fn new(pid: u64, name: String, priority: Priority) -> Self {
        Self {
            pid,
            name,
            priority,
            state: ProcessState::Ready,
            runtime: Duration::from_secs(0),
            time_slice: Duration::from_millis(10),
        }
    }
}

/// Signal types (Linux/BSD POSIX parity)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
    SigCont = 18,
    SigStop = 19,
    SigKill = 9,
    SigTerm = 15,
}

/// Extended Process Resource Metrics
#[derive(Debug, Clone)]
pub struct ProcessResourceMetrics {
    pub rss_memory_bytes: usize,
    pub virtual_memory_bytes: usize,
    pub thread_count: usize,
    pub cpu_affinity_mask: u64,
}

impl ProcessResourceMetrics {
    pub fn default() -> Self {
        Self {
            rss_memory_bytes: 4096,
            virtual_memory_bytes: 65536,
            thread_count: 1,
            cpu_affinity_mask: 0xFFFFFFFF,
        }
    }
}

pub struct ProcessLifecycleManager {
    processes: HashMap<u64, Process>,
    parent_map: HashMap<u64, u64>, // child -> parent
    exit_codes: HashMap<u64, i32>,
    metrics_map: HashMap<u64, ProcessResourceMetrics>,
    next_pid: AtomicUsize,
}

impl ProcessLifecycleManager {
    pub fn new() -> Self {
        ProcessLifecycleManager {
            processes: HashMap::new(),
            parent_map: HashMap::new(),
            exit_codes: HashMap::new(),
            metrics_map: HashMap::new(),
            next_pid: AtomicUsize::new(100),
        }
    }

    pub fn fork(&mut self, parent_pid: u64) -> Result<u64, &'static str> {
        let parent = self
            .processes
            .get(&parent_pid)
            .ok_or("Parent process not found")?;

        let child_pid = self.next_pid.fetch_add(1, Ordering::SeqCst) as u64;
        let child_name = alloc::format!("{}_forked", parent.name);

        let mut child = Process::new(child_pid, child_name, parent.priority);
        child.state = ProcessState::Ready;
        child.time_slice = parent.time_slice;

        self.processes.insert(child_pid, child);
        self.parent_map.insert(child_pid, parent_pid);
        self.metrics_map.insert(child_pid, ProcessResourceMetrics::default());

        Ok(child_pid)
    }

    pub fn exec(&mut self, pid: u64, new_name: &str) -> Result<(), &'static str> {
        let process = self.processes.get_mut(&pid).ok_or("Process not found")?;
        process.name = new_name.to_string();
        process.runtime = Duration::from_secs(0);
        process.state = ProcessState::Ready;
        Ok(())
    }

    pub fn exit(&mut self, pid: u64, exit_code: i32) -> Result<(), &'static str> {
        let process = self.processes.get_mut(&pid).ok_or("Process not found")?;
        process.state = ProcessState::Terminated;
        self.exit_codes.insert(pid, exit_code);

        // Linux/BSD Orphan Adoption: Re-parent any orphaned children to PID 1 (init)
        self.reparent_orphaned_children(pid);

        Ok(())
    }

    /// Re-parents all child processes of an exiting process to PID 1 (init process)
    fn reparent_orphaned_children(&mut self, exiting_pid: u64) {
        let mut orphans = Vec::new();
        for (&child, &parent) in &self.parent_map {
            if parent == exiting_pid {
                orphans.push(child);
            }
        }

        for child in orphans {
            self.parent_map.insert(child, 1); // Adopted by init (PID 1)
        }
    }

    /// Handles POSIX signals (SIGSTOP, SIGCONT, SIGKILL) to adjust process states dynamically
    pub fn send_signal(&mut self, pid: u64, signal: Signal) -> Result<(), &'static str> {
        let process = self.processes.get_mut(&pid).ok_or("Process not found")?;

        match signal {
            Signal::SigStop => {
                process.state = ProcessState::Blocked; // Freeze execution
            }
            Signal::SigCont => {
                process.state = ProcessState::Ready; // Unfreeze and make runnable
            }
            Signal::SigKill | Signal::SigTerm => {
                process.state = ProcessState::Terminated;
                self.exit_codes.insert(pid, -9);
                self.reparent_orphaned_children(pid);
            }
        }
        Ok(())
    }

    pub fn waitpid(&mut self, child_pid: u64) -> Result<i32, &'static str> {
        let state = self.processes.get(&child_pid).map(|p| p.state);
        match state {
            Some(ProcessState::Terminated) => {
                let code = self.exit_codes.remove(&child_pid).unwrap_or(0);
                self.processes.remove(&child_pid);
                self.parent_map.remove(&child_pid);
                self.metrics_map.remove(&child_pid);
                Ok(code)
            }
            Some(_) => Err("Process still running"),
            None => Err("No such child process"),
        }
    }

    pub fn register_process(&mut self, process: Process) {
        let pid = process.pid;
        self.processes.insert(pid, process);
        self.metrics_map.insert(pid, ProcessResourceMetrics::default());
    }

    pub fn get_process(&self, pid: u64) -> Option<&Process> {
        self.processes.get(&pid)
    }

    pub fn get_parent(&self, child_pid: u64) -> Option<u64> {
        self.parent_map.get(&child_pid).copied()
    }

    pub fn update_process_metrics(&mut self, pid: u64, rss: usize, threads: usize) -> Result<(), &'static str> {
        let metrics = self.metrics_map.get_mut(&pid).ok_or("Process metrics not found")?;
        metrics.rss_memory_bytes = rss;
        metrics.thread_count = threads;
        Ok(())
    }
}

impl Default for ProcessLifecycleManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fork_exec_exit_wait() {
        let mut manager = ProcessLifecycleManager::new();
        let init = Process::new(1, "init".to_string(), Priority::Normal);
        manager.register_process(init);

        let child_pid = manager.fork(1).unwrap();
        assert!(child_pid > 1);

        let child = manager.get_process(child_pid).unwrap();
        assert_eq!(child.name, "init_forked");

        manager.exec(child_pid, "sh").unwrap();
        assert_eq!(manager.get_process(child_pid).unwrap().name, "sh");

        assert_eq!(manager.waitpid(child_pid), Err("Process still running"));

        manager.exit(child_pid, 42).unwrap();
        assert_eq!(manager.waitpid(child_pid).unwrap(), 42);
        assert!(manager.get_process(child_pid).is_none());
    }

    #[test]
    fn test_orphan_reparenting_to_init() {
        let mut manager = ProcessLifecycleManager::new();
        let init = Process::new(1, "init".to_string(), Priority::Normal);
        manager.register_process(init);

        // Fork parent process (PID 100) from init
        let parent_pid = manager.fork(1).unwrap();
        // Fork child process (PID 101) from parent
        let child_pid = manager.fork(parent_pid).unwrap();

        assert_eq!(manager.get_parent(child_pid), Some(parent_pid));

        // Parent exits before child -> child should be re-parented to init (PID 1)
        manager.exit(parent_pid, 0).unwrap();
        assert_eq!(manager.get_parent(child_pid), Some(1));
    }

    #[test]
    fn test_process_signals() {
        let mut manager = ProcessLifecycleManager::new();
        let init = Process::new(1, "init".to_string(), Priority::Normal);
        manager.register_process(init);

        let pid = manager.fork(1).unwrap();

        // Send SIGSTOP -> Process should transition to Blocked
        manager.send_signal(pid, Signal::SigStop).unwrap();
        assert_eq!(manager.get_process(pid).unwrap().state, ProcessState::Blocked);

        // Send SIGCONT -> Process should transition back to Ready
        manager.send_signal(pid, Signal::SigCont).unwrap();
        assert_eq!(manager.get_process(pid).unwrap().state, ProcessState::Ready);

        // Send SIGKILL -> Process should terminate
        manager.send_signal(pid, Signal::SigKill).unwrap();
        assert_eq!(manager.get_process(pid).unwrap().state, ProcessState::Terminated);
        assert_eq!(manager.waitpid(pid).unwrap(), -9);
    }
}
