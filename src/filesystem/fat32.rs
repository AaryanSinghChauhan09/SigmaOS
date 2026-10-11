//! # FAT32 / exFAT Filesystem Driver
//!
//! Implements the FAT32 and exFAT filesystems used widely on removable media
//! and legacy BIOS bootable devices.
//!
//! ## FAT32 Overview
//! - Cluster-addressed allocation using a File Allocation Table (FAT)
//! - BIOS Parameter Block (BPB) in the Volume Boot Record (VBR)
//! - 8.3 short names + Long File Name (LFN) directory entries
//! - 32-bit cluster numbers (top 4 bits reserved)
//!
//! ## exFAT Overview
//! - Extended FAT for large volumes (> 32 GiB) and large files (> 4 GiB)
//! - OEM parameter tables and GUID partition records
//! - Stream extension and file name extension directory entries

#![allow(dead_code)]
#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Cluster value indicating the end of a cluster chain.
pub const FAT32_EOC: u32 = 0x0FFF_FFFF;
/// Cluster value marking a bad cluster in the FAT.
pub const FAT32_BAD_CLUSTER: u32 = 0x0FFF_FFF7;
/// The FAT32 signature bytes at offset 510/511 of the VBR.
pub const BOOT_SIGNATURE: u16 = 0xAA55;
/// exFAT OEM Name field value.
pub const EXFAT_OEM_NAME: &[u8; 8] = b"EXFAT   ";
/// Maximum number of characters in a single LFN entry.
pub const LFN_CHARS_PER_ENTRY: usize = 13;

// ---------------------------------------------------------------------------
// BIOS Parameter Block (FAT32)
// ---------------------------------------------------------------------------

/// FAT32 BIOS Parameter Block — occupies bytes 0–89 of the Volume Boot Record.
///
/// All multi-byte fields are **little-endian**.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Fat32Bpb {
    /// x86 jump instruction to the bootstrap code.
    pub jmp_boot: [u8; 3],
    /// OEM name, e.g. `b"MSDOS5.0"`.
    pub oem_name: [u8; 8],
    /// Bytes per logical sector (typically 512).
    pub bytes_per_sector: u16,
    /// Sectors per cluster (power of 2, 1–128).
    pub sectors_per_cluster: u8,
    /// Number of reserved sectors, including the VBR itself.
    pub reserved_sector_count: u16,
    /// Number of FAT copies (usually 2).
    pub num_fats: u8,
    /// Maximum root directory entry count — **must be 0** for FAT32.
    pub root_entry_count: u16,
    /// Total sectors (16-bit) — **must be 0** for FAT32; use `total_sectors_32`.
    pub total_sectors_16: u16,
    /// Media type descriptor byte.
    pub media_type: u8,
    /// Sectors per FAT (16-bit) — **must be 0** for FAT32; use `fat_size_32`.
    pub fat_size_16: u16,
    /// Sectors per track for CHS geometry.
    pub sectors_per_track: u16,
    /// Number of heads for CHS geometry.
    pub num_heads: u16,
    /// Count of hidden sectors before the partition.
    pub hidden_sectors: u32,
    /// Total sectors on the volume (32-bit).
    pub total_sectors_32: u32,
    // --- FAT32-specific fields ---
    /// Sectors per FAT table.
    pub fat_size_32: u32,
    /// FAT flags (bit 7 = mirroring disabled; bits 3:0 = active FAT).
    pub ext_flags: u16,
    /// Filesystem version (`0x0000` = FAT32 version 0.0).
    pub fs_version: u16,
    /// Cluster number of the root directory's first cluster.
    pub root_cluster: u32,
    /// Sector number of `FSINFO` structure (usually 1).
    pub fs_info: u16,
    /// Sector number of the backup VBR (usually 6).
    pub backup_boot_sector: u16,
    /// Reserved; must be zero.
    pub reserved: [u8; 12],
    /// BIOS drive number.
    pub drive_number: u8,
    /// Reserved / NT flags.
    pub reserved1: u8,
    /// Extended boot signature (`0x29`).
    pub boot_signature: u8,
    /// Volume serial number.
    pub volume_id: u32,
    /// Volume label (padded with spaces).
    pub volume_label: [u8; 11],
    /// Filesystem type string (`b"FAT32   "`).
    pub fs_type: [u8; 8],
}

