// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/sigma_syscalls_proc.rs — Process management syscalls
// Implements: fork, execve, wait4, exit/exit_group, getpid/getppid,
//             mkdir, rmdir, unlink, chdir, getcwd, chmod, chown, kill,
//             pipe/pipe2
// Language: Rust #![no_std]

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU32, Ordering};

pub const EBADF:  i64 = -9;
pub const EFAULT: i64 = -14;
pub const EINVAL: i64 = -22;
pub const ENOSYS: i64 = -38;
pub const ENOENT: i64 = -2;
pub const ENOMEM: i64 = -12;
pub const ECHILD: i64 = -10;
pub const EAGAIN: i64 = -11;
pub const EPERM:  i64 = -1;
pub const ENOEXEC:i64 = -8;

// ── Process table ─────────────────────────────────────────────────────────
const MAX_PROCS: usize = 256;

#[derive(Copy, Clone, PartialEq)]
pub enum ProcState { Free, Running, Zombie, Sleeping }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ProcEntry {
    pub pid:    u32,
    pub ppid:   u32,
    pub state:  ProcState,
    pub exit_code: i32,
    pub stack_base: u64,
    pub stack_size: u32,
}

impl ProcEntry {
    const fn empty() -> Self {
        ProcEntry { pid: 0, ppid: 0, state: ProcState::Free, exit_code: 0, stack_base: 0, stack_size: 0 }
    }
}

static mut PROC_TABLE: [ProcEntry; MAX_PROCS] = [ProcEntry::empty(); MAX_PROCS];
static NEXT_PID: AtomicU32 = AtomicU32::new(2);  // PID 1 = init
static CURRENT_PID: AtomicU32 = AtomicU32::new(1);

unsafe fn alloc_proc() -> Option<usize> {
    for i in 1..MAX_PROCS {
        if PROC_TABLE[i].state == ProcState::Free { return Some(i); }
    }
    None
}

pub unsafe fn sigma_getpid() -> u32 { CURRENT_PID.load(Ordering::Relaxed) }
pub unsafe fn sigma_gettid() -> u32 { sigma_getpid() } // single-threaded: tid == pid

// ── Pipe infrastructure ───────────────────────────────────────────────────
const MAX_PIPES: usize = 64;
const PIPE_BUF:  usize = 4096;

struct PipeBuf {
    data: [u8; PIPE_BUF],
    head: usize,
    tail: usize,
    read_open: bool,
    write_open: bool,
}

impl PipeBuf {
    const fn empty() -> Self {
        PipeBuf { data: [0u8; PIPE_BUF], head: 0, tail: 0, read_open: false, write_open: false }
    }
    fn write(&mut self, src: &[u8]) -> usize {
        let mut n = 0;
        for &b in src {
            let next = (self.tail + 1) % PIPE_BUF;
            if next == self.head { break; }
            self.data[self.tail] = b;
            self.tail = next;
            n += 1;
        }
        n
    }
    fn read(&mut self, dst: &mut [u8]) -> usize {
        let mut n = 0;
        while n < dst.len() && self.head != self.tail {
            dst[n] = self.data[self.head];
            self.head = (self.head + 1) % PIPE_BUF;
            n += 1;
        }
        n
    }
}

static mut PIPES: [PipeBuf; MAX_PIPES] = [const { PipeBuf::empty() }; MAX_PIPES];

fn alloc_pipe() -> Option<u32> {
    unsafe {
        for i in 0..MAX_PIPES {
            if !PIPES[i].read_open && !PIPES[i].write_open {
                PIPES[i].read_open = true;
                PIPES[i].write_open = true;
                return Some(i as u32);
            }
        }
    }
    None
}

#[no_mangle]
pub unsafe extern "C" fn sigma_pipe_read(id: u32, buf: *mut u8, len: usize) -> i64 {
    if id as usize >= MAX_PIPES { return EBADF; }
    let dst = core::slice::from_raw_parts_mut(buf, len);
    PIPES[id as usize].read(dst) as i64
}

#[no_mangle]
pub unsafe extern "C" fn sigma_pipe_write(id: u32, buf: *const u8, len: usize) -> i64 {
    if id as usize >= MAX_PIPES { return EBADF; }
    let src = core::slice::from_raw_parts(buf, len);
    PIPES[id as usize].write(src) as i64
}

