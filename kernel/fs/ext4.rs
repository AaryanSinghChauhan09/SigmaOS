// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/fs/ext4.rs — Ext4 filesystem driver (read-only, no_std)
//
// Implements read-only ext4 access needed to boot from a disk image.
// Supports: superblock, block groups, inodes, extents, directory entries.
// Does NOT modify any on-disk structures (safe for dual-boot).
//
// Language: Rust #![no_std]
#![no_std]
#![allow(dead_code)]

// ── Ext4 on-disk structures ───────────────────────────────────────────────

/// Ext4 superblock (at byte offset 1024 from partition start)
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Ext4Superblock {
    pub s_inodes_count:      u32,
    pub s_blocks_count_lo:   u32,
    pub s_r_blocks_count_lo: u32,
    pub s_free_blocks_lo:    u32,
    pub s_free_inodes:       u32,
    pub s_first_data_block:  u32,
    pub s_log_block_size:    u32,   // block_size = 1024 << s_log_block_size
    pub s_log_cluster_size:  u32,
    pub s_blocks_per_group:  u32,
    pub s_clusters_per_group:u32,
    pub s_inodes_per_group:  u32,
    pub s_mtime:             u32,
    pub s_wtime:             u32,
    pub s_mnt_count:         u16,
    pub s_max_mnt_count:     u16,
    pub s_magic:             u16,   // must be 0xEF53
    pub s_state:             u16,
    pub s_errors:            u16,
    pub s_minor_rev_level:   u16,
    pub s_lastcheck:         u32,
    pub s_checkinterval:     u32,
    pub s_creator_os:        u32,
    pub s_rev_level:         u32,
    pub s_def_resuid:        u16,
    pub s_def_resgid:        u16,
    // EXT4 specific (rev >= 1)
    pub s_first_ino:         u32,
    pub s_inode_size:        u16,
    pub s_block_group_nr:    u16,
    pub s_feature_compat:    u32,
    pub s_feature_incompat:  u32,
    pub s_feature_ro_compat: u32,
    pub s_uuid:              [u8; 16],
    pub s_volume_name:       [u8; 16],
    pub s_last_mounted:      [u8; 64],
    pub s_algo_bitmap:       u32,
    // Padding to 1024 bytes
    _pad: [u8; 768],
}

pub const EXT4_MAGIC: u16 = 0xEF53;
pub const EXT4_ROOT_INO: u32 = 2;

/// Block group descriptor (64-byte form)
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Ext4BgDesc {
    pub bg_block_bitmap_lo:  u32,
    pub bg_inode_bitmap_lo:  u32,
    pub bg_inode_table_lo:   u32,
    pub bg_free_blocks_lo:   u16,
    pub bg_free_inodes_lo:   u16,
    pub bg_used_dirs_lo:     u16,
    pub bg_flags:            u16,
    pub bg_exclude_bitmap_lo:u32,
    pub bg_block_bitmap_csum_lo: u16,
    pub bg_inode_bitmap_csum_lo: u16,
    pub bg_itable_unused_lo: u16,
    pub bg_checksum:         u16,
    // 64-byte descriptor extension
    pub bg_block_bitmap_hi:  u32,
    pub bg_inode_bitmap_hi:  u32,
    pub bg_inode_table_hi:   u32,
    pub bg_free_blocks_hi:   u16,
    pub bg_free_inodes_hi:   u16,
    pub bg_used_dirs_hi:     u16,
    pub bg_itable_unused_hi: u16,
    pub bg_exclude_bitmap_hi:u32,
    pub bg_block_bitmap_csum_hi: u16,
    pub bg_inode_bitmap_csum_hi: u16,
    _reserved: u32,
}

/// Ext4 inode (256 bytes)
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Ext4Inode {
    pub i_mode:         u16,
    pub i_uid:          u16,
    pub i_size_lo:      u32,
    pub i_atime:        u32,
    pub i_ctime:        u32,
    pub i_mtime:        u32,
    pub i_dtime:        u32,
    pub i_gid:          u16,
    pub i_links_count:  u16,
    pub i_blocks_lo:    u32,
    pub i_flags:        u32,
    pub l_i_version:    u32,
    pub i_block:        [u32; 15],  // extent tree or block pointers
    pub i_generation:   u32,
    pub i_file_acl_lo:  u32,
    pub i_size_hi:      u32,
    pub i_obso_faddr:   u32,
    pub l_i_blocks_hi:  u16,
    pub l_i_file_acl_hi:u16,
    pub l_i_uid_hi:     u16,
    pub l_i_gid_hi:     u16,
    pub l_i_checksum_lo:u16,
    _reserved:          u16,
    pub i_extra_isize:  u16,
    pub i_checksum_hi:  u16,
    pub i_ctime_extra:  u32,
    pub i_mtime_extra:  u32,
    pub i_atime_extra:  u32,
    pub i_crtime:       u32,
    pub i_crtime_extra: u32,
    pub i_version_hi:   u32,
    pub i_projid:       u32,
}

