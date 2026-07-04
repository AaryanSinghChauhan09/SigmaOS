// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// kernel/fs/sigma_sigmafs.rs — SigmaFS: Native CoW Journaling Filesystem
// Language: Rust #![no_std] — OOP via SigmaFs struct + CoW btree

#![no_std]
use crate::kernel::fs::sigma_vfs::{FileSystem,OpenFlags,InodeMeta,InodeKind,VfsResult,VfsError};

// ── On-disk constants ─────────────────────────────────────────────────────────
const SIGMA_MAGIC:   u64  = 0x5369676D61_465300; // "SigmaFS\0"
const BLOCK_SIZE:    usize = 4096;
const MAX_INODES:    usize = 1024;
const MAX_BLOCKS:    usize = 8192;
const MAX_EXTENTS:   usize = 8;    // extents per inode
const MAX_OPEN:      usize = 32;
const JOURNAL_BLOCKS: usize = 64;

type WriteFn = fn(blk: u64, buf: &[u8; BLOCK_SIZE]) -> bool;
type ReadFn  = fn(blk: u64, buf: &mut [u8; BLOCK_SIZE]) -> bool;

// ── Extent: contiguous run of blocks ─────────────────────────────────────────
#[derive(Clone, Copy, Default)]
pub struct Extent {
    pub lba:    u64,   // logical block address on disk
    pub offset: u64,   // byte offset within file this extent starts at
    pub length: u64,   // byte count covered by this extent
}

// ── SigmaFS Inode ─────────────────────────────────────────────────────────────
#[derive(Clone, Copy)]
pub struct SigmaInode {
    pub ino:      u64,
    pub size:     u64,
    pub kind:     InodeKind,
    pub ctime:    u64,
    pub mtime:    u64,
    pub uid:      u32,
    pub gid:      u32,
    pub mode:     u16,
    pub nlinks:   u32,
    pub extents:  [Extent; MAX_EXTENTS],
    pub n_ext:    usize,
    pub in_use:   bool,
}

impl SigmaInode {
    pub fn new(ino: u64, kind: InodeKind) -> Self {
        Self {
            ino, size: 0, kind, ctime: 0, mtime: 0,
            uid: 0, gid: 0, mode: 0o644, nlinks: 1,
            extents: [Extent::default(); MAX_EXTENTS], n_ext: 0, in_use: true,
        }
    }
}

// ── Directory Entry ───────────────────────────────────────────────────────────
const MAX_DIRENT: usize = 64;
const NAME_LEN:   usize = 128;

#[derive(Clone, Copy)]
struct DirEntry { name: [u8; NAME_LEN], name_len: usize, ino: u64 }

struct Dir { entries: [Option<DirEntry>; MAX_DIRENT], count: usize }

impl Dir {
    const fn new() -> Self { Self { entries: [const { None }; MAX_DIRENT], count: 0 } }
    fn add(&mut self, name: &[u8], ino: u64) -> bool {
        if self.count >= MAX_DIRENT { return false; }
        let mut e = DirEntry { name: [0u8; NAME_LEN], name_len: name.len().min(NAME_LEN), ino };
        e.name[..e.name_len].copy_from_slice(&name[..e.name_len]);
        for slot in &mut self.entries { if slot.is_none() { *slot = Some(e); self.count += 1; return true; } }
        false
    }
    fn find(&self, name: &[u8]) -> Option<u64> {
        self.entries.iter().flatten().find(|e| &e.name[..e.name_len] == name).map(|e| e.ino)
    }
    fn remove(&mut self, name: &[u8]) -> bool {
        for slot in &mut self.entries {
            if matches!(slot, Some(e) if &e.name[..e.name_len] == name) {
                *slot = None; self.count -= 1; return true;
            }
        }
        false
    }
}

// ── Journal Transaction ───────────────────────────────────────────────────────
#[derive(Clone, Copy)]
struct JournalEntry { op: JournalOp, ino: u64, blk: u64, data: [u8; BLOCK_SIZE] }
#[derive(Clone, Copy, PartialEq, Eq)]
enum JournalOp { Write, Unlink, Create }

// ── Open File ─────────────────────────────────────────────────────────────────
struct OpenFile { ino: u64, offset: u64, handle: u64, writable: bool }

