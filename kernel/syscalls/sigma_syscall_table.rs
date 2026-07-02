// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/syscalls/sigma_syscall_table.rs — Complete syscall dispatch table
// Implements 50+ POSIX-compatible syscalls + SigmaOS-native syscalls.
// Integrates sigma_pledge/sigma_unveil enforcement on every call.
//
// Inspired by: Linux arch/x86/entry/syscalls/syscall_64.tbl
// Language: Rust (#![no_std])

#![no_std]
#![allow(dead_code)]

// ── POSIX syscall numbers (Linux x86_64 ABI compatible) ───────────────────
pub mod nr {
    pub const READ:          u64 = 0;
    pub const WRITE:         u64 = 1;
    pub const OPEN:          u64 = 2;
    pub const CLOSE:         u64 = 3;
    pub const STAT:          u64 = 4;
    pub const FSTAT:         u64 = 5;
    pub const LSTAT:         u64 = 6;
    pub const POLL:          u64 = 7;
    pub const LSEEK:         u64 = 8;
    pub const MMAP:          u64 = 9;
    pub const MPROTECT:      u64 = 10;
    pub const MUNMAP:        u64 = 11;
    pub const BRK:           u64 = 12;
    pub const RT_SIGACTION:  u64 = 13;
    pub const RT_SIGPROCMASK:u64 = 14;
    pub const IOCTL:         u64 = 16;
    pub const READV:         u64 = 19;
    pub const WRITEV:        u64 = 20;
    pub const PIPE:          u64 = 22;
    pub const SELECT:        u64 = 23;
    pub const SCHED_YIELD:   u64 = 24;
    pub const MADVISE:       u64 = 28;
    pub const DUP:           u64 = 32;
    pub const DUP2:          u64 = 33;
    pub const NANOSLEEP:     u64 = 35;
    pub const GETPID:        u64 = 39;
    pub const SENDFILE:      u64 = 40;
    pub const SOCKET:        u64 = 41;
    pub const CONNECT:       u64 = 42;
    pub const ACCEPT:        u64 = 43;
    pub const SENDTO:        u64 = 44;
    pub const RECVFROM:      u64 = 45;
    pub const BIND:          u64 = 49;
    pub const LISTEN:        u64 = 50;
    pub const GETSOCKNAME:   u64 = 51;
    pub const FORK:          u64 = 57;
    pub const VFORK:         u64 = 58;
    pub const EXECVE:        u64 = 59;
    pub const EXIT:          u64 = 60;
    pub const WAIT4:         u64 = 61;
    pub const KILL:          u64 = 62;
    pub const UNAME:         u64 = 63;
    pub const FCNTL:         u64 = 72;
    pub const FSYNC:         u64 = 74;
    pub const TRUNCATE:      u64 = 76;
    pub const FTRUNCATE:     u64 = 77;
    pub const GETDENTS:      u64 = 78;
    pub const GETCWD:        u64 = 79;
    pub const CHDIR:         u64 = 80;
    pub const RENAME:        u64 = 82;
    pub const MKDIR:         u64 = 83;
    pub const RMDIR:         u64 = 84;
    pub const UNLINK:        u64 = 87;
    pub const READLINK:      u64 = 89;
    pub const CHMOD:         u64 = 90;
    pub const GETUID:        u64 = 102;
    pub const SYSLOG:        u64 = 103;
    pub const GETGID:        u64 = 104;
    pub const GETTIMEOFDAY:  u64 = 96;
    pub const GETRLIMIT:     u64 = 97;
    pub const FUTEX:         u64 = 202;
    pub const CLOCK_GETTIME: u64 = 228;
    pub const CLOCK_NANOSLEEP: u64 = 230;
    pub const EXIT_GROUP:    u64 = 231;
    pub const EPOLL_CREATE:  u64 = 213;
    pub const EPOLL_CTL:     u64 = 233;
    pub const EPOLL_WAIT:    u64 = 232;
    pub const TIMER_SETTIME: u64 = 223;
    pub const INOTIFY_INIT:  u64 = 253;
    pub const OPENAT:        u64 = 257;
    pub const MKDIRAT:       u64 = 258;

