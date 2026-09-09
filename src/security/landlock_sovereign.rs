#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unexpected_cfgs)]
#![allow(clippy::new_without_default)]

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

// SigmaOS Sovereign Landlock Filesystem Sandboxing
// Implements Linux Landlock v5 + OpenBSD unveil + FreeBSD Capsicum hybrid
// in 100% safe Rust with no external dependencies.
//
// Landlock was mainlined in Linux 5.13 (June 2021). This is a pure-Rust
// in-kernel reference implementation of the access-control matrix.


#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

// ─── Landlock Access Rights Bitmask (mirrors LANDLOCK_ACCESS_FS_*) ────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LandlockFsRights(u32);

impl LandlockFsRights {
    pub const EXECUTE:         LandlockFsRights = LandlockFsRights(1 << 0);
    pub const WRITE_FILE:      LandlockFsRights = LandlockFsRights(1 << 1);
    pub const READ_FILE:       LandlockFsRights = LandlockFsRights(1 << 2);
    pub const READ_DIR:        LandlockFsRights = LandlockFsRights(1 << 3);
    pub const REMOVE_DIR:      LandlockFsRights = LandlockFsRights(1 << 4);
    pub const REMOVE_FILE:     LandlockFsRights = LandlockFsRights(1 << 5);
    pub const MAKE_CHAR:       LandlockFsRights = LandlockFsRights(1 << 6);
    pub const MAKE_DIR:        LandlockFsRights = LandlockFsRights(1 << 7);
    pub const MAKE_REG:        LandlockFsRights = LandlockFsRights(1 << 8);
    pub const MAKE_SOCK:       LandlockFsRights = LandlockFsRights(1 << 9);
    pub const MAKE_FIFO:       LandlockFsRights = LandlockFsRights(1 << 10);
    pub const MAKE_BLOCK:      LandlockFsRights = LandlockFsRights(1 << 11);
    pub const MAKE_SYM:        LandlockFsRights = LandlockFsRights(1 << 12);
    pub const REFER:           LandlockFsRights = LandlockFsRights(1 << 13);
    pub const TRUNCATE:        LandlockFsRights = LandlockFsRights(1 << 14);
    pub const IOCTL_DEV:       LandlockFsRights = LandlockFsRights(1 << 15);
    // v5 additions
    pub const BIND_TCP:        LandlockFsRights = LandlockFsRights(1 << 16);
    pub const CONNECT_TCP:     LandlockFsRights = LandlockFsRights(1 << 17);

    pub const READ_ONLY: LandlockFsRights = LandlockFsRights(
        (1 << 2) | (1 << 3)
    );
    pub const READ_WRITE: LandlockFsRights = LandlockFsRights(
        (1 << 1) | (1 << 2) | (1 << 3)
    );
    pub const NONE: LandlockFsRights = LandlockFsRights(0);

    pub fn contains(self, other: LandlockFsRights) -> bool {
        (self.0 & other.0) == other.0
    }

    pub fn union(self, other: LandlockFsRights) -> LandlockFsRights {
        LandlockFsRights(self.0 | other.0)
    }

    pub fn bits(self) -> u32 { self.0 }
}

// ─── Path Rule (mirrors landlock_path_beneath_attr) ──────────────────────────

#[derive(Debug, Clone)]
pub struct LandlockPathRule {
    pub path: String,
    pub allowed_access: LandlockFsRights,
    pub recursive: bool,
}

impl LandlockPathRule {
    pub fn new(path: &str, allowed_access: LandlockFsRights, recursive: bool) -> Self {
        LandlockPathRule {
            path: path.to_string(),
            allowed_access,
            recursive,
        }
    }

    /// Returns true if the given target path falls under this rule's scope.
    pub fn matches(&self, target: &str) -> bool {
        if self.recursive {
            target == self.path || target.starts_with(&{
                let mut p = self.path.clone();
                if !p.ends_with('/') { p.push('/'); }
                p
            })
        } else {
            target == self.path
        }
    }
}

// ─── OpenBSD-style unveil state ──────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum UnveilPermission {
    Read,
    Write,
    Execute,
    Create,
}

#[derive(Debug, Clone)]
pub struct UnveilEntry {
    pub path: String,
    pub permissions: Vec<UnveilPermission>,
}

impl UnveilEntry {
    pub fn new(path: &str, perms: &str) -> Self {
        let mut permissions = Vec::new();
        for ch in perms.chars() {
            match ch {
                'r' => permissions.push(UnveilPermission::Read),
                'w' => permissions.push(UnveilPermission::Write),
                'x' => permissions.push(UnveilPermission::Execute),
                'c' => permissions.push(UnveilPermission::Create),
                _   => {}
            }
        }
        UnveilEntry { path: path.to_string(), permissions }
    }

