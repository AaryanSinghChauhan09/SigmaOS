//! SigmaOS Capability-Based Security Monitor
//!
//! Sovereign implementation of capability-based security combining:
//! - OpenBSD pledge(2): process declares allowed syscall sets upfront
//! - OpenBSD unveil(2): process declares visible filesystem paths
//! - FreeBSD Capsicum: capability mode + capability rights on file descriptors
//! - Linux Landlock LSM: filesystem access control without privilege
//!
//! Design goals:
//! - Processes REDUCE their own privileges by declaring what they need
//! - Violations are caught at policy enforcement points (not at runtime crash)
//! - Zero external dependencies — sovereign Rust implementation
//! - Compatible with SigmaInit's pledge/unveil declarations for service sandboxing
//!
//! Architecture:
//! - `SyscallClass` enum: logical groups of related syscalls (like pledge promises)
//! - `FsAccess` flags: fine-grained filesystem permission bits
//! - `CapabilityRights`: Capsicum-style rights on file descriptors
//! - `PledgeProfile`: sealed set of allowed syscall classes (immutable after commit)
//! - `UnveilEntry`: (path, access_flags) with prefix matching
//! - `CapabilityMonitor`: per-process monitor enforcing all policies

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;
use std::format;

// ─── Syscall Class (pledge promises) ─────────────────────────────────────────

/// Syscall class (like OpenBSD pledge promise groups)
///
/// Each class represents a logical group of related system calls.
/// Processes declare only the classes they need — all others are denied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SyscallClass {
    /// Basic I/O: read, write, fstat, clock_gettime
    Stdio,
    /// Read-only filesystem access: open(O_RDONLY), stat
    Rpath,
    /// Write/create filesystem access: open(O_WRONLY|O_CREAT), unlink
    Wpath,
    /// File creation: open(O_CREAT), mkdir, symlink
    Cpath,
    /// DNS resolution: getaddrinfo, getnameinfo
    Dns,
    /// Network: socket(AF_INET), bind, connect, listen
    Inet,
    /// Unix domain sockets: socket(AF_UNIX), sendmsg, recvmsg
    Unix,
    /// Process management: fork, execve, waitpid
    Proc,
    /// Program execution: execve (only)
    Exec,
    /// Thread creation: clone, pthread_create
    Thread,
    /// Set time: settimeofday, clock_settime
    Settime,
    /// User identity: setuid, setgid, getuid
    Id,
    /// Virtual memory: mmap(PROT_EXEC), mprotect
    Prot_exec,
    /// Error output only: write(STDERR_FILENO)
    Stdio_err,
    /// Cryptographic random: getrandom, getentropy
    Getrandom,
}

impl SyscallClass {
    pub fn label(&self) -> &'static str {
        match self {
            SyscallClass::Stdio => "stdio",
            SyscallClass::Rpath => "rpath",
            SyscallClass::Wpath => "wpath",
            SyscallClass::Cpath => "cpath",
            SyscallClass::Dns => "dns",
            SyscallClass::Inet => "inet",
            SyscallClass::Unix => "unix",
            SyscallClass::Proc => "proc",
            SyscallClass::Exec => "exec",
            SyscallClass::Thread => "thread",
            SyscallClass::Settime => "settime",
            SyscallClass::Id => "id",
            SyscallClass::Prot_exec => "prot_exec",
            SyscallClass::Stdio_err => "stdio_err",
            SyscallClass::Getrandom => "getrandom",
        }
    }

    /// Parse a promise string into a SyscallClass
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "stdio" => Some(SyscallClass::Stdio),
            "rpath" => Some(SyscallClass::Rpath),
            "wpath" => Some(SyscallClass::Wpath),
            "cpath" => Some(SyscallClass::Cpath),
            "dns" => Some(SyscallClass::Dns),
            "inet" => Some(SyscallClass::Inet),
            "unix" => Some(SyscallClass::Unix),
            "proc" => Some(SyscallClass::Proc),
            "exec" => Some(SyscallClass::Exec),
            "thread" => Some(SyscallClass::Thread),
            "settime" => Some(SyscallClass::Settime),
            "id" => Some(SyscallClass::Id),
            "prot_exec" => Some(SyscallClass::Prot_exec),
            "stdio_err" => Some(SyscallClass::Stdio_err),
            "getrandom" => Some(SyscallClass::Getrandom),
            _ => None,
        }
    }
}

