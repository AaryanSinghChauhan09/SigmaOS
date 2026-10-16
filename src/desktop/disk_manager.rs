// SPDX-License-Identifier: MIT
// SigmaOS - Disk Manager
// Inspired by GNOME Disks and GParted
// Provides disk and partition management

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Partition type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitionType {
    Primary,
    Logical,
    Extended,
}

/// File system type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileSystemType {
    Ext4,
    Btrfs,
    XFS,
    NTFS,
    FAT32,
    Swap,
    None,
}

/// Partition
#[derive(Debug, Clone)]
pub struct Partition {
    pub number: u32,
    pub name: String,
    pub partition_type: PartitionType,
    pub file_system: FileSystemType,
    pub start_sector: u64,
    pub end_sector: u64,
    pub size: u64,
    pub used: u64,
    pub mount_point: Option<String>,
    pub is_mounted: bool,
}

/// Disk
#[derive(Debug, Clone)]
pub struct Disk {
    pub name: String,
    pub device: String,
    pub model: String,
    pub size: u64,
    pub sector_size: u32,
    pub is_removable: bool,
    pub is_rotational: bool,
    pub partitions: Vec<Partition>,
}

/// Disk operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiskOperation {
    CreatePartition,
    DeletePartition,
    FormatPartition,
    ResizePartition,
    MountPartition,
    UnmountPartition,
}

/// Operation status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

/// Disk operation task
#[derive(Debug, Clone)]
pub struct DiskOperationTask {
    pub operation: DiskOperation,
    pub disk_name: String,
    pub partition_number: Option<u32>,
    pub status: OperationStatus,
    pub progress: u32,
    pub error_message: Option<String>,
}

/// Disk manager
pub struct DiskManager {
    pub disks: Vec<Disk>,
    pub operations: Vec<DiskOperationTask>,
    pub current_operation: Option<usize>,
}

impl DiskManager {
    pub fn new() -> Self {
        Self {
            disks: Vec::new(),
            operations: Vec::new(),
            current_operation: None,
        }
    }

    /// Add disk
    pub fn add_disk(&mut self, disk: Disk) {
        self.disks.push(disk);
    }

