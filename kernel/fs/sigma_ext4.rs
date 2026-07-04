// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// kernel/fs/sigma_ext4.rs — Ext4 Read-Only Driver (no_std, cleanroom)
// Language: Rust #![no_std]
// Pattern: OOP via Ext4Fs struct implementing FileSystem trait

#![no_std]
use crate::kernel::fs::sigma_vfs::{FileSystem,OpenFlags,InodeMeta,InodeKind,VfsResult,VfsError};

const EXT4_SUPER_MAGIC: u16 = 0xEF53;
const EXT4_ROOT_INO:    u32 = 2;
const BLOCK_SIZE:       usize = 4096;
const MAX_OPEN:         usize = 16;
const INODE_SIZE:       usize = 256;

type ReadBlockFn = fn(blk: u64, buf: &mut [u8; BLOCK_SIZE]) -> bool;

// ── Ext4 Superblock (key fields only) ────────────────────────────────────────
#[repr(C, packed)]
struct Superblock {
    s_inodes_count:      u32,
    s_blocks_count_lo:   u32,
    _pad1:               [u8; 4],
    s_first_data_block:  u32,
    s_log_block_size:    u32,
    _pad2:               [u8; 4],
    s_blocks_per_group:  u32,
    _pad3:               [u8; 4],
    s_inodes_per_group:  u32,
    _pad4:               [u8; 20],
    s_magic:             u16,
    _pad5:               [u8; 2],
    s_inode_size:        u16,
    _rest:               [u8; 196],
}

// ── Ext4 Group Descriptor (64-byte form) ──────────────────────────────────────
#[repr(C, packed)]
struct GroupDesc {
    bg_block_bitmap_lo:  u32,
    bg_inode_bitmap_lo:  u32,
    bg_inode_table_lo:   u32,
    bg_free_blocks_lo:   u16,
    bg_free_inodes_lo:   u16,
    bg_used_dirs_lo:     u16,
    _pad:                [u8; 2],
    _reserved1:          [u8; 4],
    bg_block_bitmap_hi:  u32,
    bg_inode_bitmap_hi:  u32,
    bg_inode_table_hi:   u32,
    _rest:               [u8; 28],
}

// ── Ext4 Inode ────────────────────────────────────────────────────────────────
#[repr(C, packed)]
struct Inode {
    i_mode:       u16,
    i_uid_lo:     u16,
    i_size_lo:    u32,
    i_atime:      u32,
    i_ctime:      u32,
    i_mtime:      u32,
    i_dtime:      u32,
    i_gid_lo:     u16,
    i_links_count: u16,
    i_blocks_lo:  u32,
    i_flags:      u32,
    _osd1:        u32,
    i_block:      [u32; 15], // direct(12) + indirect + dbl + tpl
    _rest:        [u8; 100],
}

impl Inode {
    fn file_size(&self) -> u64 { u32::from_le(self.i_size_lo) as u64 }
    fn is_dir(&self) -> bool { (u16::from_le(self.i_mode) & 0xF000) == 0x4000 }
    fn is_file(&self) -> bool { (u16::from_le(self.i_mode) & 0xF000) == 0x8000 }
}

// ── Ext4 Directory Entry ──────────────────────────────────────────────────────
#[repr(C, packed)]
struct DirEntry2 {
    inode:    u32,
    rec_len:  u16,
    name_len: u8,
    file_type: u8,
    // name follows immediately
}

// ── Open File Handle ──────────────────────────────────────────────────────────
struct OpenHandle { inode_no: u32, offset: u64, size: u64, handle: u64 }

// ── Ext4 Driver ───────────────────────────────────────────────────────────────
pub struct Ext4Fs {
    read_block:   ReadBlockFn,
    block_size:   usize,
    inodes_per_group: u32,
    blocks_per_group: u32,
    inode_size:   usize,
    groups_start: u64, // block number of first group descriptor table
    open:         [Option<OpenHandle>; MAX_OPEN],
    next_handle:  u64,
}

impl Ext4Fs {
    pub fn new(read: ReadBlockFn) -> Self {
        Self {
            read_block: read, block_size: BLOCK_SIZE,
            inodes_per_group: 0, blocks_per_group: 0,
            inode_size: INODE_SIZE, groups_start: 0,
            open: [const { None }; MAX_OPEN], next_handle: 1,
        }
    }

