// SigmaOS Unified Filesystem and Storage Management Subsystem
// Inspired by Linux (LVM, ext4, mdadm) and BSD (gpart, GEOM, ZFS) administrative suites

#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

// =========================================================================
// 1. BSD-Style Partition Table & Sizing (gpart / disklabel)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitionTableType {
    MBR,
    GPT,
    BSDDiskLabel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiskPartition {
    pub index: usize,
    pub name: String,
    pub start_sector: u64,
    pub end_sector: u64,
    pub fs_type: String,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartitionTable {
    pub table_type: PartitionTableType,
    pub partitions: Vec<DiskPartition>,
    pub sector_size: u64,
    pub total_sectors: u64,
}

impl PartitionTable {
    pub fn new(table_type: PartitionTableType, total_sectors: u64, sector_size: u64) -> Self {
        Self {
            table_type,
            partitions: Vec::new(),
            sector_size,
            total_sectors,
        }
    }

    pub fn add_partition(&mut self, name: &str, size_bytes: u64, fs_type: &str) -> Result<usize, &'static str> {
        let sector_size = self.sector_size;
        let needed_sectors = (size_bytes + sector_size - 1) / sector_size;

        // Determine starting sector with 4KB (8 sectors) alignment for performance
        let mut start_sector = 8; // standard alignment start
        if let Some(last) = self.partitions.last() {
            start_sector = last.end_sector + 1;
            // Align start sector to multiple of 8 (4KB)
            if start_sector % 8 != 0 {
                start_sector += 8 - (start_sector % 8);
            }
        }

        if start_sector + needed_sectors > self.total_sectors {
            return Err("Not enough sectors on device for partition");
        }

        let index = self.partitions.len() + 1;
        let partition = DiskPartition {
            index,
            name: name.to_string(),
            start_sector,
            end_sector: start_sector + needed_sectors - 1,
            fs_type: fs_type.to_string(),
            size_bytes: needed_sectors * sector_size,
        };

        self.partitions.push(partition);
        Ok(index)
    }

    pub fn delete_partition(&mut self, index: usize) -> Result<(), &'static str> {
        let pos = self.partitions.iter().position(|p| p.index == index).ok_or("Partition index not found")?;
        self.partitions.remove(pos);
        Ok(())
    }

    pub fn verify_alignment(&self, index: usize) -> bool {
        if let Some(p) = self.partitions.iter().find(|part| part.index == index) {
            // Verify 4KB alignment: start_sector * sector_size % 4096 == 0
            (p.start_sector * self.sector_size) % 4096 == 0
        } else {
            false
        }
    }
}

// =========================================================================
// 2. Linux-Inspired Logical Volume Manager (LVM)
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalVolume {
    pub path: String,
    pub size_bytes: u64,
    pub allocated_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalVolume {
    pub name: String,
    pub size_bytes: u64,
    pub fs_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VolumeGroup {
    pub name: String,
    pub pvs: Vec<PhysicalVolume>,
    pub lvs: Vec<LogicalVolume>,
    pub total_size_bytes: u64,
    pub allocated_bytes: u64,
}

impl VolumeGroup {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            pvs: Vec::new(),
            lvs: Vec::new(),
            total_size_bytes: 0,
            allocated_bytes: 0,
        }
    }

    pub fn add_pv(&mut self, path: &str, size_bytes: u64) {
        self.pvs.push(PhysicalVolume {
            path: path.to_string(),
            size_bytes,
            allocated_bytes: 0,
        });
        self.total_size_bytes += size_bytes;
    }

    pub fn free_space_bytes(&self) -> u64 {
        self.total_size_bytes.saturating_sub(self.allocated_bytes)
    }
}

#[derive(Debug, Clone, Default)]
pub struct LvmManager {
    pub volume_groups: BTreeMap<String, VolumeGroup>,
}

impl LvmManager {
    pub fn new() -> Self {
        Self {
            volume_groups: BTreeMap::new(),
        }
    }

    pub fn create_volume_group(&mut self, name: &str, pvs: Vec<(&str, u64)>) -> Result<(), &'static str> {
        if self.volume_groups.contains_key(name) {
            return Err("Volume Group already exists");
        }

        let mut vg = VolumeGroup::new(name);
        for (pv_path, size) in pvs {
            vg.add_pv(pv_path, size);
        }

