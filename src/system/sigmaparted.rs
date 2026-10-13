//! SigmaParted - Partitioning Tool
//!
//! Inspired by parted/gdisk, this module provides comprehensive disk partitioning
//! including partition table management, partition creation/deletion, and
//! partition type management.

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, Ordering};

/// Partition table type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitionTableType {
    MBR,
    GPT,
    Unknown,
}

/// Partition type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitionType {
    Primary,
    Logical,
    Extended,
    Unknown,
}

/// Partition flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitionFlag {
    Boot,
    LVM,
    RAID,
    LBA,
    Hidden,
    Unknown,
}

/// Partition state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitionState {
    Active,
    Inactive,
    Unknown,
}

/// Partition data structure
#[derive(Debug, Clone)]
pub struct Partition {
    pub partition_id: String,
    pub partition_number: u32,
    pub partition_type: PartitionType,
    pub start_sector: u64,
    pub end_sector: u64,
    pub size: u64,
    pub filesystem: String,
    pub flags: Vec<PartitionFlag>,
    pub state: PartitionState,
    pub disk_id: String,
}

impl Partition {
    /// Create a new partition
    pub fn new(
        partition_id: String,
        partition_number: u32,
        partition_type: PartitionType,
        start_sector: u64,
        end_sector: u64,
        filesystem: String,
        disk_id: String,
    ) -> Self {
        let size = end_sector - start_sector + 1;
        Self {
            partition_id,
            partition_number,
            partition_type,
            start_sector,
            end_sector,
            size,
            filesystem,
            flags: Vec::new(),
            state: PartitionState::Active,
            disk_id,
        }
    }

    /// Set partition state
    pub fn set_state(&mut self, state: PartitionState) {
        self.state = state;
    }

    /// Add a flag
    pub fn add_flag(&mut self, flag: PartitionFlag) {
        if !self.flags.contains(&flag) {
            self.flags.push(flag);
        }
    }

    /// Remove a flag
    pub fn remove_flag(&mut self, flag: PartitionFlag) {
        self.flags.retain(|f| *f != flag);
    }

    /// Check if partition has a flag
    pub fn has_flag(&self, flag: PartitionFlag) -> bool {
        self.flags.contains(&flag)
    }

    /// Check if partition is bootable
    pub fn is_bootable(&self) -> bool {
        self.has_flag(PartitionFlag::Boot)
    }

    /// Check if partition is active
    pub fn is_active(&self) -> bool {
        self.state == PartitionState::Active
    }
}

/// Disk data structure
#[derive(Debug, Clone)]
pub struct Disk {
    pub disk_id: String,
    pub disk_name: String,
    pub disk_path: String,
    pub size: u64,
    pub sector_size: u64,
    pub table_type: PartitionTableType,
    pub partitions: BTreeMap<u32, Partition>,
}

impl Disk {
    /// Create a new disk
    pub fn new(
        disk_id: String,
        disk_name: String,
        disk_path: String,
        size: u64,
        sector_size: u64,
        table_type: PartitionTableType,
    ) -> Self {
        Self {
            disk_id,
            disk_name,
            disk_path,
            size,
            sector_size,
            table_type,
            partitions: BTreeMap::new(),
        }
    }

    /// Set partition table type
    pub fn set_table_type(&mut self, table_type: PartitionTableType) {
        self.table_type = table_type;
    }

    /// Add a partition
    pub fn add_partition(&mut self, partition: Partition) {
        self.partitions.insert(partition.partition_number, partition);
    }

    /// Remove a partition
    pub fn remove_partition(&mut self, partition_number: u32) -> Option<Partition> {
        self.partitions.remove(&partition_number)
    }

    /// Get a partition by number
    pub fn get_partition(&self, partition_number: u32) -> Option<&Partition> {
        self.partitions.get(&partition_number)
    }

    /// List all partitions
    pub fn list_partitions(&self) -> Vec<&Partition> {
        self.partitions.values().collect()
    }

    /// Get partition count
    pub fn partition_count(&self) -> usize {
        self.partitions.len()
    }

    /// Get free space
    pub fn get_free_space(&self) -> u64 {
        let mut used: u64 = 0;
        for partition in self.partitions.values() {
            used += partition.size;
        }
        self.size - used
    }
}