impl Fat32Bpb {
    /// Parse a FAT32 BPB from a raw 512-byte sector buffer.
    ///
    /// Returns `None` if the boot signature is invalid or the structure is not
    /// a valid FAT32 volume.
    pub fn from_bytes(buf: &[u8; 512]) -> Option<Self> {
        // Verify boot signature at bytes 510–511.
        let sig = u16::from_le_bytes([buf[510], buf[511]]);
        if sig != BOOT_SIGNATURE {
            return None;
        }
        // SAFETY: The buffer is the correct size and we verified the signature.
        let bpb = unsafe { core::ptr::read_unaligned(buf.as_ptr() as *const Fat32Bpb) };
        // FAT32 must have root_entry_count == 0 and fat_size_16 == 0.
        if bpb.root_entry_count != 0 || bpb.fat_size_16 != 0 {
            return None;
        }
        Some(bpb)
    }

    /// Returns the first sector of the data region.
    pub fn first_data_sector(&self) -> u32 {
        let fat_sectors = self.num_fats as u32 * self.fat_size_32;
        self.reserved_sector_count as u32 + fat_sectors
    }

    /// Convert a cluster number to its first logical sector.
    pub fn cluster_to_sector(&self, cluster: u32) -> u32 {
        self.first_data_sector() + (cluster - 2) * self.sectors_per_cluster as u32
    }

    /// Compute the byte offset of a FAT entry for `cluster`.
    pub fn fat_offset(&self, cluster: u32) -> u32 {
        self.reserved_sector_count as u32 * self.bytes_per_sector as u32 + cluster * 4
    }
}

// ---------------------------------------------------------------------------
// FSINFO Structure
// ---------------------------------------------------------------------------

/// FAT32 FSINFO sector — provides a hint for the next free cluster.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Fat32FsInfo {
    /// Lead signature (`0x41615252`).
    pub lead_sig: u32,
    /// Reserved (480 bytes).
    pub reserved1: [u8; 480],
    /// Structure signature (`0x61417272`).
    pub struc_sig: u32,
    /// Last known count of free clusters, or `0xFFFFFFFF` if unknown.
    pub free_count: u32,
    /// Hint for next free cluster, or `0xFFFFFFFF` if unknown.
    pub nxt_free: u32,
    /// Reserved (12 bytes).
    pub reserved2: [u8; 12],
    /// Trail signature (`0xAA550000`).
    pub trail_sig: u32,
}

impl Fat32FsInfo {
    /// Lead signature constant.
    pub const LEAD_SIG: u32 = 0x4161_5252;
    /// Structure signature constant.
    pub const STRUC_SIG: u32 = 0x6141_7272;
    /// Trail signature constant.
    pub const TRAIL_SIG: u32 = 0xAA55_0000;

    /// Validate the FSINFO signatures.
    pub fn is_valid(&self) -> bool {
        self.lead_sig == Self::LEAD_SIG
            && self.struc_sig == Self::STRUC_SIG
            && self.trail_sig == Self::TRAIL_SIG
    }
}

// ---------------------------------------------------------------------------
// Directory Entries: 8.3 Short Names
// ---------------------------------------------------------------------------

/// FAT32 short (8.3) directory entry — exactly 32 bytes.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Fat32DirEntry {
    /// Short filename: 8 chars name + 3 chars extension. `0xE5` = deleted.
    pub name: [u8; 11],
    /// File attributes bitmask.
    pub attr: u8,
    /// Reserved (Windows NT attribute).
    pub nt_reserved: u8,
    /// Tenth of a second creation time (0–199).
    pub crt_time_tenth: u8,
    /// Creation time (FAT packed format).
    pub crt_time: u16,
    /// Creation date.
    pub crt_date: u16,
    /// Last access date.
    pub lst_acc_date: u16,
    /// High 16 bits of first cluster number.
    pub fst_clus_hi: u16,
    /// Last write time.
    pub wrt_time: u16,
    /// Last write date.
    pub wrt_date: u16,
    /// Low 16 bits of first cluster number.
    pub fst_clus_lo: u16,
    /// File size in bytes (0 for directories).
    pub file_size: u32,
}

