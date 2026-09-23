// SigmaOS Universal Application Interface (ABI & Syscall) Extensions
// Inspired by FreeBSD Linuxulator (sys/compat/linux), NetBSD COMPAT_LINUX,
// OpenBSD pledge(2) / unveil(2) application interface, and FreeBSD Capsicum cap_rights_t capability interface.

use std::collections::{BTreeMap, BTreeSet};
use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

/// Target Application Personality / ABI Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApplicationPersonality {
    NativeSigmaOS,
    LinuxX86_64,
    FreeBsdNative,
    OpenBsdNative,
    NetBsdNative,
}

/// Linuxulator Syscall Translation Mapping (FreeBSD sys/compat/linux inspired)
#[derive(Debug, Clone)]
pub struct SovereignLinuxulatorAbiBridge {
    pub syscall_table: BTreeMap<u32, String>, // Linux Syscall NR -> Native Syscall Name
}

impl SovereignLinuxulatorAbiBridge {
    pub fn new() -> Self {
        let mut table = BTreeMap::new();

        // Standard x86_64 Linux Syscall Numbers
        table.insert(0, "sys_read".to_string());
        table.insert(1, "sys_write".to_string());
        table.insert(2, "sys_open".to_string());
        table.insert(3, "sys_close".to_string());
        table.insert(4, "sys_stat".to_string());
        table.insert(5, "sys_fstat".to_string());
        table.insert(8, "sys_lseek".to_string());
        table.insert(9, "sys_mmap".to_string());
        table.insert(10, "sys_mprotect".to_string());
        table.insert(11, "sys_munmap".to_string());
        table.insert(12, "sys_brk".to_string());
        table.insert(13, "sys_rt_sigaction".to_string());
        table.insert(14, "sys_rt_sigprocmask".to_string());
        table.insert(39, "sys_getpid".to_string());
        table.insert(56, "sys_clone".to_string());
        table.insert(59, "sys_execve".to_string());
        table.insert(60, "sys_exit".to_string());

        Self {
            syscall_table: table,
        }
    }

    /// Translates a Linux syscall number into a native SigmaOS syscall handler name
    pub fn translate_linux_syscall(&self, linux_nr: u32) -> Result<String, String> {
        self.syscall_table
            .get(&linux_nr)
            .cloned()
            .ok_or_else(|| format!("ENOSYS: Linux syscall number {} not supported in Linuxulator bridge", linux_nr))
    }
}

impl Default for SovereignLinuxulatorAbiBridge {
    fn default() -> Self {
        Self::new()
    }
}

/// OpenBSD pledge(2) & unveil(2) Application Interface Guard
#[derive(Debug, Clone)]
pub struct OpenBsdPledgeUnveilApplicationInterface {
    pub pledged_promises: BTreeSet<String>,
    pub unveiled_paths: BTreeMap<String, String>, // Path -> Permissions "r", "w", "x", "c"
    pub is_pledged: bool,
}

impl OpenBsdPledgeUnveilApplicationInterface {
    pub fn new() -> Self {
        Self {
            pledged_promises: BTreeSet::new(),
            unveiled_paths: BTreeMap::new(),
            is_pledged: false,
        }
    }

    /// OpenBSD pledge(2): Restrict application syscall promises
    pub fn pledge(&mut self, promises: &[&str]) -> Result<(), String> {
        if self.is_pledged && promises.is_empty() {
            return Err("EPERM: Cannot restore pledged promises once restricted".to_string());
        }

        let mut new_set = BTreeSet::new();
        for p in promises {
            new_set.insert(p.to_string());
        }

        if self.is_pledged {
            // Promises can only be dropped, not added
            for new_promise in &new_set {
                if !self.pledged_promises.contains(new_promise) {
                    return Err(format!(
                        "EPERM: Cannot add new pledge promise '{}' to already pledged process",
                        new_promise
                    ));
                }
            }
        }

        self.pledged_promises = new_set;
        self.is_pledged = true;
        Ok(())
    }

    /// OpenBSD unveil(2): Restrict filesystem visibility
    pub fn unveil(&mut self, path: &str, permissions: &str) -> Result<(), String> {
        if path.is_empty() {
            return Err("EINVAL: Unveil path cannot be empty".to_string());
        }

        self.unveiled_paths
            .insert(path.to_string(), permissions.to_string());
        Ok(())
    }

    /// Check if path operation is permitted under unveil rules
    pub fn check_unveil_access(&self, path: &str, required_mode: char) -> Result<(), String> {
        if self.unveiled_paths.is_empty() {
            return Ok(()); // Unveil not activated
        }

        for (unveiled_path, perms) in &self.unveiled_paths {
            if path.starts_with(unveiled_path) {
                if perms.contains(required_mode) {
                    return Ok(());
                } else {
                    return Err(format!(
                        "EACCES: Path '{}' veiled without required permission '{}'",
                        path, required_mode
                    ));
                }
            }
        }

        Err(format!("ENOENT: Path '{}' concealed by unveil policy", path))
    }
}

impl Default for OpenBsdPledgeUnveilApplicationInterface {
    fn default() -> Self {
        Self::new()
    }
}

/// FreeBSD Capsicum Descriptor Rights Interface (`cap_rights_t`)
#[derive(Debug, Clone)]
pub struct FreeBsdCapsicumCapabilityRightsInterface {
    pub fd_rights_map: BTreeMap<i32, BTreeSet<String>>, // FD -> Capability Rights (e.g. "CAP_READ", "CAP_WRITE", "CAP_FSTAT")
    pub capability_mode: bool,
}

