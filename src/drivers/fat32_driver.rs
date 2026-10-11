//! # FAT32 / exFAT Block Device Driver Binding
//!
//! Provides the block-level device driver binding for FAT32 and exFAT filesystems
//! in SigmaOS. Interfaces directly with block devices (ATA, NVMe, USB mass storage,
//! virtio-blk) to load, parse, cache, and mount FAT32/exFAT partitions.

#![allow(dead_code)]

extern crate alloc;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;
use std::sync::Mutex;

use crate::filesystem::fat32::{Fat32Bpb, ExFatBootSector, Fat32Error};

/// Generic block device abstraction for FAT filesystem mounting.
pub trait BlockDevice: Send + Sync {
    /// Read one or more sectors starting at `lba`.
    fn read_sectors(&self, lba: u64, buf: &mut [u8]) -> Result<(), DriverError>;
    /// Write one or more sectors starting at `lba`.
    fn write_sectors(&self, lba: u64, buf: &[u8]) -> Result<(), DriverError>;
    /// Total sector count.
    fn sector_count(&self) -> u64;
    /// Sector size in bytes (usually 512 or 4096).
    fn sector_size(&self) -> u32;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverError {
    DeviceError,
    InvalidSector,
    IoTimeout,
    ReadOnlyDevice,
    CorruptedBpb,
    UnsupportedSectorSize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemKind {
    Fat32,
    ExFat,
    Unknown,
}

/// In-memory sector cache (LRU-like buffer).
pub struct SectorCache {
    pub cached_lba: Option<u64>,
    pub buffer: Vec<u8>,
}

impl SectorCache {
    pub fn new(sector_size: usize) -> Self {
        Self {
            cached_lba: None,
            buffer: alloc::vec![0u8; sector_size],
        }
    }
}

/// Driver state managing a mounted FAT partition on a block device.
pub struct Fat32DeviceDriver {
    pub device: Arc<dyn BlockDevice>,
    pub partition_start_lba: u64,
    pub bpb: Option<Fat32Bpb>,
    pub exfat: Option<ExFatBootSector>,
    pub kind: FilesystemKind,
    pub cache: Mutex<SectorCache>,
}

impl Fat32DeviceDriver {
    /// Probe the block device at `start_lba` to detect FAT32 or exFAT.
    pub fn probe(device: Arc<dyn BlockDevice>, start_lba: u64) -> Result<Self, DriverError> {
        let sec_size = device.sector_size() as usize;
        let mut boot_sector = alloc::vec![0u8; sec_size.max(512)];
        device.read_sectors(start_lba, &mut boot_sector[..512])?;

        if boot_sector[510] != 0x55 || boot_sector[511] != 0xAA {
            return Err(DriverError::CorruptedBpb);
        }

        let mut fixed_512 = [0u8; 512];
        fixed_512.copy_from_slice(&boot_sector[..512]);

        let (kind, bpb, exfat) = if let Some(b) = Fat32Bpb::from_bytes(&fixed_512) {
            (FilesystemKind::Fat32, Some(b), None)
        } else if let Some(e) = ExFatBootSector::from_bytes(&fixed_512) {
            (FilesystemKind::ExFat, None, Some(e))
        } else {
            return Err(DriverError::CorruptedBpb);
        };

        let cache = Mutex::new(SectorCache::new(sec_size));

        Ok(Self {
            device,
            partition_start_lba: start_lba,
            bpb,
            exfat,
            kind,
            cache,
        })
    }

    /// Read cluster data into buffer for FAT32.
    pub fn read_cluster(&self, cluster: u32, buf: &mut [u8]) -> Result<(), DriverError> {
        let bpb = self.bpb.as_ref().ok_or(DriverError::CorruptedBpb)?;
        let sectors_per_cluster = bpb.sectors_per_cluster as u64;
        let cluster_start_lba = self.partition_start_lba
            + bpb.first_data_sector() as u64
            + ((cluster.saturating_sub(2)) as u64 * sectors_per_cluster);

        let sec_size = self.device.sector_size() as usize;
        let total_bytes = (sectors_per_cluster as usize) * sec_size;
        if buf.len() < total_bytes {
            return Err(DriverError::DeviceError);
        }

        self.device.read_sectors(cluster_start_lba, &mut buf[..total_bytes])
    }

    /// Write cluster data from buffer for FAT32.
    pub fn write_cluster(&self, cluster: u32, buf: &[u8]) -> Result<(), DriverError> {
        let bpb = self.bpb.as_ref().ok_or(DriverError::CorruptedBpb)?;
        let sectors_per_cluster = bpb.sectors_per_cluster as u64;
        let cluster_start_lba = self.partition_start_lba
            + bpb.first_data_sector() as u64
            + ((cluster.saturating_sub(2)) as u64 * sectors_per_cluster);

        let sec_size = self.device.sector_size() as usize;
        let total_bytes = (sectors_per_cluster as usize) * sec_size;
        if buf.len() < total_bytes {
            return Err(DriverError::DeviceError);
        }

        self.device.write_sectors(cluster_start_lba, &buf[..total_bytes])
    }
}
