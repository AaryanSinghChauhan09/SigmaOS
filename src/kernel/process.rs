//! # Process Management Subsystem
//!
//! Core process control inspired by Linux task_struct and BSD proc structures.
//! Implements process lifecycle, scheduling state, resource limits, and namespaces.

#![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

/// Process identifier (PID)
pub type Pid = u32;
pub type ProcessId = Pid;

/// Thread group identifier (TGID - same as PID for main thread)
pub type Tgid = u32;

/// User identifier
pub type Uid = u32;

/// Group identifier
pub type Gid = u32;

/// Process state (inspired by Linux TASK_* states)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ProcessState {
    /// Process is running or ready to run
    Running = 0,
    /// Interruptible sleep (waiting for event)
    Sleeping = 1,
    /// Uninterruptible sleep (disk I/O wait)
    Uninterruptible = 2,
    /// Stopped by signal (SIGSTOP, SIGTTIN, etc.)
    Stopped = 3,
    /// Zombie - terminated but not yet reaped
    Zombie = 4,
    /// Dead - being removed
    Dead = 5,
}

/// Process credentials (inspired by Linux cred structure)
#[derive(Debug, Clone, Copy)]
pub struct ProcessCredentials {
    /// Real user ID
    pub uid: Uid,
    /// Effective user ID
    pub euid: Uid,
    /// Saved set-user-ID
    pub suid: Uid,
    /// Filesystem user ID
    pub fsuid: Uid,
    /// Real group ID
    pub gid: Gid,
    /// Effective group ID
    pub egid: Gid,
    /// Saved set-group-ID
    pub sgid: Gid,
    /// Filesystem group ID
    pub fsgid: Gid,
}

impl ProcessCredentials {
    pub const ROOT: Self = Self {
        uid: 0,
        euid: 0,
        suid: 0,
        fsuid: 0,
        gid: 0,
        egid: 0,
        sgid: 0,
        fsgid: 0,
    };

    pub const fn is_root(&self) -> bool {
        self.euid == 0
    }

    pub const fn is_privileged(&self) -> bool {
        self.euid == 0 || self.fsuid == 0
    }
}

/// Resource limits (inspired by Linux rlimit)
#[derive(Debug, Clone, Copy)]
pub struct ResourceLimit {
    pub soft: u64,
    pub hard: u64,
}

impl ResourceLimit {
    pub const UNLIMITED: Self = Self {
        soft: u64::MAX,
        hard: u64::MAX,
    };

    pub const fn new(soft: u64, hard: u64) -> Self {
        Self { soft, hard }
    }
}

/// Resource limit types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ResourceType {
    /// CPU time in seconds
    Cpu = 0,
    /// Maximum file size
    Fsize = 1,
    /// Maximum data segment size
    Data = 2,
    /// Maximum stack size
    Stack = 3,
    /// Maximum core file size
    Core = 4,
    /// Maximum resident set size
    Rss = 5,
    /// Maximum number of processes
    Nproc = 6,
    /// Maximum number of open files
    Nofile = 7,
    /// Maximum locked memory
    Memlock = 8,
    /// Maximum address space
    As = 9,
}

/// Process resource limits table
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    limits: [ResourceLimit; 10],
}

impl ResourceLimits {
    pub fn default() -> Self {
        Self {
            limits: [
                ResourceLimit::UNLIMITED,      // CPU
                ResourceLimit::UNLIMITED,      // FSIZE
                ResourceLimit::UNLIMITED,      // DATA
                ResourceLimit::new(8 * 1024 * 1024, 64 * 1024 * 1024), // STACK (8MB soft, 64MB hard)
                ResourceLimit::UNLIMITED,      // CORE
                ResourceLimit::UNLIMITED,      // RSS
                ResourceLimit::new(4096, 8192), // NPROC
                ResourceLimit::new(1024, 4096), // NOFILE
                ResourceLimit::new(64 * 1024, 64 * 1024), // MEMLOCK
                ResourceLimit::UNLIMITED,      // AS
            ],
        }
    }

    pub fn get(&self, resource: ResourceType) -> ResourceLimit {
        self.limits[resource as usize]
    }

