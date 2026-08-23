// SigmaOS Advanced Process Control & Lifecycle Management
// Absorbs Linux fork/exec/exit/waitpid, copy-on-write namespaces, BSD rlimits, Windows Priority Classes, and Orphan Re-parenting.

#![no_std]

#[cfg(test)]
extern crate std;

extern crate alloc;

use alloc::string::String;
use alloc::string::ToString;
use alloc::format;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::time::Duration;

#[cfg(not(test))]
use crate::klib::HashMap;

#[cfg(test)]
use std::collections::HashMap;

#[cfg(test)]
mod mock_scheduler {
    use core::time::Duration;
    use alloc::string::String;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Priority {
        Idle = 0,
        Low = 1,
        Normal = 2,
        High = 3,
        Realtime = 4,
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ProcessState {
        Running,
        Ready,
        Blocked,
        Terminated,
    }
    #[derive(Debug, Clone)]
    pub struct Process {
        pub pid: u64,
        pub name: String,
        pub priority: Priority,
        pub state: ProcessState,
        pub runtime: Duration,
        pub time_slice: Duration,
    }
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
}

#[cfg(test)]
use mock_scheduler::{Priority, Process, ProcessState};

#[cfg(not(test))]
use crate::kernel::scheduler::{Priority, Process, ProcessState};

/// Windows-style Process Creation Priority Classes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PriorityClass {
    Idle,
    BelowNormal,
    Normal,
    AboveNormal,
    High,
    Realtime,
}

/// Linux-style Namespace isolation flags (used during clone/fork)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NamespaceFlags {
    pub isolate_pid: bool,
    pub isolate_net: bool,
    pub isolate_mount: bool,
    pub isolate_uts: bool,
}

/// BSD-style rlimits (Process Resource Limits)
#[derive(Debug, Clone, Copy)]
pub struct ResourceLimits {
    pub max_open_files: u32,
    pub max_cpu_time_secs: u64,
    pub max_resident_set_size_bytes: u64,
}

/// Extended metadata tracking Process Contexts from various OS paradigms
#[derive(Debug, Clone)]
pub struct ProcessExtendedContext {
    pub priority_class: PriorityClass,
    pub cpu_affinity_mask: u64, // Bitmask representing active CPU cores
    pub namespaces: NamespaceFlags,
    pub rlimits: ResourceLimits,
    pub open_files_count: u32,
    pub cpu_time_accumulated_secs: u64,
}

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
    extended_contexts: HashMap<u64, ProcessExtendedContext>,
    parent_map: HashMap<u64, u64>, // child -> parent
    exit_codes: HashMap<u64, i32>,
    metrics_map: HashMap<u64, ProcessResourceMetrics>,
    next_pid: AtomicUsize,
}

impl ProcessLifecycleManager {
    pub fn new() -> Self {
        ProcessLifecycleManager {
            processes: HashMap::new(),
            extended_contexts: HashMap::new(),
            parent_map: HashMap::new(),
            exit_codes: HashMap::new(),
            metrics_map: HashMap::new(),
            next_pid: AtomicUsize::new(100),
        }
    }

    /// Linux/BSD-style clone/fork with detailed namespace isolation & rlimits copies
    pub fn fork_ext(&mut self, parent_pid: u64, ns_flags: NamespaceFlags) -> Result<u64, &'static str> {
        let parent = self
            .processes
            .get(&parent_pid)
            .ok_or("Parent process not found")?;

        let child_pid = self.next_pid.fetch_add(1, Ordering::SeqCst) as u64;
        let child_name = alloc::format!("{}_forked", parent.name);

        let mut child = Process::new(child_pid, child_name, parent.priority);
        child.state = ProcessState::Ready;
        child.time_slice = parent.time_slice;

        // Copy parent extended context or create a default one
        let parent_ctx = self.extended_contexts.get(&parent_pid).cloned().unwrap_or(ProcessExtendedContext {
            priority_class: PriorityClass::Normal,
            cpu_affinity_mask: 0b11, // run on core 0 & 1
            namespaces: NamespaceFlags { isolate_pid: false, isolate_net: false, isolate_mount: false, isolate_uts: false },
            rlimits: ResourceLimits { max_open_files: 1024, max_cpu_time_secs: 3600, max_resident_set_size_bytes: 1024 * 1024 * 1024 },
            open_files_count: 0,
            cpu_time_accumulated_secs: 0,
        });

        let child_ctx = ProcessExtendedContext {
            priority_class: parent_ctx.priority_class,
            cpu_affinity_mask: parent_ctx.cpu_affinity_mask,
            namespaces: ns_flags, // apply newly requested isolations
            rlimits: parent_ctx.rlimits,
            open_files_count: 0,
            cpu_time_accumulated_secs: 0,
        };

        self.processes.insert(child_pid, child);
        self.extended_contexts.insert(child_pid, child_ctx);
        self.parent_map.insert(child_pid, parent_pid);
        self.metrics_map.insert(child_pid, ProcessResourceMetrics::default());

