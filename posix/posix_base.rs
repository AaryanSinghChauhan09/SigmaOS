// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// posix/posix_base.rs — POSIX Compatibility Layer Base
//
// Provides the core OOP abstractions for POSIX compatibility in SigmaOS.
// Maps POSIX APIs to SigmaOS microkernel calls.
//
// Language: Rust (no_std for kernel compatibility)

#![no_std]

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;
type I64 = i64;
type Isize = isize;
type Usize = usize;

// ─── Error Codes (errno) ─────────────────────────────────────

pub static mut ERRNO: I32 = 0;

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
pub const EWOULDBLOCK: I32 = EAGAIN;
pub const EINPROGRESS: I32 = 115;
pub const EALREADY: I32 = 114;
pub const ENOTSOCK: I32 = 88;
pub const EDESTADDRREQ: I32 = 89;
pub const EMSGSIZE: I32 = 90;
pub const EPROTOTYPE: I32 = 91;
pub const ENOPROTOOPT: I32 = 92;
pub const EPROTONOSUPPORT: I32 = 93;
pub const ESOCKTNOSUPPORT: I32 = 94;
pub const EOPNOTSUPP: I32 = 95;
pub const EPFNOSUPPORT: I32 = 96;
pub const EAFNOSUPPORT: I32 = 97;
pub const EADDRINUSE: I32 = 98;
pub const EADDRNOTAVAIL: I32 = 99;
pub const ENETDOWN: I32 = 100;
pub const ENETUNREACH: I32 = 101;
pub const ENETRESET: I32 = 102;
pub const ECONNABORTED: I32 = 103;
pub const ECONNRESET: I32 = 104;
pub const ENOBUFS: I32 = 105;
pub const EISCONN: I32 = 106;
pub const ENOTCONN: I32 = 107;
pub const ESHUTDOWN: I32 = 108;
pub const ETOOMANYREFS: I32 = 109;
pub const ETIMEDOUT: I32 = 110;
pub const ECONNREFUSED: I32 = 111;
pub const EHOSTDOWN: I32 = 112;
pub const EHOSTUNREACH: I32 = 113;

// ─── File Flags ─────────────────────────────────────────────

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

// ─── Signal Definitions ─────────────────────────────────────

pub const SIGHUP: I32 = 1;
pub const SIGINT: I32 = 2;
pub const SIGQUIT: I32 = 3;
pub const SIGILL: I32 = 4;
pub const SIGTRAP: I32 = 5;
pub const SIGABRT: I32 = 6;
pub const SIGBUS: I32 = 7;
pub const SIGFPE: I32 = 8;
pub const SIGKILL: I32 = 9;
pub const SIGUSR1: I32 = 10;
pub const SIGSEGV: I32 = 11;
pub const SIGUSR2: I32 = 12;
pub const SIGPIPE: I32 = 13;
pub const SIGALRM: I32 = 14;
pub const SIGTERM: I32 = 15;
pub const SIGCHLD: I32 = 17;
pub const SIGCONT: I32 = 18;
pub const SIGSTOP: I32 = 19;
pub const SIGTSTP: I32 = 20;
pub const SIGTTIN: I32 = 21;
pub const SIGTTOU: I32 = 22;

// ─── Socket Definitions ─────────────────────────────────────

pub const AF_INET: I32 = 2;
pub const AF_INET6: I32 = 10;
pub const AF_UNIX: I32 = 1;
pub const AF_UNSPEC: I32 = 0;

pub const SOCK_STREAM: I32 = 1;
pub const SOCK_DGRAM: I32 = 2;
pub const SOCK_RAW: I32 = 3;
pub const SOCK_SEQPACKET: I32 = 5;

pub const IPPROTO_TCP: I32 = 6;
pub const IPPROTO_UDP: I32 = 17;
pub const IPPROTO_IP: I32 = 0;
pub const IPPROTO_IPV6: I32 = 41;

pub const SOL_SOCKET: I32 = 1;
pub const SO_REUSEADDR: I32 = 2;
pub const SO_KEEPALIVE: I32 = 9;
pub const SO_ERROR: I32 = 4;

// ─── SigmaOS Kernel Handle Types (stubs) ───────────────────

// These would be defined in the SigmaOS kernel
// For now, we use opaque handles

#[repr(C)]
pub struct SigmaFileHandle {
    pub handle: U64,
}

impl SigmaFileHandle {
    pub const fn new() -> Self {
        SigmaFileHandle { handle: 0 }
    }
}

#[repr(C)]
pub struct SigmaProcess {
    pub handle: U64,
}

impl SigmaProcess {
    pub const fn new() -> Self {
        SigmaProcess { handle: 0 }
    }
}

#[repr(C)]
pub struct SigmaSocket {
    pub handle: U64,
}

impl SigmaSocket {
    pub const fn new() -> Self {
        SigmaSocket { handle: 0 }
    }
}

// ─── Process State ─────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessState {
    Running,
    Sleeping,
    Stopped,
    Zombie,
    Dead,
}

// ─── Address Family ─────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AddressFamily {
    Unspecified,
    Unix,
    Inet,
    Inet6,
}

impl From<I32> for AddressFamily {
    fn from(value: I32) -> Self {
        match value {
            AF_UNIX => AddressFamily::Unix,
            AF_INET => AddressFamily::Inet,
            AF_INET6 => AddressFamily::Inet6,
            _ => AddressFamily::Unspecified,
        }
    }
}

// ─── Socket Type ───────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SocketType {
    Stream,
    Datagram,
    Raw,
    SeqPacket,
}