    pub fn set(&mut self, resource: ResourceType, limit: ResourceLimit) {
        self.limits[resource as usize] = limit;
    }
}

/// Process priority and scheduling policy
#[derive(Debug, Clone, Copy)]
pub struct SchedulingInfo {
    /// Static priority (nice value: -20 to 19)
    pub static_priority: i32,
    /// Real-time priority (0-99, 0 = not RT)
    pub rt_priority: u32,
    /// Scheduling policy
    pub policy: SchedulingPolicy,
}

/// Scheduling policies (inspired by Linux SCHED_* constants)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SchedulingPolicy {
    /// Normal time-sharing policy
    Normal = 0,
    /// First-in, first-out real-time policy
    Fifo = 1,
    /// Round-robin real-time policy
    RoundRobin = 2,
    /// Batch scheduling (CPU-intensive)
    Batch = 3,
    /// Idle priority (only when nothing else to run)
    Idle = 5,
    /// Deadline scheduling
    Deadline = 6,
}

/// CPU affinity mask (up to 64 CPUs)
#[derive(Debug, Clone, Copy)]
pub struct CpuAffinity {
    mask: u64,
}

impl CpuAffinity {
    pub const fn all() -> Self {
        Self { mask: u64::MAX }
    }

    pub const fn none() -> Self {
        Self { mask: 0 }
    }

    pub const fn single(cpu: u32) -> Self {
        Self { mask: 1u64 << cpu }
    }

    pub fn set_cpu(&mut self, cpu: u32) {
        self.mask |= 1u64 << cpu;
    }

    pub fn clear_cpu(&mut self, cpu: u32) {
        self.mask &= !(1u64 << cpu);
    }

    pub const fn is_set(&self, cpu: u32) -> bool {
        (self.mask & (1u64 << cpu)) != 0
    }
}

/// Process namespace IDs (inspired by Linux namespaces)
#[derive(Debug, Clone, Copy)]
pub struct NamespaceIds {
    pub mnt_ns: u64,  // Mount namespace
    pub pid_ns: u64,  // PID namespace
    pub net_ns: u64,  // Network namespace
    pub ipc_ns: u64,  // IPC namespace
    pub uts_ns: u64,  // UTS namespace (hostname)
    pub user_ns: u64, // User namespace
    pub cgroup_ns: u64, // Cgroup namespace
}

impl NamespaceIds {
    pub const INIT: Self = Self {
        mnt_ns: 1,
        pid_ns: 1,
        net_ns: 1,
        ipc_ns: 1,
        uts_ns: 1,
        user_ns: 1,
        cgroup_ns: 1,
    };
}

/// Process statistics (inspired by Linux task_struct stats)
#[derive(Debug)]
pub struct ProcessStats {
    pub utime: AtomicU64,      // User CPU time (nanoseconds)
    pub stime: AtomicU64,      // System CPU time (nanoseconds)
    pub voluntary_switches: AtomicU64, // Voluntary context switches
    pub involuntary_switches: AtomicU64, // Involuntary context switches
    pub minor_faults: AtomicU64, // Minor page faults
    pub major_faults: AtomicU64, // Major page faults
    pub rss_pages: AtomicU64,  // Resident set size in pages
}

impl ProcessStats {
    pub const fn new() -> Self {
        Self {
            utime: AtomicU64::new(0),
            stime: AtomicU64::new(0),
            voluntary_switches: AtomicU64::new(0),
            involuntary_switches: AtomicU64::new(0),
            minor_faults: AtomicU64::new(0),
            major_faults: AtomicU64::new(0),
            rss_pages: AtomicU64::new(0),
        }
    }

    pub fn add_user_time(&self, ns: u64) {
        self.utime.fetch_add(ns, Ordering::Relaxed);
    }

    pub fn add_system_time(&self, ns: u64) {
        self.stime.fetch_add(ns, Ordering::Relaxed);
    }

    pub fn inc_voluntary_switch(&self) {
        self.voluntary_switches.fetch_add(1, Ordering::Relaxed);
    }

    pub fn inc_involuntary_switch(&self) {
        self.involuntary_switches.fetch_add(1, Ordering::Relaxed);
    }
}

