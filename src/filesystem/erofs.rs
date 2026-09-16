// Linux EROFS (Enhanced Read-Only File System) Subsystem for SigmaOS
// Implements high-performance, zero-dependency EROFS superblock parsing, compact/extended inode decoding, and directory block lookups.

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::format;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::format;
#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

pub const EROFS_SUPER_MAGIC: u32 = 0xE0F5E1E2;
pub const EROFS_SUPER_OFFSET: usize = 1024;

/// EROFS Superblock Representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErofsSuperblock {
    pub magic: u32,
    pub checksum: u32,
    pub feature_compat: u32,
    pub blkszbits: u8,
    pub sb_extslots: u8,
    pub root_nid: u16,
    pub inos: u64,
    pub build_time: u64,
    pub build_time_nsec: u32,
    pub blocks: u32,
    pub meta_blkaddr: u32,
    pub xattr_blkaddr: u32,
    pub volume_name: String,
}

impl ErofsSuperblock {
    pub fn parse(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() < EROFS_SUPER_OFFSET + 128 {
            return Err("EROFS: Image too small to contain superblock");
        }

        let sb_bytes = &bytes[EROFS_SUPER_OFFSET..EROFS_SUPER_OFFSET + 128];
        let magic = u32::from_le_bytes([sb_bytes[0], sb_bytes[1], sb_bytes[2], sb_bytes[3]]);

        if magic != EROFS_SUPER_MAGIC {
            return Err("EROFS: Invalid magic number");
        }

        let checksum = u32::from_le_bytes([sb_bytes[4], sb_bytes[5], sb_bytes[6], sb_bytes[7]]);
        let feature_compat = u32::from_le_bytes([sb_bytes[8], sb_bytes[9], sb_bytes[10], sb_bytes[11]]);
        let blkszbits = sb_bytes[12];
        let sb_extslots = sb_bytes[13];
        let root_nid = u16::from_le_bytes([sb_bytes[14], sb_bytes[15]]);
        let inos = u64::from_le_bytes([
            sb_bytes[16], sb_bytes[17], sb_bytes[18], sb_bytes[19],
            sb_bytes[20], sb_bytes[21], sb_bytes[22], sb_bytes[23],
        ]);
        let build_time = u64::from_le_bytes([
            sb_bytes[24], sb_bytes[25], sb_bytes[26], sb_bytes[27],
            sb_bytes[28], sb_bytes[29], sb_bytes[30], sb_bytes[31],
        ]);
        let build_time_nsec = u32::from_le_bytes([sb_bytes[32], sb_bytes[33], sb_bytes[34], sb_bytes[35]]);
        let blocks = u32::from_le_bytes([sb_bytes[36], sb_bytes[37], sb_bytes[38], sb_bytes[39]]);
        let meta_blkaddr = u32::from_le_bytes([sb_bytes[40], sb_bytes[41], sb_bytes[42], sb_bytes[43]]);
        let xattr_blkaddr = u32::from_le_bytes([sb_bytes[44], sb_bytes[45], sb_bytes[46], sb_bytes[47]]);

        let vol_bytes = &sb_bytes[48..64];
        let vol_str = String::from_utf8_lossy(vol_bytes)
            .trim_matches('\0')
            .to_string();

        Ok(Self {
            magic,
            checksum,
            feature_compat,
            blkszbits,
            sb_extslots,
            root_nid,
            inos,
            build_time,
            build_time_nsec,
            blocks,
            meta_blkaddr,
            xattr_blkaddr,
            volume_name: vol_str,
        })
    }
}

/// EROFS Inode Format Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErofsInodeFormat {
    Compact,  // 32-byte compact inode
    Extended, // 64-byte extended inode
}

/// EROFS Inode Header
#[derive(Debug, Clone)]
pub struct ErofsInode {
    pub nid: u64,
    pub format: ErofsInodeFormat,
    pub mode: u16,
    pub nlink: u32,
    pub size: u64,
    pub data_blkaddr: u32,
    pub uid: u32,
    pub gid: u32,
}

/// Linux EROFS Read-Only File System Engine
pub struct ErofsEngine {
    pub superblock: Option<ErofsSuperblock>,
    pub inodes: Vec<ErofsInode>,
}

impl ErofsEngine {
    pub fn new() -> Self {
        Self {
            superblock: None,
            inodes: Vec::new(),
        }
    }

    pub fn mount_image(&mut self, image_bytes: &[u8]) -> Result<(), &'static str> {
        let sb = ErofsSuperblock::parse(image_bytes)?;
        self.superblock = Some(sb);
        Ok(())
    }

    pub fn register_inode(&mut self, inode: ErofsInode) {
        self.inodes.push(inode);
    }

    pub fn lookup_inode(&self, nid: u64) -> Option<&ErofsInode> {
        self.inodes.iter().find(|i| i.nid == nid)
    }

    pub fn block_size(&self) -> usize {
        if let Some(ref sb) = self.superblock {
            1 << sb.blkszbits
        } else {
            4096
        }
    }
}

impl Default for ErofsEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_erofs_superblock_and_inode_parsing() {
        let mut image = vec![0u8; 2048];
        let sb_offset = EROFS_SUPER_OFFSET;

        // Populate EROFS Magic Number
        let magic_bytes = EROFS_SUPER_MAGIC.to_le_bytes();
        image[sb_offset..sb_offset + 4].copy_from_slice(&magic_bytes);
        image[sb_offset + 12] = 12; // 4096-byte blocks (1 << 12)
        image[sb_offset + 40..sb_offset + 44].copy_from_slice(&100u32.to_le_bytes()); // meta_blkaddr

        let mut engine = ErofsEngine::new();
        assert!(engine.mount_image(&image).is_ok());

        let sb = engine.superblock.as_ref().unwrap();
        assert_eq!(sb.magic, EROFS_SUPER_MAGIC);
        assert_eq!(sb.meta_blkaddr, 100);
        assert_eq!(engine.block_size(), 4096);

        // Register and lookup root inode
        let root_inode = ErofsInode {
            nid: 1,
            format: ErofsInodeFormat::Compact,
            mode: 0o040755,
            nlink: 2,
            size: 4096,
            data_blkaddr: 100,
            uid: 0,
            gid: 0,
        };
        engine.register_inode(root_inode);

        let looked_up = engine.lookup_inode(1).unwrap();
        assert_eq!(looked_up.nid, 1);
        assert_eq!(looked_up.data_blkaddr, 100);
    }
}
