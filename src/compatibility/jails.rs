// Sovereign Jail and Syscall Sandboxing Isolation Subsystem
// Inspired by FreeBSD Jails, Linux Namespaces, and Linux seccomp-BPF filters.

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NamespaceType {
    Uts,     // Hostname / domain isolation
    Net,     // Network stack isolation (VNET)
    Pid,     // Process ID space isolation
    Mount,   // Filesystem mount table isolation
    User,    // User/Group ID virtualization
}

#[derive(Debug, Clone)]
pub struct NamespaceIsolation {
    pub active_types: HashSet<NamespaceType>,
    pub virtual_hostname: String,
    pub virtual_ip: String,
    pub mapped_root: String,
}

impl NamespaceIsolation {
    pub fn new() -> Self {
        Self {
            active_types: HashSet::new(),
            virtual_hostname: "sigma-node".to_string(),
            virtual_ip: "127.0.0.1".to_string(),
            mapped_root: "/".to_string(),
        }
    }

    pub fn isolate(&mut self, ns: NamespaceType) {
        self.active_types.insert(ns);
    }

    pub fn is_isolated(&self, ns: NamespaceType) -> bool {
        self.active_types.contains(&ns)
    }
}

/// Linux seccomp-BPF inspired syscall filter whitelist.
#[derive(Debug, Clone)]
pub struct SeccompFilter {
    pub allowed_syscalls: HashSet<u32>,
    pub default_action_kill: bool,
}

impl SeccompFilter {
    pub fn new() -> Self {
        let mut filter = Self {
            allowed_syscalls: HashSet::new(),
            default_action_kill: true,
        };
        // By default, allow standard base lifecycle syscalls: read (3), write (4), exit (1)
        filter.allow_syscall(1);
        filter.allow_syscall(3);
        filter.allow_syscall(4);
        filter
    }

    pub fn allow_syscall(&mut self, syscall_num: u32) {
        self.allowed_syscalls.insert(syscall_num);
    }

    pub fn deny_syscall(&mut self, syscall_num: u32) {
        self.allowed_syscalls.remove(&syscall_num);
    }

    /// Evaluates if a syscall invocation is authorized by the BPF whitelist
    pub fn evaluate_syscall(&self, syscall_num: u32) -> Result<(), &'static str> {
        if self.allowed_syscalls.contains(&syscall_num) {
            Ok(())
        } else {
            Err("SYS_SECCOMP_VIOLATION: System call blocked by BPF filter")
        }
    }
}

/// FreeBSD-inspired lightweight OS-level Virtualization Jail container.
#[derive(Debug, Clone)]
pub struct FreeBsdJail {
    pub jail_id: usize,
    pub name: String,
    pub root_path: String,
    pub ip_address: String,
    pub isolated_pids: Vec<usize>,
    pub namespaces: NamespaceIsolation,
    pub seccomp: SeccompFilter,
}

impl FreeBsdJail {
    pub fn new(jail_id: usize, name: &str, root_path: &str, ip_address: &str) -> Self {
        let mut namespaces = NamespaceIsolation::new();
        // Fully isolate all namespaces by default for FreeBSD Jails
        namespaces.isolate(NamespaceType::Uts);
        namespaces.isolate(NamespaceType::Net);
        namespaces.isolate(NamespaceType::Pid);
        namespaces.isolate(NamespaceType::Mount);
        namespaces.isolate(NamespaceType::User);
        namespaces.virtual_hostname = name.to_string();
        namespaces.virtual_ip = ip_address.to_string();
        namespaces.mapped_root = root_path.to_string();

        Self {
            jail_id,
            name: name.to_string(),
            root_path: root_path.to_string(),
            ip_address: ip_address.to_string(),
            isolated_pids: Vec::new(),
            namespaces,
            seccomp: SeccompFilter::new(),
        }
    }

    /// Attach a process ID into this Jail
    pub fn attach_pid(&mut self, pid: usize) {
        self.isolated_pids.push(pid);
    }

    /// Checks if a file path can be accessed from within the Jail root directory bounds.
    /// Prevents directory traversal attacks (jailbreaking).
    pub fn validate_path_access(&self, requested_path: &str) -> bool {
        if !self.namespaces.is_isolated(NamespaceType::Mount) {
            return true;
        }

        // Must start with our jailed root_path
        requested_path.starts_with(&self.root_path) && !requested_path.contains("../")
    }

