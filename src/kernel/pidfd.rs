// Pidfd/Procdesc Process Subsystem with Subreaper Support
// Linux pidfd and FreeBSD Capsicum procdesc integration

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

/// Pidfd - Process file descriptor (Linux-inspired)
#[derive(Debug, Clone)]
pub struct PidFd {
    pub pid: u64,
    pub fd: u64,
    pub capabilities: PidFdCapabilities,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PidFdCapabilities {
    pub can_send_signal: bool,
    pub can_get_fd: bool,
    pub can_wait: bool,
    pub can_read_status: bool,
}

impl PidFdCapabilities {
    pub fn full() -> Self {
        Self {
            can_send_signal: true,
            can_get_fd: true,
            can_wait: true,
            can_read_status: true,
        }
    }

    pub fn restricted() -> Self {
        Self {
            can_send_signal: false,
            can_get_fd: false,
            can_wait: true,
            can_read_status: true,
        }
    }
}

/// Procdesc - Process descriptor (FreeBSD Capsicum-inspired)
#[derive(Debug, Clone)]
pub struct ProcDesc {
    pub pid: u64,
    pub capabilities: ProcDescCapabilities,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProcDescCapabilities {
    pub can_kill: bool,
    pub can_wait: bool,
    pub can_getfd: bool,
    pub can_read_status: bool,
}

impl ProcDescCapabilities {
    pub fn full() -> Self {
        Self {
            can_kill: true,
            can_wait: true,
            can_getfd: true,
            can_read_status: true,
        }
    }

    pub fn restricted() -> Self {
        Self {
            can_kill: false,
            can_wait: true,
            can_getfd: false,
            can_read_status: true,
        }
    }
}

/// Subreaper process entry
#[derive(Debug, Clone)]
pub struct SubreaperEntry {
    pub pid: u64,
    pub is_subreaper: bool,
    pub parent_pid: u64,
}

/// Pidfd/Procdesc process manager
pub struct PidfdProcDescManager {
    next_pid: AtomicU64,
    next_fd: AtomicU64,
    pidfds: HashMap<u64, PidFd>,
    procdescs: HashMap<u64, ProcDesc>,
    subreapers: HashMap<u64, SubreaperEntry>,
    process_tree: HashMap<u64, Vec<u64>>, // pid -> children
}

impl PidfdProcDescManager {
    pub fn new() -> Self {
        Self {
            next_pid: AtomicU64::new(1),
            next_fd: AtomicU64::new(100),
            pidfds: HashMap::new(),
            procdescs: HashMap::new(),
            subreapers: HashMap::new(),
            process_tree: HashMap::new(),
        }
    }

    /// Open a pidfd for a process
    pub fn pidfd_open(&mut self, pid: u64, flags: u32) -> Result<PidFd, &'static str> {
        let capabilities = if flags & 0x01 != 0 {
            PidFdCapabilities::full()
        } else {
            PidFdCapabilities::restricted()
        };

        let fd = self.next_fd.fetch_add(1, Ordering::SeqCst);
        let pidfd = PidFd {
            pid,
            fd,
            capabilities,
        };

        self.pidfds.insert(fd, pidfd.clone());
        Ok(pidfd)
    }

    /// Send signal via pidfd
    pub fn pidfd_send_signal(&self, fd: u64, signal: u32) -> Result<(), &'static str> {
        let pidfd = self.pidfds.get(&fd).ok_or("Pidfd not found")?;
        
        if !pidfd.capabilities.can_send_signal {
            return Err("Insufficient capabilities to send signal");
        }

        // Simulated signal delivery
        Ok(())
    }

    /// Get file descriptor from target process via pidfd
    pub fn pidfd_getfd(&self, fd: u64, target_fd: u32) -> Result<u32, &'static str> {
        let pidfd = self.pidfds.get(&fd).ok_or("Pidfd not found")?;
        
        if !pidfd.capabilities.can_get_fd {
            return Err("Insufficient capabilities to get fd");
        }

