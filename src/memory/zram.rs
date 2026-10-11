// SPDX-License-Identifier: MIT
// SigmaOS - ZRAM Compressed RAM Swap
// Inspired by Linux ZRAM (drivers/block/zram/zram_drv.c)
// Provides compressed RAM-based swap device for improved performance

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, AtomicBool, AtomicU32, Ordering};

/// Compression algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionAlgorithm {
    LZO,
    LZ4,
    ZSTD,
    Deflate,
}

/// ZRAM device configuration
#[derive(Debug, Clone)]
pub struct ZramConfig {
    pub device_id: u32,
    pub disk_size: u64, // bytes
    pub max_comp_streams: u32,
    pub comp_algorithm: CompressionAlgorithm,
    pub mem_limit: u64, // bytes
    pub writeback_limit: u64, // bytes
}

impl Default for ZramConfig {
    fn default() -> Self {
        Self {
            device_id: 0,
            disk_size: 512 * 1024 * 1024, // 512MB default
            max_comp_streams: 4,
            comp_algorithm: CompressionAlgorithm::LZ4,
            mem_limit: 0, // No limit
            writeback_limit: 0, // No limit
        }
    }
}

/// ZRAM device statistics
#[derive(Debug, Clone)]
pub struct ZramStats {
    pub disk_size: u64,
    pub orig_data_size: u64,
    pub compr_data_size: u64,
    pub mem_used_total: u64,
    pub num_reads: u64,
    pub num_writes: u64,
    pub failed_reads: u64,
    pub failed_writes: u64,
    pub invalid_io: u64,
    pub notify_free: u64,
    pub zero_pages: u64,
}

impl Default for ZramStats {
    fn default() -> Self {
        Self {
            disk_size: 0,
            orig_data_size: 0,
            compr_data_size: 0,
            mem_used_total: 0,
            num_reads: 0,
            num_writes: 0,
            failed_reads: 0,
            failed_writes: 0,
            invalid_io: 0,
            notify_free: 0,
            zero_pages: 0,
        }
    }
}

/// Compressed page
#[derive(Debug, Clone)]
pub struct CompressedPage {
    pub index: u64,
    pub original_size: u32,
    pub compressed_size: u32,
    pub is_zero: bool,
    pub data: Vec<u8>,
}

/// ZRAM device
pub struct ZramDevice {
    pub config: ZramConfig,
    pub stats: ZramStats,
    pub pages: BTreeMap<u64, CompressedPage>,
    pub active: AtomicBool,
    pub backing_device: Option<String>,
}

impl ZramDevice {
    pub fn new(config: ZramConfig) -> Self {
        let disk_size = config.disk_size;
        Self {
            config,
            stats: ZramStats {
                disk_size,
                ..Default::default()
            },
            pages: BTreeMap::new(),
            active: AtomicBool::new(false),
            backing_device: None,
        }
    }
    