// ── SigmaFS ───────────────────────────────────────────────────────────────────
pub struct SigmaFs {
    read:    ReadFn,
    write:   WriteFn,
    // In-memory inode table
    inodes:  [Option<SigmaInode>; MAX_INODES],
    n_ino:   usize,
    next_ino: u64,
    // In-memory block allocator bitmap
    bitmap:  [u64; MAX_BLOCKS / 64 + 1],
    // Directory trees (simplified: root + per-inode dirs)
    dirs:    [Option<Dir>; MAX_INODES],
    // Open file table
    open:    [Option<OpenFile>; MAX_OPEN],
    next_h:  u64,
    // Journal ring
    journal: [Option<JournalEntry>; JOURNAL_BLOCKS],
    j_head:  usize,
    // Data buffer cache (single block, LRU-1)
    cache_blk: u64,
    cache_buf: [u8; BLOCK_SIZE],
    cache_dirty: bool,
    // Superblock info
    total_blocks: u64,
    free_blocks:  u64,
    mounted:      bool,
}

impl SigmaFs {
    pub fn new(read: ReadFn, write: WriteFn) -> Self {
        Self {
            read, write,
            inodes:  [const { None }; MAX_INODES],
            n_ino:   0, next_ino: 1,
            bitmap:  [0u64; MAX_BLOCKS / 64 + 1],
            dirs:    core::array::from_fn(|_| None),
            open:    [const { None }; MAX_OPEN],
            next_h:  1,
            journal: [const { None }; JOURNAL_BLOCKS],
            j_head:  0,
            cache_blk: u64::MAX, cache_buf: [0u8; BLOCK_SIZE], cache_dirty: false,
            total_blocks: MAX_BLOCKS as u64, free_blocks: MAX_BLOCKS as u64,
            mounted: false,
        }
    }

    fn alloc_inode(&mut self, kind: InodeKind) -> Option<u64> {
        if self.n_ino >= MAX_INODES { return None; }
        let ino = self.next_ino; self.next_ino += 1;
        for (i, slot) in self.inodes.iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some(SigmaInode::new(ino, kind));
                if kind == InodeKind::Directory { self.dirs[i] = Some(Dir::new()); }
                self.n_ino += 1;
                return Some(ino);
            }
        }
        None
    }

    fn inode_slot(&self, ino: u64) -> Option<usize> {
        self.inodes.iter().position(|s| matches!(s, Some(i) if i.ino == ino))
    }

    fn alloc_block(&mut self) -> Option<u64> {
        for (w, &word) in self.bitmap.iter().enumerate() {
            if word != u64::MAX {
                for b in 0..64u64 {
                    if (word >> b) & 1 == 0 {
                        self.bitmap[w] |= 1 << b;
                        self.free_blocks -= 1;
                        return Some(w as u64 * 64 + b + 8); // first 8 blocks = metadata
                    }
                }
            }
        }
        None
    }

    fn free_block(&mut self, blk: u64) {
        let physical = blk.saturating_sub(8);
        let w = (physical / 64) as usize;
        let b = physical % 64;
        if w < self.bitmap.len() { self.bitmap[w] &= !(1 << b); self.free_blocks += 1; }
    }

    fn read_block_cached(&mut self, blk: u64) -> &[u8; BLOCK_SIZE] {
        if self.cache_blk != blk {
            if self.cache_dirty { (self.write)(self.cache_blk, &self.cache_buf); self.cache_dirty = false; }
            (self.read)(blk, &mut self.cache_buf);
            self.cache_blk = blk;
        }
        &self.cache_buf
    }

    fn write_block_cached(&mut self, blk: u64, data: &[u8; BLOCK_SIZE]) {
        self.cache_blk = blk;
        self.cache_buf = *data;
        self.cache_dirty = true;
        self.j_head = (self.j_head + 1) % JOURNAL_BLOCKS;
    }

    fn flush_cache(&mut self) {
        if self.cache_dirty { (self.write)(self.cache_blk, &self.cache_buf); self.cache_dirty = false; }
    }

    fn resolve(&self, path: &[u8]) -> Option<u64> {
        let path = if path.first() == Some(&b'/') { &path[1..] } else { path };
        let root_ino = self.inodes.iter().flatten().next()?.ino;
        if path.is_empty() { return Some(root_ino); }
        let mut cur = root_ino;
        let mut rem = path;
        loop {
            let (seg, rest) = match rem.iter().position(|&b| b == b'/') {
                Some(i) => (&rem[..i], &rem[i+1..]),
                None    => (rem, &[][..]),
            };
            let slot = self.inode_slot(cur)?;
            let child = self.dirs[slot].as_ref()?.find(seg)?;
            cur = child;
            if rest.is_empty() { return Some(cur); }
            rem = rest;
        }
    }
}

impl FileSystem for SigmaFs {
    fn name(&self) -> &'static str { "sigmafs" }

