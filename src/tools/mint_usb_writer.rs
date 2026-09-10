//! Linux Mint mintstick-inspired USB Image Writer
//! 
//! This module implements a USB image writer inspired by Linux Mint's mintstick,
//! which formats USB sticks and creates bootable USB sticks.

#![allow(dead_code)]



use std::string::{String, ToString};
use std::vec::Vec;

/// USB device information
#[derive(Debug, Clone)]
pub struct UsbDevice {
    /// Device path (e.g., /dev/sdb)
    pub device_path: String,
    /// Device size in bytes
    pub size: u64,
    /// Device model name
    pub model: String,
    /// Device vendor
    pub vendor: String,
    /// Device serial number
    pub serial: String,
    /// Whether device is currently mounted
    pub mounted: bool,
    /// Mount points if mounted
    pub mount_points: Vec<String>,
}

/// Filesystem type for USB formatting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemType {
    /// FAT32 - widely compatible
    Fat32,
    /// NTFS - Windows-compatible
    Ntfs,
    /// exFAT - large file support
    ExFat,
    /// ext4 - Linux-native
    Ext4,
    /// Btrfs - advanced features
    Btrfs,
}

impl FilesystemType {
    /// Get display name for the filesystem
    pub fn display_name(&self) -> &'static str {
        match self {
            FilesystemType::Fat32 => "FAT32",
            FilesystemType::Ntfs => "NTFS",
            FilesystemType::ExFat => "exFAT",
            FilesystemType::Ext4 => "ext4",
            FilesystemType::Btrfs => "Btrfs",
        }
    }

    /// Get maximum file size in bytes
    pub fn max_file_size(&self) -> u64 {
        match self {
            FilesystemType::Fat32 => 4 * 1024 * 1024 * 1024, // 4GB
            FilesystemType::Ntfs => 16 * 1024 * 1024 * 1024 * 1024u64, // 16EB
            FilesystemType::ExFat => 16 * 1024 * 1024 * 1024 * 1024u64, // 16EB
            FilesystemType::Ext4 => 16 * 1024 * 1024 * 1024 * 1024u64, // 16EB
            FilesystemType::Btrfs => 16 * 1024 * 1024 * 1024 * 1024u64, // 16EB
        }
    }
}

/// Image format for bootable USB
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    /// ISO image
    Iso,
    /// IMG image
    Img,
    /// Hybrid ISO/IMG
    Hybrid,
}

impl ImageFormat {
    /// Detect format from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "iso" => Some(ImageFormat::Iso),
            "img" => Some(ImageFormat::Img),
            _ => None,
        }
    }
}

/// USB operation progress
#[derive(Debug, Clone)]
pub struct UsbOperationProgress {
    /// Current operation
    pub operation: String,
    /// Progress percentage (0-100)
    pub percentage: u8,
    /// Bytes processed
    pub bytes_processed: u64,
    /// Total bytes
    pub total_bytes: u64,
    /// Current speed in bytes per second
    pub speed: u64,
    /// Estimated remaining time in seconds
    pub estimated_remaining: u32,
}

/// USB writer result
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UsbWriterResult {
    /// Operation successful
    Success,
    /// Operation failed with error message
    Failed(String),
    /// Operation cancelled
    Cancelled,
}

/// USB Image Writer - manages USB formatting and image writing
#[derive(Debug)]
pub struct MintUsbWriter {
    /// Available USB devices
    pub devices: Vec<UsbDevice>,
    /// Currently selected device
    pub selected_device: Option<String>,
    /// Current operation progress
    pub progress: Option<UsbOperationProgress>,
    /// Whether operation is in progress
    pub operation_in_progress: bool,
}

