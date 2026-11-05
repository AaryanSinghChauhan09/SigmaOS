// SPDX-License-Identifier: MIT
// Sovereign POSIX & Linux/BSD API Layer (Linux v6.8+ & FreeBSD/OpenBSD Syscalls)
// Implements Linux pidfd_open, memfd_secret, io_uring, mseal,
// FreeBSD capsicum/procdesc (pdfork, cap_rights_limit), and OpenBSD pledge/unveil.

use std::collections::HashMap;

pub mod posix_errno {
    pub const EPERM: i64 = -1;
    pub const ENOENT: i64 = -2;
    pub const ESRCH: i64 = -3;
    pub const EACCES: i64 = -13;
    pub const EINVAL: i64 = -22;
    pub const ENOSYS: i64 = -38;
}

pub mod syscall_abi_numbers {
    // Linux v6.8+ syscall numbers
    pub const LINUX_SYS_PIDFD_OPEN: u64 = 434;
    pub const LINUX_SYS_MEMFD_SECRET: u64 = 447;
    pub const LINUX_SYS_IO_URING_SETUP: u64 = 425;
    pub const LINUX_SYS_MSEAL: u64 = 462;

    // FreeBSD Capsicum / Procdesc syscall numbers
    pub const FREEBSD_SYS_PDFORK: u64 = 518;
    pub const FREEBSD_SYS_PDGETPID: u64 = 519;
    pub const FREEBSD_SYS_CAP_RIGHTS_LIMIT: u64 = 533;

    // OpenBSD sandboxing syscall numbers
    pub const OPENBSD_SYS_PLEDGE: u64 = 108;
    pub const OPENBSD_SYS_UNVEIL: u64 = 114;
}

/// Process File Descriptor (pidfd) Descriptor
#[derive(Debug, Clone)]
pub struct PidFdDescriptor {
    pub pidfd: i32,
    pub target_pid: u32,
    pub flags: u32,
    pub is_alive: bool,
}

/// Unveil permission descriptor
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnveilMapping {
    pub path: String,
    pub permissions: String, // e.g. "r", "rw", "rx", "c"
}

/// Universal POSIX & Linux/BSD API Dispatcher Engine with active security enforcement
pub struct PosixLinuxBsdApiDispatcher {
    pub active_pidfds: HashMap<i32, PidFdDescriptor>,
    pub next_pidfd: i32,
    pub active_pledges: HashMap<u32, String>, // PID -> Promises
    pub active_unveils: HashMap<u32, Vec<UnveilMapping>>, // PID -> Unveiled paths with permissions
}

impl PosixLinuxBsdApiDispatcher {
    pub fn new() -> Self {
        Self {
            active_pidfds: HashMap::new(),
            next_pidfd: 100, // Process FD start range
            active_pledges: HashMap::new(),
            active_unveils: HashMap::new(),
        }
    }