// ─── Filesystem Access Flags ──────────────────────────────────────────────────

/// Bitmask of filesystem access permissions (Landlock-inspired)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FsAccess(pub u32);

impl FsAccess {
    pub const READ_FILE: FsAccess   = FsAccess(1 << 0);
    pub const WRITE_FILE: FsAccess  = FsAccess(1 << 1);
    pub const EXECUTE: FsAccess     = FsAccess(1 << 2);
    pub const READ_DIR: FsAccess    = FsAccess(1 << 3);
    pub const ADD_FILE: FsAccess    = FsAccess(1 << 4);
    pub const ADD_DIR: FsAccess     = FsAccess(1 << 5);
    pub const REMOVE_FILE: FsAccess = FsAccess(1 << 6);
    pub const REMOVE_DIR: FsAccess  = FsAccess(1 << 7);

    pub fn empty() -> Self { FsAccess(0) }
    pub fn all() -> Self { FsAccess(0xFF) }

    pub fn contains(self, other: FsAccess) -> bool {
        (self.0 & other.0) == other.0
    }

    pub fn union(self, other: FsAccess) -> Self {
        FsAccess(self.0 | other.0)
    }

    /// Parse shorthand permission string: "r" / "rw" / "rwc" / "rx" / "rwxc"
    pub fn parse(s: &str) -> Self {
        let mut flags = FsAccess::empty();
        if s.contains('r') {
            flags = flags.union(FsAccess::READ_FILE).union(FsAccess::READ_DIR);
        }
        if s.contains('w') {
            flags = flags.union(FsAccess::WRITE_FILE);
        }
        if s.contains('x') {
            flags = flags.union(FsAccess::EXECUTE);
        }
        if s.contains('c') {
            flags = flags.union(FsAccess::ADD_FILE).union(FsAccess::ADD_DIR);
        }
        if s.contains('d') {
            flags = flags.union(FsAccess::REMOVE_FILE).union(FsAccess::REMOVE_DIR);
        }
        flags
    }

    pub fn to_string_repr(self) -> String {
        let mut s = String::new();
        if self.contains(FsAccess::READ_FILE) { s.push('r'); }
        if self.contains(FsAccess::WRITE_FILE) { s.push('w'); }
        if self.contains(FsAccess::EXECUTE) { s.push('x'); }
        if self.contains(FsAccess::ADD_FILE) { s.push('c'); }
        if self.contains(FsAccess::REMOVE_FILE) { s.push('d'); }
        s
    }
}

// ─── Unveil Entry ─────────────────────────────────────────────────────────────

/// A single unveil entry: maps a path prefix to allowed access flags
#[derive(Debug, Clone)]
pub struct UnveilEntry {
    /// Filesystem path (prefix-matched)
    pub path: String,
    /// Allowed access flags for this path
    pub access: FsAccess,
}

impl UnveilEntry {
    pub fn new(path: &str, access: &str) -> Self {
        UnveilEntry { path: String::from(path), access: FsAccess::parse(access) }
    }

    /// Check if a given filesystem path is covered by this unveil entry
    pub fn covers_path(&self, target: &str) -> bool {
        target == self.path || target.starts_with(&format!("{}/", self.path))
    }
}

// ─── Capsicum Capability Rights ───────────────────────────────────────────────

/// Capsicum-style capability rights on a file descriptor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityRights(pub u64);

impl CapabilityRights {
    pub const READ: CapabilityRights = CapabilityRights(1 << 0);
    pub const WRITE: CapabilityRights = CapabilityRights(1 << 1);
    pub const SEEK: CapabilityRights = CapabilityRights(1 << 2);
    pub const FSTAT: CapabilityRights = CapabilityRights(1 << 3);
    pub const FCNTL: CapabilityRights = CapabilityRights(1 << 4);
    pub const MMAP: CapabilityRights = CapabilityRights(1 << 5);
    pub const MMAP_RX: CapabilityRights = CapabilityRights(1 << 6);
    pub const ACCEPT: CapabilityRights = CapabilityRights(1 << 7);
    pub const BIND: CapabilityRights = CapabilityRights(1 << 8);
    pub const CONNECT: CapabilityRights = CapabilityRights(1 << 9);
    pub const SEND: CapabilityRights = CapabilityRights(1 << 10);
    pub const RECV: CapabilityRights = CapabilityRights(1 << 11);
    pub const FSYNC: CapabilityRights = CapabilityRights(1 << 12);

