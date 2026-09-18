#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
use std::vec;

// Btrfs - Linux-style Copy-on-Write filesystem
// Supports snapshots, subvolumes, compression, and checksums

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionType {
    None,
    Zlib,
    Lzo,
    Zstd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumType {
    Crc32c,
    Xxhash,
    Sha256,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RaidProfile {
    Single,
    Dup,
    Raid0,
    Raid1,
}

#[derive(Debug, Clone)]
pub struct BtrfsSubvolume {
    pub id: u64,
    pub parent_id: Option<u64>,
    pub name: String,
    pub uuid: [u8; 16],
    pub compression: CompressionType,
    pub compression_level: Option<u8>, // Precise tuning (Fedora default is zstd:3)
    pub readonly: bool,
}

#[derive(Debug, Clone)]
pub struct BtrfsSnapshot {
    pub id: u64,
    pub source_subvol_id: u64,
    pub name: String,
    pub created_at: u64,
    pub readonly: bool,
}

#[derive(Debug, Clone)]
pub struct BtrfsDevice {
    pub id: u32,
    pub path: String,
    pub size_bytes: u64,
    pub used_bytes: u64,
}

#[derive(Debug, Clone)]
pub struct BtrfsExtentReplica {
    pub device_id: u32,
    pub physical_offset: u64,
}

#[derive(Debug, Clone)]
pub struct BtrfsExtent {
    pub subvol_id: u64, // Associated subvolume ID (for send/receive and defrag tracking)
    pub offset: u64,
    pub length: u64,
    pub compression: CompressionType,
    pub checksum: [u8; 32],
    pub replicas: Vec<BtrfsExtentReplica>,
    pub data_hash_corrupted: bool,
}

#[derive(Debug, Clone)]
pub struct BtrfsQgroup {
    pub id: u64,
    pub referenced_bytes: u64,
    pub exclusive_bytes: u64,
    pub limit_referenced: Option<u64>,
    pub limit_exclusive: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct BtrfsScrubResult {
    pub extents_scrubbed: usize,
    pub errors_found: usize,
    pub errors_healed: usize,
}

/// Linux distro-inspired default Btrfs mount options (Fedora & openSUSE defaults)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BtrfsMountOptions {
    pub ssd: bool,            // Optimize block allocation layouts for SSDs
    pub discard_async: bool,  // Background async TRIM / space reclamation
    pub autodefrag: bool,     // Background automatic defragmentation of small files
    pub compress_force: bool, // Bypass standard entropy estimator heuristics
    pub noatime: bool,        // Reduce CoW write amplification from access time updates
}

impl Default for BtrfsMountOptions {
    fn default() -> Self {
        Self {
            ssd: true,
            discard_async: true,
            autodefrag: true,
            compress_force: false,
            noatime: true,
        }
    }
}

/// Stream operations representing changes between subvolumes/snapshots (Timeshift & Snapper incremental backups)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BtrfsSendOperation {
    CreateSubvolume {
        id: u64,
        parent_id: Option<u64>,
        name: String,
        uuid: [u8; 16],
        compression: CompressionType,
        readonly: bool,
    },
    WriteExtent {
        subvol_id: u64,
        offset: u64,
        length: u64,
        compression: CompressionType,
        checksum: [u8; 32],
        data: Vec<u8>,
    },
    SetReadonly {
        subvol_id: u64,
        readonly: bool,
    },
    SetCompression {
        subvol_id: u64,
        compression: CompressionType,
    },
}

pub struct BtrfsFilesystem {
    subvolumes: BTreeMap<u64, BtrfsSubvolume>,
    snapshots: BTreeMap<u64, BtrfsSnapshot>,
    extents: BTreeMap<u64, BtrfsExtent>,
    next_subvol_id: u64,
    next_snapshot_id: u64,
    default_compression: CompressionType,
    default_compression_level: u8, // e.g. Fedora defaults to level 3
    checksum_type: ChecksumType,
    devices: BTreeMap<u32, BtrfsDevice>,
    raid_profile: RaidProfile,
    qgroups: BTreeMap<u64, BtrfsQgroup>,
    pub mount_options: BtrfsMountOptions,
    pub metadata_write_count: usize, // Track metadata CoW updates
}

impl BtrfsFilesystem {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut fs = Self {
            subvolumes: BTreeMap::new(),
            snapshots: BTreeMap::new(),
            extents: BTreeMap::new(),
            next_subvol_id: 5, // Start at 5 (0-4 are reserved)
            next_snapshot_id: 1,
            default_compression: CompressionType::Zlib,
            default_compression_level: 3,
            checksum_type: ChecksumType::Crc32c,
            devices: BTreeMap::new(),
            raid_profile: RaidProfile::Single,
            qgroups: BTreeMap::new(),
            mount_options: BtrfsMountOptions::default(),
            metadata_write_count: 0,
        };

        // Add a default primary device (Standard modern Linux layout)
        fs.add_device("/dev/sda1".to_string(), 1024 * 1024 * 1024); // 1 GB
        fs
    }

    /// Add a device to the storage pool (RAID/stripe layout inspiration)
    pub fn add_device(&mut self, path: String, size_bytes: u64) -> u32 {
        let id = (self.devices.len() + 1) as u32;
        let device = BtrfsDevice {
            id,
            path,
            size_bytes,
            used_bytes: 0,
        };
        self.devices.insert(id, device);
        id
    }

    /// Set RAID profile
    pub fn set_raid_profile(&mut self, profile: RaidProfile) {
        self.raid_profile = profile;
    }

    /// Get RAID profile
    pub fn raid_profile(&self) -> RaidProfile {
        self.raid_profile
    }

    /// Enable quota group for subvolume
    pub fn enable_qgroup(&mut self, subvol_id: u64) -> Result<(), &'static str> {
        if !self.subvolumes.contains_key(&subvol_id) {
            return Err("Subvolume not found");
        }
        let qgroup = BtrfsQgroup {
            id: subvol_id,
            referenced_bytes: 0,
            exclusive_bytes: 0,
            limit_referenced: None,
            limit_exclusive: None,
        };
        self.qgroups.insert(subvol_id, qgroup);
        Ok(())
    }

    /// Set quota group limits
    pub fn set_qgroup_limit(
        &mut self,
        subvol_id: u64,
        limit_referenced: Option<u64>,
        limit_exclusive: Option<u64>,
    ) -> Result<(), &'static str> {
        let qgroup = self
            .qgroups
            .get_mut(&subvol_id)
            .ok_or("Qgroup not enabled")?;
        qgroup.limit_referenced = limit_referenced;
        qgroup.limit_exclusive = limit_exclusive;
        Ok(())
    }

    /// Get quota group state
    pub fn get_qgroup(&self, subvol_id: u64) -> Option<&BtrfsQgroup> {
        self.qgroups.get(&subvol_id)
    }

    /// Transparent compression heuristics analyzing data entropy (Fedora style)
    pub fn heuristic_compress(&self, subvol_id: u64, data: &[u8]) -> (CompressionType, Vec<u8>) {
        let compression = self
            .subvolumes
            .get(&subvol_id)
            .map(|s| s.compression)
            .unwrap_or(self.default_compression);

        if compression == CompressionType::None {
            return (CompressionType::None, data.to_vec());
        }

        // If Fedora style 'compress-force' mount option is enabled, always compress directly!
        if self.mount_options.compress_force {
            let mut compressed = Vec::new();
            compressed.push(b'C');
            compressed.push(b'O');
            compressed.push(b'M');
            compressed.push(b'P');
            let indicator = match compression {
                CompressionType::Zlib => b'Z',
                CompressionType::Lzo => b'L',
                CompressionType::Zstd => b'S',
                _ => b'X',
            };
            compressed.push(indicator);
            let take_len = (data.len() / 2).max(1);
            for &b in &data[..take_len] {
                compressed.push(b);
            }
            return (compression, compressed);
        }

        // Entropy estimator ratio based on unique bytes density
        let mut unique_count = 0;
        let mut seen = [false; 256];
        for &b in data {
            if !seen[b as usize] {
                seen[b as usize] = true;
                unique_count += 1;
            }
        }
        let ratio = if data.is_empty() {
            0.0
        } else {
            (unique_count as f32) / (data.len() as f32).min(256.0)
        };

        if ratio >= 0.75 {
            // High entropy (e.g., highly random, encrypted, or already compressed binary data) -> skip compression
            (CompressionType::None, data.to_vec())
        } else {
            // Low entropy (e.g., repeating sequences, text) -> simulate compression
            let mut compressed = Vec::new();
            compressed.push(b'C');
            compressed.push(b'O');
            compressed.push(b'M');
            compressed.push(b'P');
            let indicator = match compression {
                CompressionType::Zlib => b'Z',
                CompressionType::Lzo => b'L',
                CompressionType::Zstd => b'S',
                _ => b'X',
            };
            compressed.push(indicator);
            let take_len = (data.len() / 2).max(1);
            for &b in &data[..take_len] {
                compressed.push(b);
            }
            (compression, compressed)
        }
    }

    /// Compute extent checksum using modern robust checksum algorithm
    pub fn compute_extent_checksum(&self, data: &[u8]) -> [u8; 32] {
        let mut hash = [0u8; 32];
        match self.checksum_type {
            ChecksumType::Crc32c => {
                let mut crc: u32 = 0xFFFFFFFF;
                for &b in data {
                    crc ^= b as u32;
                    for _ in 0..8 {
                        if (crc & 1) != 0 {
                            crc = (crc >> 1) ^ 0x82F63B78; // Castagnoli polynomial
                        } else {
                            crc >>= 1;
                        }
                    }
                }
                let bytes = (!crc).to_be_bytes();
                hash[0..4].copy_from_slice(&bytes);
            }
            ChecksumType::Xxhash => {
                let mut h: u64 = 0x243F6A8885A308D3;
                for &b in data {
                    h = h.wrapping_add(b as u64);
                    h = h.wrapping_mul(0xFF51AFD7ED558CCD);
                    h = (h << 31) | (h >> 33);
                }
                let bytes = h.to_be_bytes();
                hash[0..8].copy_from_slice(&bytes);
            }
            ChecksumType::Sha256 => {
                let mut h0: u64 = 0x6a09e667f3bcc908;
                let mut h1: u64 = 0xbb67ae8584caa73b;
                let mut h2: u64 = 0x3c6ef372fe94f82b;
                let mut h3: u64 = 0xa54ff53a5f1d36f1;
                for &b in data {
                    h0 = (h0 ^ b as u64).wrapping_mul(1099511628211);
                    h1 = (h1 ^ h0).wrapping_mul(1099511628211);
                    h2 = (h2 ^ h1).wrapping_mul(1099511628211);
                    h3 = (h3 ^ h2).wrapping_mul(1099511628211);
                }
                hash[0..8].copy_from_slice(&h0.to_be_bytes());
                hash[8..16].copy_from_slice(&h1.to_be_bytes());
                hash[16..24].copy_from_slice(&h2.to_be_bytes());
                hash[24..32].copy_from_slice(&h3.to_be_bytes());
            }
        }
        hash
    }

    /// Write data to an extent in a subvolume
    pub fn write_data(
        &mut self,
        subvol_id: u64,
        offset: u64,
        data: &[u8],
    ) -> Result<u64, &'static str> {
        let subvol = self
            .subvolumes
            .get(&subvol_id)
            .ok_or("Subvolume not found")?;
        if subvol.readonly {
            return Err("Subvolume is read-only");
        }

        let data_len = data.len() as u64;

        // Check quota group limits
        if let Some(qgroup) = self.qgroups.get(&subvol_id) {
            if let Some(limit) = qgroup.limit_referenced {
                if qgroup.referenced_bytes + data_len > limit {
                    return Err("Quota limit (referenced) exceeded");
                }
            }
            if let Some(limit) = qgroup.limit_exclusive {
                if qgroup.exclusive_bytes + data_len > limit {
                    return Err("Quota limit (exclusive) exceeded");
                }
            }
        }

        // Transparent compression heuristics
        let (comp_type, processed_data) = self.heuristic_compress(subvol_id, data);

        // Compute checksum
        let checksum = self.compute_extent_checksum(&processed_data);

        // Physical mapping placement based on RAID Profile
        let mut replicas = Vec::new();
        match self.raid_profile {
            RaidProfile::Single => {
                replicas.push(BtrfsExtentReplica {
                    device_id: 1,
                    physical_offset: offset,
                });
            }
            RaidProfile::Dup => {
                replicas.push(BtrfsExtentReplica {
                    device_id: 1,
                    physical_offset: offset,
                });
                replicas.push(BtrfsExtentReplica {
                    device_id: 1,
                    physical_offset: offset + data_len + 1024,
                });
            }
            RaidProfile::Raid0 => {
                let device_count = self.devices.len() as u32;
                if device_count > 0 {
                    let dev_id = (offset % device_count as u64) as u32 + 1;
                    replicas.push(BtrfsExtentReplica {
                        device_id: dev_id,
                        physical_offset: offset / device_count as u64,
                    });
                } else {
                    replicas.push(BtrfsExtentReplica {
                        device_id: 1,
                        physical_offset: offset,
                    });
                }
            }
            RaidProfile::Raid1 => {
                replicas.push(BtrfsExtentReplica {
                    device_id: 1,
                    physical_offset: offset,
                });
                if self.devices.len() >= 2 {
                    replicas.push(BtrfsExtentReplica {
                        device_id: 2,
                        physical_offset: offset,
                    });
                } else {
                    // Fallback to DUP layout copy
                    replicas.push(BtrfsExtentReplica {
                        device_id: 1,
                        physical_offset: offset + data_len + 1024,
                    });
                }
            }
        }

        // Deduct device pool space
        for rep in &replicas {
            if let Some(device) = self.devices.get_mut(&rep.device_id) {
                if device.used_bytes + data_len > device.size_bytes {
                    return Err("No space left on device");
                }
                device.used_bytes += data_len;
            }
        }

        // Map and save extent
        let extent_id = offset;
        let extent = BtrfsExtent {
            subvol_id,
            offset,
            length: data_len,
            compression: comp_type,
            checksum,
            replicas,
            data_hash_corrupted: false,
        };
        self.extents.insert(extent_id, extent);

        // Update qgroup stats
        if let Some(qgroup) = self.qgroups.get_mut(&subvol_id) {
            qgroup.referenced_bytes += data_len;
            qgroup.exclusive_bytes += data_len;
        }

        self.metadata_write_count += 1;

        Ok(extent_id)
    }

    /// Read data from subvolume extent
    pub fn read_data(&self, offset: u64) -> Result<Vec<u8>, &'static str> {
        let extent = self.extents.get(&offset).ok_or("Extent not found")?;
        if extent.data_hash_corrupted {
            return Err("Input/output error (checksum verification failed)");
        }
        Ok(std::vec![0u8; extent.length as usize])
    }

    /// Scrub the filesystem, verifying checksums and repairing corrupt copies (Self-Healing)
    pub fn scrub(&mut self) -> Result<BtrfsScrubResult, &'static str> {
        let mut result = BtrfsScrubResult {
            extents_scrubbed: 0,
            errors_found: 0,
            errors_healed: 0,
        };

        for extent in self.extents.values_mut() {
            result.extents_scrubbed += 1;
            if extent.data_hash_corrupted {
                result.errors_found += 1;
                // If we have redundant replicas (DUP or RAID1), we can heal the corrupted data!
                if extent.replicas.len() >= 2 {
                    extent.data_hash_corrupted = false;
                    result.errors_healed += 1;
                }
            }
        }

        Ok(result)
    }

    /// Helper to corrupt an extent for testing scrubbing/healing
    pub fn corrupt_extent(&mut self, offset: u64) -> Result<(), &'static str> {
        let extent = self.extents.get_mut(&offset).ok_or("Extent not found")?;
        extent.data_hash_corrupted = true;
        Ok(())
    }

    /// Get extent detail
    pub fn get_extent(&self, offset: u64) -> Option<&BtrfsExtent> {
        self.extents.get(&offset)
    }

    /// Create a new subvolume
    pub fn create_subvolume(
        &mut self,
        name: String,
        parent_id: Option<u64>,
    ) -> Result<u64, &'static str> {
        let id = self.next_subvol_id;
        self.next_subvol_id += 1;

        let uuid = self.generate_uuid();

        let subvol = BtrfsSubvolume {
            id,
            parent_id,
            name: name.clone(),
            uuid,
            compression: self.default_compression,
            compression_level: Some(self.default_compression_level),
            readonly: false,
        };

        self.subvolumes.insert(id, subvol);
        Ok(id)
    }

    /// Create a new subvolume with property inheritance from parent (e.g. compression, read-only, limits)
    pub fn create_subvolume_with_inheritance(
        &mut self,
        name: String,
        parent_id: Option<u64>,
    ) -> Result<u64, &'static str> {
        let id = self.next_subvol_id;
        self.next_subvol_id += 1;

        let uuid = self.generate_uuid();

        // Inherit properties if parent exists
        let (compression, level, readonly) = if let Some(p_id) = parent_id {
            if let Some(parent) = self.subvolumes.get(&p_id) {
                (
                    parent.compression,
                    parent.compression_level,
                    parent.readonly,
                )
            } else {
                (
                    self.default_compression,
                    Some(self.default_compression_level),
                    false,
                )
            }
        } else {
            (
                self.default_compression,
                Some(self.default_compression_level),
                false,
            )
        };

        let subvol = BtrfsSubvolume {
            id,
            parent_id,
            name: name.clone(),
            uuid,
            compression,
            compression_level: level,
            readonly,
        };

        self.subvolumes.insert(id, subvol);

        // Inherit parent qgroup limits if enabled on parent
        if let Some(p_id) = parent_id {
            if let Some(parent_qgroup) = self.qgroups.get(&p_id).cloned() {
                self.enable_qgroup(id)?;
                self.set_qgroup_limit(
                    id,
                    parent_qgroup.limit_referenced,
                    parent_qgroup.limit_exclusive,
                )?;
            }
        }

        Ok(id)
    }

    /// Create a snapshot of a subvolume
    pub fn create_snapshot(
        &mut self,
        source_id: u64,
        name: String,
        readonly: bool,
    ) -> Result<u64, &'static str> {
        if !self.subvolumes.contains_key(&source_id) {
            return Err("Source subvolume not found");
        }

        let id = self.next_snapshot_id;
        self.next_snapshot_id += 1;

        let snapshot = BtrfsSnapshot {
            id,
            source_subvol_id: source_id,
            name: name.clone(),
            created_at: 0, // Would use actual timestamp
            readonly,
        };

        self.snapshots.insert(id, snapshot);
        Ok(id)
    }

    /// Delete a subvolume
    pub fn delete_subvolume(&mut self, id: u64) -> Result<(), &'static str> {
        if id < 5 {
            return Err("Cannot delete reserved subvolume");
        }

        // Check if any snapshots depend on this subvolume
        for snapshot in self.snapshots.values() {
            if snapshot.source_subvol_id == id {
                return Err("Subvolume has snapshots");
            }
        }

        self.subvolumes.remove(&id).ok_or("Subvolume not found")?;

        Ok(())
    }

    /// Set subvolume as readonly
    pub fn set_readonly(&mut self, id: u64, readonly: bool) -> Result<(), &'static str> {
        let subvol = self.subvolumes.get_mut(&id).ok_or("Subvolume not found")?;

        subvol.readonly = readonly;
        Ok(())
    }

    /// Set compression for a subvolume
    pub fn set_compression(
        &mut self,
        id: u64,
        compression: CompressionType,
    ) -> Result<(), &'static str> {
        let subvol = self.subvolumes.get_mut(&id).ok_or("Subvolume not found")?;

        subvol.compression = compression;
        Ok(())
    }

    /// Get subvolume by ID
    pub fn get_subvolume(&self, id: u64) -> Option<&BtrfsSubvolume> {
        self.subvolumes.get(&id)
    }

    /// Get snapshot by ID
    pub fn get_snapshot(&self, id: u64) -> Option<&BtrfsSnapshot> {
        self.snapshots.get(&id)
    }

    /// List all subvolumes
    pub fn list_subvolumes(&self) -> Vec<&BtrfsSubvolume> {
        self.subvolumes.values().collect()
    }

    /// List all snapshots
    pub fn list_snapshots(&self) -> Vec<&BtrfsSnapshot> {
        self.snapshots.values().collect()
    }

    /// Set default compression
    pub fn set_default_compression(&mut self, compression: CompressionType) {
        self.default_compression = compression;
    }

    /// Get default compression
    pub fn default_compression(&self) -> CompressionType {
        self.default_compression
    }

    /// Generate UUID (simplified)
    fn generate_uuid(&self) -> [u8; 16] {
        [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10,
        ]
    }

    /// Get subvolume count
    pub fn subvolume_count(&self) -> usize {
        self.subvolumes.len()
    }

    /// Get snapshot count
    pub fn snapshot_count(&self) -> usize {
        self.snapshots.len()
    }

    /// Defragment a subvolume, merging small adjacent CoW extents into single contiguous extents (Fedora style autodefrag)
    pub fn defragment_subvolume(&mut self, subvol_id: u64) -> Result<usize, &'static str> {
        if !self.subvolumes.contains_key(&subvol_id) {
            return Err("Subvolume not found");
        }

        // Get all extents for this subvolume
        let mut subvol_extents: Vec<BtrfsExtent> = self
            .extents
            .values()
            .filter(|ext| ext.subvol_id == subvol_id)
            .cloned()
            .collect();

        if subvol_extents.len() < 2 {
            return Ok(0); // Nothing to defragment
        }

        // Sort extents by offset
        subvol_extents.sort_by_key(|ext| ext.offset);

        let mut defragged_count = 0;
        let mut merged_extents = Vec::new();
        let mut current: Option<BtrfsExtent> = None;

        for ext in subvol_extents {
            if let Some(mut curr) = current {
                // If adjacent in physical address space and share same compression status
                if curr.offset + curr.length == ext.offset && curr.compression == ext.compression {
                    // To safely merge, physical replicas must be contiguous on the same devices
                    let mut physical_contiguous = true;
                    if curr.replicas.len() != ext.replicas.len() {
                        physical_contiguous = false;
                    } else {
                        for i in 0..curr.replicas.len() {
                            let r1 = &curr.replicas[i];
                            let r2 = &ext.replicas[i];
                            if r1.device_id != r2.device_id
                                || r1.physical_offset + curr.length != r2.physical_offset
                            {
                                physical_contiguous = false;
                                break;
                            }
                        }
                    }

                    if physical_contiguous {
                        // Merge adjacent extents logically and physically
                        curr.length += ext.length;
                        // Recompute merged checksum
                        let mut merged_checksum = [0u8; 32];
                        for i in 0..32 {
                            merged_checksum[i] = curr.checksum[i] ^ ext.checksum[i];
                        }
                        curr.checksum = merged_checksum;
                        current = Some(curr);
                        defragged_count += 1;
                    } else {
                        merged_extents.push(curr);
                        current = Some(ext);
                    }
                } else {
                    merged_extents.push(curr);
                    current = Some(ext);
                }
            } else {
                current = Some(ext);
            }
        }
        if let Some(curr) = current {
            merged_extents.push(curr);
        }

        // Re-insert merged extents and remove old ones
        self.extents.retain(|_, ext| ext.subvol_id != subvol_id);

        for ext in merged_extents {
            self.extents.insert(ext.offset, ext);
        }

        Ok(defragged_count)
    }

    /// Trigger an asynchronous discard (TRIM) command across devices (Fedora/openSUSE style SSD maintenance)
    pub fn trigger_async_discard(&mut self) -> Result<usize, &'static str> {
        if !self.mount_options.discard_async {
            return Err("Async discard mount option is not enabled");
        }
        let mut trimmed_bytes = 0;
        for device in self.devices.values_mut() {
            let unused = device.size_bytes.saturating_sub(device.used_bytes);
            if unused > 0 {
                trimmed_bytes += unused;
            }
        }
        Ok(trimmed_bytes as usize)
    }

    /// Send subvolume as a series of instructions representing its metadata and extents (Incremental Backup Send)
    pub fn send_subvolume(&self, subvol_id: u64) -> Result<Vec<BtrfsSendOperation>, &'static str> {
        let subvol = self
            .subvolumes
            .get(&subvol_id)
            .ok_or("Subvolume not found")?;
        let mut ops = Vec::new();

        ops.push(BtrfsSendOperation::CreateSubvolume {
            id: subvol.id,
            parent_id: subvol.parent_id,
            name: subvol.name.clone(),
            uuid: subvol.uuid,
            compression: subvol.compression,
            readonly: subvol.readonly,
        });

        // Find all extents belonging to this subvolume and append write operations
        for extent in self.extents.values() {
            if extent.subvol_id == subvol_id {
                let data = self
                    .read_data(extent.offset)
                    .unwrap_or_else(|_| std::vec![0u8; extent.length as usize]);
                ops.push(BtrfsSendOperation::WriteExtent {
                    subvol_id,
                    offset: extent.offset,
                    length: extent.length,
                    compression: extent.compression,
                    checksum: extent.checksum,
                    data,
                });
            }
        }

        Ok(ops)
    }

    /// Receive and replay a series of subvolume operations to recreate/sync a subvolume (Incremental Backup Receive)
    pub fn receive_subvolume(
        &mut self,
        operations: &[BtrfsSendOperation],
    ) -> Result<u64, &'static str> {
        let mut created_subvol_id = None;

        for op in operations {
            match op {
                BtrfsSendOperation::CreateSubvolume {
                    id,
                    parent_id,
                    name,
                    uuid,
                    compression,
                    readonly,
                } => {
                    let subvol_id = *id;
                    let subvol = BtrfsSubvolume {
                        id: subvol_id,
                        parent_id: *parent_id,
                        name: name.clone(),
                        uuid: *uuid,
                        compression: *compression,
                        compression_level: Some(self.default_compression_level),
                        readonly: *readonly,
                    };
                    self.subvolumes.insert(subvol_id, subvol);
                    if self.next_subvol_id <= subvol_id {
                        self.next_subvol_id = subvol_id + 1;
                    }
                    created_subvol_id = Some(subvol_id);
                }
                BtrfsSendOperation::WriteExtent {
                    subvol_id,
                    offset,
                    length,
                    compression,
                    checksum,
                    data: _,
                } => {
                    let mut replicas = Vec::new();
                    match self.raid_profile {
                        RaidProfile::Single => {
                            replicas.push(BtrfsExtentReplica {
                                device_id: 1,
                                physical_offset: *offset,
                            });
                        }
                        _ => {
                            replicas.push(BtrfsExtentReplica {
                                device_id: 1,
                                physical_offset: *offset,
                            });
                            if self.devices.len() >= 2 {
                                replicas.push(BtrfsExtentReplica {
                                    device_id: 2,
                                    physical_offset: *offset,
                                });
                            }
                        }
                    }

                    for rep in &replicas {
                        if let Some(device) = self.devices.get_mut(&rep.device_id) {
                            device.used_bytes = (device.used_bytes + length).min(device.size_bytes);
                        }
                    }

                    let extent = BtrfsExtent {
                        subvol_id: *subvol_id,
                        offset: *offset,
                        length: *length,
                        compression: *compression,
                        checksum: *checksum,
                        replicas,
                        data_hash_corrupted: false,
                    };
                    self.extents.insert(*offset, extent);

                    if let Some(qgroup) = self.qgroups.get_mut(subvol_id) {
                        qgroup.referenced_bytes += length;
                        qgroup.exclusive_bytes += length;
                    }
                }
                BtrfsSendOperation::SetReadonly {
                    subvol_id,
                    readonly,
                } => {
                    if let Some(subvol) = self.subvolumes.get_mut(subvol_id) {
                        subvol.readonly = *readonly;
                    }
                }
                BtrfsSendOperation::SetCompression {
                    subvol_id,
                    compression,
                } => {
                    if let Some(subvol) = self.subvolumes.get_mut(subvol_id) {
                        subvol.compression = *compression;
                    }
                }
            }
        }

        created_subvol_id.ok_or("No subvolume creation operation found in stream")
    }
}

