#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Storage Driver
// Hardware abstraction for storage devices

use crate::security::capability::CapabilityToken;

/// Storage device type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageType {
    NVMe,
    AHCI,
    SATA,
    IDE,    // Legacy IDE hard drives
    Floppy, // Retro Floppy disk controllers
    SCSI,   // Legacy SCSI controllers
    Virtual,
}

/// Storage command
#[derive(Debug, Clone)]
pub enum StorageCommand {
    Read { lba: u64, sectors: u16 },
    Write { lba: u64, data: Vec<u8> },
    Flush,
    Identify,
}

/// Storage driver interface
pub struct StorageDriver {
    pub device_type: StorageType,
    pub block_size: u32,
    pub total_blocks: u64,
    pub capabilities: CapabilityToken,
    pub model: String,
}

impl StorageDriver {
    pub fn new(device_type: StorageType, block_size: u32, total_blocks: u64) -> Self {
        let model = match device_type {
            StorageType::NVMe => "NVMe SSD".to_string(),
            StorageType::AHCI => "AHCI Controller".to_string(),
            StorageType::SATA => "SATA HDD".to_string(),
            StorageType::IDE => "Legacy IDE HDD".to_string(),
            StorageType::Floppy => "Legacy Floppy Drive".to_string(),
            StorageType::SCSI => "Legacy SCSI Disk".to_string(),
            StorageType::Virtual => "Virtual Disk".to_string(),
        };

        Self {
            device_type,
            block_size,
            total_blocks,
            capabilities: CapabilityToken::new(),
            model,
        }
    }

    pub fn execute_command(&mut self, command: StorageCommand) -> Result<Vec<u8>, StorageError> {
        match command {
            StorageCommand::Read { lba, sectors } => {
                if lba >= self.total_blocks {
                    return Err(StorageError::InvalidLBA);
                }
                // Simulate read operation
                let size = (sectors as usize) * (self.block_size as usize);
                Ok(vec![0; size])
            }
            StorageCommand::Write { lba, data: _ } => {
                if lba >= self.total_blocks {
                    return Err(StorageError::InvalidLBA);
                }
                // Simulate write operation
                Ok(vec![])
            }
            StorageCommand::Flush => {
                // Simulate cache flush
                Ok(vec![])
            }
            StorageCommand::Identify => {
                // Return device information
                Ok(self.model.as_bytes().to_vec())
            }
        }
    }

    pub fn get_capacity(&self) -> u64 {
        self.total_blocks * self.block_size as u64
    }

    pub fn set_capabilities(&mut self, capabilities: CapabilityToken) {
        self.capabilities = capabilities;
    }

    pub fn has_capability(&self, capability: u64) -> bool {
        (self.capabilities.bits() & capability) != 0
    }
}

impl Default for StorageDriver {
    fn default() -> Self {
        Self::new(StorageType::Virtual, 512, 1024 * 1024)
    }
}

/// Storage errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorageError {
    InvalidLBA,
    DeviceError,
    PermissionDenied,
    WriteProtected,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_creation() {
        let storage = StorageDriver::new(StorageType::NVMe, 4096, 1024 * 1024);
        assert_eq!(storage.device_type, StorageType::NVMe);
        assert_eq!(storage.block_size, 4096);
    }

    #[test]
    fn test_read_command() {
        let mut storage = StorageDriver::new(StorageType::Virtual, 512, 1024);
        let command = StorageCommand::Read { lba: 0, sectors: 1 };
        assert!(storage.execute_command(command).is_ok());
    }

    #[test]
    fn test_invalid_lba() {
        let mut storage = StorageDriver::new(StorageType::Virtual, 512, 1024);
        let command = StorageCommand::Read {
            lba: 9999,
            sectors: 1,
        };
        assert!(storage.execute_command(command).is_err());
    }

    #[test]
    fn test_capacity() {
        let storage = StorageDriver::new(StorageType::Virtual, 512, 1024);
        assert_eq!(storage.get_capacity(), 512 * 1024);
    }
}
