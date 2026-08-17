// OpenBSD-Inspired pledge() and unveil() Security Restriction Sandbox
// Zero-dependency, safe Rust security policy engine for process restriction

use std::collections::HashMap;

/// OpenBSD pledge promises restricting system call categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PledgePromise {
    Stdio,     // Basic I/O (read, write, fstat, etc.)
    Rpath,     // Read-only filesystem operations
    Wpath,     // Write filesystem operations
    Cpath,     // File creation/deletion (open O_CREAT, unlink, mkdir)
    Dpath,     // Device file operations (mknod, etc.)
    Inet,      // Network socket creation and operations (AF_INET/AF_INET6)
    Unix,      // UNIX domain socket operations
    Dns,       // DNS resolution capabilities
    Exec,      // Executing new binaries (execve)
    Proc,      // Process creation and signaling (fork, kill, wait)
    ProtExec,  // Memory protection with PROT_EXEC
}

impl PledgePromise {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "stdio" => Some(PledgePromise::Stdio),
            "rpath" => Some(PledgePromise::Rpath),
            "wpath" => Some(PledgePromise::Wpath),
            "cpath" => Some(PledgePromise::Cpath),
            "dpath" => Some(PledgePromise::Dpath),
            "inet" => Some(PledgePromise::Inet),
            "unix" => Some(PledgePromise::Unix),
            "dns" => Some(PledgePromise::Dns),
            "exec" => Some(PledgePromise::Exec),
            "proc" => Some(PledgePromise::Proc),
            "protexec" => Some(PledgePromise::ProtExec),
            _ => None,
        }
    }
}

/// OpenBSD unveil permission flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnveilPerm {
    Read,
    Write,
    Execute,
    Create,
    None,
}

/// OpenBSD pledge process sandbox
#[derive(Debug, Clone)]
pub struct PledgeSandbox {
    pub active_promises: Vec<PledgePromise>,
    pub is_pledged: bool,
    pub violation_count: usize,
}

impl PledgeSandbox {
    pub fn new() -> Self {
        Self {
            active_promises: Vec::new(),
            is_pledged: false,
            violation_count: 0,
        }
    }

    /// Set process pledge promises. Once pledged, promises can ONLY be reduced (subset).
    pub fn pledge(&mut self, promises: &[PledgePromise]) -> Result<(), &'static str> {
        if self.is_pledged {
            // Ensure new promises are a strict subset of existing promises
            for &p in promises {
                if !self.active_promises.contains(&p) {
                    self.violation_count += 1;
                    return Err("Pledge Violation: Attempted to expand pledged promises!");
                }
            }
        }
        self.active_promises = promises.to_vec();
        self.is_pledged = true;
        Ok(())
    }

    /// Check if a given system operation is permitted under active promises
    pub fn check_permission(&self, promise: PledgePromise) -> bool {
        if !self.is_pledged {
            return true; // Not pledged -> unrestricted
        }
        self.active_promises.contains(&promise)
    }
}

impl Default for PledgeSandbox {
    fn default() -> Self {
        Self::new()
    }
}

/// OpenBSD unveil filesystem sandbox
#[derive(Debug, Clone)]
pub struct UnveilSandbox {
    pub permissions: HashMap<String, Vec<UnveilPerm>>,
    pub is_locked: bool,
}

impl UnveilSandbox {
    pub fn new() -> Self {
        Self {
            permissions: HashMap::new(),
            is_locked: false,
        }
    }

    /// Restrict access to path with permissions string ("r", "rw", "rx", "c", etc.)
    pub fn unveil(&mut self, path: &str, perms_str: &str) -> Result<(), &'static str> {
        if self.is_locked {
            return Err("Unveil Sandbox is locked! Cannot modify path unveils.");
        }

        let mut perms = Vec::new();
        for ch in perms_str.chars() {
            match ch {
                'r' => perms.push(UnveilPerm::Read),
                'w' => perms.push(UnveilPerm::Write),
                'x' => perms.push(UnveilPerm::Execute),
                'c' => perms.push(UnveilPerm::Create),
                _ => {}
            }
        }
        if perms.is_empty() {
            perms.push(UnveilPerm::None);
        }

        self.permissions.insert(path.to_string(), perms);
        Ok(())
    }

    /// Lock unveil configuration permanently for current process
    pub fn lock(&mut self) {
        self.is_locked = true;
    }

    /// Check if path access is permitted for specific operation
    pub fn is_allowed(&self, path: &str, required_perm: UnveilPerm) -> bool {
        if self.permissions.is_empty() {
            return true; // No unveils set -> unrestricted
        }

        for (unveiled_path, perms) in &self.permissions {
            if path.starts_with(unveiled_path) {
                if perms.contains(&UnveilPerm::None) {
                    return false;
                }
                return perms.contains(&required_perm);
            }
        }
        false
    }
}

impl Default for UnveilSandbox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pledge_restriction() {
        let mut sandbox = PledgeSandbox::new();
        assert!(sandbox.check_permission(PledgePromise::Inet));

        sandbox.pledge(&[PledgePromise::Stdio, PledgePromise::Rpath]).unwrap();
        assert!(sandbox.check_permission(PledgePromise::Stdio));
        assert!(sandbox.check_permission(PledgePromise::Rpath));
        assert!(!sandbox.check_permission(PledgePromise::Inet));

        // Attempting to expand promises should fail
        assert!(sandbox.pledge(&[PledgePromise::Stdio, PledgePromise::Inet]).is_err());
    }

    #[test]
    fn test_unveil_filesystem_restrictions() {
        let mut sandbox = UnveilSandbox::new();
        sandbox.unveil("/tmp", "rw").unwrap();
        sandbox.unveil("/etc", "r").unwrap();

        assert!(sandbox.is_allowed("/tmp/file.txt", UnveilPerm::Read));
        assert!(sandbox.is_allowed("/tmp/file.txt", UnveilPerm::Write));
        assert!(!sandbox.is_allowed("/tmp/file.txt", UnveilPerm::Execute));

        assert!(sandbox.is_allowed("/etc/hosts", UnveilPerm::Read));
        assert!(!sandbox.is_allowed("/etc/hosts", UnveilPerm::Write));
        assert!(!sandbox.is_allowed("/usr/bin/ls", UnveilPerm::Read));
    }
}
