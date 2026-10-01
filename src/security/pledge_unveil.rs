// Pledge/Unveil Sandbox Implementation for SigmaOS
// OpenBSD-inspired pledge/unveil for process isolation and file access restriction
// Provides defense-in-depth security through capability restriction

use crate::klib::HashMap;
use std::string::{String, ToString};
use std::vec::Vec;

/// Pledge promises - capabilities that a process can request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PledgePromise {
    Stdio,
    Rpath,
    Wpath,
    Cpath,
    Fattr,
    Chown,
    Flock,
    Unix,
    Dns,
    TTY,
    Exec,
    Proc,
    Id,
    Settime,
    Psync,
    Network,
    Audio,
    Video,
}

impl PledgePromise {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "stdio" => Some(PledgePromise::Stdio),
            "rpath" => Some(PledgePromise::Rpath),
            "wpath" => Some(PledgePromise::Wpath),
            "cpath" => Some(PledgePromise::Cpath),
            "fattr" => Some(PledgePromise::Fattr),
            "chown" => Some(PledgePromise::Chown),
            "flock" => Some(PledgePromise::Flock),
            "unix" => Some(PledgePromise::Unix),
            "dns" => Some(PledgePromise::Dns),
            "tty" => Some(PledgePromise::TTY),
            "exec" => Some(PledgePromise::Exec),
            "proc" => Some(PledgePromise::Proc),
            "id" => Some(PledgePromise::Id),
            "settime" => Some(PledgePromise::Settime),
            "psync" => Some(PledgePromise::Psync),
            "network" => Some(PledgePromise::Network),
            "audio" => Some(PledgePromise::Audio),
            "video" => Some(PledgePromise::Video),
            _ => None,
        }
    }
}

/// Pledge sandbox state
#[derive(Debug, Clone)]
pub struct PledgeSandbox {
    pub promises: Vec<PledgePromise>,
    pub pledged: bool,
}

impl PledgeSandbox {
    pub fn new() -> Self {
        PledgeSandbox {
            promises: Vec::new(),
            pledged: false,
        }
    }

    pub fn pledge(&mut self, promises_str: &str) -> bool {
        if self.promised() {
            return false; // Already pledged
        }

        let mut new_promises = Vec::new();
        for promise_str in promises_str.split_whitespace() {
            if let Some(promise) = PledgePromise::from_str(promise_str) {
                new_promises.push(promise);
            }
        }

        self.promises = new_promises;
        self.pledged = true;
        true
    }

    pub fn promised(&self) -> bool {
        self.pledged
    }

    pub fn has_promise(&self, promise: PledgePromise) -> bool {
        self.promises.contains(&promise)
    }

    pub fn check_promise(&self, promise: PledgePromise) -> bool {
        if !self.pledged {
            return true; // Not pledged yet, allow everything
        }
        self.has_promise(promise)
    }
}

impl Default for PledgeSandbox {
    fn default() -> Self {
        Self::new()
    }
}

/// Unveil path permissions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnveilPermission {
    Read,
    Write,
    Execute,
    ReadWrite,
}

impl UnveilPermission {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "r" => Some(UnveilPermission::Read),
            "w" => Some(UnveilPermission::Write),
            "x" => Some(UnveilPermission::Execute),
            "rw" => Some(UnveilPermission::ReadWrite),
            _ => None,
        }
    }
}

/// Unveil path entry
#[derive(Debug, Clone)]
pub struct UnveilPath {
    pub path: String,
    pub permissions: UnveilPermission,
}

/// Unveil sandbox state
#[derive(Debug, Clone)]
pub struct UnveilSandbox {
    pub paths: HashMap<String, UnveilPermission>,
    pub unveiled: bool,
    pub default_deny: bool,
}

impl UnveilSandbox {
    pub fn new() -> Self {
        UnveilSandbox {
            paths: HashMap::new(),
            unveiled: false,
            default_deny: false,
        }
    }

    pub fn unveil(&mut self, path: &str, permissions: &str) -> bool {
        if let Some(perm) = UnveilPermission::from_str(permissions) {
            self.paths.insert(String::from(path), perm);
            self.unveiled = true;
            self.default_deny = true;
            true
        } else {
            false
        }
    }

    pub fn set_default_deny(&mut self, deny: bool) {
        self.default_deny = deny;
    }

    pub fn unveiled(&self) -> bool {
        self.unveiled
    }