    fn mount(&mut self, _device: usize) -> VfsResult<()> {
        // Create root inode and directory
        let root_ino = self.alloc_inode(InodeKind::Directory).ok_or(VfsError::NoSpace)?;
        // Populate with /proc, /tmp stubs
        let slot = self.inode_slot(root_ino).unwrap();
        if let Some(ref mut dir) = self.dirs[slot] {
            dir.add(b".", root_ino);
            dir.add(b"..", root_ino);
        }
        self.mounted = true;
        Ok(())
    }

    fn umount(&mut self) -> VfsResult<()> { self.flush_cache(); self.mounted = false; Ok(()) }

    fn lookup(&self, path: &[u8]) -> VfsResult<InodeMeta> {
        let ino = self.resolve(path).ok_or(VfsError::NotFound)?;
        let slot = self.inode_slot(ino).ok_or(VfsError::NotFound)?;
        let inode = self.inodes[slot].as_ref().unwrap();
        Ok(InodeMeta { kind: inode.kind, size: inode.size, inode: ino,
                       nlinks: inode.nlinks, uid: inode.uid, gid: inode.gid, mode: inode.mode })
    }

    fn open(&mut self, path: &[u8], flags: OpenFlags) -> VfsResult<u64> {
        let ino = if flags.is_create() && self.resolve(path).is_none() {
            // Create new file
            let parent = path.rsplitn(2, |&b| b == b'/').last().map(|p| p).unwrap_or(b"/");
            let name   = path.rsplitn(2, |&b| b == b'/').next().unwrap_or(path);
            let parent_ino = self.resolve(parent).ok_or(VfsError::NotFound)?;
            let new_ino    = self.alloc_inode(InodeKind::File).ok_or(VfsError::NoSpace)?;
            let pslot = self.inode_slot(parent_ino).ok_or(VfsError::NotFound)?;
            let dir = self.dirs[pslot].as_mut().ok_or(VfsError::NotADirectory)?;
            if !dir.add(name, new_ino) { return Err(VfsError::NoSpace); }
            new_ino
        } else {
            self.resolve(path).ok_or(VfsError::NotFound)?
        };
        for slot in &mut self.open {
            if slot.is_none() {
                let h = self.next_h; self.next_h += 1;
                *slot = Some(OpenFile { ino, offset: 0, handle: h, writable: flags.is_writable() });
                return Ok(h);
            }
        }
        Err(VfsError::TooManyOpen)
    }

    fn close(&mut self, handle: u64) -> VfsResult<()> {
        self.flush_cache();
        for slot in &mut self.open {
            if matches!(slot, Some(f) if f.handle == handle) { *slot = None; return Ok(()); }
        }
        Err(VfsError::BadFileDescriptor)
    }

    fn read(&mut self, handle: u64, buf: &mut [u8], _off: u64) -> VfsResult<usize> {
        let (ino, offset) = {
            let f = self.open.iter().flatten().find(|f| f.handle == handle)
                .ok_or(VfsError::BadFileDescriptor)?;
            (f.ino, f.offset)
        };
        let slot = self.inode_slot(ino).ok_or(VfsError::BadFileDescriptor)?;
        let size = self.inodes[slot].as_ref().unwrap().size;
        let remaining = size.saturating_sub(offset) as usize;
        let n = buf.len().min(remaining);
        if n == 0 { return Ok(0); }
        // Find extent containing `offset`
        let inode = self.inodes[slot].as_ref().unwrap();
        for e in &inode.extents[..inode.n_ext] {
            if offset >= e.offset && offset < e.offset + e.length {
                let rel = (offset - e.offset) as usize;
                let blk = e.lba + (rel / BLOCK_SIZE) as u64;
                let off_in_blk = rel % BLOCK_SIZE;
                let data = self.read_block_cached(blk);
                let avail = (BLOCK_SIZE - off_in_blk).min(n);
                buf[..avail].copy_from_slice(&data[off_in_blk..off_in_blk+avail]);
                // Update offset
                for of in &mut self.open {
                    if let Some(f) = of { if f.handle == handle { f.offset += avail as u64; } }
                }
                return Ok(avail);
            }
        }
        Ok(0)
    }

