// SPDX-License-Identifier: MIT
// SigmaOS Sovereign Process Subsystem (`src/process/pidfd_procdesc_subreaper.rs`)
// Linux pidfd, FreeBSD Capsicum procdesc, and Subreaper Process Tree Re-parenting Engine

use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;

/// File descriptor representing a process in Linux (pidfd) or FreeBSD Capsicum (procdesc)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProcessFileDescriptor(pub i32);

/// Process Descriptor capabilities (FreeBSD Capsicum style)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProcessDescriptorRights {
    pub can_kill: bool,
    pub can_wait: bool,
    pub can_getfd: bool,
    pub can_read_status: bool,
}

impl Default for ProcessDescriptorRights {
    fn default() -> Self {
        Self {
            can_kill: true,
            can_wait: true,
            can_getfd: true,
            can_read_status: true,
        }
    }
}

/// Process state entry tracked in the Subreaper process hierarchy
#[derive(Debug, Clone)]
pub struct SubreaperProcessEntry {
    pub pid: u32,
    pub ppid: u32,
    pub is_subreaper: bool,
    pub is_zombie: bool,
    pub exit_code: i32,
    pub name: String,
    pub open_fds: BTreeMap<i32, String>,
}

/// Sovereign Linux Pidfd & FreeBSD Procdesc Subreaper Engine
pub struct SovereignPidfdProcdescEngine {
    next_fd: i32,
    pidfd_map: BTreeMap<ProcessFileDescriptor, u32>,
    procdesc_rights: BTreeMap<ProcessFileDescriptor, ProcessDescriptorRights>,
    process_tree: BTreeMap<u32, SubreaperProcessEntry>,
}

impl SovereignPidfdProcdescEngine {
    pub fn new() -> Self {
        let mut tree = BTreeMap::new();
        // Register init process (PID 1) as default root subreaper
        tree.insert(
            1,
            SubreaperProcessEntry {
                pid: 1,
                ppid: 0,
                is_subreaper: true,
                is_zombie: false,
                exit_code: 0,
                name: String::from("init"),
                open_fds: BTreeMap::new(),
            },
        );

        Self {
            next_fd: 100,
            pidfd_map: BTreeMap::new(),
            procdesc_rights: BTreeMap::new(),
            process_tree: tree,
        }
    }

    /// Register a new process in the hierarchy
    pub fn register_process(&mut self, pid: u32, ppid: u32, name: &str) {
        let mut open_fds = BTreeMap::new();
        open_fds.insert(0, String::from("/dev/null"));
        open_fds.insert(1, String::from("/dev/console"));
        open_fds.insert(2, String::from("/dev/console"));

        self.process_tree.insert(
            pid,
            SubreaperProcessEntry {
                pid,
                ppid,
                is_subreaper: false,
                is_zombie: false,
                exit_code: 0,
                name: String::from(name),
                open_fds,
            },
        );
    }

    /// Linux `pidfd_open`: Open a file descriptor referring to a process by PID
    pub fn pidfd_open(&mut self, pid: u32, flags: u32) -> Result<ProcessFileDescriptor, &'static str> {
        let _ = flags;
        if !self.process_tree.contains_key(&pid) {
            return Err("ESRCH: Process not found");
        }

        let pfd = ProcessFileDescriptor(self.next_fd);
        self.next_fd += 1;

        self.pidfd_map.insert(pfd, pid);
        self.procdesc_rights.insert(pfd, ProcessDescriptorRights::default());

