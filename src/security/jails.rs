//! FreeBSD-style Jails Isolation System for SigmaOS
//! Implements OS-level virtualization by isolating directory subtrees, hostnames, IPs, and system capabilities.

#![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::vec;
use core::sync::atomic::{AtomicU32, Ordering};

/// Represents isolated administrative capabilities inside a jail
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JailCapabilities {
    pub allow_raw_sockets: bool,
    pub allow_mounting: bool,
    pub allow_chflags: bool,
    pub allow_sysvipc: bool,
}

impl JailCapabilities {
    pub fn secure_default() -> Self {
        Self {
            allow_raw_sockets: false,
            allow_mounting: false,
            allow_chflags: false,
            allow_sysvipc: false,
        }
    }

    pub fn unrestricted() -> Self {
        Self {
            allow_raw_sockets: true,
            allow_mounting: true,
            allow_chflags: true,
            allow_sysvipc: true,
        }
    }
}

/// Structure representing a FreeBSD-style Jail
#[derive(Debug, Clone)]
pub struct Jail {
    pub jid: u32,
    pub name: String,
    pub path_root: String,
    pub hostname: String,
    pub ip_addresses: Vec<String>,
    pub capabilities: JailCapabilities,
    pub active_processes_count: u32,
}

impl Jail {
    pub fn new(
        jid: u32,
        name: &str,
        path_root: &str,
        hostname: &str,
        ip_addresses: Vec<String>,
        capabilities: JailCapabilities,
    ) -> Self {
        Self {
            jid,
            name: name.to_string(),
            path_root: path_root.to_string(),
            hostname: hostname.to_string(),
            ip_addresses,
            capabilities,
            active_processes_count: 0,
        }
    }

    /// Verifies if a given path is within the jail's isolated path root directory subtree
    pub fn verify_path_isolation(&self, path: &str) -> bool {
        path.starts_with(&self.path_root)
    }

    /// Verifies if a jail can bind to a specific IP address
    pub fn verify_ip_binding_allowed(&self, ip: &str) -> bool {
        self.ip_addresses.iter().any(|allowed_ip| allowed_ip == ip)
    }

    /// Checks if raw socket creation is permitted
    pub fn check_raw_sockets_allowed(&self) -> bool {
        self.capabilities.allow_raw_sockets
    }

    /// Checks if mounting filesystems is permitted
    pub fn check_mounting_allowed(&self) -> bool {
        self.capabilities.allow_mounting
    }
}

/// Global tracking manager for Jails
pub struct JailManager {
    pub jid_allocator: AtomicU32,
    pub active_jails: Vec<Jail>,
}

impl JailManager {
    pub fn new() -> Self {
        Self {
            jid_allocator: AtomicU32::new(1),
            active_jails: Vec::new(),
        }
    }

    /// Spawn/register a new jail under the manager
    pub fn spawn_jail(
        &mut self,
        name: &str,
        path_root: &str,
        hostname: &str,
        ips: Vec<String>,
        caps: JailCapabilities,
    ) -> u32 {
        let jid = self.jid_allocator.fetch_add(1, Ordering::SeqCst);
        let jail = Jail::new(jid, name, path_root, hostname, ips, caps);
        self.active_jails.push(jail);
        jid
    }

    /// Lookup a jail by its ID
    pub fn lookup_jail(&self, jid: u32) -> Option<&Jail> {
        self.active_jails.iter().find(|j| j.jid == jid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jail_resource_restrictions() {
        let mut manager = JailManager::new();

        // Spawn a strict production jail
        let ips = vec!["192.168.1.50".to_string(), "10.0.0.5".to_string()];
        let jid = manager.spawn_jail(
            "strict_sandbox",
            "/jails/sandbox",
            "sandbox.local",
            ips,
            JailCapabilities::secure_default(),
        );

        let jail = manager.lookup_jail(jid).unwrap();
        assert_eq!(jail.name, "strict_sandbox");
        assert_eq!(jail.hostname, "sandbox.local");

        // 1. Verify Directory subtree (chroot-parity) path isolation
        assert!(jail.verify_path_isolation("/jails/sandbox/etc/passwd"));
        assert!(!jail.verify_path_isolation("/etc/passwd"));

        // 2. Verify IP scoping / binding checks
        assert!(jail.verify_ip_binding_allowed("192.168.1.50"));
        assert!(!jail.verify_ip_binding_allowed("192.168.1.1"));

        // 3. Verify security capabilities are strictly gated
        assert!(!jail.check_raw_sockets_allowed());
        assert!(!jail.check_mounting_allowed());
    }

    #[test]
    fn test_unrestricted_jail() {
        let mut manager = JailManager::new();
        let jid = manager.spawn_jail(
            "unrestricted_sandbox",
            "/jails/dev",
            "dev.local",
            vec!["127.0.0.1".to_string()],
            JailCapabilities::unrestricted(),
        );

        let jail = manager.lookup_jail(jid).unwrap();
        assert!(jail.check_raw_sockets_allowed());
        assert!(jail.check_mounting_allowed());
    }
}