    pub fn has(&self, perm: &UnveilPermission) -> bool {
        self.permissions.contains(perm)
    }
}

// ─── Sovereign Landlock Sandbox ───────────────────────────────────────────────

#[derive(Debug, PartialEq)]
pub enum SandboxState {
    Building,
    Enforced,
    Violated,
}

pub struct SovereignLandlockV5Guard {
    pub ruleset_version: u8,
    pub path_rules: Vec<LandlockPathRule>,
    pub unveil_entries: Vec<UnveilEntry>,
    pub state: SandboxState,
    pub violation_count: u64,
    pub violation_log: Vec<String>,
    pub no_new_privs: bool,
}

impl SovereignLandlockV5Guard {
    pub fn new(version: u8) -> Self {
        SovereignLandlockV5Guard {
            ruleset_version: version,
            path_rules: Vec::new(),
            unveil_entries: Vec::new(),
            state: SandboxState::Building,
            violation_count: 0,
            violation_log: Vec::new(),
            no_new_privs: false,
        }
    }

    /// Add a Landlock path-beneath rule
    pub fn add_rule(&mut self, rule: LandlockPathRule) -> bool {
        if self.state != SandboxState::Building { return false; }
        self.path_rules.push(rule);
        true
    }

    /// Add an OpenBSD-style unveil entry
    pub fn unveil(&mut self, path: &str, perms: &str) -> bool {
        if self.state != SandboxState::Building { return false; }
        self.unveil_entries.push(UnveilEntry::new(path, perms));
        true
    }

    /// Enforce the sandbox — transitions to Enforced state.
    /// After this, no new rules can be added (matches Landlock behavior).
    pub fn enforce(&mut self, no_new_privs: bool) -> bool {
        if self.state != SandboxState::Building { return false; }
        self.no_new_privs = no_new_privs;
        self.state = SandboxState::Enforced;
        true
    }

    /// Check if a filesystem access is allowed under current sandbox.
    pub fn check_fs_access(&mut self, path: &str, rights: LandlockFsRights) -> bool {
        if self.state == SandboxState::Building { return true; } // Not yet enforced

        // Find the most-specific matching rule
        let mut best_match: Option<&LandlockPathRule> = None;
        let mut best_len = 0usize;

        for rule in &self.path_rules {
            if rule.matches(path) && rule.path.len() >= best_len {
                best_len = rule.path.len();
                best_match = Some(rule);
            }
        }

        let allowed = match best_match {
            Some(rule) => rule.allowed_access.contains(rights),
            None       => false, // Default-deny if no rule matches
        };

        if !allowed {
            self.record_violation(path, rights);
        }
        allowed
    }

    /// Check unveil permissions
    pub fn check_unveil(&self, path: &str, perm: &UnveilPermission) -> bool {
        if self.state == SandboxState::Building { return true; }

        for entry in &self.unveil_entries {
            if path.starts_with(&entry.path) {
                return entry.has(perm);
            }
        }
        false // Default-deny
    }

    fn record_violation(&mut self, path: &str, rights: LandlockFsRights) {
        self.state = SandboxState::Violated;
        self.violation_count = self.violation_count.saturating_add(1);
        let mut msg = String::from("LANDLOCK_DENY path=");
        msg.push_str(path);
        msg.push_str(" rights=0x");
        // Simple hex representation
        let bits = rights.bits();
        for nibble in (0..8).rev() {
            let n = (bits >> (nibble * 4)) & 0xF;
            let ch = if n < 10 { b'0' + n as u8 } else { b'a' + (n - 10) as u8 };
            msg.push(ch as char);
        }
        if self.violation_log.len() < 256 {
            self.violation_log.push(msg);
        }
    }

    pub fn violation_summary(&self) -> String {
        let mut s = String::from("Landlock violations: ");
        s.push_str(&self.violation_count.to_string());
        s.push('\n');
        for v in self.violation_log.iter().take(10) {
            s.push_str("  ");
            s.push_str(v);
            s.push('\n');
        }
        s
    }
}

// ─── FreeBSD Capsicum descriptor rights (simplified) ─────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapsicumRights(u64);

impl CapsicumRights {
    pub const CAP_READ:     CapsicumRights = CapsicumRights(1 << 0);
    pub const CAP_WRITE:    CapsicumRights = CapsicumRights(1 << 1);
    pub const CAP_SEEK:     CapsicumRights = CapsicumRights(1 << 2);
    pub const CAP_FCNTL:    CapsicumRights = CapsicumRights(1 << 3);
    pub const CAP_FSTAT:    CapsicumRights = CapsicumRights(1 << 4);
    pub const CAP_MMAP:     CapsicumRights = CapsicumRights(1 << 5);
    pub const CAP_FTRUNCATE:CapsicumRights = CapsicumRights(1 << 6);
    pub const CAP_CONNECT:  CapsicumRights = CapsicumRights(1 << 7);
    pub const CAP_ACCEPT:   CapsicumRights = CapsicumRights(1 << 8);
    pub const NONE:         CapsicumRights = CapsicumRights(0);

