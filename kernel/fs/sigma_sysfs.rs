// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// kernel/fs/sigma_sysfs.rs — /sys virtual filesystem (no_std)
// Language: Rust #![no_std] — OOP via SysFs + SysNode

#![no_std]
use crate::kernel::fs::sigma_vfs::{FileSystem,OpenFlags,InodeMeta,InodeKind,VfsResult,VfsError};

pub const MAX_NODES: usize = 256;
pub const MAX_NAME:  usize = 64;
pub const MAX_VAL:   usize = 256;

pub type ReadFn  = fn(out: &mut [u8; MAX_VAL]) -> usize;
pub type WriteFn = fn(data: &[u8]) -> bool;

// ── SysNode ───────────────────────────────────────────────────────────────────
#[derive(Clone, Copy)]
pub struct SysNode {
    pub path:      [u8; 128],
    pub path_len:  usize,
    pub kind:      InodeKind,
    pub read_fn:   Option<ReadFn>,
    pub write_fn:  Option<WriteFn>,
    pub inode:     u64,
}

impl SysNode {
    pub const fn new_dir(inode: u64, path: &[u8]) -> Self {
        let mut p = [0u8; 128]; let n = path.len().min(128);
        let mut i = 0; while i < n { p[i] = path[i]; i += 1; }
        Self { path: p, path_len: n, kind: InodeKind::Directory,
               read_fn: None, write_fn: None, inode }
    }
    pub const fn new_file(inode: u64, path: &[u8], r: Option<ReadFn>, w: Option<WriteFn>) -> Self {
        let mut p = [0u8; 128]; let n = path.len().min(128);
        let mut i = 0; while i < n { p[i] = path[i]; i += 1; }
        Self { path: p, path_len: n, kind: InodeKind::File,
               read_fn: r, write_fn: w, inode }
    }
}

// ── Built-in sysfs node generators ────────────────────────────────────────────

fn read_cpu_count(out: &mut [u8; MAX_VAL]) -> usize {
    let s = b"1\n"; out[..s.len()].copy_from_slice(s); s.len()
}
fn read_cpu_model(out: &mut [u8; MAX_VAL]) -> usize {
    let s = b"SovereignCPU x86_64\n"; let n = s.len().min(MAX_VAL);
    out[..n].copy_from_slice(&s[..n]); n
}
fn read_mem_total(out: &mut [u8; MAX_VAL]) -> usize {
    let s = b"524288\n"; out[..s.len()].copy_from_slice(s); s.len()
}
fn read_kernel_version(out: &mut [u8; MAX_VAL]) -> usize {
    let s = b"SigmaOS-15.0.0\n"; out[..s.len()].copy_from_slice(s); s.len()
}
fn read_uptime(out: &mut [u8; MAX_VAL]) -> usize {
    let s = b"0 0\n"; out[..s.len()].copy_from_slice(s); s.len()
}

// ── SysFs ─────────────────────────────────────────────────────────────────────

pub struct SysFs {
    nodes:     [Option<SysNode>; MAX_NODES],
    n_nodes:   usize,
    val_cache: [u8; MAX_VAL],
    cache_ino: u64,
    cache_len: usize,
}

impl SysFs {
    pub fn new() -> Self {
        let mut fs = Self {
            nodes: [const { None }; MAX_NODES], n_nodes: 0,
            val_cache: [0u8; MAX_VAL], cache_ino: 0, cache_len: 0,
        };
        fs.register_defaults();
        fs
    }

    fn reg(&mut self, node: SysNode) {
        if self.n_nodes < MAX_NODES {
            for slot in &mut self.nodes { if slot.is_none() { *slot = Some(node); self.n_nodes += 1; return; } }
        }
    }

