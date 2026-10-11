//! # NTFS Filesystem Read Driver
//!
//! Read-only support for the New Technology File System (NTFS) as used by
//! Windows NT and later.  Covers:
//!
//! - **MFT** (Master File Table) record parsing
//! - **Attribute types**: `$STANDARD_INFORMATION`, `$FILE_NAME`, `$DATA`
//! - **Runlist** (extent list) decoding for non-resident attributes
//! - **B-tree index** (`$INDEX_ROOT` / `$INDEX_ALLOCATION`) traversal
//!
//! Reference: *NTFS Documentation* by Richard Russon & Yuval Fledel (2004),
//! and Linux kernel `fs/ntfs/`.

#![allow(dead_code)]
#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Signature for a valid MFT record: `b"FILE"`.
pub const MFT_RECORD_MAGIC: u32 = 0x454C_4946; // "FILE" LE
/// Signature for an index record: `b"INDX"`.
pub const INDX_RECORD_MAGIC: u32 = 0x58444E49; // "INDX" LE
/// End-of-attributes marker.
pub const ATTR_END_MARKER: u32 = 0xFFFF_FFFF;
/// Cluster factor for standard 4 KiB clusters on a 512-byte sector volume.
pub const DEFAULT_CLUSTER_SIZE: u32 = 4096;

// ---------------------------------------------------------------------------
// Volume Boot Record / BPB
// ---------------------------------------------------------------------------

/// NTFS Volume Boot Record — first 512 bytes of the NTFS partition.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct NtfsBpb {
    /// Jump + NOP bootstrap instructions.
    pub jmp: [u8; 3],
    /// OEM ID: must be `b"NTFS    "`.
    pub oem_id: [u8; 8],
    /// Bytes per sector (typically 512).
    pub bytes_per_sector: u16,
    /// Sectors per cluster.
    pub sectors_per_cluster: u8,
    /// Reserved sectors (0 for NTFS).
    pub reserved_sectors: u16,
    /// Number of FATs (0 for NTFS).
    pub num_fats: u8,
    /// Root entry count (0 for NTFS).
    pub root_entry_count: u16,
    /// Total sectors 16-bit (0 for NTFS).
    pub total_sectors_16: u16,
    /// Media descriptor.
    pub media_descriptor: u8,
    /// Sectors per FAT (0 for NTFS).
    pub sectors_per_fat: u16,
    /// Sectors per track.
    pub sectors_per_track: u16,
    /// Number of heads.
    pub num_heads: u16,
    /// Hidden sectors.
    pub hidden_sectors: u32,
    /// Total sectors 32-bit (0 for NTFS).
    pub total_sectors_32: u32,
    /// NTFS-specific: 0x00800080.
    pub ntfs_reserved: u32,
    /// Total sectors on the volume (64-bit).
    pub total_sectors: u64,
    /// Logical Cluster Number of the first MFT entry.
    pub mft_lcn: u64,
    /// Logical Cluster Number of the MFT mirror ($MFTMirr).
    pub mft_mirror_lcn: u64,
    /// Clusters per MFT record (if positive); encoded as `2^(-n)` bytes if negative.
    pub clusters_per_mft_record: i8,
    /// Reserved.
    pub reserved1: [u8; 3],
    /// Clusters per index block.
    pub clusters_per_index_block: u8,
    /// Reserved.
    pub reserved2: [u8; 3],
    /// Volume serial number.
    pub volume_serial: u64,
    /// Boot sector checksum.
    pub checksum: u32,
}

impl NtfsBpb {
    /// Check the OEM ID for the NTFS signature.
    pub fn is_ntfs(&self) -> bool {
        &self.oem_id == b"NTFS    "
    }

    /// Compute the byte size of one MFT record.
    pub fn mft_record_size(&self) -> u32 {
        if self.clusters_per_mft_record > 0 {
            self.clusters_per_mft_record as u32
                * self.sectors_per_cluster as u32
                * self.bytes_per_sector as u32
        } else {
            1u32 << (-self.clusters_per_mft_record as u32)
        }
    }

    /// Byte offset of the MFT on the volume.
    pub fn mft_byte_offset(&self) -> u64 {
        self.mft_lcn
            * self.sectors_per_cluster as u64
            * self.bytes_per_sector as u64
    }
}

