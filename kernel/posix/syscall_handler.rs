// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/posix/syscall_handler.rs — POSIX Compatibility Layer
//
// Implements syscall wrappers mapping SigmaOS kernel calls to POSIX equivalents.
// This enables Linux/Unix applications to run on SigmaOS.
//
// Language: Rust (no_std for kernel)

#![no_std]

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;
type I64 = i64;
type SizeT = usize;

// ─── POSIX Error Codes ─────────────────────────────────────────────────────

pub const EPERM: I32 = 1;
pub const ENOENT: I32 = 2;
pub const ESRCH: I32 = 3;
pub const EINTR: I32 = 4;
pub const EIO: I32 = 5;
pub const ENXIO: I32 = 6;
pub const E2BIG: I32 = 7;
pub const ENOEXEC: I32 = 8;
pub const EBADF: I32 = 9;
pub const ECHILD: I32 = 10;
pub const EAGAIN: I32 = 11;
pub const EWOULDBLOCK: I32 = 11;
pub const ENOMEM: I32 = 12;
pub const EACCES: I32 = 13;
pub const EFAULT: I32 = 14;
pub const ENOTBLK: I32 = 15;
pub const EBUSY: I32 = 16;
pub const EEXIST: I32 = 17;
pub const EXDEV: I32 = 18;
pub const ENODEV: I32 = 19;
pub const ENOTDIR: I32 = 20;
pub const EISDIR: I32 = 21;
pub const EINVAL: I32 = 22;
pub const ENFILE: I32 = 23;
pub const EMFILE: I32 = 24;
pub const ENOTTY: I32 = 25;
pub const ETXTBSY: I32 = 26;
pub const EFBIG: I32 = 27;
pub const ENOSPC: I32 = 28;
pub const ESPIPE: I32 = 29;
pub const EROFS: I32 = 30;
pub const EMLINK: I32 = 31;
pub const EPIPE: I32 = 32;
pub const EDOM: I32 = 33;
pub const ERANGE: I32 = 34;
pub const EDEADLK: I32 = 35;
pub const ENAMETOOLONG: I32 = 36;
pub const ENOLCK: I32 = 37;
pub const ENOSYS: I32 = 38;
pub const ENOTEMPTY: I32 = 39;
pub const ELOOP: I32 = 40;
pub const EOVERFLOW: I32 = 75;

// ─── POSIX File Descriptors ───────────────────────────────────────────────

pub const STDIN_FILENO: I32 = 0;
pub const STDOUT_FILENO: I32 = 1;
pub const STDERR_FILENO: I32 = 2;

// ─── POSIX Open Flags ───────────────────────────────────────────────────

pub const O_RDONLY: I32 = 0o0000;
pub const O_WRONLY: I32 = 0o0001;
pub const O_RDWR: I32 = 0o0002;
pub const O_CREAT: I32 = 0o0100;
pub const O_EXCL: I32 = 0o0200;
pub const O_NOCTTY: I32 = 0o0400;
pub const O_TRUNC: I32 = 0o1000;
pub const O_APPEND: I32 = 0o2000;
pub const O_NONBLOCK: I32 = 0o4000;
pub const O_SYNC: I32 = 0o10000;
pub const O_ASYNC: I32 = 0o20000;

// ─── POSIX Access Modes ─────────────────────────────────────────────────

pub const F_OK: I32 = 0;
pub const X_OK: I32 = 1;
pub const W_OK: I32 = 2;
pub const R_OK: I32 = 4;

// ─── POSIX Seek Whence ─────────────────────────────────────────────────

pub const SEEK_SET: I32 = 0;
pub const SEEK_CUR: I32 = 1;
pub const SEEK_END: I32 = 2;

// ─── POSIX Syscall Numbers ───────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PosixSyscall {
    Read = 0,
    Write = 1,
    Open = 2,
    Close = 3,
    Stat = 4,
    Fstat = 5,
    Lstat = 6,
    Poll = 7,
    Lseek = 8,
    Mmap = 9,
    Mprotect = 10,
    Munmap = 11,
    Brk = 12,
    RtSigaction = 13,
    RtSigprocmask = 14,
    Ioctl = 16,
    Pread64 = 17,
    Pwrite64 = 18,
    Readv = 19,
    Writev = 20,
    Access = 21,
    Pipe = 22,
    Select = 23,
    SchedYield = 24,
    Mremap = 25,
    Madvise = 28,
    Dup = 32,
    Dup2 = 33,
    Pause = 34,
    Nanosleep = 35,
    Getpid = 39,
    Socket = 41,
    Connect = 42,
    Accept = 43,
    Sendto = 44,
    Recvfrom = 45,
    Sendmsg = 46,
    Recvmsg = 47,
    Shutdown = 48,
    Bind = 49,
    Listen = 50,
    Getsockname = 51,
    Getpeername = 52,
    Socketpair = 53,
    Setsockopt = 54,
    Getsockopt = 55,
    Clone = 56,
    Fork = 57,
    Execve = 59,
    Exit = 60,
    Wait4 = 61,
    Kill = 62,
    Uname = 63,
}

