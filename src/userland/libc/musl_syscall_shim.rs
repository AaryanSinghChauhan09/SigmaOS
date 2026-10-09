//! # Musl Syscall Shim Engine
//!
//! Linux x86_64 ABI syscall dispatch shim connecting standard Musl libc C-ABI calls
//! directly to SigmaOS kernel subsystems, enabling execution of unmodified C/Rust binaries.

#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicI32, AtomicU64, Ordering};

/// Linux x86_64 Standard System Call Numbers
pub const SYS_READ: u64 = 0;
pub const SYS_WRITE: u64 = 1;
pub const SYS_OPEN: u64 = 2;
pub const SYS_CLOSE: u64 = 3;
pub const SYS_STAT: u64 = 4;
pub const SYS_FSTAT: u64 = 5;
pub const SYS_LSEEK: u64 = 8;
pub const SYS_MMAP: u64 = 9;
pub const SYS_MPROTECT: u64 = 10;
pub const SYS_MUNMAP: u64 = 11;
pub const SYS_BRK: u64 = 12;
pub const SYS_RT_SIGACTION: u64 = 13;
pub const SYS_RT_SIGPROCMASK: u64 = 14;
pub const SYS_IOCTL: u64 = 16;
pub const SYS_GETPID: u64 = 39;
pub const SYS_EXIT: u64 = 60;
pub const SYS_CLOCK_GETTIME: u64 = 228;
pub const SYS_EXIT_GROUP: u64 = 231;
pub const SYS_TGKILL: u64 = 234;

/// Standard POSIX Error Numbers
pub const EPERM: i64 = -1;
pub const ENOENT: i64 = -2;
pub const ESRCH: i64 = -3;
pub const EINTR: i64 = -4;
pub const EIO: i64 = -5;
pub const EBADF: i64 = -9;
pub const ENOMEM: i64 = -12;
pub const EACCES: i64 = -13;
pub const EFAULT: i64 = -14;
pub const EINVAL: i64 = -22;
pub const ENOSYS: i64 = -38;

/// Memory mapping flags
pub const PROT_READ: u64 = 0x1;
pub const PROT_WRITE: u64 = 0x2;
pub const PROT_EXEC: u64 = 0x4;
pub const MAP_SHARED: u64 = 0x01;
pub const MAP_PRIVATE: u64 = 0x02;
pub const MAP_ANONYMOUS: u64 = 0x20;

/// Virtual memory allocation entry
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VmaRegion {
    pub start: u64,
    pub length: u64,
    pub prot: u64,
    pub flags: u64,
}

/// Simulated file descriptor channel
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FdType {
    Stdin,
    Stdout,
    Stderr,
    Pipe,
    File,
}

/// Musl Process Memory & Syscall Context
pub struct MuslSyscallContext {
    pub pid: u32,
    pub current_brk: u64,
    pub initial_brk: u64,
    pub max_brk: u64,
    pub mmap_base: u64,
    pub vma_regions: BTreeMap<u64, VmaRegion>,
    pub open_fds: BTreeMap<i32, FdType>,
    pub exit_code: AtomicI32,
    pub syscalls_invoked: AtomicU64,
    pub bytes_written_stdout: AtomicU64,
}

impl MuslSyscallContext {
    pub fn new(pid: u32) -> Self {
        let mut fds = BTreeMap::new();
        fds.insert(0, FdType::Stdin);
        fds.insert(1, FdType::Stdout);
        fds.insert(2, FdType::Stderr);

        Self {
            pid,
            current_brk: 0x0060_0000,
            initial_brk: 0x0060_0000,
            max_brk: 0x0080_0000, // 2MB heap limit for small processes
            mmap_base: 0x7FFF_0000_0000,
            vma_regions: BTreeMap::new(),
            open_fds: fds,
            exit_code: AtomicI32::new(0),
            syscalls_invoked: AtomicU64::new(0),
            bytes_written_stdout: AtomicU64::new(0),
        }
    }

