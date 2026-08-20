//! System Activity Manager & Process Thread State Engine
//!
//! Provides comprehensive process and thread activity monitoring, interactive priority boosting,
//! background process throttling, context snapshotting, and system activity metrics tracking.

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Activity execution state for processes and threads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActivityState {
    /// Actively executing in foreground with active user interaction.
    Interactive,
    /// Executing in foreground or background without immediate user interaction.
    Active,
    /// Executing in background with lowered priority.
    Background,
    /// Idle, waiting for events or timers.
    Idle,
    /// Throttled to conserve system resources/power.
    Throttled,
    /// Suspended execution (SIGSTOP / SIGTSTP).
    Suspended,
    /// Process/thread terminated and awaiting cleanup.
    Terminated,
}

/// Register snapshot capturing thread execution context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RegisterSnapshot {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub rip: u64,
    pub rflags: u64,
    pub cs: u64,
    pub ss: u64,
    pub ds: u64,
    pub es: u64,
    pub fs: u64,
    pub gs: u64,
    pub cr3: u64,
}

/// Resource usage metrics for an individual process or thread.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ResourceUsageMetrics {
    /// Estimated CPU usage percentage (0.0 to 100.0).
    pub cpu_percent: f32,
    /// Resident Set Size (RSS) in bytes.
    pub memory_rss_bytes: u64,
    /// Virtual memory size in bytes.
    pub memory_vsz_bytes: u64,
    /// Total bytes read from storage/IPC.
    pub io_read_bytes: u64,
    /// Total bytes written to storage/IPC.
    pub io_write_bytes: u64,
    /// Total Context Switches (Voluntary + Involuntary).
    pub context_switches: u64,
}

/// Information tracking an individual thread inside a process.
#[derive(Debug, Clone)]
pub struct ThreadActivityRecord {
    pub tid: u64,
    pub pid: u64,
    pub name: String,
    pub state: ActivityState,
    pub priority: u8,
    pub context: RegisterSnapshot,
    pub cpu_time_ns: u64,
}

/// Information tracking a process activity record.
#[derive(Debug, Clone)]
pub struct ProcessActivityRecord {
    pub pid: u64,
    pub name: String,
    pub command_line: String,
    pub state: ActivityState,
    pub nice: i8,
    pub is_foreground: bool,
    pub metrics: ResourceUsageMetrics,
    pub threads: BTreeMap<u64, ThreadActivityRecord>,
}

/// System Activity Manager managing process states, thread contexts, and throttling policies.
pub struct ProcessActivityManager {
    processes: BTreeMap<u64, ProcessActivityRecord>,
    foreground_pid: Option<u64>,
    cpu_limit_throttled_percent: f32,
}

impl ProcessActivityManager {
    /// Creates a new `ProcessActivityManager`.
    pub fn new() -> Self {
        Self {
            processes: BTreeMap::new(),
            foreground_pid: None,
            cpu_limit_throttled_percent: 10.0,
        }
    }

    /// Registers a new process in the activity manager.
    pub fn register_process(&mut self, pid: u64, name: &str, cmdline: &str) -> Result<(), &'static str> {
        if self.processes.contains_key(&pid) {
            return Err("Process with PID already exists");
        }

        let record = ProcessActivityRecord {
            pid,
            name: name.to_string(),
            command_line: cmdline.to_string(),
            state: ActivityState::Active,
            nice: 0,
            is_foreground: false,
            metrics: ResourceUsageMetrics::default(),
            threads: BTreeMap::new(),
        };