/// FAT directory entry attribute flags.
pub mod attr {
    pub const READ_ONLY: u8 = 0x01;
    pub const HIDDEN: u8 = 0x02;
    pub const SYSTEM: u8 = 0x04;
    pub const VOLUME_ID: u8 = 0x08;
    pub const DIRECTORY: u8 = 0x10;
    pub const ARCHIVE: u8 = 0x20;
    /// Combination that marks a Long File Name entry.
    pub const LFN: u8 = READ_ONLY | HIDDEN | SYSTEM | VOLUME_ID;
}

impl Fat32DirEntry {
    /// Returns `true` if this entry is the start of a LFN sequence.
    pub fn is_lfn(&self) -> bool {
        self.attr == attr::LFN
    }

    /// Returns `true` if this entry is a directory.
    pub fn is_dir(&self) -> bool {
        self.attr & attr::DIRECTORY != 0
    }

    /// Returns `true` if the entry is free (never used).
    pub fn is_free(&self) -> bool {
        self.name[0] == 0x00
    }

    /// Returns `true` if the entry is deleted.
    pub fn is_deleted(&self) -> bool {
        self.name[0] == 0xE5
    }

    /// Reconstruct the first cluster number.
    pub fn first_cluster(&self) -> u32 {
        (self.fst_clus_hi as u32) << 16 | self.fst_clus_lo as u32
    }

    /// Decode the 8.3 short name to an ASCII string.
    pub fn short_name(&self) -> String {
        let base = core::str::from_utf8(&self.name[..8])
            .unwrap_or("????????")
            .trim_end();
        let ext = core::str::from_utf8(&self.name[8..11])
            .unwrap_or("???")
            .trim_end();
        if ext.is_empty() {
            String::from(base)
        } else {
            let mut s = String::from(base);
            s.push('.');
            s.push_str(ext);
            s
        }
    }
}

// ---------------------------------------------------------------------------
// Long File Name (LFN) Directory Entries
// ---------------------------------------------------------------------------

/// FAT32 Long File Name directory entry — shares the 32-byte layout.
///
/// LFN entries are stored in **reverse order** immediately preceding the
/// corresponding 8.3 entry.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Fat32LfnEntry {
    /// Sequence number (1-based). Bit 6 (`0x40`) set on the last entry in
    /// the sequence (which appears *first* on disk).
    pub order: u8,
    /// First 5 UTF-16LE characters.
    pub name1: [u16; 5],
    /// Must be `attr::LFN` (0x0F).
    pub attr: u8,
    /// Entry type (always 0 for LFN).
    pub entry_type: u8,
    /// Checksum of the corresponding 8.3 short name.
    pub checksum: u8,
    /// Next 6 UTF-16LE characters.
    pub name2: [u16; 6],
    /// Must be 0x0000.
    pub first_cluster_lo: u16,
    /// Final 2 UTF-16LE characters.
    pub name3: [u16; 2],
}

impl Fat32LfnEntry {
    /// Extract all up-to-13 UTF-16LE code units from this LFN entry.
    pub fn name_chars(&self) -> [u16; 13] {
        let mut chars = [0u16; 13];
        let n1 = self.name1;
        let n2 = self.name2;
        let n3 = self.name3;
        chars[..5].copy_from_slice(&n1);
        chars[5..11].copy_from_slice(&n2);
        chars[11..13].copy_from_slice(&n3);
        chars
    }

    /// Compute the 8.3 short name checksum that an LFN entry should carry.
    pub fn compute_checksum(short_name: &[u8; 11]) -> u8 {
        let mut sum: u8 = 0;
        for &b in short_name {
            sum = (sum >> 1).wrapping_add(sum << 7).wrapping_add(b);
        }
        sum
    }
}

// ---------------------------------------------------------------------------
// FAT Table Management
// ---------------------------------------------------------------------------

/// In-memory representation of a FAT32 File Allocation Table.
pub struct Fat32Table {
    /// Raw FAT entries indexed by cluster number.
    entries: Vec<u32>,
}

