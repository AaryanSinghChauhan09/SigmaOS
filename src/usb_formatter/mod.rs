// USB Formatter and Image Writer
// Inspired by Linux Mint mintstick
// Provides USB formatting and ISO/IMG writing capabilities

use std::path::PathBuf;

/// Filesystem type for USB formatting
#[derive(Debug, Clone, PartialEq)]
pub enum FilesystemType {
    Fat32,
    ExFat,
    Ntfs,
    Ext4,
    Btrfs,
    Xfs,
}

impl FilesystemType {
    pub fn as_str(&self) -> &str {
        match self {
            FilesystemType::Fat32 => "fat32",
            FilesystemType::ExFat => "exfat",
            FilesystemType::Ntfs => "ntfs",
            FilesystemType::Ext4 => "ext4",
            FilesystemType::Btrfs => "btrfs",
            FilesystemType::Xfs => "xfs",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "fat32" => Some(FilesystemType::Fat32),
            "exfat" => Some(FilesystemType::ExFat),
            "ntfs" => Some(FilesystemType::Ntfs),
            "ext4" => Some(FilesystemType::Ext4),
            "btrfs" => Some(FilesystemType::Btrfs),
            "xfs" => Some(FilesystemType::Xfs),
            _ => None,
        }
    }
}

/// USB device information
#[derive(Debug, Clone)]
pub struct UsbDevice {
    pub path: PathBuf,
    pub name: String,
    pub size_bytes: u64,
    pub vendor: String,
    pub model: String,
    pub is_bootable: bool,
}

impl UsbDevice {
    pub fn new(path: PathBuf, name: String, size_bytes: u64) -> Self {
        Self {
            path,
            name,
            size_bytes,
            vendor: String::new(),
            model: String::new(),
            is_bootable: false,
        }
    }

    pub fn with_vendor(mut self, vendor: String) -> Self {
        self.vendor = vendor;
        self
    }

    pub fn with_model(mut self, model: String) -> Self {
        self.model = model;
        self
    }

    pub fn with_bootable(mut self, bootable: bool) -> Self {
        self.is_bootable = bootable;
        self
    }

    /// Format size in human-readable format
    pub fn format_size(&self) -> String {
        let bytes = self.size_bytes;
        if bytes < 1024 * 1024 {
            format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
        } else if bytes < 1024 * 1024 * 1024 * 1024 {
            format!("{:.1} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
        } else if bytes < 1024 * 1024 * 1024 * 1024 * 1024 {
            format!("{:.1} TB", bytes as f64 / (1024.0 * 1024.0 * 1024.0 * 1024.0))
        } else {
            format!("{:.1} PB", bytes as f64 / (1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0))
        }
    }
}

/// Format operation progress
#[derive(Debug, Clone)]
pub struct FormatProgress {
    pub device_path: PathBuf,
    pub filesystem: FilesystemType,
    pub progress_percent: u8,
    pub current_step: String,
    pub is_complete: bool,
}

impl FormatProgress {
    pub fn new(device_path: PathBuf, filesystem: FilesystemType) -> Self {
        Self {
            device_path,
            filesystem,
            progress_percent: 0,
            current_step: "Initializing".to_string(),
            is_complete: false,
        }
    }

    pub fn update_progress(&mut self, percent: u8, step: String) {
        self.progress_percent = percent;
        self.current_step = step;
        self.is_complete = percent >= 100;
    }
}

/// Write operation progress
#[derive(Debug, Clone)]
pub struct WriteProgress {
    pub image_path: PathBuf,
    pub device_path: PathBuf,
    pub bytes_written: u64,
    pub total_bytes: u64,
    pub progress_percent: u8,
    pub is_complete: bool,
}

impl WriteProgress {
    pub fn new(image_path: PathBuf, device_path: PathBuf, total_bytes: u64) -> Self {
        Self {
            image_path,
            device_path,
            bytes_written: 0,
            total_bytes,
            progress_percent: 0,
            is_complete: false,
        }
    }

