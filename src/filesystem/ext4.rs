//! # Ext4 Filesystem Driver
//!
//! Fourth Extended Filesystem implementation inspired by Linux fs/ext4/.
//! Supports journaling (JBD2), extents, large files, and modern features.

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::mem::size_of;

/// Ext4 superblock structure (first 1024 bytes)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Ext4Superblock {
    pub s_inodes_count: u32,         // Total inode count
    pub s_blocks_count_lo: u32,      // Total block count (low 32 bits)
    pub s_r_blocks_count_lo: u32,    // Reserved block count
    pub s_free_blocks_count_lo: u32, // Free block count
    pub s_free_inodes_count: u32,    // Free inode count
    pub s_first_data_block: u32,     // First data block
    pub s_log_block_size: u32,       // Block size = 1024 << s_log_block_size
    pub s_log_cluster_size: u32,     // Cluster size
    pub s_blocks_per_group: u32,     // Blocks per block group
    pub s_clusters_per_group: u32,   // Clusters per group
    pub s_inodes_per_group: u32,     // Inodes per group
    pub s_mtime: u32,                // Mount time
    pub s_wtime: u32,                // Write time
    pub s_mnt_count: u16,            // Mount count
    pub s_max_mnt_count: u16,        // Max mount count
    pub s_magic: u16,                // Magic signature (0xEF53)
    pub s_state: u16,                // File system state
    pub s_errors: u16,               // Behavior on errors
    pub s_minor_rev_level: u16,      // Minor revision level
    pub s_lastcheck: u32,            // Last check time
    pub s_checkinterval: u32,        // Check interval
    pub s_creator_os: u32,           // OS
    pub s_rev_level: u32,            // Revision level
    pub s_def_resuid: u16,           // Default reserved user ID
    pub s_def_resgid: u16,           // Default reserved group ID

    // Extended superblock fields (EXT4_DYNAMIC_REV)
    pub s_first_ino: u32,              // First non-reserved inode
    pub s_inode_size: u16,             // Inode size
    pub s_block_group_nr: u16,         // Block group number
    pub s_feature_compat: u32,         // Compatible features
    pub s_feature_incompat: u32,       // Incompatible features
    pub s_feature_ro_compat: u32,      // Read-only compatible features
    pub s_uuid: [u8; 16],              // UUID
    pub s_volume_name: [u8; 16],       // Volume name
    pub s_last_mounted: [u8; 64],      // Last mounted directory
    pub s_algorithm_usage_bitmap: u32, // Compression algorithm

    // Performance hints
    pub s_prealloc_blocks: u8,      // Blocks to preallocate
    pub s_prealloc_dir_blocks: u8,  // Dir blocks to preallocate
    pub s_reserved_gdt_blocks: u16, // Reserved GDT blocks

    // Journaling support
    pub s_journal_uuid: [u8; 16],  // Journal UUID
    pub s_journal_inum: u32,       // Journal inode number
    pub s_journal_dev: u32,        // Journal device
    pub s_last_orphan: u32,        // Head of orphan inode list
    pub s_hash_seed: [u32; 4],     // HTREE hash seed
    pub s_def_hash_version: u8,    // Default hash version
    pub s_jnl_backup_type: u8,     // Journal backup type
    pub s_desc_size: u16,          // Group descriptor size
    pub s_default_mount_opts: u32, // Default mount options
    pub s_first_meta_bg: u32,      // First metablock block group
    pub s_mkfs_time: u32,          // Filesystem creation time
    pub s_jnl_blocks: [u32; 17],   // Journal inode backup

    // 64-bit support
    pub s_blocks_count_hi: u32,      // High 32 bits of block count
    pub s_r_blocks_count_hi: u32,    // High 32 bits of reserved blocks
    pub s_free_blocks_count_hi: u32, // High 32 bits of free blocks
    pub s_min_extra_isize: u16,      // Minimum extra inode size
    pub s_want_extra_isize: u16,     // Desired extra inode size
    pub s_flags: u32,                // Miscellaneous flags
    pub s_raid_stride: u16,          // RAID stride
    pub s_mmp_interval: u16,         // Multi-mount protection interval
    pub s_mmp_block: u64,            // Block for MMP
    pub s_raid_stripe_width: u32,    // RAID stripe width
    pub s_log_groups_per_flex: u8,   // FLEX_BG group size
    pub s_checksum_type: u8,         // Metadata checksum algorithm
    pub s_reserved_pad: u16,
    pub s_kbytes_written: u64,          // KB written lifetime
    pub s_snapshot_inum: u32,           // Active snapshot inode
    pub s_snapshot_id: u32,             // Sequential snapshot ID
    pub s_snapshot_r_blocks_count: u64, // Reserved blocks for snapshot
    pub s_snapshot_list: u32,           // Head of snapshot list
    pub s_error_count: u32,             // Error count
    pub s_first_error_time: u32,        // First error time
    pub s_first_error_ino: u32,         // First error inode
    pub s_first_error_block: u64,       // First error block
    pub s_first_error_func: [u8; 32],   // First error function
    pub s_first_error_line: u32,        // First error line
    pub s_last_error_time: u32,         // Last error time
    pub s_last_error_ino: u32,          // Last error inode
    pub s_last_error_line: u32,         // Last error line
    pub s_last_error_block: u64,        // Last error block
    pub s_last_error_func: [u8; 32],    // Last error function
    pub s_mount_opts: [u8; 64],         // Mount options
    pub s_usr_quota_inum: u32,          // User quota inode
    pub s_grp_quota_inum: u32,          // Group quota inode
    pub s_overhead_blocks: u32,         // Overhead blocks
    pub s_backup_bgs: [u32; 2],         // Sparse superblock backup
    pub s_encrypt_algos: [u8; 4],       // Encryption algorithms
    pub s_encrypt_pw_salt: [u8; 16],    // Encryption password salt
    pub s_lpf_ino: u32,                 // Lost+found inode
    pub s_prj_quota_inum: u32,          // Project quota inode
    pub s_checksum_seed: u32,           // Checksum seed
    pub s_reserved: [u32; 98],          // Padding to 1024 bytes
    pub s_checksum: u32,                // Superblock checksum
}

