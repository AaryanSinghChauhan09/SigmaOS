extern crate alloc;
// SigmaOS Advanced Process Control Subsystem
// Inspired by Linux and BSD distribution process management paradigms:
// - Cross-process memory inspection (Linux process_vm_readv / process_vm_writev)
// - Job control lifecycle & core dump abort isolation (BSD job control / POSIX daemonization)
// - Process waiting with flags (WNOHANG, WUNTRACED, WCONTINUED) & BSD rusage accounting
// - Cancellation tokens (pthread_cancel) & Zombie process reaping/reparenting
// - Advanced IPC Hub (POSIX Message Queue, eventfd counters, sigqueue rich signal dispatch)

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::vec;
use alloc::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessControlError {
    ProcessNotFound(usize),
    PermissionDenied,
    InvalidAddress(u64),
    BufferTooSmall,
    AlreadyTerminated,
    Interrupted,
    WouldBlock,
    IpcChannelClosed,
}

/// 1. Cross-Process Memory Read/Write Engine (process_vm_readv / process_vm_writev)
pub struct ProcessVmReadWriteEngine {
    // Simulated process address spaces (PID -> (BaseVirtualAddr -> ByteData))
    pub address_spaces: BTreeMap<usize, BTreeMap<u64, Vec<u8>>>,
}

impl ProcessVmReadWriteEngine {
    pub fn new() -> Self {
        Self {
            address_spaces: BTreeMap::new(),
        }
    }

    pub fn register_process_memory(&mut self, pid: usize, base_addr: u64, data: Vec<u8>) {
        self.address_spaces.entry(pid).or_insert_with(BTreeMap::new).insert(base_addr, data);
    }

    pub fn process_vm_readv(
        &self,
        pid: usize,
        remote_addr: u64,
        len: usize,
    ) -> Result<Vec<u8>, ProcessControlError> {
        let space = self.address_spaces.get(&pid).ok_or(ProcessControlError::ProcessNotFound(pid))?;
        for (&base, block) in space {
            let block_len = block.len() as u64;
            if remote_addr >= base && remote_addr < base + block_len {
                let offset = (remote_addr - base) as usize;
                let end = core::cmp::min(offset + len, block.len());
                return Ok(block[offset..end].to_vec());
            }
        }
        Err(ProcessControlError::InvalidAddress(remote_addr))
    }

    pub fn process_vm_writev(
        &mut self,
        pid: usize,
        remote_addr: u64,
        buffer: &[u8],
    ) -> Result<usize, ProcessControlError> {
        let space = self.address_spaces.get_mut(&pid).ok_or(ProcessControlError::ProcessNotFound(pid))?;
        for (&base, block) in space {
            let block_len = block.len() as u64;
            if remote_addr >= base && remote_addr < base + block_len {
                let offset = (remote_addr - base) as usize;
                let writable_len = core::cmp::min(buffer.len(), block.len() - offset);
                block[offset..offset + writable_len].copy_from_slice(&buffer[..writable_len]);
                return Ok(writable_len);
            }
        }
        Err(ProcessControlError::InvalidAddress(remote_addr))
    }
}

impl Default for ProcessVmReadWriteEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 2. Job Control Lifecycle & Abort/Core-Dump Isolation Engine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobState {
    Running,
    Stopped,
    Background,
    Zombie,
    Aborted,
}

#[derive(Debug, Clone)]
pub struct CoreDumpMetadata {
    pub pid: usize,
    pub fatal_signal: u32,
    pub fault_addr: u64,
    pub registers: Vec<u64>,
    pub memory_dump_size: usize,
}

#[derive(Debug, Clone)]
pub struct ProcessJobEntry {
    pub pid: usize,
    pub pgid: usize,
    pub sid: usize,
    pub is_foreground: bool,
    pub state: JobState,
    pub command_line: String,
}

pub struct JobControlLifecycleEngine {
    pub jobs: BTreeMap<usize, ProcessJobEntry>,
    pub foreground_pgid: usize,
}

impl JobControlLifecycleEngine {
    pub fn new() -> Self {
        Self {
            jobs: BTreeMap::new(),
            foreground_pgid: 1,
        }
    }

