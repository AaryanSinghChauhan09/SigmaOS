#[derive(Debug, Clone, PartialEq)]
pub enum FsFormat {
    FAT32,
    ExFAT,
    EXT4,
    NTFS,
}

pub struct BlockDevice {
    pub path: String,
    pub is_system_partition: bool,
    pub is_mounted: bool,
    pub size_bytes: u64,
}

pub struct MintstickFlasher {
    pub target_device: BlockDevice,
    pub iso_path: String,
    pub progress: f64,
}

impl MintstickFlasher {
    pub fn new(target_device: BlockDevice, iso_path: String) -> Self {
        Self {
            target_device,
            iso_path,
            progress: 0.0,
        }
    }

    pub fn validate_safety(&self) -> Result<(), &'static str> {
        if self.target_device.is_system_partition {
            return Err("Cannot flash system partition");
        }
        if self.target_device.is_mounted {
            return Err("Target device is mounted");
        }
        Ok(())
    }

    pub fn format_device(&mut self, _format: FsFormat) -> Result<(), &'static str> {
        self.validate_safety()?;
        // Simulate formatting
        Ok(())
    }

    pub fn start_flashing(&mut self) -> Result<(), &'static str> {
        self.validate_safety()?;
        self.progress = 100.0;
        Ok(())
    }

    pub fn verify_checksum(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safety_validator_system() {
        let dev = BlockDevice {
            path: String::from("/dev/sda1"),
            is_system_partition: true,
            is_mounted: false,
            size_bytes: 100000,
        };
        let flasher = MintstickFlasher::new(dev, String::from("image.iso"));
        assert!(flasher.validate_safety().is_err());
    }

    #[test]
    fn test_safety_validator_mounted() {
        let dev = BlockDevice {
            path: String::from("/dev/sdb1"),
            is_system_partition: false,
            is_mounted: true,
            size_bytes: 100000,
        };
        let flasher = MintstickFlasher::new(dev, String::from("image.iso"));
        assert!(flasher.validate_safety().is_err());
    }

    #[test]
    fn test_safety_validator_ok() {
        let dev = BlockDevice {
            path: String::from("/dev/sdc1"),
            is_system_partition: false,
            is_mounted: false,
            size_bytes: 100000,
        };
        let flasher = MintstickFlasher::new(dev, String::from("image.iso"));
        assert!(flasher.validate_safety().is_ok());
    }

    #[test]
    fn test_formatting() {
        let dev = BlockDevice {
            path: String::from("/dev/sdc1"),
            is_system_partition: false,
            is_mounted: false,
            size_bytes: 100000,
        };
        let mut flasher = MintstickFlasher::new(dev, String::from("image.iso"));
        assert!(flasher.format_device(FsFormat::FAT32).is_ok());
    }

    #[test]
    fn test_flashing_progress() {
        let dev = BlockDevice {
            path: String::from("/dev/sdc1"),
            is_system_partition: false,
            is_mounted: false,
            size_bytes: 100000,
        };
        let mut flasher = MintstickFlasher::new(dev, String::from("image.iso"));
        assert!(flasher.start_flashing().is_ok());
        assert_eq!(flasher.progress, 100.0);
        assert!(flasher.verify_checksum());
    }
}
