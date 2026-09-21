// Linux-inspired procfs for process information
// Provides procfs for process introspection

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Process information
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub ppid: u32,
    pub comm: String,
    pub state: String,
    pub utime: u64,
    pub stime: u64,
}

impl ProcessInfo {
    pub fn new(pid: u32, ppid: u32, comm: String) -> Self {
        Self {
            pid,
            ppid,
            comm,
            state: "R".to_string(),
            utime: 0,
            stime: 0,
        }
    }

    /// Set state
    pub fn set_state(&mut self, state: String) {
        self.state = state;
    }

    /// Update CPU time
    pub fn update_cpu_time(&mut self, utime: u64, stime: u64) {
        self.utime = utime;
        self.stime = stime;
    }

    /// Format as /proc/[pid]/stat
    pub fn format_stat(&self) -> String {
        format!("{} ({}) {} 0 0 0 0 0 0 0 0 {} {}",
            self.pid, self.comm, self.state, self.utime, self.stime)
    }

    /// Format as /proc/[pid]/status
    pub fn format_status(&self) -> String {
        format!(
            "Name:\t{}\nState:\t{}\nPid:\t{}\nPPid:\t{}\n",
            self.comm, self.state, self.pid, self.ppid
        )
    }
}

/// Procfs entry
#[derive(Debug, Clone)]
pub enum ProcfsEntry {
    Process(ProcessInfo),
    Directory(Vec<String>),
}

/// Procfs manager for system-wide procfs management
pub struct ProcfsManager {
    pub processes: Arc<Mutex<HashMap<u32, ProcessInfo>>>,
    pub next_pid: Arc<Mutex<u32>>,
}

impl ProcfsManager {
    pub fn new() -> Self {
        let mut processes = HashMap::new();

        // Add init process (PID 1)
        processes.insert(1, ProcessInfo::new(1, 0, "init".to_string()));

        Self {
            processes: Arc::new(Mutex::new(processes)),
            next_pid: Arc::new(Mutex::new(2)),
        }
    }

    /// Create process
    pub fn create_process(&self, ppid: u32, comm: String) -> u32 {
        let mut next_pid = self.next_pid.lock().unwrap();
        let pid = *next_pid;
        *next_pid += 1;
        drop(next_pid);

        let process = ProcessInfo::new(pid, ppid, comm);
        let mut processes = self.processes.lock().unwrap();
        processes.insert(pid, process);

        pid
    }

    /// Get process
    pub fn get_process(&self, pid: u32) -> Option<ProcessInfo> {
        let processes = self.processes.lock().unwrap();
        processes.get(&pid).cloned()
    }

    /// Remove process
    pub fn remove_process(&self, pid: u32) -> Result<(), String> {
        let mut processes = self.processes.lock().unwrap();
        match processes.remove(&pid) {
            Some(_) => Ok(()),
            None => Err(format!("Process {} not found", pid)),
        }
    }

    /// Set process state
    pub fn set_state(&self, pid: u32, state: String) -> Result<(), String> {
        let mut processes = self.processes.lock().unwrap();
        match processes.get_mut(&pid) {
            Some(process) => {
                process.set_state(state);
                Ok(())
            }
            None => Err(format!("Process {} not found", pid)),
        }
    }

    /// Update CPU time
    pub fn update_cpu_time(&self, pid: u32, utime: u64, stime: u64) -> Result<(), String> {
        let mut processes = self.processes.lock().unwrap();
        match processes.get_mut(&pid) {
            Some(process) => {
                process.update_cpu_time(utime, stime);
                Ok(())
            }
            None => Err(format!("Process {} not found", pid)),
        }
    }

    /// Read /proc/[pid]/stat
    pub fn read_stat(&self, pid: u32) -> Result<String, String> {
        let processes = self.processes.lock().unwrap();
        match processes.get(&pid) {
            Some(process) => Ok(process.format_stat()),
            None => Err(format!("Process {} not found", pid)),
        }
    }

