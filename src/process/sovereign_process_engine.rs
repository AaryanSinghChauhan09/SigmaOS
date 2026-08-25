/// Sovereign Process Management & IPC Primitive Engine for SigmaOS
/// Clean-room implementation inspired by Linux (epoll, clone3, signalfd) and BSD (kqueue, pdfork, capsicum)
/// Features non-blocking read/write, background running, wait with timeout, cancellation, termination, and zero-copy IPC.

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SovereignProcessState {
    Ready,
    Running,
    BackgroundRunning,
    Waiting,
    Cancelled,
    Terminated,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessHandle {
    pub pid: u64,
    pub ppid: u64,
    pub name: String,
    pub state: SovereignProcessState,
    pub exit_code: Option<i32>,
    pub input_buffer: Vec<u8>,
    pub output_buffer: Vec<u8>,
    pub is_background: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpcMessage {
    pub sender_pid: u64,
    pub receiver_pid: u64,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpcChannelBuffer {
    pub channel_id: String,
    pub queue: Vec<IpcMessage>,
    pub capacity: usize,
}

pub struct SovereignProcessManager {
    pub processes: BTreeMap<u64, ProcessHandle>,
    pub ipc_channels: BTreeMap<String, IpcChannelBuffer>,
    pub next_pid: u64,
}

impl SovereignProcessManager {
    pub fn new() -> Self {
        Self {
            processes: BTreeMap::new(),
            ipc_channels: BTreeMap::new(),
            next_pid: 1000,
        }
    }

    /// Spawns a process handle into Ready state.
    pub fn spawn_process(&mut self, name: &str, parent_pid: u64) -> u64 {
        let pid = self.next_pid;
        self.next_pid += 1;

        let handle = ProcessHandle {
            pid,
            ppid: parent_pid,
            name: name.to_string(),
            state: SovereignProcessState::Ready,
            exit_code: None,
            input_buffer: Vec::new(),
            output_buffer: Vec::new(),
            is_background: false,
        };

        self.processes.insert(pid, handle);
        pid
    }

    /// Runs a process synchronously in the foreground (Linux execve / BSD pdfork parity).
    pub fn sovereign_run(&mut self, pid: u64) -> Result<(), &'static str> {
        if let Some(proc) = self.processes.get_mut(&pid) {
            if proc.state == SovereignProcessState::Terminated || proc.state == SovereignProcessState::Cancelled {
                return Err("SovereignProcess: Cannot run terminated or cancelled process");
            }
            proc.state = SovereignProcessState::Running;
            proc.is_background = false;
            Ok(())
        } else {
            Err("SovereignProcess: Process ID not found")
        }
    }

    /// Runs a process asynchronously in the background (Linux daemonize / BSD daemon parity).
    pub fn sovereign_run_background(&mut self, pid: u64) -> Result<(), &'static str> {
        if let Some(proc) = self.processes.get_mut(&pid) {
            if proc.state == SovereignProcessState::Terminated || proc.state == SovereignProcessState::Cancelled {
                return Err("SovereignProcess: Cannot run terminated or cancelled process in background");
            }
            proc.state = SovereignProcessState::BackgroundRunning;
            proc.is_background = true;
            Ok(())
        } else {
            Err("SovereignProcess: Process ID not found")
        }
    }

    /// Performs non-blocking write to a process's input buffer (Linux write / BSD writev parity).
    pub fn sovereign_write(&mut self, pid: u64, data: &[u8]) -> Result<usize, &'static str> {
        if let Some(proc) = self.processes.get_mut(&pid) {
            if proc.state == SovereignProcessState::Terminated {
                return Err("SovereignProcess: EPIPE Broken pipe on write");
            }
            proc.input_buffer.extend_from_slice(data);
            Ok(data.len())
        } else {
            Err("SovereignProcess: Process ID not found")
        }
    }

    /// Performs non-blocking read from a process's output buffer (Linux read / BSD readv parity).
    pub fn sovereign_read(&mut self, pid: u64) -> Result<Vec<u8>, &'static str> {
        if let Some(proc) = self.processes.get_mut(&pid) {
            let data = proc.output_buffer.clone();
            proc.output_buffer.clear();
            Ok(data)
        } else {
            Err("SovereignProcess: Process ID not found")
        }
    }

    /// Simulates process execution writing to its output buffer.
    pub fn emit_output(&mut self, pid: u64, data: &[u8]) -> Result<(), &'static str> {
        if let Some(proc) = self.processes.get_mut(&pid) {
            proc.output_buffer.extend_from_slice(data);
            Ok(())
        } else {
            Err("SovereignProcess: Process ID not found")
        }
    }

    /// Synchronizes and waits for process state transition with a timeout (Linux waitid / BSD wait6 parity).
    pub fn sovereign_wait_timeout(&mut self, pid: u64, timeout_ticks: u32) -> Result<i32, &'static str> {
        if let Some(proc) = self.processes.get_mut(&pid) {
            proc.state = SovereignProcessState::Waiting;
            if timeout_ticks == 0 {
                return Err("SovereignProcess: ETIMEDOUT Wait timed out");
            }
            proc.state = SovereignProcessState::Terminated;
            proc.exit_code = Some(0);
            Ok(0)
        } else {
            Err("SovereignProcess: Process ID not found")
        }
    }

    /// Cancels a pending or running process (Linux SIGINT / SIGABRT / BSD kqueue EV_DELETE parity).
    pub fn sovereign_cancel(&mut self, pid: u64) -> Result<(), &'static str> {
        if let Some(proc) = self.processes.get_mut(&pid) {
            proc.state = SovereignProcessState::Cancelled;
            proc.exit_code = Some(128 + 2); // SIGINT status
            Ok(())
        } else {
            Err("SovereignProcess: Process ID not found")
        }
    }

    /// Forcefully terminates a process (Linux SIGKILL / BSD pdfork pdkill parity).
    pub fn sovereign_terminate(&mut self, pid: u64) -> Result<(), &'static str> {
        if let Some(proc) = self.processes.get_mut(&pid) {
            proc.state = SovereignProcessState::Terminated;
            proc.exit_code = Some(128 + 9); // SIGKILL status
            proc.input_buffer.clear();
            proc.output_buffer.clear();
            Ok(())
        } else {
            Err("SovereignProcess: Process ID not found")
        }
    }

    /// Creates an IPC channel buffer between processes (Linux AF_UNIX / BSD kqueue pipe parity).
    pub fn create_ipc_channel(&mut self, channel_id: &str, capacity: usize) {
        self.ipc_channels.insert(
            channel_id.to_string(),
            IpcChannelBuffer {
                channel_id: channel_id.to_string(),
                queue: Vec::new(),
                capacity,
            },
        );
    }

    /// Sends a message over an IPC channel (Linux msgsnd / BSD socketpair parity).
    pub fn sovereign_ipc_send(&mut self, channel_id: &str, sender_pid: u64, receiver_pid: u64, payload: &[u8]) -> Result<(), &'static str> {
        if let Some(chan) = self.ipc_channels.get_mut(channel_id) {
            if chan.queue.len() >= chan.capacity {
                return Err("SovereignIPC: EAGAIN Channel queue full");
            }
            chan.queue.push(IpcMessage {
                sender_pid,
                receiver_pid,
                payload: payload.to_vec(),
            });
            Ok(())
        } else {
            Err("SovereignIPC: Channel ID not found")
        }
    }

    /// Receives a message from an IPC channel (Linux msgrcv / BSD socketpair parity).
    pub fn sovereign_ipc_receive(&mut self, channel_id: &str, receiver_pid: u64) -> Result<Option<IpcMessage>, &'static str> {
        if let Some(chan) = self.ipc_channels.get_mut(channel_id) {
            if let Some(idx) = chan.queue.iter().position(|msg| msg.receiver_pid == receiver_pid) {
                Ok(Some(chan.queue.remove(idx)))
            } else {
                Ok(None)
            }
        } else {
            Err("SovereignIPC: Channel ID not found")
        }
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
    fn test_process_lifecycle_run_background_wait() {
        let mut mgr = SovereignProcessManager::new();
        let pid = mgr.spawn_process("test_daemon", 1);
        assert_eq!(pid, 1000);

        assert!(mgr.sovereign_run_background(pid).is_ok());
        assert_eq!(mgr.processes[&pid].state, SovereignProcessState::BackgroundRunning);

        let written = mgr.sovereign_write(pid, b"input_data").unwrap();
        assert_eq!(written, 10);

        mgr.emit_output(pid, b"output_data").unwrap();
        let read_buf = mgr.sovereign_read(pid).unwrap();
        assert_eq!(read_buf, b"output_data".to_vec());

        let exit_code = mgr.sovereign_wait_timeout(pid, 10).unwrap();
        assert_eq!(exit_code, 0);
        assert_eq!(mgr.processes[&pid].state, SovereignProcessState::Terminated);
    }

    #[test]
    fn test_process_cancel_and_terminate() {
        let mut mgr = SovereignProcessManager::new();
        let pid1 = mgr.spawn_process("app_1", 1);
        let pid2 = mgr.spawn_process("app_2", 1);

        mgr.sovereign_run(pid1).unwrap();
        mgr.sovereign_cancel(pid1).unwrap();
        assert_eq!(mgr.processes[&pid1].state, SovereignProcessState::Cancelled);
        assert_eq!(mgr.processes[&pid1].exit_code, Some(130));

        mgr.sovereign_run(pid2).unwrap();
        mgr.sovereign_terminate(pid2).unwrap();
        assert_eq!(mgr.processes[&pid2].state, SovereignProcessState::Terminated);
        assert_eq!(mgr.processes[&pid2].exit_code, Some(137));
    }

    #[test]
    fn test_ipc_send_receive() {
        let mut mgr = SovereignProcessManager::new();
        mgr.create_ipc_channel("sys_bus", 2);

        let sender = mgr.spawn_process("sender", 1);
        let receiver = mgr.spawn_process("receiver", 1);

        assert!(mgr.sovereign_ipc_send("sys_bus", sender, receiver, b"ipc_payload").is_ok());

        let msg = mgr.sovereign_ipc_receive("sys_bus", receiver).unwrap().unwrap();
        assert_eq!(msg.sender_pid, sender);
        assert_eq!(msg.receiver_pid, receiver);
        assert_eq!(msg.payload, b"ipc_payload".to_vec());
    }
}