    pub fn spawn_job(&mut self, pid: usize, pgid: usize, sid: usize, is_fg: bool, cmd: &str) {
        let entry = ProcessJobEntry {
            pid,
            pgid,
            sid,
            is_foreground: is_fg,
            state: JobState::Running,
            command_line: cmd.to_string(),
        };
        if is_fg {
            self.foreground_pgid = pgid;
        }
        self.jobs.insert(pid, entry);
    }

    pub fn set_foreground_pgid(&mut self, pgid: usize) {
        self.foreground_pgid = pgid;
        for job in self.jobs.values_mut() {
            job.is_foreground = job.pgid == pgid;
            if job.is_foreground && job.state == JobState::Background {
                job.state = JobState::Running;
            }
        }
    }

    pub fn send_to_background(&mut self, pid: usize) -> Result<(), ProcessControlError> {
        let job = self.jobs.get_mut(&pid).ok_or(ProcessControlError::ProcessNotFound(pid))?;
        job.is_foreground = false;
        job.state = JobState::Background;
        Ok(())
    }

    pub fn daemonize(&mut self, pid: usize) -> Result<(), ProcessControlError> {
        let job = self.jobs.get_mut(&pid).ok_or(ProcessControlError::ProcessNotFound(pid))?;
        job.sid = pid;  // setsid()
        job.pgid = pid; // setpgid()
        job.is_foreground = false;
        job.state = JobState::Background;
        Ok(())
    }

    pub fn abort_process(&mut self, pid: usize, signal: u32, fault_addr: u64) -> Result<CoreDumpMetadata, ProcessControlError> {
        let job = self.jobs.get_mut(&pid).ok_or(ProcessControlError::ProcessNotFound(pid))?;
        job.state = JobState::Aborted;

        let core = CoreDumpMetadata {
            pid,
            fatal_signal: signal,
            fault_addr,
            registers: vec![0xDEADBEEF, 0x12345678, fault_addr],
            memory_dump_size: 4096,
        };

        Ok(core)
    }
}

impl Default for JobControlLifecycleEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 3. Advanced Process Waiting & BSD Rusage Collector
pub const WNOHANG: u32 = 0x01;
pub const WUNTRACED: u32 = 0x02;
pub const WCONTINUED: u32 = 0x04;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BsdRusage {
    pub ru_utime_ms: u64,
    pub ru_stime_ms: u64,
    pub ru_maxrss_kb: u64,
    pub ru_minflt: u64,
    pub ru_majflt: u64,
    pub ru_inblock: u64,
    pub ru_oublock: u64,
    pub ru_nvcsw: u64,
    pub ru_nivcsw: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WaitStatus {
    Exited(usize, i32),
    Signaled(usize, u32),
    Stopped(usize, u32),
    Continued(usize),
    StillRunning,
}

pub struct ProcessWaiterAndRusageCollector {
    pub process_rusage: BTreeMap<usize, BsdRusage>,
    pub terminated_queue: BTreeMap<usize, (i32, Option<u32>)>, // pid -> (exit_code, fatal_signal)
}

impl ProcessWaiterAndRusageCollector {
    pub fn new() -> Self {
        Self {
            process_rusage: BTreeMap::new(),
            terminated_queue: BTreeMap::new(),
        }
    }

    pub fn record_rusage(&mut self, pid: usize, rusage: BsdRusage) {
        self.process_rusage.insert(pid, rusage);
    }

    pub fn notify_process_exit(&mut self, pid: usize, exit_code: i32, fatal_sig: Option<u32>) {
        self.terminated_queue.insert(pid, (exit_code, fatal_sig));
    }

    pub fn waitpid(&mut self, pid_arg: i32, flags: u32) -> Result<WaitStatus, ProcessControlError> {
        if pid_arg > 0 {
            let pid = pid_arg as usize;
            if let Some((exit_code, fatal_sig)) = self.terminated_queue.remove(&pid) {
                if let Some(sig) = fatal_sig {
                    return Ok(WaitStatus::Signaled(pid, sig));
                } else {
                    return Ok(WaitStatus::Exited(pid, exit_code));
                }
            } else if (flags & WNOHANG) != 0 {
                return Ok(WaitStatus::StillRunning);
            }
        } else if pid_arg == -1 {
            if let Some((&pid, _)) = self.terminated_queue.iter().next() {
                let (exit_code, fatal_sig) = self.terminated_queue.remove(&pid).unwrap();
                if let Some(sig) = fatal_sig {
                    return Ok(WaitStatus::Signaled(pid, sig));
                } else {
                    return Ok(WaitStatus::Exited(pid, exit_code));
                }
            } else if (flags & WNOHANG) != 0 {
                return Ok(WaitStatus::StillRunning);
            }
        }
        Err(ProcessControlError::WouldBlock)
    }

    pub fn get_rusage(&self, pid: usize) -> Option<BsdRusage> {
        self.process_rusage.get(&pid).copied()
    }
}

impl Default for ProcessWaiterAndRusageCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// 4. Process Cancellation & Termination Manager (pthread_cancel & Zombie Reparenting)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CancellationType {
    Deferred,
    Asynchronous,
    Disabled,
}