        // Simulated fd duplication
        Ok(target_fd)
    }

    /// Fork with procdesc (FreeBSD pdfork-inspired)
    pub fn pdfork(&mut self, parent_pid: u64, capabilities: ProcDescCapabilities) -> Result<(u64, ProcDesc), &'static str> {
        let pid = self.next_pid.fetch_add(1, Ordering::SeqCst);
        
        let procdesc = ProcDesc {
            pid,
            capabilities,
        };

        self.procdescs.insert(pid, procdesc.clone());
        
        // Add to process tree
        self.process_tree.entry(parent_pid).or_insert_with(Vec::new).push(pid);

        Ok((pid, procdesc))
    }

    /// Kill process via procdesc
    pub fn pdkill(&self, pid: u64, signal: u32) -> Result<(), &'static str> {
        let procdesc = self.procdescs.get(&pid).ok_or("Procdesc not found")?;
        
        if !procdesc.capabilities.can_kill {
            return Err("Insufficient capabilities to kill");
        }

        // Simulated signal delivery
        Ok(())
    }

    /// Wait for process via procdesc
    pub fn pdwait(&self, pid: u64) -> Result<u32, &'static str> {
        let procdesc = self.procdescs.get(&pid).ok_or("Procdesc not found")?;
        
        if !procdesc.capabilities.can_wait {
            return Err("Insufficient capabilities to wait");
        }

        // Simulated exit status
        Ok(0)
    }

    /// Set process as subreaper
    pub fn set_subreaper(&mut self, pid: u64) -> Result<(), &'static str> {
        let entry = SubreaperEntry {
            pid,
            is_subreaper: true,
            parent_pid: self.find_parent(pid),
        };
        
        self.subreapers.insert(pid, entry);
        Ok(())
    }

    /// Find parent PID in process tree
    fn find_parent(&self, pid: u64) -> u64 {
        for (parent, children) in &self.process_tree {
            if children.contains(&pid) {
                return *parent;
            }
        }
        0 // No parent found
    }

    /// Reparent orphans to nearest subreaper
    pub fn reparent_orphans(&mut self, orphan_pid: u64) -> Result<u64, &'static str> {
        let current_parent = self.find_parent(orphan_pid);
        
        // Find nearest subreaper ancestor
        let mut subreaper_pid = 0u64;
        let mut temp_pid = current_parent;
        
        while temp_pid != 0 {
            if let Some(entry) = self.subreapers.get(&temp_pid) {
                if entry.is_subreaper {
                    subreaper_pid = temp_pid;
                    break;
                }
            }
            temp_pid = self.find_parent(temp_pid);
        }

        if subreaper_pid == 0 {
            return Err("No subreaper found in ancestor chain");
        }

        // Move orphan to subreaper
        if let Some(children) = self.process_tree.get_mut(&current_parent) {
            children.retain(|p| *p != orphan_pid);
        }
        
        self.process_tree.entry(subreaper_pid).or_insert_with(Vec::new).push(orphan_pid);

        Ok(subreaper_pid)
    }

    /// Get number of pidfds
    pub fn pidfd_count(&self) -> usize {
        self.pidfds.len()
    }

    /// Get number of procdescs
    pub fn procdesc_count(&self) -> usize {
        self.procdescs.len()
    }

    /// Get number of subreapers
    pub fn subreaper_count(&self) -> usize {
        self.subreapers.len()
    }

    /// Get process tree info
    pub fn get_children(&self, pid: u64) -> Vec<u64> {
        self.process_tree.get(&pid).cloned().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pidfd_open() {
        let manager = PidfdProcDescManager::new();
        
        let pidfd = manager.pidfd_open(1, 0x01).unwrap();
        assert_eq!(pidfd.pid, 1);
        assert!(pidfd.capabilities.can_send_signal);
        assert_eq!(manager.pidfd_count(), 1);
    }

    #[test]
    fn test_pidfd_send_signal() {
        let manager = PidfdProcDescManager::new();
        
        let pidfd = manager.pidfd_open(1, 0x01).unwrap();
        assert!(manager.pidfd_send_signal(pidfd.fd, 9).is_ok());
    }

    #[test]
    fn test_pidfd_restricted() {
        let manager = PidfdProcDescManager::new();
        
        let pidfd = manager.pidfd_open(1, 0x00).unwrap();
        assert!(!pidfd.capabilities.can_send_signal);
        assert!(manager.pidfd_send_signal(pidfd.fd, 9).is_err());
    }

    #[test]
    fn test_pdfork() {
        let mut manager = PidfdProcDescManager::new();
        
        let (pid, procdesc) = manager.pdfork(0, ProcDescCapabilities::full()).unwrap();
        assert_eq!(pid, 1);
        assert!(procdesc.capabilities.can_kill);
        assert_eq!(manager.procdesc_count(), 1);
    }

    #[test]
    fn test_pdkill() {
        let mut manager = PidfdProcDescManager::new();
        
        let (pid, _) = manager.pdfork(0, ProcDescCapabilities::full()).unwrap();
        assert!(manager.pdkill(pid, 9).is_ok());
    }

    #[test]
    fn test_subreaper() {
        let mut manager = PidfdProcDescManager::new();
        
        assert!(manager.set_subreaper(1).is_ok());
        assert_eq!(manager.subreaper_count(), 1);
    }

    #[test]
    fn test_reparent_orphans() {
        let mut manager = PidfdProcDescManager::new();
        
        // Create process tree: 0 -> 1 -> 2
        manager.pdfork(0, ProcDescCapabilities::full());
        manager.pdfork(1, ProcDescCapabilities::full());
        
        // Set pid 1 as subreaper
        manager.set_subreaper(1).unwrap();
        
        // Reparent orphan 2 to subreaper 1
        let new_parent = manager.reparent_orphans(2).unwrap();
        assert_eq!(new_parent, 1);
    }

    #[test]
    fn test_process_tree() {
        let mut manager = PidfdProcDescManager::new();
        
        manager.pdfork(0, ProcDescCapabilities::full());
        manager.pdfork(0, ProcDescCapabilities::full());
        
        let children = manager.get_children(0);
        assert_eq!(children.len(), 2);
    }
}