// ─── POSIX File Stat Structure ───────────────────────────────────────────

#[repr(C)]
pub struct Stat {
    pub st_dev: U64,
    pub st_ino: U64,
    pub st_nlink: U64,
    pub st_mode: U32,
    pub st_uid: U32,
    pub st_gid: U32,
    pub st_rdev: U64,
    pub st_size: I64,
    pub st_blksize: I64,
    pub st_blocks: I64,
    pub st_atime: I64,
    pub st_mtime: I64,
    pub st_ctime: I64,
}

impl Stat {
    pub const fn new() -> Self {
        Stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atime: 0,
            st_mtime: 0,
            st_ctime: 0,
        }
    }
}

// ─── POSIX Utsname Structure ─────────────────────────────────────────────

#[repr(C)]
pub struct Utsname {
    pub sysname: [U8; 65],
    pub nodename: [U8; 65],
    pub release: [U8; 65],
    pub version: [U8; 65],
    pub machine: [U8; 65],
    pub domainname: [U8; 65],
}

impl Utsname {
    pub const fn new() -> Self {
        Utsname {
            sysname: [0; 65],
            nodename: [0; 65],
            release: [0; 65],
            version: [0; 65],
            machine: [0; 65],
            domainname: [0; 65],
        }
    }
}

// ─── Syscall Handler Trait ───────────────────────────────────────────────

/// Trait for syscall handling strategies
pub trait SyscallHandler {
    /// Handle a POSIX syscall
    fn handle_syscall(&mut self, syscall: PosixSyscall, args: &[U64]) -> I64;
    
    /// Map SigmaOS error to POSIX error
    fn map_error(&self, sigma_error: I32) -> I32;
    
    /// Get handler name
    fn get_name(&self) -> &'static str;
}

// ─── POSIX Compatibility Layer ───────────────────────────────────────────

pub struct PosixCompatLayer {
    enabled: bool,
    translate_paths: bool,
    emulate_fork: bool,
}

impl PosixCompatLayer {
    pub const fn new() -> Self {
        PosixCompatLayer {
            enabled: true,
            translate_paths: true,
            emulate_fork: true,
        }
    }

    /// Enable or disable POSIX compatibility
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Enable or disable path translation
    pub fn set_path_translation(&mut self, translate: bool) {
        self.translate_paths = translate;
    }

    /// Enable or disable fork emulation
    pub fn set_fork_emulation(&mut self, emulate: bool) {
        self.emulate_fork = emulate;
    }

    /// Translate Linux path to SigmaOS path
    fn translate_path(&self, linux_path: &[U8]) -> [U8; 256] {
        if !self.translate_paths {
            let mut result = [0u8; 256];
            let len = linux_path.len().min(255);
            for i in 0..len {
                result[i] = linux_path[i];
            }
            return result;
        }

        // In a real implementation, this would:
        // 1. Translate /usr to /sigma/usr
        // 2. Translate /var to /sigma/var
        // 3. Translate /etc to /sigma/etc
        // 4. Handle symlinks and mount points

        let mut result = [0u8; 256];
        let len = linux_path.len().min(255);
        for i in 0..len {
            result[i] = linux_path[i];
        }
        result
    }

    /// Handle read syscall
    fn handle_read(&self, fd: I32, buf: *mut U8, count: SizeT) -> I64 {
        if fd < 0 {
            return -EBADF as I64;
        }

        // In a real implementation, this would call SigmaOS read syscall
        // For now, return stub value
        count as I64
    }

    /// Handle write syscall
    fn handle_write(&self, fd: I32, buf: *const U8, count: SizeT) -> I64 {
        if fd < 0 {
            return -EBADF as I64;
        }

        // In a real implementation, this would call SigmaOS write syscall
        // For now, return stub value
        count as I64
    }

    /// Handle open syscall
    fn handle_open(&self, pathname: *const U8, flags: I32, mode: U32) -> I64 {
        if pathname.is_null() {
            return -EFAULT as I64;
        }

        // Translate path
        let linux_path = unsafe { core::slice::from_raw_parts(pathname, 256) };
        let sigma_path = self.translate_path(linux_path);

        // In a real implementation, this would call SigmaOS open syscall
        // For now, return stub file descriptor
        3
    }

