// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/sigma_syscalls_io.rs — File I/O syscall implementations
// Implements: read, write, open, close, lseek, stat, fstat, dup, dup2,
//             readv, writev, pread64, pwrite64, ioctl, fcntl
//
// These wire the syscall dispatch table to the VFS layer.
// Language: Rust #![no_std]

#![no_std]
#![allow(dead_code)]

// ── Error codes ────────────────────────────────────────────────────────────
pub const EBADF:   i64 = -9;
pub const EFAULT:  i64 = -14;
pub const EINVAL:  i64 = -22;
pub const ENOSYS:  i64 = -38;
pub const ENOENT:  i64 = -2;
pub const ENOMEM:  i64 = -12;
pub const EPERM:   i64 = -1;
pub const ENOSPC:  i64 = -28;
pub const EISDIR:  i64 = -21;
pub const EEXIST:  i64 = -17;
pub const ENOTDIR: i64 = -20;
pub const EIO:     i64 = -5;

// ── File Descriptor Table ─────────────────────────────────────────────────
pub const MAX_FD: usize = 256;

#[derive(Copy, Clone, PartialEq)]
pub enum FdKind {
    Free,
    File,   // regular file backed by VFS
    Pipe,   // one end of a pipe
    Device, // /dev/null, /dev/zero, /dev/urandom
    Socket, // network socket
}

#[derive(Copy, Clone)]
pub struct FdEntry {
    pub kind:    FdKind,
    pub flags:   u32,    // O_* flags
    pub offset:  u64,    // current file position
    pub vfs_handle: u64, // handle from VFS
    pub path_hash: u32,  // hash of the path for VFS dispatch
    pub pipe_id: u32,    // for FdKind::Pipe
}

impl FdEntry {
    pub const fn empty() -> Self {
        FdEntry { kind: FdKind::Free, flags: 0, offset: 0, vfs_handle: 0, path_hash: 0, pipe_id: 0 }
    }
}

static mut FD_TABLE: [FdEntry; MAX_FD] = [FdEntry::empty(); MAX_FD];

// ── Open flags (Linux-compatible) ─────────────────────────────────────────
pub const O_RDONLY:  u32 = 0o0;
pub const O_WRONLY:  u32 = 0o1;
pub const O_RDWR:    u32 = 0o2;
pub const O_CREAT:   u32 = 0o100;
pub const O_EXCL:    u32 = 0o200;
pub const O_TRUNC:   u32 = 0o1000;
pub const O_APPEND:  u32 = 0o2000;
pub const O_NONBLOCK:u32 = 0o4000;
pub const O_CLOEXEC: u32 = 0o2000000;

// ── Device special paths ──────────────────────────────────────────────────
const DEV_NULL:    &[u8] = b"/dev/null";
const DEV_ZERO:    &[u8] = b"/dev/zero";
const DEV_URANDOM: &[u8] = b"/dev/urandom";
const DEV_STDIN:   &[u8] = b"/dev/stdin";
const DEV_STDOUT:  &[u8] = b"/dev/stdout";
const DEV_STDERR:  &[u8] = b"/dev/stderr";

fn is_dev_path(path: &[u8]) -> Option<u32> {
    if path == DEV_NULL    { return Some(1); }
    if path == DEV_ZERO    { return Some(2); }
    if path == DEV_URANDOM { return Some(3); }
    if path == DEV_STDIN   { return Some(4); }
    if path == DEV_STDOUT  { return Some(5); }
    if path == DEV_STDERR  { return Some(6); }
    None
}

fn djb2_hash(s: &[u8]) -> u32 {
    s.iter().fold(5381u32, |h, &b| h.wrapping_mul(33).wrapping_add(b as u32))
}

// ── Serial/console output (for write to fd 1/2) ────────────────────────────
unsafe fn serial_write_bytes(data: &[u8]) {
    extern "C" { fn serial_putc(c: u8); }
    for &b in data { serial_putc(b); }
}

