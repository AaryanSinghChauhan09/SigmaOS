// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/fs/sigma_procfs.rs — /proc filesystem (no_std, virtual)
// Language: Rust #![no_std]
// Pattern: OOP via ProcFs struct implementing FileSystem trait

#![no_std]

use crate::kernel::fs::sigma_vfs::{FileSystem, OpenFlags, InodeMeta, InodeKind, VfsResult, VfsError};
use crate::kernel::core::sigma_sched::{Task, TaskState};

pub const MAX_PROC_FILES: usize = 64;

// ── ProcEntry: generated-on-read virtual file ─────────────────────────────────

type ProcReadFn = fn(out: &mut [u8; 4096]) -> usize;

struct ProcEntry {
    name:     [u8; 32],
    name_len: usize,
    read_fn:  ProcReadFn,
    inode:    u64,
}

// ── Built-in /proc generators ─────────────────────────────────────────────────

fn proc_version(out: &mut [u8; 4096]) -> usize {
    let s = b"SigmaOS version 15.0.0 (Zenith) #1 SMP (Rust/Zig/Nim/SPARK)\n";
    let n = s.len().min(4096);
    out[..n].copy_from_slice(&s[..n]);
    n
}

fn proc_uptime(out: &mut [u8; 4096]) -> usize {
    // TODO: read real tick counter from sigma_timer
    let s = b"0.00 0.00\n";
    let n = s.len();
    out[..n].copy_from_slice(s);
    n
}

fn proc_meminfo(out: &mut [u8; 4096]) -> usize {
    let s = b"MemTotal:        524288 kB\nMemFree:         524288 kB\nMemAvailable:    524288 kB\n";
    let n = s.len().min(4096);
    out[..n].copy_from_slice(&s[..n]);
    n
}

fn proc_cpuinfo(out: &mut [u8; 4096]) -> usize {
    let s = b"processor\t: 0\nvendor_id\t: SigmaOS\nmodel name\t: SovereignCPU\nbogomips\t: 1000.00\n";
    let n = s.len().min(4096);
    out[..n].copy_from_slice(&s[..n]);
    n
}

fn proc_filesystems(out: &mut [u8; 4096]) -> usize {
    let s = b"nodev\ttmpfs\nnodev\tprocfs\n\tfat32\n\tsigmafs\n";
    let n = s.len().min(4096);
    out[..n].copy_from_slice(&s[..n]);
    n
}

fn proc_mounts(out: &mut [u8; 4096]) -> usize {
    let s = b"tmpfs / tmpfs rw 0 0\nprocfs /proc procfs rw 0 0\n";
    let n = s.len().min(4096);
    out[..n].copy_from_slice(&s[..n]);
    n
}

// ── ProcFs ────────────────────────────────────────────────────────────────────

pub struct ProcFs {
    entries:    [Option<ProcEntry>; MAX_PROC_FILES],
    count:      usize,
    read_cache: [u8; 4096],
    cache_ino:  u64,
    cache_len:  usize,
}

impl ProcFs {
    pub fn new() -> Self {
        let mut fs = Self {
            entries: [const { None }; MAX_PROC_FILES],
            count: 0,
            read_cache: [0u8; 4096],
            cache_ino: 0,
            cache_len: 0,
        };
        // Register built-in entries
        fs.register(b"version",     proc_version,     1);
        fs.register(b"uptime",      proc_uptime,      2);
        fs.register(b"meminfo",     proc_meminfo,     3);
        fs.register(b"cpuinfo",     proc_cpuinfo,     4);
        fs.register(b"filesystems", proc_filesystems, 5);
        fs.register(b"mounts",      proc_mounts,      6);
        fs
    }

    pub fn register(&mut self, name: &[u8], read_fn: ProcReadFn, inode: u64) -> bool {
        if self.count >= MAX_PROC_FILES { return false; }
        let mut e = ProcEntry {
            name: [0u8; 32], name_len: name.len().min(32),
            read_fn, inode,
        };
        e.name[..e.name_len].copy_from_slice(&name[..e.name_len]);
        self.entries[self.count] = Some(e);
        self.count += 1;
        true
    }

    fn find_entry(&self, name: &[u8]) -> Option<&ProcEntry> {
        let stripped = if name.first() == Some(&b'/') { &name[1..] } else { name };
        for e in self.entries[..self.count].iter().flatten() {
            if &e.name[..e.name_len] == stripped { return Some(e); }
        }
        None
    }
}

impl FileSystem for ProcFs {
    fn name(&self) -> &'static str { "procfs" }
    fn mount(&mut self, _device: usize) -> VfsResult<()> { Ok(()) }
    fn umount(&mut self) -> VfsResult<()> { Ok(()) }

    fn lookup(&self, path: &[u8]) -> VfsResult<InodeMeta> {
        let stripped = if path.first() == Some(&b'/') { &path[1..] } else { path };
        if stripped.is_empty() {
            return Ok(InodeMeta { kind: InodeKind::Directory, size: 0,
                                  inode: 0, nlinks: 1, uid: 0, gid: 0, mode: 0o555 });
        }
        let e = self.find_entry(path).ok_or(VfsError::NotFound)?;
        Ok(InodeMeta { kind: InodeKind::File, size: 0,
                       inode: e.inode, nlinks: 1, uid: 0, gid: 0, mode: 0o444 })
    }

    fn open(&mut self, path: &[u8], _flags: OpenFlags) -> VfsResult<u64> {
        let e = self.find_entry(path).ok_or(VfsError::NotFound)?;
        let ino = e.inode;
        // Pre-fill cache
        self.cache_len = (e.read_fn)(&mut self.read_cache);
        self.cache_ino = ino;
        Ok(ino)
    }

    fn close(&mut self, _handle: u64) -> VfsResult<()> { Ok(()) }

    fn read(&mut self, handle: u64, buf: &mut [u8], offset: u64) -> VfsResult<usize> {
        if handle != self.cache_ino {
            // Re-generate for this inode
            for e in self.entries[..self.count].iter().flatten() {
                if e.inode == handle {
                    self.cache_len = (e.read_fn)(&mut self.read_cache);
                    self.cache_ino = handle;
                    break;
                }
            }
        }
        let off = offset as usize;
        if off >= self.cache_len { return Ok(0); }
        let n = buf.len().min(self.cache_len - off);
        buf[..n].copy_from_slice(&self.read_cache[off..off+n]);
        Ok(n)
    }

    fn write  (&mut self, _h: u64, _b: &[u8], _o: u64) -> VfsResult<usize> { Err(VfsError::NotSupported) }
    fn mkdir  (&mut self, _p: &[u8]) -> VfsResult<()> { Err(VfsError::NotSupported) }
    fn unlink (&mut self, _p: &[u8]) -> VfsResult<()> { Err(VfsError::NotSupported) }
    fn rename (&mut self, _o: &[u8], _n: &[u8]) -> VfsResult<()> { Err(VfsError::NotSupported) }

    fn readdir(&self, _path: &[u8], cb: &mut dyn FnMut(&[u8], InodeKind)) -> VfsResult<()> {
        for e in self.entries[..self.count].iter().flatten() {
            cb(&e.name[..e.name_len], InodeKind::File);
        }
        Ok(())
    }

    fn stat(&self, path: &[u8]) -> VfsResult<InodeMeta> { self.lookup(path) }
}
