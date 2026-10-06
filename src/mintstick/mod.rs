// Mintstick-inspired USB Flasher
// USB stick formatting and bootable image writing tool

use std::fmt;

/// Filesystem types for USB formatting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemType {
    Fat32,
    ExFAT,
    NTFS,
    Ext4,
}

impl fmt::Display for FilesystemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FilesystemType::Fat32 => write!(f, "FAT32"),
            FilesystemType::ExFAT => write!(f, "exFAT"),
            FilesystemType::NTFS => write!(f, "NTFS"),
            FilesystemType::Ext4 => write!(f, "ext4"),
        }
    }
}

/// USB device information
#[derive(Debug, Clone)]
pub struct UsbDevice {
    pub device_path: String,
    pub model: String,
    pub size_bytes: u64,
    pub is_removable: bool,
}

impl UsbDevice {
    pub fn new(device_path: String, model: String, size_bytes: u64, is_removable: bool) -> Self {
        Self {
            device_path,
            model,
            size_bytes,
            is_removable,
        }
    }

    /// Format size in human-readable format
    pub fn size_human(&self) -> String {
        const GB: u64 = 1024 * 1024 * 1024;
        const MB: u64 = 1024 * 1024;
        
        if self.size_bytes >= GB {
            format!("{:.2} GB", self.size_bytes as f64 / GB as f64)
        } else {
            format!("{:.2} MB", self.size_bytes as f64 / MB as f64)
        }
    }
}

/// Image write operation
#[derive(Debug, Clone)]
pub struct ImageWriteOperation {
    pub image_path: String,
    pub device_path: String,
    pub image_size_bytes: u64,
    pub progress: u8,
    pub verify_checksum: bool,
}

impl ImageWriteOperation {
    pub fn new(image_path: String, device_path: String, image_size_bytes: u64) -> Self {
        Self {
            image_path,
            device_path,
            image_size_bytes,
            progress: 0,
            verify_checksum: true,
        }
    }

    /// Set verification enabled/disabled
    pub fn with_verification(mut self, verify: bool) -> Self {
        self.verify_checksum = verify;
        self
    }
}

/// Format operation
#[derive(Debug, Clone)]
pub struct FormatOperation {
    pub device_path: String,
    pub filesystem: FilesystemType,
    pub volume_label: Option<String>,
    pub progress: u8,
}

impl FormatOperation {
    pub fn new(device_path: String, filesystem: FilesystemType) -> Self {
        Self {
            device_path,
            filesystem,
            volume_label: None,
            progress: 0,
        }
    }

    /// Set volume label
    pub fn with_label(mut self, label: String) -> Self {
        self.volume_label = Some(label);
        self
    }
}

/// Checksum information
#[derive(Debug, Clone)]
pub struct ChecksumInfo {
    pub algorithm: String,
    pub expected: String,
    pub computed: Option<String>,
    pub valid: bool,
}

impl ChecksumInfo {
    pub fn new(algorithm: String, expected: String) -> Self {
        Self {
            algorithm,
            expected,
            computed: None,
            valid: false,
        }
    }

    /// Set computed checksum
    pub fn with_computed(mut self, computed: String) -> Self {
        self.computed = Some(computed.clone());
        self.valid = computed == self.expected;
        self
    }
}

/// Mintstick USB flasher manager
pub struct MintstickManager {
    devices: Vec<UsbDevice>,
    write_operations: Vec<ImageWriteOperation>,
    format_operations: Vec<FormatOperation>,
}