// ---------------------------------------------------------------------------
// MFT Record
// ---------------------------------------------------------------------------

/// NTFS MFT record header (48 bytes).
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct MftRecordHeader {
    /// Magic number: `0x454C4946` (`"FILE"`).
    pub magic: u32,
    /// Offset to the update sequence array from the start of the record.
    pub update_sequence_offset: u16,
    /// Number of entries in the update sequence array (including the USN).
    pub update_sequence_size: u16,
    /// Log file sequence number.
    pub log_file_sequence_number: u64,
    /// Sequence number (incremented each time the record is (re)allocated).
    pub sequence_number: u16,
    /// Hard link count.
    pub hard_link_count: u16,
    /// Byte offset to the first attribute from the record start.
    pub first_attribute_offset: u16,
    /// Record flags (bit 0 = in use; bit 1 = directory).
    pub flags: u16,
    /// Used size of the record in bytes.
    pub used_size: u32,
    /// Allocated size of the record in bytes.
    pub allocated_size: u32,
    /// File reference to the base record (0 if this *is* the base record).
    pub base_file_record: u64,
    /// Next available attribute ID.
    pub next_attribute_id: u16,
    /// (NTFS 3.1+) Padding.
    pub reserved: u16,
    /// (NTFS 3.1+) MFT record number.
    pub mft_record_number: u32,
}

impl MftRecordHeader {
    /// Returns `true` if this record is currently in use.
    pub fn is_in_use(&self) -> bool {
        self.flags & 0x01 != 0
    }

    /// Returns `true` if this record represents a directory.
    pub fn is_directory(&self) -> bool {
        self.flags & 0x02 != 0
    }

    /// Validate the `"FILE"` magic.
    pub fn is_valid(&self) -> bool {
        self.magic == MFT_RECORD_MAGIC
    }
}

// ---------------------------------------------------------------------------
// Attribute Header
// ---------------------------------------------------------------------------

/// Type codes for NTFS attributes.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttributeType {
    StandardInformation = 0x10,
    AttributeList = 0x20,
    FileName = 0x30,
    ObjectId = 0x40,
    SecurityDescriptor = 0x50,
    VolumeName = 0x60,
    VolumeInformation = 0x70,
    Data = 0x80,
    IndexRoot = 0x90,
    IndexAllocation = 0xA0,
    Bitmap = 0xB0,
    ReparsePoint = 0xC0,
    EaInformation = 0xD0,
    Ea = 0xE0,
    LoggedUtilityStream = 0x100,
    End = 0xFFFF_FFFF,
}

impl AttributeType {
    /// Try to parse a `u32` into an `AttributeType`.
    pub fn from_u32(v: u32) -> Option<Self> {
        match v {
            0x10 => Some(Self::StandardInformation),
            0x20 => Some(Self::AttributeList),
            0x30 => Some(Self::FileName),
            0x40 => Some(Self::ObjectId),
            0x50 => Some(Self::SecurityDescriptor),
            0x60 => Some(Self::VolumeName),
            0x70 => Some(Self::VolumeInformation),
            0x80 => Some(Self::Data),
            0x90 => Some(Self::IndexRoot),
            0xA0 => Some(Self::IndexAllocation),
            0xB0 => Some(Self::Bitmap),
            0xC0 => Some(Self::ReparsePoint),
            0xD0 => Some(Self::EaInformation),
            0xE0 => Some(Self::Ea),
            0x100 => Some(Self::LoggedUtilityStream),
            0xFFFF_FFFF => Some(Self::End),
            _ => None,
        }
    }
}

/// Resident attribute header (common prefix for all attributes).
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct AttributeHeader {
    /// Attribute type code.
    pub attr_type: u32,
    /// Length of the attribute record in bytes.
    pub length: u32,
    /// 0 = resident, 1 = non-resident.
    pub non_resident: u8,
    /// Length of the attribute name in characters (0 = no name).
    pub name_length: u8,
    /// Offset to the attribute name from the start of the attribute.
    pub name_offset: u16,
    /// Attribute flags.
    pub flags: u16,
    /// Attribute instance number (unique within the MFT record).
    pub attribute_id: u16,
}