    /// Read /proc/[pid]/status
    pub fn read_status(&self, pid: u32) -> Result<String, String> {
        let processes = self.processes.lock().unwrap();
        match processes.get(&pid) {
            Some(process) => Ok(process.format_status()),
            None => Err(format!("Process {} not found", pid)),
        }
    }

    /// List processes
    pub fn list_processes(&self) -> Vec<u32> {
        let processes = self.processes.lock().unwrap();
        processes.keys().cloned().collect()
    }

    /// Get process count
    pub fn process_count(&self) -> usize {
        let processes = self.processes.lock().unwrap();
        processes.len()
    }
}

impl Default for ProcfsManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_info() {
        let info = ProcessInfo::new(1, 0, "init".to_string());
        assert_eq!(info.pid, 1);
        assert_eq!(info.ppid, 0);
        assert_eq!(info.comm, "init");
    }

    #[test]
    fn test_process_info_set_state() {
        let mut info = ProcessInfo::new(1, 0, "init".to_string());
        info.set_state("S".to_string());
        assert_eq!(info.state, "S");
    }

    #[test]
    fn test_process_info_update_cpu_time() {
        let mut info = ProcessInfo::new(1, 0, "init".to_string());
        info.update_cpu_time(100, 50);
        assert_eq!(info.utime, 100);
        assert_eq!(info.stime, 50);
    }

    #[test]
    fn test_process_info_format_stat() {
        let info = ProcessInfo::new(1, 0, "init".to_string());
        let stat = info.format_stat();
        assert!(stat.contains("1"));
        assert!(stat.contains("init"));
    }

    #[test]
    fn test_process_info_format_status() {
        let info = ProcessInfo::new(1, 0, "init".to_string());
        let status = info.format_status();
        assert!(status.contains("Name:"));
        assert!(status.contains("init"));
    }

    #[test]
    fn test_procfs_manager() {
        let manager = ProcfsManager::new();
        assert_eq!(manager.process_count(), 1); // init process
    }

    #[test]
    fn test_procfs_manager_create_process() {
        let manager = ProcfsManager::new();
        let pid = manager.create_process(1, "test".to_string());

        assert_eq!(pid, 2);
        assert_eq!(manager.process_count(), 2);
    }

    #[test]
    fn test_procfs_manager_get_process() {
        let manager = ProcfsManager::new();
        let pid = manager.create_process(1, "test".to_string());

        let process = manager.get_process(pid).unwrap();
        assert_eq!(process.comm, "test");
    }

    #[test]
    fn test_procfs_manager_remove_process() {
        let manager = ProcfsManager::new();
        let pid = manager.create_process(1, "test".to_string());
        manager.remove_process(pid).unwrap();

        assert_eq!(manager.process_count(), 1);
    }

    #[test]
    fn test_procfs_manager_set_state() {
        let manager = ProcfsManager::new();
        let pid = manager.create_process(1, "test".to_string());
        manager.set_state(pid, "S".to_string()).unwrap();

        let process = manager.get_process(pid).unwrap();
        assert_eq!(process.state, "S");
    }

    #[test]
    fn test_procfs_manager_read_stat() {
        let manager = ProcfsManager::new();
        let pid = manager.create_process(1, "test".to_string());

        let stat = manager.read_stat(pid).unwrap();
        assert!(stat.contains("test"));
    }

    #[test]
    fn test_procfs_manager_read_status() {
        let manager = ProcfsManager::new();
        let pid = manager.create_process(1, "test".to_string());

        let status = manager.read_status(pid).unwrap();
        assert!(status.contains("Name:"));
    }

    #[test]
    fn test_procfs_manager_list_processes() {
        let manager = ProcfsManager::new();
        manager.create_process(1, "test".to_string());

        let pids = manager.list_processes();
        assert!(pids.contains(&1));
        assert!(pids.contains(&2));
    }

    #[test]
    fn test_procfs_manager_invalid() {
        let manager = ProcfsManager::new();
        assert!(manager.get_process(999).is_none());
        assert!(manager.set_state(999, "S".to_string()).is_err());
    }
}
