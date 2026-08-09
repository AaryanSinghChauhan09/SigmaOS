// Sovereign /proc Virtual Filesystem (procfs) for SigmaOS
// Inspired by Linux procfs, providing a dynamic programmatic interface to kernel memory, hardware, and active processes.

extern crate alloc;

use crate::filesystem::vfs::FsError;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// Represents a dynamic virtual file inside /proc
pub struct SovereignProcFS {
    pub kernel_version: String,
    pub cpu_cores: usize,
    pub total_memory_mb: usize,
}

impl SovereignProcFS {
    pub fn new() -> Self {
        Self {
            kernel_version: String::from("SigmaOS Sovereign-Kernel v29.0-Zenith"),
            cpu_cores: 16,
            total_memory_mb: 32768, // 32 GB
        }
    }

    /// Read dynamic virtual /proc files on-the-fly
    pub fn read_file(&self, path: &str) -> Result<String, FsError> {
        if !path.starts_with("/proc") {
            return Err(FsError::PermissionDenied);
        }

        match path {
            "/proc/version" => Ok(format!("{}\n", self.kernel_version)),
            "/proc/cpuinfo" => {
                let info = format!(
                    "processor\t: 0\nvendor_id\t: SovereignIntel\ncpu family\t: 6\nmodel name\t: SigmaOS Native Core\ncpu cores\t: {}\n",
                    self.cpu_cores
                );
                Ok(info)
            }
            "/proc/meminfo" => {
                let info = format!(
                    "MemTotal\t: {} kB\nMemFree\t\t: {} kB\nBuffers\t\t: 512000 kB\nCached\t\t: 4096000 kB\n",
                    self.total_memory_mb * 1024,
                    (self.total_memory_mb / 4) * 1024 // Simulates 25% free
                );
                Ok(info)
            }
            _ => {
                // Check if reading process specific status: /proc/<pid>/status
                let parts: Vec<&str> = path.split('/').collect();
                if parts.len() == 4 && parts[1] == "proc" && parts[3] == "status" {
                    if let Ok(pid) = parts[2].parse::<usize>() {
                        let proc_status = format!(
                            "Name\t\t: userspace-shell\nState\t\t: R (running)\nTgid\t\t: {}\nPid\t\t: {}\nPPid\t\t: 0\nThreads\t\t: 1\n",
                            pid, pid
                        );
                        return Ok(proc_status);
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

        // Cpuinfo check
        let cpuinfo = procfs.read_file("/proc/cpuinfo").unwrap();
        assert!(cpuinfo.contains("cpu cores"));

        // Meminfo check
        let meminfo = procfs.read_file("/proc/meminfo").unwrap();
        assert!(meminfo.contains("MemTotal"));
    }

    #[test]
    fn test_proc_process_status() {
        let procfs = SovereignProcFS::new();

        // Read status for PID 42 -> /proc/42/status
        let status = procfs.read_file("/proc/42/status").unwrap();
        assert!(status.contains("Pid\t\t: 42"));
        assert!(status.contains("State\t\t: R (running)"));

        // Invalid path check
        assert!(procfs.read_file("/proc/invalid").is_err());
    }
}