// ── PRNG for /dev/urandom ──────────────────────────────────────────────────
static mut URANDOM_STATE: u64 = 0xdeadbeef_12345678;
unsafe fn urandom_byte() -> u8 {
    URANDOM_STATE ^= URANDOM_STATE << 13;
    URANDOM_STATE ^= URANDOM_STATE >> 7;
    URANDOM_STATE ^= URANDOM_STATE << 17;
    (URANDOM_STATE & 0xFF) as u8
}

// ── FD allocation helpers ──────────────────────────────────────────────────
unsafe fn alloc_fd() -> Option<usize> {
    for i in 3..MAX_FD {  // 0/1/2 reserved for stdin/stdout/stderr
        if FD_TABLE[i].kind == FdKind::Free { return Some(i); }
    }
    None
}

unsafe fn get_fd(fd: i32) -> Option<&'static mut FdEntry> {
    if fd < 0 || fd as usize >= MAX_FD { return None; }
    let entry = &mut FD_TABLE[fd as usize];
    if entry.kind == FdKind::Free { return None; }
    Some(entry)
}

// ── sys_open ──────────────────────────────────────────────────────────────
pub unsafe fn sys_open(path_ptr: u64, flags: u64, _mode: u64) -> i64 {
    if path_ptr == 0 { return EFAULT; }
    // Read C-string from user pointer (bounded read)
    let mut path_buf = [0u8; 256];
    let mut len = 0;
    let ptr = path_ptr as *const u8;
    while len < 255 {
        let b = core::ptr::read_volatile(ptr.add(len));
        if b == 0 { break; }
        path_buf[len] = b;
        len += 1;
    }
    let path = &path_buf[..len];

    let fd_idx = alloc_fd().ok_or(()).map_err(|_| -24i64)?;  // EMFILE
    let f_flags = flags as u32;

    // Check for device special files
    if let Some(dev_id) = is_dev_path(path) {
        FD_TABLE[fd_idx] = FdEntry {
            kind: FdKind::Device, flags: f_flags,
            offset: 0, vfs_handle: dev_id as u64,
            path_hash: dev_id, pipe_id: 0,
        };
        return fd_idx as i64;
    }

    // /proc and /sys paths handled by proc_shim
    if path.starts_with(b"/proc") || path.starts_with(b"/sys") {
        FD_TABLE[fd_idx] = FdEntry {
            kind: FdKind::File, flags: f_flags,
            offset: 0, vfs_handle: djb2_hash(path) as u64,
            path_hash: djb2_hash(path), pipe_id: 0,
        };
        return fd_idx as i64;
    }

    // VFS dispatch
    extern "C" {
        fn sigma_vfs_open(path: *const u8, len: usize, flags: u32, out_handle: *mut u64) -> i32;
    }
    let mut handle: u64 = 0;
    let ret = sigma_vfs_open(path.as_ptr(), path.len(), f_flags, &mut handle);
    if ret < 0 { return ENOENT; }

    FD_TABLE[fd_idx] = FdEntry {
        kind: FdKind::File, flags: f_flags,
        offset: 0, vfs_handle: handle,
        path_hash: djb2_hash(path), pipe_id: 0,
    };
    fd_idx as i64
}

// ── sys_close ─────────────────────────────────────────────────────────────
pub unsafe fn sys_close(fd: u64) -> i64 {
    let f = fd as i32;
    if f < 0 || f as usize >= MAX_FD { return EBADF; }
    if FD_TABLE[f as usize].kind == FdKind::Free { return EBADF; }
    FD_TABLE[f as usize] = FdEntry::empty();
    0
}

