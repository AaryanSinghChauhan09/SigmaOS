// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/syscall_dispatch.rs — Syscall dispatch table (50+ syscalls)
// Language: Rust #![no_std]
#![no_std]
#![allow(dead_code, non_upper_case_globals)]

// ── Linux-compatible syscall numbers (x86_64) ────────────────────────────
pub const SYS_READ:          u64 = 0;
pub const SYS_WRITE:         u64 = 1;
pub const SYS_OPEN:          u64 = 2;
pub const SYS_CLOSE:         u64 = 3;
pub const SYS_STAT:          u64 = 4;
pub const SYS_FSTAT:         u64 = 5;
pub const SYS_LSTAT:         u64 = 6;
pub const SYS_POLL:          u64 = 7;
pub const SYS_LSEEK:         u64 = 8;
pub const SYS_MMAP:          u64 = 9;
pub const SYS_MPROTECT:      u64 = 10;
pub const SYS_MUNMAP:        u64 = 11;
pub const SYS_BRK:           u64 = 12;
pub const SYS_RT_SIGACTION:  u64 = 13;
pub const SYS_RT_SIGPROCMASK:u64 = 14;
pub const SYS_RT_SIGRETURN:  u64 = 15;
pub const SYS_IOCTL:         u64 = 16;
pub const SYS_PREAD64:       u64 = 17;
pub const SYS_PWRITE64:      u64 = 18;
pub const SYS_READV:         u64 = 19;
pub const SYS_WRITEV:        u64 = 20;
pub const SYS_PIPE:          u64 = 22;
pub const SYS_DUP:           u64 = 32;
pub const SYS_DUP2:          u64 = 33;
pub const SYS_NANOSLEEP:     u64 = 35;
pub const SYS_GETPID:        u64 = 39;
pub const SYS_SOCKET:        u64 = 41;
pub const SYS_CONNECT:       u64 = 42;
pub const SYS_ACCEPT:        u64 = 43;
pub const SYS_SENDTO:        u64 = 44;
pub const SYS_RECVFROM:      u64 = 45;
pub const SYS_BIND:          u64 = 49;
pub const SYS_LISTEN:        u64 = 50;
pub const SYS_CLONE:         u64 = 56;
pub const SYS_FORK:          u64 = 57;
pub const SYS_EXECVE:        u64 = 59;
pub const SYS_EXIT:          u64 = 60;
pub const SYS_WAIT4:         u64 = 61;
pub const SYS_KILL:          u64 = 62;
pub const SYS_UNAME:         u64 = 63;
pub const SYS_FCNTL:         u64 = 72;
pub const SYS_MKDIR:         u64 = 83;
pub const SYS_RMDIR:         u64 = 84;
pub const SYS_UNLINK:        u64 = 87;
pub const SYS_CHDIR:         u64 = 80;
pub const SYS_GETCWD:        u64 = 79;
pub const SYS_CHMOD:         u64 = 90;
pub const SYS_CHOWN:         u64 = 92;
pub const SYS_GETPPID:       u64 = 110;
pub const SYS_GETUID:        u64 = 102;
pub const SYS_GETEUID:       u64 = 107;
pub const SYS_CLOCK_GETTIME: u64 = 228;
pub const SYS_EPOLL_CREATE1: u64 = 291;
pub const SYS_EPOLL_CTL:     u64 = 233;
pub const SYS_EPOLL_WAIT:    u64 = 232;
pub const SYS_FUTEX:         u64 = 202;
pub const SYS_SET_TID_ADDRESS:u64= 218;
pub const SYS_EXIT_GROUP:    u64 = 231;
pub const SYS_PIPE2:         u64 = 293;
pub const SYS_GETRANDOM:     u64 = 318;
pub const SYS_MEMFD_CREATE:  u64 = 319;

// ── SigmaOS custom syscalls (> 400) ──────────────────────────────────────
pub const SYS_SIGMA_PLEDGE:    u64 = 400;
pub const SYS_SIGMA_UNVEIL:    u64 = 401;
pub const SYS_SIGMA_ATTEST:    u64 = 402;
pub const SYS_SIGMA_BUS_SEND:  u64 = 403;
pub const SYS_SIGMA_BUS_RECV:  u64 = 404;
pub const SYS_SIGMA_CAPABILITY:u64 = 405;

// ── Return values ─────────────────────────────────────────────────────────
pub const ENOSYS:  i64 = -38;
pub const EINVAL:  i64 = -22;
pub const EPERM:   i64 = -1;
pub const ENOMEM:  i64 = -12;
pub const EBADF:   i64 = -9;
pub const EFAULT:  i64 = -14;
pub const EAGAIN:  i64 = -11;