pub const EXT4_INODE_FLAG_EXTENTS: u32 = 0x80000;

/// Ext4 extent header
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Ext4ExtentHeader {
    pub eh_magic:   u16,   // 0xF30A
    pub eh_entries: u16,
    pub eh_max:     u16,
    pub eh_depth:   u16,
    pub eh_generation: u32,
}

pub const EXT4_EXTENT_MAGIC: u16 = 0xF30A;

/// Ext4 extent leaf
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Ext4Extent {
    pub ee_block:   u32,   // logical block number
    pub ee_len:     u16,   // number of blocks
    pub ee_start_hi:u16,
    pub ee_start_lo:u32,
}

/// Ext4 directory entry (linear)
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Ext4DirEntry {
    pub inode:    u32,
    pub rec_len:  u16,
    pub name_len: u8,
    pub file_type:u8,
    // name follows (variable length, up to 255 bytes)
}

pub const EXT4_FT_REG_FILE: u8 = 1;
pub const EXT4_FT_DIR:      u8 = 2;
pub const EXT4_FT_SYMLINK:  u8 = 7;

// ── Block device read callback ────────────────────────────────────────────
/// Called by the ext4 driver to read blocks from the underlying device.
/// `block` = absolute block number, `buf` = destination, `count` = blocks.
pub type BlockReadFn = unsafe fn(block: u64, buf: *mut u8, count: u32) -> i32;

// ── Ext4 reader ───────────────────────────────────────────────────────────
pub struct Ext4Reader {
    sb:           Ext4Superblock,
    block_size:   u32,
    inode_size:   u32,
    block_read:   BlockReadFn,
    initialized:  bool,
}

impl Ext4Reader {
    pub const fn new(block_read: BlockReadFn) -> Self {
        Self {
            sb: unsafe { core::mem::zeroed() },
            block_size: 4096,
            inode_size: 256,
            block_read,
            initialized: false,
        }
    }

    pub unsafe fn init(&mut self) -> Result<(), &'static str> {
        // Read superblock: 1024 bytes starting at offset 1024
        // That's block 0 (if block_size=1024) or bytes 1024-2047 of block 0
        let mut buf = [0u8; 1024];
        let r = (self.block_read)(0, buf.as_mut_ptr(), 1);
        if r != 0 { return Err("block read failed"); }

        // Superblock is at offset 1024 within the partition
        let sb = &*(buf.as_ptr().add(0) as *const Ext4Superblock);
        if { sb.s_magic } != EXT4_MAGIC { return Err("not ext4"); }