    pub fn check_path_access(&self, path: &str, permission: UnveilPermission) -> bool {
        if !self.unveiled {
            return true; // Not unveiled yet, allow everything
        }

        // Check exact path match
        if let Some(perm) = self.paths.get(path) {
            return match permission {
                UnveilPermission::Read => {
                    *perm == UnveilPermission::Read || *perm == UnveilPermission::ReadWrite
                }
                UnveilPermission::Write => {
                    *perm == UnveilPermission::Write || *perm == UnveilPermission::ReadWrite
                }
                UnveilPermission::Execute => *perm == UnveilPermission::Execute,
                UnveilPermission::ReadWrite => *perm == UnveilPermission::ReadWrite,
            };
        }

        // Use the most specific component boundary match; `/tmp-old` is not under `/tmp`.
        if let Some((_, perm)) = self
            .paths
            .iter()
            .filter(|(base, _)| {
                path == base.as_str()
                    || (path.starts_with(base.as_str())
                        && (base.ends_with('/') || path.as_bytes().get(base.len()) == Some(&b'/')))
            })
            .max_by_key(|(base, _)| base.len())
        {
            return match permission {
                UnveilPermission::Read => {
                    *perm == UnveilPermission::Read || *perm == UnveilPermission::ReadWrite
                }
                UnveilPermission::Write => {
                    *perm == UnveilPermission::Write || *perm == UnveilPermission::ReadWrite
                }
                UnveilPermission::Execute => *perm == UnveilPermission::Execute,
                UnveilPermission::ReadWrite => *perm == UnveilPermission::ReadWrite,
            };
        // Check parent path match
        for (unveiled_path, perm) in &self.paths {
            if path.starts_with(unveiled_path) {
                return match permission {
                    UnveilPermission::Read => {
                        *perm == UnveilPermission::Read || *perm == UnveilPermission::ReadWrite
                    }
                    UnveilPermission::Write => {
                        *perm == UnveilPermission::Write || *perm == UnveilPermission::ReadWrite
                    }
                    UnveilPermission::Execute => *perm == UnveilPermission::Execute,
                    UnveilPermission::ReadWrite => *perm == UnveilPermission::ReadWrite,
                };
            }
        }

        // Default deny if unveiled and no match
        !self.default_deny
    }
}

impl Default for UnveilSandbox {
    fn default() -> Self {
        Self::new()
    }
}

/// Combined pledge/unveil sandbox
#[derive(Debug, Clone)]
pub struct Sandbox {
    pub pledge: PledgeSandbox,
    pub unveil: UnveilSandbox,
}

impl Sandbox {
    pub fn new() -> Self {
        Sandbox {
            pledge: PledgeSandbox::new(),
            unveil: UnveilSandbox::new(),
        }
    }

    pub fn pledge(&mut self, promises: &str) -> bool {
        self.pledge.pledge(promises)
    }

    pub fn unveil(&mut self, path: &str, permissions: &str) -> bool {
        self.unveil.unveil(path, permissions)
    }

    pub fn check_operation(
        &self,
        promise: PledgePromise,
        path: Option<&str>,
        permission: Option<UnveilPermission>,
    ) -> bool {
        // Check pledge promise
        if !self.pledge.check_promise(promise) {
            return false;
        }

        // Check unveil path access if path provided
        if let Some(p) = path {
            if let Some(perm) = permission {
                if !self.unveil.check_path_access(p, perm) {
                    return false;
                }
            }
        }

        true
    }
}

impl Default for Sandbox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pledge_sandbox_creation() {
        let sandbox = PledgeSandbox::new();
        assert!(!sandbox.promised());
    }

    #[test]
    fn test_pledge_promises() {
        let mut sandbox = PledgeSandbox::new();
        let success = sandbox.pledge("stdio rpath wpath");
        assert!(success);
        assert!(sandbox.promised());
        assert!(sandbox.has_promise(PledgePromise::Stdio));
        assert!(sandbox.has_promise(PledgePromise::Rpath));
        assert!(sandbox.has_promise(PledgePromise::Wpath));
    }

    #[test]
    fn test_pledge_cannot_pledge_twice() {
        let mut sandbox = PledgeSandbox::new();
        sandbox.pledge("stdio");
        let success = sandbox.pledge("rpath");
        assert!(!success);
    }

    #[test]
    fn test_pledge_check_promise() {
        let mut sandbox = PledgeSandbox::new();
        sandbox.pledge("stdio rpath");
        assert!(sandbox.check_promise(PledgePromise::Stdio));
        assert!(sandbox.check_promise(PledgePromise::Rpath));
        assert!(!sandbox.check_promise(PledgePromise::Wpath));
    }

    #[test]
    fn test_unveil_sandbox_creation() {
        let sandbox = UnveilSandbox::new();
        assert!(!sandbox.unveiled());
    }

    #[test]
    fn test_unveil_path() {
        let mut sandbox = UnveilSandbox::new();
        let success = sandbox.unveil("/tmp", "rw");
        assert!(success);
        assert!(sandbox.unveiled());
    }

    #[test]
    fn test_unveil_check_path_access() {
        let mut sandbox = UnveilSandbox::new();
        sandbox.unveil("/tmp", "rw");
        sandbox.set_default_deny(true);

        assert!(sandbox.check_path_access("/tmp", UnveilPermission::Read));
        assert!(sandbox.check_path_access("/tmp", UnveilPermission::Write));
        assert!(!sandbox.check_path_access("/tmp", UnveilPermission::Execute));
        assert!(!sandbox.check_path_access("/etc", UnveilPermission::Read));
    }

    #[test]
    fn test_unveil_subpath_access() {
        let mut sandbox = UnveilSandbox::new();
        sandbox.unveil("/home/user", "r");
        sandbox.set_default_deny(true);

        assert!(sandbox.check_path_access("/home/user", UnveilPermission::Read));
        assert!(sandbox.check_path_access("/home/user/document.txt", UnveilPermission::Read));
        assert!(!sandbox.check_path_access("/etc/passwd", UnveilPermission::Read));
    }

    #[test]
    fn test_combined_sandbox() {
        let mut sandbox = Sandbox::new();
        sandbox.pledge("stdio rpath");
        sandbox.unveil("/tmp", "r");

        assert!(sandbox.check_operation(PledgePromise::Stdio, None, None));
        assert!(sandbox.check_operation(
            PledgePromise::Rpath,
            Some("/tmp"),
            Some(UnveilPermission::Read)
        ));
        assert!(!sandbox.check_operation(PledgePromise::Wpath, None, None));
        assert!(!sandbox.check_operation(
            PledgePromise::Rpath,
            Some("/etc"),
            Some(UnveilPermission::Read)
        ));
    }
}