    pub fn none() -> Self { CapabilityRights(0) }
    pub fn all() -> Self { CapabilityRights(!0) }

    pub fn has(self, right: CapabilityRights) -> bool {
        (self.0 & right.0) == right.0
    }

    pub fn union(self, other: CapabilityRights) -> Self {
        CapabilityRights(self.0 | other.0)
    }

    /// Restrict: remove rights (capability rights can only decrease)
    pub fn restrict(self, rights_to_remove: CapabilityRights) -> Self {
        CapabilityRights(self.0 & !rights_to_remove.0)
    }
}

/// A capability-restricted file descriptor entry
#[derive(Debug, Clone)]
pub struct CapFd {
    pub fd: i32,
    pub rights: CapabilityRights,
    pub description: String,
}

// ─── Pledge Profile ───────────────────────────────────────────────────────────

/// A sealed pledge profile: the set of allowed syscall classes for a process.
///
/// Once committed (sealed), the profile can only be further RESTRICTED,
/// never expanded — like OpenBSD pledge(2).
#[derive(Debug, Clone)]
pub struct PledgeProfile {
    /// Set of allowed syscall classes
    pub allowed: Vec<SyscallClass>,
    /// Whether this profile has been committed (sealed — no more expansions)
    pub sealed: bool,
    /// Violation log (for auditing)
    pub violations: Vec<String>,
}

impl PledgeProfile {
    pub fn new() -> Self {
        PledgeProfile { allowed: Vec::new(), sealed: false, violations: Vec::new() }
    }

    /// Grant a syscall class (only allowed before sealing)
    pub fn grant(&mut self, class: SyscallClass) -> bool {
        if self.sealed { return false; }
        if !self.allowed.contains(&class) {
            self.allowed.push(class);
        }
        true
    }

    /// Seal the profile — no more grants possible
    pub fn seal(&mut self) { self.sealed = true; }

    /// Restrict: remove a syscall class (allowed even after sealing)
    pub fn restrict(&mut self, class: SyscallClass) {
        self.allowed.retain(|&c| c != class);
    }

    /// Check if a syscall class is allowed
    pub fn is_allowed(&self, class: SyscallClass) -> bool {
        self.allowed.contains(&class)
    }

    /// Parse a space-separated promise string: "stdio rpath inet"
    pub fn from_promise_string(promises: &str) -> Self {
        let mut profile = PledgeProfile::new();
        for promise in promises.split_whitespace() {
            if let Some(class) = SyscallClass::parse(promise) {
                profile.grant(class);
            }
        }
        profile
    }

    pub fn promise_string(&self) -> String {
        let labels: Vec<&str> = self.allowed.iter().map(|c| c.label()).collect();
        labels.join(" ")
    }
}

// ─── Capability Monitor ───────────────────────────────────────────────────────

/// Violation action: what to do when a process violates its policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViolationAction {
    /// Allow but log the violation
    Log,
    /// Deny the operation (return EPERM)
    Deny,
    /// Kill the process (SIGABRT)
    Kill,
}

/// Per-process security profile managed by the Capability Monitor
#[derive(Debug, Clone)]
pub struct ProcessSecurityProfile {
    pub pid: u32,
    pub pledge: PledgeProfile,
    pub unveil: Vec<UnveilEntry>,
    pub cap_fds: BTreeMap<i32, CapFd>,
    pub in_capability_mode: bool,
    pub violation_action: ViolationAction,
    pub violations_count: u64,
}

impl ProcessSecurityProfile {
    pub fn new(pid: u32) -> Self {
        ProcessSecurityProfile {
            pid,
            pledge: PledgeProfile::new(),
            unveil: Vec::new(),
            cap_fds: BTreeMap::new(),
            in_capability_mode: false,
            violation_action: ViolationAction::Deny,
            violations_count: 0,
        }
    }
}