impl MintstickManager {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            write_operations: Vec::new(),
            format_operations: Vec::new(),
        }
    }

    /// Scan for USB devices
    pub fn scan_devices(&mut self) -> Vec<UsbDevice> {
        // In a real implementation, this would scan /sys/block or use udev
        // For now, return mock devices
        self.devices = vec![
            UsbDevice::new("/dev/sdb".to_string(), "SanDisk Cruzer".to_string(), 16 * 1024 * 1024 * 1024, true),
            UsbDevice::new("/dev/sdc".to_string(), "Kingston DataTraveler".to_string(), 32 * 1024 * 1024 * 1024, true),
        ];
        self.devices.clone()
    }

    /// Get removable USB devices
    pub fn get_removable_devices(&self) -> Vec<UsbDevice> {
        self.devices.iter().filter(|d| d.is_removable).cloned().collect()
    }

    /// Get device by path
    pub fn get_device(&self, path: &str) -> Option<UsbDevice> {
        self.devices.iter().find(|d| d.device_path == path).cloned()
    }

    /// Start image write operation
    pub fn write_image(&mut self, operation: ImageWriteOperation) -> Result<(), String> {
        // Validate device exists
        if !self.devices.iter().any(|d| d.device_path == operation.device_path) {
            return Err(format!("Device {} not found", operation.device_path));
        }

        // In a real implementation, this would use dd or similar
        self.write_operations.push(operation);
        Ok(())
    }

    /// Start format operation
    pub fn format_device(&mut self, operation: FormatOperation) -> Result<(), String> {
        // Validate device exists
        if !self.devices.iter().any(|d| d.device_path == operation.device_path) {
            return Err(format!("Device {} not found", operation.device_path));
        }

        // In a real implementation, this would use mkfs or similar
        self.format_operations.push(operation);
        Ok(())
    }

    /// Calculate SHA-256 checksum
    pub fn calculate_checksum(&self, _image_path: &str) -> Result<ChecksumInfo, String> {
        // In a real implementation, this would use sha256sum
        Ok(ChecksumInfo::new("SHA-256".to_string(), "mock_checksum".to_string()))
    }

    /// Verify image checksum
    pub fn verify_checksum(&self, image_path: &str, _expected: &str) -> Result<bool, String> {
        let checksum = self.calculate_checksum(image_path)?;
        Ok(checksum.valid)
    }

    /// Get write operations
    pub fn get_write_operations(&self) -> Vec<ImageWriteOperation> {
        self.write_operations.clone()
    }

    /// Get format operations
    pub fn get_format_operations(&self) -> Vec<FormatOperation> {
        self.format_operations.clone()
    }

    /// Cancel write operation
    pub fn cancel_write(&mut self, device_path: &str) -> bool {
        if let Some(pos) = self.write_operations.iter().position(|op| op.device_path == device_path) {
            self.write_operations.remove(pos);
            true
        } else {
            false
        }
    }

    /// Cancel format operation
    pub fn cancel_format(&mut self, device_path: &str) -> bool {
        if let Some(pos) = self.format_operations.iter().position(|op| op.device_path == device_path) {
            self.format_operations.remove(pos);
            true
        } else {
            false
        }
    }
}