// ── sys_read ──────────────────────────────────────────────────────────────
pub unsafe fn sys_read(fd: u64, buf_ptr: u64, count: u64) -> i64 {
    if buf_ptr == 0 { return EFAULT; }
    let entry = match get_fd(fd as i32) {
        Some(e) => e,
        None => {
            // stdin (fd=0): return EOF
            if fd == 0 { return 0; }
            return EBADF;
        }
    };

    let buf = core::slice::from_raw_parts_mut(buf_ptr as *mut u8, count as usize);

    match entry.kind {
        FdKind::Device => {
            match entry.vfs_handle {
                1 => { return 0; }  // /dev/null → EOF
                2 => { buf.fill(0); return count as i64; }  // /dev/zero
                3 => {
                    for b in buf.iter_mut() { *b = urandom_byte(); }
                    return count as i64;
                }
                4 => { return 0; }  // stdin EOF for now
                _ => { return EBADF; }
            }
        }
        FdKind::File => {
            extern "C" {
                fn sigma_vfs_read(handle: u64, buf: *mut u8, len: usize, off: u64) -> i64;
            }
            let n = sigma_vfs_read(entry.vfs_handle, buf.as_mut_ptr(), buf.len(), entry.offset);
            if n > 0 { entry.offset += n as u64; }
            n
        }
        FdKind::Pipe => {
            extern "C" { fn sigma_pipe_read(id: u32, buf: *mut u8, len: usize) -> i64; }
            sigma_pipe_read(entry.pipe_id, buf.as_mut_ptr(), buf.len())
        }
        _ => EBADF,
    }
}

// ── sys_write ─────────────────────────────────────────────────────────────
pub unsafe fn sys_write(fd: u64, buf_ptr: u64, count: u64) -> i64 {
    if buf_ptr == 0 { return EFAULT; }
    let buf = core::slice::from_raw_parts(buf_ptr as *const u8, count as usize);

    // fd 1/2: write to serial console
    if fd == 1 || fd == 2 {
        serial_write_bytes(buf);
        return count as i64;
    }

    let entry = match get_fd(fd as i32) {
        Some(e) => e,
        None => return EBADF,
    };

    match entry.kind {
        FdKind::Device => {
            match entry.vfs_handle {
                1 | 5 | 6 => { return count as i64; }  // /dev/null, stdout, stderr → discard
                _ => return EBADF,
            }
        }
        FdKind::File => {
            extern "C" {
                fn sigma_vfs_write(handle: u64, buf: *const u8, len: usize, off: u64) -> i64;
            }
            let n = sigma_vfs_write(entry.vfs_handle, buf.as_ptr(), buf.len(), entry.offset);
            if n > 0 { entry.offset += n as u64; }
            n
        }
        FdKind::Pipe => {
            extern "C" { fn sigma_pipe_write(id: u32, buf: *const u8, len: usize) -> i64; }
            sigma_pipe_write(entry.pipe_id, buf.as_ptr(), buf.len())
        }
        _ => EBADF,
    }
}

// ── sys_lseek ─────────────────────────────────────────────────────────────
pub unsafe fn sys_lseek(fd: u64, offset: u64, whence: u64) -> i64 {
    const SEEK_SET: u64 = 0; const SEEK_CUR: u64 = 1; const SEEK_END: u64 = 2;
    let entry = match get_fd(fd as i32) { Some(e) => e, None => return EBADF };
    let signed_off = offset as i64;
    let new_off = match whence {
        SEEK_SET => { if signed_off < 0 { return EINVAL; } signed_off as u64 }
        SEEK_CUR => entry.offset.wrapping_add(signed_off as u64),
        SEEK_END  => {
            // Get file size from VFS
            extern "C" { fn sigma_vfs_size(handle: u64) -> u64; }
            let size = sigma_vfs_size(entry.vfs_handle);
            size.wrapping_add(signed_off as u64)
        }
        _ => return EINVAL,
    };
    entry.offset = new_off;
    new_off as i64
}

// ── sys_dup / sys_dup2 ─────────────────────────────────────────────────────
pub unsafe fn sys_dup(fd: u64) -> i64 {
    let entry_copy = match get_fd(fd as i32) { Some(e) => *e, None => return EBADF };
    let new_fd = match alloc_fd() { Some(f) => f, None => return -24 };
    FD_TABLE[new_fd] = entry_copy;
    new_fd as i64
}

pub unsafe fn sys_dup2(old_fd: u64, new_fd: u64) -> i64 {
    if new_fd as usize >= MAX_FD { return EBADF; }
    let entry_copy = match get_fd(old_fd as i32) { Some(e) => *e, None => return EBADF };
    if FD_TABLE[new_fd as usize].kind != FdKind::Free {
        // Close existing fd
        let _ = sys_close(new_fd);
    }
    FD_TABLE[new_fd as usize] = entry_copy;
    new_fd as i64
}

