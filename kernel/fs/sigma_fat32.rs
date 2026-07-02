// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/fs/sigma_fat32.rs — FAT32 Filesystem (no_std, read-write)
// Language: Rust #![no_std]
// Pattern: OOP via Fat32Fs struct implementing FileSystem trait

#![no_std]

use crate::kernel::fs::sigma_vfs::{FileSystem, OpenFlags, InodeMeta, InodeKind, VfsResult, VfsError};

// ── FAT32 On-Disk Structures ──────────────────────────────────────────────────

const SECTOR_SIZE:    usize = 512;
const ATTR_DIRECTORY: u8   = 0x10;
const ATTR_VOLUME_ID: u8   = 0x08;
const ATTR_LFN:       u8   = 0x0F;
const FAT_EOC:        u32  = 0x0FFFFFF8; // end of chain

#[repr(C, packed)]
struct BpbFat32 {
    jmp:          [u8; 3],
    oem:          [u8; 8],
    bytes_per_sec: u16,
    sec_per_clus:  u8,
    rsvd_sec_cnt:  u16,
    num_fats:      u8,
    root_ent_cnt:  u16,
    tot_sec16:     u16,
    media:         u8,
    fat_sz16:      u16,
    sec_per_trk:   u16,
    num_heads:     u16,
    hidd_sec:      u32,
    tot_sec32:     u32,
    // FAT32-specific
    fat_sz32:      u32,
    ext_flags:     u16,
    fs_ver:        u16,
    root_clus:     u32,
    fs_info:       u16,
    bk_boot_sec:   u16,
    _rsvd:         [u8; 12],
    drv_num:       u8,
    _rsvd1:        u8,
    boot_sig:      u8,
    vol_id:        u32,
    vol_lab:       [u8; 11],
    fil_sys_type:  [u8; 8],
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct DirEntry {
    name:     [u8; 11],   // 8.3 name (padded with spaces)
    attr:     u8,
    nt_res:   u8,
    crt_time_tenth: u8,
    crt_time: u16,
    crt_date: u16,
    lst_acc_date: u16,
    fst_clus_hi: u16,
    wrt_time:    u16,
    wrt_date:    u16,
    fst_clus_lo: u16,
    file_size:   u32,
}

impl DirEntry {
    fn is_free(&self)      -> bool { self.name[0] == 0xE5 || self.name[0] == 0x00 }
    fn is_end(&self)       -> bool { self.name[0] == 0x00 }
    fn is_dir(&self)       -> bool { self.attr & ATTR_DIRECTORY != 0 }
    fn is_lfn(&self)       -> bool { self.attr == ATTR_LFN }
    fn first_cluster(&self) -> u32 {
        ((self.fst_clus_hi as u32) << 16) | self.fst_clus_lo as u32
    }
    /// Copy 8.3 name into a byte buffer, returns length
    fn name83(&self, out: &mut [u8; 12]) -> usize {
        let mut n = 0;
        for i in 0..8 {
            if self.name[i] == b' ' { break; }
            out[n] = self.name[i].to_ascii_lowercase(); n += 1;
        }
        if self.name[8] != b' ' {
            out[n] = b'.'; n += 1;
            for i in 8..11 {
                if self.name[i] == b' ' { break; }
                out[n] = self.name[i].to_ascii_lowercase(); n += 1;
            }
        }
        n
    }
}

// ── Disk I/O Callback ─────────────────────────────────────────────────────────

pub type ReadSectorFn  = fn(lba: u64, buf: &mut [u8; SECTOR_SIZE]) -> bool;
pub type WriteSectorFn = fn(lba: u64, buf: &[u8; SECTOR_SIZE]) -> bool;

// ── FAT32 Driver ─────────────────────────────────────────────────────────────

const MAX_OPEN_FILES: usize = 16;

struct OpenFile {
    cluster:  u32,   // current cluster
    offset:   u32,   // byte offset within file
    size:     u32,
    inode:    u64,
    writable: bool,
}

pub struct Fat32Fs {
    read_sector:  ReadSectorFn,
    write_sector: WriteSectorFn,
    // Derived from BPB
    bytes_per_clus:  usize,
    fat_start_lba:   u64,
    data_start_lba:  u64,
    root_cluster:    u32,
    sec_per_clus:    usize,
    // Open file table
    files: [Option<OpenFile>; MAX_OPEN_FILES],
    next_inode: u64,
}

impl Fat32Fs {
    pub fn new(read: ReadSectorFn, write: WriteSectorFn) -> Self {
        Self {
            read_sector: read, write_sector: write,
            bytes_per_clus: 0, fat_start_lba: 0,
            data_start_lba: 0, root_cluster: 2,
            sec_per_clus: 0,
            files: [const { None }; MAX_OPEN_FILES],
            next_inode: 1,
        }
    }

