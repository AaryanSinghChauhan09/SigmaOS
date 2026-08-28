// SigmaOS Sovereign Process Management & Advanced IPC Engine
// High-performance process execution, non-blocking stream I/O,
// background process management, timeout waiting, process cancellation/termination,
// and zero-copy IPC channels inspired by Linux and BSD distributions.

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::vec;
use alloc::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SovereignProcessState {
    Ready,
    Running,
    BackgroundRunning,
    Waiting,
    Cancelled,
    Terminated(i32),
}

#[derive(Debug, Clone)]
pub struct SovereignProcess {
    pub pid: usize,
    pub name: String,
    pub state: SovereignProcessState,
    pub priority: u32,
    pub stdin_buffer: Vec<u8>,
    pub stdout_buffer: Vec<u8>,
    pub execution_time_ms: u64,
}

#[derive(Debug, Clone)]
pub struct ZeroCopyIpcChannel {
    pub channel_id: usize,
    pub sender_pid: usize,
    pub receiver_pid: usize,
    pub ring_buffer: Vec<u8>,
}

// ================= Linux & FreeBSD ProcessID Allocation & Recycling Engine =================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PidNamespaceScope {
    pub ns_id: u32,
    pub name: String,
    pub parent_ns_id: Option<u32>,
}

/// FreeBSD & Linux inspired bitmap ProcessID allocator with PID recycling and namespace isolation
pub struct SovereignPidAllocator {
    pub allocated_pids: BTreeMap<u64, u32>, // PID -> ns_id
    pub recycled_pids: Vec<u64>,
    pub max_pid: u64,
    pub last_pid: u64,
}

impl SovereignPidAllocator {
    pub fn new(max_pid: u64) -> Self {
        Self {
            allocated_pids: BTreeMap::new(),
            recycled_pids: Vec::new(),
            max_pid,
            last_pid: 999, // Start PIDs at 1000
        }
    }

    pub fn alloc_pid(&mut self, ns_id: u32) -> Result<u64, &'static str> {
        // Reuse recycled PID if available (FreeBSD pid_alloc recycling parity)
        if let Some(recycled) = self.recycled_pids.pop() {
            self.allocated_pids.insert(recycled, ns_id);
            return Ok(recycled);
        }

        if self.last_pid >= self.max_pid {
            return Err("EAGAIN: Maximum ProcessID limit reached; no free PIDs available");
        }

        self.last_pid += 1;
        let pid = self.last_pid;
        self.allocated_pids.insert(pid, ns_id);
        Ok(pid)
    }

    pub fn free_pid(&mut self, pid: u64) {
        if self.allocated_pids.remove(&pid).is_some() {
            if !self.recycled_pids.contains(&pid) {
                self.recycled_pids.push(pid);
            }
        }
    }

    pub fn is_allocated(&self, pid: u64) -> bool {
        self.allocated_pids.contains_key(&pid)
    }
}