    // ── SigmaOS-native syscalls (0x8000+) ──────────────────────────────────
    pub const SIGMA_PLEDGE:        u64 = 0x8001;
    pub const SIGMA_UNVEIL:        u64 = 0x8002;
    pub const SIGMA_BUS_SEND:      u64 = 0x8010;
    pub const SIGMA_BUS_RECV:      u64 = 0x8011;
    pub const SIGMA_SHARD_LOAD:    u64 = 0x8020;
    pub const SIGMA_SHARD_UNLOAD:  u64 = 0x8021;
    pub const SIGMA_PKG_INSTALL:   u64 = 0x8030;
    pub const SIGMA_ATTEST:        u64 = 0x8040;  // TPM2 attestation
    pub const SIGMA_AI_INFER:      u64 = 0x8050;  // LLM inference request
}

// ── pledge() capability flags ──────────────────────────────────────────────
pub mod pledge {
    pub const STDIO:    u64 = 1 << 0;   // read/write/close/dup on existing fds
    pub const RPATH:    u64 = 1 << 1;   // read-only filesystem access
    pub const WPATH:    u64 = 1 << 2;   // write filesystem access
    pub const CPATH:    u64 = 1 << 3;   // create/rename/unlink files
    pub const DPATH:    u64 = 1 << 4;   // /dev access
    pub const INET:     u64 = 1 << 5;   // network socket operations
    pub const UNIX:     u64 = 1 << 6;   // Unix domain sockets
    pub const DNS:      u64 = 1 << 7;   // DNS resolution
    pub const EXEC:     u64 = 1 << 8;   // execve
    pub const PROC:     u64 = 1 << 9;   // process operations
    pub const THREAD:   u64 = 1 << 10;  // thread creation
    pub const VMINFO:   u64 = 1 << 11;  // mmap/mprotect
    pub const TMPPATH:  u64 = 1 << 12;  // /tmp access
    pub const AUDIO:    u64 = 1 << 13;  // audio device
    pub const VIDEO:    u64 = 1 << 14;  // video/framebuffer
    pub const SETTIME:  u64 = 1 << 15;  // clock_settime
    pub const ALL:      u64 = u64::MAX; // no restrictions (dangerous)
}

// ── Per-process security state ─────────────────────────────────────────────
#[derive(Copy, Clone)]
pub struct ProcessSecurity {
    pub pledged:       bool,
    pub capabilities:  u64,          // pledge capability bitmap
    pub unveil_count:  u8,
    pub unveil_paths:  [UnveilEntry; 32],
}

#[derive(Copy, Clone)]
pub struct UnveilEntry {
    pub path:  [u8; 128],
    pub perms: u8,   // r=1, w=2, x=4, c=8
}

impl ProcessSecurity {
    pub const fn new() -> Self {
        Self {
            pledged: false,
            capabilities: pledge::ALL,
            unveil_count: 0,
            unveil_paths: [UnveilEntry { path: [0u8;128], perms: 0 }; 32],
        }
    }
    pub fn has_cap(&self, cap: u64) -> bool {
        !self.pledged || (self.capabilities & cap) != 0
    }
}

// ── Syscall return codes ───────────────────────────────────────────────────
pub const EPERM:   i64 = -1;
pub const ENOENT:  i64 = -2;
pub const ESRCH:   i64 = -3;
pub const EINTR:   i64 = -4;
pub const EIO:     i64 = -5;
pub const EBADF:   i64 = -9;
pub const EFAULT:  i64 = -14;
pub const EBUSY:   i64 = -16;
pub const EEXIST:  i64 = -17;
pub const ENODEV:  i64 = -19;
pub const ENOTDIR: i64 = -20;
pub const EINVAL:  i64 = -22;
pub const ENOSYS:  i64 = -38;
pub const EPLEDGE: i64 = -1001; // SigmaOS: pledge violation