        self.volume_groups.insert(name.to_string(), vg);
        Ok(())
    }

    pub fn create_logical_volume(&mut self, vg_name: &str, lv_name: &str, size_bytes: u64, fs_type: &str) -> Result<(), &'static str> {
        let vg = self.volume_groups.get_mut(vg_name).ok_or("Volume Group not found")?;
        if vg.free_space_bytes() < size_bytes {
            return Err("Insufficient free space in Volume Group");
        }

        if vg.lvs.iter().any(|lv| lv.name == lv_name) {
            return Err("Logical Volume already exists");
        }

        // Allocate across PVs
        let mut remaining = size_bytes;
        for pv in vg.pvs.iter_mut() {
            let free_pv = pv.size_bytes.saturating_sub(pv.allocated_bytes);
            let alloc = remaining.min(free_pv);
            pv.allocated_bytes += alloc;
            remaining -= alloc;
            if remaining == 0 {
                break;
            }
        }

        vg.allocated_bytes += size_bytes;
        vg.lvs.push(LogicalVolume {
            name: lv_name.to_string(),
            size_bytes,
            fs_type: fs_type.to_string(),
        });

        Ok(())
    }

    pub fn extend_logical_volume(&mut self, vg_name: &str, lv_name: &str, extra_bytes: u64) -> Result<(), &'static str> {
        let vg = self.volume_groups.get_mut(vg_name).ok_or("Volume Group not found")?;
        if vg.free_space_bytes() < extra_bytes {
            return Err("Insufficient free space in Volume Group to extend Logical Volume");
        }

        let lv = vg.lvs.iter_mut().find(|l| l.name == lv_name).ok_or("Logical Volume not found")?;

        // Allocate across PVs
        let mut remaining = extra_bytes;
        for pv in vg.pvs.iter_mut() {
            let free_pv = pv.size_bytes.saturating_sub(pv.allocated_bytes);
            let alloc = remaining.min(free_pv);
            pv.allocated_bytes += alloc;
            remaining -= alloc;
            if remaining == 0 {
                break;
            }
        }

        vg.allocated_bytes += extra_bytes;
        lv.size_bytes += extra_bytes;

        Ok(())
    }

    pub fn reduce_logical_volume(&mut self, vg_name: &str, lv_name: &str, reduce_bytes: u64) -> Result<(), &'static str> {
        let vg = self.volume_groups.get_mut(vg_name).ok_or("Volume Group not found")?;
        let lv = vg.lvs.iter_mut().find(|l| l.name == lv_name).ok_or("Logical Volume not found")?;

        if lv.size_bytes < reduce_bytes {
            return Err("Cannot reduce Logical Volume beyond its current size");
        }

        // Deallocate across PVs
        let mut remaining = reduce_bytes;
        for pv in vg.pvs.iter_mut() {
            let alloc = remaining.min(pv.allocated_bytes);
            pv.allocated_bytes -= alloc;
            remaining -= alloc;
            if remaining == 0 {
                break;
            }
        }

        vg.allocated_bytes -= reduce_bytes;
        lv.size_bytes -= reduce_bytes;

        Ok(())
    }
}