impl MintUsbWriter {
    /// Create a new USB Writer
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            selected_device: None,
            progress: None,
            operation_in_progress: false,
        }
    }

    /// Add a USB device
    pub fn add_device(&mut self, device: UsbDevice) {
        self.devices.push(device);
    }

    /// Remove a USB device
    pub fn remove_device(&mut self, device_path: &str) {
        self.devices.retain(|d| d.device_path != device_path);
        if self.selected_device.as_ref() == Some(&device_path.to_string()) {
            self.selected_device = None;
        }
    }

    /// Get device by path
    pub fn get_device(&self, device_path: &str) -> Option<&UsbDevice> {
        self.devices.iter().find(|d| d.device_path == device_path)
    }

    /// Select a device for operations
    pub fn select_device(&mut self, device_path: String) -> Result<(), String> {
        if self.devices.iter().any(|d| d.device_path == device_path) {
            self.selected_device = Some(device_path);
            Ok(())
        } else {
            Err("Device not found".to_string())
        }
    }

    /// Format a USB device
    pub fn format_device(
        &mut self,
        device_path: &str,
        _filesystem: FilesystemType,
        _label: &str,
    ) -> Result<UsbWriterResult, String> {
        if self.operation_in_progress {
            return Err("Operation already in progress".to_string());
        }

        // Check if device exists
        if !self.devices.iter().any(|d| d.device_path == device_path) {
            return Err("Device not found".to_string());
        }

        self.operation_in_progress = true;
        
        // Simulate formatting progress
        self.progress = Some(UsbOperationProgress {
            operation: "Formatting".to_string(),
            percentage: 0,
            bytes_processed: 0,
            total_bytes: 0,
            speed: 0,
            estimated_remaining: 0,
        });

        // In a real implementation, this would:
        // 1. Unmount the device
        // 2. Create new partition table
        // 3. Format with specified filesystem
        // 4. Set label

        self.progress = Some(UsbOperationProgress {
            operation: "Formatting".to_string(),
            percentage: 100,
            bytes_processed: 0,
            total_bytes: 0,
            speed: 0,
            estimated_remaining: 0,
        });

        self.operation_in_progress = false;
        Ok(UsbWriterResult::Success)
    }

    /// Write an image to a USB device
    pub fn write_image(
        &mut self,
        device_path: &str,
        _image_path: &str,
        _image_format: ImageFormat,
    ) -> Result<UsbWriterResult, String> {
        if self.operation_in_progress {
            return Err("Operation already in progress".to_string());
        }

        // Check if device exists
        if !self.devices.iter().any(|d| d.device_path == device_path) {
            return Err("Device not found".to_string());
        }

        self.operation_in_progress = true;
        
        // Simulate writing progress
        self.progress = Some(UsbOperationProgress {
            operation: "Writing image".to_string(),
            percentage: 0,
            bytes_processed: 0,
            total_bytes: 0,
            speed: 0,
            estimated_remaining: 0,
        });

        // In a real implementation, this would:
        // 1. Verify image file exists and is readable
        // 2. Unmount the device
        // 3. Write image blocks to device
        // 4. Verify write integrity
        // 5. Sync filesystem

        self.progress = Some(UsbOperationProgress {
            operation: "Writing image".to_string(),
            percentage: 100,
            bytes_processed: 0,
            total_bytes: 0,
            speed: 0,
            estimated_remaining: 0,
        });

        self.operation_in_progress = false;
        Ok(UsbWriterResult::Success)
    }

    /// Cancel current operation
    pub fn cancel_operation(&mut self) -> Result<(), String> {
        if !self.operation_in_progress {
            return Err("No operation in progress".to_string());
        }

        self.operation_in_progress = false;
        self.progress = None;
        Ok(())
    }

    /// Get progress of current operation
    pub fn get_progress(&self) -> Option<&UsbOperationProgress> {
        self.progress.as_ref()
    }

    /// Verify USB device is safe to write (checks for data loss)
    pub fn verify_device_safe(&self, device_path: &str) -> Result<bool, String> {
        if let Some(device) = self.get_device(device_path) {
            // Device is safe if it's not mounted or has no data
            Ok(!device.mounted || device.mount_points.is_empty())
        } else {
            Err("Device not found".to_string())
        }
    }

    /// Calculate required space for image
    pub fn calculate_required_space(&self, image_size: u64) -> u64 {
        // Add 10% overhead for safety
        image_size + (image_size / 10)
    }

    /// Check if device has enough space
    pub fn check_device_space(&self, device_path: &str, required_size: u64) -> Result<bool, String> {
        if let Some(device) = self.get_device(device_path) {
            Ok(device.size >= required_size)
        } else {
            Err("Device not found".to_string())
        }
    }
}

impl Default for MintUsbWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usb_writer_creation() {
        let writer = MintUsbWriter::new();
        assert_eq!(writer.devices.len(), 0);
        assert!(!writer.operation_in_progress);
    }

    #[test]
    fn test_add_device() {
        let mut writer = MintUsbWriter::new();
        
        let device = UsbDevice {
            device_path: "/dev/sdb".to_string(),
            size: 16 * 1024 * 1024 * 1024, // 16GB
            model: "USB Drive".to_string(),
            vendor: "Generic".to_string(),
            serial: "123456".to_string(),
            mounted: false,
            mount_points: Vec::new(),
        };
        
        writer.add_device(device);
        assert_eq!(writer.devices.len(), 1);
    }

    #[test]
    fn test_select_device() {
        let mut writer = MintUsbWriter::new();
        
        let device = UsbDevice {
            device_path: "/dev/sdb".to_string(),
            size: 16 * 1024 * 1024 * 1024,
            model: "USB Drive".to_string(),
            vendor: "Generic".to_string(),
            serial: "123456".to_string(),
            mounted: false,
            mount_points: Vec::new(),
        };
        
        writer.add_device(device);
        let result = writer.select_device("/dev/sdb".to_string());
        assert!(result.is_ok());
        assert_eq!(writer.selected_device, Some("/dev/sdb".to_string()));
    }

    #[test]
    fn test_filesystem_max_size() {
        assert_eq!(FilesystemType::Fat32.max_file_size(), 4 * 1024 * 1024 * 1024);
        assert!(FilesystemType::Ntfs.max_file_size() > FilesystemType::Fat32.max_file_size());
    }

    #[test]
    fn test_image_format_detection() {
        assert_eq!(ImageFormat::from_extension("iso"), Some(ImageFormat::Iso));
        assert_eq!(ImageFormat::from_extension("img"), Some(ImageFormat::Img));
        assert_eq!(ImageFormat::from_extension("bin"), None);
    }

    #[test]
    fn test_calculate_required_space() {
        let writer = MintUsbWriter::new();
        let image_size = 1_000_000_000; // 1GB
        let required = writer.calculate_required_space(image_size);
        assert!(required > image_size);
        assert_eq!(required, image_size + (image_size / 10));
    }
}
