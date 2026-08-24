// #![no_std]
extern crate alloc;
use alloc::vec::Vec;
use alloc::string::{String, ToString};
use alloc::collections::BTreeMap;
use crate::filesystem::vfs::{Inode, FileType, FilePermissions};

pub struct ProcNode {
    pub name: String,
    pub is_dir: bool,
    pub children: BTreeMap<String, ProcNode>,
    pub data_generator: fn() -> String,
}

pub struct ProcFs {
    pub root: ProcNode,
}

impl ProcFs {
    pub fn new() -> Self {
        let mut root = ProcNode {
            name: String::from(""),
            is_dir: true,
            children: BTreeMap::new(),
            data_generator: || String::new(),
        };
        
        root.children.insert("meminfo".to_string(), ProcNode {
            name: "meminfo".to_string(), is_dir: false, children: BTreeMap::new(),
            data_generator: || "MemTotal: 4194304 kB\nMemFree: 2048576 kB\n".to_string()
        });
        
        root.children.insert("cpuinfo".to_string(), ProcNode {
            name: "cpuinfo".to_string(), is_dir: false, children: BTreeMap::new(),
            data_generator: || "processor: 0\nvendor_id: GenuineIntel\ncpu family: 6\n".to_string()
        });
        
        root.children.insert("uptime".to_string(), ProcNode {
            name: "uptime".to_string(), is_dir: false, children: BTreeMap::new(),
            data_generator: || "1234.56 789.01\n".to_string()
        });
        
        root.children.insert("version".to_string(), ProcNode {
            name: "version".to_string(), is_dir: false, children: BTreeMap::new(),
            data_generator: || "SigmaOS version 0.1.0\n".to_string()
        });
        
        Self { root }
    }
    
    pub fn add_process(&mut self, pid: u32) {
        let mut proc_dir = ProcNode {
            name: pid.to_string(), is_dir: true, children: BTreeMap::new(),
            data_generator: || String::new()
        };
        
        proc_dir.children.insert("status".to_string(), ProcNode {
            name: "status".to_string(), is_dir: false, children: BTreeMap::new(),
            data_generator: || "State: R (running)\n".to_string()
        });
        
        self.root.children.insert(pid.to_string(), proc_dir);
    }
    
    pub fn read_path(&self, path: &str) -> Option<String> {
        let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        let mut current = &self.root;
        
        for part in parts {
            if let Some(child) = current.children.get(part) {
                current = child;
            } else {
                return None;
            }
        }
        
        if current.is_dir { None } else { Some((current.data_generator)()) }
    }
}