    /// Handle close syscall
    fn handle_close(&self, fd: I32) -> I64 {
        if fd < 0 {
            return -EBADF as I64;
        }

        // In a real implementation, this would call SigmaOS close syscall
        0
    }

    /// Handle stat syscall
    fn handle_stat(&self, pathname: *const U8, statbuf: *mut Stat) -> I64 {
        if pathname.is_null() || statbuf.is_null() {
            return -EFAULT as I64;
        }

        // Translate path
        let linux_path = unsafe { core::slice::from_raw_parts(pathname, 256) };
        let sigma_path = self.translate_path(linux_path);

        // In a real implementation, this would call SigmaOS stat syscall
        // For now, return stub stat
        0
    }

    /// Handle lseek syscall
    fn handle_lseek(&self, fd: I32, offset: I64, whence: I32) -> I64 {
        if fd < 0 {
            return -EBADF as I64;
        }

        // In a real implementation, this would call SigmaOS seek syscall
        // For now, return stub offset
        offset
    }

    /// Handle fork syscall
    fn handle_fork(&self) -> I64 {
        if !self.emulate_fork {
            return -ENOSYS as I64;
        }

        // In a real implementation, this would:
        // 1. Create a new process using SigmaOS process creation
        // 2. Copy parent's memory to child
        // 3. Set up child's execution context
        // 4. Return child PID to parent, 0 to child

        // Stub: return child PID
        1234
    }

    /// Handle execve syscall
    fn handle_execve(&self, pathname: *const U8, argv: *const *const U8, envp: *const *const U8) -> I64 {
        if pathname.is_null() {
            return -EFAULT as I64;
        }

        // In a real implementation, this would:
        // 1. Translate path
        // 2. Load executable
        // 3. Set up new memory layout
        // 4. Transfer control to new program

        // Stub: return success
        0
    }

    /// Handle exit syscall
    fn handle_exit(&self, exit_code: I32) -> ! {
        // In a real implementation, this would call SigmaOS exit syscall
        loop {}
    }

    /// Handle getpid syscall
    fn handle_getpid(&self) -> I64 {
        // In a real implementation, this would call SigmaOS get process ID
        1000
    }

    /// Handle uname syscall
    fn handle_uname(&self, buf: *mut Utsname) -> I64 {
        if buf.is_null() {
            return -EFAULT as I64;
        }

        unsafe {
            let uname = &mut *buf;
            
            // Set system name
            let sysname = b"SigmaOS";
            for i in 0..sysname.len() {
                uname.sysname[i] = sysname[i];
            }
            
            // Set release
            let release = b"1.0.0";
            for i in 0..release.len() {
                uname.release[i] = release[i];
            }
            
            // Set version
            let version = b"#1 SMP SigmaOS";
            for i in 0..version.len() {
                uname.version[i] = version[i];
            }
            
            // Set machine
            let machine = b"x86_64";
            for i in 0..machine.len() {
                uname.machine[i] = machine[i];
            }
        }

        0
    }
}

impl SyscallHandler for PosixCompatLayer {
    fn handle_syscall(&mut self, syscall: PosixSyscall, args: &[U64]) -> I64 {
        if !self.enabled {
            return -ENOSYS as I64;
        }

        match syscall {
            PosixSyscall::Read => {
                if args.len() >= 3 {
                    self.handle_read(args[0] as I32, args[1] as *mut U8, args[2] as SizeT)
                } else {
                    -EINVAL as I64
                }
            }
            PosixSyscall::Write => {
                if args.len() >= 3 {
                    self.handle_write(args[0] as I32, args[1] as *const U8, args[2] as SizeT)
                } else {
                    -EINVAL as I64
                }
            }
            PosixSyscall::Open => {
                if args.len() >= 3 {
                    self.handle_open(args[0] as *const U8, args[1] as I32, args[2] as U32)
                } else {
                    -EINVAL as I64
                }
            }
            PosixSyscall::Close => {
                if args.len() >= 1 {
                    self.handle_close(args[0] as I32)
                } else {
                    -EINVAL as I64
                }
            }
            PosixSyscall::Stat => {
                if args.len() >= 2 {
                    self.handle_stat(args[0] as *const U8, args[1] as *mut Stat)
                } else {
                    -EINVAL as I64
                }
            }
            PosixSyscall::Lseek => {
                if args.len() >= 3 {
                    self.handle_lseek(args[0] as I32, args[1] as I64, args[2] as I32)
                } else {
                    -EINVAL as I64
                }
            }
            PosixSyscall::Fork => self.handle_fork(),
            PosixSyscall::Execve => {
                if args.len() >= 3 {
                    self.handle_execve(args[0] as *const U8, args[1] as *const *const U8, args[2] as *const *const U8)
                } else {
                    -EINVAL as I64
                }
            }
            PosixSyscall::Exit => {
                if args.len() >= 1 {
                    self.handle_exit(args[0] as I32)
                } else {
                    -EINVAL as I64
                }
            }
            PosixSyscall::Getpid => self.handle_getpid(),
            PosixSyscall::Uname => {
                if args.len() >= 1 {
                    self.handle_uname(args[0] as *mut Utsname)
                } else {
                    -EINVAL as I64
                }
            }
            _ => -ENOSYS as I64,
        }
    }