impl Fat32Table {
    /// Load a FAT table from a raw byte slice.
    pub fn from_bytes(data: &[u8]) -> Self {
        let mut entries = Vec::with_capacity(data.len() / 4);
        for chunk in data.chunks_exact(4) {
            entries.push(u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]) & 0x0FFF_FFFF);
        }
        Fat32Table { entries }
    }

    /// Read the FAT entry for `cluster`.
    pub fn get(&self, cluster: u32) -> Option<u32> {
        self.entries.get(cluster as usize).copied()
    }

    /// Set the FAT entry for `cluster`.
    pub fn set(&mut self, cluster: u32, value: u32) {
        if let Some(e) = self.entries.get_mut(cluster as usize) {
            *e = value & 0x0FFF_FFFF;
        }
    }

    /// Walk a cluster chain starting at `start`, collecting all clusters.
    ///
    /// Returns an error if the chain exceeds `max_clusters` (loop guard).
    pub fn chain(&self, start: u32, max_clusters: usize) -> Result<Vec<u32>, Fat32Error> {
        let mut chain = Vec::new();
        let mut current = start;
        while current < FAT32_BAD_CLUSTER {
            if chain.len() >= max_clusters {
                return Err(Fat32Error::ChainTooLong);
            }
            chain.push(current);
            current = self.get(current).ok_or(Fat32Error::InvalidCluster(current))?;
        }
        Ok(chain)
    }

    /// Allocate a free cluster, marking it as end-of-chain.
    pub fn alloc(&mut self) -> Option<u32> {
        for (i, &entry) in self.entries.iter().enumerate().skip(2) {
            if entry == 0 {
                self.entries[i] = FAT32_EOC;
                return Some(i as u32);
            }
        }
        None
    }

    /// Free the entire cluster chain starting at `start`.
    pub fn free_chain(&mut self, start: u32) {
        let mut current = start;
        while current < FAT32_BAD_CLUSTER {
            let next = match self.get(current) {
                Some(n) => n,
                None => break,
            };
            self.set(current, 0);
            current = next;
        }
    }
}

// ---------------------------------------------------------------------------
// Error Type
// ---------------------------------------------------------------------------

/// Errors produced by the FAT32 / exFAT driver.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Fat32Error {
    /// The boot sector signature (`0xAA55`) is missing.
    InvalidSignature,
    /// The volume is not a FAT32 filesystem.
    NotFat32,
    /// The cluster number is out of range.
    InvalidCluster(u32),
    /// The cluster chain is longer than the configured maximum.
    ChainTooLong,
    /// A directory entry could not be decoded.
    InvalidDirEntry,
    /// An I/O error occurred reading or writing the block device.
    IoError,
    /// The LFN checksum does not match the 8.3 entry.
    LfnChecksumMismatch,
}

impl fmt::Display for Fat32Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fat32Error::InvalidSignature => write!(f, "invalid boot signature"),
            Fat32Error::NotFat32 => write!(f, "not a FAT32 volume"),
            Fat32Error::InvalidCluster(c) => write!(f, "invalid cluster {}", c),
            Fat32Error::ChainTooLong => write!(f, "cluster chain too long"),
            Fat32Error::InvalidDirEntry => write!(f, "invalid directory entry"),
            Fat32Error::IoError => write!(f, "I/O error"),
            Fat32Error::LfnChecksumMismatch => write!(f, "LFN checksum mismatch"),
        }
    }
}

// ---------------------------------------------------------------------------
// exFAT Structures
// ---------------------------------------------------------------------------

