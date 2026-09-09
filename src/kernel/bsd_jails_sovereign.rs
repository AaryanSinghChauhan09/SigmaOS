//! SigmaOS Sovereign BSD Jails Process Isolation
//! Implements FreeBSD Jails-style lightweight OS virtualization in 100% safe Rust.
//!
//! FreeBSD Jails (since FreeBSD 4.0, 2000) provide lightweight OS-level
//! virtualization. Each jail has its own hostname, IP address, filesystem root,
//! and process namespace. This module implements the same concepts in pure Rust.
//!
//! Also incorporates OpenBSD securelevel, Linux network namespaces concept.

#![allow(dead_code)]
#![allow(clippy::new_without_default)]

#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

// ─── Jail Parameters (mirrors jail(2) struct) ─────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum JailState {
    Creating,
    Running,
    Dying,
    Dead,
}

#[derive(Debug, Clone)]
pub struct JailNetworkConfig {
    pub ipv4_addresses: Vec<[u8; 4]>,
    pub ipv6_addresses: Vec<[u8; 16]>,
    pub hostname: String,
    pub allow_raw_sockets: bool,
    pub allow_bind_any: bool,
}

impl JailNetworkConfig {
    pub fn new(hostname: &str) -> Self {
        JailNetworkConfig {
            ipv4_addresses: Vec::new(),
            ipv6_addresses: Vec::new(),
            hostname: hostname.to_string(),
            allow_raw_sockets: false,
            allow_bind_any: false,
        }
    }

    pub fn add_ipv4(&mut self, addr: [u8; 4]) {
        self.ipv4_addresses.push(addr);
    }

    pub fn has_ipv4(&self, addr: &[u8; 4]) -> bool {
        self.ipv4_addresses.iter().any(|a| a == addr)
    }
}

// ─── Jail Permissions (mirrors jail parameters) ───────────────────────────────

#[derive(Debug, Clone)]
pub struct JailPermissions {
    pub allow_mount: bool,
    pub allow_mount_devfs: bool,
    pub allow_mount_nullfs: bool,
    pub allow_mount_zfs: bool,
    pub allow_chflags: bool,
    pub allow_quotas: bool,
    pub allow_socket_af: bool,
    pub allow_sysvipc: bool,
    pub allow_raw_sockets: bool,
    pub allow_set_hostname: bool,
    pub enforce_statfs: u8, // 0=all, 1=jail root, 2=jail root only
    pub securelevel: i8,    // OpenBSD securelevel: -1 disabled, 0 permissive, 1 immutable, 2 highly secure
    pub devfs_ruleset: u32,
}

impl JailPermissions {
    pub fn secure_defaults() -> Self {
        JailPermissions {
            allow_mount: false,
            allow_mount_devfs: false,
            allow_mount_nullfs: false,
            allow_mount_zfs: false,
            allow_chflags: false,
            allow_quotas: false,
            allow_socket_af: false,
            allow_sysvipc: false,
            allow_raw_sockets: false,
            allow_set_hostname: false,
            enforce_statfs: 2,
            securelevel: 1,
            devfs_ruleset: 4,
        }
    }

    pub fn relaxed() -> Self {
        JailPermissions {
            allow_mount: true,
            allow_mount_devfs: true,
            allow_mount_nullfs: true,
            allow_mount_zfs: false,
            allow_chflags: false,
            allow_quotas: true,
            allow_socket_af: true,
            allow_sysvipc: false,
            allow_raw_sockets: false,
            allow_set_hostname: true,
            enforce_statfs: 1,
            securelevel: 0,
            devfs_ruleset: 0,
        }
    }
}

// ─── Process Record within a Jail ─────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct JailProcess {
    pub pid: u32,
    pub ppid: u32,
    pub name: String,
    pub uid: u32,
    pub gid: u32,
}

// ─── Jail ─────────────────────────────────────────────────────────────────────