    fn read_inode(&self, ino: u32) -> Option<Inode> {
        if ino == 0 { return None; }
        let idx = (ino - 1) as u64;
        let grp = idx / self.inodes_per_group as u64;
        let off = idx % self.inodes_per_group as u64;
        // Read group descriptor
        let gdesc_blk = self.groups_start;
        let mut gd_buf = [0u8; BLOCK_SIZE];
        (self.read_block)(gdesc_blk, &mut gd_buf);
        let gd_off = (grp as usize) * 64;
        if gd_off + 64 > BLOCK_SIZE { return None; }
        let gd: &GroupDesc = unsafe { &*(gd_buf.as_ptr().add(gd_off) as *const GroupDesc) };
        let inode_table_blk = u32::from_le(gd.bg_inode_table_lo) as u64;
        // Read inode block
        let byte_off = off * self.inode_size as u64;
        let blk = inode_table_blk + byte_off / BLOCK_SIZE as u64;
        let off_in_blk = (byte_off % BLOCK_SIZE as u64) as usize;
        let mut buf = [0u8; BLOCK_SIZE];
        (self.read_block)(blk, &mut buf);
        if off_in_blk + INODE_SIZE > BLOCK_SIZE { return None; }
        let inode: Inode = unsafe { core::ptr::read_unaligned(buf.as_ptr().add(off_in_blk) as *const Inode) };
        Some(inode)
    }

    fn read_data_block(&self, inode: &Inode, block_idx: usize, buf: &mut [u8; BLOCK_SIZE]) -> bool {
        if block_idx < 12 {
            let blk = u32::from_le(inode.i_block[block_idx]) as u64;
            return (self.read_block)(blk, buf);
        }
        // Single indirect
        if block_idx < 12 + BLOCK_SIZE / 4 {
            let indirect_blk = u32::from_le(inode.i_block[12]) as u64;
            let mut ib = [0u8; BLOCK_SIZE];
            if !(self.read_block)(indirect_blk, &mut ib) { return false; }
            let ptr_idx = (block_idx - 12) * 4;
            let blk = u32::from_le_bytes(ib[ptr_idx..ptr_idx+4].try_into().unwrap_or([0;4])) as u64;
            return (self.read_block)(blk, buf);
        }
        false // double/triple indirect not implemented
    }

    fn lookup_in_dir(&self, dir_ino: u32, name: &[u8]) -> Option<u32> {
        let inode = self.read_inode(dir_ino)?;
        if !inode.is_dir() { return None; }
        let size = inode.file_size();
        let mut blk_idx = 0usize;
        let mut byte_off = 0u64;
        while byte_off < size {
            let mut buf = [0u8; BLOCK_SIZE];
            if !self.read_data_block(&inode, blk_idx, &mut buf) { break; }
            let mut pos = 0usize;
            while pos + 8 <= BLOCK_SIZE {
                let de: &DirEntry2 = unsafe { &*(buf.as_ptr().add(pos) as *const DirEntry2) };
                let rec_len = u16::from_le(de.rec_len) as usize;
                if rec_len == 0 { break; }
                let ino = u32::from_le(de.inode);
                if ino != 0 {
                    let nl = de.name_len as usize;
                    if nl == name.len() && &buf[pos+8..pos+8+nl] == name {
                        return Some(ino);
                    }
                }
                pos += rec_len;
            }
            blk_idx += 1;
            byte_off += BLOCK_SIZE as u64;
        }
        None
    }

    fn resolve_path(&self, path: &[u8]) -> Option<u32> {
        let path = if path.first() == Some(&b'/') { &path[1..] } else { path };
        let mut ino = EXT4_ROOT_INO;
        if path.is_empty() { return Some(ino); }
        let mut rem = path;
        loop {
            let (seg, rest) = match rem.iter().position(|&b| b == b'/') {
                Some(i) => (&rem[..i], &rem[i+1..]),
                None    => (rem, &[][..]),
            };
            ino = self.lookup_in_dir(ino, seg)?;
            if rest.is_empty() { return Some(ino); }
            rem = rest;
        }
    }
}

impl FileSystem for Ext4Fs {
    fn name(&self) -> &'static str { "ext4" }

    fn mount(&mut self, _device: usize) -> VfsResult<()> {
        let mut buf = [0u8; BLOCK_SIZE];
        // Superblock at byte offset 1024 = block 0 offset 1024
        (self.read_block)(0, &mut buf);
        let sb: &Superblock = unsafe { &*(buf.as_ptr().add(1024) as *const Superblock) };
        if u16::from_le(sb.s_magic) != EXT4_SUPER_MAGIC { return Err(VfsError::IoError); }
        self.inodes_per_group = u32::from_le(sb.s_inodes_per_group);
        self.blocks_per_group = u32::from_le(sb.s_blocks_per_group);
        self.inode_size       = u16::from_le(sb.s_inode_size) as usize;
        let log_bs            = u32::from_le(sb.s_log_block_size);
        self.block_size       = 1024 << log_bs;
        // Group descriptors start at block 1 (for 1KB blocks) or block 2 (for 4KB)
        self.groups_start     = if self.block_size == 1024 { 2 } else { 1 };
        Ok(())
    }

