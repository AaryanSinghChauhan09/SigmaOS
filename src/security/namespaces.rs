extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;
use alloc::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamespaceType {
    Mount,
    Uts,
    Ipc,
    Pid,
    Net,
    User,
    Cgroup,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceId(pub u64);

pub struct Namespace {
    pub id: NamespaceId,
    pub ns_type: NamespaceType,
    pub ref_count: usize,
}

pub struct PidNamespace {
    pub id: NamespaceId,
    pub parent: Option<NamespaceId>,
    pub pid_map: BTreeMap<u32, u32>, // Virtual PID -> Global PID
    pub next_pid: u32,
}

impl PidNamespace {
    pub fn new(id: NamespaceId, parent: Option<NamespaceId>) -> Self {
        Self {
            id,
            parent,
            pid_map: BTreeMap::new(),
            next_pid: 1, // Init is 1
        }
    }

    pub fn allocate_pid(&mut self, global_pid: u32) -> u32 {
        let virtual_pid = self.next_pid;
        self.next_pid += 1;
        self.pid_map.insert(virtual_pid, global_pid);
        virtual_pid
    }

    pub fn get_global_pid(&self, virtual_pid: u32) -> Option<u32> {
        self.pid_map.get(&virtual_pid).copied()
    }
}

pub struct NetworkNamespace {
    pub id: NamespaceId,
    pub interfaces: Vec<String>,
    pub loopback_up: bool,
}

pub struct UtsNamespace {
    pub id: NamespaceId,
    pub hostname: String,
    pub domainname: String,
}