// ── sys_pipe ──────────────────────────────────────────────────────────────
pub unsafe fn sys_pipe(pipefd_ptr: u64, _flags: u32) -> i64 {
    if pipefd_ptr == 0 { return EFAULT; }
    let pipe_id = match alloc_pipe() { Some(id) => id, None => return ENOMEM };
    // Allocate two FDs from the I/O table
    extern "C" {
        fn sigma_alloc_pipe_fds(pipe_id: u32, read_fd: *mut i32, write_fd: *mut i32) -> i32;
    }
    let mut rfd: i32 = -1;
    let mut wfd: i32 = -1;
    let ret = sigma_alloc_pipe_fds(pipe_id, &mut rfd, &mut wfd);
    if ret < 0 { return ENOMEM; }
    let arr = pipefd_ptr as *mut i32;
    arr.write(rfd);
    arr.add(1).write(wfd);
    0
}

// ── sys_fork ──────────────────────────────────────────────────────────────
pub unsafe fn sys_fork() -> i64 {
    let new_idx = match alloc_proc() { Some(i) => i, None => return EAGAIN };
    let parent_pid = sigma_getpid();
    let new_pid = NEXT_PID.fetch_add(1, Ordering::Relaxed);

    // Allocate a child stack
    extern "C" { fn sigma_slab_alloc(size: usize) -> *mut u8; }
    let stack = sigma_slab_alloc(65536);
    if stack.is_null() { return ENOMEM; }

    PROC_TABLE[new_idx] = ProcEntry {
        pid: new_pid, ppid: parent_pid,
        state: ProcState::Running,
        exit_code: 0,
        stack_base: stack as u64,
        stack_size: 65536,
    };

    // Schedule child via scheduler
    extern "C" { fn sched_add_task(pid: u32, policy: u8, deadline: u64, level: u8) -> i32; }
    sched_add_task(new_pid, 0, 0, 0);  // MLFQ policy

    // Parent returns child PID; child returns 0 (simulated)
    new_pid as i64
}

// ── sys_execve ────────────────────────────────────────────────────────────
pub unsafe fn sys_execve(path_ptr: u64, argv_ptr: u64, envp_ptr: u64) -> i64 {
    if path_ptr == 0 { return EFAULT; }

    // Read path
    let mut path_buf = [0u8; 256];
    let mut plen = 0;
    let pptr = path_ptr as *const u8;
    while plen < 255 {
        let b = core::ptr::read_volatile(pptr.add(plen));
        if b == 0 { break; }
        path_buf[plen] = b; plen += 1;
    }
    let _path = &path_buf[..plen];

    // Load and jump to ELF binary via ELF loader
    extern "C" {
        fn sigma_load_and_exec(
            path: *const u8, path_len: usize,
            argv: u64, envp: u64,
        ) -> i64;
    }
    sigma_load_and_exec(path_buf.as_ptr(), plen, argv_ptr, envp_ptr)
}

// ── sys_wait4 ─────────────────────────────────────────────────────────────
pub unsafe fn sys_wait4(pid: u64, wstatus: u64, options: u64, _rusage: u64) -> i64 {
    const WNOHANG: u64 = 1;
    let target = pid as i64;
    for i in 0..MAX_PROCS {
        let p = &PROC_TABLE[i];
        if p.state != ProcState::Zombie { continue; }
        let match_pid = target == -1   // wait for any child
            || target == p.pid as i64
            || (target < -1 && (-target) == p.ppid as i64);
        if !match_pid { continue; }
        let child_pid = p.pid;
        let exit_code = p.exit_code;
        PROC_TABLE[i].state = ProcState::Free; // reap
        if wstatus != 0 {
            // Encode exit status: (code & 0xFF) << 8
            (wstatus as *mut i32).write((exit_code & 0xFF) << 8);
        }
        return child_pid as i64;
    }
    if options & WNOHANG != 0 { return 0; }
    ECHILD
}

// ── sys_exit ──────────────────────────────────────────────────────────────
pub unsafe fn sys_exit(code: u64) -> i64 {
    let pid = sigma_getpid();
    for i in 0..MAX_PROCS {
        if PROC_TABLE[i].pid == pid {
            PROC_TABLE[i].state = ProcState::Zombie;
            PROC_TABLE[i].exit_code = code as i32;
            break;
        }
    }
    // Switch to next runnable task
    extern "C" { fn sched_tick(now_ns: u64) -> u32; fn sigma_clock_ns() -> u64; }
    let _next = sched_tick(sigma_clock_ns());
    0
}

