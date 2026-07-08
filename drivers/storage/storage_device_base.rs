// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/storage/storage_device_base.rs — Base Device Trait for Storage Drivers
//
// Defines the OOP base class for all storage devices using Rust traits.
// This provides a common interface for storage operations.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ─── Storage Error Codes ─────────────────────────────────────────────────────

pub const STORAGE_OK: I32 = 0;
pub const STORAGE_ERR_NO_DEVICE: I32 = -1;
pub const STORAGE_ERR_INIT_FAILED: I32 = -2;
pub const STORAGE_ERR_OUT_OF_MEM: I32 = -3;
pub const STORAGE_ERR_NOT_SUPPORTED: I32 = -4;
pub const STORAGE_ERR_IO: I32 = -5;
pub const STORAGE_ERR_TIMEOUT: I32 = -6;
pub const STORAGE_ERR_MEDIA_ERROR: I32 = -7;
pub const STORAGE_ERR_INVALID_PARAM: I32 = -8;

// ─── Storage Device Types ───────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StorageType {
    NVMe,
    SATA,
    AHCI,
    SCSI,
    IDE,
    MMC,
    SD,
    Virtual,
}

// ─── Storage Protocol ───────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StorageProtocol {
    NVMe,
    ATA,
    ATAPI,
    SCSI,
    SD,
    MMC,
}

// ─── Storage Geometry ─────────────────────────────────────────────────

#[repr(C)]
pub struct StorageGeometry {
    pub cylinders: U32,
    pub heads: U32,
    pub sectors_per_track: U32,
    pub total_sectors: U64,
    pub sector_size: U32,
}

impl StorageGeometry {
    pub const fn new() -> Self {
        StorageGeometry {
            cylinders: 0,
            heads: 0,
            sectors_per_track: 0,
            total_sectors: 0,
            sector_size: 512,
        }
    }
}

// ─── Storage Identify Data ─────────────────────────────────────────────

#[repr(C)]
pub struct StorageIdentify {
    pub model_number: [U8; 40],
    pub serial_number: [U8; 20],
    pub firmware_revision: [U8; 8],
    pub capacity: U64,
    pub sector_size: U32,
    pub max_lba: U64,
}

impl StorageIdentify {
    pub const fn new() -> Self {
        StorageIdentify {
            model_number: [0; 40],
            serial_number: [0; 20],
            firmware_revision: [0; 8],
            capacity: 0,
            sector_size: 512,
            max_lba: 0,
        }
    }
}

// ─── Storage Request ─────────────────────────────────────────────────

#[repr(C)]
pub struct StorageRequest {
    pub lba: U64,
    pub sector_count: U32,
    pub buffer: *mut U8,
    pub direction: StorageDirection,
    pub complete: bool,
    pub status: I32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StorageDirection {
    Read,
    Write,
    Flush,
}

impl StorageRequest {
    pub const fn new() -> Self {
        StorageRequest {
            lba: 0,
            sector_count: 0,
            buffer: 0 as *mut U8,
            direction: StorageDirection::Read,
            complete: false,
            status: STORAGE_OK,
        }
    }
}

// ─── Storage Device Trait ─────────────────────────────────────────────

/// Trait for storage device operations
pub trait StorageDevice {
    /// Initialize the storage device
    fn init(&mut self, pci_bar: U64, device_id: U16) -> I32;
    
    /// Check if device is initialized
    fn is_initialized(&self) -> bool;
    
    /// Get device name
    fn get_device_name(&self) -> &'static str;
    
    /// Get storage type
    fn get_storage_type(&self) -> StorageType;
    
    /// Get storage protocol
    fn get_storage_protocol(&self) -> StorageProtocol;
    
    /// Get device geometry
    fn get_geometry(&self) -> StorageGeometry;
    
    /// Get identify data
    fn get_identify(&self, identify: *mut StorageIdentify) -> I32;
    
    /// Read sectors
    fn read(&mut self, lba: U64, buffer: *mut U8, sector_count: U32) -> I32;
    
    /// Write sectors
    fn write(&mut self, lba: U64, buffer: *const U8, sector_count: U32) -> I32;
    
    /// Submit storage request
    fn submit_request(&mut self, request: *mut StorageRequest) -> I32;
    
    /// Cancel request
    fn cancel_request(&mut self, request: *mut StorageRequest) -> I32;
    
    /// Flush cache
    fn flush(&mut self) -> I32;
    
    /// Get capacity
    fn get_capacity(&self) -> U64;
    
    /// Get sector size
    fn get_sector_size(&self) -> U32;
    
    /// Reset the device
    fn reset(&mut self) -> I32;
    
    /// Shutdown the device
    fn shutdown(&mut self) -> I32;
}

// ─── Storage Statistics ─────────────────────────────────────────────

#[repr(C)]
pub struct StorageStats {
    pub reads: U64,
    pub writes: U64,
    pub read_bytes: U64,
    pub write_bytes: U64,
    pub read_errors: U64,
    pub write_errors: U64,
    pub flushes: U64,
}

impl StorageStats {
    pub const fn new() -> Self {
        StorageStats {
            reads: 0,
            writes: 0,
            read_bytes: 0,
            write_bytes: 0,
            read_errors: 0,
            write_errors: 0,
            flushes: 0,
        }
    }
}
