// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/sigma_syscall_dispatch.rs — Syscall Dispatch Table
// Implements 30 core syscalls + extensible dispatch mechanism.
//
// x86-64 ABI: syscall number in rax, args in rdi, rsi, rdx, r10, r8, r9.
// Return value in rax; negative = errno (negated).
//
// Syscall table is inspired by Linux (but uses SigmaOS-native numbers).

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, Ordering};

// ── Syscall numbers ────────────────────────────────────────────────────────
#[allow(non_camel_case_types)]
#[repr(u64)]
pub enum Syscall {
    // I/O
    SYS_READ    = 0,
    SYS_WRITE   = 1,
    SYS_OPEN    = 2,
    SYS_CLOSE   = 3,
    SYS_STAT    = 4,
    SYS_FSTAT   = 5,
    SYS_LSEEK   = 6,
    SYS_IOCTL   = 7,

    // Memory
    SYS_MMAP    = 8,
    SYS_MUNMAP  = 9,
    SYS_BRK     = 10,
    SYS_MPROTECT = 11,

    // Process
    SYS_FORK    = 12,
    SYS_EXEC    = 13,
    SYS_EXIT    = 14,
    SYS_WAIT    = 15,
    SYS_GETPID  = 16,
    SYS_GETPPID = 17,
    SYS_KILL    = 18,

    // Scheduling
    SYS_SCHED_YIELD = 19,
    SYS_NANOSLEEP   = 20,
    SYS_SETPRIORITY = 21,

    // Filesystem
    SYS_MKDIR   = 22,
    SYS_RMDIR   = 23,
    SYS_UNLINK  = 24,
    SYS_RENAME  = 25,
    SYS_CHDIR   = 26,
    SYS_GETCWD  = 27,

    // Network
    SYS_SOCKET  = 28,
    SYS_CONNECT = 29,

    // Security (SigmaOS-native)
    SYS_PLEDGE  = 100,
    SYS_UNVEIL  = 101,
    SYS_SANDBOX = 102,

    SYS_UNKNOWN = 0xFFFF,
}

impl Syscall {
    pub fn from_u64(n: u64) -> Self {
        match n {
            0  => Syscall::SYS_READ,
            1  => Syscall::SYS_WRITE,
            2  => Syscall::SYS_OPEN,
            3  => Syscall::SYS_CLOSE,
            4  => Syscall::SYS_STAT,
            5  => Syscall::SYS_FSTAT,
            6  => Syscall::SYS_LSEEK,
            7  => Syscall::SYS_IOCTL,
            8  => Syscall::SYS_MMAP,
            9  => Syscall::SYS_MUNMAP,
            10 => Syscall::SYS_BRK,
            11 => Syscall::SYS_MPROTECT,
            12 => Syscall::SYS_FORK,
            13 => Syscall::SYS_EXEC,
            14 => Syscall::SYS_EXIT,
            15 => Syscall::SYS_WAIT,
            16 => Syscall::SYS_GETPID,
            17 => Syscall::SYS_GETPPID,
            18 => Syscall::SYS_KILL,
            19 => Syscall::SYS_SCHED_YIELD,
            20 => Syscall::SYS_NANOSLEEP,
            21 => Syscall::SYS_SETPRIORITY,
            22 => Syscall::SYS_MKDIR,
            23 => Syscall::SYS_RMDIR,
            24 => Syscall::SYS_UNLINK,
            25 => Syscall::SYS_RENAME,
            26 => Syscall::SYS_CHDIR,
            27 => Syscall::SYS_GETCWD,
            28 => Syscall::SYS_SOCKET,
            29 => Syscall::SYS_CONNECT,
            100 => Syscall::SYS_PLEDGE,
            101 => Syscall::SYS_UNVEIL,
            102 => Syscall::SYS_SANDBOX,
            _  => Syscall::SYS_UNKNOWN,
        }
    }
}

