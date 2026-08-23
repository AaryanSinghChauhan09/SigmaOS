extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CgroupController {
    Cpu,
    Memory,
    Io,
    Pid,
    Freezer,
}

#[derive(Debug, Clone)]
pub struct CpuLimit {
    pub quota_us: i64,
    pub period_us: u64,
    pub weight: u32,
}

#[derive(Debug, Clone)]
pub struct MemoryLimit {
    pub max_bytes: u64,
    pub current_bytes: u64,
    pub oom_kill_disable: bool,
}

pub struct Cgroup {
    pub name: String,
    pub path: String,
    pub pids: Vec<u32>,
    pub children: Vec<Cgroup>,
    pub controllers: Vec<CgroupController>,
    
    // Limits
    pub cpu: Option<CpuLimit>,
    pub memory: Option<MemoryLimit>,
    pub frozen: bool,
}

impl Cgroup {
    pub fn new(name: &str, path: &str) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_string(),
            pids: Vec::new(),
            children: Vec::new(),
            controllers: Vec::new(),
            cpu: None,
            memory: None,
            frozen: false,
        }
    }

    pub fn add_pid(&mut self, pid: u32) -> Result<(), &'static str> {
        if !self.pids.contains(&pid) {
            self.pids.push(pid);
        }
        Ok(())
    }

    pub fn remove_pid(&mut self, pid: u32) {
        self.pids.retain(|&p| p != pid);
    }

    pub fn set_memory_limit(&mut self, max_bytes: u64) {
        let current = self.memory.as_ref().map(|m| m.current_bytes).unwrap_or(0);
        self.memory = Some(MemoryLimit {
            max_bytes,
            current_bytes: current,
            oom_kill_disable: false,
        });
    }

    pub fn check_memory(&mut self, alloc_bytes: u64) -> Result<(), &'static str> {
        if let Some(mem) = &mut self.memory {
            if mem.current_bytes + alloc_bytes > mem.max_bytes {
                return Err("Out of memory in cgroup");
            }
            mem.current_bytes += alloc_bytes;
        }
        Ok(())
    }
    
    pub fn freeze(&mut self) {
        self.frozen = true;
        // In a real OS, this would send signals to all PIDs in self.pids
    }
    
    pub fn unfreeze(&mut self) {
        self.frozen = false;
    }
}