    pub fn contains(self, other: CapsicumRights) -> bool {
        (self.0 & other.0) == other.0
    }

    pub fn restrict(self, mask: CapsicumRights) -> CapsicumRights {
        CapsicumRights(self.0 & mask.0)
    }
}

#[derive(Debug, Clone)]
pub struct CapsicumFdDescriptor {
    pub fd: u32,
    pub rights: CapsicumRights,
    pub in_capability_mode: bool,
}

impl CapsicumFdDescriptor {
    pub fn new(fd: u32, rights: CapsicumRights) -> Self {
        CapsicumFdDescriptor { fd, rights, in_capability_mode: false }
    }

    /// Enter capability mode — cap_enter() equivalent
    pub fn enter_capability_mode(&mut self) {
        self.in_capability_mode = true;
    }

    /// Check if operation is allowed
    pub fn check(&self, required: CapsicumRights) -> bool {
        if !self.in_capability_mode { return true; }
        self.rights.contains(required)
    }

    /// Reduce rights (cannot be expanded once reduced)
    pub fn limit_rights(&mut self, new_rights: CapsicumRights) {
        self.rights = self.rights.restrict(new_rights);
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_landlock_path_rule_matching() {
        let rule = LandlockPathRule::new("/home/user", LandlockFsRights::READ_ONLY, true);
        assert!(rule.matches("/home/user"));
        assert!(rule.matches("/home/user/docs/file.txt"));
        assert!(!rule.matches("/home/other"));
        assert!(!rule.matches("/var/log"));
    }

    #[test]
    fn test_landlock_sandbox_enforce_and_check() {
        let mut guard = SovereignLandlockV5Guard::new(5);
        guard.add_rule(LandlockPathRule::new("/etc", LandlockFsRights::READ_ONLY, true));
        guard.add_rule(LandlockPathRule::new("/tmp", LandlockFsRights::READ_WRITE, true));
        guard.enforce(true);

        assert!(guard.check_fs_access("/etc/hostname", LandlockFsRights::READ_FILE));
        assert!(!guard.check_fs_access("/etc/passwd", LandlockFsRights::WRITE_FILE));
        assert!(guard.check_fs_access("/tmp/scratch", LandlockFsRights::WRITE_FILE));
        assert!(!guard.check_fs_access("/var/log/syslog", LandlockFsRights::READ_FILE));
    }

    #[test]
    fn test_landlock_violation_logging() {
        let mut guard = SovereignLandlockV5Guard::new(5);
        guard.enforce(true);
        guard.check_fs_access("/secret", LandlockFsRights::READ_FILE);
        assert_eq!(guard.violation_count, 1);
        assert!(!guard.violation_log.is_empty());
    }

    #[test]
    fn test_unveil_permissions() {
        let mut guard = SovereignLandlockV5Guard::new(5);
        guard.unveil("/home/user", "rw");
        guard.unveil("/usr/bin", "rx");
        guard.enforce(true);

        assert!(guard.check_unveil("/home/user/file", &UnveilPermission::Read));
        assert!(guard.check_unveil("/home/user/file", &UnveilPermission::Write));
        assert!(!guard.check_unveil("/home/user/file", &UnveilPermission::Execute));
        assert!(!guard.check_unveil("/etc/passwd", &UnveilPermission::Read));
    }

    #[test]
    fn test_capsicum_capability_mode() {
        let mut fd = CapsicumFdDescriptor::new(
            3,
            CapsicumRights::CAP_READ.restrict(
                CapsicumRights(CapsicumRights::CAP_READ.0 | CapsicumRights::CAP_FSTAT.0)
            )
        );
        // Before capability mode: all allowed
        assert!(fd.check(CapsicumRights::CAP_WRITE));
        fd.enter_capability_mode();
        // After: only CAP_READ and CAP_FSTAT allowed
        assert!(fd.check(CapsicumRights::CAP_READ));
        assert!(!fd.check(CapsicumRights::CAP_WRITE));
    }

    #[test]
    fn test_rights_restriction_monotonic() {
        let mut fd = CapsicumFdDescriptor::new(
            5,
            CapsicumRights(CapsicumRights::CAP_READ.0 | CapsicumRights::CAP_WRITE.0 | CapsicumRights::CAP_SEEK.0)
        );
        fd.enter_capability_mode();
        fd.limit_rights(CapsicumRights(CapsicumRights::CAP_READ.0 | CapsicumRights::CAP_SEEK.0));
        assert!(fd.check(CapsicumRights::CAP_READ));
        assert!(!fd.check(CapsicumRights::CAP_WRITE)); // restricted away
    }
}