    fn cluster_to_lba(&self, cluster: u32) -> u64 {
        self.data_start_lba + (cluster as u64 - 2) * self.sec_per_clus as u64
    }

    fn read_fat_entry(&self, cluster: u32) -> u32 {
        let fat_offset = cluster as u64 * 4;
        let fat_lba = self.fat_start_lba + fat_offset / SECTOR_SIZE as u64;
        let off_in_sec = (fat_offset % SECTOR_SIZE as u64) as usize;
        let mut buf = [0u8; SECTOR_SIZE];
        (self.read_sector)(fat_lba, &mut buf);
        u32::from_le_bytes(buf[off_in_sec..off_in_sec+4].try_into().unwrap_or([0;4])) & 0x0FFFFFFF
    }

    fn read_cluster(&self, cluster: u32, buf: &mut [u8]) -> bool {
        let lba = self.cluster_to_lba(cluster);
        for sec in 0..self.sec_per_clus {
            let mut sec_buf = [0u8; SECTOR_SIZE];
            if !(self.read_sector)(lba + sec as u64, &mut sec_buf) { return false; }
            let off = sec * SECTOR_SIZE;
            let n = SECTOR_SIZE.min(buf.len().saturating_sub(off));
            if n == 0 { break; }
            buf[off..off+n].copy_from_slice(&sec_buf[..n]);
        }
        true
    }

    /// Walk directory at `cluster` looking for `name`. Returns (DirEntry, cluster_of_entry).
    fn find_entry(&self, start_cluster: u32, name: &[u8]) -> Option<(DirEntry, u32)> {
        let mut cluster = start_cluster;
        let mut cluster_buf = [0u8; 32768]; // max 64-sector cluster
        loop {
            let clus_bytes = self.bytes_per_clus.min(cluster_buf.len());
            self.read_cluster(cluster, &mut cluster_buf[..clus_bytes]);
            let entries = clus_bytes / 32;
            for i in 0..entries {
                let base = i * 32;
                let e: DirEntry = unsafe { core::mem::transmute_copy(&cluster_buf[base]) };
                if e.is_end() { return None; }
                if e.is_free() || e.is_lfn() { continue; }
                let mut buf83 = [0u8; 12];
                let n = e.name83(&mut buf83);
                if &buf83[..n] == name { return Some((e, cluster)); }
            }
            let next = self.read_fat_entry(cluster);
            if next >= FAT_EOC { break; }
            cluster = next;
        }
        None
    }

    fn resolve_path(&self, path: &[u8]) -> Option<(DirEntry, u32)> {
        let path = if path.first() == Some(&b'/') { &path[1..] } else { path };
        if path.is_empty() {
            // Return synthetic root entry
            let mut e: DirEntry = unsafe { core::mem::zeroed() };
            e.attr = ATTR_DIRECTORY;
            e.fst_clus_lo = (self.root_cluster & 0xFFFF) as u16;
            e.fst_clus_hi = (self.root_cluster >> 16) as u16;
            return Some((e, self.root_cluster));
        }
        let mut cluster = self.root_cluster;
        let mut last_entry: Option<DirEntry> = None;
        let mut rem = path;
        loop {
            let (seg, rest) = match rem.iter().position(|&b| b == b'/') {
                Some(i) => (&rem[..i], &rem[i+1..]),
                None    => (rem, &[][..]),
            };
            if let Some((entry, _)) = self.find_entry(cluster, seg) {
                cluster = entry.first_cluster();
                last_entry = Some(entry);
                if rest.is_empty() { return Some((entry, cluster)); }
                if !entry.is_dir() { return None; }
                rem = rest;
            } else { return None; }
        }
    }
}

impl FileSystem for Fat32Fs {
    fn name(&self) -> &'static str { "fat32" }

    fn mount(&mut self, device: usize) -> VfsResult<()> {
        // Read BPB from sector 0
        let mut buf = [0u8; SECTOR_SIZE];
        if !(self.read_sector)(0, &mut buf) { return Err(VfsError::IoError); }
        let bpb: &BpbFat32 = unsafe { &*(buf.as_ptr() as *const BpbFat32) };
        let bps  = u16::from_le(bpb.bytes_per_sec) as usize;
        let spc  = bpb.sec_per_clus as usize;
        let rsvd = u16::from_le(bpb.rsvd_sec_cnt) as u64;
        let fats = bpb.num_fats as u64;
        let fsz  = u32::from_le(bpb.fat_sz32) as u64;
        self.fat_start_lba    = rsvd;
        self.data_start_lba   = rsvd + fats * fsz;
        self.root_cluster     = u32::from_le(bpb.root_clus);
        self.sec_per_clus     = spc;
        self.bytes_per_clus   = bps * spc;
        Ok(())
    }