#[derive(Debug, Clone)]
pub struct ProcessCancelState {
    pub pid: usize,
    pub cancel_type: CancellationType,
    pub cancel_requested: bool,
}

pub struct ProcessCancellationAndTerminationManager {
    pub process_parents: BTreeMap<usize, usize>, // child_pid -> parent_pid
    pub cancel_states: BTreeMap<usize, ProcessCancelState>,
    pub zombies: Vec<usize>,
}

impl ProcessCancellationAndTerminationManager {
    pub fn new() -> Self {
        let mut parents = BTreeMap::new();
        parents.insert(1, 0); // PID 1 (init) has parent 0
        Self {
            process_parents: parents,
            cancel_states: BTreeMap::new(),
            zombies: Vec::new(),
        }
    }

    pub fn register_process(&mut self, pid: usize, parent_pid: usize, cancel_type: CancellationType) {
        self.process_parents.insert(pid, parent_pid);
        self.cancel_states.insert(
            pid,
            ProcessCancelState {
                pid,
                cancel_type,
                cancel_requested: false,
            },
        );
    }

    pub fn request_cancellation(&mut self, pid: usize) -> Result<(), ProcessControlError> {
        let state = self.cancel_states.get_mut(&pid).ok_or(ProcessControlError::ProcessNotFound(pid))?;
        if state.cancel_type == CancellationType::Disabled {
            return Err(ProcessControlError::PermissionDenied);
        }
        state.cancel_requested = true;
        Ok(())
    }

    pub fn test_cancellation_point(&self, pid: usize) -> bool {
        if let Some(state) = self.cancel_states.get(&pid) {
            state.cancel_requested && state.cancel_type != CancellationType::Disabled
        } else {
            false
        }
    }

    pub fn mark_zombie(&mut self, pid: usize) {
        if !self.zombies.contains(&pid) {
            self.zombies.push(pid);
        }
    }

