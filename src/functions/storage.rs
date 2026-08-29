//! Storage Management Functions (lsblk/parted Inspiration)
//! Block device management, partition manager, and filesystem tools
use alloc::format;
extern crate alloc;



use alloc::vec::Vec;
use alloc::string::{String, ToString};

/// Block device
#[derive(Debug, Clone)]
pub struct BlockDevice {
    pub name: String,
    pub size: u64,
    pub model: String,
    pub serial: String,
    pub removable: bool,
}

impl BlockDevice {
    pub fn new(name: &str, size: u64) -> Self {
        Self {
            name: name.to_string(),
            size,
            model: String::new(),
            serial: String::new(),
            removable: false,
        }
    }

    pub fn set_model(&mut self, model: &str) {
        self.model = model.to_string();
    }

    pub fn set_serial(&mut self, serial: &str) {
        self.serial = serial.to_string();
    }
}

/// Partition
#[derive(Debug, Clone)]
pub struct Partition {
    pub name: String,
    pub device: String,
    pub size: u64,
    pub start: u64,
    pub partition_type: PartitionType,
    pub filesystem: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitionType {
    Primary,
    Logical,
    Extended,
}

impl Partition {
    pub fn new(name: &str, device: &str, partition_type: PartitionType) -> Self {
        Self {
            name: name.to_string(),
            device: device.to_string(),
            size: 0,
            start: 0,
            partition_type,
            filesystem: None,
        }
    }

    pub fn set_filesystem(&mut self, fs: &str) {
        self.filesystem = Some(fs.to_string());
    }
}

/// Filesystem info
#[derive(Debug, Clone)]
pub struct FilesystemInfo {
    pub device: String,
    pub mount_point: String,
    pub fs_type: String,
    pub size: u64,
    pub used: u64,
    pub available: u64,
}

impl FilesystemInfo {
    pub fn new(device: &str, mount_point: &str, fs_type: &str) -> Self {
        Self {
            device: device.to_string(),
            mount_point: mount_point.to_string(),
            fs_type: fs_type.to_string(),
            size: 0,
            used: 0,
            available: 0,
        }
    }

    pub fn get_usage_percentage(&self) -> f64 {
        if self.size == 0 {
            0.0
        } else {
            (self.used as f64 / self.size as f64) * 100.0
        }
    }
}

/// Block device manager
pub struct BlockDeviceManager {
    pub devices: Vec<BlockDevice>,
    pub partitions: Vec<Partition>,
    pub filesystems: Vec<FilesystemInfo>,
}

impl BlockDeviceManager {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            partitions: Vec::new(),
            filesystems: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device: BlockDevice) {
        self.devices.push(device);
    }

    pub fn add_partition(&mut self, partition: Partition) {
        self.partitions.push(partition);
    }

    pub fn add_filesystem(&mut self, fs: FilesystemInfo) {
        self.filesystems.push(fs);
    }

    pub fn get_partitions_by_device(&self, device: &str) -> Vec<&Partition> {
        self.partitions.iter().filter(|p| p.device == device).collect()
    }