    /// Initialize ZRAM device
    pub fn init(&mut self) -> Result<(), &'static str> {
        self.active.store(true, Ordering::Relaxed);
        Ok(())
    }
    
    /// Reset ZRAM device
    pub fn reset(&mut self) {
        self.pages.clear();
        self.stats = ZramStats {
            disk_size: self.config.disk_size,
            ..Default::default()
        };
    }
    
    /// Write page to ZRAM
    pub fn write_page(&mut self, index: u64, data: &[u8]) -> Result<(), &'static str> {
        if !self.active.load(Ordering::Relaxed) {
            return Err("ZRAM device not active");
        }
        
        // Check if page is all zeros
        let is_zero = data.iter().all(|&b| b == 0);
        
        if is_zero {
            self.stats.zero_pages += 1;
            self.pages.insert(index, CompressedPage {
                index,
                original_size: data.len() as u32,
                compressed_size: 0,
                is_zero: true,
                data: Vec::new(),
            });
        } else {
            // Compress data (simulated - in real implementation would use actual compression)
            let compressed_data = self.compress(data);
            let compression_ratio = compressed_data.len() as f64 / data.len() as f64;
            
            self.pages.insert(index, CompressedPage {
                index,
                original_size: data.len() as u32,
                compressed_size: compressed_data.len() as u32,
                is_zero: false,
                data: compressed_data,
            });
            
            self.stats.orig_data_size += data.len() as u64;
            self.stats.compr_data_size += compressed_data.len() as u64;
        }
        
        self.stats.num_writes += 1;
        Ok(())
    }
    
    /// Read page from ZRAM
    pub fn read_page(&self, index: u64) -> Result<Vec<u8>, &'static str> {
        if !self.active.load(Ordering::Relaxed) {
            return Err("ZRAM device not active");
        }
        
        let page = self.pages.get(&index).ok_or("Page not found")?;
        
        if page.is_zero {
            // Return zero-filled page
            Ok(vec![0u8; page.original_size as usize])
        } else {
            // Decompress data (simulated)
            Ok(self.decompress(&page.data, page.original_size as usize))
        }
    }
    
    /// Compress data (simulated - in real implementation would use actual compression library)
    fn compress(&self, data: &[u8]) -> Vec<u8> {
        // Simulate LZ4 compression with ~50% ratio
        let compressed_size = (data.len() + 1) / 2;
        let mut compressed = Vec::with_capacity(compressed_size);
        for i in (0..data.len()).step_by(2) {
            compressed.push(data[i]);
            if i + 1 < data.len() {
                compressed.push(data[i + 1]);
            }
        }
        compressed
    }
    
    /// Decompress data (simulated)
    fn decompress(&self, data: &[u8], original_size: usize) -> Vec<u8> {
        // Simulate LZ4 decompression
        let mut decompressed = Vec::with_capacity(original_size);
        for &byte in data {
            decompressed.push(byte);
            if decompressed.len() < original_size {
                decompressed.push(byte);
            }
        }
        decompressed.resize(original_size, 0);
        decompressed
    }
    
    /// Get compression ratio
    pub fn get_compression_ratio(&self) -> f64 {
        if self.stats.orig_data_size == 0 {
            0.0
        } else {
            self.stats.compr_data_size as f64 / self.stats.orig_data_size as f64
        }
    }
    
    /// Get memory used
    pub fn get_memory_used(&self) -> u64 {
        self.stats.compr_data_size
    }
    
    /// Set disk size
    pub fn set_disk_size(&mut self, size: u64) {
        self.config.disk_size = size;
        self.stats.disk_size = size;
    }
    
    /// Set compression algorithm
    pub fn set_compression_algorithm(&mut self, algo: CompressionAlgorithm) {
        self.config.comp_algorithm = algo;
    }
    
    /// Set memory limit
    pub fn set_mem_limit(&mut self, limit: u64) {
        self.config.mem_limit = limit;
    }
    
    /// Get stats
    pub fn get_stats(&self) -> &ZramStats {
        &self.stats
    }
    
    /// Check if active
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }
}

/// ZRAM manager for multiple devices
pub struct ZramManager {
    pub devices: BTreeMap<u32, ZramDevice>,
    pub next_device_id: AtomicU32,
}

impl ZramManager {
    pub fn new() -> Self {
        Self {
            devices: BTreeMap::new(),
            next_device_id: AtomicU32::new(0),
        }
    }
    
    /// Create ZRAM device
    pub fn create_device(&mut self, config: ZramConfig) -> u32 {
        let device_id = self.next_device_id.fetch_add(1, Ordering::Relaxed);
        let mut config = config;
        config.device_id = device_id;
        let device = ZramDevice::new(config);
        self.devices.insert(device_id, device);
        device_id
    }
    
    /// Get device by ID
    pub fn get_device(&self, device_id: u32) -> Option<&ZramDevice> {
        self.devices.get(&device_id)
    }
    
    /// Get mutable device by ID
    pub fn get_device_mut(&mut self, device_id: u32) -> Option<&mut ZramDevice> {
        self.devices.get_mut(&device_id)
    }
    
    /// Destroy device
    pub fn destroy_device(&mut self, device_id: u32) -> Result<(), &'static str> {
        self.devices.remove(&device_id).ok_or("Device not found")?;
        Ok(())
    }
    
    /// Get total memory used by all devices
    pub fn get_total_memory_used(&self) -> u64 {
        self.devices.values().map(|d| d.get_memory_used()).sum()
    }
    
    /// Get total compression ratio
    pub fn get_total_compression_ratio(&self) -> f64 {
        let total_orig: u64 = self.devices.values().map(|d| d.stats.orig_data_size).sum();
        let total_compr: u64 = self.devices.values().map(|d| d.stats.compr_data_size).sum();
        
        if total_orig == 0 {
            0.0
        } else {
            total_compr as f64 / total_orig as f64
        }
    }
}