    pub fn reap_child(&mut self, parent_pid: usize, child_pid: usize) -> Result<bool, ProcessControlError> {
        if self.process_parents.get(&child_pid) == Some(&parent_pid) {
            if let Some(pos) = self.zombies.iter().position(|&z| z == child_pid) {
                self.zombies.remove(pos);
                self.process_parents.remove(&child_pid);
                self.cancel_states.remove(&child_pid);
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn reparent_orphans(&mut self, dying_parent_pid: usize) {
        let children: Vec<usize> = self
            .process_parents
            .iter()
            .filter(|&(_, &p)| p == dying_parent_pid)
            .map(|(&c, _)| c)
            .collect();

        for child in children {
            self.process_parents.insert(child, 1); // Reparent to init (PID 1)
        }
        self.process_parents.remove(&dying_parent_pid);
        self.cancel_states.remove(&dying_parent_pid);
    }
}

impl Default for ProcessCancellationAndTerminationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 5. Advanced Interprocess Communication (IPC) Hub
/// Supports POSIX Message Queue, eventfd counter sync, and sigqueue rich signal payload dispatch.
#[derive(Debug, Clone)]
pub struct PosixMessage {
    pub priority: u32,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct PosixMessageQueue {
    pub name: String,
    pub max_msg: usize,
    pub messages: Vec<PosixMessage>,
}

#[derive(Debug, Clone)]
pub struct EventFd {
    pub counter: u64,
    pub is_semaphore: bool,
}

#[derive(Debug, Clone)]
pub struct SigQueuePayload {
    pub target_pid: usize,
    pub signal_nr: u32,
    pub value_u64: u64,
}

pub struct AdvancedIpcHub {
    pub message_queues: BTreeMap<String, PosixMessageQueue>,
    pub event_fds: BTreeMap<usize, EventFd>,
    pub signal_queues: BTreeMap<usize, Vec<SigQueuePayload>>, // target_pid -> signals
    next_event_fd: usize,
}

// ==========================================
// SOVEREIGN PROCESS LIFECYCLE CONTROLLER
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessLifecycleState {
    Created,
    Running,
    Waiting,
    Background,
    Cancelling,
    Terminated,
    Aborted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaitReason {
    IoWait,
    LockWait,
    SignalWait,
    TimerWait,
}

#[derive(Debug, Clone)]
pub struct ProcessLifecycleRecord {
    pub pid: u64,
    pub name: String,
    pub state: ProcessLifecycleState,
    pub exit_code: Option<i32>,
    pub wait_reason: Option<WaitReason>,
    pub io_buffer: Vec<u8>,
}

/// Comprehensive process lifecycle controller managing read, run, abort, background, write, waiting, cancellation, termination, and IPC
pub struct SovereignProcessLifecycleController {
    pub processes: BTreeMap<u64, ProcessLifecycleRecord>,
}

impl SovereignProcessLifecycleController {
    pub fn new() -> Self {
        Self {
            processes: BTreeMap::new(),
        }
    }

    pub fn register_process(&mut self, pid: u64, name: &str) {
        self.processes.insert(
            pid,
            ProcessLifecycleRecord {
                pid,
                name: name.to_string(),
                state: ProcessLifecycleState::Running,
                exit_code: None,
                wait_reason: None,
                io_buffer: Vec::new(),
            },
        );
    }

    pub fn write_process_buffer(&mut self, pid: u64, data: &[u8]) -> Result<usize, &'static str> {
        let proc_rec = self
            .processes
            .get_mut(&pid)
            .ok_or("Process record not found")?;
        proc_rec.io_buffer.extend_from_slice(data);
        Ok(data.len())
    }

    pub fn read_process_buffer(&mut self, pid: u64, buffer: &mut [u8]) -> Result<usize, &'static str> {
        let proc_rec = self
            .processes
            .get_mut(&pid)
            .ok_or("Process record not found")?;
        let len = buffer.len().min(proc_rec.io_buffer.len());
        if len > 0 {
            let chunk: Vec<u8> = proc_rec.io_buffer.drain(..len).collect();
            buffer[..len].copy_from_slice(&chunk);
            Ok(len)
        } else {
            Ok(0)
        }
    }

    pub fn move_to_background(&mut self, pid: u64) -> Result<(), &'static str> {
        let proc_rec = self
            .processes
            .get_mut(&pid)
            .ok_or("Process record not found")?;
        proc_rec.state = ProcessLifecycleState::Background;
        Ok(())
    }

    pub fn move_to_waiting(&mut self, pid: u64, reason: WaitReason) -> Result<(), &'static str> {
        let proc_rec = self
            .processes
            .get_mut(&pid)
            .ok_or("Process record not found")?;
        proc_rec.state = ProcessLifecycleState::Waiting;
        proc_rec.wait_reason = Some(reason);
        Ok(())
    }

    pub fn cancel_process(&mut self, pid: u64) -> Result<(), &'static str> {
        let proc_rec = self
            .processes
            .get_mut(&pid)
            .ok_or("Process record not found")?;
        proc_rec.state = ProcessLifecycleState::Cancelling;
        Ok(())
    }

    pub fn terminate_process(&mut self, pid: u64, exit_code: i32) -> Result<(), &'static str> {
        let proc_rec = self
            .processes
            .get_mut(&pid)
            .ok_or("Process record not found")?;
        proc_rec.state = ProcessLifecycleState::Terminated;
        proc_rec.exit_code = Some(exit_code);
        Ok(())
    }

    pub fn abort_process(&mut self, pid: u64, signal: i32) -> Result<(), &'static str> {
        let proc_rec = self
            .processes
            .get_mut(&pid)
            .ok_or("Process record not found")?;
        proc_rec.state = ProcessLifecycleState::Aborted;
        proc_rec.exit_code = Some(128 + signal);
        Ok(())
    }

    pub fn get_state(&self, pid: u64) -> Option<ProcessLifecycleState> {
        self.processes.get(&pid).map(|p| p.state)
    }
}

impl Default for SovereignProcessLifecycleController {
    fn default() -> Self {
        Self::new()
    }
}