    pub fn get_device_stats(&self) -> DeviceStats {
        DeviceStats {
            total_devices: self.devices.len(),
            total_partitions: self.partitions.len(),
            total_filesystems: self.filesystems.len(),
            total_size: self.devices.iter().map(|d| d.size).sum(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeviceStats {
    pub total_devices: usize,
    pub total_partitions: usize,
    pub total_filesystems: usize,
    pub total_size: u64,
}

/// Partition table type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitionTableType {
    GPT,
    MBR,
}

/// Disk
#[derive(Debug, Clone)]
pub struct Disk {
    pub name: String,
    pub size: u64,
    pub partition_table: Option<PartitionTableType>,
}

impl Disk {
    pub fn new(name: &str, size: u64) -> Self {
        Self {
            name: name.to_string(),
            size,
            partition_table: None,
        }
    }

    pub fn set_partition_table(&mut self, table_type: PartitionTableType) {
        self.partition_table = Some(table_type);
    }
}

/// Partition manager
pub struct PartitionManager {
    pub disks: Vec<Disk>,
    pub partition_tables: Vec<PartitionTable>,
    pub partitions: Vec<Partition>,
}

#[derive(Debug, Clone)]
pub struct PartitionTable {
    pub disk: String,
    pub table_type: PartitionTableType,
    pub partitions: Vec<Partition>,
}

impl PartitionManager {
    pub fn new() -> Self {
        Self {
            disks: Vec::new(),
            partition_tables: Vec::new(),
            partitions: Vec::new(),
        }
    }

    pub fn add_disk(&mut self, disk: Disk) {
        self.disks.push(disk);
    }

    pub fn create_partition_table(&mut self, disk: &str, table_type: PartitionTableType) -> Result<(), StorageError> {
        let table = PartitionTable {
            disk: disk.to_string(),
            table_type,
            partitions: Vec::new(),
        };
        self.partition_tables.push(table);
        Ok(())
    }

    pub fn create_partition(&mut self, disk: &str, partition_type: PartitionType, size: u64) -> Result<String, StorageError> {
        let partition = Partition::new(&format!("{}p1", disk.split('/').last()), disk, partition_type);
        partition.size = size;
        let id = partition.name.clone();
        self.partitions.push(partition);
        Ok(id)
    }

    pub fn delete_partition(&mut self, partition_name: &str) -> Result<(), StorageError> {
        self.partitions.retain(|p| p.name != partition_name);
        Ok(())
    }
}

/// Filesystem manager
pub struct FilesystemManager {
    pub filesystems: Vec<Filesystem>,
    pub mount_points: Vec<MountPoint>,
}

#[derive(Debug, Clone)]
pub struct Filesystem {
    pub device: String,
    pub fs_type: String,
    pub size: u64,
}

#[derive(Debug, Clone)]
pub struct MountPoint {
    pub device: String,
    pub mount_point: String,
    pub options: Vec<String>,
}

impl FilesystemManager {
    pub fn new() -> Self {
        Self {
            filesystems: Vec::new(),
            mount_points: Vec::new(),
        }
    }

    pub fn create_filesystem(&mut self, device: &str, fs_type: &str) -> Result<(), StorageError> {
        let fs = Filesystem {
            device: device.to_string(),
            fs_type: fs_type.to_string(),
            size: 0,
        };
        self.filesystems.push(fs);
        Ok(())
    }

    pub fn mount(&mut self, device: &str, mount_point: &str, options: Vec<String>) -> Result<(), StorageError> {
        let mount = MountPoint {
            device: device.to_string(),
            mount_point: mount_point.to_string(),
            options,
        };
        self.mount_points.push(mount);
        Ok(())
    }

    pub fn unmount(&mut self, mount_point: &str) -> Result<(), StorageError> {
        self.mount_points.retain(|m| m.mount_point != mount_point);
        Ok(())
    }

    pub fn resize_filesystem(&mut self, device: &str, new_size: u64) -> Result<(), StorageError> {
        // Resize filesystem
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorageError {
    DeviceNotFound,
    PartitionNotFound,
    FilesystemNotFound,
    CreateFailed,
    MountFailed,
    ResizeFailed,
}

impl Default for BlockDeviceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for PartitionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for FilesystemManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_device() {
        let device = BlockDevice::new("/dev/sda", 1024000);
        assert_eq!(device.name, "/dev/sda");
    }

    #[test]
    fn test_partition() {
        let partition = Partition::new("sda1", "/dev/sda", PartitionType::Primary);
        assert_eq!(partition.partition_type, PartitionType::Primary);
    }

    #[test]
    fn test_block_device_manager() {
        let mut manager = BlockDeviceManager::new();
        let device = BlockDevice::new("/dev/sda", 1024000);
        manager.add_device(device);
        assert_eq!(manager.devices.len(), 1);
    }

    #[test]
    fn test_partition_manager() {
        let mut manager = PartitionManager::new();
        let disk = Disk::new("/dev/sda", 1024000);
        manager.add_disk(disk);
        assert_eq!(manager.disks.len(), 1);
    }

    #[test]
    fn test_filesystem_manager() {
        let mut manager = FilesystemManager::new();
        assert!(manager.create_filesystem("/dev/sda1", "ext4").is_ok());
    }
}