impl From<I32> for SocketType {
    fn from(value: I32) -> Self {
        match value {
            SOCK_STREAM => SocketType::Stream,
            SOCK_DGRAM => SocketType::Datagram,
            SOCK_RAW => SocketType::Raw,
            SOCK_SEQPACKET => SocketType::SeqPacket,
            _ => SocketType::Stream,
        }
    }
}

// ─── File Descriptor ───────────────────────────────────────

#[repr(C)]
pub struct PosixFileDescriptor {
    pub fd: I32,
    pub sigma_handle: SigmaFileHandle,
    pub flags: U32,
    pub mode: U32,
    pub is_open: bool,
}

impl PosixFileDescriptor {
    pub const fn new() -> Self {
        PosixFileDescriptor {
            fd: -1,
            sigma_handle: SigmaFileHandle::new(),
            flags: 0,
            mode: 0,
            is_open: false,
        }
    }
}

// ─── Process ───────────────────────────────────────────────

#[repr(C)]
pub struct PosixProcess {
    pub pid: I32,
    pub sigma_process: SigmaProcess,
    pub parent_pid: I32,
    pub state: ProcessState,
    pub exit_code: I32,
}

impl PosixProcess {
    pub const fn new() -> Self {
        PosixProcess {
            pid: -1,
            sigma_process: SigmaProcess::new(),
            parent_pid: -1,
            state: ProcessState::Dead,
            exit_code: 0,
        }
    }
}

// ─── Socket ───────────────────────────────────────────────

#[repr(C)]
pub struct PosixSocket {
    pub fd: I32,
    pub sigma_socket: SigmaSocket,
    pub domain: AddressFamily,
    pub type_: SocketType,
    pub protocol: I32,
    pub is_bound: bool,
    pub is_connected: bool,
    pub is_listening: bool,
}

impl PosixSocket {
    pub const fn new() -> Self {
        PosixSocket {
            fd: -1,
            sigma_socket: SigmaSocket::new(),
            domain: AddressFamily::Unspecified,
            type_: SocketType::Stream,
            protocol: 0,
            is_bound: false,
            is_connected: false,
            is_listening: false,
        }
    }
}

// ─── File Descriptor Table ───────────────────────────────

pub const MAX_FDS: Usize = 1024;

pub struct FileDescriptorTable {
    pub fds: [PosixFileDescriptor; MAX_FDS],
    pub next_fd: I32,
}

impl FileDescriptorTable {
    pub const fn new() -> Self {
        FileDescriptorTable {
            fds: [PosixFileDescriptor::new(); MAX_FDS],
            next_fd: 0,
        }
    }

    pub fn allocate_fd(&mut self) -> I32 {
        if self.next_fd >= MAX_FDS as I32 {
            unsafe { ERRNO = EMFILE };
            return -1;
        }

        let fd = self.next_fd;
        self.next_fd += 1;
        fd
    }

    pub fn free_fd(&mut self, fd: I32) {
        if fd >= 0 && (fd as Usize) < MAX_FDS {
            self.fds[fd as Usize].is_open = false;
            self.fds[fd as Usize].fd = -1;
        }
    }

    pub fn get_fd(&mut self, fd: I32) -> Option<&mut PosixFileDescriptor> {
        if fd >= 0 && (fd as Usize) < MAX_FDS && self.fds[fd as Usize].is_open {
            Some(&mut self.fds[fd as Usize])
        } else {
            None
        }
    }
}

// ─── Global State ─────────────────────────────────────────

static mut FD_TABLE: FileDescriptorTable = FileDescriptorTable::new();

// ─── Helper Functions ─────────────────────────────────────

/// Set errno and return error value
pub fn set_errno_and_return(error: I32) -> I32 {
    unsafe { ERRNO = error };
    -1
}

/// Get current errno
pub fn get_errno() -> I32 {
    unsafe { ERRNO }
}

/// Clear errno
pub fn clear_errno() {
    unsafe { ERRNO = 0 };
}

// ─── C-ABI Exports ─────────────────────────────────────────

#[no_mangle]
pub extern "C" fn posix_get_errno() -> I32 {
    get_errno()
}

#[no_mangle]
pub extern "C" fn posix_set_errno(error: I32) {
    unsafe { ERRNO = error };
}

#[no_mangle]
pub extern "C" fn posix_strerror(error: I32) -> *const U8 {
    // Return static error messages
    static mut ERROR_MESSAGES: [&str; 35] = [
        "Success",
        "Operation not permitted",
        "No such file or directory",
        "No such process",
        "Interrupted system call",
        "I/O error",
        "No such device or address",
        "Argument list too long",
        "Exec format error",
        "Bad file number",
        "No child processes",
        "Try again",
        "Out of memory",
        "Permission denied",
        "Bad address",
        "Block device required",
        "Device or resource busy",
        "File exists",
        "Cross-device link",
        "No such device",
        "Not a directory",
        "Is a directory",
        "Invalid argument",
        "File table overflow",
        "Too many open files",
        "Not a typewriter",
        "Text file busy",
        "File too large",
        "No space left on device",
        "Illegal seek",
        "Read-only file system",
        "Too many links",
        "Broken pipe",
        "Math argument out of domain of func",
        "Math result not representable",
    ];

    if error >= 0 && (error as Usize) < ERROR_MESSAGES.len() {
        unsafe { ERROR_MESSAGES[error as Usize].as_ptr() as *const U8 }
    } else {
        unsafe { ERROR_MESSAGES[0].as_ptr() as *const U8 }
    }
}
