//! USB Mass Storage Class (MSC) Driver
//! Supports USB flash drives, external hard drives, card readers
//! Reference: USB Mass Storage Class Bulk-Only Transport spec and Linux drivers/usb/storage/

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

/// USB Mass Storage subclass codes
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MscSubclass {
    Rbc = 0x01,             // Reduced Block Commands
    Atapi = 0x02,           // CD/DVD (ATAPI/MMC-5)
    Qic157 = 0x03,          // Tape (QIC-157)
    Ufi = 0x04,             // Floppy (UFI)
    Sff8070i = 0x05,        // Floppy (SFF-8070i)
    ScsiTransparent = 0x06, // SCSI transparent command set
    Lsdfs = 0x07,           // LSD FS
    Ieee1667 = 0x08,        // IEEE 1667
}

/// USB Mass Storage protocol codes
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MscProtocol {
    Cbi = 0x00,      // Control/Bulk/Interrupt
    CbiNoInt = 0x01, // Control/Bulk without interrupt
    BulkOnly = 0x50, // Bulk-Only Transport (most common)
    Uas = 0x62,      // USB Attached SCSI
}

/// Command Block Wrapper (CBW) for Bulk-Only Transport
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct CommandBlockWrapper {
    pub signature: u32,            // 0x43425355 "USBC"
    pub tag: u32,                  // Command tag
    pub data_transfer_length: u32, // Expected data transfer length
    pub flags: u8,                 // Bit 7: direction (0=OUT, 1=IN)
    pub lun: u8,                   // Logical Unit Number (bits 0-3)
    pub cb_length: u8,             // Command block length (1-16)
    pub command_block: [u8; 16],   // Command block (SCSI CDB)
}

impl CommandBlockWrapper {
    pub const SIGNATURE: u32 = 0x43425355; // "USBC"

    pub fn new(tag: u32, data_length: u32, direction_in: bool, lun: u8, cdb: &[u8]) -> Self {
        let mut cb = [0u8; 16];
        let len = cdb.len().min(16);
        cb[..len].copy_from_slice(&cdb[..len]);

        Self {
            signature: Self::SIGNATURE,
            tag,
            data_transfer_length: data_length,
            flags: if direction_in { 0x80 } else { 0x00 },
            lun: lun & 0x0F,
            cb_length: len as u8,
            command_block: cb,
        }
    }
}

/// Command Status Wrapper (CSW) for Bulk-Only Transport
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct CommandStatusWrapper {
    pub signature: u32,    // 0x53425355 "USBS"
    pub tag: u32,          // Command tag (matches CBW)
    pub data_residue: u32, // Difference in data transferred
    pub status: u8,        // Command status
}

impl CommandStatusWrapper {
    pub const SIGNATURE: u32 = 0x53425355; // "USBS"

    pub fn is_valid(&self) -> bool {
        self.signature == Self::SIGNATURE
    }

    pub fn status(&self) -> CswStatus {
        match self.status {
            0x00 => CswStatus::Passed,
            0x01 => CswStatus::Failed,
            0x02 => CswStatus::PhaseError,
            _ => CswStatus::Invalid,
        }
    }
}

/// CSW status codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CswStatus {
    Passed,
    Failed,
    PhaseError,
    Invalid,
}

/// SCSI Command Descriptor Block (CDB) opcodes
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScsiOpcode {
    TestUnitReady = 0x00,
    RequestSense = 0x03,
    Inquiry = 0x12,
    ModeSense6 = 0x1A,
    StartStopUnit = 0x1B,
    ReadFormatCapacities = 0x23,
    ReadCapacity10 = 0x25,
    Read10 = 0x28,
    Write10 = 0x2A,
    Verify10 = 0x2F,
    ModeSense10 = 0x5A,
    ReadCapacity16 = 0x9E,
    Read16 = 0x88,
    Write16 = 0x8A,
}

/// SCSI INQUIRY data structure
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct ScsiInquiryData {
    pub peripheral: u8,        // Peripheral device type
    pub removable: u8,         // Removable media bit
    pub version: u8,           // SCSI version
    pub response_format: u8,   // Response data format
    pub additional_length: u8, // Additional length
    pub flags1: u8,
    pub flags2: u8,
    pub flags3: u8,
    pub vendor_id: [u8; 8],
    pub product_id: [u8; 16],
    pub product_rev: [u8; 4],
}