/// Partitioning tool manager
pub struct SigmaParted {
    disks: BTreeMap<String, Disk>,
    running: AtomicBool,
}

impl SigmaParted {
    /// Create a new partitioning tool
    pub fn new() -> Self {
        Self {
            disks: BTreeMap::new(),
            running: AtomicBool::new(false),
        }
    }

    /// Start the partitioning tool
    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
    }

    /// Stop the partitioning tool
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if the tool is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Add a disk
    pub fn add_disk(&mut self, disk: Disk) {
        self.disks.insert(disk.disk_id.clone(), disk);
    }

    /// Remove a disk
    pub fn remove_disk(&mut self, disk_id: &str) -> Option<Disk> {
        self.disks.remove(disk_id)
    }

    /// Get a disk by ID
    pub fn get_disk(&self, disk_id: &str) -> Option<&Disk> {
        self.disks.get(disk_id)
    }

    /// List all disks
    pub fn list_disks(&self) -> Vec<&Disk> {
        self.disks.values().collect()
    }

    /// Create a partition table on a disk
    pub fn create_partition_table(&mut self, disk_id: &str, table_type: PartitionTableType) -> bool {
        if let Some(disk) = self.disks.get_mut(disk_id) {
            disk.set_table_type(table_type);
            disk.partitions.clear();
            true
        } else {
            false
        }
    }

    /// Create a partition on a disk
    pub fn create_partition(&mut self, disk_id: &str, partition: Partition) -> bool {
        if let Some(disk) = self.disks.get_mut(disk_id) {
            disk.add_partition(partition);
            true
        } else {
            false
        }
    }

    /// Delete a partition
    pub fn delete_partition(&mut self, disk_id: &str, partition_number: u32) -> bool {
        if let Some(disk) = self.disks.get_mut(disk_id) {
            disk.remove_partition(partition_number).is_some()
        } else {
            false
        }
    }

    /// Set partition flag
    pub fn set_partition_flag(&mut self, disk_id: &str, partition_number: u32, flag: PartitionFlag) -> bool {
        if let Some(disk) = self.disks.get_mut(disk_id) {
            if let Some(partition) = disk.partitions.get_mut(&partition_number) {
                partition.add_flag(flag);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Clear partition flag
    pub fn clear_partition_flag(&mut self, disk_id: &str, partition_number: u32, flag: PartitionFlag) -> bool {
        if let Some(disk) = self.disks.get_mut(disk_id) {
            if let Some(partition) = disk.partitions.get_mut(&partition_number) {
                partition.remove_flag(flag);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Get disk count
    pub fn disk_count(&self) -> usize {
        self.disks.len()
    }
}

impl Default for SigmaParted {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaParted instance (placeholder for actual global state)
static unsafe mutable SIGMAPARTED_INSTANCE: Option<SigmaParted> = None;

/// Initialize the global SigmaParted instance
pub fn init_sigmaparted() {
    unsafe {
        SIGMAPARTED_INSTANCE = Some(SigmaParted::new());
    }
}

/// Get the global SigmaParted instance
pub fn get_sigmaparted() -> Option<&'static SigmaParted> {
    unsafe {
        SIGMAPARTED_INSTANCE.as_ref()
    }
}

/// Get mutable global SigmaParted instance
pub fn get_sigmaparted_mut() -> Option<&'static mut SigmaParted> {
    unsafe {
        SIGMAPARTED_INSTANCE.as_mut()
    }
}

/// Convenience function to create a partition table
pub fn create_partition_table(disk_id: String, table_type: PartitionTableType) -> bool {
    if let Some(parted) = get_sigmaparted_mut() {
        parted.create_partition_table(&disk_id, table_type)
    } else {
        false
    }
}

/// Convenience function to create a partition
pub fn create_partition(disk_id: String, partition: Partition) -> bool {
    if let Some(parted) = get_sigmaparted_mut() {
        parted.create_partition(&disk_id, partition)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_creation() {
        let partition = Partition::new(
            "part1".to_string(),
            1,
            PartitionType::Primary,
            2048,
            2097151,
            "ext4".to_string(),
            "disk1".to_string(),
        );

        assert_eq!(partition.partition_id, "part1");
        assert_eq!(partition.partition_number, 1);
        assert_eq!(partition.partition_type, PartitionType::Primary);
        assert_eq!(partition.filesystem, "ext4");
        assert!(partition.is_active());
    }

    #[test]
    fn test_partition_flags() {
        let mut partition = Partition::new(
            "part1".to_string(),
            1,
            PartitionType::Primary,
            2048,
            2097151,
            "ext4".to_string(),
            "disk1".to_string(),
        );

        partition.add_flag(PartitionFlag::Boot);
        assert!(partition.is_bootable());
        assert!(partition.has_flag(PartitionFlag::Boot));

        partition.remove_flag(PartitionFlag::Boot);
        assert!(!partition.is_bootable());
    }

    #[test]
    fn test_disk_creation() {
        let disk = Disk::new(
            "disk1".to_string(),
            "sda".to_string(),
            "/dev/sda".to_string(),
            1024 * 1024 * 1024 * 512,
            512,
            PartitionTableType::GPT,
        );

        assert_eq!(disk.disk_id, "disk1");
        assert_eq!(disk.disk_name, "sda");
        assert_eq!(disk.table_type, PartitionTableType::GPT);
        assert_eq!(disk.partition_count(), 0);
    }

    #[test]
    fn test_disk_add_partition() {
        let mut disk = Disk::new(
            "disk1".to_string(),
            "sda".to_string(),
            "/dev/sda".to_string(),
            1024 * 1024 * 1024 * 512,
            512,
            PartitionTableType::GPT,
        );

        let partition = Partition::new(
            "part1".to_string(),
            1,
            PartitionType::Primary,
            2048,
            2097151,
            "ext4".to_string(),
            "disk1".to_string(),
        );

        disk.add_partition(partition);
        assert_eq!(disk.partition_count(), 1);
        assert!(disk.get_partition(1).is_some());
    }

    #[test]
    fn test_disk_remove_partition() {
        let mut disk = Disk::new(
            "disk1".to_string(),
            "sda".to_string(),
            "/dev/sda".to_string(),
            1024 * 1024 * 1024 * 512,
            512,
            PartitionTableType::GPT,
        );

        let partition = Partition::new(
            "part1".to_string(),
            1,
            PartitionType::Primary,
            2048,
            2097151,
            "ext4".to_string(),
            "disk1".to_string(),
        );

        disk.add_partition(partition);
        assert_eq!(disk.partition_count(), 1);

        disk.remove_partition(1);
        assert_eq!(disk.partition_count(), 0);
    }

    #[test]
    fn test_disk_free_space() {
        let mut disk = Disk::new(
            "disk1".to_string(),
            "sda".to_string(),
            "/dev/sda".to_string(),
            1024 * 1024 * 1024 * 512,
            512,
            PartitionTableType::GPT,
        );

        let partition = Partition::new(
            "part1".to_string(),
            1,
            PartitionType::Primary,
            2048,
            2097151,
            "ext4".to_string(),
            "disk1".to_string(),
        );

        disk.add_partition(partition);
        let free = disk.get_free_space();
        assert!(free > 0);
    }

    #[test]
    fn test_sigmaparted_creation() {
        let parted = SigmaParted::new();
        assert!(!parted.is_running());
        assert_eq!(parted.disk_count(), 0);
    }

    #[test]
    fn test_sigmaparted_start_stop() {
        let mut parted = SigmaParted::new();
        parted.start();
        assert!(parted.is_running());
        parted.stop();
        assert!(!parted.is_running());
    }

    #[test]
    fn test_sigmaparted_add_disk() {
        let mut parted = SigmaParted::new();
        let disk = Disk::new(
            "disk1".to_string(),
            "sda".to_string(),
            "/dev/sda".to_string(),
            1024 * 1024 * 1024 * 512,
            512,
            PartitionTableType::GPT,
        );

        parted.add_disk(disk);
        assert_eq!(parted.disk_count(), 1);
        assert!(parted.get_disk("disk1").is_some());
    }

    #[test]
    fn test_sigmaparted_create_partition_table() {
        let mut parted = SigmaParted::new();
        let disk = Disk::new(
            "disk1".to_string(),
            "sda".to_string(),
            "/dev/sda".to_string(),
            1024 * 1024 * 1024 * 512,
            512,
            PartitionTableType::Unknown,
        );

        parted.add_disk(disk);
        let created = parted.create_partition_table("disk1", PartitionTableType::GPT);
        assert!(created);

        let disk = parted.get_disk("disk1").unwrap();
        assert_eq!(disk.table_type, PartitionTableType::GPT);
    }

    #[test]
    fn test_sigmaparted_create_partition() {
        let mut parted = SigmaParted::new();
        let disk = Disk::new(
            "disk1".to_string(),
            "sda".to_string(),
            "/dev/sda".to_string(),
            1024 * 1024 * 1024 * 512,
            512,
            PartitionTableType::GPT,
        );

        parted.add_disk(disk);

        let partition = Partition::new(
            "part1".to_string(),
            1,
            PartitionType::Primary,
            2048,
            2097151,
            "ext4".to_string(),
            "disk1".to_string(),
        );

        let created = parted.create_partition("disk1", partition);
        assert!(created);

        let disk = parted.get_disk("disk1").unwrap();
        assert_eq!(disk.partition_count(), 1);
    }

    #[test]
    fn test_sigmaparted_delete_partition() {
        let mut parted = SigmaParted::new();
        let disk = Disk::new(
            "disk1".to_string(),
            "sda".to_string(),
            "/dev/sda".to_string(),
            1024 * 1024 * 1024 * 512,
            512,
            PartitionTableType::GPT,
        );

        parted.add_disk(disk);

        let partition = Partition::new(
            "part1".to_string(),
            1,
            PartitionType::Primary,
            2048,
            2097151,
            "ext4".to_string(),
            "disk1".to_string(),
        );

        parted.create_partition("disk1", partition);
        let deleted = parted.delete_partition("disk1", 1);
        assert!(deleted);

        let disk = parted.get_disk("disk1").unwrap();
        assert_eq!(disk.partition_count(), 0);
    }

    #[test]
    fn test_sigmaparted_set_partition_flag() {
        let mut parted = SigmaParted::new();
        let disk = Disk::new(
            "disk1".to_string(),
            "sda".to_string(),
            "/dev/sda".to_string(),
            1024 * 1024 * 1024 * 512,
            512,
            PartitionTableType::GPT,
        );

        parted.add_disk(disk);

        let partition = Partition::new(
            "part1".to_string(),
            1,
            PartitionType::Primary,
            2048,
            2097151,
            "ext4".to_string(),
            "disk1".to_string(),
        );

        parted.create_partition("disk1", partition);
        let set = parted.set_partition_flag("disk1", 1, PartitionFlag::Boot);
        assert!(set);

        let disk = parted.get_disk("disk1").unwrap();
        let partition = disk.get_partition(1).unwrap();
        assert!(partition.is_bootable());
    }

    #[test]
    fn test_global_instance() {
        init_sigmaparted();

        let parted = get_sigmaparted();
        assert!(parted.is_some());

        let mut parted_mut = get_sigmaparted_mut();
        assert!(parted_mut.is_some());

        if let Some(parted) = parted_mut {
            parted.start();
            assert!(parted.is_running());
        }
    }

    #[test]
    fn test_convenience_create_partition_table() {
        init_sigmaparted();

        let parted = get_sigmaparted_mut().unwrap();
        let disk = Disk::new(
            "disk1".to_string(),
            "sda".to_string(),
            "/dev/sda".to_string(),
            1024 * 1024 * 1024 * 512,
            512,
            PartitionTableType::Unknown,
        );
        parted.add_disk(disk);

        let created = create_partition_table("disk1".to_string(), PartitionTableType::GPT);
        assert!(created);
    }

    #[test]
    fn test_convenience_create_partition() {
        init_sigmaparted();

        let parted = get_sigmaparted_mut().unwrap();
        let disk = Disk::new(
            "disk1".to_string(),
            "sda".to_string(),
            "/dev/sda".to_string(),
            1024 * 1024 * 1024 * 512,
            512,
            PartitionTableType::GPT,
        );
        parted.add_disk(disk);

        let partition = Partition::new(
            "part1".to_string(),
            1,
            PartitionType::Primary,
            2048,
            2097151,
            "ext4".to_string(),
            "disk1".to_string(),
        );

        let created = create_partition("disk1".to_string(), partition);
        assert!(created);
    }
}
