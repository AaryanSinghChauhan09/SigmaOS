//! SigmaOS Fdisk Compatibility
//! Disk partitioning (fdisk command)
//! Zero external dependencies

#![no_std]
#![allow(dead_code)]

type SigmaU8 = u8;
type SigmaU32 = u32;
type SigmaI32 = i32;
type SigmaBool = bool;
type SigmaU64 = u64;

/// Partition types
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub enum PartitionType {
    Empty,
    Primary,
    Extended,
    Logical,
}

/// Partition entry
#[repr(C)]
pub struct Partition {
    pub number: SigmaU32,
    pub bootable: SigmaBool,
    pub start_sector: SigmaU64,
    pub size_sectors: SigmaU64,
    pub partition_type: PartitionType,
    pub partition_id: [u8; 4],
}

/// Disk device
#[repr(C)]
pub struct DiskDevice {
    pub name: [u8; 32],
    pub size_bytes: SigmaU64,
    pub sector_size: SigmaU32,
    pub partition_count: SigmaU32,
}

/// Fdisk state
const MAX_DISKS: usize = 16;
const MAX_PARTITIONS: usize = 128;

static mut DISKS: [DiskDevice; MAX_DISKS] = [DiskDevice {
    name: [0; 32],
    size_bytes: 0,
    sector_size: 512,
    partition_count: 0,
}; MAX_DISKS];

static mut PARTITIONS: [Partition; MAX_PARTITIONS] = [Partition {
    number: 0,
    bootable: false,
    start_sector: 0,
    size_sectors: 0,
    partition_type:PartitionType::Empty,
    partition_id: [0; 4],
}; MAX_PARTITIONS];

static mut DISK_COUNT: SigmaU32 = 0;
static mut PARTITION_COUNT: SigmaU32 = 0;
static mut FDISK_INITIALIZED: SigmaBool = false;

/// Initialize fdisk
#[no_mangle]
pub unsafe extern "C" fn fdisk_init() -> SigmaI32 {
    FDISK_INITIALIZED = true;
    DISK_COUNT = 0;
    PARTITION_COUNT = 0;
    
    // Add sample disk
    let mut sda = DiskDevice {
        name: [0; 32],
        size_bytes: 500 * 1024 * 1024 * 1024, // 500GB
        sector_size: 512,
        partition_count: 0,
    };
    
    for i in 0..31 {
        sda.name[i] = b"/dev/sda"[i.min(8)];
    }
    
    DISKS[0] = sda;
    DISK_COUNT = 1;
    
    0 // Success
}

/// List disks
#[no_mangle]
pub unsafe extern "C" fn fdisk_list_disks(disks: *mut DiskDevice, max_count: SigmaU32) -> SigmaU32 {
    if !FDISK_INITIALIZED || disks.isnull() {
        return 0;
    }
    
    let mut count = 0;
    for i in 0..DISK_COUNT as usize {
        if count >= max_count as usize {
            break;
        }
        *disks.add(count) = DISKS[i];
        count += 1;
    }
    
    count
}

/// List partitions for disk
#[no_mangle]
pub unsafe extern "C" fn fdisk_list_partitions(
    disk_name: *const u8,
    partitions: *mut Partition,
    max_count: SigmaU32,
) -> SigmaU32 {
    if !FDISK_INITIALIZED || disk_name.isnull() || partitions.isnull() {
        return 0;
    }
    
    let mut count = 0;
    for i in 0..PARTITION_COUNT as usize {
        if count >= max_count as usize {
            break;
        }
        *partitions.add(count) = PARTITIONS[i];
        count += 1;
    }
    
    count
}

/// Create partition
#[no_mangle]
pub unsafe extern "C" fn fdisk_create_partition(
    disk_name: *const u8,
    partition_type: PartitionType,
    start_sector: SigmaU64,
    size_sectors: SigmaU64,
) -> SigmaI32 {
    if !FDISK_INITIALIZED || PARTITION_COUNT >= MAX_PARTITIONS as SigmaU32 {
        return -1;
    }
    
    let mut partition = Partition {
        number: PARTITION_COUNT as SigmaU32 + 1,
        bootable: false,
        start_sector,
        size_sectors,
        partition_type,
        partition_id: [0x83, 0, 0, 0], // Linux partition
    };
    
    PARTITIONS[PARTITION_COUNT as usize] = partition;
    PARTITION_COUNT += 1;
    
    // Update disk partition count
    for i in 0..DISK_COUNT as usize {
        let disk = &mut DISKS[i];
        
        let mut matches = true;
        for j in 0..32 {
            if disk.name[j] != *disk_name.add(j) {
                if disk.name[j] == 0 && *disk_name.add(j) == 0 {
                    break;
                }
                matches = false;
                break;
            }
            if disk.name[j] == 0 {
                break;
            }
        }
        
        if matches {
            disk.partition_count += 1;
            break;
        }
    }
    
    0 // Success
}

/// Delete partition
#[no_mangle]
pub unsafe extern "C" fn fdisk_delete_partition(disk_name: *const u8, partition_number: SigmaU32) -> SigmaI32 {
    if !FDISK_INITIALIZED || disk_name.isnull() {
        return -1;
    }
    
    for i in 0..PARTITION_COUNT as usize {
        if PARTITIONS[i].number == partition_number {
            // Shift remaining partitions
            for k in i..PARTITION_COUNT as usize - 1 {
                PARTITIONS[k] = PARTITIONS[k + 1];
            }
            PARTITION_COUNT -= 1;
            
            // Update disk partition count
            for j in 0..DISK_COUNT as usize {
                let disk = &mut DISKS[j];
                
                let mut matches = true;
                for k in 0..32 {
                    if disk.name[k] != *disk_name.add(k) {
                        if disk.name[k] == 0 && *disk_name.add(k) == 0 {
                            break;
                        }
                        matches = false;
                        break;
                    }
                    if disk.name[k] == 0 {
                        break;
                    }
                }
                
                if matches {
                    disk.partition_count -= 1;
                    break;
                }
            }
            
            return 0;
        }
    }
    
    -2 // Partition not found
}

/// Set bootable flag
#[no_mangle]
pub unsafe extern "C" fn fdisk_set_bootable(partition_number: SigmaU32, bootable: SigmaBool) -> SigmaI32 {
    if !FDISK_INITIALIZED {
        return -1;
    }
    
    for i in 0..PARTITION_COUNT as usize {
        if PARTITIONS[i].number == partition_number {
            PARTITIONS[i].bootable = bootable;
            return 0;
        }
    }
    
    -2 // Partition not found
}

/// Get disk count
#[no_mangle]
pub unsafe extern "C" fn fdisk_get_disk_count() -> SigmaU32 {
    DISK_COUNT
}

/// Get partition count
#[no_mangle]
pub unsafe extern "C" fn fdisk_get_partition_count() -> SigmaU32 {
    PARTITION_COUNT
}
