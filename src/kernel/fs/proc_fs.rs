/// SigmaOS proc filesystem (/proc) implementation
/// Provides dynamic system statistics and process information in-memory
use std::collections::HashMap;
use std::string::{String, ToString};

pub struct ProcEntry {
    pub name: String,
    pub is_dir: bool,
    pub content_generator: fn() -> String,
}

pub struct ProcFileSystem {
    entries: HashMap<String, ProcEntry>,
}

impl ProcFileSystem {
    pub fn new() -> Self {
        let mut fs = ProcFileSystem {
            entries: HashMap::new(),
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
        let entry = self.entries.get(path).ok_or("File not found in /proc")?;
        Ok((entry.content_generator)())
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
    }
}
