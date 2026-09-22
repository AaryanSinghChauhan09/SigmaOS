// Linux-inspired procfs virtual filesystem
// Process and system information interface for SigmaOS

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Process status information
#[derive(Debug, Clone)]
pub struct ProcessStatus {
    pub pid: u32,
    pub name: String,
    pub state: String,
    pub ppid: u32,
    pub uid: u32,
    pub gid: u32,
    pub threads: u32,
    pub cmdline: String,
}

impl ProcessStatus {
    pub fn new(pid: u32, name: String) -> Self {
        ProcessStatus {
            pid,
            name: name.clone(),
            state: "R".to_string(), // Running
            ppid: 1,
            uid: 1000,
            gid: 1000,
            threads: 1,
            cmdline: name,
        }
    }
}

/// Memory information
#[derive(Debug, Clone)]
pub struct MemInfo {
    pub total: u64,
    pub free: u64,
    pub available: u64,
    pub buffers: u64,
    pub cached: u64,
    pub swap_total: u64,
    pub swap_free: u64,
}

impl MemInfo {
    pub fn new() -> Self {
        MemInfo {
            total: 8 * 1024 * 1024 * 1024, // 8 GB
            free: 4 * 1024 * 1024 * 1024,  // 4 GB
            available: 4 * 1024 * 1024 * 1024,
            buffers: 256 * 1024 * 1024,
            cached: 512 * 1024 * 1024,
            swap_total: 2 * 1024 * 1024 * 1024,
            swap_free: 2 * 1024 * 1024 * 1024,
        }
    }
}

impl Default for MemInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// CPU statistics
#[derive(Debug, Clone)]
pub struct CpuStats {
    pub user: u64,
    pub nice: u64,
    pub system: u64,
    pub idle: u64,
    pub iowait: u64,
    pub irq: u64,
    pub softirq: u64,
}

impl CpuStats {
    pub fn new() -> Self {
        CpuStats {
            user: 100000,
            nice: 0,
            system: 50000,
            idle: 900000,
            iowait: 10000,
            irq: 5000,
            softirq: 10000,
        }
    }
}

impl Default for CpuStats {
    fn default() -> Self {
        Self::new()
    }
}

/// procfs entry type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcEntryType {
    Directory,
    File,
    Symlink,
}

/// procfs entry
#[derive(Debug, Clone)]
pub struct ProcEntry {
    pub name: String,
    pub entry_type: ProcEntryType,
    pub value: String,
    pub children: Vec<String>,
}

impl ProcEntry {
    pub fn new(name: String, entry_type: ProcEntryType, value: String) -> Self {
        ProcEntry {
            name,
            entry_type,
            value,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: String) {
        self.children.push(child);
    }
}

/// procfs instance
pub struct Procfs {
    entries: HashMap<String, Arc<Mutex<ProcEntry>>>,
    processes: HashMap<u32, ProcessStatus>,
    mem_info: MemInfo,
    cpu_stats: CpuStats,
}

impl Procfs {
    pub fn new() -> Self {
        let mut procfs = Procfs {
            entries: HashMap::new(),
            processes: HashMap::new(),
            mem_info: MemInfo::new(),
            cpu_stats: CpuStats::new(),
        };

        procfs.create_standard_structure();
        procfs
    }

    fn create_standard_structure(&mut self) {
        // /proc
        self.create_entry("/proc".to_string(), ProcEntryType::Directory, "".to_string());

        // /proc/cpuinfo
        self.create_entry("/proc/cpuinfo".to_string(), ProcEntryType::File, self.generate_cpuinfo());

        // /proc/meminfo
        self.create_entry("/proc/meminfo".to_string(), ProcEntryType::File, self.generate_meminfo());

        // /proc/stat
        self.create_entry("/proc/stat".to_string(), ProcEntryType::File, self.generate_stat());

        // /proc/version
        self.create_entry("/proc/version".to_string(), ProcEntryType::File, "SigmaOS version 1.0.0".to_string());

        // /proc/uptime
        self.create_entry("/proc/uptime".to_string(), ProcEntryType::File, "1000.0 5000.0".to_string());

        // /proc/loadavg
        self.create_entry("/proc/loadavg".to_string(), ProcEntryType::File, "0.50 0.45 0.40 1/100 1234".to_string());

        // /proc/self
        self.create_entry("/proc/self".to_string(), ProcEntryType::Symlink, "1".to_string());

        // /proc/1 (init process)
        self.create_process_entry(1, "init".to_string());
    }