/// exFAT Boot Sector — begins with the OEM name `b"EXFAT   "`.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct ExFatBootSector {
    /// x86 jump + NOP instruction.
    pub jump_boot: [u8; 3],
    /// Must be `b"EXFAT   "`.
    pub oem_name: [u8; 8],
    /// Must be all zeros.
    pub must_be_zero: [u8; 53],
    /// Partition offset in sectors.
    pub partition_offset: u64,
    /// Volume length in sectors.
    pub volume_length: u64,
    /// Sector offset of the FAT.
    pub fat_offset: u32,
    /// Length of the FAT in sectors.
    pub fat_length: u32,
    /// Sector offset of the cluster heap.
    pub cluster_heap_offset: u32,
    /// Total cluster count.
    pub cluster_count: u32,
    /// First cluster of the root directory.
    pub first_cluster_of_root_directory: u32,
    /// Volume serial number.
    pub volume_serial_number: u32,
    /// Filesystem revision (`0x0100` = 1.0).
    pub filesystem_revision: u16,
    /// Volume flags.
    pub volume_flags: u16,
    /// Log₂(bytes per sector) — valid range 9–12.
    pub bytes_per_sector_shift: u8,
    /// Log₂(sectors per cluster) — valid range 0–25.
    pub sectors_per_cluster_shift: u8,
    /// Number of FAT copies (1 or 2).
    pub number_of_fats: u8,
    /// Drive select for extended INT 13h.
    pub drive_select: u8,
    /// Percentage of cluster heap in use.
    pub percent_in_use: u8,
    /// Reserved.
    pub reserved: [u8; 7],
}

impl ExFatBootSector {
    /// Try to parse an exFAT boot sector from raw 512 bytes.
    pub fn from_bytes(buf: &[u8; 512]) -> Option<Self> {
        let sector: Self = unsafe { core::ptr::read_unaligned(buf.as_ptr() as *const Self) };
        if sector.is_exfat() {
            Some(sector)
        } else {
            None
        }
    }

    /// Validate the OEM name field.
    pub fn is_exfat(&self) -> bool {
        &self.oem_name == EXFAT_OEM_NAME
    }

    /// Bytes per sector derived from the shift field.
    pub fn bytes_per_sector(&self) -> u32 {
        1 << self.bytes_per_sector_shift
    }

    /// Bytes per cluster.
    pub fn bytes_per_cluster(&self) -> u32 {
        self.bytes_per_sector() << self.sectors_per_cluster_shift
    }
}

/// exFAT OEM Parameters GUID entry (one of up to 10 entries in the OEM table).
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct ExFatOemParam {
    /// GUID identifying this parameter set.
    pub guid: [u8; 16],
    /// Parameter data (48 bytes, layout depends on GUID).
    pub custom_defined: [u8; 48],
}

// ---------------------------------------------------------------------------
// High-Level FAT32 Filesystem
// ---------------------------------------------------------------------------

/// Represents a mounted FAT32 or exFAT volume.
pub struct Fat32Volume {
    /// Parsed BIOS Parameter Block.
    pub bpb: Fat32Bpb,
    /// In-memory copy of the primary FAT.
    pub fat: Fat32Table,
    /// Cached FSINFO data.
    pub fsinfo: Fat32FsInfo,
    /// Whether this volume is mounted read-only.
    pub read_only: bool,
}

impl Fat32Volume {
    /// Mount a FAT32 volume from raw sector data.
    ///
    /// # Arguments
    /// * `vbr` - 512-byte Volume Boot Record.
    /// * `fat_data` - Complete FAT sector data.
    /// * `fsinfo_sector` - 512-byte FSINFO sector.
    pub fn mount(vbr: &[u8; 512], fat_data: &[u8], fsinfo_sector: &[u8; 512]) -> Result<Self, Fat32Error> {
        let bpb = Fat32Bpb::from_bytes(vbr).ok_or(Fat32Error::InvalidSignature)?;
        let fat = Fat32Table::from_bytes(fat_data);
        // SAFETY: fsinfo_sector is the correct size.
        let fsinfo: Fat32FsInfo =
            unsafe { core::ptr::read_unaligned(fsinfo_sector.as_ptr() as *const Fat32FsInfo) };
        Ok(Fat32Volume { bpb, fat, fsinfo, read_only: false })
    }

    /// Read a file's cluster chain given its starting cluster.
    pub fn read_chain(&self, start: u32) -> Result<Vec<u32>, Fat32Error> {
        self.fat.chain(start, 1 << 24)
    }

    /// Allocate a new cluster, linking it to `prev` if provided.
    pub fn alloc_cluster(&mut self, prev: Option<u32>) -> Result<u32, Fat32Error> {
        let cluster = self.fat.alloc().ok_or(Fat32Error::IoError)?;
        if let Some(p) = prev {
            self.fat.set(p, cluster);
        }
        Ok(cluster)
    }
}
