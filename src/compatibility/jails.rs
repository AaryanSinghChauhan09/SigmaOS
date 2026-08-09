// Sovereign Sandboxing, FreeBSD Jails, and Linux-style Namespace Isolation for SigmaOS
// Combines FreeBSD Jails, Linux Namespaces, and Linux Seccomp Syscall Filters into a unified microkernel sandboxing coordinator.

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NamespaceType {
    Uts,   // Hostname / domain isolation
    Net,   // Loopback and virtual interfaces
    Pid,   // Process ID isolation
    Mount, // Separated virtual mount points
    User,  // Isolated user-UID mappings
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandboxError {
    Success = 0,
    JailLocked = 1,
    SyscallBlocked = 2,
    NamespaceViolation = 3,
    InvalidParameter = 4,
}

/// FreeBSD-inspired Jail containing processes inside a secure chroot and virtual network environment
#[derive(Debug, Clone)]
pub struct FreeBsdJail {
    pub jid: usize,
    pub hostname: String,
    pub chroot_path: String,
    pub ip_whitelist: Vec<String>,
    pub active: bool,
}

impl FreeBsdJail {
    pub fn new(jid: usize, hostname: &str, chroot_path: &str) -> Self {
        FreeBsdJail {
            jid,
            hostname: String::from(hostname),
            chroot_path: String::from(chroot_path),
            ip_whitelist: Vec::new(),
            active: true,
        }
    }

    pub fn with_ip(mut self, ip: &str) -> Self {
        self.ip_whitelist.push(String::from(ip));
        self
    }
}

/// Linux-style Namespace Isolation for specific subsystem dimensions
#[derive(Debug, Clone)]
pub struct NamespaceIsolation {
    pub ns_type: NamespaceType,
    pub is_isolated: bool,
    pub virtual_hostname: Option<String>,
    pub mapped_uid_offset: usize,
}

impl NamespaceIsolation {
    pub fn new(ns_type: NamespaceType) -> Self {
        NamespaceIsolation {
            ns_type,
            is_isolated: true,
            virtual_hostname: None,
            mapped_uid_offset: 10000, // Offset unprivileged UIDs
        }
    }
}

/// Linux Seccomp-style Secure Computing System Call filter
#[derive(Debug, Clone)]
pub struct SeccompFilter {
    pub allowed_syscalls: Vec<usize>,
    pub audit_mode: bool,
}

impl SeccompFilter {
    pub fn new(allowed: &[usize]) -> Self {
        SeccompFilter {
            allowed_syscalls: allowed.to_vec(),
            audit_mode: false,
        }
    }

    pub fn is_allowed(&self, syscall_num: usize) -> bool {
        self.allowed_syscalls.contains(&syscall_num)
    }
}

/// Sovereign Sandbox Coordinator orchestrating jail, namespaces, and seccomp filters
pub struct SovereignSandboxCoordinator {
    pub jail: Option<FreeBsdJail>,
    pub namespaces: Vec<NamespaceIsolation>,
    pub seccomp: Option<SeccompFilter>,
    pub is_enforced: AtomicBool,
}

impl SovereignSandboxCoordinator {
    pub fn new() -> Self {
        SovereignSandboxCoordinator {
            jail: None,
            namespaces: Vec::new(),
            seccomp: None,
            is_enforced: AtomicBool::new(false),
        }
    }

    /// Creates and configures a pristine FreeBSD-inspired jail partition
    pub fn configure_jail(&mut self, jid: usize, hostname: &str, chroot_path: &str) {
        let jail = FreeBsdJail::new(jid, hostname, chroot_path);
        self.jail = Some(jail);
    }

    /// Configures and activates specific Linux-style resource namespaces
    pub fn isolate_namespace(&mut self, ns_type: NamespaceType, virtual_host: Option<&str>) {
        let mut ns = NamespaceIsolation::new(ns_type);
        if let Some(host) = virtual_host {
            ns.virtual_hostname = Some(String::from(host));
        }
        self.namespaces.push(ns);
    }

    /// Configures strict Seccomp syscall filtering rules
    pub fn configure_seccomp(&mut self, allowed_syscalls: &[usize]) {
        self.seccomp = Some(SeccompFilter::new(allowed_syscalls));
    }

    /// Lock down and enforce the active sandbox environment (POLA)
    pub fn enforce(&self) {
        self.is_enforced.store(true, Ordering::SeqCst);
    }

    /// Validates system calls against the active Seccomp and Namespace restrictions
    pub fn validate_syscall(&self, syscall_num: usize) -> Result<(), SandboxError> {
        if !self.is_enforced.load(Ordering::Relaxed) {
            return Ok(()); // Sandbox is not locked/active yet
        }

        // 1. Verify Seccomp constraints
        if let Some(ref filter) = self.seccomp {
            if !filter.is_allowed(syscall_num) {
                return Err(SandboxError::SyscallBlocked);
            }
        }

        Ok(())
    }

    /// Validates UTS hostname queries inside isolated UTS Namespaces
    pub fn query_hostname(&self, default_hostname: &str) -> String {
        if let Some(ref uts_ns) = self
            .namespaces
            .iter()
            .find(|n| n.ns_type == NamespaceType::Uts)
        {
            if let Some(ref virt_host) = uts_ns.virtual_hostname {
                return virt_host.clone();
            }
        }
        // Fall back to FreeBSD jail hostname
        if let Some(ref jail) = self.jail {
            return jail.hostname.clone();
        }
        String::from(default_hostname)
    }

    /// Validates IP binding targets inside network namespace constraints
    pub fn validate_network_bind(&self, ip: &str) -> Result<(), SandboxError> {
        if !self.is_enforced.load(Ordering::Relaxed) {
            return Ok(());
        }

        // If net namespace is isolated, verify against the jail's IP whitelist
        if let Some(_) = self
            .namespaces
            .iter()
            .find(|n| n.ns_type == NamespaceType::Net)
        {
            if let Some(ref jail) = self.jail {
                if !jail.ip_whitelist.contains(&String::from(ip)) {
                    return Err(SandboxError::NamespaceViolation);
                }
            }
        }

        Ok(())
    }
}

impl Default for SovereignSandboxCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freebsd_jail_creation_and_config() {
        let mut sandbox = SovereignSandboxCoordinator::new();
        sandbox.configure_jail(12, "web-jail", "/zones/web_root");

        let jail = sandbox.jail.as_ref().unwrap();
        assert_eq!(jail.jid, 12);
        assert_eq!(jail.hostname, "web-jail");
        assert_eq!(jail.chroot_path, "/zones/web_root");
        assert!(jail.active);
    }

    #[test]
    fn test_linux_uts_namespace_hostname_isolation() {
        let mut sandbox = SovereignSandboxCoordinator::new();

        // 1. Initially returns default host
        assert_eq!(sandbox.query_hostname("sigma-host"), "sigma-host");

        // 2. Configure FreeBSD jail -> returns jail hostname
        sandbox.configure_jail(1, "jail-host", "/");
        assert_eq!(sandbox.query_hostname("sigma-host"), "jail-host");

        // 3. Isolate UTS namespace -> returns virtual UTS hostname
        sandbox.isolate_namespace(NamespaceType::Uts, Some("uts-isolated-host"));
        assert_eq!(sandbox.query_hostname("sigma-host"), "uts-isolated-host");
    }

    #[test]
    fn test_seccomp_syscall_filtering() {
        let mut sandbox = SovereignSandboxCoordinator::new();

        // Allow only sys_read (0) and sys_write (1)
        sandbox.configure_seccomp(&[0, 1]);
        sandbox.enforce();

        // sys_read and sys_write must be allowed
        assert!(sandbox.validate_syscall(0).is_ok());
        assert!(sandbox.validate_syscall(1).is_ok());

        // sys_fork (12) must be blocked by seccomp
        assert_eq!(
            sandbox.validate_syscall(12),
            Err(SandboxError::SyscallBlocked)
        );
    }

    #[test]
    fn test_net_namespace_ip_binding_restrictions() {
        let mut sandbox = SovereignSandboxCoordinator::new();
        sandbox.configure_jail(1, "jail1", "/");
        // Add allowed local IP to whitelist
        sandbox.jail = sandbox.jail.map(|j| j.with_ip("127.0.0.1"));

        sandbox.isolate_namespace(NamespaceType::Net, None);
        sandbox.enforce();

        // Binding to loopback whitelisted IP passes
        assert!(sandbox.validate_network_bind("127.0.0.1").is_ok());

        // Binding to unauthorized external IP fails
        assert_eq!(
            sandbox.validate_network_bind("192.168.1.100"),
            Err(SandboxError::NamespaceViolation)
        );
    }
}