/// Process control block (inspired by Linux task_struct and BSD proc)
#[derive(Debug)]
pub struct Process {
    pub pid: Pid,
    pub tgid: Tgid,
    pub parent_pid: Pid,
    pub state: ProcessState,
    pub name: String,
    pub credentials: ProcessCredentials,
    pub limits: ResourceLimits,
    pub scheduling: SchedulingInfo,
    pub cpu_affinity: CpuAffinity,
    pub namespaces: NamespaceIds,
    pub stats: ProcessStats,
    pub exit_code: Option<i32>,
}

impl Process {
    pub fn new(pid: Pid, parent_pid: Pid, name: String) -> Self {
        Self {
            pid,
            tgid: pid, // Main thread has TGID = PID
            parent_pid,
            state: ProcessState::Running,
            name,
            credentials: ProcessCredentials::ROOT,
            limits: ResourceLimits::default(),
            scheduling: SchedulingInfo {
                static_priority: 0, // Nice 0
                rt_priority: 0,
                policy: SchedulingPolicy::Normal,
            },
            cpu_affinity: CpuAffinity::all(),
            namespaces: NamespaceIds::INIT,
            stats: ProcessStats::new(),
            exit_code: None,
        }
    }

    pub fn set_state(&mut self, new_state: ProcessState) {
        self.state = new_state;
    }

    pub fn is_alive(&self) -> bool {
        !matches!(self.state, ProcessState::Zombie | ProcessState::Dead)
    }

    pub fn terminate(&mut self, exit_code: i32) {
        self.exit_code = Some(exit_code);
        self.state = ProcessState::Zombie;
    }
}

/// Process table manager
#[derive(Debug)]
pub struct ProcessTable {
    next_pid: AtomicU32,
    processes: Vec<Process>,
}

impl ProcessTable {
    pub const fn new() -> Self {
        Self {
            next_pid: AtomicU32::new(1),
            processes: Vec::new(),
        }
    }

    pub fn allocate_pid(&self) -> Pid {
        self.next_pid.fetch_add(1, Ordering::SeqCst)
    }

    pub fn add_process(&mut self, process: Process) {
        self.processes.push(process);
    }

    pub fn find_process(&self, pid: Pid) -> Option<&Process> {
        self.processes.iter().find(|p| p.pid == pid)
    }

    pub fn find_process_mut(&mut self, pid: Pid) -> Option<&mut Process> {
        self.processes.iter_mut().find(|p| p.pid == pid)
    }

    pub fn remove_process(&mut self, pid: Pid) -> Option<Process> {
        if let Some(pos) = self.processes.iter().position(|p| p.pid == pid) {
            Some(self.processes.remove(pos))
        } else {
            None
        }
    }

    pub fn count_processes(&self) -> usize {
        self.processes.len()
    }

    pub fn reap_zombies(&mut self) -> Vec<(Pid, i32)> {
        let mut reaped = Vec::new();
        self.processes.retain(|p| {
            if p.state == ProcessState::Zombie {
                reaped.push((p.pid, p.exit_code.unwrap_or(-1)));
                false
            } else {
                true
            }
        });
        reaped
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_creation() {
        let proc = Process::new(1, 0, String::from("init"));
        assert_eq!(proc.pid, 1);
        assert_eq!(proc.parent_pid, 0);
        assert_eq!(proc.state, ProcessState::Running);
        assert!(proc.is_alive());
    }

    #[test]
    fn test_process_termination() {
        let mut proc = Process::new(2, 1, String::from("child"));
        proc.terminate(0);
        assert_eq!(proc.state, ProcessState::Zombie);
        assert!(!proc.is_alive());
        assert_eq!(proc.exit_code, Some(0));
    }

    #[test]
    fn test_cpu_affinity() {
        let mut affinity = CpuAffinity::none();
        affinity.set_cpu(0);
        affinity.set_cpu(2);
        assert!(affinity.is_set(0));
        assert!(!affinity.is_set(1));
        assert!(affinity.is_set(2));
    }

    #[test]
    fn test_process_table() {
        let mut table = ProcessTable::new();
        let pid = table.allocate_pid();
        let proc = Process::new(pid, 0, String::from("test"));
        table.add_process(proc);
        assert_eq!(table.count_processes(), 1);
        assert!(table.find_process(pid).is_some());
    }

    #[test]
    fn test_resource_limits() {
        let limits = ResourceLimits::default();
        let stack_limit = limits.get(ResourceType::Stack);
        assert_eq!(stack_limit.soft, 8 * 1024 * 1024);
        assert_eq!(stack_limit.hard, 64 * 1024 * 1024);
    }
}


// ========================================
// Advanced IPC and Process Management Types (Stubs for Phase 1)
// ========================================

/// Advanced IPC Hub for inter-process communication
#[derive(Debug, Clone)]
pub struct AdvancedIpcHub {
    pub channels: alloc::vec::Vec<u64>,
}

/// BSD rusage structure for resource usage statistics
#[derive(Debug, Clone, Copy)]
pub struct BsdRusage {
    pub utime_sec: u64,
    pub utime_usec: u64,
    pub stime_sec: u64,
    pub stime_usec: u64,
    pub maxrss: u64,
}

/// Cancellation type for process operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CancellationType {
    Deferred,
    Asynchronous,
    Disabled,
}