impl Default for BtrfsFilesystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_subvolume() {
        let mut fs = BtrfsFilesystem::new();

        let id = fs.create_subvolume("test".to_string(), None).unwrap();
        assert_eq!(fs.subvolume_count(), 1);

        let subvol = fs.get_subvolume(id).unwrap();
        assert_eq!(subvol.name, "test");
    }

    #[test]
    fn test_create_snapshot() {
        let mut fs = BtrfsFilesystem::new();

        let subvol_id = fs.create_subvolume("test".to_string(), None).unwrap();
        let snapshot_id = fs
            .create_snapshot(subvol_id, "test_snapshot".to_string(), true)
            .unwrap();

        assert_eq!(fs.snapshot_count(), 1);

        let snapshot = fs.get_snapshot(snapshot_id).unwrap();
        assert_eq!(snapshot.source_subvol_id, subvol_id);
    }

    #[test]
    fn test_delete_subvolume() {
        let mut fs = BtrfsFilesystem::new();

        let id = fs.create_subvolume("test".to_string(), None).unwrap();
        fs.delete_subvolume(id).unwrap();

        assert_eq!(fs.subvolume_count(), 0);
    }

    #[test]
    fn test_delete_with_snapshots() {
        let mut fs = BtrfsFilesystem::new();

        let subvol_id = fs.create_subvolume("test".to_string(), None).unwrap();
        fs.create_snapshot(subvol_id, "snapshot".to_string(), true)
            .unwrap();

        let result = fs.delete_subvolume(subvol_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_set_readonly() {
        let mut fs = BtrfsFilesystem::new();

        let id = fs.create_subvolume("test".to_string(), None).unwrap();
        fs.set_readonly(id, true).unwrap();

        let subvol = fs.get_subvolume(id).unwrap();
        assert!(subvol.readonly);
    }

    #[test]
    fn test_set_compression() {
        let mut fs = BtrfsFilesystem::new();

        let id = fs.create_subvolume("test".to_string(), None).unwrap();
        fs.set_compression(id, CompressionType::Lzo).unwrap();

        let subvol = fs.get_subvolume(id).unwrap();
        assert_eq!(subvol.compression, CompressionType::Lzo);
    }

    #[test]
    fn test_list_subvolumes() {
        let mut fs = BtrfsFilesystem::new();

        fs.create_subvolume("test1".to_string(), None).unwrap();
        fs.create_subvolume("test2".to_string(), None).unwrap();

        let subvols = fs.list_subvolumes();
        assert_eq!(subvols.len(), 2);
    }

    #[test]
    fn test_btrfs_quota_limits() {
        let mut fs = BtrfsFilesystem::new();
        let subvol_id = fs
            .create_subvolume("limited_subvol".to_string(), None)
            .unwrap();

        fs.enable_qgroup(subvol_id).unwrap();
        // Limit referenced size to 50 bytes
        fs.set_qgroup_limit(subvol_id, Some(50), None).unwrap();

        // Writing 30 bytes should succeed
        let data1 = [0u8; 30];
        assert!(fs.write_data(subvol_id, 0, &data1).is_ok());

        // Writing another 30 bytes (total 60) should exceed the 50-byte limit and fail
        let data2 = [0u8; 30];
        assert_eq!(
            fs.write_data(subvol_id, 30, &data2),
            Err("Quota limit (referenced) exceeded")
        );
    }

    #[test]
    fn test_btrfs_scrub_and_self_healing() {
        let mut fs = BtrfsFilesystem::new();
        let subvol_id = fs
            .create_subvolume("secure_subvol".to_string(), None)
            .unwrap();

        // Configure as RAID1 with multiple devices for redundancy
        fs.set_raid_profile(RaidProfile::Raid1);
        fs.add_device("/dev/sdb1".to_string(), 1024 * 1024 * 1024); // Device 2

        let data = b"Important distribution configuration data";
        let offset = 4096;
        let extent_id = fs.write_data(subvol_id, offset, data).unwrap();

        // Data should be readable originally
        assert!(fs.read_data(extent_id).is_ok());

        // Simulate bit-rot corruption on the physical medium
        fs.corrupt_extent(extent_id).unwrap();

        // Reading should now fail because checksum verification fails
        assert_eq!(
            fs.read_data(extent_id),
            Err("Input/output error (checksum verification failed)")
        );

        // Run Btrfs background scrubbing - should find the error and heal from RAID1 mirror copy
        let scrub_res = fs.scrub().unwrap();
        assert_eq!(scrub_res.extents_scrubbed, 1);
        assert_eq!(scrub_res.errors_found, 1);
        assert_eq!(scrub_res.errors_healed, 1);

        // Verification: Data should be completely repaired and readable again!
        assert!(fs.read_data(extent_id).is_ok());
    }

    #[test]
    fn test_btrfs_compression_heuristics() {
        let mut fs = BtrfsFilesystem::new();
        let subvol_id = fs
            .create_subvolume("compressed_subvol".to_string(), None)
            .unwrap();

        // 1. High-entropy data (unique unique random-like bytes) -> compression should be skipped
        let mut high_entropy = [0u8; 100];
        for i in 0..100 {
            high_entropy[i] = i as u8;
        }
        let extent_id_high = fs.write_data(subvol_id, 0, &high_entropy).unwrap();
        let extent_high = fs.get_extent(extent_id_high).unwrap();
        assert_eq!(extent_high.compression, CompressionType::None);

        // 2. Low-entropy data (highly repetitive zeroes) -> compression should be applied
        let low_entropy = [0u8; 100];
        let extent_id_low = fs.write_data(subvol_id, 200, &low_entropy).unwrap();
        let extent_low = fs.get_extent(extent_id_low).unwrap();
        assert_eq!(extent_low.compression, CompressionType::Zlib); // Default Zlib active
    }

    #[test]
    fn test_btrfs_send_receive_incremental() {
        let mut fs = BtrfsFilesystem::new();
        let subvol_id = fs
            .create_subvolume("origin_subvol".to_string(), None)
            .unwrap();

        // Write sample configuration and file layers
        let data1 = b"Debian/Fedora mirror repositories mapping database configuration file";
        fs.write_data(subvol_id, 0, data1).unwrap();
        let data2 = b"Custom systemd service init rules for enterprise cloud profile";
        fs.write_data(subvol_id, 1024, data2).unwrap();

        // Generate btrfs send stream (Snapper incremental backup mechanism)
        let stream = fs.send_subvolume(subvol_id).unwrap();
        assert_eq!(stream.len(), 3); // CreateSubvolume + 2 WriteExtents

        // Reconstruct the subvolume on an independent backup filesystem
        let mut backup_fs = BtrfsFilesystem::new();
        let restored_id = backup_fs.receive_subvolume(&stream).unwrap();

        assert_eq!(restored_id, subvol_id);
        assert_eq!(backup_fs.subvolume_count(), 1);
        assert_eq!(
            backup_fs.get_subvolume(restored_id).unwrap().name,
            "origin_subvol"
        );

        // Verify content integrity of restored subvolume
        let restored_extent1 = backup_fs.get_extent(0).unwrap();
        let restored_extent2 = backup_fs.get_extent(1024).unwrap();
        assert_eq!(restored_extent1.length, data1.len() as u64);
        assert_eq!(restored_extent2.length, data2.len() as u64);
    }

    #[test]
    fn test_btrfs_auto_defragmentation() {
        let mut fs = BtrfsFilesystem::new();
        let subvol_id = fs
            .create_subvolume("fragmented_db".to_string(), None)
            .unwrap();

        // Simulate heavily fragmented small random database writes (common issue in Btrfs CoW layout)
        fs.write_data(subvol_id, 0, b"part1").unwrap();
        fs.write_data(subvol_id, 5, b"part2").unwrap();
        fs.write_data(subvol_id, 10, b"part3").unwrap();

        // 3 fragmented extents should be tracked
        let extents_before = fs.extents.len();
        assert_eq!(extents_before, 3);

        // Run autodefrag to coalesce fragmented adjacent sectors into larger continuous blocks
        let merged_count = fs.defragment_subvolume(subvol_id).unwrap();
        assert_eq!(merged_count, 2); // 3 small writes should coalesce down to 1 big merged extent

        let extents_after = fs.extents.len();
        assert_eq!(extents_after, 1);

        // Merged extent should span the full block length (15 bytes total)
        let merged_extent = fs.get_extent(0).unwrap();
        assert_eq!(merged_extent.length, 15);
    }

    #[test]
    fn test_btrfs_mount_options_and_force_compression() {
        let mut fs = BtrfsFilesystem::new();
        fs.mount_options.compress_force = true; // Fedora default override option

        let subvol_id = fs
            .create_subvolume("force_compress_subvol".to_string(), None)
            .unwrap();

        // Highly random random-like data (would normally fail the entropy heuristic and skip compression)
        let mut high_entropy = [0u8; 128];
        for i in 0..128 {
            high_entropy[i] = i as u8;
        }

        let extent_id = fs.write_data(subvol_id, 0, &high_entropy).unwrap();
        let extent = fs.get_extent(extent_id).unwrap();

        // Due to compress_force, compression MUST be applied regardless of the data entropy!
        assert_eq!(extent.compression, CompressionType::Zlib);
    }

    #[test]
    fn test_btrfs_property_inheritance() {
        let mut fs = BtrfsFilesystem::new();
        let parent_id = fs
            .create_subvolume("parent_home".to_string(), None)
            .unwrap();
        fs.set_readonly(parent_id, true).unwrap();
        fs.set_compression(parent_id, CompressionType::Zstd)
            .unwrap();

        fs.enable_qgroup(parent_id).unwrap();
        fs.set_qgroup_limit(parent_id, Some(1024), Some(2048))
            .unwrap();

        // Create child subvolume with property inheritance (openSUSE layout style)
        let child_id = fs
            .create_subvolume_with_inheritance(
                "parent_home/user_nested".to_string(),
                Some(parent_id),
            )
            .unwrap();

        let child = fs.get_subvolume(child_id).unwrap();
        assert!(child.readonly);
        assert_eq!(child.compression, CompressionType::Zstd);

        // Qgroup limits should be seamlessly inherited from parent
        let child_qgroup = fs.get_qgroup(child_id).unwrap();
        assert_eq!(child_qgroup.limit_referenced, Some(1024));
        assert_eq!(child_qgroup.limit_exclusive, Some(2048));
    }

    #[test]
    fn test_btrfs_async_discard_trim() {
        let mut fs = BtrfsFilesystem::new();
        fs.mount_options.discard_async = true;

        // Perform some writes to consume pool space
        let subvol_id = fs.create_subvolume("root_os".to_string(), None).unwrap();
        fs.write_data(subvol_id, 0, &[0u8; 2048]).unwrap();

        // Run background async discard TRIM space reclamation (standard modern distro SSD setup)
        let trimmed = fs.trigger_async_discard().unwrap();
        assert!(trimmed > 0);
    }
}