impl Default for MintstickManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mintstick_manager_creation() {
        let manager = MintstickManager::new();
        assert_eq!(manager.devices.len(), 0);
    }

    #[test]
    fn test_scan_devices() {
        let mut manager = MintstickManager::new();
        let devices = manager.scan_devices();
        assert_eq!(devices.len(), 2);
        assert!(devices[0].is_removable);
    }

    #[test]
    fn test_get_removable_devices() {
        let mut manager = MintstickManager::new();
        manager.scan_devices();
        let removable = manager.get_removable_devices();
        assert_eq!(removable.len(), 2);
    }

    #[test]
    fn test_get_device() {
        let mut manager = MintstickManager::new();
        manager.scan_devices();
        let device = manager.get_device("/dev/sdb");
        assert!(device.is_some());
        assert_eq!(device.unwrap().model, "SanDisk Cruzer");
    }

    #[test]
    fn test_usb_device_size_human() {
        let device = UsbDevice::new("/dev/sdb".to_string(), "Test".to_string(), 16 * 1024 * 1024 * 1024, true);
        assert_eq!(device.size_human(), "16.00 GB");
    }

    #[test]
    fn test_filesystem_display() {
        assert_eq!(FilesystemType::Fat32.to_string(), "FAT32");
        assert_eq!(FilesystemType::ExFAT.to_string(), "exFAT");
        assert_eq!(FilesystemType::NTFS.to_string(), "NTFS");
        assert_eq!(FilesystemType::Ext4.to_string(), "ext4");
    }

    #[test]
    fn test_image_write_operation() {
        let operation = ImageWriteOperation::new(
            "/path/to/image.iso".to_string(),
            "/dev/sdb".to_string(),
            1024 * 1024 * 1024,
        );
        assert_eq!(operation.image_path, "/path/to/image.iso");
        assert!(operation.verify_checksum);
    }

    #[test]
    fn test_image_write_operation_with_verification() {
        let operation = ImageWriteOperation::new(
            "/path/to/image.iso".to_string(),
            "/dev/sdb".to_string(),
            1024 * 1024 * 1024,
        ).with_verification(false);
        assert!(!operation.verify_checksum);
    }

    #[test]
    fn test_format_operation() {
        let operation = FormatOperation::new("/dev/sdb".to_string(), FilesystemType::Fat32);
        assert_eq!(operation.device_path, "/dev/sdb");
        assert_eq!(operation.filesystem, FilesystemType::Fat32);
        assert!(operation.volume_label.is_none());
    }

    #[test]
    fn test_format_operation_with_label() {
        let operation = FormatOperation::new("/dev/sdb".to_string(), FilesystemType::Fat32)
            .with_label("MYUSB".to_string());
        assert_eq!(operation.volume_label, Some("MYUSB".to_string()));
    }

    #[test]
    fn test_write_image() {
        let mut manager = MintstickManager::new();
        manager.scan_devices();
        let operation = ImageWriteOperation::new(
            "/path/to/image.iso".to_string(),
            "/dev/sdb".to_string(),
            1024 * 1024 * 1024,
        );
        let result = manager.write_image(operation);
        assert!(result.is_ok());
        assert_eq!(manager.write_operations.len(), 1);
    }

    #[test]
    fn test_write_image_invalid_device() {
        let mut manager = MintstickManager::new();
        manager.scan_devices();
        let operation = ImageWriteOperation::new(
            "/path/to/image.iso".to_string(),
            "/dev/sdx".to_string(),
            1024 * 1024 * 1024,
        );
        let result = manager.write_image(operation);
        assert!(result.is_err());
    }

    #[test]
    fn test_format_device() {
        let mut manager = MintstickManager::new();
        manager.scan_devices();
        let operation = FormatOperation::new("/dev/sdb".to_string(), FilesystemType::Fat32);
        let result = manager.format_device(operation);
        assert!(result.is_ok());
        assert_eq!(manager.format_operations.len(), 1);
    }

    #[test]
    fn test_format_device_invalid() {
        let mut manager = MintstickManager::new();
        manager.scan_devices();
        let operation = FormatOperation::new("/dev/sdx".to_string(), FilesystemType::Fat32);
        let result = manager.format_device(operation);
        assert!(result.is_err());
    }

    #[test]
    fn test_checksum_info() {
        let checksum = ChecksumInfo::new("SHA-256".to_string(), "abc123".to_string());
        assert_eq!(checksum.algorithm, "SHA-256");
        assert!(!checksum.valid);
    }

    #[test]
    fn test_checksum_info_with_computed() {
        let checksum = ChecksumInfo::new("SHA-256".to_string(), "abc123".to_string())
            .with_computed("abc123".to_string());
        assert!(checksum.valid);
    }

    #[test]
    fn test_checksum_info_invalid() {
        let checksum = ChecksumInfo::new("SHA-256".to_string(), "abc123".to_string())
            .with_computed("wrong".to_string());
        assert!(!checksum.valid);
    }

    #[test]
    fn test_cancel_write() {
        let mut manager = MintstickManager::new();
        manager.scan_devices();
        let operation = ImageWriteOperation::new(
            "/path/to/image.iso".to_string(),
            "/dev/sdb".to_string(),
            1024 * 1024 * 1024,
        );
        manager.write_image(operation).unwrap();
        let cancelled = manager.cancel_write("/dev/sdb");
        assert!(cancelled);
        assert_eq!(manager.write_operations.len(), 0);
    }

    #[test]
    fn test_cancel_write_not_found() {
        let mut manager = MintstickManager::new();
        let cancelled = manager.cancel_write("/dev/sdb");
        assert!(!cancelled);
    }

    #[test]
    fn test_cancel_format() {
        let mut manager = MintstickManager::new();
        manager.scan_devices();
        let operation = FormatOperation::new("/dev/sdb".to_string(), FilesystemType::Fat32);
        manager.format_device(operation).unwrap();
        let cancelled = manager.cancel_format("/dev/sdb");
        assert!(cancelled);
        assert_eq!(manager.format_operations.len(), 0);
    }
}