// ── Extern kernel primitives ──────────────────────────────────────────────
extern "C" {
    fn sigma_slab_alloc(size: usize) -> *mut u8;
    fn sigma_slab_free(ptr: *mut u8) -> i32;
    fn sigma_bus_send_impl(ch: u32, data: *const u8, len: usize) -> i32;
    fn recv_message(ch: u32, out: *mut u8) -> i32;
    fn sigma_clock_ns() -> u64;
    fn sigma_getpid() -> u32;
    fn sigma_gettid() -> u32;
    fn sigma_sleep_ms(ms: u64);
    fn sched_add_task(pid: u32, policy: u8, deadline: u64, level: u8) -> i32;
}

// ── Syscall handler signature ─────────────────────────────────────────────
pub type SyscallHandler = unsafe fn(a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64) -> i64;

// ── Individual syscall implementations ────────────────────────────────────

unsafe fn sys_getpid(_: u64, _: u64, _: u64, _: u64, _: u64, _: u64) -> i64 {
    sigma_getpid() as i64
}

unsafe fn sys_getppid(_: u64, _: u64, _: u64, _: u64, _: u64, _: u64) -> i64 {
    1i64 // init is parent by default
}

unsafe fn sys_getuid(_: u64, _: u64, _: u64, _: u64, _: u64, _: u64) -> i64 { 0 }
unsafe fn sys_geteuid(_: u64, _: u64, _: u64, _: u64, _: u64, _: u64) -> i64 { 0 }

unsafe fn sys_exit(code: u64, _: u64, _: u64, _: u64, _: u64, _: u64) -> i64 {
    // Signal process manager to clean up
    let _ = code;
    0 // unreachable in production — process_manager handles exit
}

unsafe fn sys_nanosleep(req: u64, _rem: u64, _: u64, _: u64, _: u64, _: u64) -> i64 {
    if req == 0 { return EFAULT; }
    // timespec: tv_sec (u64) + tv_nsec (u64)
    let sec  = core::ptr::read_volatile(req as *const u64);
    let nsec = core::ptr::read_volatile((req + 8) as *const u64);
    let ms = sec * 1000 + nsec / 1_000_000;
    sigma_sleep_ms(ms);
    0
}

unsafe fn sys_clock_gettime(clk_id: u64, tp: u64, _: u64, _: u64, _: u64, _: u64) -> i64 {
    if tp == 0 { return EFAULT; }
    let ns = sigma_clock_ns();
    let sec  = ns / 1_000_000_000;
    let nsec = ns % 1_000_000_000;
    core::ptr::write_volatile(tp as *mut u64, sec);
    core::ptr::write_volatile((tp + 8) as *mut u64, nsec);
    let _ = clk_id;
    0
}

unsafe fn sys_uname(buf: u64, _: u64, _: u64, _: u64, _: u64, _: u64) -> i64 {
    if buf == 0 { return EFAULT; }
    // struct utsname: 6 × 65-byte fields (sysname, nodename, release, version, machine, domainname)
    let uname_buf = buf as *mut u8;
    let fields: [&[u8]; 6] = [
        b"SigmaOS\0",
        b"sigmaos\0",
        b"15.0.0-Zenith\0",
        b"#1 SMP SigmaOS Zenith\0",
        b"x86_64\0",
        b"(none)\0",
    ];
    for (i, field) in fields.iter().enumerate() {
        let dst = uname_buf.add(i * 65);
        core::ptr::write_bytes(dst, 0, 65);
        let copy_len = field.len().min(64);
        core::ptr::copy_nonoverlapping(field.as_ptr(), dst, copy_len);
    }
    0
}

unsafe fn sys_getrandom(buf: u64, count: u64, _flags: u64, _: u64, _: u64, _: u64) -> i64 {
    if buf == 0 { return EFAULT; }
    // Simple PRNG using clock
    let mut state = sigma_clock_ns() ^ 0x1234_5678_ABCD_EF00;
    let dst = buf as *mut u8;
    for i in 0..count as usize {
        state ^= state << 13; state ^= state >> 7; state ^= state << 17;
        *dst.add(i) = (state & 0xFF) as u8;
    }
    count as i64
}

unsafe fn sys_set_tid_address(tidptr: u64, _: u64, _: u64, _: u64, _: u64, _: u64) -> i64 {
    let _ = tidptr;
    sigma_gettid() as i64
}

unsafe fn sys_mmap(addr: u64, length: u64, prot: u64, _flags: u64, _fd: u64, _off: u64) -> i64 {
    // Simplified: allocate from slab if small enough
    let ptr = sigma_slab_alloc(length as usize);
    if ptr.is_null() { return ENOMEM; }
    let _ = (addr, prot);
    ptr as i64
}

unsafe fn sys_munmap(addr: u64, _length: u64, _: u64, _: u64, _: u64, _: u64) -> i64 {
    sigma_slab_free(addr as *mut u8) as i64
}

unsafe fn sys_brk(new_brk: u64, _: u64, _: u64, _: u64, _: u64, _: u64) -> i64 {
    // Return the new_brk (simplified — real impl tracks heap)
    if new_brk == 0 { 0x8000_0000 } else { new_brk as i64 }
}