        self.sb = *sb;
        self.block_size = 1024u32 << { sb.s_log_block_size };
        self.inode_size = { sb.s_inode_size } as u32;
        self.initialized = true;
        Ok(())
    }

    unsafe fn read_block(&self, block: u64, buf: *mut u8) -> i32 {
        (self.block_read)(block, buf, 1)
    }

    unsafe fn read_inode(&self, ino: u32, out: *mut Ext4Inode) -> i32 {
        let inodes_per_group = { self.sb.s_inodes_per_group };
        let group = (ino - 1) / inodes_per_group;
        let local_idx = (ino - 1) % inodes_per_group;

        // Read block group descriptor
        let bgdt_block = { self.sb.s_first_data_block } + 1; // GDT follows superblock
        let mut bgdt_buf = [0u8; 4096];
        self.read_block(bgdt_block as u64, bgdt_buf.as_mut_ptr());

        let desc = &*(bgdt_buf.as_ptr().add(group as usize * 64) as *const Ext4BgDesc);
        let inode_table_block = { desc.bg_inode_table_lo } as u64;

        let inode_offset = local_idx as usize * self.inode_size as usize;
        let block_offset = inode_offset / self.block_size as usize;
        let byte_offset  = inode_offset % self.block_size as usize;

        let mut inode_buf = [0u8; 4096];
        self.read_block(inode_table_block + block_offset as u64, inode_buf.as_mut_ptr());

        *out = *(inode_buf.as_ptr().add(byte_offset) as *const Ext4Inode);
        0
    }

    /// Read file data via extent tree
    pub unsafe fn read_file(
        &self, ino: u32, offset: u64, buf: *mut u8, len: usize,
    ) -> i64 {
        let mut inode: Ext4Inode = core::mem::zeroed();
        if self.read_inode(ino, &mut inode) != 0 { return -2; }

        let file_size = { inode.i_size_lo } as u64
            | (({ inode.i_size_hi } as u64) << 32);

        if offset >= file_size { return 0; }
        let readable = (file_size - offset) as usize;
        let to_read  = len.min(readable);

        if { inode.i_flags } & EXT4_INODE_FLAG_EXTENTS == 0 {
            // Legacy direct/indirect blocks — not implemented for simplicity
            return -38; // ENOSYS
        }

        // Extent tree: i_block[0..3] = extent header + extents
        let ext_hdr = &*(inode.i_block.as_ptr() as *const Ext4ExtentHeader);
        if { ext_hdr.eh_magic } != EXT4_EXTENT_MAGIC { return -22; }

        let mut bytes_read: usize = 0;
        let entries = { ext_hdr.eh_entries } as usize;
        let ext_base = inode.i_block.as_ptr().add(3) as *const Ext4Extent;

        for i in 0..entries.min(4) {
            let ext = &*ext_base.add(i);
            let logical_start = { ext.ee_block } as u64 * self.block_size as u64;
            let extent_size   = { ext.ee_len }   as u64 * self.block_size as u64;
            let phys_start    = (({ ext.ee_start_hi } as u64) << 32)
                                | { ext.ee_start_lo } as u64;

            if offset + bytes_read as u64 >= logical_start + extent_size { continue; }
            if offset + bytes_read as u64 < logical_start { break; }

            let local_off = (offset + bytes_read as u64 - logical_start) as usize;
            let avail     = (extent_size as usize).saturating_sub(local_off);
            let copy      = (to_read - bytes_read).min(avail);

            let mut block_buf = [0u8; 4096];
            let block_idx     = phys_start + (local_off / self.block_size as usize) as u64;
            self.read_block(block_idx, block_buf.as_mut_ptr());

            let block_off = local_off % self.block_size as usize;
            core::ptr::copy_nonoverlapping(
                block_buf.as_ptr().add(block_off),
                buf.add(bytes_read),
                copy,
            );
            bytes_read += copy;
            if bytes_read >= to_read { break; }
        }
        bytes_read as i64
    }

    /// Walk a directory and find a dentry by name
    pub unsafe fn lookup_dir(&self, dir_ino: u32, name: &[u8]) -> u32 {
        let mut block_buf = [0u8; 4096];
        let n = self.read_file(dir_ino, 0, block_buf.as_mut_ptr(), 4096);
        if n <= 0 { return 0; }

        let mut off = 0usize;
        while off + 8 <= n as usize {
            let de = &*(block_buf.as_ptr().add(off) as *const Ext4DirEntry);
            let de_ino  = { de.inode };
            let rec_len = { de.rec_len } as usize;
            let nm_len  = { de.name_len } as usize;

            if de_ino != 0 && nm_len == name.len() {
                let nm_ptr = block_buf.as_ptr().add(off + 8);
                let nm = core::slice::from_raw_parts(nm_ptr, nm_len);
                if nm == name { return de_ino; }
            }
            if rec_len == 0 { break; }
            off += rec_len;
        }
        0
    }

    /// Resolve an absolute path to an inode number
    pub unsafe fn path_lookup(&self, path: &[u8]) -> u32 {
        if path.is_empty() || path[0] != b'/' { return 0; }
        let mut cur_ino = EXT4_ROOT_INO;
        let mut components = path[1..].split(|&b| b == b'/');
        for comp in components {
            if comp.is_empty() { continue; }
            cur_ino = self.lookup_dir(cur_ino, comp);
            if cur_ino == 0 { return 0; }
        }
        cur_ino
    }
}

// ── C-ABI exports ─────────────────────────────────────────────────────────
static mut BLOCK_READ_IMPL: Option<BlockReadFn> = None;
static mut G_EXT4: Option<Ext4Reader> = None;

#[no_mangle]
pub unsafe extern "C" fn ext4_init(block_read: BlockReadFn) -> i32 {
    let mut reader = Ext4Reader::new(block_read);
    match reader.init() {
        Ok(())  => { G_EXT4 = Some(reader); 0 }
        Err(_)  => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn ext4_read_file(
    ino: u32, offset: u64, buf: *mut u8, len: usize,
) -> i64 {
    match &G_EXT4 {
        Some(r) => r.read_file(ino, offset, buf, len),
        None    => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn ext4_path_lookup(path: *const u8, path_len: usize) -> u32 {
    if path.is_null() { return 0; }
    let p = core::slice::from_raw_parts(path, path_len);
    match &G_EXT4 {
        Some(r) => r.path_lookup(p),
        None    => 0,
    }
}