    fn umount(&mut self) -> VfsResult<()> { Ok(()) }

    fn lookup(&self, path: &[u8]) -> VfsResult<InodeMeta> {
        let ino = self.resolve_path(path).ok_or(VfsError::NotFound)?;
        let inode = self.read_inode(ino).ok_or(VfsError::IoError)?;
        Ok(InodeMeta {
            kind:   if inode.is_dir() { InodeKind::Directory } else { InodeKind::File },
            size:   inode.file_size(),
            inode:  ino as u64, nlinks: 1,
            uid:    u16::from_le(inode.i_uid_lo) as u32, gid: 0, mode: 0o644,
        })
    }

    fn open(&mut self, path: &[u8], _flags: OpenFlags) -> VfsResult<u64> {
        let ino = self.resolve_path(path).ok_or(VfsError::NotFound)?;
        let inode = self.read_inode(ino).ok_or(VfsError::IoError)?;
        for slot in &mut self.open {
            if slot.is_none() {
                let h = self.next_handle; self.next_handle += 1;
                *slot = Some(OpenHandle { inode_no: ino, offset: 0, size: inode.file_size(), handle: h });
                return Ok(h);
            }
        }
        Err(VfsError::TooManyOpen)
    }

    fn close(&mut self, handle: u64) -> VfsResult<()> {
        for slot in &mut self.open {
            if matches!(slot, Some(h) if h.handle == handle) { *slot = None; return Ok(()); }
        }
        Err(VfsError::BadFileDescriptor)
    }

    fn read(&mut self, handle: u64, buf: &mut [u8], _off: u64) -> VfsResult<usize> {
        let (ino, offset, size) = {
            let h = self.open.iter().flatten().find(|h| h.handle == handle)
                .ok_or(VfsError::BadFileDescriptor)?;
            (h.inode_no, h.offset, h.size)
        };
        let inode = self.read_inode(ino).ok_or(VfsError::IoError)?;
        let remaining = size.saturating_sub(offset) as usize;
        let to_read = buf.len().min(remaining);
        if to_read == 0 { return Ok(0); }
        let blk_idx = (offset as usize) / BLOCK_SIZE;
        let off_in_blk = (offset as usize) % BLOCK_SIZE;
        let mut blk_buf = [0u8; BLOCK_SIZE];
        self.read_data_block(&inode, blk_idx, &mut blk_buf);
        let n = to_read.min(BLOCK_SIZE - off_in_blk);
        buf[..n].copy_from_slice(&blk_buf[off_in_blk..off_in_blk+n]);
        for slot in &mut self.open {
            if let Some(h) = slot { if h.handle == handle { h.offset += n as u64; } }
        }
        Ok(n)
    }

    fn write  (&mut self, _h: u64, _b: &[u8], _o: u64) -> VfsResult<usize> { Err(VfsError::NotSupported) }
    fn mkdir  (&mut self, _p: &[u8]) -> VfsResult<()> { Err(VfsError::NotSupported) }
    fn unlink (&mut self, _p: &[u8]) -> VfsResult<()> { Err(VfsError::NotSupported) }
    fn rename (&mut self, _o: &[u8], _n: &[u8]) -> VfsResult<()> { Err(VfsError::NotSupported) }

    fn readdir(&self, path: &[u8], cb: &mut dyn FnMut(&[u8], InodeKind)) -> VfsResult<()> {
        let ino = self.resolve_path(path).ok_or(VfsError::NotFound)?;
        let inode = self.read_inode(ino).ok_or(VfsError::IoError)?;
        if !inode.is_dir() { return Err(VfsError::NotADirectory); }
        let size = inode.file_size();
        let mut blk_idx = 0usize; let mut byte_off = 0u64;
        while byte_off < size {
            let mut buf = [0u8; BLOCK_SIZE];
            if !self.read_data_block(&inode, blk_idx, &mut buf) { break; }
            let mut pos = 0usize;
            while pos + 8 <= BLOCK_SIZE {
                let de: &DirEntry2 = unsafe { &*(buf.as_ptr().add(pos) as *const DirEntry2) };
                let rec_len = u16::from_le(de.rec_len) as usize;
                if rec_len == 0 { break; }
                if u32::from_le(de.inode) != 0 {
                    let nl = de.name_len as usize;
                    let name = &buf[pos+8..pos+8+nl.min(BLOCK_SIZE-pos-8)];
                    let kind = if de.file_type == 2 { InodeKind::Directory } else { InodeKind::File };
                    cb(name, kind);
                }
                pos += rec_len;
            }
            blk_idx += 1; byte_off += BLOCK_SIZE as u64;
        }
        Ok(())
    }

    fn stat(&self, path: &[u8]) -> VfsResult<InodeMeta> { self.lookup(path) }
}