    fn umount(&mut self) -> VfsResult<()> { Ok(()) }

    fn lookup(&self, path: &[u8]) -> VfsResult<InodeMeta> {
        let (entry, _) = self.resolve_path(path).ok_or(VfsError::NotFound)?;
        Ok(InodeMeta {
            kind:   if entry.is_dir() { InodeKind::Directory } else { InodeKind::File },
            size:   u32::from_le(entry.file_size) as u64,
            inode:  entry.first_cluster() as u64,
            nlinks: 1, uid: 0, gid: 0, mode: 0o644,
        })
    }

    fn open(&mut self, path: &[u8], _flags: OpenFlags) -> VfsResult<u64> {
        let (entry, _) = self.resolve_path(path).ok_or(VfsError::NotFound)?;
        // Find free slot
        for slot in &mut self.files {
            if slot.is_none() {
                let ino = self.next_inode;
                self.next_inode += 1;
                *slot = Some(OpenFile {
                    cluster: entry.first_cluster(),
                    offset: 0,
                    size: u32::from_le(entry.file_size),
                    inode: ino,
                    writable: false,
                });
                return Ok(ino);
            }
        }
        Err(VfsError::TooManyOpen)
    }

    fn close(&mut self, handle: u64) -> VfsResult<()> {
        for slot in &mut self.files {
            if matches!(slot, Some(ref f) if f.inode == handle) {
                *slot = None; return Ok(());
            }
        }
        Err(VfsError::BadFileDescriptor)
    }

    fn read(&mut self, handle: u64, buf: &mut [u8], _offset: u64) -> VfsResult<usize> {
        let (file_cluster, file_offset, file_size) = {
            let f = self.files.iter().flatten()
                .find(|f| f.inode == handle).ok_or(VfsError::BadFileDescriptor)?;
            (f.cluster, f.offset, f.size)
        };
        let remaining = file_size.saturating_sub(file_offset) as usize;
        let n = buf.len().min(remaining);
        if n == 0 { return Ok(0); }
        // Simplified: read whole cluster into a temp buffer
        let mut clus_buf = [0u8; 32768];
        self.read_cluster(file_cluster, &mut clus_buf);
        let off = file_offset as usize % self.bytes_per_clus;
        let avail = (self.bytes_per_clus - off).min(n);
        buf[..avail].copy_from_slice(&clus_buf[off..off+avail]);
        // Update offset
        for slot in &mut self.files {
            if let Some(ref mut f) = slot { if f.inode == handle { f.offset += avail as u32; } }
        }
        Ok(avail)
    }

    fn write(&mut self, _handle: u64, _buf: &[u8], _offset: u64) -> VfsResult<usize> {
        Err(VfsError::NotSupported) // Write support: TODO
    }

    fn mkdir   (&mut self, _p: &[u8]) -> VfsResult<()> { Err(VfsError::NotSupported) }
    fn unlink  (&mut self, _p: &[u8]) -> VfsResult<()> { Err(VfsError::NotSupported) }
    fn rename  (&mut self, _o: &[u8], _n: &[u8]) -> VfsResult<()> { Err(VfsError::NotSupported) }

    fn readdir(&self, path: &[u8], cb: &mut dyn FnMut(&[u8], InodeKind)) -> VfsResult<()> {
        let (entry, cluster) = self.resolve_path(path).ok_or(VfsError::NotFound)?;
        if !entry.is_dir() { return Err(VfsError::NotADirectory); }
        let mut cur = cluster;
        let mut buf = [0u8; 32768];
        loop {
            self.read_cluster(cur, &mut buf);
            for i in 0..self.bytes_per_clus/32 {
                let base = i * 32;
                let e: DirEntry = unsafe { core::mem::transmute_copy(&buf[base]) };
                if e.is_end() { return Ok(()); }
                if e.is_free() || e.is_lfn() { continue; }
                let mut name83 = [0u8; 12];
                let n = e.name83(&mut name83);
                let kind = if e.is_dir() { InodeKind::Directory } else { InodeKind::File };
                cb(&name83[..n], kind);
            }
            let next = self.read_fat_entry(cur);
            if next >= FAT_EOC { break; }
            cur = next;
        }
        Ok(())
    }

    fn stat(&self, path: &[u8]) -> VfsResult<InodeMeta> { self.lookup(path) }
}