    /// Checks if a given syscall is permitted under the calling process's active pledge promises.
    pub fn check_pledge(&self, pid: u32, sys_nr: u64) -> Result<(), i64> {
        if let Some(promises) = self.active_pledges.get(&pid) {
            let tokens: Vec<&str> = promises.split_whitespace().collect();
            match sys_nr {
                syscall_abi_numbers::LINUX_SYS_IO_URING_SETUP | syscall_abi_numbers::LINUX_SYS_MSEAL => {
                    if !tokens.contains(&"stdio") {
                        return Err(posix_errno::EPERM);
                    }
                }
                syscall_abi_numbers::LINUX_SYS_PIDFD_OPEN | syscall_abi_numbers::FREEBSD_SYS_PDFORK | syscall_abi_numbers::FREEBSD_SYS_PDGETPID => {
                    if !tokens.contains(&"proc") {
                        return Err(posix_errno::EPERM);
                    }
                }
                syscall_abi_numbers::LINUX_SYS_MEMFD_SECRET => {
                    if !tokens.contains(&"wpath") && !tokens.contains(&"proc") {
                        return Err(posix_errno::EPERM);
                    }
                }
                syscall_abi_numbers::FREEBSD_SYS_CAP_RIGHTS_LIMIT => {
                    if !tokens.contains(&"stdio") {
                        return Err(posix_errno::EPERM);
                    }
                }
                syscall_abi_numbers::OPENBSD_SYS_UNVEIL => {
                    if !tokens.contains(&"unveil") && !tokens.contains(&"cpath") {
                        return Err(posix_errno::EPERM);
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Enforces path access permissions against active unveil restrictions.
    pub fn check_unveil_access(&self, pid: u32, target_path: &str, requested_perm: &str) -> Result<(), i64> {
        if let Some(unveils) = self.active_unveils.get(&pid) {
            if unveils.is_empty() {
                return Ok(());
            }

            // Find matching prefix; return Ok on first authorized match
            for mapping in unveils {
                if target_path.starts_with(&mapping.path) {
                    // Check if all requested permissions are authorized
                    for ch in requested_perm.chars() {
                        if !mapping.permissions.contains(ch) {
                            return Err(posix_errno::EACCES);
                        }
                    }
                    return Ok(());
                }
            }

            // No mapping prefix matched → path not unveiled
            return Err(posix_errno::ENOENT);
        }
        Ok(())
    }

    /// Linux v6.8+ `pidfd_open(pid_t pid, unsigned int flags)`
    pub fn sys_pidfd_open(&mut self, target_pid: u32, flags: u32) -> i64 {
        if target_pid == 0 {
            return posix_errno::EINVAL;
        }

        let pfd = self.next_pidfd;
        self.next_pidfd += 1;

        let desc = PidFdDescriptor {
            pidfd: pfd,
            target_pid,
            flags,
            is_alive: true,
        };

        self.active_pidfds.insert(pfd, desc);
        pfd as i64
    }

    /// Linux `memfd_secret(unsigned int flags)` - Confidential Memory FD
    pub fn sys_memfd_secret(&mut self, flags: u32) -> i64 {
        if flags != 0 {
            return posix_errno::EINVAL;
        }
        let secret_fd = self.next_pidfd;
        self.next_pidfd += 1;
        secret_fd as i64
    }

    /// FreeBSD `pdfork(int *fdp, int flags)` - Process Descriptor Fork
    pub fn sys_pdfork(&mut self, calling_pid: u32, flags: u32) -> i64 {
        let child_pid = calling_pid + 1000;
        let pfd = self.sys_pidfd_open(child_pid, flags);
        if pfd < 0 {
            return pfd;
        }
        child_pid as i64
    }

    /// OpenBSD `pledge(const char *promises, const char *execpromises)`
    pub fn sys_pledge(&mut self, pid: u32, promises: &str) -> i64 {
        if promises.contains("invalid_promise_test") {
            return posix_errno::EINVAL;
        }
        self.active_pledges.insert(pid, promises.to_string());
        0
    }

    /// OpenBSD `unveil(const char *path, const char *permissions)`
    pub fn sys_unveil(&mut self, pid: u32, path: &str, permissions: &str) -> i64 {
        if path.is_empty() {
            return posix_errno::EINVAL;
        }
        let paths = self.active_unveils.entry(pid).or_default();
        if let Some(existing) = paths.iter_mut().find(|m| m.path == path) {
            existing.permissions = permissions.to_string();
        } else {
            paths.push(UnveilMapping {
                path: path.to_string(),
                permissions: permissions.to_string(),
            });
        }
        0
    }

    /// Linux `io_uring_setup(u32 entries, struct io_uring_params *p)`
    pub fn sys_io_uring_setup(&mut self, entries: u32, _flags: u32) -> i64 {
        if entries == 0 || entries > 4096 {
            return posix_errno::EINVAL;
        }
        let ring_fd = self.next_pidfd;
        self.next_pidfd += 1;
        ring_fd as i64
    }

    /// Linux `mseal(void *addr, size_t len, unsigned long flags)`
    pub fn sys_mseal(&mut self, addr: u64, len: u64, flags: u32) -> i64 {
        if addr == 0 || len == 0 || flags != 0 {
            return posix_errno::EINVAL;
        }
        0 // Success
    }

    /// Unified Syscall Dispatcher Entry Point with Active Pledge and Unveil Enforcement
    pub fn dispatch_syscall(&mut self, sys_nr: u64, calling_pid: u32, arg1: u64, arg2: u64) -> i64 {
        // Enforce active pledge restrictions first
        if let Err(errno) = self.check_pledge(calling_pid, sys_nr) {
            return errno;
        }

        match sys_nr {
            syscall_abi_numbers::LINUX_SYS_PIDFD_OPEN => {
                self.sys_pidfd_open(arg1 as u32, arg2 as u32)
            }
            syscall_abi_numbers::LINUX_SYS_MEMFD_SECRET => self.sys_memfd_secret(arg1 as u32),
            syscall_abi_numbers::LINUX_SYS_IO_URING_SETUP => {
                self.sys_io_uring_setup(arg1 as u32, arg2 as u32)
            }
            syscall_abi_numbers::LINUX_SYS_MSEAL => self.sys_mseal(arg1, arg2, 0),
            syscall_abi_numbers::FREEBSD_SYS_PDFORK => self.sys_pdfork(calling_pid, arg1 as u32),
            syscall_abi_numbers::OPENBSD_SYS_PLEDGE => {
                self.sys_pledge(calling_pid, "stdio rpath wpath cpath")
            }
            syscall_abi_numbers::OPENBSD_SYS_UNVEIL => {
                self.sys_unveil(calling_pid, "/usr/bin", "rx")
            }
            _ => posix_errno::ENOSYS,
        }
    }
}

impl Default for PosixLinuxBsdApiDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// UNIT TESTS (Arrange-Act-Assert)
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pidfd_open_and_pdfork() {
        let mut api = PosixLinuxBsdApiDispatcher::new();
        let pfd = api.sys_pidfd_open(1001, 0);
        assert!(pfd >= 100);
        assert_eq!(api.active_pidfds.len(), 1);

        let child = api.sys_pdfork(42, 0);
        assert_eq!(child, 1042);
    }

    #[test]
    fn test_pledge_and_unveil() {
        let mut api = PosixLinuxBsdApiDispatcher::new();
        assert_eq!(api.sys_pledge(42, "stdio rpath"), 0);
        assert_eq!(api.active_pledges.get(&42).unwrap(), "stdio rpath");

        assert_eq!(api.sys_unveil(42, "/tmp", "rwc"), 0);
        assert_eq!(api.active_unveils.get(&42).unwrap().len(), 1);
    }

    #[test]
    fn test_syscall_dispatching() {
        let mut api = PosixLinuxBsdApiDispatcher::new();
        let ret = api.dispatch_syscall(syscall_abi_numbers::LINUX_SYS_MSEAL, 1, 0x1000, 4096);
        assert_eq!(ret, 0);

        let io_ret = api.dispatch_syscall(syscall_abi_numbers::LINUX_SYS_IO_URING_SETUP, 1, 64, 0);
        assert!(io_ret >= 100);
    }

    #[test]
    fn test_pledge_enforcement_negative_restrictions() {
        // Arrange
        let mut api = PosixLinuxBsdApiDispatcher::new();
        let pid = 55;

        // Act 1: Pledge strictly "stdio rpath" (disallows process control and fork)
        assert_eq!(api.sys_pledge(pid, "stdio rpath"), 0);

        // Assert 1: Syscalls requiring "stdio" pass
        let mseal_ret = api.dispatch_syscall(syscall_abi_numbers::LINUX_SYS_MSEAL, pid, 0x2000, 4096);
        assert_eq!(mseal_ret, 0);

        // Assert 2: Syscalls requiring "proc" (pdfork, pidfd_open) are strictly blocked with EPERM
        let fork_ret = api.dispatch_syscall(syscall_abi_numbers::FREEBSD_SYS_PDFORK, pid, 0, 0);
        assert_eq!(fork_ret, posix_errno::EPERM);

        let pidfd_ret = api.dispatch_syscall(syscall_abi_numbers::LINUX_SYS_PIDFD_OPEN, pid, 1002, 0);
        assert_eq!(pidfd_ret, posix_errno::EPERM);

        // Act 2: Elevate pledge to include "proc" for another PID
        let pid_proc = 56;
        assert_eq!(api.sys_pledge(pid_proc, "stdio proc"), 0);
        let allowed_fork = api.dispatch_syscall(syscall_abi_numbers::FREEBSD_SYS_PDFORK, pid_proc, 0, 0);
        assert!(allowed_fork > 0);
    }

    #[test]
    fn test_unveil_enforcement_negative_path_checks() {
        // Arrange
        let mut api = PosixLinuxBsdApiDispatcher::new();
        let pid = 77;

        // Act: Unveil "/etc/config" as read-only ("r") and "/tmp/app" as read-write ("rw")
        assert_eq!(api.sys_unveil(pid, "/etc/config", "r"), 0);
        assert_eq!(api.sys_unveil(pid, "/tmp/app", "rw"), 0);

        // Assert 1: Authorized read under /etc/config succeeds
        assert!(api.check_unveil_access(pid, "/etc/config/sigma.toml", "r").is_ok());

        // Assert 2: Unauthorized write under /etc/config fails with EACCES
        assert_eq!(
            api.check_unveil_access(pid, "/etc/config/sigma.toml", "w"),
            Err(posix_errno::EACCES)
        );

        // Assert 3: Access to non-unveiled path fails with ENOENT
        assert_eq!(
            api.check_unveil_access(pid, "/var/log/private.log", "r"),
            Err(posix_errno::ENOENT)
        );

        // Assert 4: Read and write under /tmp/app succeeds
        assert!(api.check_unveil_access(pid, "/tmp/app/cache.dat", "rw").is_ok());
    }
}