    fn map_error(&self, sigma_error: I32) -> I32 {
        // Map SigmaOS-specific errors to POSIX errors
        match sigma_error {
            0 => 0,
            _ => EIO, // Default to I/O error for unknown errors
        }
    }

    fn get_name(&self) -> &'static str {
        "POSIX Compatibility Layer"
    }
}

// ─── Global POSIX Compat Layer ───────────────────────────────────────────

static mut POSIX_COMPAT: PosixCompatLayer = PosixCompatLayer::new();

// ─── C-ABI Exports ─────────────────────────────────────────────────────────

/// Get global POSIX compatibility layer
pub unsafe fn get_posix_compat() -> &'static mut PosixCompatLayer {
    &mut POSIX_COMPAT
}

/// Handle POSIX syscall from userspace
#[no_mangle]
pub unsafe extern "C" fn posix_handle_syscall(
    syscall_num: I32,
    args: *const U64,
    args_count: SizeT,
) -> I64 {
    let args_slice = core::slice::from_raw_parts(args, args_count);
    
    let syscall = match syscall_num {
        0 => PosixSyscall::Read,
        1 => PosixSyscall::Write,
        2 => PosixSyscall::Open,
        3 => PosixSyscall::Close,
        4 => PosixSyscall::Stat,
        5 => PosixSyscall::Fstat,
        6 => PosixSyscall::Lstat,
        7 => PosixSyscall::Poll,
        8 => PosixSyscall::Lseek,
        9 => PosixSyscall::Mmap,
        10 => PosixSyscall::Mprotect,
        11 => PosixSyscall::Munmap,
        12 => PosixSyscall::Brk,
        13 => PosixSyscall::RtSigaction,
        14 => PosixSyscall::RtSigprocmask,
        16 => PosixSyscall::Ioctl,
        17 => PosixSyscall::Pread64,
        18 => PosixSyscall::Pwrite64,
        19 => PosixSyscall::Readv,
        20 => PosixSyscall::Writev,
        21 => PosixSyscall::Access,
        22 => PosixSyscall::Pipe,
        23 => PosixSyscall::Select,
        24 => PosixSyscall::SchedYield,
        25 => PosixSyscall::Mremap,
        28 => PosixSyscall::Madvise,
        32 => PosixSyscall::Dup,
        33 => PosixSyscall::Dup2,
        34 => PosixSyscall::Pause,
        35 => PosixSyscall::Nanosleep,
        39 => PosixSyscall::Getpid,
        41 => PosixSyscall::Socket,
        42 => PosixSyscall::Connect,
        43 => PosixSyscall::Accept,
        44 => PosixSyscall::Sendto,
        45 => PosixSyscall::Recvfrom,
        46 => PosixSyscall::Sendmsg,
        47 => PosixSyscall::Recvmsg,
        48 => PosixSyscall::Shutdown,
        49 => PosixSyscall::Bind,
        50 => PosixSyscall::Listen,
        51 => PosixSyscall::Getsockname,
        52 => PosixSyscall::Getpeername,
        53 => PosixSyscall::Socketpair,
        54 => PosixSyscall::Setsockopt,
        55 => PosixSyscall::Getsockopt,
        56 => PosixSyscall::Clone,
        57 => PosixSyscall::Fork,
        59 => PosixSyscall::Execve,
        60 => PosixSyscall::Exit,
        61 => PosixSyscall::Wait4,
        62 => PosixSyscall::Kill,
        63 => PosixSyscall::Uname,
        _ => return -ENOSYS as I64,
    };
    
    POSIX_COMPAT.handle_syscall(syscall, args_slice)
}

/// Enable POSIX compatibility
#[no_mangle]
pub unsafe extern "C" fn posix_enable(enabled: bool) {
    POSIX_COMPAT.set_enabled(enabled);
}

/// Enable path translation
#[no_mangle]
pub unsafe extern "C" fn posix_set_path_translation(translate: bool) {
    POSIX_COMPAT.set_path_translation(translate);
}

/// Enable fork emulation
#[no_mangle]
pub unsafe extern "C" fn posix_set_fork_emulation(emulate: bool) {
    POSIX_COMPAT.set_fork_emulation(emulate);
}
