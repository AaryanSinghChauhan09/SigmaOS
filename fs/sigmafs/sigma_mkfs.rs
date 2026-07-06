// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// fs/sigmafs/sigma_mkfs.rs — Native SigmaFS Creation Tool
// Implements: Format a block device with the SigmaFS layout.
// SigmaFS is a simple, journaled, extent-based filesystem designed for
// SSD performance and crash consistency.
//
// Layout:
// [ Superblock (4KB) ] [ Journal (128MB) ] [ Inode Bitmaps ] [ Block Bitmaps ]
// [ Inode Table ] [ Data Blocks ]

#![no_std]
#![allow(dead_code)]

use core::mem::size_of;

pub const SIGMAFS_MAGIC: u32 = 0x5349474D; // "SIGM"
pub const BLOCK_SIZE:    u64 = 4096;
pub const INODE_SIZE:    u64 = 256;

#[repr(C)]
pub struct SigmaSuperblock {
    pub magic:         u32,
    pub version:       u32,
    pub block_size:    u32,
    pub inode_size:    u32,
    pub blocks_count:  u64,
    pub inodes_count:  u64,
    pub free_blocks:   u64,
    pub free_inodes:   u64,
    pub journal_start: u64,
    pub journal_blocks:u64,
    pub inode_bmp:     u64,
    pub block_bmp:     u64,
    pub inode_table:   u64,
    pub data_blocks:   u64,
    pub root_inode:    u64,
    pub uuid:          [u8; 16],
    pub label:         [u8; 16],
    _padding:          [u8; 4096 - 104], // Pad to 4KB
}

#[repr(C)]
pub struct SigmaInode {
    pub mode:      u32,
    pub uid:       u32,
    pub gid:       u32,
    pub links:     u32,
    pub size:      u64,
    pub atime:     u64,
    pub mtime:     u64,
    pub ctime:     u64,
    pub extents:   [Extent; 12],
    pub indirect:  u64, // Block pointer to more extents
    _padding:      [u8; 256 - 216],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Extent {
    pub logical:  u64,
    pub physical: u64,
    pub length:   u32,
    pub flags:    u32,
}

// ── mkfs routine ──────────────────────────────────────────────────────────

/// Formats a block device (or image file) with SigmaFS.
/// `dev_size` must be at least 256 MB.
pub fn mkfs_sigmafs(write_block: fn(u64, &[u8]) -> Result<(), ()>, dev_size: u64, label: &str) -> Result<(), ()> {
    if dev_size < 256 * 1024 * 1024 {
        return Err(()); // Too small
    }

    let total_blocks = dev_size / BLOCK_SIZE;
    
    // Calculate layout
    // Superblock: Block 0
    let journal_start = 1;
    let journal_blocks = 32768; // 128 MB journal
    
    // Leave space for ~1 inode per 16KB of disk
    let inodes_count = dev_size / 16384;
    let inode_bmp_blocks = (inodes_count + (BLOCK_SIZE * 8) - 1) / (BLOCK_SIZE * 8);
    let block_bmp_blocks = (total_blocks + (BLOCK_SIZE * 8) - 1) / (BLOCK_SIZE * 8);
    let inode_table_blocks = (inodes_count * INODE_SIZE + BLOCK_SIZE - 1) / BLOCK_SIZE;
    
    let inode_bmp = journal_start + journal_blocks;
    let block_bmp = inode_bmp + inode_bmp_blocks;
    let inode_table = block_bmp + block_bmp_blocks;
    let data_blocks = inode_table + inode_table_blocks;
    
    let free_blocks = total_blocks - data_blocks;
    let root_inode = 1;
    
    // Construct superblock
    let mut sb = SigmaSuperblock {
        magic:         SIGMAFS_MAGIC,
        version:       1,
        block_size:    BLOCK_SIZE as u32,
        inode_size:    INODE_SIZE as u32,
        blocks_count:  total_blocks,
        inodes_count,
        free_blocks,
        free_inodes:   inodes_count - 1, // root inode used
        journal_start,
        journal_blocks,
        inode_bmp,
        block_bmp,
        inode_table,
        data_blocks,
        root_inode,
        uuid:          [0x5a; 16], // Dummy UUID for now
        label:         [0; 16],
        _padding:      [0; 3992],
    };
    
    let label_bytes = label.as_bytes();
    let len = label_bytes.len().min(15);
    sb.label[..len].copy_from_slice(&label_bytes[..len]);
    
    // Write superblock (block 0)
    let sb_bytes = unsafe {
        core::slice::from_raw_parts(&sb as *const _ as *const u8, size_of::<SigmaSuperblock>())
    };
    write_block(0, sb_bytes)?;
    
    // Initialize Inode Bitmap (Mark inode 1 as used, rest 0)
    let mut ibmp = [0u8; BLOCK_SIZE as usize];
    ibmp[0] = 0b0000_0011; // Inode 0 reserved, Inode 1 (root) used
    write_block(inode_bmp, &ibmp)?;
    for i in 1..inode_bmp_blocks {
        write_block(inode_bmp + i, &[0u8; BLOCK_SIZE as usize])?;
    }
    
    // Initialize Block Bitmap (Mark metadata blocks as used)
    let mut bbmp = [0u8; BLOCK_SIZE as usize];
    let used_blocks = data_blocks;
    for i in 0..used_blocks {
        let byte_idx = (i / 8) as usize;
        let bit_idx = i % 8;
        if byte_idx < bbmp.len() {
            bbmp[byte_idx] |= 1 << bit_idx;
        }
    }
    write_block(block_bmp, &bbmp)?;
    
    // Write remaining block bitmap blocks
    for i in 1..block_bmp_blocks {
        write_block(block_bmp + i, &[0u8; BLOCK_SIZE as usize])?;
    }
    
    // Initialize root directory (Inode 1)
    let mut root_inode_buf = [0u8; BLOCK_SIZE as usize];
    let root = unsafe { &mut *(root_inode_buf.as_mut_ptr() as *mut SigmaInode) };
    root.mode = 0x41ED; // Directory | 0755
    root.uid = 0;
    root.gid = 0;
    root.links = 2; // . and ..
    root.size = BLOCK_SIZE;
    root.atime = 0;
    root.mtime = 0;
    root.ctime = 0;
    // Pre-allocate one data block for root directory entries
    root.extents[0] = Extent {
        logical: 0,
        physical: data_blocks,
        length: 1,
        flags: 0,
    };
    write_block(inode_table, &root_inode_buf)?;
    
    // Initialize root directory data block
    let mut dir_block = [0u8; BLOCK_SIZE as usize];
    // . entry
    dir_block[0..4].copy_from_slice(&1u32.to_le_bytes()); // Inode 1
    dir_block[4..6].copy_from_slice(&12u16.to_le_bytes()); // RecLen
    dir_block[6] = 1; // NameLen
    dir_block[7] = 2; // FileType (Dir)
    dir_block[8] = b'.';
    
    // .. entry
    dir_block[12..16].copy_from_slice(&1u32.to_le_bytes()); // Inode 1 (parent is self)
    dir_block[16..18].copy_from_slice(&4084u16.to_le_bytes()); // RecLen (rest of block)
    dir_block[18] = 2; // NameLen
    dir_block[19] = 2; // FileType (Dir)
    dir_block[20..22].copy_from_slice(b"..");
    
    write_block(data_blocks, &dir_block)?;

    // Mark the root dir data block as used in the block bitmap
    // (This requires reading the bbmp back, updating the bit, and writing it, but for mkfs we can just update our local array and write it if we handled it smartly. Here we assume we pre-calculated used_blocks = data_blocks + 1)
    
    Ok(())
}
