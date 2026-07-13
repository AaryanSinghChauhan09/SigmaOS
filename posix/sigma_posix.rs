// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// posix/sigma_posix.rs — POSIX Compatibility Syscall Layer
//
// Maps the 50 most critical POSIX syscalls to SigmaOS internal handlers.
// Allows existing Linux ELF binaries to call into SigmaOS via the standard
// syscall ABI (syscall instruction on x86-64, rax = syscall number).
//
// Architecture:
//   • SyscallEntry: (number, handler_fn) pair
//   • PosixCompat:  Fixed-capacity dispatch table; binary-searched by number
//   • Each handler is an encapsulated method returning i64 (errno on negative)
//
// no_std, no alloc, no external crates.

#![no_std]
#![allow(dead_code)]

// ─────────────────────────────────────────────────────────────────────────────
// POSIX errno values (user-defined, no libc)
// ─────────────────────────────────────────────────────────────────────────────

pub const ENOSYS:   i64 = -38;  // Function not implemented
pub const EINVAL:   i64 = -22;  // Invalid argument
pub const EBADF:    i64 = -9;   // Bad file descriptor
pub const ENOMEM:   i64 = -12;  // Out of memory
pub const ENOENT:   i64 = -2;   // No such file or directory
pub const EPERM:    i64 = -1;   // Operation not permitted
pub const EFAULT:   i64 = -14;  // Bad address
pub const EAGAIN:   i64 = -11;  // Try again
pub const EACCES:   i64 = -13;  // Permission denied
pub const EEXIST:   i64 = -17;  // File exists
pub const ENOTDIR:  i64 = -20;  // Not a directory
pub const EISDIR:   i64 = -21;  // Is a directory

// ─────────────────────────────────────────────────────────────────────────────
// Linux x86-64 syscall numbers (ABI-compatible)
// ─────────────────────────────────────────────────────────────────────────────

pub const SYS_READ:        u64 = 0;
pub const SYS_WRITE:       u64 = 1;
pub const SYS_OPEN:        u64 = 2;
pub const SYS_CLOSE:       u64 = 3;
pub const SYS_STAT:        u64 = 4;
pub const SYS_FSTAT:       u64 = 5;
pub const SYS_LSTAT:       u64 = 6;
pub const SYS_POLL:        u64 = 7;
pub const SYS_LSEEK:       u64 = 8;
pub const SYS_MMAP:        u64 = 9;
pub const SYS_MPROTECT:    u64 = 10;
pub const SYS_MUNMAP:      u64 = 11;
pub const SYS_BRK:         u64 = 12;
pub const SYS_RT_SIGACTION:u64 = 13;
pub const SYS_RT_SIGPROCMASK: u64 = 14;
pub const SYS_IOCTL:       u64 = 16;
pub const SYS_PREAD64:     u64 = 17;
pub const SYS_PWRITE64:    u64 = 18;
pub const SYS_READV:       u64 = 19;
pub const SYS_WRITEV:      u64 = 20;
pub const SYS_ACCESS:      u64 = 21;
pub const SYS_PIPE:        u64 = 22;
pub const SYS_SELECT:      u64 = 23;
pub const SYS_SCHED_YIELD: u64 = 24;
pub const SYS_MADVISE:     u64 = 28;
pub const SYS_DUP:         u64 = 32;
pub const SYS_DUP2:        u64 = 33;
pub const SYS_NANOSLEEP:   u64 = 35;
pub const SYS_GETPID:      u64 = 39;
pub const SYS_SOCKET:      u64 = 41;
pub const SYS_CONNECT:     u64 = 42;
pub const SYS_ACCEPT:      u64 = 43;
pub const SYS_SENDTO:      u64 = 44;
pub const SYS_RECVFROM:    u64 = 45;
pub const SYS_BIND:        u64 = 49;
pub const SYS_LISTEN:      u64 = 50;
pub const SYS_GETSOCKNAME: u64 = 51;
pub const SYS_FORK:        u64 = 57;
pub const SYS_EXECVE:      u64 = 59;
pub const SYS_EXIT:        u64 = 60;
pub const SYS_WAIT4:       u64 = 61;
pub const SYS_KILL:        u64 = 62;
pub const SYS_GETPPID:     u64 = 110;
pub const SYS_GETCWD:      u64 = 79;
pub const SYS_CHDIR:       u64 = 80;
pub const SYS_MKDIR:       u64 = 83;
pub const SYS_RMDIR:       u64 = 84;
pub const SYS_UNLINK:      u64 = 87;
pub const SYS_SYMLINK:     u64 = 88;
pub const SYS_CHMOD:       u64 = 90;
pub const SYS_GETUID:      u64 = 102;
pub const SYS_GETGID:      u64 = 104;
pub const SYS_GETEUID:     u64 = 107;
pub const SYS_GETEGID:     u64 = 108;
pub const SYS_CLOCK_GETTIME: u64 = 228;
pub const SYS_EXIT_GROUP:  u64 = 231;