/// SCSI Read Capacity (10) response
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct ScsiReadCapacity10Data {
    pub last_lba: u32,   // Last logical block address (big-endian)
    pub block_size: u32, // Block size in bytes (big-endian)
}

impl ScsiReadCapacity10Data {
    pub fn get_last_lba(&self) -> u32 {
        u32::from_be(self.last_lba)
    }

    pub fn get_block_size(&self) -> u32 {
        u32::from_be(self.block_size)
    }

    pub fn total_bytes(&self) -> u64 {
        (self.get_last_lba() as u64 + 1) * self.get_block_size() as u64
    }
}

/// USB Mass Storage device structure
pub struct MscDevice {
    pub subclass: MscSubclass,
    pub protocol: MscProtocol,
    pub vendor_id: u16,
    pub product_id: u16,
    pub max_lun: u8, // Maximum Logical Unit Number
    pub tag_counter: AtomicU32,
    pub block_size: u32,
    pub total_blocks: u64,
    pub inquiry_data: Option<ScsiInquiryData>,
}

impl MscDevice {
    pub fn new(
        subclass: MscSubclass,
        protocol: MscProtocol,
        vendor_id: u16,
        product_id: u16,
    ) -> Self {
        Self {
            subclass,
            protocol,
            vendor_id,
            product_id,
            max_lun: 0,
            tag_counter: AtomicU32::new(1),
            block_size: 512,
            total_blocks: 0,
            inquiry_data: None,
        }
    }

    /// Get next command tag
    fn next_tag(&self) -> u32 {
        self.tag_counter.fetch_add(1, Ordering::SeqCst)
    }

    /// Build SCSI INQUIRY command
    pub fn build_inquiry_cdb(&self) -> Vec<u8> {
        vec![
            ScsiOpcode::Inquiry as u8,
            0x00, // Reserved/flags
            0x00, // Page code
            0x00, // Reserved
            36,   // Allocation length
            0x00, // Control
        ]
    }

    /// Build SCSI TEST UNIT READY command
    pub fn build_test_unit_ready_cdb(&self) -> Vec<u8> {
        vec![
            ScsiOpcode::TestUnitReady as u8,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
        ]
    }

    /// Build SCSI READ CAPACITY (10) command
    pub fn build_read_capacity10_cdb(&self) -> Vec<u8> {
        vec![
            ScsiOpcode::ReadCapacity10 as u8,
            0x00, // Reserved/LUN
            0x00,
            0x00,
            0x00,
            0x00, // LBA (0 = report last LBA)
            0x00,
            0x00, // Reserved
            0x00, // PMI (0)
            0x00, // Control
        ]
    }

    /// Build SCSI READ (10) command
    pub fn build_read10_cdb(&self, lba: u32, num_blocks: u16) -> Vec<u8> {
        vec![
            ScsiOpcode::Read10 as u8,
            0x00,              // Flags
            (lba >> 24) as u8, // LBA (big-endian)
            (lba >> 16) as u8,
            (lba >> 8) as u8,
            lba as u8,
            0x00,                    // Group number
            (num_blocks >> 8) as u8, // Transfer length (big-endian)
            num_blocks as u8,
            0x00, // Control
        ]
    }

    /// Build SCSI WRITE (10) command
    pub fn build_write10_cdb(&self, lba: u32, num_blocks: u16) -> Vec<u8> {
        vec![
            ScsiOpcode::Write10 as u8,
            0x00,              // Flags
            (lba >> 24) as u8, // LBA (big-endian)
            (lba >> 16) as u8,
            (lba >> 8) as u8,
            lba as u8,
            0x00,                    // Group number
            (num_blocks >> 8) as u8, // Transfer length (big-endian)
            num_blocks as u8,
            0x00, // Control
        ]
    }