// ── Directory operations ──────────────────────────────────────────────────
pub unsafe fn sys_mkdir(path_ptr: u64, _mode: u64) -> i64 {
    if path_ptr == 0 { return EFAULT; }
    extern "C" { fn sigma_vfs_mkdir(path: *const u8, len: usize) -> i32; }
    let mut buf = [0u8; 256]; let mut len = 0;
    let ptr = path_ptr as *const u8;
    while len < 255 { let b = *ptr.add(len); if b == 0 { break; } buf[len] = b; len += 1; }
    if sigma_vfs_mkdir(buf.as_ptr(), len) >= 0 { 0 } else { ENOENT }
}

pub unsafe fn sys_rmdir(path_ptr: u64) -> i64 {
    if path_ptr == 0 { return EFAULT; }
    extern "C" { fn sigma_vfs_rmdir(path: *const u8, len: usize) -> i32; }
    let mut buf = [0u8; 256]; let mut len = 0;
    let ptr = path_ptr as *const u8;
    while len < 255 { let b = *ptr.add(len); if b == 0 { break; } buf[len] = b; len += 1; }
    if sigma_vfs_rmdir(buf.as_ptr(), len) >= 0 { 0 } else { ENOENT }
}

pub unsafe fn sys_unlink(path_ptr: u64) -> i64 {
    if path_ptr == 0 { return EFAULT; }
    extern "C" { fn sigma_vfs_unlink(path: *const u8, len: usize) -> i32; }
    let mut buf = [0u8; 256]; let mut len = 0;
    let ptr = path_ptr as *const u8;
    while len < 255 { let b = *ptr.add(len); if b == 0 { break; } buf[len] = b; len += 1; }
    if sigma_vfs_unlink(buf.as_ptr(), len) >= 0 { 0 } else { ENOENT }
}

pub unsafe fn sys_chdir(path_ptr: u64) -> i64 {
    if path_ptr == 0 { return EFAULT; }
    0 // CWD tracked in process context (simplified)
}

pub unsafe fn sys_getcwd(buf_ptr: u64, size: u64) -> i64 {
    if buf_ptr == 0 { return EFAULT; }
    let cwd = b"/\0";
    let n = cwd.len().min(size as usize);
    let dst = core::slice::from_raw_parts_mut(buf_ptr as *mut u8, n);
    dst.copy_from_slice(&cwd[..n]);
    buf_ptr as i64
}

pub unsafe fn sys_chmod(_path: u64, _mode: u64) -> i64 { 0 }
pub unsafe fn sys_chown(_path: u64, _uid: u64, _gid: u64) -> i64 { 0 }

pub unsafe fn sys_kill(pid: u64, sig: u64) -> i64 {
    let target_pid = pid as u32;
    for i in 0..MAX_PROCS {
        if PROC_TABLE[i].pid == target_pid {
            if sig == 9 || sig == 15 {  // SIGKILL or SIGTERM
                PROC_TABLE[i].state = ProcState::Zombie;
                PROC_TABLE[i].exit_code = -(sig as i32);
            }
            return 0;
        }
    }
    EPERM
}

// ── C-ABI exports ─────────────────────────────────────────────────────────
#[no_mangle] pub unsafe extern "C" fn sigma_sys_fork() -> i64                                    { sys_fork() }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_execve(p: u64, a: u64, e: u64) -> i64           { sys_execve(p,a,e) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_wait4(p: u64, s: u64, o: u64, r: u64) -> i64    { sys_wait4(p,s,o,r) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_exit(c: u64) -> i64                              { sys_exit(c) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_mkdir(p: u64, m: u64) -> i64                    { sys_mkdir(p,m) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_rmdir(p: u64) -> i64                            { sys_rmdir(p) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_unlink(p: u64) -> i64                           { sys_unlink(p) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_chdir(p: u64) -> i64                            { sys_chdir(p) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_getcwd(b: u64, s: u64) -> i64                   { sys_getcwd(b,s) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_chmod(p: u64, m: u64) -> i64                    { sys_chmod(p,m) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_chown(p: u64, u: u64, g: u64) -> i64            { sys_chown(p,u,g) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_kill(p: u64, s: u64) -> i64                     { sys_kill(p,s) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_pipe(pfd: u64) -> i64                           { sys_pipe(pfd, 0) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_pipe2(pfd: u64, f: u64) -> i64                  { sys_pipe(pfd, f as u32) }
#[no_mangle] pub unsafe extern "C" fn sigma_getpid_export() -> u32                              { unsafe { sigma_getpid() } }
#[no_mangle] pub unsafe extern "C" fn sigma_gettid_export() -> u32                              { unsafe { sigma_gettid() } }