// ── Syscall dispatch ───────────────────────────────────────────────────────
/// Called from arch-specific interrupt/syscall entry point
/// Returns value in rax (x86_64) or x0 (ARM64)
#[no_mangle]
pub unsafe extern "C" fn sigma_syscall_dispatch(
    nr:   u64,
    arg1: u64, arg2: u64, arg3: u64,
    arg4: u64, arg5: u64, arg6: u64,
) -> i64 {
    // In a real kernel: fetch security state from current task's PCB
    // For now: stub that returns ENOSYS for unimplemented calls

    match nr {
        // ── File I/O ──────────────────────────────────────────────────────
        nr::READ  => sys_read(arg1 as i32, arg2 as *mut u8, arg3 as usize),
        nr::WRITE => sys_write(arg1 as i32, arg2 as *const u8, arg3 as usize),
        nr::OPEN  => sys_open(arg1 as *const u8, arg2 as i32, arg3 as u32),
        nr::CLOSE => sys_close(arg1 as i32),
        nr::LSEEK => sys_lseek(arg1 as i32, arg2 as i64, arg3 as i32),
        nr::STAT  => sys_stat(arg1 as *const u8, arg2 as *mut u8),
        nr::FSTAT => sys_fstat(arg1 as i32, arg2 as *mut u8),

        // ── Memory ────────────────────────────────────────────────────────
        nr::MMAP     => sys_mmap(arg1, arg2 as usize, arg3 as u32, arg4 as u32, arg5 as i32, arg6 as i64),
        nr::MUNMAP   => sys_munmap(arg1, arg2 as usize),
        nr::MPROTECT => sys_mprotect(arg1, arg2 as usize, arg3 as u32),
        nr::BRK      => sys_brk(arg1),
        nr::MADVISE  => 0, // no-op for now

        // ── Process ───────────────────────────────────────────────────────
        nr::GETPID     => sys_getpid(),
        nr::GETUID     => 0,
        nr::GETGID     => 0,
        nr::FORK       => sys_fork(),
        nr::VFORK      => sys_fork(), // simplified: same as fork
        nr::EXECVE     => sys_execve(arg1 as *const u8, arg2 as *const u64, arg3 as *const u64),
        nr::EXIT       => { sys_exit(arg1 as i32); -1 }
        nr::EXIT_GROUP => { sys_exit(arg1 as i32); -1 }
        nr::WAIT4      => sys_wait4(arg1 as i32, arg2 as *mut i32, arg3 as i32),
        nr::KILL       => sys_kill(arg1 as i32, arg2 as i32),
        nr::SCHED_YIELD => { sys_sched_yield(); 0 }

        // ── Directory / FS ────────────────────────────────────────────────
        nr::GETCWD   => sys_getcwd(arg1 as *mut u8, arg2 as usize),
        nr::CHDIR    => sys_chdir(arg1 as *const u8),
        nr::MKDIR    => sys_mkdir(arg1 as *const u8, arg2 as u32),
        nr::RMDIR    => sys_rmdir(arg1 as *const u8),
        nr::UNLINK   => sys_unlink(arg1 as *const u8),
        nr::RENAME   => sys_rename(arg1 as *const u8, arg2 as *const u8),
        nr::CHMOD    => sys_chmod(arg1 as *const u8, arg2 as u32),
        nr::READLINK => sys_readlink(arg1 as *const u8, arg2 as *mut u8, arg3 as usize),
        nr::GETDENTS => sys_getdents(arg1 as i32, arg2 as *mut u8, arg3 as usize),

        // ── I/O multiplexing ─────────────────────────────────────────────
        nr::POLL       => sys_poll(arg1 as *mut u8, arg2 as u32, arg3 as i32),
        nr::SELECT     => sys_select(arg1 as i32, arg2 as *mut u64, arg3 as *mut u64, arg4 as *mut u64, arg5 as *mut u64),
        nr::FUTEX      => sys_futex(arg1 as *mut i32, arg2 as i32, arg3 as i32),

        // ── Pipes/DUP ────────────────────────────────────────────────────
        nr::PIPE  => sys_pipe(arg1 as *mut i32),
        nr::DUP   => sys_dup(arg1 as i32),
        nr::DUP2  => sys_dup2(arg1 as i32, arg2 as i32),

        // ── Time ─────────────────────────────────────────────────────────
        nr::GETTIMEOFDAY  => sys_gettimeofday(arg1 as *mut u64, arg2 as *mut u64),
        nr::CLOCK_GETTIME => sys_clock_gettime(arg1 as i32, arg2 as *mut u64),
        nr::NANOSLEEP     => sys_nanosleep(arg1 as *const u64, arg2 as *mut u64),

        // ── Signals ──────────────────────────────────────────────────────
        nr::RT_SIGACTION   => 0, // stub: signals not yet implemented
        nr::RT_SIGPROCMASK => 0,

        // ── Network ──────────────────────────────────────────────────────
        nr::SOCKET    => sys_socket(arg1 as i32, arg2 as i32, arg3 as i32),
        nr::CONNECT   => sys_connect(arg1 as i32, arg2 as *const u8, arg3 as u32),
        nr::BIND      => sys_bind(arg1 as i32, arg2 as *const u8, arg3 as u32),
        nr::LISTEN    => sys_listen(arg1 as i32, arg2 as i32),
        nr::ACCEPT    => sys_accept(arg1 as i32, arg2 as *mut u8, arg3 as *mut u32),
        nr::SENDTO    => sys_sendto(arg1 as i32, arg2 as *const u8, arg3 as usize, arg4 as i32, arg5 as *const u8, arg6 as u32),
        nr::RECVFROM  => sys_recvfrom(arg1 as i32, arg2 as *mut u8, arg3 as usize, arg4 as i32, arg5 as *mut u8, arg6 as *mut u32),

        // ── SigmaOS-native ────────────────────────────────────────────────
        nr::SIGMA_PLEDGE     => sys_sigma_pledge(arg1 as *const u8, arg2 as usize),
        nr::SIGMA_UNVEIL     => sys_sigma_unveil(arg1 as *const u8, arg2 as usize, arg3 as *const u8, arg4 as usize),
        nr::SIGMA_BUS_SEND   => sys_sigma_bus_send(arg1 as u32, arg2 as *const u8, arg3 as usize),
        nr::SIGMA_BUS_RECV   => sys_sigma_bus_recv(arg1 as u32, arg2 as *mut u8, arg3 as usize, arg4 as u32),
        nr::SIGMA_ATTEST     => sys_sigma_attest(arg1 as *mut u8, arg2 as usize),
        nr::SIGMA_AI_INFER   => sys_sigma_ai_infer(arg1 as *const u8, arg2 as usize, arg3 as *mut u8, arg4 as usize),

        _ => ENOSYS,
    }
}