// ── Error codes ────────────────────────────────────────────────────────────
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum Errno {
    SUCCESS     =  0,
    EPERM       = -1,
    ENOENT      = -2,
    ESRCH       = -3,
    EINTR       = -4,
    EIO         = -5,
    EBADF       = -9,
    ECHILD      = -10,
    ENOMEM      = -12,
    EACCES      = -13,
    EFAULT      = -14,
    EEXIST      = -17,
    EINVAL      = -22,
    ENOSYS      = -38,
}

// ── Syscall context ────────────────────────────────────────────────────────
/// All arguments passed by the CPU on syscall entry.
#[repr(C)]
pub struct SyscallCtx {
    pub num: u64,   // rax — syscall number
    pub a0:  u64,   // rdi
    pub a1:  u64,   // rsi
    pub a2:  u64,   // rdx
    pub a3:  u64,   // r10
    pub a4:  u64,   // r8
    pub a5:  u64,   // r9
    pub pid: u32,   // current PID (filled by kernel)
}

// ── Syscall statistics ────────────────────────────────────────────────────
static SYSCALL_COUNTS: [AtomicU64; 256] = {
    // const init trick for atomic arrays
    const ZERO: AtomicU64 = AtomicU64::new(0);
    [ZERO; 256]
};

pub fn syscall_count(num: u64) -> u64 {
    SYSCALL_COUNTS.get(num as usize)
        .map(|c| c.load(Ordering::Relaxed))
        .unwrap_or(0)
}

// ── Main dispatch function ─────────────────────────────────────────────────

/// Entry point called from assembly `syscall` handler.
/// Returns value to place in rax (negative = error).
#[no_mangle]
pub extern "C" fn sigma_syscall_dispatch(ctx: &SyscallCtx) -> i64 {
    let n = ctx.num;
    if (n as usize) < 256 {
        SYSCALL_COUNTS[n as usize].fetch_add(1, Ordering::Relaxed);
    }

    // Security filter: check sigma_pledge / sigma_seccomp before dispatch
    if !security_check(ctx) {
        return Errno::EPERM as i64;
    }

    match Syscall::from_u64(n) {
        Syscall::SYS_READ    => sys_read(ctx.a0 as i32, ctx.a1 as *mut u8, ctx.a2 as usize),
        Syscall::SYS_WRITE   => sys_write(ctx.a0 as i32, ctx.a1 as *const u8, ctx.a2 as usize),
        Syscall::SYS_OPEN    => sys_open(ctx.a0 as *const u8, ctx.a1 as u32, ctx.a2 as u32),
        Syscall::SYS_CLOSE   => sys_close(ctx.a0 as i32),
        Syscall::SYS_STAT    => sys_stat(ctx.a0 as *const u8, ctx.a1 as *mut FileStat),
        Syscall::SYS_FSTAT   => sys_fstat(ctx.a0 as i32, ctx.a1 as *mut FileStat),
        Syscall::SYS_LSEEK   => sys_lseek(ctx.a0 as i32, ctx.a1 as i64, ctx.a2 as i32),
        Syscall::SYS_IOCTL   => sys_ioctl(ctx.a0 as i32, ctx.a1, ctx.a2),
        Syscall::SYS_MMAP    => sys_mmap(ctx.a0, ctx.a1 as usize, ctx.a2 as u32, ctx.a3 as u32, ctx.a4 as i32, ctx.a5 as i64),
        Syscall::SYS_MUNMAP  => sys_munmap(ctx.a0, ctx.a1 as usize),
        Syscall::SYS_BRK     => sys_brk(ctx.a0),
        Syscall::SYS_MPROTECT => sys_mprotect(ctx.a0, ctx.a1 as usize, ctx.a2 as u32),
        Syscall::SYS_FORK    => sys_fork(ctx.pid),
        Syscall::SYS_EXEC    => sys_exec(ctx.a0 as *const u8, ctx.a1 as *const *const u8, ctx.a2 as *const *const u8),
        Syscall::SYS_EXIT    => sys_exit(ctx.a0 as i32),
        Syscall::SYS_WAIT    => sys_wait(ctx.a0 as *mut i32),
        Syscall::SYS_GETPID  => ctx.pid as i64,
        Syscall::SYS_GETPPID => sys_getppid(ctx.pid),
        Syscall::SYS_KILL    => sys_kill(ctx.a0 as u32, ctx.a1 as u32),
        Syscall::SYS_SCHED_YIELD => { crate::kernel::sched::yield_cpu(); 0 },
        Syscall::SYS_NANOSLEEP   => sys_nanosleep(ctx.a0 as *const Timespec),
        Syscall::SYS_SETPRIORITY => sys_setpriority(ctx.a0 as i32, ctx.a1 as u32, ctx.a2 as i32),
        Syscall::SYS_MKDIR   => sys_mkdir(ctx.a0 as *const u8, ctx.a1 as u32),
        Syscall::SYS_RMDIR   => sys_rmdir(ctx.a0 as *const u8),
        Syscall::SYS_UNLINK  => sys_unlink(ctx.a0 as *const u8),
        Syscall::SYS_RENAME  => sys_rename(ctx.a0 as *const u8, ctx.a1 as *const u8),
        Syscall::SYS_CHDIR   => sys_chdir(ctx.a0 as *const u8),
        Syscall::SYS_GETCWD  => sys_getcwd(ctx.a0 as *mut u8, ctx.a1 as usize),
        Syscall::SYS_SOCKET  => sys_socket(ctx.a0 as i32, ctx.a1 as i32, ctx.a2 as i32),
        Syscall::SYS_CONNECT => sys_connect(ctx.a0 as i32, ctx.a1, ctx.a2 as u32),
        Syscall::SYS_PLEDGE  => sys_pledge(ctx.a0 as *const u8, ctx.pid),
        Syscall::SYS_UNVEIL  => sys_unveil(ctx.a0 as *const u8, ctx.a1 as *const u8, ctx.pid),
        Syscall::SYS_SANDBOX => sys_sandbox(ctx.a0, ctx.pid),
        Syscall::SYS_UNKNOWN => Errno::ENOSYS as i64,
    }
}

