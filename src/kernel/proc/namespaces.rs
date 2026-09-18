#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

/// SigmaOS Linux-style Namespaces for process isolation
/// Supports: PID, Net, Mount, UTS, IPC, User, Cgroup namespaces
use std::string::{String, ToString};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamespaceType {
    Pid,
    Net,
    Mnt,
    Uts,
    Ipc,
    User,
    Cgroup,
}

#[derive(Debug, Clone)]
pub struct Namespace {
    pub id: u64,
    pub ns_type: NamespaceType,
    pub name: String,
}

impl Namespace {
    pub fn new(id: u64, ns_type: NamespaceType, name: &str) -> Self {
        Namespace {
            id,
            ns_type,
            name: name.to_string(),
        }
    }
}

pub struct NamespaceManager {
    next_ns_id: u64,
}

impl NamespaceManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        NamespaceManager { next_ns_id: 1 }
    }

    pub fn create_namespace(&mut self, ns_type: NamespaceType, name: &str) -> Namespace {
        let ns = Namespace::new(self.next_ns_id, ns_type, name);
        self.next_ns_id += 1;
        ns
    }
}

impl Default for NamespaceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_namespace_creation() {
        let mut nsm = NamespaceManager::new();
        let ns1 = nsm.create_namespace(NamespaceType::Pid, "init_pid_ns");
        assert_eq!(ns1.id, 1);
        assert_eq!(ns1.ns_type, NamespaceType::Pid);
        assert_eq!(ns1.name, "init_pid_ns");

        let ns2 = nsm.create_namespace(NamespaceType::Net, "net_ns");
        assert_eq!(ns2.id, 2);
    }
}