impl FreeBsdCapsicumCapabilityRightsInterface {
    pub fn new() -> Self {
        Self {
            fd_rights_map: BTreeMap::new(),
            capability_mode: false,
        }
    }

    /// Enter Capsicum capability mode (`cap_enter`)
    pub fn enter_capability_mode(&mut self) {
        self.capability_mode = true;
    }

    /// Limit file descriptor rights (`cap_rights_limit`)
    pub fn limit_fd_rights(&mut self, fd: i32, rights: &[&str]) {
        let mut set = BTreeSet::new();
        for r in rights {
            set.insert(r.to_string());
        }
        self.fd_rights_map.insert(fd, set);
    }

    /// Verify descriptor right
    pub fn check_fd_right(&self, fd: i32, required_right: &str) -> Result<(), String> {
        if let Some(rights) = self.fd_rights_map.get(&fd) {
            if rights.contains(required_right) {
                Ok(())
            } else {
                Err(format!(
                    "ENOTCAPABLE: File descriptor {} lacks required capability right '{}'",
                    fd, required_right
                ))
            }
        } else if self.capability_mode {
            Err(format!(
                "ENOTCAPABLE: Unregistered file descriptor {} in capability mode",
                fd
            ))
        } else {
            Ok(())
        }
    }
}

impl Default for FreeBsdCapsicumCapabilityRightsInterface {
    fn default() -> Self {
        Self::new()
    }
}

/// Unified Application Interface Manager
#[derive(Debug, Clone)]
pub struct SovereignUniversalAppInterfaceManager {
    pub linuxulator: SovereignLinuxulatorAbiBridge,
    pub pledge_unveil: OpenBsdPledgeUnveilApplicationInterface,
    pub capsicum: FreeBsdCapsicumCapabilityRightsInterface,
    pub default_personality: ApplicationPersonality,
}

impl SovereignUniversalAppInterfaceManager {
    pub fn new(personality: ApplicationPersonality) -> Self {
        Self {
            linuxulator: SovereignLinuxulatorAbiBridge::new(),
            pledge_unveil: OpenBsdPledgeUnveilApplicationInterface::new(),
            capsicum: FreeBsdCapsicumCapabilityRightsInterface::new(),
            default_personality: personality,
        }
    }

    /// Dispatch application syscall invocation based on target personality
    pub fn dispatch_app_syscall(
        &self,
        personality: ApplicationPersonality,
        syscall_nr: u32,
    ) -> Result<String, String> {
        match personality {
            ApplicationPersonality::LinuxX86_64 => {
                self.linuxulator.translate_linux_syscall(syscall_nr)
            }
            ApplicationPersonality::NativeSigmaOS | ApplicationPersonality::FreeBsdNative | ApplicationPersonality::OpenBsdNative | ApplicationPersonality::NetBsdNative => {
                Ok(format!("sys_native_{}", syscall_nr))
            }
        }
    }
}

impl Default for SovereignUniversalAppInterfaceManager {
    fn default() -> Self {
        Self::new(ApplicationPersonality::NativeSigmaOS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linuxulator_syscall_translation() {
        let bridge = SovereignLinuxulatorAbiBridge::new();

        assert_eq!(bridge.translate_linux_syscall(0).unwrap(), "sys_read");
        assert_eq!(bridge.translate_linux_syscall(1).unwrap(), "sys_write");
        assert_eq!(bridge.translate_linux_syscall(59).unwrap(), "sys_execve");

        let err = bridge.translate_linux_syscall(999);
        assert!(err.is_err());
        assert!(err.unwrap_err().contains("ENOSYS"));
    }

    #[test]
    fn test_openbsd_pledge_unveil_interface() {
        let mut guard = OpenBsdPledgeUnveilApplicationInterface::new();

        guard.pledge(&["stdio", "rpath", "wpath"]).unwrap();
        assert!(guard.is_pledged);

        // Cannot add pledges once restricted
        let res = guard.pledge(&["stdio", "rpath", "wpath", "inet"]);
        assert!(res.is_err());

        guard.unveil("/var/log", "rw").unwrap();
        assert!(guard.check_unveil_access("/var/log/syslog", 'r').is_ok());
        assert!(guard.check_unveil_access("/var/log/syslog", 'x').is_err());
        assert!(guard.check_unveil_access("/etc/shadow", 'r').is_err());
    }

    #[test]
    fn test_freebsd_capsicum_rights_interface() {
        let mut capsicum = FreeBsdCapsicumCapabilityRightsInterface::new();

        capsicum.limit_fd_rights(3, &["CAP_READ", "CAP_FSTAT"]);
        capsicum.enter_capability_mode();

        assert!(capsicum.check_fd_right(3, "CAP_READ").is_ok());
        assert!(capsicum.check_fd_right(3, "CAP_WRITE").is_err());
        assert!(capsicum.check_fd_right(4, "CAP_READ").is_err());
    }

    #[test]
    fn test_universal_app_interface_manager() {
        let mgr = SovereignUniversalAppInterfaceManager::new(ApplicationPersonality::LinuxX86_64);

        let handler = mgr
            .dispatch_app_syscall(ApplicationPersonality::LinuxX86_64, 9)
            .unwrap();
        assert_eq!(handler, "sys_mmap");

        let native_handler = mgr
            .dispatch_app_syscall(ApplicationPersonality::NativeSigmaOS, 10)
            .unwrap();
        assert_eq!(native_handler, "sys_native_10");
    }
}