    fn generate_cpuinfo(&self) -> String {
        "processor\t: 0\nvendor_id\t: SigmaOS\ncpu family\t: 6\nmodel\t\t: 1\nmodel name\t: SigmaOS CPU v1\n".to_string()
    }

    fn generate_meminfo(&self) -> String {
        format!(
            "MemTotal:\t{}\nMemFree:\t{}\nMemAvailable:\t{}\nBuffers:\t{}\nCached:\t{}\nSwapTotal:\t{}\nSwapFree:\t{}\n",
            self.mem_info.total / 1024,
            self.mem_info.free / 1024,
            self.mem_info.available / 1024,
            self.mem_info.buffers / 1024,
            self.mem_info.cached / 1024,
            self.mem_info.swap_total / 1024,
            self.mem_info.swap_free / 1024
        )
    }

    fn generate_stat(&self) -> String {
        format!(
            "cpu  {} {} {} {} {} {} {}\n",
            self.cpu_stats.user,
            self.cpu_stats.nice,
            self.cpu_stats.system,
            self.cpu_stats.idle,
            self.cpu_stats.iowait,
            self.cpu_stats.irq,
            self.cpu_stats.softirq
        )
    }

    pub fn create_entry(&mut self, path: String, entry_type: ProcEntryType, value: String) -> Arc<Mutex<ProcEntry>> {
        let entry = Arc::new(Mutex::new(ProcEntry::new(
            path.split('/').last().unwrap_or(&path).to_string(),
            entry_type,
            value,
        )));

        self.entries.insert(path.clone(), entry.clone());

        // Add to parent directory
        if let Some(parent_path) = Self::parent_path(&path) {
            if let Some(parent) = self.entries.get_mut(&parent_path) {
                let mut parent_guard = parent.lock().unwrap();
                parent_guard.add_child(path.clone());
            }
        }

        entry
    }

    pub fn create_process_entry(&mut self, pid: u32, name: String) {
        let process_dir = format!("/proc/{}", pid);
        self.create_entry(process_dir.clone(), ProcEntryType::Directory, "".to_string());

        // /proc/{pid}/status
        let status = ProcessStatus::new(pid, name.clone());
        self.processes.insert(pid, status.clone());
        self.create_entry(format!("{}/status", process_dir), ProcEntryType::File, self.generate_status(&status));

        // /proc/{pid}/cmdline
        self.create_entry(format!("{}/cmdline", process_dir), ProcEntryType::File, status.cmdline.clone());

        // /proc/{pid}/exe
        self.create_entry(format!("{}/exe", process_dir), ProcEntryType::Symlink, format!("/bin/{}", name));
    }

    fn generate_status(&self, status: &ProcessStatus) -> String {
        format!(
            "Name:\t{}\nState:\t{}\nPid:\t{}\nPPid:\t{}\nUid:\t{}\nGid:\t{}\nThreads:\t{}\n",
            status.name, status.state, status.pid, status.ppid, status.uid, status.gid, status.threads
        )
    }

    pub fn get_entry(&self, path: &str) -> Option<Arc<Mutex<ProcEntry>>> {
        self.entries.get(path).cloned()
    }

    pub fn read_entry(&self, path: &str) -> Result<String, String> {
        let entry = self.entries.get(path)
            .ok_or_else(|| format!("Entry not found: {}", path))?;

        let entry_guard = entry.lock().unwrap();
        if entry_guard.entry_type != ProcEntryType::File {
            return Err("Not a file".to_string());
        }

        Ok(entry_guard.value.clone())
    }