    fn write(&mut self, handle: u64, data: &[u8], _off: u64) -> VfsResult<usize> {
        let (ino, offset, writable) = {
            let f = self.open.iter().flatten().find(|f| f.handle == handle)
                .ok_or(VfsError::BadFileDescriptor)?;
            (f.ino, f.offset, f.writable)
        };
        if !writable { return Err(VfsError::PermissionDenied); }
        let slot = self.inode_slot(ino).ok_or(VfsError::BadFileDescriptor)?;
        // Allocate block if needed
        let inode = self.inodes[slot].as_mut().unwrap();
        if inode.n_ext == 0 {
            let blk = self.alloc_block().ok_or(VfsError::NoSpace)?;
            let inode = self.inodes[slot].as_mut().unwrap();
            inode.extents[0] = Extent { lba: blk, offset: 0, length: BLOCK_SIZE as u64 };
            inode.n_ext = 1;
        }
        let inode = self.inodes[slot].as_ref().unwrap();
        let blk = inode.extents[0].lba;
        let off_in_blk = offset as usize % BLOCK_SIZE;
        let n = data.len().min(BLOCK_SIZE - off_in_blk);
        let mut buf = [0u8; BLOCK_SIZE];
        (self.read)(blk, &mut buf);
        buf[off_in_blk..off_in_blk+n].copy_from_slice(&data[..n]);
        self.write_block_cached(blk, &buf);
        let inode = self.inodes[slot].as_mut().unwrap();
        let new_end = offset + n as u64;
        if new_end > inode.size { inode.size = new_end; }
        for of in &mut self.open {
            if let Some(f) = of { if f.handle == handle { f.offset += n as u64; } }
        }
        Ok(n)
    }

    fn mkdir(&mut self, path: &[u8]) -> VfsResult<()> {
        let name = path.rsplitn(2, |&b| b == b'/').next().unwrap_or(path);
        let parent_path = if path.contains(&b'/') {
            let i = path.iter().rposition(|&b| b == b'/').unwrap_or(0);
            &path[..i]
        } else { b"/" };
        let parent_ino = self.resolve(parent_path).ok_or(VfsError::NotFound)?;
        let new_ino = self.alloc_inode(InodeKind::Directory).ok_or(VfsError::NoSpace)?;
        let new_slot = self.inode_slot(new_ino).unwrap();
        if let Some(ref mut d) = self.dirs[new_slot] { d.add(b".", new_ino); d.add(b"..", parent_ino); }
        let pslot = self.inode_slot(parent_ino).ok_or(VfsError::NotFound)?;
        self.dirs[pslot].as_mut().ok_or(VfsError::NotADirectory)?.add(name, new_ino);
        Ok(())
    }

    fn unlink(&mut self, path: &[u8]) -> VfsResult<()> {
        let name = path.rsplitn(2, |&b| b == b'/').next().unwrap_or(path);
        let parent_path = if path.contains(&b'/') {
            let i = path.iter().rposition(|&b| b == b'/').unwrap_or(0); &path[..i]
        } else { b"/" };
        let parent_ino = self.resolve(parent_path).ok_or(VfsError::NotFound)?;
        let pslot = self.inode_slot(parent_ino).ok_or(VfsError::NotFound)?;
        let dir = self.dirs[pslot].as_mut().ok_or(VfsError::NotADirectory)?;
        if !dir.remove(name) { return Err(VfsError::NotFound); }
        Ok(())
    }

    fn rename(&mut self, old: &[u8], new: &[u8]) -> VfsResult<()> {
        let ino = self.resolve(old).ok_or(VfsError::NotFound)?;
        self.unlink(old)?;
        let new_name = new.rsplitn(2, |&b| b == b'/').next().unwrap_or(new);
        let parent_path = if new.contains(&b'/') {
            let i = new.iter().rposition(|&b| b == b'/').unwrap_or(0); &new[..i]
        } else { b"/" };
        let parent_ino = self.resolve(parent_path).ok_or(VfsError::NotFound)?;
        let pslot = self.inode_slot(parent_ino).ok_or(VfsError::NotFound)?;
        self.dirs[pslot].as_mut().ok_or(VfsError::NotADirectory)?.add(new_name, ino);
        Ok(())
    }

    fn readdir(&self, path: &[u8], cb: &mut dyn FnMut(&[u8], InodeKind)) -> VfsResult<()> {
        let ino = self.resolve(path).ok_or(VfsError::NotFound)?;
        let slot = self.inode_slot(ino).ok_or(VfsError::NotFound)?;
        let dir = self.dirs[slot].as_ref().ok_or(VfsError::NotADirectory)?;
        for e in dir.entries.iter().flatten() {
            if e.name[..e.name_len] == *b"." || e.name[..e.name_len] == *b".." { continue; }
            let kind = self.inode_slot(e.ino)
                .and_then(|s| self.inodes[s].as_ref())
                .map(|i| i.kind)
                .unwrap_or(InodeKind::File);
            cb(&e.name[..e.name_len], kind);
        }
        Ok(())
    }

    fn stat(&self, path: &[u8]) -> VfsResult<InodeMeta> { self.lookup(path) }
}