// ── sys_stat / sys_fstat ──────────────────────────────────────────────────
// struct stat layout (Linux x86_64, simplified)
#[repr(C)]
struct KStat {
    st_dev: u64, st_ino: u64, st_nlink: u64,
    st_mode: u32, st_uid: u32, st_gid: u32, _pad0: u32,
    st_rdev: u64, st_size: i64, st_blksize: i64, st_blocks: i64,
    st_atime: i64, _atime_ns: i64, st_mtime: i64, _mtime_ns: i64,
    st_ctime: i64, _ctime_ns: i64, _unused: [i64; 3],
}

pub unsafe fn sys_stat(path_ptr: u64, stat_ptr: u64) -> i64 {
    if path_ptr == 0 || stat_ptr == 0 { return EFAULT; }
    let stat = &mut *(stat_ptr as *mut KStat);
    // Fake stat for now — real impl queries VFS
    *stat = core::mem::zeroed();
    stat.st_mode = 0o100644;
    stat.st_size = 4096;
    stat.st_blksize = 4096;
    stat.st_blocks = 8;
    0
}

pub unsafe fn sys_fstat(fd: u64, stat_ptr: u64) -> i64 {
    if stat_ptr == 0 { return EFAULT; }
    let _ = get_fd(fd as i32).ok_or(EBADF);
    let stat = &mut *(stat_ptr as *mut KStat);
    *stat = core::mem::zeroed();
    stat.st_mode = 0o100644;
    stat.st_size = 4096;
    stat.st_blksize = 4096;
    stat.st_blocks = 8;
    0
}

// ── sys_fcntl ─────────────────────────────────────────────────────────────
pub unsafe fn sys_fcntl(fd: u64, cmd: u64, arg: u64) -> i64 {
    const F_DUPFD:     u64 = 0;
    const F_GETFD:     u64 = 1;
    const F_SETFD:     u64 = 2;
    const F_GETFL:     u64 = 3;
    const F_SETFL:     u64 = 4;
    const F_DUPFD_CLOEXEC: u64 = 1030;
    match cmd {
        F_DUPFD | F_DUPFD_CLOEXEC => sys_dup(fd),
        F_GETFD => 0,
        F_SETFD => 0,
        F_GETFL => {
            let e = match get_fd(fd as i32) { Some(e) => e, None => return EBADF };
            e.flags as i64
        }
        F_SETFL => {
            let e = match get_fd(fd as i32) { Some(e) => e, None => return EBADF };
            e.flags = arg as u32;
            0
        }
        _ => EINVAL,
    }
}

// ── sys_ioctl ─────────────────────────────────────────────────────────────
pub unsafe fn sys_ioctl(fd: u64, request: u64, arg: u64) -> i64 {
    const TIOCGWINSZ: u64 = 0x5413;
    const TCGETS:     u64 = 0x5401;
    const TCSETS:     u64 = 0x5402;
    const FIONREAD:   u64 = 0x541B;
    match request {
        TIOCGWINSZ => {
            if arg != 0 {
                // winsize: ws_row, ws_col, ws_xpixel, ws_ypixel (each u16)
                let p = arg as *mut u16;
                p.write(24); p.add(1).write(80); p.add(2).write(0); p.add(3).write(0);
            }
            0
        }
        TCGETS => 0,  // return success (pretend terminal is happy)
        TCSETS => 0,
        FIONREAD => {
            if arg != 0 { (arg as *mut i32).write(0); }
            0
        }
        _ => EINVAL,
    }
}

// ── sys_readv / sys_writev ────────────────────────────────────────────────
#[repr(C)] struct IoVec { base: u64, len: u64 }

pub unsafe fn sys_readv(fd: u64, iov_ptr: u64, iovcnt: u64) -> i64 {
    if iov_ptr == 0 { return EFAULT; }
    let mut total = 0i64;
    for i in 0..iovcnt as usize {
        let iov = &*(( iov_ptr as *const IoVec).add(i));
        if iov.base == 0 || iov.len == 0 { continue; }
        let n = sys_read(fd, iov.base, iov.len);
        if n < 0 { return if total == 0 { n } else { total }; }
        total += n;
    }
    total
}