impl AdvancedIpcHub {
    pub fn new() -> Self {
        Self {
            message_queues: BTreeMap::new(),
            event_fds: BTreeMap::new(),
            signal_queues: BTreeMap::new(),
            next_event_fd: 10,
        }
    }

    // --- POSIX Message Queue ---
    pub fn mq_open(&mut self, name: &str, max_msg: usize) -> String {
        let q_name = name.to_string();
        self.message_queues.entry(q_name.clone()).or_insert_with(|| PosixMessageQueue {
            name: q_name.clone(),
            max_msg,
            messages: Vec::new(),
        });
        q_name
    }

    pub fn mq_send(&mut self, name: &str, payload: &[u8], priority: u32) -> Result<(), ProcessControlError> {
        let mq = self.message_queues.get_mut(name).ok_or(ProcessControlError::IpcChannelClosed)?;
        if mq.messages.len() >= mq.max_msg {
            return Err(ProcessControlError::WouldBlock);
        }

        let msg = PosixMessage {
            priority,
            payload: payload.to_vec(),
        };

        // Insert sorted by priority (higher priority first)
        let pos = mq.messages.iter().position(|m| m.priority < priority).unwrap_or(mq.messages.len());
        mq.messages.insert(pos, msg);
        Ok(())
    }

    pub fn mq_receive(&mut self, name: &str) -> Result<PosixMessage, ProcessControlError> {
        let mq = self.message_queues.get_mut(name).ok_or(ProcessControlError::IpcChannelClosed)?;
        if mq.messages.is_empty() {
            return Err(ProcessControlError::WouldBlock);
        }
        Ok(mq.messages.remove(0))
    }

    // --- EventFd Counter Sync ---
    pub fn eventfd_create(&mut self, init_val: u64, is_semaphore: bool) -> usize {
        let efd = self.next_event_fd;
        self.next_event_fd += 1;

        self.event_fds.insert(efd, EventFd { counter: init_val, is_semaphore });
        efd
    }

    pub fn eventfd_write(&mut self, efd: usize, val: u64) -> Result<(), ProcessControlError> {
        let event_fd = self.event_fds.get_mut(&efd).ok_or(ProcessControlError::IpcChannelClosed)?;
        event_fd.counter += val;
        Ok(())
    }

    pub fn eventfd_read(&mut self, efd: usize) -> Result<u64, ProcessControlError> {
        let event_fd = self.event_fds.get_mut(&efd).ok_or(ProcessControlError::IpcChannelClosed)?;
        if event_fd.counter == 0 {
            return Err(ProcessControlError::WouldBlock);
        }

        if event_fd.is_semaphore {
            event_fd.counter -= 1;
            Ok(1)
        } else {
            let val = event_fd.counter;
            event_fd.counter = 0;
            Ok(val)
        }
    }

    // --- SigQueue Rich Signal Dispatching ---
    pub fn sigqueue(&mut self, target_pid: usize, sig_nr: u32, value: u64) {
        let payload = SigQueuePayload {
            target_pid,
            signal_nr: sig_nr,
            value_u64: value,
        };
        self.signal_queues.entry(target_pid).or_insert_with(Vec::new).push(payload);
    }

    pub fn pop_sigqueue(&mut self, target_pid: usize) -> Option<SigQueuePayload> {
        if let Some(queue) = self.signal_queues.get_mut(&target_pid) {
            if !queue.is_empty() {
                return Some(queue.remove(0));
            }
        }
        None
    }
}

impl Default for AdvancedIpcHub {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_vm_read_write() {
        let mut vm_engine = ProcessVmReadWriteEngine::new();
        vm_engine.register_process_memory(100, 0x10000, vec![0x11, 0x22, 0x33, 0x44, 0x55]);

        let read_data = vm_engine.process_vm_readv(100, 0x10001, 3).unwrap();
        assert_eq!(read_data, vec![0x22, 0x33, 0x44]);

        let written = vm_engine.process_vm_writev(100, 0x10002, &[0xAA, 0xBB]).unwrap();
        assert_eq!(written, 2);

        let updated_data = vm_engine.process_vm_readv(100, 0x10000, 5).unwrap();
        assert_eq!(updated_data, vec![0x11, 0x22, 0xAA, 0xBB, 0x55]);
    }