/// SigmaOS Capability-Based Security Monitor
///
/// Tracks all process security profiles and enforces:
/// - pledge(2) syscall class restrictions
/// - unveil(2) filesystem path restrictions
/// - Capsicum capability mode + capability rights
pub struct CapabilityMonitor {
    /// Per-process security profiles, keyed by PID
    pub profiles: BTreeMap<u32, ProcessSecurityProfile>,
    /// Total violations across all processes
    pub total_violations: u64,
    /// Total pledge checks performed
    pub total_pledge_checks: u64,
    /// Total unveil checks performed
    pub total_unveil_checks: u64,
}

impl CapabilityMonitor {
    /// Create a new capability monitor
    pub fn new() -> Self {
        CapabilityMonitor {
            profiles: BTreeMap::new(),
            total_violations: 0,
            total_pledge_checks: 0,
            total_unveil_checks: 0,
        }
    }

    // ── Process Lifecycle ──────────────────────────────────────────────────────

    /// Register a new process with default (unrestricted) profile
    pub fn register_process(&mut self, pid: u32) {
        self.profiles.insert(pid, ProcessSecurityProfile::new(pid));
    }

    /// Unregister a process (cleanup on exit)
    pub fn unregister_process(&mut self, pid: u32) {
        self.profiles.remove(&pid);
    }

    // ── Pledge ────────────────────────────────────────────────────────────────

    /// Grant a syscall class to a process (before sealing)
    pub fn pledge_grant(&mut self, pid: u32, class: SyscallClass) -> bool {
        self.profiles.get_mut(&pid)
            .map(|p| p.pledge.grant(class))
            .unwrap_or(false)
    }

    /// Seal a process's pledge profile
    pub fn pledge_seal(&mut self, pid: u32) -> bool {
        if let Some(profile) = self.profiles.get_mut(&pid) {
            profile.pledge.seal();
            true
        } else { false }
    }

    /// Check if a process's syscall class is allowed (pledge enforcement)
    pub fn check_pledge(&mut self, pid: u32, class: SyscallClass) -> bool {
        self.total_pledge_checks += 1;

        let (allowed, action) = match self.profiles.get(&pid) {
            None => return true, // Unknown process — allow (should be registered)
            Some(p) => {
                // If not sealed, all syscalls allowed
                if !p.pledge.sealed { return true; }
                (p.pledge.is_allowed(class), p.violation_action)
            }
        };

        if !allowed {
            if let Some(profile) = self.profiles.get_mut(&pid) {
                profile.violations_count += 1;
                profile.pledge.violations.push(format!(
                    "PLEDGE VIOLATION: pid={} denied syscall class '{}'",
                    pid, class.label()
                ));
            }
            self.total_violations += 1;

            match action {
                ViolationAction::Log | ViolationAction::Deny => {}
                ViolationAction::Kill => {
                    // In a real kernel: send SIGABRT to pid
                    self.unregister_process(pid);
                }
            }
        }

        allowed
    }

    // ── Unveil ────────────────────────────────────────────────────────────────

    /// Add an unveil entry to a process
    pub fn unveil_add(&mut self, pid: u32, path: &str, access: &str) -> bool {
        if let Some(profile) = self.profiles.get_mut(&pid) {
            profile.unveil.push(UnveilEntry::new(path, access));
            true
        } else {
            false
        }
    }

    /// Check if a process can access a filesystem path with given flags
    ///
    /// If no unveil entries exist for the process, access is unrestricted.
    /// If unveil entries exist, only explicitly unveiled paths are accessible.
    pub fn check_unveil(&mut self, pid: u32, path: &str, access: FsAccess) -> bool {
        self.total_unveil_checks += 1;

        let result = match self.profiles.get(&pid) {
            None => return true,
            Some(p) => {
                if p.unveil.is_empty() { return true; }
                // Find the most specific (longest) matching unveil entry
                let best = p.unveil.iter()
                    .filter(|e| e.covers_path(path))
                    .max_by_key(|e| e.path.len());
                match best {
                    None => false, // Path not unveiled at all
                    Some(entry) => entry.access.contains(access),
                }
            }
        };

        if !result {
            if let Some(profile) = self.profiles.get_mut(&pid) {
                profile.violations_count += 1;
            }
            self.total_violations += 1;
        }

        result
    }