impl Default for ZramManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global ZRAM manager
static mut ZRAM_MANAGER: Option<ZramManager> = None;

/// Initialize ZRAM manager
pub fn init_zram_manager() {
    unsafe {
        ZRAM_MANAGER = Some(ZramManager::new());
    }
}

/// Get global ZRAM manager
pub fn get_zram_manager() -> Option<&'static mut ZramManager> {
    unsafe {
        ZRAM_MANAGER.as_mut()
    }
}

/// Create default ZRAM device
pub fn create_default_zram() -> Result<u32, &'static str> {
    if let Some(manager) = get_zram_manager() {
        let config = ZramConfig::default();
        let device_id = manager.create_device(config);
        manager.get_device_mut(device_id).unwrap().init()?;
        Ok(device_id)
    } else {
        Err("ZRAM manager not initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_zram_config_default() {
        let config = ZramConfig::default();
        assert_eq!(config.disk_size, 512 * 1024 * 1024);
        assert_eq!(config.comp_algorithm, CompressionAlgorithm::LZ4);
    }
    
    #[test]
    fn test_zram_device_init() {
        let config = ZramConfig::default();
        let mut device = ZramDevice::new(config);
        assert!(!device.is_active());
        device.init().unwrap();
        assert!(device.is_active());
    }
    
    #[test]
    fn test_zram_write_page() {
        let config = ZramConfig::default();
        let mut device = ZramDevice::new(config);
        device.init().unwrap();
        let data = vec![1u8, 2, 3, 4, 5];
        device.write_page(0, &data).unwrap();
        assert_eq!(device.stats.num_writes, 1);
    }
    
    #[test]
    fn test_zram_read_page() {
        let config = ZramConfig::default();
        let mut device = ZramDevice::new(config);
        device.init().unwrap();
        let data = vec![1u8, 2, 3, 4, 5];
        device.write_page(0, &data).unwrap();
        let read_data = device.read_page(0).unwrap();
        assert_eq!(read_data.len(), data.len());
    }
    
    #[test]
    fn test_zram_zero_page() {
        let config = ZramConfig::default();
        let mut device = ZramDevice::new(config);
        device.init().unwrap();
        let data = vec![0u8; 4096];
        device.write_page(0, &data).unwrap();
        assert_eq!(device.stats.zero_pages, 1);
        let read_data = device.read_page(0).unwrap();
        assert!(read_data.iter().all(|&b| b == 0));
    }
    
    #[test]
    fn test_zram_compression_ratio() {
        let config = ZramConfig::default();
        let mut device = ZramDevice::new(config);
        device.init().unwrap();
        let data = vec![1u8; 4096];
        device.write_page(0, &data).unwrap();
        let ratio = device.get_compression_ratio();
        assert!(ratio > 0.0 && ratio < 1.0);
    }
    
    #[test]
    fn test_zram_reset() {
        let config = ZramConfig::default();
        let mut device = ZramDevice::new(config);
        device.init().unwrap();
        device.write_page(0, &[1, 2, 3]).unwrap();
        device.reset();
        assert_eq!(device.pages.len(), 0);
        assert_eq!(device.stats.num_writes, 0);
    }
    
    #[test]
    fn test_zram_manager() {
        let mut manager = ZramManager::new();
        let config = ZramConfig::default();
        let device_id = manager.create_device(config);
        assert!(manager.get_device(device_id).is_some());
    }
    
    #[test]
    fn test_zram_manager_multiple_devices() {
        let mut manager = ZramManager::new();
        let config = ZramConfig::default();
        let id1 = manager.create_device(config.clone());
        let id2 = manager.create_device(config);
        assert!(id1 != id2);
        assert_eq!(manager.devices.len(), 2);
    }
    
    #[test]
    fn test_zram_total_memory_used() {
        let mut manager = ZramManager::new();
        let config = ZramConfig::default();
        let id1 = manager.create_device(config.clone());
        let id2 = manager.create_device(config);
        manager.get_device_mut(id1).unwrap().init().unwrap();
        manager.get_device_mut(id2).unwrap().init().unwrap();
        manager.get_device_mut(id1).unwrap().write_page(0, &[1, 2, 3]).unwrap();
        manager.get_device_mut(id2).unwrap().write_page(0, &[4, 5, 6]).unwrap();
        let total = manager.get_total_memory_used();
        assert!(total > 0);
    }
}