impl Default for SovereignPidAllocator {
    fn default() -> Self {
        Self::new(32768)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessTreeNode {
    pub pid: u64,
    pub ppid: u64,
    pub pgid: u64,
    pub sid: u64,
    pub pdfork_fd: Option<i32>,
    pub children_pids: Vec<u64>,
}

pub struct SovereignProcessManager {
    pub processes: BTreeMap<u64, ProcessHandle>,
    pub process_tree: BTreeMap<u64, ProcessTreeNode>,
    pub ipc_channels: BTreeMap<String, IpcChannelBuffer>,
    pub pid_allocator: SovereignPidAllocator,
}

impl SovereignProcessManager {
    pub fn new() -> Self {
        Self {
            processes: BTreeMap::new(),
            process_tree: BTreeMap::new(),
            ipc_channels: BTreeMap::new(),
            pid_allocator: SovereignPidAllocator::new(32768),
        }
    }

    /// Spawns a process handle into Ready state with process group & session ID hierarchy tracking.
    pub fn spawn_process(&mut self, name: &str, parent_pid: u64) -> u64 {
        let pid = self.pid_allocator.alloc_pid(1).unwrap_or(1000);

        let (pgid, sid) = if parent_pid != 0 {
            if let Some(parent_node) = self.process_tree.get(&parent_pid) {
                (parent_node.pgid, parent_node.sid)
            } else {
                (pid, pid)
            }
        } else {
            (pid, pid)
        };

        let proc = SovereignProcess {
            pid,
            name: name.to_string(),
            state: SovereignProcessState::Ready,
            priority,
            stdin_buffer: Vec::new(),
            stdout_buffer: Vec::new(),
            execution_time_ms: 0,
        };

        let node = ProcessTreeNode {
            pid,
            ppid: parent_pid,
            pgid,
            sid,
            pdfork_fd: Some((pid % 1000) as i32 + 10), // FreeBSD process descriptor
            children_pids: Vec::new(),
        };

        if parent_pid != 0 {
            if let Some(parent_node) = self.process_tree.get_mut(&parent_pid) {
                parent_node.children_pids.push(pid);
            }
        }

        self.process_tree.insert(pid, node);
        self.processes.insert(pid, handle);
        pid
    }

    pub fn sovereign_run_background(&mut self, pid: usize) -> Result<(), String> {
        let proc = self.processes.get_mut(&pid).ok_or_else(|| format!("Process {} not found", pid))?;
        if proc.state == SovereignProcessState::Cancelled || matches!(proc.state, SovereignProcessState::Terminated(_)) {
            return Err(format!("Cannot run background on dead process {}", pid));
        }
        proc.state = SovereignProcessState::BackgroundRunning;
        Ok(())
    }

    pub fn sovereign_write(&mut self, pid: usize, data: &[u8]) -> Result<usize, String> {
        let proc = self.processes.get_mut(&pid).ok_or_else(|| format!("Process {} not found", pid))?;
        proc.stdin_buffer.extend_from_slice(data);
        Ok(data.len())
    }

    pub fn sovereign_read(&mut self, pid: usize, max_len: usize) -> Result<Vec<u8>, String> {
        let proc = self.processes.get_mut(&pid).ok_or_else(|| format!("Process {} not found", pid))?;
        let read_len = core::cmp::min(max_len, proc.stdout_buffer.len());
        let read_bytes = proc.stdout_buffer.drain(..read_len).collect();
        Ok(read_bytes)
    }

    pub fn sovereign_wait_timeout(&mut self, pid: usize, timeout_ms: u64) -> Result<SovereignProcessState, String> {
        let proc = self.processes.get_mut(&pid).ok_or_else(|| format!("Process {} not found", pid))?;
        proc.execution_time_ms += timeout_ms;
        if proc.execution_time_ms >= 100 && proc.state == SovereignProcessState::BackgroundRunning {
            proc.state = SovereignProcessState::Terminated(0);
        }
        Ok(proc.state)
    }

    pub fn sovereign_cancel(&mut self, pid: usize) -> Result<(), String> {
        let proc = self.processes.get_mut(&pid).ok_or_else(|| format!("Process {} not found", pid))?;
        proc.state = SovereignProcessState::Cancelled;
        Ok(())
    }

    pub fn sovereign_terminate(&mut self, pid: usize, exit_code: i32) -> Result<(), String> {
        let proc = self.processes.get_mut(&pid).ok_or_else(|| format!("Process {} not found", pid))?;
        proc.state = SovereignProcessState::Terminated(exit_code);
        Ok(())
    }

    // --- Zero-Copy IPC Channels ---
    pub fn create_ipc_channel(&mut self, sender: usize, receiver: usize) -> usize {
        let channel_id = self.next_channel_id;
        self.next_channel_id += 1;

        let channel = ZeroCopyIpcChannel {
            channel_id,
            sender_pid: sender,
            receiver_pid: receiver,
            ring_buffer: Vec::new(),
        };

        self.ipc_channels.insert(channel_id, channel);
        channel_id
    }

    /// Forcefully terminates a process (Linux SIGKILL / BSD pdfork pdkill parity).
    pub fn sovereign_terminate(&mut self, pid: u64) -> Result<(), &'static str> {
        if let Some(proc) = self.processes.get_mut(&pid) {
            proc.state = SovereignProcessState::Terminated;
            proc.exit_code = Some(128 + 9); // SIGKILL status
            proc.input_buffer.clear();
            proc.output_buffer.clear();
            self.pid_allocator.free_pid(pid);
            Ok(())
        } else {
            Err("SovereignProcess: Process ID not found")
        }
    }

    pub fn sovereign_ipc_receive(&mut self, channel_id: usize) -> Result<Vec<u8>, String> {
        let channel = self.ipc_channels.get_mut(&channel_id).ok_or_else(|| format!("IPC channel {} not found", channel_id))?;
        let data = channel.ring_buffer.clone();
        channel.ring_buffer.clear();
        Ok(data)
    }
}

impl Default for SovereignProcessManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sovereign_process_lifecycle() {
        let mut mgr = SovereignProcessManager::new();
        let pid = mgr.sovereign_spawn("worker_daemon", 10);
        assert_eq!(pid, 1);

        mgr.sovereign_run_background(pid).unwrap();
        assert_eq!(mgr.processes.get(&pid).unwrap().state, SovereignProcessState::BackgroundRunning);

        mgr.sovereign_write(pid, b"input_data").unwrap();
        assert_eq!(mgr.processes.get(&pid).unwrap().stdin_buffer, b"input_data");

        let state = mgr.sovereign_wait_timeout(pid, 150).unwrap();
        assert_eq!(state, SovereignProcessState::Terminated(0));
    }

    #[test]
    fn test_sovereign_ipc_channels() {
        let mut mgr = SovereignProcessManager::new();
        let sender = mgr.sovereign_spawn("sender_proc", 5);
        let receiver = mgr.sovereign_spawn("recv_proc", 5);

        let ch = mgr.create_ipc_channel(sender, receiver);
        mgr.sovereign_ipc_send(ch, b"hello_ipc").unwrap();

        let recv_data = mgr.sovereign_ipc_receive(ch).unwrap();
        assert_eq!(recv_data, b"hello_ipc");
    }

    #[test]
    fn test_sovereign_process_cancellation() {
        let mut mgr = SovereignProcessManager::new();
        let pid = mgr.sovereign_spawn("cancel_target", 1);
        mgr.sovereign_cancel(pid).unwrap();

        assert_eq!(mgr.processes.get(&pid).unwrap().state, SovereignProcessState::Cancelled);
        assert!(mgr.sovereign_run_background(pid).is_err());
    }
}