// =========================================================================
// 3. ZFS-Style Storage Pool & Datasets (zpool / zfs)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZpoolStatus {
    Online,
    Degraded,
    Faulted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZfsDataset {
    pub name: String,
    pub compression: String,
    pub dedup: bool,
    pub quota_bytes: Option<u64>,
    pub used_bytes: u64,
    pub snapshots: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZfsPool {
    pub name: String,
    pub status: ZpoolStatus,
    pub devices: Vec<String>,
    pub raid_level: String,
    pub total_space_bytes: u64,
    pub allocated_bytes: u64,
    pub datasets: BTreeMap<String, ZfsDataset>,
}

impl ZfsPool {
    pub fn new(name: &str, raid_level: &str) -> Self {
        Self {
            name: name.to_string(),
            status: ZpoolStatus::Online,
            devices: Vec::new(),
            raid_level: raid_level.to_string(),
            total_space_bytes: 0,
            allocated_bytes: 0,
            datasets: BTreeMap::new(),
        }
    }

    pub fn free_space_bytes(&self) -> u64 {
        self.total_space_bytes.saturating_sub(self.allocated_bytes)
    }
}

#[derive(Debug, Clone, Default)]
pub struct ZfsManager {
    pub pools: BTreeMap<String, ZfsPool>,
}

impl ZfsManager {
    pub fn new() -> Self {
        Self {
            pools: BTreeMap::new(),
        }
    }

    pub fn create_pool(&mut self, name: &str, raid_level: &str, devices: Vec<(&str, u64)>) -> Result<(), &'static str> {
        if self.pools.contains_key(name) {
            return Err("Pool already exists");
        }

        let mut pool = ZfsPool::new(name, raid_level);
        for (device_path, size) in devices {
            pool.devices.push(device_path.to_string());
            pool.total_space_bytes += size;
        }

        // Create default root dataset
        pool.datasets.insert(name.to_string(), ZfsDataset {
            name: name.to_string(),
            compression: "lz4".to_string(),
            dedup: false,
            quota_bytes: None,
            used_bytes: 0,
            snapshots: Vec::new(),
        });

        self.pools.insert(name.to_string(), pool);
        Ok(())
    }

    pub fn create_dataset(&mut self, pool_name: &str, dataset_name: &str) -> Result<(), &'static str> {
        let pool = self.pools.get_mut(pool_name).ok_or("Pool not found")?;
        let full_dataset_name = alloc::format!("{}/{}", pool_name, dataset_name);

        if pool.datasets.contains_key(&full_dataset_name) {
            return Err("Dataset already exists");
        }

        // Inherit compression/dedup settings from root dataset
        let root = pool.datasets.get(pool_name).unwrap();
        let dataset = ZfsDataset {
            name: full_dataset_name.clone(),
            compression: root.compression.clone(),
            dedup: root.dedup,
            quota_bytes: None,
            used_bytes: 0,
            snapshots: Vec::new(),
        };

        pool.datasets.insert(full_dataset_name, dataset);
        Ok(())
    }

    pub fn set_dataset_property(&mut self, pool_name: &str, dataset_name: &str, prop: &str, value: &str) -> Result<(), &'static str> {
        let pool = self.pools.get_mut(pool_name).ok_or("Pool not found")?;
        let full_dataset_name = alloc::format!("{}/{}", pool_name, dataset_name);
        let dataset = pool.datasets.get_mut(&full_dataset_name).ok_or("Dataset not found")?;

        match prop {
            "compression" => {
                if value == "lz4" || value == "zstd" || value == "none" {
                    dataset.compression = value.to_string();
                } else {
                    return Err("Invalid compression value. Allowed: lz4, zstd, none");
                }
            }
            "dedup" => {
                dataset.dedup = value == "on";
            }
            "quota" => {
                if let Ok(bytes) = value.parse::<u64>() {
                    dataset.quota_bytes = Some(bytes);
                } else {
                    return Err("Invalid quota value; must be a valid integer");
                }
            }
            _ => return Err("Unsupported ZFS dataset property"),
        }
        Ok(())
    }

    pub fn take_snapshot(&mut self, pool_name: &str, dataset_name: &str, snap_name: &str) -> Result<(), &'static str> {
        let pool = self.pools.get_mut(pool_name).ok_or("Pool not found")?;
        let full_dataset_name = alloc::format!("{}/{}", pool_name, dataset_name);
        let dataset = pool.datasets.get_mut(&full_dataset_name).ok_or("Dataset not found")?;

        let snapshot_full_name = alloc::format!("{}@{}", full_dataset_name, snap_name);
        if dataset.snapshots.iter().any(|s| s == &snapshot_full_name) {
            return Err("Snapshot already exists");
        }

        dataset.snapshots.push(snapshot_full_name);
        Ok(())
    }

    pub fn rollback_to_snapshot(&mut self, pool_name: &str, dataset_name: &str, snap_name: &str) -> Result<(), &'static str> {
        let pool = self.pools.get_mut(pool_name).ok_or("Pool not found")?;
        let full_dataset_name = alloc::format!("{}/{}", pool_name, dataset_name);
        let dataset = pool.datasets.get_mut(&full_dataset_name).ok_or("Dataset not found")?;

        let snapshot_full_name = alloc::format!("{}@{}", full_dataset_name, snap_name);
        if !dataset.snapshots.iter().any(|s| s == &snapshot_full_name) {
            return Err("Snapshot not found");
        }

        // In a real CoW filesystem, rollback would restore block pointers.
        // For our high-fidelity simulation, we rollback the simulated metadata
        dataset.used_bytes = 0; // Simulated rollback reset
        Ok(())
    }
}

// =========================================================================
// 4. Mount Manager (Universal OS compatibility)
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MountPoint {
    pub device: String,
    pub target: String,
    pub fs_type: String,
}

#[derive(Debug, Clone, Default)]
pub struct MountManager {
    pub mounts: Vec<MountPoint>,
}

impl MountManager {
    pub fn new() -> Self {
        Self { mounts: Vec::new() }
    }

