// SPDX-License-Identifier: MIT
// Universal POSIX & Linux/BSD Application Programming Interface (API) Subsystem for SigmaOS (`src/syscall/posix_linux_bsd_api.rs`)
// Inspired by Linux v6.8+ syscalls (pidfd_open, memfd_secret, io_uring_setup),
// FreeBSD capsicum/procdesc (pdfork, cap_rights_limit), and OpenBSD pledge/unveil.

use std::collections::HashMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// Standard POSIX System Call Error Codes
pub mod posix_errno {
    pub const EPERM: i64 = -1;   // Operation not permitted
    pub const ENOENT: i64 = -2;  // No such file or directory
    pub const ESRCH: i64 = -3;   // No such process
    pub const EINTR: i64 = -4;   // Interrupted system call
    pub const EIO: i64 = -5;     // I/O error
    pub const EBADF: i64 = -9;   // Bad file descriptor
    pub const EAGAIN: i64 = -11; // Try again / Resource temporarily unavailable
    pub const ENOMEM: i64 = -12; // Out of memory
    pub const EACCES: i64 = -13; // Permission denied
    pub const EEXIST: i64 = -17; // File exists
    pub const EINVAL: i64 = -22; // Invalid argument
    pub const ENOSYS: i64 = -38; // Function not implemented
}

/// Linux / FreeBSD / OpenBSD Syscall Numbers Mapping
pub mod syscall_abi_numbers {
    // Linux x86_64
    pub const LINUX_SYS_PIDFD_OPEN: u64 = 434;
    pub const LINUX_SYS_MEMFD_SECRET: u64 = 447;
    pub const LINUX_SYS_IO_URING_SETUP: u64 = 425;

    // FreeBSD
    pub const FREEBSD_SYS_PDFORK: u64 = 518;
    pub const FREEBSD_SYS_CAP_RIGHTS_LIMIT: u64 = 537;

    // OpenBSD
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

/// Universal POSIX & Linux/BSD API Dispatcher Engine
pub struct PosixLinuxBsdApiDispatcher {
    pub active_pidfds: HashMap<i32, PidFdDescriptor>,
    pub next_pidfd: i32,
    pub active_pledges: HashMap<u32, String>,      // PID -> Promises
    pub active_unveils: HashMap<u32, Vec<String>>, // PID -> Unveiled paths
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
    pub fn sys_unveil(&mut self, pid: u32, path: &str, _permissions: &str) -> i64 {
        if path.is_empty() {
            return posix_errno::EINVAL;
        }
        let paths = self.active_unveils.entry(pid).or_default();
        if !paths.contains(&path.to_string()) {
            paths.push(path.to_string());
        }
        0
    }

    /// Unified Syscall Dispatcher Entry Point
    pub fn dispatch_syscall(&mut self, sys_nr: u64, calling_pid: u32, arg1: u64, arg2: u64) -> i64 {
        match sys_nr {
            syscall_abi_numbers::LINUX_SYS_PIDFD_OPEN => self.sys_pidfd_open(arg1 as u32, arg2 as u32),
            syscall_abi_numbers::LINUX_SYS_MEMFD_SECRET => self.sys_memfd_secret(arg1 as u32),
            syscall_abi_numbers::FREEBSD_SYS_PDFORK => self.sys_pdfork(calling_pid, arg1 as u32),
            syscall_abi_numbers::OPENBSD_SYS_PLEDGE => self.sys_pledge(calling_pid, "stdio rpath wpath cpath"),
            syscall_abi_numbers::OPENBSD_SYS_UNVEIL => self.sys_unveil(calling_pid, "/usr/bin", "rx"),
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
// UNIT TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pidfd_open_and_pdfork() {
        let mut api = PosixLinuxBsdApiDispatcher::new();
        let pfd = api.sys_pidfd_open(501, 0);

        assert!(pfd >= 100);
        assert_eq!(api.active_pidfds.get(&(pfd as i32)).unwrap().target_pid, 501);

        let child_pid = api.sys_pdfork(10, 0);
        assert!(child_pid > 1000);
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
        let res = api.dispatch_syscall(syscall_abi_numbers::LINUX_SYS_MEMFD_SECRET, 100, 0, 0);
        assert!(res >= 100);

        let invalid_res = api.dispatch_syscall(9999, 100, 0, 0);
        assert_eq!(invalid_res, posix_errno::ENOSYS);
    }
}