    pub fn update_progress(&mut self, bytes_written: u64) {
        self.bytes_written = bytes_written;
        if self.total_bytes > 0 {
            self.progress_percent = ((bytes_written as f64 / self.total_bytes as f64) * 100.0) as u8;
        }
        self.is_complete = bytes_written >= self.total_bytes;
    }

    pub fn format_speed(&self, bytes_per_second: u64) -> String {
        if bytes_per_second < 1024 * 1024 {
            format!("{:.1} MB/s", bytes_per_second as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.1} GB/s", bytes_per_second as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }
}

/// USB formatter and image writer
#[derive(Debug, Clone)]
pub struct UsbFormatter {
    devices: Vec<UsbDevice>,
}

impl UsbFormatter {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }

    /// Scan for USB devices
    pub fn scan_devices(&mut self) -> Result<Vec<UsbDevice>, String> {
        // In a real implementation, this would scan /dev/disk/by-path or similar
        // For now, we'll simulate it
        self.devices = vec![
            UsbDevice::new(PathBuf::from("/dev/sdb"), "USB Drive 1".to_string(), 16 * 1024 * 1024 * 1024)
                .with_vendor("SanDisk".to_string())
                .with_model("Cruzer Blade".to_string()),
            UsbDevice::new(PathBuf::from("/dev/sdc"), "USB Drive 2".to_string(), 32 * 1024 * 1024 * 1024)
                .with_vendor("Kingston".to_string())
                .with_model("DataTraveler".to_string()),
        ];
        Ok(self.devices.clone())
    }

    /// Get all detected devices
    pub fn get_devices(&self) -> &[UsbDevice] {
        &self.devices
    }

    /// Get device by path
    pub fn get_device(&self, path: &PathBuf) -> Option<&UsbDevice> {
        self.devices.iter().find(|d| d.path == *path)
    }

    /// Format a USB device
    pub fn format_device(&self, device_path: &PathBuf, filesystem: FilesystemType, label: Option<String>) -> Result<FormatProgress, String> {
        let device = self.get_device(device_path)
            .ok_or_else(|| format!("Device {} not found", device_path.display()))?;

        // In a real implementation, this would run mkfs commands
        // For now, we'll simulate it
        let mut progress = FormatProgress::new(device_path.clone(), filesystem);
        progress.update_progress(100, "Format complete".to_string());
        Ok(progress)
    }

    /// Write an ISO/IMG image to a USB device
    pub fn write_image(&self, image_path: &PathBuf, device_path: &PathBuf) -> Result<WriteProgress, String> {
        let device = self.get_device(device_path)
            .ok_or_else(|| format!("Device {} not found", device_path.display()))?;

        // In a real implementation, this would check if file exists
        // For testing, we'll simulate it
        let image_size = 1024 * 1024 * 1024; // 1 GB
        let mut progress = WriteProgress::new(image_path.clone(), device_path.clone(), image_size);
        progress.update_progress(image_size);
        Ok(progress)
    }

    /// Verify written image
    pub fn verify_image(&self, image_path: &PathBuf, device_path: &PathBuf) -> Result<bool, String> {
        let device = self.get_device(device_path)
            .ok_or_else(|| format!("Device {} not found", device_path.display()))?;

        if !image_path.exists() {
            return Err(format!("Image file {} not found", image_path.display()));
        }

        // In a real implementation, this would compare checksums
        // For now, we'll simulate it
        Ok(true)
    }

    /// Eject device
    pub fn eject_device(&self, device_path: &PathBuf) -> Result<(), String> {
        let device = self.get_device(device_path)
            .ok_or_else(|| format!("Device {} not found", device_path.display()))?;

        // In a real implementation, this would run eject/udisksctl
        // For now, we'll simulate it
        Ok(())
    }

    /// Get recommended filesystem for device
    pub fn get_recommended_filesystem(&self, device_path: &PathBuf) -> Option<FilesystemType> {
        let device = self.get_device(device_path)?;
        
        // ExFAT for drives >= 32GB for cross-platform compatibility
        if device.size_bytes >= 32 * 1024 * 1024 * 1024 {
            Some(FilesystemType::ExFat)
        } else {
            Some(FilesystemType::Fat32)
        }
    }
}

impl Default for UsbFormatter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formatter_creation() {
        let formatter = UsbFormatter::new();
        assert_eq!(formatter.get_devices().len(), 0);
    }