    /// Execute SCSI command via Bulk-Only Transport
    pub fn execute_command(
        &self,
        cdb: &[u8],
        data_direction_in: bool,
        data_length: u32,
    ) -> Result<Vec<u8>, MscError> {
        let tag = self.next_tag();

        // Build and send CBW
        let cbw = CommandBlockWrapper::new(tag, data_length, data_direction_in, 0, cdb);
        // In real implementation: send CBW via USB bulk OUT endpoint

        // Transfer data if needed
        let mut data = Vec::new();
        if data_length > 0 {
            if data_direction_in {
                // Read data from bulk IN endpoint
                data.resize(data_length as usize, 0);
                // In real implementation: receive data via USB bulk IN endpoint
            } else {
                // Write data to bulk OUT endpoint
                // In real implementation: send data via USB bulk OUT endpoint
            }
        }

        // Receive CSW
        // In real implementation: receive CSW via USB bulk IN endpoint
        let csw = CommandStatusWrapper {
            signature: CommandStatusWrapper::SIGNATURE,
            tag,
            data_residue: 0,
            status: 0x00, // Passed
        };

        if !csw.is_valid() {
            return Err(MscError::InvalidCsw);
        }

        if csw.status() != CswStatus::Passed {
            return Err(MscError::CommandFailed);
        }

        Ok(data)
    }

    /// Read sectors from device
    pub fn read_sectors(&self, lba: u64, count: u32, buffer: &mut [u8]) -> Result<usize, MscError> {
        if buffer.len() < (count as usize * self.block_size as usize) {
            return Err(MscError::BufferTooSmall);
        }

        let cdb = self.build_read10_cdb(lba as u32, count as u16);
        let data = self.execute_command(&cdb, true, count * self.block_size)?;

        let bytes_read = data.len();
        buffer[..bytes_read].copy_from_slice(&data);

        Ok(bytes_read)
    }

    /// Write sectors to device
    pub fn write_sectors(&self, lba: u64, count: u32, data: &[u8]) -> Result<usize, MscError> {
        if data.len() < (count as usize * self.block_size as usize) {
            return Err(MscError::BufferTooSmall);
        }

        let cdb = self.build_write10_cdb(lba as u32, count as u16);
        self.execute_command(&cdb, false, count * self.block_size)?;

        Ok(count as usize * self.block_size as usize)
    }

    /// Initialize device (inquiry and read capacity)
    pub fn initialize(&mut self) -> Result<(), MscError> {
        // Test unit ready
        let cdb = self.build_test_unit_ready_cdb();
        self.execute_command(&cdb, true, 0)?;

        // Read capacity
        let cdb = self.build_read_capacity10_cdb();
        let data = self.execute_command(&cdb, true, 8)?;

        if data.len() >= 8 {
            let capacity = unsafe { *(data.as_ptr() as *const ScsiReadCapacity10Data) };
            self.total_blocks = (capacity.get_last_lba() as u64) + 1;
            self.block_size = capacity.get_block_size();
        }

        Ok(())
    }
}

/// USB Mass Storage error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MscError {
    InvalidCbw,
    InvalidCsw,
    CommandFailed,
    PhaseError,
    TransferError,
    BufferTooSmall,
    DeviceNotReady,
    InvalidLun,
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cbw_creation() {
        let cdb = vec![0x12, 0x00, 0x00, 0x00, 36, 0x00]; // INQUIRY
        let cbw = CommandBlockWrapper::new(1, 36, true, 0, &cdb);
        assert_eq!({cbw.signature}, CommandBlockWrapper::SIGNATURE);
        assert_eq!({cbw.tag}, 1);
        assert_eq!(cbw.flags, 0x80);
    }

    #[test]
    fn test_csw_validation() {
        let csw = CommandStatusWrapper {
            signature: CommandStatusWrapper::SIGNATURE,
            tag: 1,
            data_residue: 0,
            status: 0x00,
        };
        assert!(csw.is_valid());
        assert_eq!(csw.status(), CswStatus::Passed);
    }

    #[test]
    fn test_read_capacity() {
        let capacity = ScsiReadCapacity10Data {
            last_lba: u32::to_be(1023),
            block_size: u32::to_be(512),
        };
        assert_eq!(capacity.get_last_lba(), 1023);
        assert_eq!(capacity.get_block_size(), 512);
        assert_eq!(capacity.total_bytes(), 1024 * 512);
    }
}