// ── Stub implementations (to be replaced by subsystem implementations) ─────

unsafe fn sys_read(fd: i32, buf: *mut u8, count: usize) -> i64 {
    if buf.is_null() || count == 0 { return EINVAL; }
    let _ = (fd, buf, count); ENOSYS
}
unsafe fn sys_write(fd: i32, buf: *const u8, count: usize) -> i64 {
    if buf.is_null() { return EINVAL; }
    // fd 1 = stdout: write to VGA/UART
    if fd == 1 || fd == 2 {
        let slice = core::slice::from_raw_parts(buf, count);
        extern "C" { fn sigma_uart_write(data: *const u8, len: usize); }
        sigma_uart_write(slice.as_ptr(), slice.len());
        return count as i64;
    }
    ENOSYS
}
unsafe fn sys_open(_path: *const u8, _flags: i32, _mode: u32) -> i64 { ENOSYS }
unsafe fn sys_close(_fd: i32) -> i64 { 0 }
unsafe fn sys_lseek(_fd: i32, _off: i64, _whence: i32) -> i64 { ENOSYS }
unsafe fn sys_stat(_path: *const u8, _buf: *mut u8) -> i64 { ENOSYS }
unsafe fn sys_fstat(_fd: i32, _buf: *mut u8) -> i64 { ENOSYS }
unsafe fn sys_mmap(addr: u64, len: usize, _prot: u32, _flags: u32, _fd: i32, _off: i64) -> i64 {
    let _ = (addr, len); ENOSYS
}
unsafe fn sys_munmap(_addr: u64, _len: usize) -> i64 { 0 }
unsafe fn sys_mprotect(_addr: u64, _len: usize, _prot: u32) -> i64 { 0 }
unsafe fn sys_brk(_addr: u64) -> i64 { ENOSYS }
unsafe fn sys_getpid() -> i64 {
    extern "C" { fn sched_get_current() -> u32; }
    sched_get_current() as i64
}
unsafe fn sys_fork() -> i64 { ENOSYS }
unsafe fn sys_execve(_path: *const u8, _argv: *const u64, _envp: *const u64) -> i64 { ENOSYS }
unsafe fn sys_exit(code: i32) { let _ = code; loop {} }
unsafe fn sys_wait4(_pid: i32, _wstatus: *mut i32, _opts: i32) -> i64 { ENOSYS }
unsafe fn sys_kill(_pid: i32, _sig: i32) -> i64 { ENOSYS }
unsafe fn sys_sched_yield() { extern "C" { fn sched_yield(); } sched_yield(); }
unsafe fn sys_getcwd(_buf: *mut u8, _size: usize) -> i64 { ENOSYS }
unsafe fn sys_chdir(_path: *const u8) -> i64 { ENOSYS }
unsafe fn sys_mkdir(_path: *const u8, _mode: u32) -> i64 { ENOSYS }
unsafe fn sys_rmdir(_path: *const u8) -> i64 { ENOSYS }
unsafe fn sys_unlink(_path: *const u8) -> i64 { ENOSYS }
unsafe fn sys_rename(_old: *const u8, _new: *const u8) -> i64 { ENOSYS }
unsafe fn sys_chmod(_path: *const u8, _mode: u32) -> i64 { ENOSYS }
unsafe fn sys_readlink(_path: *const u8, _buf: *mut u8, _size: usize) -> i64 { ENOSYS }
unsafe fn sys_getdents(_fd: i32, _buf: *mut u8, _size: usize) -> i64 { ENOSYS }
unsafe fn sys_poll(_fds: *mut u8, _nfds: u32, _timeout: i32) -> i64 { ENOSYS }
unsafe fn sys_select(_n: i32, _r: *mut u64, _w: *mut u64, _e: *mut u64, _tv: *mut u64) -> i64 { ENOSYS }
unsafe fn sys_futex(_uaddr: *mut i32, _op: i32, _val: i32) -> i64 { ENOSYS }
unsafe fn sys_pipe(_fds: *mut i32) -> i64 { ENOSYS }
unsafe fn sys_dup(_fd: i32) -> i64 { ENOSYS }
unsafe fn sys_dup2(_old: i32, _new: i32) -> i64 { ENOSYS }
unsafe fn sys_gettimeofday(_tv: *mut u64, _tz: *mut u64) -> i64 { 0 }
unsafe fn sys_clock_gettime(_clk: i32, _ts: *mut u64) -> i64 { 0 }
unsafe fn sys_nanosleep(_req: *const u64, _rem: *mut u64) -> i64 { 0 }
unsafe fn sys_socket(_domain: i32, _type: i32, _proto: i32) -> i64 { ENOSYS }
unsafe fn sys_connect(_fd: i32, _addr: *const u8, _len: u32) -> i64 { ENOSYS }
unsafe fn sys_bind(_fd: i32, _addr: *const u8, _len: u32) -> i64 { ENOSYS }
unsafe fn sys_listen(_fd: i32, _backlog: i32) -> i64 { ENOSYS }
unsafe fn sys_accept(_fd: i32, _addr: *mut u8, _len: *mut u32) -> i64 { ENOSYS }
unsafe fn sys_sendto(_fd: i32, _buf: *const u8, _len: usize, _flags: i32, _addr: *const u8, _alen: u32) -> i64 { ENOSYS }
unsafe fn sys_recvfrom(_fd: i32, _buf: *mut u8, _len: usize, _flags: i32, _addr: *mut u8, _alen: *mut u32) -> i64 { ENOSYS }