/// Extension header for a **resident** attribute (follows `AttributeHeader`).
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct ResidentAttributeHeader {
    /// Length of the attribute value.
    pub value_length: u32,
    /// Offset to the value from the start of the attribute.
    pub value_offset: u16,
    /// Resident flags (bit 0 = indexed).
    pub resident_flags: u8,
    /// Reserved.
    pub reserved: u8,
}

/// Extension header for a **non-resident** attribute (follows `AttributeHeader`).
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct NonResidentAttributeHeader {
    /// Lowest Virtual Cluster Number (VCN) covered by this attribute record.
    pub lowest_vcn: u64,
    /// Highest VCN covered by this attribute record.
    pub highest_vcn: u64,
    /// Offset to the runlist from the start of the attribute.
    pub runlist_offset: u16,
    /// Compression unit size (0 = not compressed).
    pub compression_unit: u8,
    /// Reserved (5 bytes).
    pub reserved: [u8; 5],
    /// Allocated size in bytes (multiple of cluster size).
    pub allocated_size: u64,
    /// Actual data size in bytes.
    pub data_size: u64,
    /// Initialized data size in bytes (may be ≤ `data_size`).
    pub initialized_size: u64,
}

// ---------------------------------------------------------------------------
// Standard Attributes: $STANDARD_INFORMATION, $FILE_NAME, $DATA
// ---------------------------------------------------------------------------

/// `$STANDARD_INFORMATION` attribute value (at least 48 bytes; up to 72 in v3.x+).
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct StandardInformation {
    /// File creation time (FILETIME = 100-ns intervals since 1601-01-01).
    pub creation_time: u64,
    /// Last modification time.
    pub last_modified_time: u64,
    /// MFT record last modified time.
    pub mft_modified_time: u64,
    /// Last access time.
    pub last_accessed_time: u64,
    /// File attribute flags (same as DOS attributes).
    pub file_attributes: u32,
    /// Maximum allowed versions (0 = no versioning).
    pub max_versions: u32,
    /// Current version number.
    pub version: u32,
    /// Class ID.
    pub class_id: u32,
    // v3.x+ extensions (present if attribute value length >= 72 bytes)
    /// Owner ID.
    pub owner_id: u32,
    /// Security ID into `$Secure`.
    pub security_id: u32,
    /// Quota charged for this file.
    pub quota_charged: u64,
    /// Update sequence number (used by journaling).
    pub usn: u64,
}

/// Namespace values for `$FILE_NAME` attribute.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileNameNamespace {
    /// POSIX (case-sensitive, allows all chars except `\0` and `/`).
    Posix = 0,
    /// Win32 (case-insensitive, restricted character set).
    Win32 = 1,
    /// DOS 8.3 (uppercase, short names).
    Dos = 2,
    /// Combined Win32 & DOS (the Win32 name is also the DOS name).
    Win32Dos = 3,
}

/// `$FILE_NAME` attribute value.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct FileNameAttribute {
    /// File reference to the parent directory MFT record.
    pub parent_directory: u64,
    /// Creation time.
    pub creation_time: u64,
    /// Last data modification time.
    pub last_modified_time: u64,
    /// MFT record modification time.
    pub mft_modified_time: u64,
    /// Last access time.
    pub last_accessed_time: u64,
    /// Allocated size of file data.
    pub allocated_size: u64,
    /// Actual data size.
    pub real_size: u64,
    /// File attribute flags.
    pub file_attributes: u32,
    /// Extended attribute / reparse point size.
    pub reparse_value: u32,
    /// Number of Unicode characters in the file name.
    pub file_name_length: u8,
    /// Filename namespace.
    pub file_name_namespace: u8,
    // Followed by `file_name_length` UTF-16LE code units.
}

// ---------------------------------------------------------------------------
// Runlist Decoding
// ---------------------------------------------------------------------------

/// A single run (extent) mapping Virtual Cluster Numbers (VCNs) to Logical
/// Cluster Numbers (LCNs).
#[derive(Debug, Clone, Copy)]
pub struct Run {
    /// Starting VCN of this run.
    pub vcn: u64,
    /// Length of the run in clusters.
    pub length: u64,
    /// Starting LCN on disk (`None` for a sparse/compressed hole).
    pub lcn: Option<u64>,
}