pub unsafe fn sys_writev(fd: u64, iov_ptr: u64, iovcnt: u64) -> i64 {
    if iov_ptr == 0 { return EFAULT; }
    let mut total = 0i64;
    for i in 0..iovcnt as usize {
        let iov = &*((iov_ptr as *const IoVec).add(i));
        if iov.base == 0 || iov.len == 0 { continue; }
        let n = sys_write(fd, iov.base, iov.len);
        if n < 0 { return if total == 0 { n } else { total }; }
        total += n;
    }
    total
}

// ── sys_pread64 / sys_pwrite64 ────────────────────────────────────────────
pub unsafe fn sys_pread64(fd: u64, buf: u64, count: u64, offset: u64) -> i64 {
    let entry = match get_fd(fd as i32) { Some(e) => e, None => return EBADF };
    let saved = entry.offset;
    entry.offset = offset;
    let n = sys_read(fd, buf, count);
    if let Some(e) = get_fd(fd as i32) { e.offset = saved; }
    n
}

pub unsafe fn sys_pwrite64(fd: u64, buf: u64, count: u64, offset: u64) -> i64 {
    let entry = match get_fd(fd as i32) { Some(e) => e, None => return EBADF };
    let saved = entry.offset;
    entry.offset = offset;
    let n = sys_write(fd, buf, count);
    if let Some(e) = get_fd(fd as i32) { e.offset = saved; }
    n
}

// ── C-ABI bridge (called from sigma_syscall_dispatch) ─────────────────────
#[no_mangle] pub unsafe extern "C" fn sigma_sys_open(p: u64, f: u64, m: u64) -> i64   { sys_open(p,f,m) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_close(fd: u64) -> i64                   { sys_close(fd) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_read(fd: u64, b: u64, n: u64) -> i64   { sys_read(fd,b,n) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_write(fd: u64, b: u64, n: u64) -> i64  { sys_write(fd,b,n) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_lseek(fd: u64, o: u64, w: u64) -> i64  { sys_lseek(fd,o,w) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_dup(fd: u64) -> i64                     { sys_dup(fd) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_dup2(o: u64, n: u64) -> i64             { sys_dup2(o,n) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_stat(p: u64, s: u64) -> i64             { sys_stat(p,s) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_fstat(fd: u64, s: u64) -> i64           { sys_fstat(fd,s) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_fcntl(fd: u64, c: u64, a: u64) -> i64  { sys_fcntl(fd,c,a) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_ioctl(fd: u64, r: u64, a: u64) -> i64  { sys_ioctl(fd,r,a) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_readv(fd: u64, v: u64, c: u64) -> i64  { sys_readv(fd,v,c) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_writev(fd: u64, v: u64, c: u64) -> i64 { sys_writev(fd,v,c) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_pread64(fd: u64, b: u64, n: u64, o: u64) -> i64  { sys_pread64(fd,b,n,o) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_pwrite64(fd: u64, b: u64, n: u64, o: u64) -> i64 { sys_pwrite64(fd,b,n,o) }

/// Initialize stdin/stdout/stderr file descriptors (called at boot)
#[no_mangle]
pub unsafe extern "C" fn sigma_fd_init() {
    // fd 0 = stdin (device 4)
    FD_TABLE[0] = FdEntry { kind: FdKind::Device, flags: O_RDONLY, offset: 0, vfs_handle: 4, path_hash: 4, pipe_id: 0 };
    // fd 1 = stdout (device 5)
    FD_TABLE[1] = FdEntry { kind: FdKind::Device, flags: O_WRONLY, offset: 0, vfs_handle: 5, path_hash: 5, pipe_id: 0 };
    // fd 2 = stderr (device 6)
    FD_TABLE[2] = FdEntry { kind: FdKind::Device, flags: O_WRONLY, offset: 0, vfs_handle: 6, path_hash: 6, pipe_id: 0 };
}