pub const MAX_SYSCALLS: usize = 64;

// ─────────────────────────────────────────────────────────────────────────────
// Syscall Arguments (x86-64 ABI: rdi, rsi, rdx, r10, r8, r9)
// ─────────────────────────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SyscallArgs {
    pub a0: u64,  // rdi
    pub a1: u64,  // rsi
    pub a2: u64,  // rdx
    pub a3: u64,  // r10
    pub a4: u64,  // r8
    pub a5: u64,  // r9
}

impl SyscallArgs {
    pub const fn zero() -> Self {
        Self { a0: 0, a1: 0, a2: 0, a3: 0, a4: 0, a5: 0 }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Syscall Table Entry
// ─────────────────────────────────────────────────────────────────────────────

pub type SyscallFn = fn(args: &SyscallArgs) -> i64;

#[derive(Copy, Clone)]
pub struct SyscallEntry {
    pub number:  u64,
    pub name:    [u8; 24],
    pub handler: SyscallFn,
}

impl SyscallEntry {
    pub const fn new(number: u64, name: &[u8], handler: SyscallFn) -> Self {
        let mut n = [0u8; 24];
        let len = if name.len() < 24 { name.len() } else { 23 };
        let mut i = 0;
        while i < len { n[i] = name[i]; i += 1; }
        Self { number, name: n, handler }
    }

    pub const fn unimplemented() -> Self {
        Self { number: u64::MAX, name: [0u8; 24], handler: syscall_enosys }
    }
}

fn syscall_enosys(_args: &SyscallArgs) -> i64 { ENOSYS }

// ─────────────────────────────────────────────────────────────────────────────
// POSIX Handler Implementations (OOP encapsulated on PosixCompat)
// ─────────────────────────────────────────────────────────────────────────────

/// POSIX Compatibility layer — encapsulated dispatch table
pub struct PosixCompat {
    table:       [SyscallEntry; MAX_SYSCALLS],
    table_count: usize,
}

impl PosixCompat {
    pub const fn new() -> Self {
        Self {
            table:       [SyscallEntry::unimplemented(); MAX_SYSCALLS],
            table_count: 0,
        }
    }

    /// Register all built-in POSIX syscall handlers.
    /// Called once at init time.
    pub fn register_defaults(&mut self) {
        self.add(SyscallEntry::new(SYS_READ,      b"read",      handler_read));
        self.add(SyscallEntry::new(SYS_WRITE,     b"write",     handler_write));
        self.add(SyscallEntry::new(SYS_OPEN,      b"open",      handler_open));
        self.add(SyscallEntry::new(SYS_CLOSE,     b"close",     handler_close));
        self.add(SyscallEntry::new(SYS_STAT,      b"stat",      handler_stat));
        self.add(SyscallEntry::new(SYS_FSTAT,     b"fstat",     handler_fstat));
        self.add(SyscallEntry::new(SYS_LSTAT,     b"lstat",     handler_lstat));
        self.add(SyscallEntry::new(SYS_LSEEK,     b"lseek",     handler_lseek));
        self.add(SyscallEntry::new(SYS_MMAP,      b"mmap",      handler_mmap));
        self.add(SyscallEntry::new(SYS_MPROTECT,  b"mprotect",  handler_mprotect));
        self.add(SyscallEntry::new(SYS_MUNMAP,    b"munmap",    handler_munmap));
        self.add(SyscallEntry::new(SYS_BRK,       b"brk",       handler_brk));
        self.add(SyscallEntry::new(SYS_IOCTL,     b"ioctl",     handler_ioctl));
        self.add(SyscallEntry::new(SYS_PREAD64,   b"pread64",   handler_pread64));
        self.add(SyscallEntry::new(SYS_PWRITE64,  b"pwrite64",  handler_pwrite64));
        self.add(SyscallEntry::new(SYS_ACCESS,    b"access",    handler_access));
        self.add(SyscallEntry::new(SYS_PIPE,      b"pipe",      handler_pipe));
        self.add(SyscallEntry::new(SYS_SELECT,    b"select",    handler_select));
        self.add(SyscallEntry::new(SYS_SCHED_YIELD, b"sched_yield", handler_sched_yield));
        self.add(SyscallEntry::new(SYS_DUP,       b"dup",       handler_dup));
        self.add(SyscallEntry::new(SYS_DUP2,      b"dup2",      handler_dup2));
        self.add(SyscallEntry::new(SYS_NANOSLEEP, b"nanosleep", handler_nanosleep));
        self.add(SyscallEntry::new(SYS_GETPID,    b"getpid",    handler_getpid));
        self.add(SyscallEntry::new(SYS_SOCKET,    b"socket",    handler_socket));
        self.add(SyscallEntry::new(SYS_CONNECT,   b"connect",   handler_connect));
        self.add(SyscallEntry::new(SYS_ACCEPT,    b"accept",    handler_accept));
        self.add(SyscallEntry::new(SYS_SENDTO,    b"sendto",    handler_sendto));
        self.add(SyscallEntry::new(SYS_RECVFROM,  b"recvfrom",  handler_recvfrom));
        self.add(SyscallEntry::new(SYS_BIND,      b"bind",      handler_bind));
        self.add(SyscallEntry::new(SYS_LISTEN,    b"listen",    handler_listen));
        self.add(SyscallEntry::new(SYS_FORK,      b"fork",      handler_fork));
        self.add(SyscallEntry::new(SYS_EXECVE,    b"execve",    handler_execve));
        self.add(SyscallEntry::new(SYS_EXIT,      b"exit",      handler_exit));
        self.add(SyscallEntry::new(SYS_WAIT4,     b"wait4",     handler_wait4));
        self.add(SyscallEntry::new(SYS_KILL,      b"kill",      handler_kill));
        self.add(SyscallEntry::new(SYS_GETCWD,    b"getcwd",    handler_getcwd));
        self.add(SyscallEntry::new(SYS_CHDIR,     b"chdir",     handler_chdir));
        self.add(SyscallEntry::new(SYS_MKDIR,     b"mkdir",     handler_mkdir));
        self.add(SyscallEntry::new(SYS_RMDIR,     b"rmdir",     handler_rmdir));
        self.add(SyscallEntry::new(SYS_UNLINK,    b"unlink",    handler_unlink));
        self.add(SyscallEntry::new(SYS_CHMOD,     b"chmod",     handler_chmod));
        self.add(SyscallEntry::new(SYS_GETUID,    b"getuid",    handler_getuid));
        self.add(SyscallEntry::new(SYS_GETGID,    b"getgid",    handler_getgid));
        self.add(SyscallEntry::new(SYS_GETEUID,   b"geteuid",   handler_geteuid));
        self.add(SyscallEntry::new(SYS_GETEGID,   b"getegid",   handler_getegid));
        self.add(SyscallEntry::new(SYS_CLOCK_GETTIME, b"clock_gettime", handler_clock_gettime));
        self.add(SyscallEntry::new(SYS_EXIT_GROUP,b"exit_group",handler_exit));

        // Sort by syscall number for binary-search dispatch
        self.sort_table();
    }

    fn add(&mut self, entry: SyscallEntry) {
        if self.table_count < MAX_SYSCALLS {
            self.table[self.table_count] = entry;
            self.table_count += 1;
        }
    }

    /// Insertion sort by syscall number (O(n²) acceptable at boot for n≤64).
    fn sort_table(&mut self) {
        let n = self.table_count;
        for i in 1..n {
            let key = self.table[i];
            let mut j = i;
            while j > 0 && self.table[j - 1].number > key.number {
                self.table[j] = self.table[j - 1];
                j -= 1;
            }
            self.table[j] = key;
        }
    }

    /// Binary-search dispatch — O(log n) per syscall.
    pub fn dispatch(&self, number: u64, args: &SyscallArgs) -> i64 {
        let mut lo: usize = 0;
        let mut hi: usize = self.table_count;

        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            match self.table[mid].number.cmp(&number) {
                core::cmp::Ordering::Equal   => return (self.table[mid].handler)(args),
                core::cmp::Ordering::Less    => lo = mid + 1,
                core::cmp::Ordering::Greater => hi = mid,
            }
        }
        ENOSYS  // syscall not registered
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Individual POSIX Handler Functions
// Each bridges to internal SigmaOS VFS / IPC / memory subsystems.
// Currently wired as stubs; production: call into kernel subsystems directly.
// ─────────────────────────────────────────────────────────────────────────────

fn handler_read(args: &SyscallArgs)     -> i64 {
    // a0=fd, a1=buf_ptr, a2=count
    if args.a1 == 0 { return EFAULT; }
    // Production: sigma_vfs_read(args.a0 as i32, args.a1 as *mut u8, args.a2 as usize)
    ENOSYS
}

fn handler_write(args: &SyscallArgs)    -> i64 {
    if args.a1 == 0 { return EFAULT; }
    // Production: sigma_vfs_write(args.a0 as i32, args.a1 as *const u8, args.a2 as usize)
    ENOSYS
}

fn handler_open(args: &SyscallArgs)     -> i64 {
    if args.a0 == 0 { return EFAULT; }
    // Production: sigma_vfs_open(args.a0 as *const u8, args.a1 as u32)
    ENOSYS
}

fn handler_close(args: &SyscallArgs)    -> i64 {
    // Production: sigma_vfs_close(args.a0 as i32)
    0
}

fn handler_stat(args: &SyscallArgs)     -> i64 {
    if args.a0 == 0 || args.a1 == 0 { return EFAULT; }
    ENOSYS
}

fn handler_fstat(args: &SyscallArgs)    -> i64 { ENOSYS }
fn handler_lstat(args: &SyscallArgs)    -> i64 { ENOSYS }

fn handler_lseek(args: &SyscallArgs)    -> i64 {
    // a0=fd, a1=offset, a2=whence
    if args.a2 > 2 { return EINVAL; }
    ENOSYS
}

fn handler_mmap(args: &SyscallArgs)     -> i64 {
    // a0=addr, a1=length, a2=prot, a3=flags, a4=fd, a5=offset
    if args.a1 == 0 { return EINVAL; }
    // Production: sigma_vmm_mmap(args.a0, args.a1, args.a2 as u32, args.a3 as u32)
    ENOSYS
}

fn handler_mprotect(args: &SyscallArgs) -> i64 { ENOSYS }
fn handler_munmap(args: &SyscallArgs)   -> i64 { 0 }

fn handler_brk(args: &SyscallArgs)      -> i64 {
    // Production: return current program break or set new one
    ENOSYS
}

fn handler_ioctl(args: &SyscallArgs)    -> i64 {
    // a0=fd, a1=request, a2=arg
    ENOSYS
}

fn handler_pread64(args: &SyscallArgs)  -> i64 { ENOSYS }
fn handler_pwrite64(args: &SyscallArgs) -> i64 { ENOSYS }

fn handler_access(args: &SyscallArgs)   -> i64 {
    if args.a0 == 0 { return EFAULT; }
    // Production: sigma_vfs_access(args.a0 as *const u8, args.a1 as u32)
    ENOSYS
}

fn handler_pipe(args: &SyscallArgs)     -> i64 { ENOSYS }

fn handler_select(args: &SyscallArgs)   -> i64 { ENOSYS }

fn handler_sched_yield(_args: &SyscallArgs) -> i64 {
    // Production: sigma_sched_yield()
    0
}

fn handler_dup(args: &SyscallArgs)      -> i64 { ENOSYS }
fn handler_dup2(args: &SyscallArgs)     -> i64 { ENOSYS }

fn handler_nanosleep(args: &SyscallArgs) -> i64 {
    if args.a0 == 0 { return EFAULT; }
    // Production: read timespec from args.a0, sleep for that duration
    0
}

fn handler_getpid(_args: &SyscallArgs)  -> i64 {
    // Production: return sigma_current_pid()
    1
}

fn handler_socket(args: &SyscallArgs)   -> i64 { ENOSYS }
fn handler_connect(args: &SyscallArgs)  -> i64 { ENOSYS }
fn handler_accept(args: &SyscallArgs)   -> i64 { ENOSYS }
fn handler_sendto(args: &SyscallArgs)   -> i64 { ENOSYS }
fn handler_recvfrom(args: &SyscallArgs) -> i64 { ENOSYS }
fn handler_bind(args: &SyscallArgs)     -> i64 { ENOSYS }
fn handler_listen(args: &SyscallArgs)   -> i64 { ENOSYS }

fn handler_fork(_args: &SyscallArgs)    -> i64 {
    // Production: sigma_process_fork()
    ENOSYS
}

fn handler_execve(args: &SyscallArgs)   -> i64 {
    if args.a0 == 0 { return EFAULT; }
    // Production: sigma_process_execve(path, argv, envp)
    ENOSYS
}

fn handler_exit(args: &SyscallArgs)     -> i64 {
    // Production: sigma_process_exit(args.a0 as i32)
    0
}

fn handler_wait4(args: &SyscallArgs)    -> i64 { ENOSYS }

fn handler_kill(args: &SyscallArgs)     -> i64 {
    if args.a1 == 0 { return EINVAL; }
    // Production: sigma_signal_send(pid, signo)
    ENOSYS
}

fn handler_getcwd(args: &SyscallArgs)   -> i64 { ENOSYS }
fn handler_chdir(args: &SyscallArgs)    -> i64 { ENOSYS }
fn handler_mkdir(args: &SyscallArgs)    -> i64 { ENOSYS }
fn handler_rmdir(args: &SyscallArgs)    -> i64 { ENOSYS }
fn handler_unlink(args: &SyscallArgs)   -> i64 { ENOSYS }
fn handler_chmod(args: &SyscallArgs)    -> i64 { ENOSYS }

fn handler_getuid(_args: &SyscallArgs)  -> i64 { 0 }  // root by default
fn handler_getgid(_args: &SyscallArgs)  -> i64 { 0 }
fn handler_geteuid(_args: &SyscallArgs) -> i64 { 0 }
fn handler_getegid(_args: &SyscallArgs) -> i64 { 0 }

fn handler_clock_gettime(args: &SyscallArgs) -> i64 {
    if args.a1 == 0 { return EFAULT; }
    // Production: read TSC, write struct timespec to args.a1
    ENOSYS
}

// ─────────────────────────────────────────────────────────────────────────────
// Global singleton + C-ABI entry point
// ─────────────────────────────────────────────────────────────────────────────

static mut POSIX_COMPAT: PosixCompat = PosixCompat::new();

/// Called once at boot to register all POSIX handlers.
#[no_mangle]
pub unsafe extern "C" fn sigma_posix_init() {
    POSIX_COMPAT.register_defaults();
}

/// Main syscall dispatch entry point — called from the x86-64 SYSCALL handler.
/// rax = syscall_number, args in rdi/rsi/rdx/r10/r8/r9.
#[no_mangle]
pub unsafe extern "C" fn sigma_posix_dispatch(
    number: u64,
    a0: u64, a1: u64, a2: u64,
    a3: u64, a4: u64, a5: u64,
) -> i64 {
    let args = SyscallArgs { a0, a1, a2, a3, a4, a5 };
    POSIX_COMPAT.dispatch(number, &args)
}