    pub fn list_directory(&self, path: &str) -> Result<Vec<String>, String> {
        let entry = self.entries.get(path)
            .ok_or_else(|| format!("Entry not found: {}", path))?;

        let entry_guard = entry.lock().unwrap();
        if entry_guard.entry_type != ProcEntryType::Directory {
            return Err("Not a directory".to_string());
        }

        Ok(entry_guard.children.clone())
    }

    fn parent_path(path: &str) -> Option<String> {
        if path == "/proc" || !path.contains('/') {
            return None;
        }

        let last_slash = path.rfind('/');
        if let Some(pos) = last_slash {
            if pos == 0 {
                Some("/".to_string())
            } else {
                Some(path[..pos].to_string())
            }
        } else {
            None
        }
    }

    pub fn get_process(&self, pid: u32) -> Option<&ProcessStatus> {
        self.processes.get(&pid)
    }

    pub fn add_process(&mut self, pid: u32, name: String) {
        self.create_process_entry(pid, name);
    }

    pub fn update_mem_info(&mut self, mem_info: MemInfo) {
        self.mem_info = mem_info;
        self.update_meminfo_entry();
    }

    fn update_meminfo_entry(&mut self) {
        let new_value = self.generate_meminfo();
        if let Some(entry) = self.entries.get_mut("/proc/meminfo") {
            let mut entry_guard = entry.lock().unwrap();
            entry_guard.value = new_value;
        }
    }

    pub fn update_cpu_stats(&mut self, cpu_stats: CpuStats) {
        self.cpu_stats = cpu_stats;
        self.update_stat_entry();
    }

    fn update_stat_entry(&mut self) {
        let new_value = self.generate_stat();
        if let Some(entry) = self.entries.get_mut("/proc/stat") {
            let mut entry_guard = entry.lock().unwrap();
            entry_guard.value = new_value;
        }
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}

impl Default for Procfs {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_procfs_creation() {
        let procfs = Procfs::new();
        assert!(procfs.entry_count() > 0);
    }

    #[test]
    fn test_procfs_read_cpuinfo() {
        let procfs = Procfs::new();
        let cpuinfo = procfs.read_entry("/proc/cpuinfo").unwrap();
        assert!(cpuinfo.contains("SigmaOS CPU"));
    }

    #[test]
    fn test_procfs_read_meminfo() {
        let procfs = Procfs::new();
        let meminfo = procfs.read_entry("/proc/meminfo").unwrap();
        assert!(meminfo.contains("MemTotal"));
    }

    #[test]
    fn test_procfs_read_stat() {
        let procfs = Procfs::new();
        let stat = procfs.read_entry("/proc/stat").unwrap();
        assert!(stat.contains("cpu"));
    }

    #[test]
    fn test_procfs_list_proc() {
        let procfs = Procfs::new();
        let children = procfs.list_directory("/proc").unwrap();
        assert!(children.len() > 0);
    }

    #[test]
    fn test_procfs_create_process() {
        let mut procfs = Procfs::new();
        procfs.add_process(100, "test".to_string());

        let process = procfs.get_process(100);
        assert!(process.is_some());
        assert_eq!(process.unwrap().name, "test");
    }

    #[test]
    fn test_process_status() {
        let status = ProcessStatus::new(1, "init".to_string());
        assert_eq!(status.pid, 1);
        assert_eq!(status.name, "init");
    }

    #[test]
    fn test_mem_info() {
        let mem_info = MemInfo::new();
        assert!(mem_info.total > 0);
        assert!(mem_info.free > 0);
    }

    #[test]
    fn test_cpu_stats() {
        let cpu_stats = CpuStats::new();
        assert!(cpu_stats.idle > 0);
    }

    #[test]
    fn test_procfs_update_mem_info() {
        let mut procfs = Procfs::new();
        let mut new_mem = MemInfo::new();
        new_mem.total = 16 * 1024 * 1024 * 1024;

        procfs.update_mem_info(new_mem);

        let meminfo = procfs.read_entry("/proc/meminfo").unwrap();
        assert!(meminfo.contains("16777216")); // 16 GB in KB
    }
}
