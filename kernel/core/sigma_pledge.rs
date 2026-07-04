// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/sigma_pledge.rs — sigma_pledge + sigma_unveil
// OpenBSD-inspired capability restriction for SigmaOS processes.
//
// sigma_pledge(promises, paths):
//   Restricts a process to a declared set of syscall groups.
//   Once set, pledges can only be narrowed, never widened.
//
// sigma_unveil(path, permissions):
//   Restricts filesystem access to declared paths.
//   Any path not unveil'd returns ENOENT to the process.
//
// Language: Rust #![no_std]

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, Ordering};

// ── Promise bits ───────────────────────────────────────────────────────────
pub const PLEDGE_STDIO:    u64 = 1 <<  0; // read, write, poll, clock
pub const PLEDGE_RPATH:    u64 = 1 <<  1; // open/read files and dirs
pub const PLEDGE_WPATH:    u64 = 1 <<  2; // create/write files
pub const PLEDGE_CPATH:    u64 = 1 <<  3; // create dirs, symlinks
pub const PLEDGE_DPATH:    u64 = 1 <<  4; // /dev/ nodes
pub const PLEDGE_TMPPATH:  u64 = 1 <<  5; // /tmp access
pub const PLEDGE_INET:     u64 = 1 <<  6; // IPv4/IPv6 sockets
pub const PLEDGE_UNIX:     u64 = 1 <<  7; // Unix domain sockets
pub const PLEDGE_DNS:      u64 = 1 <<  8; // DNS resolution
pub const PLEDGE_GETPW:    u64 = 1 <<  9; // getpwuid, getpwnam
pub const PLEDGE_PROC:     u64 = 1 << 10; // fork, exec, wait
pub const PLEDGE_EXEC:     u64 = 1 << 11; // execve only
pub const PLEDGE_ID:       u64 = 1 << 12; // setuid, setgid
pub const PLEDGE_SETTIME:  u64 = 1 << 13; // settimeofday
pub const PLEDGE_PROT_EXEC:u64 = 1 << 14; // mmap PROT_EXEC
pub const PLEDGE_TTY:      u64 = 1 << 15; // terminal ioctls
pub const PLEDGE_AUDIO:    u64 = 1 << 16; // audio device
pub const PLEDGE_VIDEO:    u64 = 1 << 17; // video/GPU device
pub const PLEDGE_VMINFO:   u64 = 1 << 18; // /proc/meminfo reads
pub const PLEDGE_RECVFD:   u64 = 1 << 19; // receive file descriptors
pub const PLEDGE_SENDFD:   u64 = 1 << 20; // send file descriptors
pub const PLEDGE_FLOCK:    u64 = 1 << 21; // file locking
pub const PLEDGE_CHOWN:    u64 = 1 << 22; // chown, lchown
pub const PLEDGE_FATTR:    u64 = 1 << 23; // chmod, utimes
pub const PLEDGE_MCAST:    u64 = 1 << 24; // multicast sockets
pub const PLEDGE_BPF:      u64 = 1 << 25; // eBPF programs
pub const PLEDGE_UNVEIL:   u64 = 1 << 26; // unveil() calls allowed
pub const PLEDGE_ERROR:    u64 = 1 << 27; // return EPERM vs SIGKILL on violation
pub const PLEDGE_ALL:      u64 = u64::MAX;

// ── Parse promise string into bitmask ─────────────────────────────────────
fn parse_promise(token: &[u8]) -> u64 {
    match token {
        b"stdio"     => PLEDGE_STDIO,
        b"rpath"     => PLEDGE_RPATH,
        b"wpath"     => PLEDGE_WPATH,
        b"cpath"     => PLEDGE_CPATH,
        b"dpath"     => PLEDGE_DPATH,
        b"tmppath"   => PLEDGE_TMPPATH,
        b"inet"      => PLEDGE_INET,
        b"unix"      => PLEDGE_UNIX,
        b"dns"       => PLEDGE_DNS,
        b"getpw"     => PLEDGE_GETPW,
        b"proc"      => PLEDGE_PROC,
        b"exec"      => PLEDGE_EXEC,
        b"id"        => PLEDGE_ID,
        b"settime"   => PLEDGE_SETTIME,
        b"prot_exec" => PLEDGE_PROT_EXEC,
        b"tty"       => PLEDGE_TTY,
        b"audio"     => PLEDGE_AUDIO,
        b"video"     => PLEDGE_VIDEO,
        b"vminfo"    => PLEDGE_VMINFO,
        b"recvfd"    => PLEDGE_RECVFD,
        b"sendfd"    => PLEDGE_SENDFD,
        b"flock"     => PLEDGE_FLOCK,
        b"chown"     => PLEDGE_CHOWN,
        b"fattr"     => PLEDGE_FATTR,
        b"mcast"     => PLEDGE_MCAST,
        b"bpf"       => PLEDGE_BPF,
        b"error"     => PLEDGE_ERROR,
        _            => 0,
    }
}