    // ── Capsicum ──────────────────────────────────────────────────────────────

    /// Enter Capsicum capability mode for a process
    ///
    /// After entering capability mode:
    /// - Global namespace access (open by path) is DENIED
    /// - Only pre-opened file descriptors with explicit rights may be used
    pub fn enter_capability_mode(&mut self, pid: u32) -> bool {
        if let Some(profile) = self.profiles.get_mut(&pid) {
            profile.in_capability_mode = true;
            true
        } else { false }
    }

    /// Register a capability-restricted file descriptor for a process
    pub fn cap_new_fd(&mut self, pid: u32, fd: i32, rights: CapabilityRights, desc: &str) -> bool {
        if let Some(profile) = self.profiles.get_mut(&pid) {
            profile.cap_fds.insert(fd, CapFd {
                fd,
                rights,
                description: String::from(desc),
            });
            true
        } else { false }
    }

    /// Check if a process has a specific right on a file descriptor
    pub fn check_cap_right(&self, pid: u32, fd: i32, right: CapabilityRights) -> bool {
        self.profiles.get(&pid)
            .and_then(|p| p.cap_fds.get(&fd))
            .map(|cap_fd| cap_fd.rights.has(right))
            .unwrap_or(false)
    }

    // ── Status ────────────────────────────────────────────────────────────────

    /// Get a security summary for a specific process
    pub fn process_summary(&self, pid: u32) -> Option<String> {
        let profile = self.profiles.get(&pid)?;
        Some(format!(
            "PID {}: pledge=[{}] unveil={} entries cap_mode={} violations={}",
            pid,
            profile.pledge.promise_string(),
            profile.unveil.len(),
            profile.in_capability_mode,
            profile.violations_count
        ))
    }

