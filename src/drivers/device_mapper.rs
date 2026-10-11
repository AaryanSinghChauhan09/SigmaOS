// SigmaOS Device Mapper Framework (dm-mod)
// Supports linear, striped, dm-crypt, and dm-verity virtual block device mappings.

#![allow(dead_code)]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

/// Device Mapper Target Type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DmTargetType {
    Linear {
        target_dev: String,
        offset_sectors: u64,
    },
    Striped {
        target_devs: Vec<String>,
        chunk_size_sectors: u32,
    },
    Crypt {
        cipher: String,
        target_dev: String,
        key_id: u32,
    },
    Verity {
        data_dev: String,
        hash_dev: String,
        root_digest: [u8; 32],
    },
}

/// A mapped table range: [start_sector, length_sectors] -> Target
#[derive(Debug, Clone)]
pub struct DmTableEntry {
    pub start_sector: u64,
    pub length_sectors: u64,
    pub target: DmTargetType,
}

/// A virtual mapped block device (e.g. /dev/mapper/dm-0)
#[derive(Debug, Clone)]
pub struct DmMappedDevice {
    pub name: String,
    pub uuid: String,
    pub total_sectors: u64,
    pub table: Vec<DmTableEntry>,
    pub is_active: bool,
}

/// Device Mapper Core Controller
#[derive(Debug, Clone, Default)]
pub struct DeviceMapperController {
    pub devices: Vec<DmMappedDevice>,
}

impl DeviceMapperController {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }

    /// Create a new device mapper node
    pub fn create_device(&mut self, name: &str, uuid: &str) -> Result<usize, &'static str> {
        if self.devices.iter().any(|d| d.name == name) {
            return Err("Device mapper name already in use");
        }

        self.devices.push(DmMappedDevice {
            name: String::from(name),
            uuid: String::from(uuid),
            total_sectors: 0,
            table: Vec::new(),
            is_active: false,
        });

        Ok(self.devices.len() - 1)
    }

    /// Load mapping table entries for a device
    pub fn load_table(&mut self, dev_idx: usize, table: &[DmTableEntry]) -> Result<(), &'static str> {
        let dev = self.devices.get_mut(dev_idx).ok_or("Device index invalid")?;
        let mut total = 0u64;
        for entry in table {
            total += entry.length_sectors;
        }
        dev.table = table.to_vec();
        dev.total_sectors = total;
        Ok(())
    }

    /// Resume / activate the mapped device
    pub fn resume(&mut self, dev_idx: usize) -> Result<(), &'static str> {
        let dev = self.devices.get_mut(dev_idx).ok_or("Device index invalid")?;
        dev.is_active = true;
        Ok(())
    }

    /// Map a logical sector I/O request to underlying physical device and offset
    pub fn map_sector(&self, dev_name: &str, sector: u64) -> Result<(String, u64), &'static str> {
        let dev = self
            .devices
            .iter()
            .find(|d| d.name == dev_name && d.is_active)
            .ok_or("Mapped device not found or inactive")?;

        for entry in &dev.table {
            if sector >= entry.start_sector && sector < entry.start_sector + entry.length_sectors {
                let offset_in_range = sector - entry.start_sector;
                match &entry.target {
                    DmTargetType::Linear { target_dev, offset_sectors } => {
                        return Ok((target_dev.clone(), offset_sectors + offset_in_range));
                    }
                    DmTargetType::Crypt { target_dev, .. } => {
                        return Ok((target_dev.clone(), offset_in_range));
                    }
                    DmTargetType::Verity { data_dev, .. } => {
                        return Ok((data_dev.clone(), offset_in_range));
                    }
                    DmTargetType::Striped { target_devs, chunk_size_sectors } => {
                        let chunk = (offset_in_range / (*chunk_size_sectors as u64)) as usize;
                        let target_idx = chunk % target_devs.len();
                        return Ok((target_devs[target_idx].clone(), offset_in_range));
                    }
                }
            }
        }

        Err("Sector beyond mapped device table boundary")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_mapper_linear_and_crypt_mapping() {
        let mut dm = DeviceMapperController::new();
        let idx = dm.create_device("root-luks", "uuid-1234").expect("Created");

        let table = [
            DmTableEntry {
                start_sector: 0,
                length_sectors: 2048,
                target: DmTargetType::Linear {
                    target_dev: String::from("/dev/nvme0n1p2"),
                    offset_sectors: 4096,
                },
            },
        ];

        assert!(dm.load_table(idx, &table).is_ok());
        assert!(dm.resume(idx).is_ok());

        let (target, phys_sector) = dm.map_sector("root-luks", 10).expect("Mapped");
        assert_eq!(target, "/dev/nvme0n1p2");
        assert_eq!(phys_sector, 4106); // 4096 + 10
    }
}