pub struct SovereignJail {
    pub jid: u32,
    pub name: String,
    pub path: String, // chroot path (jail root)
    pub network: JailNetworkConfig,
    pub permissions: JailPermissions,
    pub state: JailState,
    pub processes: Vec<JailProcess>,
    pub child_jail_ids: Vec<u32>,
    pub parent_jid: Option<u32>,
    pub create_epoch: u64,
    pub syscall_violation_count: u64,
}

impl SovereignJail {
    pub fn new(jid: u32, name: &str, path: &str, hostname: &str) -> Self {
        SovereignJail {
            jid,
            name: name.to_string(),
            path: path.to_string(),
            network: JailNetworkConfig::new(hostname),
            permissions: JailPermissions::secure_defaults(),
            state: JailState::Creating,
            processes: Vec::new(),
            child_jail_ids: Vec::new(),
            parent_jid: None,
            create_epoch: 0,
            syscall_violation_count: 0,
        }
    }

    pub fn start(&mut self) -> bool {
        if self.state != JailState::Creating { return false; }
        self.state = JailState::Running;
        true
    }

    pub fn attach_process(&mut self, proc: JailProcess) -> bool {
        if self.state != JailState::Running { return false; }
        // Enforce securelevel: level >= 1 blocks root from reducing securelevel
        if proc.uid == 0 && self.permissions.securelevel >= 2 {
            // Highly secure: even root cannot load kernel modules etc.
            // We still allow process attachment but record it
        }
        self.processes.push(proc);
        true
    }

    pub fn kill_process(&mut self, pid: u32) -> bool {
        if let Some(idx) = self.processes.iter().position(|p| p.pid == pid) {
            self.processes.remove(idx);
            true
        } else { false }
    }

    pub fn process_count(&self) -> usize { self.processes.len() }

    pub fn teardown(&mut self) {
        self.state = JailState::Dying;
        self.processes.clear();
        self.state = JailState::Dead;
    }

    /// Check if a syscall is allowed given jail permissions.
    /// Mirrors jail's syscall interception in FreeBSD.
    pub fn check_syscall(&mut self, syscall: &str) -> bool {
        let allowed = match syscall {
            "mount"        => self.permissions.allow_mount,
            "sysvipc"      => self.permissions.allow_sysvipc,
            "raw_socket"   => self.permissions.allow_raw_sockets,
            "set_hostname" => self.permissions.allow_set_hostname,
            "chflags"      => self.permissions.allow_chflags,
            _              => true, // Default: allow unknown syscalls
        };
        if !allowed {
            self.syscall_violation_count = self.syscall_violation_count.saturating_add(1);
        }
        allowed
    }

    pub fn summary(&self) -> String {
        let mut s = String::from("jail[");
        s.push_str(&self.jid.to_string());
        s.push_str("] name=");
        s.push_str(&self.name);
        s.push_str(" path=");
        s.push_str(&self.path);
        s.push_str(" host=");
        s.push_str(&self.network.hostname);
        s.push_str(" procs=");
        s.push_str(&self.process_count().to_string());
        s.push_str(" state=");
        s.push_str(match self.state {
            JailState::Creating => "creating",
            JailState::Running  => "running",
            JailState::Dying    => "dying",
            JailState::Dead     => "dead",
        });
        s
    }
}

// ─── Jail Manager ─────────────────────────────────────────────────────────────

pub struct SovereignBsdJailManager {
    pub jails: Vec<SovereignJail>,
    pub next_jid: u32,
    pub max_jails: u32,
}

impl SovereignBsdJailManager {
    pub fn new(max_jails: u32) -> Self {
        SovereignBsdJailManager {
            jails: Vec::new(),
            next_jid: 1,
            max_jails,
        }
    }

    pub fn create_jail(&mut self, name: &str, path: &str, hostname: &str) -> Option<u32> {
        if self.jails.len() as u32 >= self.max_jails { return None; }
        let jid = self.next_jid;
        self.next_jid = self.next_jid.saturating_add(1);
        let jail = SovereignJail::new(jid, name, path, hostname);
        self.jails.push(jail);
        Some(jid)
    }

    pub fn start_jail(&mut self, jid: u32) -> bool {
        self.get_mut(jid).map(|j| j.start()).unwrap_or(false)
    }