        Ok(child_pid)
    }

    pub fn fork(&mut self, parent_pid: u64) -> Result<u64, &'static str> {
        self.fork_ext(parent_pid, NamespaceFlags {
            isolate_pid: false,
            isolate_net: false,
            isolate_mount: false,
            isolate_uts: false,
        })
    }

    pub fn exec(&mut self, pid: u64, new_name: &str) -> Result<(), &'static str> {
        let process = self.processes.get_mut(&pid).ok_or("Process not found")?;
        process.name = new_name.to_string();
        process.runtime = Duration::from_secs(0);
        process.state = ProcessState::Ready;
        Ok(())
    }

    /// Windows-style Priority Class and CPU affinity configurations
    pub fn configure_win32_scheduling(&mut self, pid: u64, priority_class: PriorityClass, cpu_mask: u64) -> Result<(), &'static str> {
        let ctx = self.extended_contexts.get_mut(&pid).ok_or("Process extended context not found")?;
        ctx.priority_class = priority_class;
        ctx.cpu_affinity_mask = cpu_mask;
        Ok(())
    }

    /// BSD-style file open validation against maximum allowed rlimits
    pub fn bsd_open_file(&mut self, pid: u64) -> Result<u32, &'static str> {
        let ctx = self.extended_contexts.get_mut(&pid).ok_or("Process extended context not found")?;
        if ctx.open_files_count >= ctx.rlimits.max_open_files {
            return Err("EMFILE: Maximum open files limit exceeded under BSD rlimits policy");
        }
        ctx.open_files_count += 1;
        Ok(ctx.open_files_count)
    }

    /// Simulate checking process running time against CPU time limit
    pub fn tick_cpu_time(&mut self, pid: u64) -> Result<bool, &'static str> {
        let ctx = self.extended_contexts.get_mut(&pid).ok_or("Process extended context not found")?;
        ctx.cpu_time_accumulated_secs += 1;
        if ctx.cpu_time_accumulated_secs > ctx.rlimits.max_cpu_time_secs {
            return Ok(true); // Signal SIGXCPU or terminate
        }
        Ok(false)
    }

    /// Handles parent termination, re-parenting all orphan processes to Init (PID 1)
    pub fn exit_and_reparent_orphans(&mut self, parent_pid: u64, exit_code: i32) -> Result<(), &'static str> {
        // Exits parent process
        let parent = self.processes.get_mut(&parent_pid).ok_or("Parent process not found")?;
        parent.state = ProcessState::Terminated;
        self.exit_codes.insert(parent_pid, exit_code);

        // Find children and reparent orphans to PID 1 (Init daemon parity)
        let mut orphans = Vec::new();
        for (&child, &parent) in &self.parent_map {
            if parent == parent_pid {
                orphans.push(child);
            }
        }

        for orphan_pid in orphans {
            self.parent_map.insert(orphan_pid, 1); // Re-parented to Init daemon (PID 1)
        }

        Ok(())
    }

    pub fn exit(&mut self, pid: u64, exit_code: i32) -> Result<(), &'static str> {
        self.exit_and_reparent_orphans(pid, exit_code)
    }

    pub fn waitpid(&mut self, child_pid: u64) -> Result<i32, &'static str> {
        let state = self.processes.get(&child_pid).map(|p| p.state);
        match state {
            Some(ProcessState::Terminated) => {
                let code = self.exit_codes.remove(&child_pid).unwrap_or(0);
                self.processes.remove(&child_pid);
                self.parent_map.remove(&child_pid);
                self.extended_contexts.remove(&child_pid);
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

    pub fn get_extended_context(&self, pid: u64) -> Option<&ProcessExtendedContext> {
        self.extended_contexts.get(&pid)
    }

    pub fn get_parent(&self, pid: u64) -> Option<u64> {
        self.parent_map.get(&pid).cloned()
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
    fn test_linux_namespaces_and_bsd_rlimits() {
        let mut manager = ProcessLifecycleManager::new();
        let init = Process::new(1, "init".to_string(), Priority::Normal);
        manager.register_process(init);

        // 1. Isolate Network & Mount namespaces Linux-style
        let flags = NamespaceFlags {
            isolate_pid: false,
            isolate_net: true,
            isolate_mount: true,
            isolate_uts: false,
        };
        let child_pid = manager.fork_ext(1, flags).unwrap();
        let ctx = manager.get_extended_context(child_pid).unwrap();
        assert!(ctx.namespaces.isolate_net);
        assert!(ctx.namespaces.isolate_mount);
        assert!(!ctx.namespaces.isolate_pid);

        // Configure Windows scheduling priority and cpu mask
        manager.configure_win32_scheduling(child_pid, PriorityClass::High, 0b101).unwrap();
        assert_eq!(manager.get_extended_context(child_pid).unwrap().priority_class, PriorityClass::High);
        assert_eq!(manager.get_extended_context(child_pid).unwrap().cpu_affinity_mask, 0b101);

        // 2. Test BSD open file limit (set max open to 2 files)
        manager.extended_contexts.get_mut(&child_pid).unwrap().rlimits.max_open_files = 2;
        assert_eq!(manager.bsd_open_file(child_pid).unwrap(), 1);
        assert_eq!(manager.bsd_open_file(child_pid).unwrap(), 2);
        assert!(manager.bsd_open_file(child_pid).is_err()); // limit exceeded!

        // 3. Test CPU time rlimits
        manager.extended_contexts.get_mut(&child_pid).unwrap().rlimits.max_cpu_time_secs = 2;
        assert!(!manager.tick_cpu_time(child_pid).unwrap());
        assert!(!manager.tick_cpu_time(child_pid).unwrap());
        assert!(manager.tick_cpu_time(child_pid).unwrap()); // exceeded cpu limit
    }

    #[test]
    fn test_orphan_reparenting_to_init() {
        let mut manager = ProcessLifecycleManager::new();
        let init = Process::new(1, "init".to_string(), Priority::Normal);
        let parent = Process::new(10, "parent_service".to_string(), Priority::Normal);

        manager.register_process(init);
        manager.register_process(parent);

        // Create child forked from parent (PID 10)
        let child_pid = manager.fork(10).unwrap();
        assert_eq!(manager.get_parent(child_pid), Some(10));

        // Terminate parent (PID 10) -> child should be re-parented to init (PID 1)
        manager.exit(10, 0).unwrap();
        assert_eq!(manager.get_parent(child_pid), Some(1));
    }
}
