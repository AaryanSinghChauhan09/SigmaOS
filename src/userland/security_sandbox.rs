// OpenBSD & HardenedBSD Inspired Userland Security Sandbox
// Location: src/userland/security_sandbox.rs
//
// Provides OpenBSD pledge() promiscuous capability restriction,
// unveil() filesystem path sandboxing, and HardenedBSD PaX W^X memory security enforcement.

use std::collections::{BTreeMap, BTreeSet};
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// OpenBSD pledge() syscall capability categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PledgeCapability {
    StdIo,
    RPath,
    WPath,
    CPath,
    INet,
    Unix,
    Dns,
    Proc,
    Exec,
    ProtExec,
}

impl PledgeCapability {
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "stdio" => Some(Self::StdIo),
            "rpath" => Some(Self::RPath),
            "wpath" => Some(Self::WPath),
            "cpath" => Some(Self::CPath),
            "inet" => Some(Self::INet),
            "unix" => Some(Self::Unix),
            "dns" => Some(Self::Dns),
            "proc" => Some(Self::Proc),
            "exec" => Some(Self::Exec),
            "protexec" => Some(Self::ProtExec),
            _ => None,
        }
    }
}

/// OpenBSD unveil() filesystem permissions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnveilPermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub create: bool,
}

impl UnveilPermissions {
    pub fn parse(perms: &str) -> Self {
        Self {
            read: perms.contains('r'),
            write: perms.contains('w'),
            execute: perms.contains('x'),
            create: perms.contains('c'),
        }
    }
}

pub struct UserlandSecuritySandbox {
    pub process_id: u64,
    pub active_pledges: Option<BTreeSet<PledgeCapability>>,
    pub unveiled_paths: BTreeMap<String, UnveilPermissions>,
    pub wx_enforced: bool,
}

impl UserlandSecuritySandbox {
    pub fn new(pid: u64) -> Self {
        Self {
            process_id: pid,
            active_pledges: None, // None means unpledged (full capabilities)
            unveiled_paths: BTreeMap::new(),
            wx_enforced: true,
        }
    }

    /// OpenBSD pledge() capability restriction implementation.
    /// Can only restrict capabilities; capabilities cannot be re-granted once dropped.
    pub fn pledge(&mut self, promises: &str) -> Result<(), &'static str> {
        let mut new_set = BTreeSet::new();
        for token in promises.split_whitespace() {
            if let Some(cap) = PledgeCapability::parse(token) {
                new_set.insert(cap);
            } else {
                return Err("Invalid pledge capability specified");
            }
        }

        if let Some(ref current) = self.active_pledges {
            // Subset check: new pledge set must be a subset of active set
            if !new_set.is_subset(current) {
                return Err("pledge error: attempted to expand active capability set");
            }
        }

        self.active_pledges = Some(new_set);
        Ok(())
    }

    /// OpenBSD unveil() path restriction system implementation.
    pub fn unveil(&mut self, path: &str, perms: &str) -> Result<(), &'static str> {
        if path.is_empty() {
            return Err("Path cannot be empty");
        }
        let parsed_perms = UnveilPermissions::parse(perms);
        self.unveiled_paths.insert(path.to_string(), parsed_perms);
        Ok(())
    }

    /// Verifies if a process is authorized to perform a system operation given its pledge.
    pub fn check_pledge(&self, required: PledgeCapability) -> bool {
        match self.active_pledges {
            None => true, // Unpledged
            Some(ref set) => set.contains(&required),
        }
    }

    /// Verifies if a path access is allowed under active unveil rules.
    pub fn check_unveil(&self, path: &str, access_kind: &str) -> bool {
        if self.unveiled_paths.is_empty() {
            return true; // Unveiled system allows default access
        }

        for (unveiled_path, perms) in &self.unveiled_paths {
            if path.starts_with(unveiled_path) {
                match access_kind {
                    "r" => return perms.read,
                    "w" => return perms.write,
                    "x" => return perms.execute,
                    "c" => return perms.create,
                    _ => return false,
                }
            }
        }
        false
    }

    /// HardenedBSD W^X memory protection check.
    pub fn check_memory_protection(&self, is_writable: bool, is_executable: bool) -> bool {
        if self.wx_enforced {
            !(is_writable && is_executable)
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openbsd_pledge_sandbox() {
        let mut sandbox = UserlandSecuritySandbox::new(101);
        assert!(sandbox.check_pledge(PledgeCapability::Exec));

        // Restrict process to stdio, rpath, and exec
        assert!(sandbox.pledge("stdio rpath exec").is_ok());
        assert!(sandbox.check_pledge(PledgeCapability::StdIo));
        assert!(sandbox.check_pledge(PledgeCapability::RPath));
        assert!(sandbox.check_pledge(PledgeCapability::Exec));
        assert!(!sandbox.check_pledge(PledgeCapability::INet)); // INet dropped

        // Attempting to re-grant INet should fail
        assert!(sandbox.pledge("stdio rpath exec inet").is_err());
    }

    #[test]
    fn test_openbsd_unveil_sandbox() {
        let mut sandbox = UserlandSecuritySandbox::new(102);
        assert!(sandbox.unveil("/usr/lib", "r").is_ok());
        assert!(sandbox.unveil("/tmp", "rwc").is_ok());

        assert!(sandbox.check_unveil("/usr/lib/libc.so", "r"));
        assert!(!sandbox.check_unveil("/usr/lib/libc.so", "w"));
        assert!(sandbox.check_unveil("/tmp/scratch.txt", "w"));
        assert!(!sandbox.check_unveil("/etc/shadow", "r"));
    }

    #[test]
    fn test_hardenedbsd_wx_memory_enforcement() {
        let sandbox = UserlandSecuritySandbox::new(103);
        assert!(sandbox.check_memory_protection(true, false));  // Writable only
        assert!(sandbox.check_memory_protection(false, true));  // Executable only
        assert!(!sandbox.check_memory_protection(true, true));  // W^X violation!
    }
}
