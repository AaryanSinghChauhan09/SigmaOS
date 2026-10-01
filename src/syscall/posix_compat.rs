//! POSIX compatibility syscall stubs for SigmaOS
//! Inspired by Linux kernel sys_call_table and FreeBSD syscall registry.
//! These provide POSIX ABI compatibility for applications.
//!
//! Linux syscall numbers: https://github.com/torvalds/linux/blob/master/arch/x86/entry/syscalls/syscall_64.tbl
//! FreeBSD syscalls: https://github.com/freebsd/freebsd-src/blob/main/sys/kern/syscalls.master

/// sys_prctl(2) — process control (Linux-compatible)
/// Supports PR_SET_NAME (1), PR_GET_NAME (15), PR_SET_DUMPABLE (4), PR_GET_DUMPABLE (3)
pub fn sys_prctl(option: i32, arg2: u64, _arg3: u64, _arg4: u64, _arg5: u64) -> i32 {
    match option {
        1 => 0,   // PR_SET_NAME: accepted, not stored yet
        3 => 1,   // PR_GET_DUMPABLE: returns 1 (dumpable)
        4 => 0,   // PR_SET_DUMPABLE
        15 => {   // PR_GET_NAME: write "sigmaos\0" to arg2 ptr
            if arg2 != 0 {
                // Safety note: in real impl validate user pointer before writing
                let _ptr = arg2 as *mut u8;
                // Stub: name copied by caller convention
            }
            0
        }
        22 => 0,  // PR_SET_SECCOMP
        _ => -22, // EINVAL
    }
}

/// sys_madvise(2) — give advice about memory usage (Linux/POSIX compatible)
/// Hints are accepted and silently ignored in this stub.
pub fn sys_madvise(_addr: usize, _length: usize, advice: i32) -> i32 {
    match advice {
        0 => 0,  // MADV_NORMAL
        1 => 0,  // MADV_RANDOM
        2 => 0,  // MADV_SEQUENTIAL
        3 => 0,  // MADV_WILLNEED
        4 => 0,  // MADV_DONTNEED
        8 => 0,  // MADV_FREE (Linux 4.5+)
        9 => 0,  // MADV_REMOVE
        _ => 0,  // Accept unknown advice (non-fatal, per POSIX)
    }
}

/// sys_pread64(2) — read from file at offset without changing file position
pub fn sys_pread64(fd: i32, buf: *mut u8, count: usize, _offset: i64) -> isize {
    if buf.is_null() || count == 0 { return -22; } // EINVAL
    if fd < 0 { return -9; }  // EBADF
    -38 // ENOSYS — not yet fully implemented
}

/// sys_pwrite64(2) — write to file at offset without changing file position
pub fn sys_pwrite64(fd: i32, buf: *const u8, count: usize, _offset: i64) -> isize {
    if buf.is_null() || count == 0 { return -22; } // EINVAL
    if fd < 0 { return -9; }  // EBADF
    -38 // ENOSYS
}

/// sys_sigaction(2) — examine/change a signal action
pub fn sys_sigaction(signum: i32, _act: *const u8, _oldact: *mut u8) -> i32 {
    if signum <= 0 || signum > 64 { return -22; } // EINVAL: bad signal number
    if signum == 9 || signum == 19 { return -22; } // Can't catch SIGKILL/SIGSTOP
    0 // Accepted (signal disposition not yet fully tracked)
}

/// sys_sigprocmask(2) — examine/change blocked signals
pub fn sys_sigprocmask(_how: i32, _set: *const u8, _oldset: *mut u8) -> i32 {
    0 // Stub: accepted silently
}

/// sys_getpid(2) — get process ID
pub fn sys_getpid() -> i32 {
    1 // Stub: PID 1 (init process)
}

/// sys_getppid(2) — get parent process ID
pub fn sys_getppid() -> i32 {
    0 // Stub: PID 0 (idle/kernel)
}

/// sys_exit(2) — terminate current process
pub fn sys_exit(status: i32) -> ! {
    // In bare metal: halt CPU or enter idle loop
    let _ = status;
    loop {
        #[cfg(target_arch = "x86_64")]
        unsafe { core::arch::asm!("hlt"); }
        #[cfg(not(target_arch = "x86_64"))]
        { } // spin
    }
}

/// sys_exit_group(2) — exit all threads in process group
pub fn sys_exit_group(status: i32) -> ! {
    sys_exit(status)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prctl_set_name() {
        assert_eq!(sys_prctl(1, 0xDEAD, 0, 0, 0), 0);
    }

    #[test]
    fn test_prctl_invalid() {
        assert_eq!(sys_prctl(9999, 0, 0, 0, 0), -22);
    }

    #[test]
    fn test_madvise_all_known_hints() {
        for advice in [0, 1, 2, 3, 4, 8, 9] {
            assert_eq!(sys_madvise(0x1000, 4096, advice), 0);
        }
    }

    #[test]
    fn test_pread64_null_buf() {
        assert_eq!(sys_pread64(3, core::ptr::null_mut(), 128, 0), -22);
    }

    #[test]
    fn test_sigaction_invalid_signal() {
        assert_eq!(sys_sigaction(-1, core::ptr::null(), core::ptr::null_mut()), -22);
    }

    #[test]
    fn test_sigaction_sigkill_blocked() {
        assert_eq!(sys_sigaction(9, core::ptr::null(), core::ptr::null_mut()), -22);
    }

    #[test]
    fn test_getpid() {
        assert_eq!(sys_getpid(), 1);
    }
}