        self.processes.insert(pid, record);
        Ok(())
    }

    /// Registers a main or worker thread under a parent process.
    pub fn register_thread(&mut self, pid: u64, tid: u64, thread_name: &str) -> Result<(), &'static str> {
        let process = self.processes.get_mut(&pid).ok_or("Process not found")?;

        if process.threads.contains_key(&tid) {
            return Err("Thread with TID already registered");
        }

        let thread_record = ThreadActivityRecord {
            tid,
            pid,
            name: thread_name.to_string(),
            state: ActivityState::Active,
            priority: 128,
            context: RegisterSnapshot::default(),
            cpu_time_ns: 0,
        };

        process.threads.insert(tid, thread_record);
        Ok(())
    }

    /// Focuses a process into the foreground, setting it to Interactive and boosting priority.
    pub fn set_foreground_process(&mut self, pid: u64) -> Result<(), &'static str> {
        if !self.processes.contains_key(&pid) {
            return Err("Process not found");
        }

        // Unset previous foreground process
        if let Some(old_fg_pid) = self.foreground_pid {
            if let Some(old_proc) = self.processes.get_mut(&old_fg_pid) {
                old_proc.is_foreground = false;
                if old_proc.state == ActivityState::Interactive {
                    old_proc.state = ActivityState::Active;
                }
            }
        }

        // Set new foreground process
        if let Some(proc) = self.processes.get_mut(&pid) {
            proc.is_foreground = true;
            proc.state = ActivityState::Interactive;
            proc.nice = -5; // Boost interactive process
            for thread in proc.threads.values_mut() {
                thread.state = ActivityState::Interactive;
                thread.priority = 160; // Elevated priority
            }
        }

        self.foreground_pid = Some(pid);
        Ok(())
    }

    /// Updates process activity state and propagates to its threads if desired.
    pub fn update_process_state(&mut self, pid: u64, new_state: ActivityState) -> Result<(), &'static str> {
        let proc = self.processes.get_mut(&pid).ok_or("Process not found")?;
        proc.state = new_state;

        for thread in proc.threads.values_mut() {
            thread.state = new_state;
        }

        Ok(())
    }

    /// Throttles all background processes to conserve system resources.
    pub fn throttle_background_processes(&mut self) -> usize {
        let mut throttled_count = 0;
        for proc in self.processes.values_mut() {
            if !proc.is_foreground && proc.state == ActivityState::Background {
                proc.state = ActivityState::Throttled;
                proc.nice = 15; // Lower CPU priority
                proc.metrics.cpu_percent = proc.metrics.cpu_percent.min(self.cpu_limit_throttled_percent);
                for thread in proc.threads.values_mut() {
                    thread.state = ActivityState::Throttled;
                    thread.priority = 64;
                }
                throttled_count += 1;
            }
        }
        throttled_count
    }

    /// Updates resource metrics for a process.
    pub fn update_metrics(&mut self, pid: u64, metrics: ResourceUsageMetrics) -> Result<(), &'static str> {
        let proc = self.processes.get_mut(&pid).ok_or("Process not found")?;
        proc.metrics = metrics;
        Ok(())
    }

    /// Saves register execution context snapshot for a thread.
    pub fn save_thread_context(&mut self, pid: u64, tid: u64, context: RegisterSnapshot) -> Result<(), &'static str> {
        let proc = self.processes.get_mut(&pid).ok_or("Process not found")?;
        let thread = proc.threads.get_mut(&tid).ok_or("Thread not found")?;
        thread.context = context;
        Ok(())
    }

    /// Retrieves register snapshot for a thread.
    pub fn get_thread_context(&self, pid: u64, tid: u64) -> Result<RegisterSnapshot, &'static str> {
        let proc = self.processes.get(&pid).ok_or("Process not found")?;
        let thread = proc.threads.get(&tid).ok_or("Thread not found")?;
        Ok(thread.context)
    }

    /// Retrieves list of process activity summaries.
    pub fn get_active_processes(&self) -> Vec<&ProcessActivityRecord> {
        self.processes.values().collect()
    }

    /// Terminates and removes a process record.
    pub fn terminate_process(&mut self, pid: u64) -> Result<(), &'static str> {
        if let Some(mut proc) = self.processes.remove(&pid) {
            proc.state = ActivityState::Terminated;
            if self.foreground_pid == Some(pid) {
                self.foreground_pid = None;
            }
            Ok(())
        } else {
            Err("Process not found")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_activity_registration_and_focus() {
        let mut pam = ProcessActivityManager::new();

        pam.register_process(1001, "zenith_wm", "/usr/bin/zenith_wm").unwrap();
        pam.register_process(1002, "chrome", "/usr/bin/chrome").unwrap();

        pam.register_thread(1001, 1001, "main").unwrap();
        pam.register_thread(1002, 1002, "main").unwrap();
        pam.register_thread(1002, 1003, "gpu_worker").unwrap();

        // Focus Chrome
        pam.set_foreground_process(1002).unwrap();
        let proc_1002 = pam.processes.get(&1002).unwrap();
        assert_eq!(proc_1002.state, ActivityState::Interactive);
        assert_eq!(proc_1002.is_foreground, true);
        assert_eq!(proc_1002.nice, -5);

        // Background window manager
        pam.update_process_state(1001, ActivityState::Background).unwrap();
        let throttled = pam.throttle_background_processes();
        assert_eq!(throttled, 1);

        let proc_1001 = pam.processes.get(&1001).unwrap();
        assert_eq!(proc_1001.state, ActivityState::Throttled);
        assert_eq!(proc_1001.nice, 15);
    }

    #[test]
    fn test_thread_register_snapshot() {
        let mut pam = ProcessActivityManager::new();
        pam.register_process(200, "test_app", "test_app").unwrap();
        pam.register_thread(200, 201, "worker").unwrap();

        let ctx = RegisterSnapshot {
            rip: 0x00007FFF00001000,
            rsp: 0x00007FFFFFFFE000,
            rax: 42,
            ..Default::default()
        };

        pam.save_thread_context(200, 201, ctx).unwrap();
        let retrieved = pam.get_thread_context(200, 201).unwrap();
        assert_eq!(retrieved.rip, 0x00007FFF00001000);
        assert_eq!(retrieved.rax, 42);
    }
}