fn parse_promises(s: &[u8]) -> u64 {
    let mut mask = 0u64;
    for token in s.split(|&b| b == b' ') {
        if !token.is_empty() {
            mask |= parse_promise(token);
        }
    }
    mask
}

// ── Per-process pledge state ───────────────────────────────────────────────
const MAX_PROCS: usize = 256;
const MAX_UNVEIL: usize = 32;

#[derive(Copy, Clone)]
pub struct UnveilEntry {
    path:     [u8; 128],
    path_len: usize,
    perms:    u8,  // r=1, w=2, x=4, c=8
    valid:    bool,
}

impl UnveilEntry {
    const fn empty() -> Self {
        UnveilEntry { path: [0u8; 128], path_len: 0, perms: 0, valid: false }
    }
}

#[derive(Copy, Clone)]
pub struct PledgeState {
    pub promises:      u64,    // active promise bitmask
    pub pledged:       bool,   // has pledge() been called?
    pub unveil_locked: bool,   // unveil() finalized
    pub unveils:       [UnveilEntry; MAX_UNVEIL],
    pub unveil_count:  usize,
}

impl PledgeState {
    pub const fn new() -> Self {
        PledgeState {
            promises:      PLEDGE_ALL,
            pledged:       false,
            unveil_locked: false,
            unveils:       [const { UnveilEntry::empty() }; MAX_UNVEIL],
            unveil_count:  0,
        }
    }
}

static mut PLEDGE_TABLE: [PledgeState; MAX_PROCS] =
    [const { PledgeState::new() }; MAX_PROCS];

fn get_proc_state(pid: u32) -> &'static mut PledgeState {
    let idx = (pid as usize).min(MAX_PROCS - 1);
    unsafe { &mut PLEDGE_TABLE[idx] }
}

// ── sigma_pledge implementation ────────────────────────────────────────────

/// Called via SYS_SIGMA_PLEDGE (400) — restricts syscall groups.
/// promises_ptr: pointer to space-separated promise string (or NULL = unlock all).
/// Returns 0 on success, -EPERM if trying to widen existing pledge.
#[no_mangle]
pub unsafe extern "C" fn sigma_pledge(promises_ptr: *const u8, len: usize) -> i32 {
    extern "C" { fn sigma_getpid() -> u32; }
    let pid   = sigma_getpid();
    let state = get_proc_state(pid);

    let new_mask = if promises_ptr.is_null() || len == 0 {
        PLEDGE_ALL
    } else {
        let slice = core::slice::from_raw_parts(promises_ptr, len.min(512));
        parse_promises(slice)
    };

    if state.pledged {
        // Can only narrow: new mask must be a subset of current
        if new_mask & !state.promises != 0 {
            return -1; // EPERM — attempted to widen
        }
    }

    state.promises = new_mask;
    state.pledged  = true;
    0
}