    #[test]
    fn test_job_control_and_core_dump() {
        let mut job_engine = JobControlLifecycleEngine::new();
        job_engine.spawn_job(10, 10, 10, true, "bash");
        assert_eq!(job_engine.foreground_pgid, 10);

        job_engine.daemonize(10).unwrap();
        let job = job_engine.jobs.get(&10).unwrap();
        assert_eq!(job.state, JobState::Background);
        assert!(!job.is_foreground);

        let core = job_engine.abort_process(10, 11, 0x7FFF0000).unwrap();
        assert_eq!(core.fatal_signal, 11);
        assert_eq!(core.fault_addr, 0x7FFF0000);
        assert_eq!(job_engine.jobs.get(&10).unwrap().state, JobState::Aborted);
    }

    #[test]
    fn test_process_waiter_and_rusage() {
        let mut waiter = ProcessWaiterAndRusageCollector::new();
        let rusage = BsdRusage {
            ru_utime_ms: 120,
            ru_stime_ms: 45,
            ru_maxrss_kb: 4096,
            ..Default::default()
        };
        waiter.record_rusage(20, rusage);
        assert_eq!(waiter.get_rusage(20).unwrap().ru_utime_ms, 120);

        waiter.notify_process_exit(20, 0, None);
        let status = waiter.waitpid(20, 0).unwrap();
        assert_eq!(status, WaitStatus::Exited(20, 0));
    }

    #[test]
    fn test_cancellation_and_reparenting() {
        let mut cancel_mgr = ProcessCancellationAndTerminationManager::new();
        cancel_mgr.register_process(200, 100, CancellationType::Deferred);
        cancel_mgr.register_process(201, 100, CancellationType::Disabled);

        assert!(cancel_mgr.request_cancellation(200).is_ok());
        assert!(cancel_mgr.test_cancellation_point(200));

        assert!(cancel_mgr.request_cancellation(201).is_err()); // disabled

        cancel_mgr.reparent_orphans(100);
        assert_eq!(cancel_mgr.process_parents.get(&200), Some(&1)); // Reparented to init (PID 1)
        assert_eq!(cancel_mgr.process_parents.get(&201), Some(&1));
    }

    #[test]
    fn test_advanced_ipc_hub() {
        let mut ipc = AdvancedIpcHub::new();

        // POSIX Message Queue
        ipc.mq_open("/test_queue", 5);
        ipc.mq_send("/test_queue", b"low_pri", 1).unwrap();
        ipc.mq_send("/test_queue", b"high_pri", 10).unwrap();

        let msg = ipc.mq_receive("/test_queue").unwrap();
        assert_eq!(msg.priority, 10);
        assert_eq!(msg.payload, b"high_pri");

        // EventFd
        let efd = ipc.eventfd_create(0, false);
        ipc.eventfd_write(efd, 5).unwrap();
        assert_eq!(ipc.eventfd_read(efd).unwrap(), 5);

        // SigQueue
        ipc.sigqueue(50, 10, 0x1234);
        let payload = ipc.pop_sigqueue(50).unwrap();
        assert_eq!(payload.signal_nr, 10);
        assert_eq!(payload.value_u64, 0x1234);
    }

    #[test]
    fn test_sovereign_process_lifecycle_controller() {
        let mut plc = SovereignProcessLifecycleController::new();
        plc.register_process(1001, "sovereign_daemon");

        assert_eq!(plc.get_state(1001), Some(ProcessLifecycleState::Running));

        // Read/Write buffer
        assert_eq!(plc.write_process_buffer(1001, b"LIFECYCLE_TEST_DATA").unwrap(), 19);
        let mut read_buf = [0u8; 19];
        assert_eq!(plc.read_process_buffer(1001, &mut read_buf).unwrap(), 19);
        assert_eq!(&read_buf, b"LIFECYCLE_TEST_DATA");

        // Background / Wait / Abort
        assert!(plc.move_to_background(1001).is_ok());
        assert_eq!(plc.get_state(1001), Some(ProcessLifecycleState::Background));

        assert!(plc.move_to_waiting(1001, WaitReason::IoWait).is_ok());
        assert_eq!(plc.get_state(1001), Some(ProcessLifecycleState::Waiting));

        assert!(plc.abort_process(1001, 134).is_ok());
        assert_eq!(plc.get_state(1001), Some(ProcessLifecycleState::Aborted));
    }
}
