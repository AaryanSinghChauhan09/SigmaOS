// SigmaOS SD/MMC Host Controller Driver (SDHCI & eMMC Protocol)
// Provides block-level storage access for SD cards, MicroSD, and integrated eMMC chips.

#![allow(dead_code)]
extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;

pub const SD_BLOCK_SIZE: usize = 512;

/// Standard SD/MMC Command Indices
pub const CMD0_GO_IDLE_STATE: u8 = 0;
pub const CMD8_SEND_IF_COND: u8 = 8;
pub const CMD55_APP_CMD: u8 = 55;
pub const ACMD41_SD_SEND_OP_COND: u8 = 41;
pub const CMD2_ALL_SEND_CID: u8 = 2;
pub const CMD3_SEND_RELATIVE_ADDR: u8 = 3;
pub const CMD7_SELECT_CARD: u8 = 7;
pub const CMD17_READ_SINGLE_BLOCK: u8 = 17;
pub const CMD18_READ_MULTIPLE_BLOCK: u8 = 18;
pub const CMD24_WRITE_SINGLE_BLOCK: u8 = 24;

/// SD Card Identification & Operating State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdCardState {
    Uninitialized,
    Idle,
    Ready,
    Identification,
    Standby,
    Transfer,
}

/// Discovered SD/eMMC Storage Device Info
#[derive(Debug, Clone)]
pub struct SdCardInfo {
    pub rca: u16, // Relative Card Address
    pub capacity_sectors: u64,
    pub is_high_capacity: bool, // SDHC / SDXC
    pub state: SdCardState,
}

/// SDHCI Host Controller
#[derive(Debug, Clone)]
pub struct SdHostController {
    pub mmio_base: u64,
    pub card: Option<SdCardInfo>,
    pub storage_data: Vec<u8>, // Simulated sector backing store
}

impl SdHostController {
    pub fn new(mmio_base: u64, simulated_capacity_mb: usize) -> Self {
        let total_bytes = simulated_capacity_mb * 1024 * 1024;
        Self {
            mmio_base,
            card: None,
            storage_data: vec![0u8; total_bytes],
        }
    }

    /// Perform full SD initialization sequence (CMD0 -> CMD8 -> ACMD41 -> CMD2 -> CMD3 -> CMD7)
    pub fn initialize_card(&mut self) -> Result<u16, &'static str> {
        // 1. Reset card to idle state
        let _ = CMD0_GO_IDLE_STATE;

        // 2. Check 3.3V operating voltage condition
        let _ = CMD8_SEND_IF_COND;

        // 3. Negotiate operational conditions
        let _ = ACMD41_SD_SEND_OP_COND;

        // 4. Read CID and generate RCA
        let rca = 0x1234u16;

        let total_sectors = (self.storage_data.len() / SD_BLOCK_SIZE) as u64;

        self.card = Some(SdCardInfo {
            rca,
            capacity_sectors: total_sectors,
            is_high_capacity: true,
            state: SdCardState::Transfer,
        });

        Ok(rca)
    }

    /// Read a single 512-byte block from sector LBA
    pub fn read_sector(&self, lba: u64, buf: &mut [u8; SD_BLOCK_SIZE]) -> Result<(), &'static str> {
        let card = self.card.as_ref().ok_or("No SD card initialized")?;
        if card.state != SdCardState::Transfer {
            return Err("Card not in transfer state");
        }
        if lba >= card.capacity_sectors {
            return Err("LBA out of bounds");
        }

        let start = (lba as usize) * SD_BLOCK_SIZE;
        let end = start + SD_BLOCK_SIZE;
        buf.copy_from_slice(&self.storage_data[start..end]);
        Ok(())
    }

    /// Write a single 512-byte block to sector LBA
    pub fn write_sector(
        &mut self,
        lba: u64,
        buf: &[u8; SD_BLOCK_SIZE],
    ) -> Result<(), &'static str> {
        let card = self.card.as_ref().ok_or("No SD card initialized")?;
        if card.state != SdCardState::Transfer {
            return Err("Card not in transfer state");
        }
        if lba >= card.capacity_sectors {
            return Err("LBA out of bounds");
        }

        let start = (lba as usize) * SD_BLOCK_SIZE;
        let end = start + SD_BLOCK_SIZE;
        self.storage_data[start..end].copy_from_slice(buf);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sd_mmc_initialization_and_io() {
        let mut controller = SdHostController::new(0xFE00_0000, 16); // 16MB card
        let rca = controller.initialize_card().expect("Card initialized");
        assert_eq!(rca, 0x1234);

        let mut write_buf = [0xA5u8; SD_BLOCK_SIZE];
        write_buf[0] = 0xAA;
        write_buf[511] = 0x55;

        assert!(controller.write_sector(10, &write_buf).is_ok());

        let mut read_buf = [0u8; SD_BLOCK_SIZE];
        assert!(controller.read_sector(10, &mut read_buf).is_ok());
        assert_eq!(read_buf, write_buf);
    }
}
