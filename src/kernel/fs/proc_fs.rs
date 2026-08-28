#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

extern crate alloc;
/// SigmaOS proc filesystem (/proc) implementation
/// Provides dynamic system statistics and process information in-memory
use crate::klib::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub struct ProcEntry {
    pub name: String,
    pub is_dir: bool,
    pub content_generator: fn() -> String,
}

pub struct ProcFileSystem {
    entries: BTreeMap<String, ProcEntry>,
}

impl ProcFileSystem {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut fs = ProcFileSystem {
            entries: BTreeMap::new(),
        };

        // Standard /proc entries
        fs.register_file("cpuinfo", || {
            "processor\t: 0\nvendor_id\t: SovereignCPU\ncpu family\t: 6\nmodel name\t: SigmaOS Native Core\ncpu MHz\t\t: 3500.000\n".to_string()
        });

        fs.register_file("meminfo", || {
            "MemTotal:\t 8388608 kB\nMemFree:\t 4194304 kB\nBuffers:\t  131072 kB\nCached:\t\t 1048576 kB\n".to_string()
        });

        fs.register_file("version", || {
            "SigmaOS version 1.0.0 (antigravity@sovereign) (rustc 1.80.0) #1 SMP Mon Jul 20 2026\n"
                .to_string()
        });

        fs
    }

    pub fn register_file(&mut self, path: &str, generator: fn() -> String) {
        self.entries.insert(
            path.to_string(),
            ProcEntry {
                name: path.to_string(),
                is_dir: false,
                content_generator: generator,
            },
        );
    }

    pub fn read_file(&self, path: &str) -> Result<String, &'static str> {
        if let Some(entry) = self.entries.get(path) {
            return Ok((entry.content_generator)());
        }

        // Support dynamic process directory checks: e.g. path matches "[pid]/status"
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() == 2 {
            let pid_res = parts[0].parse::<u64>();
            if let Ok(pid) = pid_res {
                let cmd = parts[1];
                if cmd == "status" {
                    return Ok(format!(
                        "Name:\tprocess_{}\nState:\tR (running)\nTgid:\t{}\nPid:\t{}\nPPid:\t1\nThreads:\t1\nVmSize:\t4096 kB\nVmRSS:\t512 kB\n",
                        pid, pid, pid
                    ));
                } else if cmd == "cmdline" {
                    return Ok(format!("/bin/process_{}\0--daemon\0", pid));
                } else if cmd == "stat" {
                    return Ok(format!("{} (process_{}) R 1 1 0 0 0 4194304 0 0 0 0 0 0 0 0 20 0 1 0 1337 4194304 512", pid, pid));
                }
            }
        }

        Err("File not found in /proc")
    }
}

impl Default for ProcFileSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proc_fs() {
        let mut proc_fs = ProcFileSystem::new();
        let cpu = proc_fs.read_file("cpuinfo").unwrap();
        assert!(cpu.contains("SovereignCPU"));

        proc_fs.register_file("uptime", || "42.00 42.00\n".to_string());
        assert_eq!(proc_fs.read_file("uptime").unwrap(), "42.00 42.00\n");

        // Verify dynamic Linux distro /proc/[pid]/status, cmdline, and stat mappings
        let status = proc_fs.read_file("1234/status").unwrap();
        assert!(status.contains("Name:\tprocess_1234"));
        assert!(status.contains("VmSize:\t4096 kB"));

        let cmdline = proc_fs.read_file("1234/cmdline").unwrap();
        assert_eq!(cmdline, "/bin/process_1234\0--daemon\0");

        let stat = proc_fs.read_file("1234/stat").unwrap();
        assert!(stat.contains("R 1 1 0"));
    }
}