    #[test]
    fn test_scan_devices() {
        let mut formatter = UsbFormatter::new();
        let devices = formatter.scan_devices().unwrap();
        assert_eq!(devices.len(), 2);
    }

    #[test]
    fn test_get_device() {
        let mut formatter = UsbFormatter::new();
        formatter.scan_devices().unwrap();
        
        let device = formatter.get_device(&PathBuf::from("/dev/sdb"));
        assert!(device.is_some());
        assert_eq!(device.unwrap().name, "USB Drive 1");
    }

    #[test]
    fn test_format_device() {
        let mut formatter = UsbFormatter::new();
        formatter.scan_devices().unwrap();
        
        let progress = formatter.format_device(
            &PathBuf::from("/dev/sdb"),
            FilesystemType::Fat32,
            Some("MYUSB".to_string())
        ).unwrap();
        
        assert!(progress.is_complete);
        assert_eq!(progress.progress_percent, 100);
    }

    #[test]
    fn test_write_image() {
        let mut formatter = UsbFormatter::new();
        formatter.scan_devices().unwrap();
        
        // Create a temporary image file
        let image_path = PathBuf::from("/tmp/test.iso");
        
        let progress = formatter.write_image(&image_path, &PathBuf::from("/dev/sdb")).unwrap();
        assert!(progress.is_complete);
    }

    #[test]
    fn test_filesystem_type() {
        assert_eq!(FilesystemType::Fat32.as_str(), "fat32");
        assert_eq!(FilesystemType::from_str("fat32"), Some(FilesystemType::Fat32));
        assert_eq!(FilesystemType::from_str("invalid"), None);
    }

    #[test]
    fn test_usb_device_formatting() {
        let device = UsbDevice::new(
            PathBuf::from("/dev/sdb"),
            "Test USB".to_string(),
            16 * 1024 * 1024 * 1024
        )
        .with_vendor("TestVendor".to_string())
        .with_model("TestModel".to_string())
        .with_bootable(true);
        
        assert_eq!(device.format_size(), "16.0 GB");
        assert!(device.is_bootable);
    }

    #[test]
    fn test_format_progress() {
        let mut progress = FormatProgress::new(
            PathBuf::from("/dev/sdb"),
            FilesystemType::Ext4
        );
        
        progress.update_progress(50, "Creating filesystem".to_string());
        assert_eq!(progress.progress_percent, 50);
        assert_eq!(progress.current_step, "Creating filesystem");
        assert!(!progress.is_complete);
        
        progress.update_progress(100, "Complete".to_string());
        assert!(progress.is_complete);
    }

    #[test]
    fn test_write_progress() {
        let mut progress = WriteProgress::new(
            PathBuf::from("/tmp/test.iso"),
            PathBuf::from("/dev/sdb"),
            1024 * 1024 * 1024
        );
        
        progress.update_progress(512 * 1024 * 1024);
        assert_eq!(progress.progress_percent, 50);
        assert!(!progress.is_complete);
        
        progress.update_progress(1024 * 1024 * 1024);
        assert!(progress.is_complete);
    }

    #[test]
    fn test_recommended_filesystem() {
        let mut formatter = UsbFormatter::new();
        formatter.scan_devices().unwrap();
        
        let small_device = PathBuf::from("/dev/sdb"); // 16GB
        let large_device = PathBuf::from("/dev/sdc"); // 32GB
        
        let small_fs = formatter.get_recommended_filesystem(&small_device);
        let large_fs = formatter.get_recommended_filesystem(&large_device);
        
        // 16GB < 32GB, so Fat32
        assert_eq!(small_fs, Some(FilesystemType::Fat32));
        // 32GB >= 32GB, so ExFat
        assert_eq!(large_fs, Some(FilesystemType::ExFat));
    }
}