/// Core dump metadata
#[derive(Debug, Clone)]
pub struct CoreDumpMetadata {
    pub pid: Pid,
    pub timestamp: u64,
    pub signal: u32,
}

/// Event file descriptor for signaling
#[derive(Debug, Clone, Copy)]
pub struct EventFd {
    pub fd: u32,
    pub counter: u64,
}

/// Job control lifecycle engine
#[derive(Debug)]
pub struct JobControlLifecycleEngine {
    pub jobs: alloc::vec::Vec<ProcessJobEntry>,
}

/// Job state enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobState {
    Running,
    Stopped,
    Background,
    Foreground,
}

/// POSIX message for message queues
#[derive(Debug, Clone)]
pub struct PosixMessage {
    pub data: alloc::vec::Vec<u8>,
    pub priority: u32,
}

/// POSIX message queue
#[derive(Debug)]
pub struct PosixMessageQueue {
    pub messages: alloc::vec::Vec<PosixMessage>,
    pub max_messages: usize,
}

/// Process cancellation state
#[derive(Debug, Clone, Copy)]
pub struct ProcessCancelState {
    pub cancellation_type: CancellationType,
    pub enabled: bool,
}

/// Process cancellation and termination manager
#[derive(Debug)]
pub struct ProcessCancellationAndTerminationManager {
    pub cancel_states: alloc::vec::Vec<(Pid, ProcessCancelState)>,
}

/// Process control error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessControlError {
    ProcessNotFound,
    PermissionDenied,
    InvalidOperation,
}

/// Process job entry
#[derive(Debug, Clone)]
pub struct ProcessJobEntry {
    pub pid: Pid,
    pub state: JobState,
    pub command: alloc::vec::Vec<u8>,
}

/// Process VM read/write engine
#[derive(Debug)]
pub struct ProcessVmReadWriteEngine {
    pub pid: Pid,
}

/// Process waiter and rusage collector
#[derive(Debug)]
pub struct ProcessWaiterAndRusageCollector {
    pub wait_queue: alloc::vec::Vec<Pid>,
}

/// Signal queue payload
#[derive(Debug, Clone, Copy)]
pub struct SigQueuePayload {
    pub signal: u32,
    pub value: i32,
}

/// Sovereign process (extended process structure)
pub type SovereignProcess = Process;

/// Sovereign process manager
#[derive(Debug)]
pub struct SovereignProcessManager {
    pub process_table: ProcessTable,
}

/// Sovereign process state
pub type SovereignProcessState = ProcessState;

/// Wait status for process waiting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaitStatus {
    Exited(i32),
    Signaled(u32),
    Stopped(u32),
    Continued,
}

/// Zero-copy IPC channel
#[derive(Debug)]
pub struct ZeroCopyIpcChannel {
    pub buffer_addr: u64,
    pub buffer_size: usize,
}

/// Wait options constants
pub const WCONTINUED: u32 = 0x08;
pub const WNOHANG: u32 = 0x01;
pub const WUNTRACED: u32 = 0x02;
