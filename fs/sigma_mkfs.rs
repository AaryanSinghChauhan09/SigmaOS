// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS SigmaFS mkfs + mount
//! On-disk layout: superblock + inode table + data bitmap + data blocks.
//! no_std, no alloc. All layout constants hand-defined.

#![no_std]
#![allow(dead_code)]

type SigmaU8   = u8;
type SigmaU16  = u16;
type SigmaU32  = u32;
type SigmaU64  = u64;
type SigmaI32  = i32;

pub const SIGMAFS_MAGIC:        SigmaU32 = 0x5349474D;  // "SIGM"
pub const SIGMAFS_BLOCK_SIZE:   usize    = 4096;
pub const SIGMAFS_MAX_INODES:   usize    = 256;
pub const SIGMAFS_MAX_BLOCKS:   usize    = 4096;
pub const SIGMAFS_DIRECT_PTRS:  usize    = 12;

/// On-disk superblock (fits in one 4096-byte block)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SigmaSuperblock {
    pub magic:         SigmaU32,
    pub version:       SigmaU16,
    pub block_size:    SigmaU32,
    pub total_blocks:  SigmaU32,
    pub total_inodes:  SigmaU32,
    pub free_blocks:   SigmaU32,
    pub free_inodes:   SigmaU32,
    pub inode_table_block: SigmaU32,
    pub data_bitmap_block: SigmaU32,
    pub first_data_block:  SigmaU32,
    pub checksum:      SigmaU32,
}

/// On-disk inode
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SigmaInode {
    pub ino:        SigmaU32,
    pub mode:       SigmaU16,   // permissions + type
    pub uid:        SigmaU16,
    pub gid:        SigmaU16,
    pub size:       SigmaU64,
    pub blocks:     SigmaU32,
    pub direct:     [SigmaU32; SIGMAFS_DIRECT_PTRS],
    pub indirect:   SigmaU32,
    pub created:    SigmaU64,
    pub modified:   SigmaU64,
    pub in_use:     bool,
}

/// In-memory mount state
static mut MOUNT_SB: SigmaSuperblock = SigmaSuperblock {
    magic: 0, version: 0, block_size: 0, total_blocks: 0, total_inodes: 0,
    free_blocks: 0, free_inodes: 0, inode_table_block: 0,
    data_bitmap_block: 0, first_data_block: 0, checksum: 0,
};
static mut INODE_TABLE: [SigmaInode; SIGMAFS_MAX_INODES] = [SigmaInode {
    ino: 0, mode: 0, uid: 0, gid: 0, size: 0, blocks: 0,
    direct: [0u32; SIGMAFS_DIRECT_PTRS], indirect: 0,
    created: 0, modified: 0, in_use: false,
}; SIGMAFS_MAX_INODES];
static mut DATA_BITMAP: [u8; SIGMAFS_MAX_BLOCKS / 8] = [0u8; SIGMAFS_MAX_BLOCKS / 8];
static mut MOUNTED: bool = false;

fn simple_checksum(sb: &SigmaSuperblock) -> SigmaU32 {
    let v: [SigmaU32; 9] = [
        sb.magic, sb.block_size, sb.total_blocks, sb.total_inodes,
        sb.free_blocks, sb.free_inodes, sb.inode_table_block,
        sb.data_bitmap_block, sb.first_data_block,
    ];
    let mut s: SigmaU32 = 0;
    for x in v { s = s.wrapping_add(x); }
    s
}

/// Format a new SigmaFS volume (writes in-memory structures).
#[no_mangle]
pub unsafe extern "C" fn sigma_mkfs(total_blocks: SigmaU32, total_inodes: SigmaU32) -> SigmaI32 {
    if total_blocks < 8 || total_inodes == 0 { return -1; }
    let inodes = total_inodes.min(SIGMAFS_MAX_INODES as SigmaU32);
    let blocks = total_blocks.min(SIGMAFS_MAX_BLOCKS as SigmaU32);

    MOUNT_SB = SigmaSuperblock {
        magic: SIGMAFS_MAGIC,
        version: 1,
        block_size: SIGMAFS_BLOCK_SIZE as SigmaU32,
        total_blocks: blocks,
        total_inodes: inodes,
        free_blocks: blocks - 4,   // reserve sb + inode table + bitmap + root dir
        free_inodes: inodes - 1,   // reserve inode 0 for root
        inode_table_block: 1,
        data_bitmap_block: 2,
        first_data_block:  3,
        checksum: 0,
    };
    MOUNT_SB.checksum = simple_checksum(&MOUNT_SB);

    // Clear inode table; create root inode
    for i in 0..SIGMAFS_MAX_INODES { INODE_TABLE[i].in_use = false; }
    INODE_TABLE[0] = SigmaInode {
        ino: 0, mode: 0o40755, uid: 0, gid: 0, size: 0, blocks: 0,
        direct: [0u32; SIGMAFS_DIRECT_PTRS], indirect: 0,
        created: 0, modified: 0, in_use: true,
    };

    // Clear data bitmap
    for i in 0..DATA_BITMAP.len() { DATA_BITMAP[i] = 0; }
    DATA_BITMAP[0] = 0x07;  // blocks 0-2 reserved (sb, inode table, bitmap)

    MOUNTED = true;
    0
}

/// Mount (validate superblock magic + checksum).
#[no_mangle]
pub unsafe extern "C" fn sigma_mount() -> SigmaI32 {
    if MOUNT_SB.magic != SIGMAFS_MAGIC { return -1; }
    if simple_checksum(&MOUNT_SB) != MOUNT_SB.checksum { return -1; }
    MOUNTED = true;
    0
}

/// Allocate a free inode. Returns inode number or u32::MAX on failure.
#[no_mangle]
pub unsafe extern "C" fn sigma_alloc_inode() -> SigmaU32 {
    if !MOUNTED { return SigmaU32::MAX; }
    for i in 1..SIGMAFS_MAX_INODES {
        if !INODE_TABLE[i].in_use {
            INODE_TABLE[i].in_use = true;
            INODE_TABLE[i].ino = i as SigmaU32;
            if MOUNT_SB.free_inodes > 0 { MOUNT_SB.free_inodes -= 1; }
            return i as SigmaU32;
        }
    }
    SigmaU32::MAX
}

/// Allocate a free data block. Returns block number or u32::MAX on failure.
#[no_mangle]
pub unsafe extern "C" fn sigma_alloc_block() -> SigmaU32 {
    if !MOUNTED { return SigmaU32::MAX; }
    for byte in 0..DATA_BITMAP.len() {
        if DATA_BITMAP[byte] != 0xFF {
            for bit in 0..8u8 {
                if DATA_BITMAP[byte] & (1 << bit) == 0 {
                    DATA_BITMAP[byte] |= 1 << bit;
                    if MOUNT_SB.free_blocks > 0 { MOUNT_SB.free_blocks -= 1; }
                    return (byte * 8 + bit as usize) as SigmaU32;
                }
            }
        }
    }
    SigmaU32::MAX
}

#[no_mangle]
pub unsafe extern "C" fn sigma_fs_free_blocks() -> SigmaU32 { MOUNT_SB.free_blocks }
#[no_mangle]
pub unsafe extern "C" fn sigma_fs_free_inodes() -> SigmaU32 { MOUNT_SB.free_inodes }