        Ok(pfd)
    }

    /// FreeBSD Capsicum `pdfork`: Fork a process and obtain a process descriptor
    pub fn pdfork(&mut self, parent_pid: u32, child_pid: u32, child_name: &str) -> Result<(ProcessFileDescriptor, u32), &'static str> {
        self.register_process(child_pid, parent_pid, child_name);
        let pfd = self.pidfd_open(child_pid, 0)?;
        Ok((pfd, child_pid))
    }

    /// Send signal via process file descriptor (`pidfd_send_signal` / `pdkill`)
    pub fn send_signal(&self, pfd: ProcessFileDescriptor, signal: i32) -> Result<String, &'static str> {
        let pid = self.pidfd_map.get(&pfd).ok_or("EBADF: Invalid process descriptor")?;
        let rights = self.procdesc_rights.get(&pfd).ok_or("EBADF: Rights not found")?;

        if !rights.can_kill {
            return Err("EPERM: Process descriptor lacks can_kill capability");
        }

        let entry = self.process_tree.get(pid).ok_or("ESRCH: Target process exited")?;
        Ok(format!("Signalled process '{}' (PID {}) with signal {}", entry.name, pid, signal))
    }

    /// Obtain duplicate file descriptor from target process (`pidfd_getfd`)
    pub fn pidfd_getfd(&self, pfd: ProcessFileDescriptor, target_fd: i32) -> Result<String, &'static str> {
        let pid = self.pidfd_map.get(&pfd).ok_or("EBADF: Invalid process descriptor")?;
        let rights = self.procdesc_rights.get(&pfd).ok_or("EBADF: Rights not found")?;

        if !rights.can_getfd {
            return Err("EPERM: Process descriptor lacks can_getfd capability");
        }

        let entry = self.process_tree.get(pid).ok_or("ESRCH: Target process not found")?;
        let fd_path = entry.open_fds.get(&target_fd).ok_or("EBADF: Target FD not open in process")?;

        Ok(format!("Duplicated FD {} ({}) from PID {}", target_fd, fd_path, pid))
    }

    /// Set process as a Child Subreaper (`PR_SET_CHILD_SUBREAPER` / `PROC_REAP_ACQUIRE`)
    pub fn set_subreaper(&mut self, pid: u32, enable: bool) -> Result<bool, &'static str> {
        let entry = self.process_tree.get_mut(&pid).ok_or("ESRCH: Process not found")?;
        entry.is_subreaper = enable;
        Ok(enable)
    }

    /// Re-parent orphan child processes to the nearest ancestor Subreaper
    pub fn terminate_and_reparent_orphans(&mut self, pid: u32, exit_code: i32) -> Result<Vec<u32>, &'static str> {
        let ppid = {
            let entry = self.process_tree.get_mut(&pid).ok_or("ESRCH: Process not found")?;
            entry.is_zombie = true;
            entry.exit_code = exit_code;
            entry.ppid
        };

        // Find nearest ancestor subreaper
        let mut subreaper_pid = 1; // Default fallback init
        let mut curr = ppid;

        while curr != 0 {
            if let Some(parent) = self.process_tree.get(&curr) {
                if parent.is_subreaper {
                    subreaper_pid = parent.pid;
                    break;
                }
                curr = parent.ppid;
            } else {
                break;
            }
        }

        // Re-parent all children of dying process to the found subreaper
        let mut reparented = Vec::new();
        for entry in self.process_tree.values_mut() {
            if entry.ppid == pid {
                entry.ppid = subreaper_pid;
                reparented.push(entry.pid);
            }
        }

        Ok(reparented)
    }
}

impl Default for SovereignPidfdProcdescEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pidfd_procdesc_subreaper_flow() {
        let mut engine = SovereignPidfdProcdescEngine::new();

        // Register Subreaper supervisor process (PID 10) and child worker (PID 20)
        engine.register_process(10, 1, "supervisor");
        assert!(engine.set_subreaper(10, true).unwrap());

        engine.register_process(20, 10, "worker_worker");

        // Spawn child via pdfork (PID 30)
        let (pfd_child, child_pid) = engine.pdfork(20, 30, "orphan_task").unwrap();
        assert_eq!(child_pid, 30);

        // Signal via pidfd
        let sig_res = engine.send_signal(pfd_child, 9).unwrap();
        assert!(sig_res.contains("orphan_task"));

        // Getfd via pidfd
        let fd_res = engine.pidfd_getfd(pfd_child, 1).unwrap();
        assert!(fd_res.contains("/dev/console"));

        // Terminate PID 20 and verify PID 30 is re-parented to Subreaper PID 10 (not init PID 1)
        let reparented = engine.terminate_and_reparent_orphans(20, 0).unwrap();
        assert_eq!(reparented, vec![30]);
    }
}