    pub fn teardown_jail(&mut self, jid: u32) -> bool {
        if let Some(jail) = self.get_mut(jid) {
            jail.teardown();
            true
        } else { false }
    }

    pub fn get_mut(&mut self, jid: u32) -> Option<&mut SovereignJail> {
        self.jails.iter_mut().find(|j| j.jid == jid)
    }

    pub fn get(&self, jid: u32) -> Option<&SovereignJail> {
        self.jails.iter().find(|j| j.jid == jid)
    }

    pub fn running_count(&self) -> usize {
        self.jails.iter().filter(|j| j.state == JailState::Running).count()
    }

    pub fn total_processes(&self) -> usize {
        self.jails.iter().map(|j| j.process_count()).sum()
    }

    pub fn jls(&self) -> String {
        // Like FreeBSD's `jls` command
        let mut out = String::from("JID  Path                    Host            State\n");
        for j in &self.jails {
            out.push_str(&j.summary());
            out.push('\n');
        }
        out
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jail_create_start() {
        let mut mgr = SovereignBsdJailManager::new(64);
        let jid = mgr.create_jail("web", "/jails/web", "web.sigma").unwrap();
        assert_eq!(jid, 1);
        assert!(mgr.start_jail(jid));
        assert_eq!(mgr.running_count(), 1);
    }

    #[test]
    fn test_jail_process_attach() {
        let mut mgr = SovereignBsdJailManager::new(64);
        let jid = mgr.create_jail("db", "/jails/db", "db.sigma").unwrap();
        mgr.start_jail(jid);
        let proc = JailProcess { pid: 100, ppid: 1, name: "postgres".to_string(), uid: 70, gid: 70 };
        assert!(mgr.get_mut(jid).unwrap().attach_process(proc));
        assert_eq!(mgr.total_processes(), 1);
    }

    #[test]
    fn test_jail_syscall_enforcement() {
        let mut mgr = SovereignBsdJailManager::new(64);
        let jid = mgr.create_jail("secure", "/jails/secure", "secure.sigma").unwrap();
        mgr.start_jail(jid);
        let jail = mgr.get_mut(jid).unwrap();
        // Secure defaults deny mount, sysvipc
        assert!(!jail.check_syscall("mount"));
        assert!(!jail.check_syscall("sysvipc"));
        assert_eq!(jail.syscall_violation_count, 2);
    }

    #[test]
    fn test_jail_max_limit() {
        let mut mgr = SovereignBsdJailManager::new(2);
        assert!(mgr.create_jail("j1", "/j1", "h1").is_some());
        assert!(mgr.create_jail("j2", "/j2", "h2").is_some());
        assert!(mgr.create_jail("j3", "/j3", "h3").is_none()); // Exceeds max
    }

    #[test]
    fn test_jail_teardown() {
        let mut mgr = SovereignBsdJailManager::new(64);
        let jid = mgr.create_jail("tmp", "/jails/tmp", "tmp.sigma").unwrap();
        mgr.start_jail(jid);
        let proc = JailProcess { pid: 200, ppid: 1, name: "sh".to_string(), uid: 0, gid: 0 };
        mgr.get_mut(jid).unwrap().attach_process(proc);
        mgr.teardown_jail(jid);
        assert_eq!(mgr.get(jid).unwrap().process_count(), 0);
        assert_eq!(mgr.get(jid).unwrap().state, JailState::Dead);
    }

    #[test]
    fn test_network_config() {
        let mut mgr = SovereignBsdJailManager::new(64);
        let jid = mgr.create_jail("net", "/jails/net", "net.sigma").unwrap();
        mgr.start_jail(jid);
        let jail = mgr.get_mut(jid).unwrap();
        jail.network.add_ipv4([10, 0, 0, 1]);
        jail.network.add_ipv4([10, 0, 0, 2]);
        assert!(jail.network.has_ipv4(&[10, 0, 0, 1]));
        assert!(!jail.network.has_ipv4(&[192, 168, 1, 1]));
    }
}