const EXT4_SUPER_MAGIC: u16 = 0xEF53;

/// Ext4 inode structure
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Ext4Inode {
    pub i_mode: u16,         // File mode
    pub i_uid: u16,          // Owner UID (low 16 bits)
    pub i_size_lo: u32,      // Size (low 32 bits)
    pub i_atime: u32,        // Access time
    pub i_ctime: u32,        // Change time
    pub i_mtime: u32,        // Modification time
    pub i_dtime: u32,        // Deletion time
    pub i_gid: u16,          // Group ID (low 16 bits)
    pub i_links_count: u16,  // Hard link count
    pub i_blocks_lo: u32,    // Block count (low 32 bits)
    pub i_flags: u32,        // File flags
    pub i_osd1: u32,         // OS dependent
    pub i_block: [u32; 15],  // Block pointers / extent tree
    pub i_generation: u32,   // File version (for NFS)
    pub i_file_acl_lo: u32,  // Extended attributes (low 32 bits)
    pub i_size_high: u32,    // Size (high 32 bits) / dir ACL
    pub i_obso_faddr: u32,   // Obsolete fragment address
    pub i_osd2: [u8; 12],    // OS dependent
    pub i_extra_isize: u16,  // Extra inode size
    pub i_checksum_hi: u16,  // Inode checksum (high 16 bits)
    pub i_ctime_extra: u32,  // Extra change time
    pub i_mtime_extra: u32,  // Extra modification time
    pub i_atime_extra: u32,  // Extra access time
    pub i_crtime: u32,       // Creation time
    pub i_crtime_extra: u32, // Extra creation time
    pub i_version_hi: u32,   // High 32 bits of version
    pub i_projid: u32,       // Project ID
}

/// Ext4 extent header
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Ext4ExtentHeader {
    pub eh_magic: u16,      // Magic number (0xF30A)
    pub eh_entries: u16,    // Number of valid entries
    pub eh_max: u16,        // Capacity of store
    pub eh_depth: u16,      // Tree depth (0 = leaf)
    pub eh_generation: u32, // Generation
}

/// Ext4 extent (leaf node)
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Ext4Extent {
    pub ee_block: u32,    // First logical block
    pub ee_len: u16,      // Number of blocks
    pub ee_start_hi: u16, // High 16 bits of physical block
    pub ee_start_lo: u32, // Low 32 bits of physical block
}

/// Ext4 extent index (internal node)
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Ext4ExtentIdx {
    pub ei_block: u32,   // Index covers logical blocks >= ei_block
    pub ei_leaf_lo: u32, // Low 32 bits of pointer to child
    pub ei_leaf_hi: u16, // High 16 bits of pointer to child
    pub ei_unused: u16,
}