    pub fn mount(&mut self, device: &str, target: &str, fs_type: &str) -> Result<(), &'static str> {
        if self.mounts.iter().any(|m| m.target == target) {
            return Err("Target path is already mounted");
        }

        self.mounts.push(MountPoint {
            device: device.to_string(),
            target: target.to_string(),
            fs_type: fs_type.to_string(),
        });
        Ok(())
    }

    pub fn unmount(&mut self, target: &str) -> Result<(), &'static str> {
        let pos = self.mounts.iter().position(|m| m.target == target).ok_or("Target path is not mounted")?;
        self.mounts.remove(pos);
        Ok(())
    }
}

// =========================================================================
// 5. Unified Administrative CLI Interpreter (fs_admin_cli)
// =========================================================================

pub struct StorageAdminCli {
    pub gpart_tables: BTreeMap<String, PartitionTable>,
    pub lvm: LvmManager,
    pub zfs: ZfsManager,
    pub mount_manager: MountManager,
}

impl StorageAdminCli {
    pub fn new() -> Self {
        Self {
            gpart_tables: BTreeMap::new(),
            lvm: LvmManager::new(),
            zfs: ZfsManager::new(),
            mount_manager: MountManager::new(),
        }
    }

    pub fn execute_admin_command(&mut self, command: &str) -> Result<String, String> {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return Err("Empty command".to_string());
        }