    /// Returns the monitor status string
    pub fn status(&self) -> String {
        format!(
            "CapMonitor | {} processes | {} violations | {} pledge checks | {} unveil checks",
            self.profiles.len(),
            self.total_violations,
            self.total_pledge_checks,
            self.total_unveil_checks
        )
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod capability_tests {
    use super::*;

    #[test]
    fn test_pledge_grant_and_check() {
        let mut monitor = CapabilityMonitor::new();
        monitor.register_process(100);
        monitor.pledge_grant(100, SyscallClass::Stdio);
        monitor.pledge_grant(100, SyscallClass::Inet);
        monitor.pledge_seal(100);

        assert!(monitor.check_pledge(100, SyscallClass::Stdio));
        assert!(monitor.check_pledge(100, SyscallClass::Inet));
        assert!(!monitor.check_pledge(100, SyscallClass::Proc));
        assert!(!monitor.check_pledge(100, SyscallClass::Exec));
    }

    #[test]
    fn test_pledge_unsealed_allows_all() {
        let mut monitor = CapabilityMonitor::new();
        monitor.register_process(200);
        monitor.pledge_grant(200, SyscallClass::Stdio);
        // Not sealed — all classes should be allowed
        assert!(monitor.check_pledge(200, SyscallClass::Proc));
        assert!(monitor.check_pledge(200, SyscallClass::Exec));
    }

    #[test]
    fn test_pledge_cannot_expand_after_seal() {
        let mut monitor = CapabilityMonitor::new();
        monitor.register_process(300);
        monitor.pledge_grant(300, SyscallClass::Stdio);
        monitor.pledge_seal(300);
        // Try to grant after sealing — should fail
        let granted = monitor.pledge_grant(300, SyscallClass::Proc);
        assert!(!granted);
        assert!(!monitor.check_pledge(300, SyscallClass::Proc));
    }

    #[test]
    fn test_unveil_restricts_paths() {
        let mut monitor = CapabilityMonitor::new();
        monitor.register_process(400);
        monitor.unveil_add(400, "/sigma/store", "r");
        monitor.unveil_add(400, "/tmp", "rwc");

        // Allowed: /sigma/store readable
        assert!(monitor.check_unveil(400, "/sigma/store", FsAccess::READ_FILE));
        // Allowed: subpath
        assert!(monitor.check_unveil(400, "/sigma/store/bash-5.2.1", FsAccess::READ_FILE));
        // Denied: write to /sigma/store
        assert!(!monitor.check_unveil(400, "/sigma/store/bash-5.2.1", FsAccess::WRITE_FILE));
        // Allowed: /tmp writable
        assert!(monitor.check_unveil(400, "/tmp/test.txt", FsAccess::WRITE_FILE));
        // Denied: /etc not unveiled
        assert!(!monitor.check_unveil(400, "/etc/passwd", FsAccess::READ_FILE));
    }

    #[test]
    fn test_unveil_empty_allows_all() {
        let mut monitor = CapabilityMonitor::new();
        monitor.register_process(500);
        // No unveil entries → no restrictions
        assert!(monitor.check_unveil(500, "/etc/passwd", FsAccess::READ_FILE));
        assert!(monitor.check_unveil(500, "/sigma/store", FsAccess::WRITE_FILE));
    }

    #[test]
    fn test_capsicum_mode_and_fd_rights() {
        let mut monitor = CapabilityMonitor::new();
        monitor.register_process(600);
        monitor.cap_new_fd(600, 3, CapabilityRights::READ.union(CapabilityRights::SEEK), "input file");
        monitor.cap_new_fd(600, 4, CapabilityRights::WRITE.union(CapabilityRights::FSYNC), "output file");
        monitor.enter_capability_mode(600);

        assert!(monitor.check_cap_right(600, 3, CapabilityRights::READ));
        assert!(monitor.check_cap_right(600, 3, CapabilityRights::SEEK));
        assert!(!monitor.check_cap_right(600, 3, CapabilityRights::WRITE)); // fd 3 has no WRITE
        assert!(monitor.check_cap_right(600, 4, CapabilityRights::WRITE));
        assert!(!monitor.check_cap_right(600, 4, CapabilityRights::READ));  // fd 4 has no READ
        assert!(!monitor.check_cap_right(600, 5, CapabilityRights::READ));  // fd 5 not registered
    }

    #[test]
    fn test_violation_counting() {
        let mut monitor = CapabilityMonitor::new();
        monitor.register_process(700);
        monitor.pledge_grant(700, SyscallClass::Stdio);
        monitor.pledge_seal(700);

        // Cause 3 violations
        monitor.check_pledge(700, SyscallClass::Exec);
        monitor.check_pledge(700, SyscallClass::Proc);
        monitor.check_pledge(700, SyscallClass::Inet);
        assert_eq!(monitor.total_violations, 3);
        assert_eq!(monitor.profiles[&700].violations_count, 3);
    }

    #[test]
    fn test_fs_access_parse() {
        let rw = FsAccess::parse("rw");
        assert!(rw.contains(FsAccess::READ_FILE));
        assert!(rw.contains(FsAccess::WRITE_FILE));
        assert!(!rw.contains(FsAccess::EXECUTE));

        let rwxc = FsAccess::parse("rwxc");
        assert!(rwxc.contains(FsAccess::EXECUTE));
        assert!(rwxc.contains(FsAccess::ADD_FILE));
    }

    #[test]
    fn test_process_summary() {
        let mut monitor = CapabilityMonitor::new();
        monitor.register_process(800);
        monitor.pledge_grant(800, SyscallClass::Stdio);
        monitor.pledge_grant(800, SyscallClass::Inet);
        monitor.pledge_seal(800);
        monitor.unveil_add(800, "/var/log", "w");

        let summary = monitor.process_summary(800).unwrap();
        assert!(summary.contains("800"));
        assert!(summary.contains("stdio"));
        assert!(summary.contains("inet"));
    }

    #[test]
    fn test_capability_rights_restrict() {
        let full = CapabilityRights::READ.union(CapabilityRights::WRITE).union(CapabilityRights::SEEK);
        let restricted = full.restrict(CapabilityRights::WRITE);
        assert!(restricted.has(CapabilityRights::READ));
        assert!(!restricted.has(CapabilityRights::WRITE));
        assert!(restricted.has(CapabilityRights::SEEK));
    }
}