/// Decode an NTFS runlist (also called a "data run" list) from a raw byte slice.
///
/// The format is a sequence of variable-length elements terminated by a `0x00`
/// header byte:
///
/// ```text
/// Header byte: bits [7:4] = bytes for LCN delta, bits [3:0] = bytes for run length
/// Followed by: run-length bytes (little-endian), then LCN-delta bytes (signed LE)
/// ```
pub fn decode_runlist(data: &[u8]) -> Result<Vec<Run>, NtfsError> {
    let mut runs = Vec::new();
    let mut pos = 0usize;
    let mut current_lcn: i64 = 0;
    let mut current_vcn: u64 = 0;

    while pos < data.len() {
        let header = data[pos];
        if header == 0x00 {
            break;
        }
        pos += 1;

        let len_bytes = (header & 0x0F) as usize;
        let off_bytes = ((header >> 4) & 0x0F) as usize;

        if pos + len_bytes + off_bytes > data.len() {
            return Err(NtfsError::InvalidRunlist);
        }

        // Run length (unsigned, little-endian).
        let mut run_len = 0u64;
        for i in 0..len_bytes {
            run_len |= (data[pos + i] as u64) << (8 * i);
        }
        pos += len_bytes;

        // LCN delta (signed, little-endian).
        let lcn: Option<u64> = if off_bytes > 0 {
            let mut delta = 0i64;
            for i in 0..off_bytes {
                delta |= (data[pos + i] as i64) << (8 * i);
            }
            // Sign-extend if the high bit of the last byte is set.
            if data[pos + off_bytes - 1] & 0x80 != 0 {
                delta |= !((1i64 << (8 * off_bytes)) - 1);
            }
            pos += off_bytes;
            current_lcn = current_lcn.checked_add(delta).ok_or(NtfsError::InvalidRunlist)?;
            Some(current_lcn as u64)
        } else {
            // Sparse run.
            pos += off_bytes;
            None
        };

        runs.push(Run { vcn: current_vcn, length: run_len, lcn });
        current_vcn += run_len;
    }
    Ok(runs)
}

// ---------------------------------------------------------------------------
// B-tree Index ($INDEX_ROOT)
// ---------------------------------------------------------------------------

/// `$INDEX_ROOT` attribute value header.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct IndexRootHeader {
    /// Attribute type that is indexed (e.g. `0x30` for `$FILE_NAME`).
    pub indexed_attribute_type: u32,
    /// Collation rule (e.g. `0x01` = Unicode string).
    pub collation_rule: u32,
    /// Index allocation block size in bytes.
    pub index_block_size: u32,
    /// Clusters per index block.
    pub clusters_per_index_block: u8,
    /// Reserved (3 bytes).
    pub reserved: [u8; 3],
}

/// Index node header (follows `IndexRootHeader` or begins an `$INDEX_ALLOCATION` block).
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct IndexNodeHeader {
    /// Offset to the first index entry, relative to this header.
    pub first_entry_offset: u32,
    /// Offset past the last index entry, relative to this header.
    pub used_size: u32,
    /// Allocated size of the index entry list, relative to this header.
    pub allocated_size: u32,
    /// Flags (bit 0 = node has sub-nodes).
    pub flags: u8,
    /// Reserved (3 bytes).
    pub reserved: [u8; 3],
}

/// A single index entry within an NTFS B-tree node.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct IndexEntry {
    /// File reference for the indexed file.
    pub file_reference: u64,
    /// Length of this index entry in bytes.
    pub entry_length: u16,
    /// Length of the key attribute in bytes.
    pub key_length: u16,
    /// Entry flags (bit 0 = has sub-node; bit 1 = last entry in node).
    pub flags: u16,
    /// Reserved (2 bytes).
    pub reserved: u16,
    // Followed by the key attribute data and (if has sub-node) a VCN.
}

impl IndexEntry {
    /// Returns `true` if this is the last entry in the node (no key data).
    pub fn is_last(&self) -> bool {
        self.flags & 0x02 != 0
    }

    /// Returns `true` if this entry points to a sub-node.
    pub fn has_sub_node(&self) -> bool {
        self.flags & 0x01 != 0
    }
}

// ---------------------------------------------------------------------------
// Error Type
// ---------------------------------------------------------------------------