/// sigma_unveil(path, permissions) — restricts filesystem access.
/// permissions: string of "r", "w", "x", "c" chars.
/// Call sigma_unveil(NULL, NULL) to finalize (lock further unveils).
#[no_mangle]
pub unsafe extern "C" fn sigma_unveil(
    path_ptr: *const u8, path_len: usize,
    perms_ptr: *const u8, perms_len: usize,
) -> i32 {
    extern "C" { fn sigma_getpid() -> u32; }
    let pid   = sigma_getpid();
    let state = get_proc_state(pid);

    // NULL path + NULL perms = lock unveil table
    if path_ptr.is_null() {
        state.unveil_locked = true;
        return 0;
    }

    if state.unveil_locked { return -1; } // EPERM

    if state.unveil_count >= MAX_UNVEIL { return -12; } // ENOMEM

    let path  = core::slice::from_raw_parts(path_ptr,  path_len.min(127));
    let perms = if perms_ptr.is_null() { &[] as &[u8] }
                else { core::slice::from_raw_parts(perms_ptr, perms_len.min(8)) };

    let mut entry = UnveilEntry::empty();
    entry.path[..path.len()].copy_from_slice(path);
    entry.path_len = path.len();
    for &b in perms {
        match b {
            b'r' => entry.perms |= 1,
            b'w' => entry.perms |= 2,
            b'x' => entry.perms |= 4,
            b'c' => entry.perms |= 8,
            _ => {}
        }
    }
    entry.valid = true;

    state.unveils[state.unveil_count] = entry;
    state.unveil_count += 1;
    0
}

// ── Enforcement API (called by VFS and syscall gate) ──────────────────────

/// Check if the current process has a given promise.
/// Returns true if allowed, false if blocked.
#[no_mangle]
pub unsafe extern "C" fn sigma_pledge_check(promise_bit: u64) -> bool {
    extern "C" { fn sigma_getpid() -> u32; }
    let state = get_proc_state(sigma_getpid());
    if !state.pledged { return true; } // unrestricted
    state.promises & promise_bit != 0
}

/// Check if the current process can access a given path with given operation.
/// operation: 'r'=1, 'w'=2, 'x'=4, 'c'=8
/// Returns true if allowed.
#[no_mangle]
pub unsafe extern "C" fn sigma_unveil_check(
    path_ptr: *const u8, path_len: usize, operation: u8,
) -> bool {
    extern "C" { fn sigma_getpid() -> u32; }
    let state = get_proc_state(sigma_getpid());

    // If no unveils registered, all paths are accessible
    if state.unveil_count == 0 { return true; }

    let path = core::slice::from_raw_parts(path_ptr, path_len);

    for i in 0..state.unveil_count {
        let e = &state.unveils[i];
        if !e.valid { continue; }
        let ep = &e.path[..e.path_len];
        // Match if path starts with the unveiled prefix
        if path.starts_with(ep) || path == ep {
            return e.perms & operation != 0;
        }
    }

    false // no matching unveil → deny
}

/// Map syscall number → required promise bit.
/// Called from sigma_syscall_dispatch before executing each syscall.
pub fn promise_for_syscall(nr: u64) -> u64 {
    match nr {
        0..=3   => PLEDGE_STDIO,   // read, write, open, close
        4..=6   => PLEDGE_RPATH,   // stat, fstat, lstat
        7       => PLEDGE_STDIO,   // poll
        8       => PLEDGE_STDIO,   // lseek
        9..=11  => PLEDGE_STDIO,   // mmap, mprotect, munmap
        22      => PLEDGE_STDIO,   // pipe
        32|33   => PLEDGE_STDIO,   // dup, dup2
        35      => PLEDGE_STDIO,   // nanosleep
        39|110  => PLEDGE_STDIO,   // getpid, getppid
        41..=50 => PLEDGE_INET,    // socket, connect, accept, send, recv, bind, listen
        56|57   => PLEDGE_PROC,    // clone, fork
        59      => PLEDGE_EXEC,    // execve
        60|231  => PLEDGE_STDIO,   // exit, exit_group
        61      => PLEDGE_PROC,    // wait4
        62      => PLEDGE_PROC,    // kill
        79|80   => PLEDGE_RPATH,   // getcwd, chdir
        83      => PLEDGE_CPATH,   // mkdir
        84      => PLEDGE_CPATH,   // rmdir
        87      => PLEDGE_WPATH,   // unlink
        90      => PLEDGE_FATTR,   // chmod
        92      => PLEDGE_CHOWN,   // chown
        228     => PLEDGE_STDIO,   // clock_gettime
        _       => PLEDGE_STDIO,   // default: require at least stdio
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_pledge_check_syscall(nr: u64) -> bool {
    let bit = promise_for_syscall(nr);
    sigma_pledge_check(bit)
}