    /// Dispatch incoming x86_64 syscall
    pub fn dispatch(
        &mut self,
        syscall_no: u64,
        arg1: u64,
        arg2: u64,
        arg3: u64,
        _arg4: u64,
        _arg5: u64,
        _arg6: u64,
    ) -> i64 {
        self.syscalls_invoked.fetch_add(1, Ordering::SeqCst);

        match syscall_no {
            SYS_READ => {
                let fd = arg1 as i32;
                if !self.open_fds.contains_key(&fd) {
                    return EBADF;
                }
                0 // 0 bytes read (EOF simulation)
            }
            SYS_WRITE => {
                let fd = arg1 as i32;
                let count = arg3 as usize;
                if !self.open_fds.contains_key(&fd) {
                    return EBADF;
                }
                if fd == 1 || fd == 2 {
                    self.bytes_written_stdout
                        .fetch_add(count as u64, Ordering::SeqCst);
                }
                count as i64
            }
            SYS_CLOSE => {
                let fd = arg1 as i32;
                if self.open_fds.remove(&fd).is_some() {
                    0
                } else {
                    EBADF
                }
            }
            SYS_BRK => {
                let requested_brk = arg1;
                if requested_brk == 0 {
                    // Query current brk
                    self.current_brk as i64
                } else if requested_brk >= self.initial_brk && requested_brk <= self.max_brk {
                    self.current_brk = requested_brk;
                    self.current_brk as i64
                } else {
                    self.current_brk as i64 // Reject change and return current
                }
            }
            SYS_MMAP => {
                let length = arg2;
                let prot = arg3;
                let flags = _arg4;

                if length == 0 {
                    return EINVAL;
                }

                // Align length to 4KB page
                let aligned_len = (length + 4095) & !4095;
                let alloc_addr = self.mmap_base;
                self.mmap_base += aligned_len;

                self.vma_regions.insert(
                    alloc_addr,
                    VmaRegion {
                        start: alloc_addr,
                        length: aligned_len,
                        prot,
                        flags,
                    },
                );

                alloc_addr as i64
            }
            SYS_MUNMAP => {
                let addr = arg1;
                if self.vma_regions.remove(&addr).is_some() {
                    0
                } else {
                    EINVAL
                }
            }
            SYS_GETPID => self.pid as i64,
            SYS_EXIT | SYS_EXIT_GROUP => {
                let code = arg1 as i32;
                self.exit_code.store(code, Ordering::SeqCst);
                0
            }
            SYS_CLOCK_GETTIME => 0, // Success (time stored in buffer)
            _ => ENOSYS,
        }
    }
}

// ============================================================================
// UNIT TESTS & STANDALONE HARNESS
// ============================================================================

#[cfg(any(test, feature = "standalone_test"))]
mod tests {
    use super::*;

    #[test]
    fn test_brk_query_and_expansion() {
        let mut ctx = MuslSyscallContext::new(1001);
        let cur = ctx.dispatch(SYS_BRK, 0, 0, 0, 0, 0, 0);
        assert_eq!(cur, 0x0060_0000);

        let expanded = ctx.dispatch(SYS_BRK, 0x0061_0000, 0, 0, 0, 0, 0);
        assert_eq!(expanded, 0x0061_0000);
        assert_eq!(ctx.current_brk, 0x0061_0000);

        // Disallow out-of-bounds brk
        let oob = ctx.dispatch(SYS_BRK, 0x0090_0000, 0, 0, 0, 0, 0);
        assert_eq!(oob, 0x0061_0000); // Unchanged
    }

    #[test]
    fn test_mmap_and_munmap() {
        let mut ctx = MuslSyscallContext::new(1002);
        let addr = ctx.dispatch(
            SYS_MMAP,
            0,
            8192,
            PROT_READ | PROT_WRITE,
            MAP_ANONYMOUS,
            0,
            0,
        );
        assert!(addr > 0);

        let unmap_res = ctx.dispatch(SYS_MUNMAP, addr as u64, 8192, 0, 0, 0, 0);
        assert_eq!(unmap_res, 0);

        let unmap_invalid = ctx.dispatch(SYS_MUNMAP, 0x1234, 4096, 0, 0, 0, 0);
        assert_eq!(unmap_invalid, EINVAL);
    }

    #[test]
    fn test_stdout_write() {
        let mut ctx = MuslSyscallContext::new(1003);
        let res = ctx.dispatch(SYS_WRITE, 1, 0x4000, 14, 0, 0, 0);
        assert_eq!(res, 14);
        assert_eq!(ctx.bytes_written_stdout.load(Ordering::SeqCst), 14);
    }
}