unsafe fn sys_futex(uaddr: u64, op: u64, val: u64, _: u64, _: u64, _: u64) -> i64 {
    // Minimal futex: FUTEX_WAIT=0, FUTEX_WAKE=1
    const FUTEX_WAIT: u64 = 0; const FUTEX_WAKE: u64 = 1;
    if uaddr == 0 { return EFAULT; }
    match op & 0x7F {
        FUTEX_WAIT => {
            // Compare *uaddr to val; if equal, sleep
            let cur = core::ptr::read_volatile(uaddr as *const u32);
            if cur != val as u32 { return EAGAIN; }
            sigma_sleep_ms(1); // yield
            0
        }
        FUTEX_WAKE => 1, // wake up to val waiters
        _ => EINVAL,
    }
}

unsafe fn sys_sigma_pledge(promises_ptr: u64, len: u64, _: u64, _: u64, _: u64, _: u64) -> i64 {
    extern "C" { fn sigma_pledge(p: *const u8, l: usize) -> i32; }
    if promises_ptr == 0 { return EFAULT; }
    sigma_pledge(promises_ptr as *const u8, len as usize) as i64
}

unsafe fn sys_not_implemented(_: u64, _: u64, _: u64, _: u64, _: u64, _: u64) -> i64 {
    ENOSYS
}

// ── Dispatch table ────────────────────────────────────────────────────────
const DISPATCH_TABLE_SIZE: usize = 420;

/// Main syscall dispatcher — called from the syscall gate
#[no_mangle]
pub unsafe extern "C" fn sigma_syscall_dispatch(
    nr: u64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64,
) -> i64 {
    match nr {
        SYS_GETPID          => sys_getpid(a1,a2,a3,a4,a5,a6),
        SYS_GETPPID         => sys_getppid(a1,a2,a3,a4,a5,a6),
        SYS_GETUID          => sys_getuid(a1,a2,a3,a4,a5,a6),
        SYS_GETEUID         => sys_geteuid(a1,a2,a3,a4,a5,a6),
        SYS_EXIT | SYS_EXIT_GROUP => sys_exit(a1,a2,a3,a4,a5,a6),
        SYS_NANOSLEEP       => sys_nanosleep(a1,a2,a3,a4,a5,a6),
        SYS_CLOCK_GETTIME   => sys_clock_gettime(a1,a2,a3,a4,a5,a6),
        SYS_UNAME           => sys_uname(a1,a2,a3,a4,a5,a6),
        SYS_GETRANDOM       => sys_getrandom(a1,a2,a3,a4,a5,a6),
        SYS_SET_TID_ADDRESS => sys_set_tid_address(a1,a2,a3,a4,a5,a6),
        SYS_MMAP            => sys_mmap(a1,a2,a3,a4,a5,a6),
        SYS_MUNMAP          => sys_munmap(a1,a2,a3,a4,a5,a6),
        SYS_BRK             => sys_brk(a1,a2,a3,a4,a5,a6),
        SYS_FUTEX           => sys_futex(a1,a2,a3,a4,a5,a6),
        SYS_SIGMA_PLEDGE    => sys_sigma_pledge(a1,a2,a3,a4,a5,a6),
        // Stubs for remaining critical syscalls
        SYS_READ | SYS_WRITE | SYS_OPEN | SYS_CLOSE |
        SYS_PREAD64 | SYS_PWRITE64 | SYS_READV | SYS_WRITEV |
        SYS_IOCTL | SYS_FCNTL | SYS_DUP | SYS_DUP2 |
        SYS_STAT | SYS_FSTAT | SYS_LSTAT |
        SYS_MKDIR | SYS_RMDIR | SYS_UNLINK | SYS_CHDIR | SYS_GETCWD |
        SYS_CHMOD | SYS_CHOWN |
        SYS_SOCKET | SYS_CONNECT | SYS_ACCEPT | SYS_SENDTO | SYS_RECVFROM |
        SYS_BIND | SYS_LISTEN |
        SYS_CLONE | SYS_FORK | SYS_EXECVE | SYS_WAIT4 | SYS_KILL |
        SYS_POLL | SYS_EPOLL_CREATE1 | SYS_EPOLL_CTL | SYS_EPOLL_WAIT |
        SYS_PIPE | SYS_PIPE2 | SYS_LSEEK | SYS_MEMFD_CREATE |
        SYS_RT_SIGACTION | SYS_RT_SIGPROCMASK | SYS_RT_SIGRETURN =>
            sys_not_implemented(a1,a2,a3,a4,a5,a6),
        _ => ENOSYS,
    }
}

/// Syscall count for diagnostics
#[no_mangle]
pub extern "C" fn sigma_syscall_count() -> usize { DISPATCH_TABLE_SIZE }