// ── Security gate ──────────────────────────────────────────────────────────
fn security_check(ctx: &SyscallCtx) -> bool {
    // Delegate to sigma_pledge + sigma_seccomp filters
    // Returns false if the syscall is denied for this PID
    crate::kernel::security::pledge_check(ctx.pid, ctx.num) &&
    crate::kernel::security::seccomp_check(ctx.pid, ctx.num)
}

// ── Timespec ──────────────────────────────────────────────────────────────
#[repr(C)]
pub struct Timespec {
    pub tv_sec:  i64,
    pub tv_nsec: i64,
}

// ── FileStat ──────────────────────────────────────────────────────────────
#[repr(C)]
pub struct FileStat {
    pub st_dev:   u64,
    pub st_ino:   u64,
    pub st_mode:  u32,
    pub st_nlink: u32,
    pub st_uid:   u32,
    pub st_gid:   u32,
    pub st_size:  i64,
    pub st_atime: i64,
    pub st_mtime: i64,
    pub st_ctime: i64,
}

// ── Syscall implementations (stub bodies, wire to VFS/MM/Proc) ────────────

fn sys_read(fd: i32, buf: *mut u8, count: usize) -> i64 {
    crate::kernel::vfs::vfs_read(fd, buf, count)
}
fn sys_write(fd: i32, buf: *const u8, count: usize) -> i64 {
    crate::kernel::vfs::vfs_write(fd, buf, count)
}
fn sys_open(path: *const u8, flags: u32, mode: u32) -> i64 {
    crate::kernel::vfs::vfs_open(path, flags, mode)
}
fn sys_close(fd: i32) -> i64 {
    crate::kernel::vfs::vfs_close(fd)
}
fn sys_stat(path: *const u8, stat: *mut FileStat) -> i64 {
    crate::kernel::vfs::vfs_stat(path, stat as *mut u8)
}
fn sys_fstat(fd: i32, stat: *mut FileStat) -> i64 {
    crate::kernel::vfs::vfs_fstat(fd, stat as *mut u8)
}
fn sys_lseek(fd: i32, offset: i64, whence: i32) -> i64 {
    crate::kernel::vfs::vfs_lseek(fd, offset, whence)
}
fn sys_ioctl(fd: i32, request: u64, arg: u64) -> i64 {
    crate::kernel::vfs::vfs_ioctl(fd, request, arg)
}
fn sys_mmap(addr: u64, len: usize, prot: u32, flags: u32, fd: i32, off: i64) -> i64 {
    crate::kernel::mm::mm_mmap(addr, len, prot, flags, fd, off)
}
fn sys_munmap(addr: u64, len: usize) -> i64 {
    crate::kernel::mm::mm_munmap(addr, len)
}
fn sys_brk(addr: u64) -> i64 {
    crate::kernel::mm::mm_brk(addr)
}
fn sys_mprotect(addr: u64, len: usize, prot: u32) -> i64 {
    crate::kernel::mm::mm_mprotect(addr, len, prot)
}
fn sys_fork(pid: u32) -> i64 {
    crate::kernel::proc::proc_fork(pid)
}
fn sys_exec(path: *const u8, argv: *const *const u8, envp: *const *const u8) -> i64 {
    crate::kernel::proc::proc_exec(path, argv, envp)
}
fn sys_exit(code: i32) -> i64 {
    crate::kernel::proc::proc_exit(code); 0
}
fn sys_wait(status: *mut i32) -> i64 {
    crate::kernel::proc::proc_wait(status)
}
fn sys_getppid(pid: u32) -> i64 {
    crate::kernel::proc::proc_getppid(pid)
}
fn sys_kill(target: u32, sig: u32) -> i64 {
    crate::kernel::proc::proc_kill(target, sig)
}
fn sys_nanosleep(ts: *const Timespec) -> i64 {
    if ts.is_null() { return Errno::EFAULT as i64; }
    let ts = unsafe { &*ts };
    let ms = ts.tv_sec as u64 * 1000 + ts.tv_nsec as u64 / 1_000_000;
    crate::kernel::core::sigma_irq::sleep_ms(ms);
    0
}
fn sys_setpriority(which: i32, who: u32, prio: i32) -> i64 {
    crate::kernel::sched::set_priority(who, prio)
}
fn sys_mkdir(path: *const u8, mode: u32) -> i64 {
    crate::kernel::vfs::vfs_mkdir(path, mode)
}
fn sys_rmdir(path: *const u8) -> i64 {
    crate::kernel::vfs::vfs_rmdir(path)
}
fn sys_unlink(path: *const u8) -> i64 {
    crate::kernel::vfs::vfs_unlink(path)
}
fn sys_rename(old: *const u8, new: *const u8) -> i64 {
    crate::kernel::vfs::vfs_rename(old, new)
}
fn sys_chdir(path: *const u8) -> i64 {
    crate::kernel::vfs::vfs_chdir(path)
}
fn sys_getcwd(buf: *mut u8, size: usize) -> i64 {
    crate::kernel::vfs::vfs_getcwd(buf, size)
}
fn sys_socket(domain: i32, sock_type: i32, protocol: i32) -> i64 {
    crate::kernel::net::net_socket(domain, sock_type, protocol)
}
fn sys_connect(fd: i32, addr: u64, addrlen: u32) -> i64 {
    crate::kernel::net::net_connect(fd, addr, addrlen)
}
fn sys_pledge(promises: *const u8, pid: u32) -> i64 {
    crate::kernel::security::pledge_set(pid, promises)
}
fn sys_unveil(path: *const u8, perms: *const u8, pid: u32) -> i64 {
    crate::kernel::security::unveil_add(pid, path, perms)
}
fn sys_sandbox(flags: u64, pid: u32) -> i64 {
    crate::kernel::security::sandbox_enter(pid, flags)
}