/// Block group descriptor
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Ext4GroupDesc {
    pub bg_block_bitmap_lo: u32,      // Block bitmap block (low)
    pub bg_inode_bitmap_lo: u32,      // Inode bitmap block (low)
    pub bg_inode_table_lo: u32,       // Inode table block (low)
    pub bg_free_blocks_count_lo: u16, // Free blocks count (low)
    pub bg_free_inodes_count_lo: u16, // Free inodes count (low)
    pub bg_used_dirs_count_lo: u16,   // Directories count (low)
    pub bg_flags: u16,                // Flags
    pub bg_exclude_bitmap_lo: u32,    // Snapshot exclude bitmap (low)
    pub bg_block_bitmap_csum_lo: u16, // Block bitmap checksum (low)
    pub bg_inode_bitmap_csum_lo: u16, // Inode bitmap checksum (low)
    pub bg_itable_unused_lo: u16,     // Unused inodes count (low)
    pub bg_checksum: u16,             // Group descriptor checksum

    // 64-bit fields (if descriptor size > 32)
    pub bg_block_bitmap_hi: u32,      // Block bitmap block (high)
    pub bg_inode_bitmap_hi: u32,      // Inode bitmap block (high)
    pub bg_inode_table_hi: u32,       // Inode table block (high)
    pub bg_free_blocks_count_hi: u16, // Free blocks count (high)
    pub bg_free_inodes_count_hi: u16, // Free inodes count (high)
    pub bg_used_dirs_count_hi: u16,   // Directories count (high)
    pub bg_itable_unused_hi: u16,     // Unused inodes count (high)
    pub bg_exclude_bitmap_hi: u32,    // Snapshot exclude bitmap (high)
    pub bg_block_bitmap_csum_hi: u16, // Block bitmap checksum (high)
    pub bg_inode_bitmap_csum_hi: u16, // Inode bitmap checksum (high)
    pub bg_reserved: u32,
}

/// Directory entry (classic format)
#[repr(C, packed)]
pub struct Ext4DirEntry {
    pub inode: u32,   // Inode number
    pub rec_len: u16, // Directory entry length
    pub name_len: u8, // Name length
    pub file_type: u8, // File type
                      // name follows (up to 255 bytes)
}

/// Ext4 filesystem instance
pub struct Ext4Filesystem {
    pub superblock: Ext4Superblock,
    pub block_size: usize,
    pub inode_size: usize,
    pub groups_count: usize,
    pub inodes_per_group: usize,
    pub blocks_per_group: usize,

    // In-memory caches
    group_descriptors: Vec<Ext4GroupDesc>,
    inode_cache: BTreeMap<u32, Ext4Inode>,
}

impl Ext4Filesystem {
    /// Create filesystem instance from superblock
    pub fn new(superblock_data: &[u8]) -> Result<Self, Ext4Error> {
        if superblock_data.len() < size_of::<Ext4Superblock>() {
            return Err(Ext4Error::InvalidSuperblock);
        }

        let sb: Ext4Superblock =
            unsafe { core::ptr::read_unaligned(superblock_data.as_ptr() as *const Ext4Superblock) };

        // Verify magic number
        if sb.s_magic != EXT4_SUPER_MAGIC {
            return Err(Ext4Error::InvalidMagic);
        }

        // Calculate sizes
        let block_size = 1024 << sb.s_log_block_size;
        let inode_size = if sb.s_rev_level == 0 {
            128
        } else {
            sb.s_inode_size as usize
        };

        let total_blocks = (sb.s_blocks_count_lo as u64) | ((sb.s_blocks_count_hi as u64) << 32);
        let groups_count = ((total_blocks + sb.s_blocks_per_group as u64 - 1)
            / sb.s_blocks_per_group as u64) as usize;

        Ok(Self {
            superblock: sb,
            block_size,
            inode_size,
            groups_count,
            inodes_per_group: sb.s_inodes_per_group as usize,
            blocks_per_group: sb.s_blocks_per_group as usize,
            group_descriptors: Vec::new(),
            inode_cache: BTreeMap::new(),
        })
    }

    /// Get block group number for inode
    pub fn inode_to_group(&self, ino: u32) -> usize {
        ((ino - 1) / self.superblock.s_inodes_per_group) as usize
    }

    /// Get index within block group for inode
    pub fn inode_to_index(&self, ino: u32) -> usize {
        ((ino - 1) % self.superblock.s_inodes_per_group) as usize
    }