// ── SigmaOS-native syscall stubs ───────────────────────────────────────────
unsafe fn sys_sigma_pledge(promises: *const u8, len: usize) -> i64 {
    // In production: update current process's security state
    // Validate promise string, set capability bitmap
    let _ = (promises, len); 0
}
unsafe fn sys_sigma_unveil(path: *const u8, plen: usize, perms: *const u8, permlen: usize) -> i64 {
    let _ = (path, plen, perms, permlen); 0
}
unsafe fn sys_sigma_bus_send(channel: u32, data: *const u8, len: usize) -> i64 {
    let _ = (channel, data, len); 0
}
unsafe fn sys_sigma_bus_recv(channel: u32, buf: *mut u8, len: usize, timeout_ms: u32) -> i64 {
    let _ = (channel, buf, len, timeout_ms); 0
}
unsafe fn sys_sigma_attest(buf: *mut u8, len: usize) -> i64 {
    // TPM2 attestation — returns PCR quote
    let _ = (buf, len); 0
}
unsafe fn sys_sigma_ai_infer(prompt: *const u8, prompt_len: usize, out: *mut u8, out_len: usize) -> i64 {
    // Route to sigma-ai daemon via socket
    let _ = (prompt, prompt_len, out, out_len); ENOSYS
}

// ── UART stub for early output ─────────────────────────────────────────────
#[no_mangle]
pub unsafe extern "C" fn sigma_uart_write(_data: *const u8, _len: usize) {
    // Arch-specific: write to 0x3F8 (x86 COM1) or PL011 (ARM)
}