    /// Remove disk
    pub fn remove_disk(&mut self, name: String) -> Result<(), &'static str> {
        let index = self
            .disks
            .iter()
            .position(|d| d.name == name)
            .ok_or("Disk not found")?;
        self.disks.remove(index);
        Ok(())
    }

    /// Get disk
    pub fn get_disk(&self, name: &str) -> Option<&Disk> {
        self.disks.iter().find(|d| d.name == name)
    }

    /// Get all disks
    pub fn get_disks(&self) -> &[Disk] {
        &self.disks
    }

    /// Create partition
    pub fn create_partition(
        &mut self,
        disk_name: String,
        partition_type: PartitionType,
        file_system: FileSystemType,
        size: u64,
    ) -> Result<(), &'static str> {
        let disk = self
            .disks
            .iter_mut()
            .find(|d| d.name == disk_name)
            .ok_or("Disk not found")?;
        let partition_number = disk.partitions.len() as u32 + 1;
        let partition = Partition {
            number: partition_number,
            name: format!("{}{}", disk_name, partition_number),
            partition_type,
            file_system,
            start_sector: 0,
            end_sector: size / disk.sector_size as u64,
            size,
            used: 0,
            mount_point: None,
            is_mounted: false,
        };
        disk.partitions.push(partition);
        Ok(())
    }

    /// Delete partition
    pub fn delete_partition(
        &mut self,
        disk_name: String,
        partition_number: u32,
    ) -> Result<(), &'static str> {
        let disk = self
            .disks
            .iter_mut()
            .find(|d| d.name == disk_name)
            .ok_or("Disk not found")?;
        let index = disk
            .partitions
            .iter()
            .position(|p| p.number == partition_number)
            .ok_or("Partition not found")?;
        disk.partitions.remove(index);
        Ok(())
    }

    /// Format partition
    pub fn format_partition(
        &mut self,
        disk_name: String,
        partition_number: u32,
        file_system: FileSystemType,
    ) -> Result<(), &'static str> {
        let disk = self
            .disks
            .iter_mut()
            .find(|d| d.name == disk_name)
            .ok_or("Disk not found")?;
        let partition = disk
            .partitions
            .iter_mut()
            .find(|p| p.number == partition_number)
            .ok_or("Partition not found")?;
        partition.file_system = file_system;
        partition.used = 0;
        Ok(())
    }

    /// Mount partition
    pub fn mount_partition(
        &mut self,
        disk_name: String,
        partition_number: u32,
        mount_point: String,
    ) -> Result<(), &'static str> {
        let disk = self
            .disks
            .iter_mut()
            .find(|d| d.name == disk_name)
            .ok_or("Disk not found")?;
        let partition = disk
            .partitions
            .iter_mut()
            .find(|p| p.number == partition_number)
            .ok_or("Partition not found")?;
        partition.mount_point = Some(mount_point);
        partition.is_mounted = true;
        Ok(())
    }

    /// Unmount partition
    pub fn unmount_partition(
        &mut self,
        disk_name: String,
        partition_number: u32,
    ) -> Result<(), &'static str> {
        let disk = self
            .disks
            .iter_mut()
            .find(|d| d.name == disk_name)
            .ok_or("Disk not found")?;
        let partition = disk
            .partitions
            .iter_mut()
            .find(|p| p.number == partition_number)
            .ok_or("Partition not found")?;
        partition.mount_point = None;
        partition.is_mounted = false;
        Ok(())
    }

    /// Add operation
    pub fn add_operation(&mut self, operation: DiskOperationTask) {
        self.operations.push(operation);
    }

    /// Get operations
    pub fn get_operations(&self) -> &[DiskOperationTask] {
        &self.operations
    }

    /// Clear operations
    pub fn clear_operations(&mut self) {
        self.operations.clear();
    }

    /// Disk count
    pub fn disk_count(&self) -> usize {
        self.disks.len()
    }

    /// Format size
    pub fn format_size(size: u64) -> String {
        const GB: u64 = 1024 * 1024 * 1024;
        const MB: u64 = 1024 * 1024;
        const KB: u64 = 1024;

        if size >= GB {
            format!("{} GB", size / GB)
        } else if size >= MB {
            format!("{} MB", size / MB)
        } else if size >= KB {
            format!("{} KB", size / KB)
        } else {
            format!("{} B", size)
        }
    }
}

/// Global disk manager
static mut DISK_MANAGER: Option<DiskManager> = None;

/// Initialize disk manager
pub fn init_disk_manager() {
    unsafe {
        DISK_MANAGER = Some(DiskManager::new());
    }
}

/// Get disk manager
pub fn get_disk_manager() -> &'static mut DiskManager {
    unsafe { DISK_MANAGER.as_mut().expect("Disk manager not initialized") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disk_manager_creation() {
        let dm = DiskManager::new();
        assert_eq!(dm.disk_count(), 0);
    }

    #[test]
    fn test_add_disk() {
        let mut dm = DiskManager::new();
        let disk = Disk {
            name: "sda".to_string(),
            device: "/dev/sda".to_string(),
            model: "Test Disk".to_string(),
            size: 500 * 1024 * 1024 * 1024,
            sector_size: 512,
            is_removable: false,
            is_rotational: true,
            partitions: Vec::new(),
        };
        dm.add_disk(disk);
        assert_eq!(dm.disk_count(), 1);
    }

    #[test]
    fn test_create_partition() {
        let mut dm = DiskManager::new();
        let disk = Disk {
            name: "sda".to_string(),
            device: "/dev/sda".to_string(),
            model: "Test Disk".to_string(),
            size: 500 * 1024 * 1024 * 1024,
            sector_size: 512,
            is_removable: false,
            is_rotational: true,
            partitions: Vec::new(),
        };
        dm.add_disk(disk);
        dm.create_partition(
            "sda".to_string(),
            PartitionType::Primary,
            FileSystemType::Ext4,
            100 * 1024 * 1024 * 1024,
        )
        .unwrap();
        let disk = dm.get_disk("sda").unwrap();
        assert_eq!(disk.partitions.len(), 1);
    }

    #[test]
    fn test_format_size() {
        assert_eq!(DiskManager::format_size(1024), "1 KB");
        assert_eq!(DiskManager::format_size(1024 * 1024), "1 MB");
        assert_eq!(DiskManager::format_size(1024 * 1024 * 1024), "1 GB");
    }
}