        match parts[0] {
            "gpart" => {
                if parts.len() < 3 {
                    return Err("Usage: gpart create -t <type> <disk> <total_sectors> OR gpart add -t <fs> -s <size> <disk>".to_string());
                }
                match parts[1] {
                    "create" => {
                        // gpart create -t gpt /dev/sda 2097152
                        if parts.len() < 6 || parts[2] != "-t" {
                            return Err("Usage: gpart create -t <type> <disk> <total_sectors>".to_string());
                        }
                        let t_type = match parts[3] {
                            "gpt" => PartitionTableType::GPT,
                            "mbr" => PartitionTableType::MBR,
                            "bsd" => PartitionTableType::BSDDiskLabel,
                            _ => return Err("Invalid table type. Allowed: gpt, mbr, bsd".to_string()),
                        };
                        let disk = parts[4].to_string();
                        let total_sectors = parts[5].parse::<u64>().map_err(|_| "Invalid total sectors count")?;

                        let table = PartitionTable::new(t_type, total_sectors, 512);
                        self.gpart_tables.insert(disk.clone(), table);
                        Ok(alloc::format!("gpart: Partition table ({}) successfully created on {}", parts[3].to_uppercase(), disk))
                    }
                    "add" => {
                        // gpart add -t ext4 -s 524288000 /dev/sda
                        if parts.len() < 7 || parts[2] != "-t" || parts[4] != "-s" {
                            return Err("Usage: gpart add -t <fs_type> -s <size_bytes> <disk>".to_string());
                        }
                        let fs_type = parts[3];
                        let size = parts[5].parse::<u64>().map_err(|_| "Invalid size in bytes")?;
                        let disk = parts[6];

                        let table = self.gpart_tables.get_mut(disk).ok_or("Disk partition table not found")?;
                        let index = table.add_partition("slice", size, fs_type)?;
                        Ok(alloc::format!("gpart: Added partition index {} ({} bytes, {}) on {}", index, size, fs_type, disk))
                    }
                    _ => Err("Invalid gpart subcommand. Use: create, add".to_string())
                }
            }
            "vgcreate" => {
                // vgcreate vg_data /dev/sda1:524288000 /dev/sdb1:524288000
                if parts.len() < 3 {
                    return Err("Usage: vgcreate <vg_name> <pv1_path>:<size1> ...".to_string());
                }
                let vg_name = parts[1];
                let mut pvs = Vec::new();
                for &pv_spec in &parts[2..] {
                    let spec_parts: Vec<&str> = pv_spec.split(':').collect();
                    if spec_parts.len() != 2 {
                        return Err("PV specs must be in format path:size_bytes".to_string());
                    }
                    let path = spec_parts[0];
                    let size = spec_parts[1].parse::<u64>().map_err(|_| "Invalid PV size")?;
                    pvs.push((path, size));
                }
                self.lvm.create_volume_group(vg_name, pvs)?;
                Ok(alloc::format!("lvm: Created volume group '{}'", vg_name))
            }
            "lvcreate" => {
                // lvcreate -n lv_work -L 262144000 vg_data
                if parts.len() < 6 || parts[1] != "-n" || parts[3] != "-L" {
                    return Err("Usage: lvcreate -n <lv_name> -L <size_bytes> <vg_name>".to_string());
                }
                let lv_name = parts[2];
                let size = parts[4].parse::<u64>().map_err(|_| "Invalid LV size")?;
                let vg_name = parts[5];
                self.lvm.create_logical_volume(vg_name, lv_name, size, "ext4")?;
                Ok(alloc::format!("lvm: Created logical volume '{}' inside VG '{}' with {} bytes", lv_name, vg_name, size))
            }
            "zpool" => {
                if parts.len() < 3 {
                    return Err("Usage: zpool create <pool_name> <raid_level> <device1>:<size1> ...".to_string());
                }
                match parts[1] {
                    "create" => {
                        let pool_name = parts[2];
                        let raid_level = parts[3];
                        let mut devices = Vec::new();
                        for &dev_spec in &parts[4..] {
                            let spec_parts: Vec<&str> = dev_spec.split(':').collect();
                            if spec_parts.len() != 2 {
                                return Err("Device specs must be in format path:size_bytes".to_string());
                            }
                            let path = spec_parts[0];
                            let size = spec_parts[1].parse::<u64>().map_err(|_| "Invalid device size")?;
                            devices.push((path, size));
                        }
                        self.zfs.create_pool(pool_name, raid_level, devices)?;
                        Ok(alloc::format!("zfs: Created storage pool '{}' with RAID profile '{}'", pool_name, raid_level))
                    }
                    "status" => {
                        let mut output = String::from("  pool: zfs-pools-status\n");
                        for pool in self.zfs.pools.values() {
                            output.push_str(&alloc::format!("  pool: {}\n state: {:?}\n config:\n\t{}\n", pool.name, pool.status, pool.raid_level));
                            for dev in &pool.devices {
                                output.push_str(&alloc::format!("\t  {}\n", dev));
                            }
                        }
                        Ok(output)
                    }
                    _ => Err("Invalid zpool subcommand. Use: create, status".to_string())
                }
            }
            "zfs" => {
                if parts.len() < 3 {
                    return Err("Usage: zfs create <pool_name>/<dataset_name> OR zfs snapshot <pool_name>/<dataset_name>@<snap_name>".to_string());
                }
                match parts[1] {
                    "create" => {
                        let full_spec = parts[2];
                        let spec_parts: Vec<&str> = full_spec.split('/').collect();
                        if spec_parts.len() != 2 {
                            return Err("Dataset spec must be in pool/dataset format".to_string());
                        }
                        let pool = spec_parts[0];
                        let dataset = spec_parts[1];
                        self.zfs.create_dataset(pool, dataset)?;
                        Ok(alloc::format!("zfs: Created dataset '{}' inside pool '{}'", dataset, pool))
                    }
                    "snapshot" => {
                        let full_spec = parts[2];
                        let spec_at: Vec<&str> = full_spec.split('@').collect();
                        if spec_at.len() != 2 {
                            return Err("Snapshot spec must be in pool/dataset@snapshot format".to_string());
                        }
                        let ds_spec = spec_at[0];
                        let snap_name = spec_at[1];

                        let spec_slash: Vec<&str> = ds_spec.split('/').collect();
                        if spec_slash.len() != 2 {
                            return Err("Dataset spec must be in pool/dataset format".to_string());
                        }
                        let pool = spec_slash[0];
                        let dataset = spec_slash[1];

                        self.zfs.take_snapshot(pool, dataset, snap_name)?;
                        Ok(alloc::format!("zfs: Snapshot '{}' successfully created for dataset '{}'", snap_name, ds_spec))
                    }
                    _ => Err("Invalid zfs subcommand. Use: create, snapshot".to_string())
                }
            }
            "mount" => {
                if parts.len() < 3 {
                    return Err("Usage: mount <device> <target>".to_string());
                }
                let dev = parts[1];
                let target = parts[2];
                self.mount_manager.mount(dev, target, "auto")?;
                Ok(alloc::format!("system: Mounted {} to {}", dev, target))
            }
            "unmount" => {
                if parts.len() < 2 {
                    return Err("Usage: unmount <target>".to_string());
                }
                let target = parts[1];
                self.mount_manager.unmount(target)?;
                Ok(alloc::format!("system: Unmounted {}", target))
            }
            "df" => {
                let mut output = String::from("Filesystem           Mounted on\n");
                for mount in &self.mount_manager.mounts {
                    output.push_str(&alloc::format!("{:<20} {}\n", mount.device, mount.target));
                }
                Ok(output)
            }
            _ => Err(alloc::format!("Unknown storage administration command: {}", parts[0])),
        }
    }
}
