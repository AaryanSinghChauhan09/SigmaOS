// SigmaOS SquashFS 4.0 Read-Only Compressed Filesystem Implementation
// Reference: Linux fs/squashfs, standard SquashFS 4.0 specification

#![allow(dead_code)]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub const SQUASHFS_MAGIC: u32 = 0x73717368; // "sqsh"
pub const SQUASHFS_MAJOR: u16 = 4;
pub const SQUASHFS_MINOR: u16 = 0;

/// SquashFS compression algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum SquashfsCompression {
    Gzip = 1,
    Lzma = 2,
    Lzo = 3,
    Xz = 4,
    Lz4 = 5,
    Zstd = 6,
    Unknown = 0xFFFF,
}

impl From<u16> for SquashfsCompression {
    fn from(val: u16) -> Self {
        match val {
            1 => SquashfsCompression::Gzip,
            2 => SquashfsCompression::Lzma,
            3 => SquashfsCompression::Lzo,
            4 => SquashfsCompression::Xz,
            5 => SquashfsCompression::Lz4,
            6 => SquashfsCompression::Zstd,
            _ => SquashfsCompression::Unknown,
        }
    }
}

/// SquashFS 4.0 Superblock layout (96 bytes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SquashfsSuperblock {
    pub magic: u32,
    pub inodes: u32,
    pub mkfs_time: u32,
    pub block_size: u32,
    pub fragments: u32,
    pub compression: SquashfsCompression,
    pub block_log: u16,
    pub flags: u16,
    pub id_count: u16,
    pub version_major: u16,
    pub version_minor: u16,
    pub root_inode_ref: u64,
    pub bytes_used: u64,
    pub id_table_start: u64,
    pub xattr_table_start: u64,
    pub inode_table_start: u64,
    pub directory_table_start: u64,
    pub fragment_table_start: u64,
    pub export_table_start: u64,
}

impl SquashfsSuperblock {
    pub fn parse(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() < 96 {
            return Err("SquashFS image too short for superblock");
        }

        let magic = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        if magic != SQUASHFS_MAGIC {
            return Err("Invalid SquashFS magic header (expected 'sqsh')");
        }

        let inodes = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
        let mkfs_time = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
        let block_size = u32::from_le_bytes(bytes[12..16].try_into().unwrap());
        let fragments = u32::from_le_bytes(bytes[16..20].try_into().unwrap());
        let compression_val = u16::from_le_bytes(bytes[20..22].try_into().unwrap());
        let block_log = u16::from_le_bytes(bytes[22..24].try_into().unwrap());
        let flags = u16::from_le_bytes(bytes[24..26].try_into().unwrap());
        let id_count = u16::from_le_bytes(bytes[26..28].try_into().unwrap());
        let version_major = u16::from_le_bytes(bytes[28..30].try_into().unwrap());
        let version_minor = u16::from_le_bytes(bytes[30..32].try_into().unwrap());
        let root_inode_ref = u64::from_le_bytes(bytes[32..40].try_into().unwrap());
        let bytes_used = u64::from_le_bytes(bytes[40..48].try_into().unwrap());
        let id_table_start = u64::from_le_bytes(bytes[48..56].try_into().unwrap());
        let xattr_table_start = u64::from_le_bytes(bytes[56..64].try_into().unwrap());
        let inode_table_start = u64::from_le_bytes(bytes[64..72].try_into().unwrap());
        let directory_table_start = u64::from_le_bytes(bytes[72..80].try_into().unwrap());
        let fragment_table_start = u64::from_le_bytes(bytes[80..88].try_into().unwrap());
        let export_table_start = u64::from_le_bytes(bytes[88..96].try_into().unwrap());

        if version_major != SQUASHFS_MAJOR || version_minor != SQUASHFS_MINOR {
            return Err("Unsupported SquashFS version (only 4.0 supported)");
        }

        Ok(Self {
            magic,
            inodes,
            mkfs_time,
            block_size,
            fragments,
            compression: SquashfsCompression::from(compression_val),
            block_log,
            flags,
            id_count,
            version_major,
            version_minor,
            root_inode_ref,
            bytes_used,
            id_table_start,
            xattr_table_start,
            inode_table_start,
            directory_table_start,
            fragment_table_start,
            export_table_start,
        })
    }
}

/// SquashFS Inode Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SquashfsInodeType {
    BasicDirectory = 1,
    BasicFile = 2,
    BasicSymlink = 3,
    BasicBlockDevice = 4,
    BasicCharDevice = 5,
    BasicFifo = 6,
    BasicSocket = 7,
    ExtendedDirectory = 8,
    ExtendedFile = 9,
    ExtendedSymlink = 10,
    ExtendedBlockDevice = 11,
    ExtendedCharDevice = 12,
    ExtendedFifo = 13,
    ExtendedSocket = 14,
}

/// Represents a parsed SquashFS directory entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SquashfsDirEntry {
    pub name: String,
    pub inode_number: u32,
    pub inode_type: u16,
    pub offset: u16,
}

/// SquashFS Fragment Entry
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SquashfsFragmentEntry {
    pub start_block: u64,
    pub size: u32,
    pub unused: u32,
}

/// SquashFS Reader & Mounting Engine
pub struct SquashfsReader {
    pub superblock: SquashfsSuperblock,
    pub raw_data: Vec<u8>,
}

impl SquashfsReader {
    pub fn new(data: &[u8]) -> Result<Self, &'static str> {
        let sb = SquashfsSuperblock::parse(data)?;
        Ok(Self {
            superblock: sb,
            raw_data: data.to_vec(),
        })
    }

    /// Check if the image has valid uncompressed data size
    pub fn is_valid(&self) -> bool {
        self.superblock.magic == SQUASHFS_MAGIC && self.raw_data.len() >= self.superblock.bytes_used as usize
    }

    /// Total number of inodes
    pub fn inode_count(&self) -> u32 {
        self.superblock.inodes
    }

    /// Block size in bytes
    pub fn block_size(&self) -> u32 {
        self.superblock.block_size
    }

    /// Compression format
    pub fn compression(&self) -> SquashfsCompression {
        self.superblock.compression
    }
}
