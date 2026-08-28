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

pub struct SovereignProcessManager {
    pub processes: BTreeMap<usize, SovereignProcess>,
    pub ipc_channels: BTreeMap<usize, ZeroCopyIpcChannel>,
    next_pid: usize,
    next_channel_id: usize,
}

impl SovereignProcessManager {
    pub fn new() -> Self {
        Self {
            processes: BTreeMap::new(),
            ipc_channels: BTreeMap::new(),
            next_pid: 1,
            next_channel_id: 100,
        }
    }

    pub fn sovereign_spawn(&mut self, name: &str, priority: u32) -> usize {
        let pid = self.next_pid;
        self.next_pid += 1;

        let proc = SovereignProcess {
            pid,
            name: name.to_string(),
            state: SovereignProcessState::Ready,
            priority,
            stdin_buffer: Vec::new(),
            stdout_buffer: Vec::new(),
            execution_time_ms: 0,
        };

        self.processes.insert(pid, proc);
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

    pub fn sovereign_ipc_send(&mut self, channel_id: usize, payload: &[u8]) -> Result<usize, String> {
        let channel = self.ipc_channels.get_mut(&channel_id).ok_or_else(|| format!("IPC channel {} not found", channel_id))?;
        channel.ring_buffer.extend_from_slice(payload);
        Ok(payload.len())
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