    /// Evaluates a system call requested by an attached process.
    pub fn filter_syscall(&self, pid: usize, syscall_num: u32) -> Result<(), &'static str> {
        if !self.isolated_pids.contains(&pid) {
            return Ok(()); // Not attached, skip filter
        }

        self.seccomp.evaluate_syscall(syscall_num)
    }
}

/// Sovereign Sandbox Isolation Coordinator
pub struct SovereignSandboxCoordinator {
    pub active_jails: Vec<FreeBsdJail>,
}

impl SovereignSandboxCoordinator {
    pub fn new() -> Self {
        Self {
            active_jails: Vec::new(),
        }
    }

    pub fn spawn_jail(&mut self, name: &str, root_path: &str, ip_address: &str) -> usize {
        let next_id = self.active_jails.len() + 1;
        let jail = FreeBsdJail::new(next_id, name, root_path, ip_address);
        self.active_jails.push(jail);
        next_id
    }

    pub fn get_jail_mut(&mut self, jail_id: usize) -> Option<&mut FreeBsdJail> {
        self.active_jails.iter_mut().find(|j| j.jail_id == jail_id)
    }

    pub fn get_jail(&self, jail_id: usize) -> Option<&FreeBsdJail> {
        self.active_jails.iter().find(|j| j.jail_id == jail_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seccomp_bpf_filter() {
        let mut filter = SeccompFilter::new();

        // Allowed default syscalls
        assert!(filter.evaluate_syscall(1).is_ok()); // SYS_exit
        assert!(filter.evaluate_syscall(3).is_ok()); // SYS_read

        // Blocked raw socket syscall by default
        assert!(filter.evaluate_syscall(41).is_err()); // SYS_socket

        // Manually authorize socket syscall
        filter.allow_syscall(41);
        assert!(filter.evaluate_syscall(41).is_ok());

        // Revoke write syscall
        filter.deny_syscall(4);
        assert!(filter.evaluate_syscall(4).is_err());
    }

    #[test]
    fn test_freebsd_jail_creation_and_attachment() {
        let mut jail = FreeBsdJail::new(42, "secure-nginx", "/jails/secure-nginx", "10.0.0.5");

        assert_eq!(jail.jail_id, 42);
        assert_eq!(jail.name, "secure-nginx");
        assert_eq!(jail.root_path, "/jails/secure-nginx");
        assert_eq!(jail.ip_address, "10.0.0.5");

        // Attach some process IDs
        jail.attach_pid(2001);
        jail.attach_pid(2002);

        assert_eq!(jail.isolated_pids, vec![2001, 2002]);
    }

    #[test]
    fn test_jail_path_traversal_gating() {
        let jail = FreeBsdJail::new(1, "sandbox-app", "/var/sandbox", "192.168.1.100");

        // Safe nested paths within jail root
        assert!(jail.validate_path_access("/var/sandbox/config.json"));
        assert!(jail.validate_path_access("/var/sandbox/html/index.html"));

        // Jailbreak traversal attacks (Should be gated/blocked!)
        assert!(!jail.validate_path_access("/var/sandbox/../etc/passwd"));
        assert!(!jail.validate_path_access("/etc/passwd"));
    }

    #[test]
    fn test_jail_syscall_gating() {
        let mut jail = FreeBsdJail::new(1, "sandbox-app", "/var/sandbox", "192.168.1.100");
        jail.attach_pid(500);

        // Allowed syscall (SYS_read) -> success
        assert!(jail.filter_syscall(500, 3).is_ok());

        // Unallowed syscall (SYS_fork = 57) -> fails
        assert!(jail.filter_syscall(500, 57).is_err());

        // Unattached process -> bypasses jail filter
        assert!(jail.filter_syscall(999, 57).is_ok());
    }

    #[test]
    fn test_sandbox_coordinator() {
        let mut coordinator = SovereignSandboxCoordinator::new();
        let jail_id = coordinator.spawn_jail("database-jail", "/jails/postgres", "10.1.1.1");

        assert_eq!(jail_id, 1);

        let jail = coordinator.get_jail(1).unwrap();
        assert_eq!(jail.name, "database-jail");

        assert!(coordinator.get_jail(99).is_none());
    }
}