    fn register_defaults(&mut self) {
        // Directories
        self.reg(SysNode::new_dir(1, b"/"));
        self.reg(SysNode::new_dir(2, b"/cpu"));
        self.reg(SysNode::new_dir(3, b"/memory"));
        self.reg(SysNode::new_dir(4, b"/kernel"));
        self.reg(SysNode::new_dir(5, b"/net"));
        self.reg(SysNode::new_dir(6, b"/block"));
        self.reg(SysNode::new_dir(7, b"/class"));
        // CPU files
        self.reg(SysNode::new_file(10, b"/cpu/count",  Some(read_cpu_count),  None));
        self.reg(SysNode::new_file(11, b"/cpu/model",  Some(read_cpu_model),  None));
        // Memory files
        self.reg(SysNode::new_file(20, b"/memory/total", Some(read_mem_total), None));
        // Kernel files
        self.reg(SysNode::new_file(30, b"/kernel/version", Some(read_kernel_version), None));
        self.reg(SysNode::new_file(31, b"/kernel/uptime",  Some(read_uptime),         None));
    }

    pub fn add_node(&mut self, node: SysNode) -> bool {
        if self.n_nodes >= MAX_NODES { return false; }
        self.reg(node); true
    }

    fn find(&self, path: &[u8]) -> Option<&SysNode> {
        let p = if path.first() == Some(&b'/') { path } else { path };
        self.nodes.iter().flatten().find(|n| &n.path[..n.path_len] == p)
    }
}

impl FileSystem for SysFs {
    fn name(&self) -> &'static str { "sysfs" }
    fn mount(&mut self, _: usize) -> VfsResult<()> { Ok(()) }
    fn umount(&mut self)           -> VfsResult<()> { Ok(()) }

    fn lookup(&self, path: &[u8]) -> VfsResult<InodeMeta> {
        let node = self.find(path).ok_or(VfsError::NotFound)?;
        Ok(InodeMeta { kind: node.kind, size: 0, inode: node.inode,
                       nlinks: 1, uid: 0, gid: 0, mode: 0o444 })
    }

    fn open(&mut self, path: &[u8], _: OpenFlags) -> VfsResult<u64> {
        let node = self.find(path).ok_or(VfsError::NotFound)?;
        let ino  = node.inode;
        if let Some(rf) = node.read_fn {
            self.cache_len = rf(&mut self.val_cache);
            self.cache_ino = ino;
        }
        Ok(ino)
    }

    fn close(&mut self, _: u64) -> VfsResult<()> { Ok(()) }

    fn read(&mut self, handle: u64, buf: &mut [u8], offset: u64) -> VfsResult<usize> {
        if handle != self.cache_ino {
            for node in self.nodes.iter().flatten() {
                if node.inode == handle {
                    if let Some(rf) = node.read_fn {
                        self.cache_len = rf(&mut self.val_cache);
                        self.cache_ino = handle;
                    }
                    break;
                }
            }
        }
        let off = offset as usize;
        if off >= self.cache_len { return Ok(0); }
        let n = buf.len().min(self.cache_len - off);
        buf[..n].copy_from_slice(&self.val_cache[off..off+n]);
        Ok(n)
    }

    fn write(&mut self, handle: u64, data: &[u8], _: u64) -> VfsResult<usize> {
        for node in self.nodes.iter().flatten() {
            if node.inode == handle {
                if let Some(wf) = node.write_fn {
                    return if wf(data) { Ok(data.len()) } else { Err(VfsError::IoError) };
                }
                return Err(VfsError::PermissionDenied);
            }
        }
        Err(VfsError::BadFileDescriptor)
    }

    fn mkdir(&mut self, _: &[u8])         -> VfsResult<()> { Err(VfsError::NotSupported) }
    fn unlink(&mut self, _: &[u8])        -> VfsResult<()> { Err(VfsError::NotSupported) }
    fn rename(&mut self, _: &[u8], _: &[u8]) -> VfsResult<()> { Err(VfsError::NotSupported) }

    fn readdir(&self, path: &[u8], cb: &mut dyn FnMut(&[u8], InodeKind)) -> VfsResult<()> {
        let dir_path = if path.first() == Some(&b'/') { path } else { path };
        for node in self.nodes.iter().flatten() {
            let np = &node.path[..node.path_len];
            if np == dir_path { continue; } // skip self
            if np.starts_with(dir_path) {
                let rest = &np[dir_path.len()..];
                let rest = if rest.first() == Some(&b'/') { &rest[1..] } else { rest };
                if !rest.is_empty() && !rest.contains(&b'/') {
                    cb(rest, node.kind);
                }
            }
        }
        Ok(())
    }

    fn stat(&self, path: &[u8]) -> VfsResult<InodeMeta> { self.lookup(path) }
}
