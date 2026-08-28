extern crate alloc;
// Sovereign /proc Virtual Filesystem (procfs) for SigmaOS
// Inspired by Linux procfs, providing a dynamic programmatic interface to kernel memory, hardware, and active processes.


use crate::filesystem::vfs::FsError;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// Process entry metadata for ProcFS
#[derive(Debug, Clone)]
pub struct ProcProcessEntry {
    pub pid: usize,
    pub ppid: usize,
    pub name: String,
    pub state: String,
    pub cmdline: Vec<String>,
    pub cwd: String,
    pub exe: String,
    pub environ: BTreeMap<String, String>,
    pub read_bytes: u64,
    pub write_bytes: u64,
}

impl ProcProcessEntry {
    pub fn new(pid: usize, name: &str) -> Self {
        let mut environ = BTreeMap::new();
        environ.insert("PATH".to_string(), "/bin:/usr/bin:/sbin".to_string());
        environ.insert("HOME".to_string(), "/root".to_string());

        Self {
            pid,
            ppid: 0,
            name: name.to_string(),
            state: "R (running)".to_string(),
            cmdline: vec![name.to_string()],
            cwd: "/".to_string(),
            exe: format!("/bin/{}", name),
            environ,
            read_bytes: 4096,
            write_bytes: 2048,
        }
    }
}

/// Represents a dynamic virtual file inside /proc (Linux & BSD inspired procfs)
pub struct SovereignProcFS {
    pub kernel_version: String,
    pub cmdline: String,
    pub cpu_cores: usize,
    pub total_memory_mb: usize,
    pub processes: BTreeMap<usize, ProcProcessEntry>,
}

impl SovereignProcFS {
    pub fn new() -> Self {
        let mut processes = BTreeMap::new();
        processes.insert(1, ProcProcessEntry::new(1, "sigmainit"));
        processes.insert(42, ProcProcessEntry::new(42, "userspace-shell"));

        Self {
            kernel_version: String::from("SigmaOS Sovereign-Kernel v29.0-Zenith"),
            cmdline: String::from("root=LABEL=SIGMA_ROOT rw console=ttyS0 quiet"),
            cpu_cores: 16,
            total_memory_mb: 32768, // 32 GB
            processes,
        }
    }

    pub fn register_process(&mut self, entry: ProcProcessEntry) {
        self.processes.insert(entry.pid, entry);
    }

    pub fn unregister_process(&mut self, pid: usize) {
        self.processes.remove(&pid);
    }

    /// Read dynamic virtual /proc files on-the-fly
    pub fn read_file(&self, path: &str) -> Result<String, FsError> {
        if !path.starts_with("/proc") {
            return Err(FsError::PermissionDenied);
        }

        match path {
            "/proc/version" => Ok(format!("{}\n", self.kernel_version)),
            "/proc/cmdline" => Ok(format!("{}\n", self.cmdline)),
            "/proc/cpuinfo" => {
                let mut info = String::new();
                for core in 0..self.cpu_cores {
                    info.push_str(&format!(
                        "processor\t: {}\nvendor_id\t: SovereignIntel\ncpu family\t: 6\nmodel name\t: SigmaOS Native Core\ncpu cores\t: {}\n\n",
                        core, self.cpu_cores
                    ));
                }
                Ok(info)
            }
            "/proc/meminfo" => {
                let info = format!(
                    "MemTotal\t: {} kB\nMemFree\t\t: {} kB\nMemAvailable\t: {} kB\nBuffers\t\t: 512000 kB\nCached\t\t: 4096000 kB\nSwapTotal\t: {} kB\nSwapFree\t: {} kB\n",
                    self.total_memory_mb * 1024,
                    (self.total_memory_mb / 4) * 1024, // Simulates 25% free
                    (self.total_memory_mb / 2) * 1024,
                    8192 * 1024,
                    8192 * 1024,
                );
                Ok(info)
            }
            "/proc/stat" => {
                Ok("cpu  12345 678 91011 121314 1516 1718 1920 0 0 0\nctxt 987654\nbtime 1680000000\nprocesses 420\nprocs_running 2\nprocs_blocked 0\n".to_string())
            }
            "/proc/loadavg" => Ok("0.15 0.10 0.05 1/128 42\n".to_string()),
            "/proc/filesystems" => Ok("nodev\tsysfs\nnodev\tproc\nnodev\ttmpfs\n\tsigmafs\n\text4\n\tzfs\n".to_string()),
            "/proc/swaps" => Ok("Filename\t\t\t\tType\t\tSize\tUsed\tPriority\n/dev/zram0\t\t\t\tpartition\t8388608\t0\t100\n".to_string()),
            "/proc/mounts" => Ok("rootfs / sigmafs rw,relatime 0 0\nproc /proc proc rw,nosuid,nodev,noexec 0 0\nsysfs /sys sysfs rw,nosuid,nodev,noexec 0 0\n".to_string()),
            _ => {
                // Parse process-specific paths: /proc/<pid>/<file>
                let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
                if parts.len() == 3 && parts[0] == "proc" {
                    if let Ok(pid) = parts[1].parse::<usize>() {
                        if let Some(proc_entry) = self.processes.get(&pid) {
                            match parts[2] {
                                "status" => {
                                    return Ok(format!(
                                        "Name\t\t: {}\nState\t\t: {}\nTgid\t\t: {}\nPid\t\t: {}\nPPid\t\t: {}\nThreads\t\t: 1\n",
                                        proc_entry.name, proc_entry.state, proc_entry.pid, proc_entry.pid, proc_entry.ppid
                                    ));
                                }
                                "cmdline" => {
                                    return Ok(format!("{}\n", proc_entry.cmdline.join(" ")));
                                }
                                "cwd" => return Ok(format!("{}\n", proc_entry.cwd)),
                                "exe" => return Ok(format!("{}\n", proc_entry.exe)),
                                "io" => {
                                    return Ok(format!(
                                        "rchar: {}\nwchar: {}\nread_bytes: {}\nwrite_bytes: {}\n",
                                        proc_entry.read_bytes, proc_entry.write_bytes, proc_entry.read_bytes, proc_entry.write_bytes
                                    ));
                                }
                                "environ" => {
                                    let mut env_str = String::new();
                                    for (k, v) in &proc_entry.environ {
                                        env_str.push_str(&format!("{}={}\n", k, v));
                                    }
                                    return Ok(env_str);
                                }
                                _ => return Err(FsError::NotFound),
                            }
                        }
                    }
                }
                Err(FsError::NotFound)
            }
        }
    }
}

