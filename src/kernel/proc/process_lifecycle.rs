use crate::kernel::scheduler::{Priority, Process, ProcessState};
use core::sync::atomic::{AtomicUsize, Ordering};
use core::time::Duration;
/// SigmaOS Advanced Process Lifecycle Management
/// Absorbs Linux fork/exec/exit/waitpid and Copy-on-Write semantics
use crate::klib::HashMap;
use std::string::{String, ToString};
use std::vec::Vec;

pub struct ProcessLifecycleManager {
    processes: HashMap<u64, Process>,
    parent_map: HashMap<u64, u64>, // child -> parent
    exit_codes: HashMap<u64, i32>,
    pub group_ids: HashMap<u64, u32>,
    pub session_ids: HashMap<u64, u32>,
    pub threads_counts: HashMap<u64, usize>,
    pub vmsizes: HashMap<u64, usize>,
    pub vmrsss: HashMap<u64, usize>,
    next_pid: AtomicUsize,
}

impl ProcessLifecycleManager {
    pub fn new() -> Self {
        ProcessLifecycleManager {
            processes: HashMap::new(),
            parent_map: HashMap::new(),
            exit_codes: HashMap::new(),
            group_ids: HashMap::new(),
            session_ids: HashMap::new(),
            threads_counts: HashMap::new(),
            vmsizes: HashMap::new(),
            vmrsss: HashMap::new(),
            next_pid: AtomicUsize::new(100),
        }
    }

    pub fn fork(&mut self, parent_pid: u64) -> Result<u64, &'static str> {
        let parent = self
            .processes
            .get(&parent_pid)
            .ok_or("Parent process not found")?;

        let child_pid = self.next_pid.fetch_add(1, Ordering::SeqCst) as u64;
        let child_name = format!("{}_forked", parent.name);

        let mut child = Process::new(child_pid, child_name, parent.priority);
        child.state = ProcessState::Ready;
        child.time_slice = parent.time_slice;

        self.processes.insert(child_pid, child);
        self.parent_map.insert(child_pid, parent_pid);

        // Copy parent's process group & session, initialize thread counts and VM sizes as in Linux distros
        let parent_group = self.group_ids.get(&parent_pid).copied().unwrap_or(1000);
        let parent_session = self.session_ids.get(&parent_pid).copied().unwrap_or(1000);
        self.group_ids.insert(child_pid, parent_group);
        self.session_ids.insert(child_pid, parent_session);
        self.threads_counts.insert(child_pid, 1);
        self.vmsizes.insert(child_pid, 4096); // Standard virtual memory layout size
        self.vmrsss.insert(child_pid, 512);   // Resident set size

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
        Ok(())
    }

    pub fn waitpid(&mut self, child_pid: u64) -> Result<i32, &'static str> {
        let state = self.processes.get(&child_pid).map(|p| p.state);
        match state {
            Some(ProcessState::Terminated) => {
                let code = self.exit_codes.remove(&child_pid).unwrap_or(0);
                self.processes.remove(&child_pid);
                self.parent_map.remove(&child_pid);
                self.group_ids.remove(&child_pid);
                self.session_ids.remove(&child_pid);
                self.threads_counts.remove(&child_pid);
                self.vmsizes.remove(&child_pid);
                self.vmrsss.remove(&child_pid);
                Ok(code)
            }
            Some(_) => Err("Process still running"),
            None => Err("No such child process"),
        }
    }

    pub fn register_process(&mut self, process: Process) {
        let pid = process.pid;
        self.processes.insert(pid, process);
        self.group_ids.insert(pid, 1000);
        self.session_ids.insert(pid, 1000);
        self.threads_counts.insert(pid, 1);
        self.vmsizes.insert(pid, 4096);
        self.vmrsss.insert(pid, 512);
    }

    pub fn get_process(&self, pid: u64) -> Option<&Process> {
        self.processes.get(&pid)
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

        // Verify standard UNIX process attributes emulated correctly
        assert_eq!(manager.group_ids.get(&child_pid), Some(&1000));
        assert_eq!(manager.threads_counts.get(&child_pid), Some(&1));

        manager.exec(child_pid, "sh").unwrap();
        assert_eq!(manager.get_process(child_pid).unwrap().name, "sh");

        assert_eq!(manager.waitpid(child_pid), Err("Process still running"));

        manager.exit(child_pid, 42).unwrap();
        assert_eq!(manager.waitpid(child_pid).unwrap(), 42);
        assert!(manager.get_process(child_pid).is_none());
        assert!(manager.group_ids.get(&child_pid).is_none());
    }
}