/// Errors produced by the NTFS driver.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NtfsError {
    /// The volume does not carry the `"NTFS    "` OEM ID.
    NotNtfs,
    /// An MFT record has an invalid `"FILE"` signature.
    InvalidMftRecord,
    /// An attribute header is malformed.
    InvalidAttribute,
    /// The runlist encoding is invalid.
    InvalidRunlist,
    /// An index record has an invalid `"INDX"` signature.
    InvalidIndexRecord,
    /// A requested MFT record number is out of range.
    OutOfRange,
    /// An I/O error occurred.
    IoError,
}

impl fmt::Display for NtfsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NtfsError::NotNtfs => write!(f, "not an NTFS volume"),
            NtfsError::InvalidMftRecord => write!(f, "invalid MFT record"),
            NtfsError::InvalidAttribute => write!(f, "invalid attribute"),
            NtfsError::InvalidRunlist => write!(f, "invalid runlist"),
            NtfsError::InvalidIndexRecord => write!(f, "invalid index record"),
            NtfsError::OutOfRange => write!(f, "MFT record number out of range"),
            NtfsError::IoError => write!(f, "I/O error"),
        }
    }
}

// ---------------------------------------------------------------------------
// High-Level NTFS Reader
// ---------------------------------------------------------------------------

/// Read-only handle to a mounted NTFS volume.
pub struct NtfsVolume {
    /// Parsed BPB.
    pub bpb: NtfsBpb,
    /// Size of one MFT record in bytes.
    pub mft_record_size: u32,
    /// Runs describing the location of `$MFT` on the volume.
    pub mft_runs: Vec<Run>,
}

impl NtfsVolume {
    /// Initialise an NTFS volume from the raw VBR sector.
    pub fn open(vbr: &[u8; 512]) -> Result<Self, NtfsError> {
        // SAFETY: buffer is exactly 512 bytes.
        let bpb: NtfsBpb =
            unsafe { core::ptr::read_unaligned(vbr.as_ptr() as *const NtfsBpb) };
        if !bpb.is_ntfs() {
            return Err(NtfsError::NotNtfs);
        }
        let mft_record_size = bpb.mft_record_size();
        Ok(NtfsVolume { bpb, mft_record_size, mft_runs: Vec::new() })
    }

    /// Load the MFT runlist after reading the boot MFT record from the device.
    pub fn load_mft_runs(&mut self, mft_data_runlist: &[u8]) -> Result<(), NtfsError> {
        self.mft_runs = decode_runlist(mft_data_runlist)?;
        Ok(())
    }

    /// Translate a VCN within `$MFT` to a byte offset on the device.
    pub fn vcn_to_byte_offset(&self, vcn: u64) -> Option<u64> {
        let cluster_size = self.bpb.sectors_per_cluster as u64 * self.bpb.bytes_per_sector as u64;
        for run in &self.mft_runs {
            if vcn >= run.vcn && vcn < run.vcn + run.length {
                let lcn = run.lcn? + (vcn - run.vcn);
                return Some(lcn * cluster_size);
            }
        }
        None
    }

    /// Parse attributes from raw MFT record bytes.
    ///
    /// Returns a list of `(AttributeType, byte_offset_in_record)` pairs.
    pub fn parse_attributes(&self, record: &[u8]) -> Result<Vec<(AttributeType, usize)>, NtfsError> {
        let hdr: MftRecordHeader =
            unsafe { core::ptr::read_unaligned(record.as_ptr() as *const MftRecordHeader) };
        if !hdr.is_valid() {
            return Err(NtfsError::InvalidMftRecord);
        }
        let mut attrs = Vec::new();
        let mut offset = hdr.first_attribute_offset as usize;
        while offset + core::mem::size_of::<AttributeHeader>() <= record.len() {
            let attr: AttributeHeader =
                unsafe { core::ptr::read_unaligned(record[offset..].as_ptr() as *const AttributeHeader) };
            if attr.attr_type == ATTR_END_MARKER || attr.length == 0 {
                break;
            }
            if let Some(ty) = AttributeType::from_u32(attr.attr_type) {
                attrs.push((ty, offset));
            }
            offset += attr.length as usize;
        }
        Ok(attrs)
    }
}