impl Default for SovereignProcFS {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proc_system_files() {
        let procfs = SovereignProcFS::new();

        // Version check
        let version = procfs.read_file("/proc/version").unwrap();
        assert!(version.contains("SigmaOS"));

        // Cmdline check
        let cmdline = procfs.read_file("/proc/cmdline").unwrap();
        assert!(cmdline.contains("SIGMA_ROOT"));

        // Cpuinfo check
        let cpuinfo = procfs.read_file("/proc/cpuinfo").unwrap();
        assert!(cpuinfo.contains("cpu cores"));

        // Meminfo check
        let meminfo = procfs.read_file("/proc/meminfo").unwrap();
        assert!(meminfo.contains("MemTotal"));

        // System stat, loadavg, filesystems, swaps, mounts
        assert!(procfs.read_file("/proc/stat").unwrap().contains("ctxt"));
        assert!(procfs.read_file("/proc/loadavg").unwrap().contains("0.15"));
        assert!(procfs.read_file("/proc/filesystems").unwrap().contains("sigmafs"));
        assert!(procfs.read_file("/proc/swaps").unwrap().contains("/dev/zram0"));
        assert!(procfs.read_file("/proc/mounts").unwrap().contains("sigmafs"));
    }

    #[test]
    fn test_proc_process_status() {
        let mut procfs = SovereignProcFS::new();

        // Read status for PID 42 -> /proc/42/status
        let status = procfs.read_file("/proc/42/status").unwrap();
        assert!(status.contains("Pid\t\t: 42"));
        assert!(status.contains("State\t\t: R (running)"));

        // Read cmdline, cwd, exe, io, environ
        assert!(procfs.read_file("/proc/42/cmdline").unwrap().contains("userspace-shell"));
        assert_eq!(procfs.read_file("/proc/42/cwd").unwrap(), "/\n");
        assert_eq!(procfs.read_file("/proc/42/exe").unwrap(), "/bin/userspace-shell\n");
        assert!(procfs.read_file("/proc/42/io").unwrap().contains("read_bytes"));
        assert!(procfs.read_file("/proc/42/environ").unwrap().contains("PATH"));

        // Register custom process
        let mut entry = ProcProcessEntry::new(100, "worker_daemon");
        entry.ppid = 1;
        procfs.register_process(entry);
        assert!(procfs.read_file("/proc/100/status").unwrap().contains("worker_daemon"));

        // Unregister process
        procfs.unregister_process(100);
        assert!(procfs.read_file("/proc/100/status").is_err());

        // Invalid path check
        assert!(procfs.read_file("/proc/invalid").is_err());
    }
}