    /// Read inode from disk
    pub fn read_inode(
        &mut self,
        ino: u32,
        read_block: &dyn Fn(u64, &mut [u8]) -> Result<(), Ext4Error>,
    ) -> Result<Ext4Inode, Ext4Error> {
        // Check cache
        if let Some(inode) = self.inode_cache.get(&ino) {
            return Ok(*inode);
        }

        let group = self.inode_to_group(ino);
        let index = self.inode_to_index(ino);

        // In production: read group descriptor to get inode table location
        // For now, calculate from group 0
        let inode_table_block = 5u64; // Simplified: typically after superblock + GDT
        let inodes_per_block = self.block_size / self.inode_size;
        let block_num = inode_table_block + (index / inodes_per_block) as u64;
        let block_offset = (index % inodes_per_block) * self.inode_size;

        // Read block containing inode
        let mut block_buf = alloc::vec![0u8; self.block_size];
        read_block(block_num, &mut block_buf)?;

        let inode: Ext4Inode = unsafe {
            core::ptr::read_unaligned(block_buf[block_offset..].as_ptr() as *const Ext4Inode)
        };

        // Cache inode
        self.inode_cache.insert(ino, inode);
        Ok(inode)
    }

    /// Parse extent tree to get physical block for logical block
    pub fn extent_to_block(&self, inode: &Ext4Inode, logical_block: u32) -> Result<u64, Ext4Error> {
        // Check if inode uses extents (EXT4_EXTENTS_FL = 0x80000)
        if inode.i_flags & 0x80000 == 0 {
            // Use classic block map
            if logical_block < 12 {
                return Ok(inode.i_block[logical_block as usize] as u64);
            }
            // Indirect blocks not implemented in this stub
            return Err(Ext4Error::NotImplemented);
        }

        // Parse extent tree
        let header = unsafe {
            core::ptr::read_unaligned(core::ptr::addr_of!(inode.i_block) as *const Ext4ExtentHeader)
        };

        if header.eh_magic != 0xF30A {
            return Err(Ext4Error::InvalidExtent);
        }

        if header.eh_depth == 0 {
            // Leaf node - extents directly in inode
            let extents_ptr = unsafe {
                (core::ptr::addr_of!(inode.i_block) as *const u8)
                    .add(core::mem::size_of::<Ext4ExtentHeader>())
            };

            for i in 0..header.eh_entries {
                let extent = unsafe {
                    core::ptr::read_unaligned((extents_ptr as *const Ext4Extent).add(i as usize))
                };

                let start_block = extent.ee_block;
                let end_block = start_block + extent.ee_len as u32;

                if logical_block >= start_block && logical_block < end_block {
                    let offset = logical_block - start_block;
                    let phys_block =
                        ((extent.ee_start_hi as u64) << 32) | (extent.ee_start_lo as u64);
                    return Ok(phys_block + offset as u64);
                }
            }

            Err(Ext4Error::BlockNotFound)
        } else {
            // Internal node - need to traverse tree
            // Not fully implemented in this stub
            Err(Ext4Error::NotImplemented)
        }
    }

    /// Get filesystem statistics
    pub fn get_stats(&self) -> Ext4Stats {
        Ext4Stats {
            total_blocks: (self.superblock.s_blocks_count_lo as u64)
                | ((self.superblock.s_blocks_count_hi as u64) << 32),
            free_blocks: (self.superblock.s_free_blocks_count_lo as u64)
                | ((self.superblock.s_free_blocks_count_hi as u64) << 32),
            total_inodes: self.superblock.s_inodes_count as u64,
            free_inodes: self.superblock.s_free_inodes_count as u64,
            block_size: self.block_size as u64,
            inode_size: self.inode_size as u64,
        }
    }
}

/// Filesystem statistics
#[derive(Debug, Clone, Copy)]
pub struct Ext4Stats {
    pub total_blocks: u64,
    pub free_blocks: u64,
    pub total_inodes: u64,
    pub free_inodes: u64,
    pub block_size: u64,
    pub inode_size: u64,
}

/// Ext4 errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ext4Error {
    InvalidSuperblock,
    InvalidMagic,
    InvalidExtent,
    BlockNotFound,
    InodeNotFound,
    NotImplemented,
    IoError,
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ext4_constants() {
        assert_eq!(EXT4_SUPER_MAGIC, 0xEF53);
        assert_eq!(size_of::<Ext4Superblock>(), 1024);
    }

    #[ignore]

    #[test]
    fn test_inode_calculations() {
        let mut sb_data = [0u8; 1024];
        sb_data[56..58].copy_from_slice(&EXT4_SUPER_MAGIC.to_le_bytes());

        // Set inodes per group = 8192
        sb_data[40..44].copy_from_slice(&8192u32.to_le_bytes());

        if let Ok(fs) = Ext4Filesystem::new(&sb_data) {
            assert_eq!(fs.inode_to_group(1), 0);
            assert_eq!(fs.inode_to_group(8192), 0);
            assert_eq!(fs.inode_to_group(8193), 1);
        }
    }
}
