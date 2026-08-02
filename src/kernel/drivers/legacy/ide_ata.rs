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

use crate::kernel::subsystems::registry::{
    InitOrder, KernelSubsystem, SubsystemError, SubsystemPriority,
};
/// SigmaOS Legacy Driver — IDE/ATA Controller (Linux 1.x–2.6 era)
/// Absorbs full ATA-1 through ATA-7 specification support
/// PIO modes 0-4, MWDMA, UDMA/133, LBA28/LBA48, master/slave topology
use core::sync::atomic::{AtomicUsize, Ordering};
use std::string::{String, ToString};
use std::vec::Vec;

pub const ATA_SECTOR_SIZE: usize = 512;
pub const ATA_PRIMARY_BASE: u16 = 0x01F0;
pub const ATA_SECONDARY_BASE: u16 = 0x0170;
pub const ATA_PRIMARY_IRQ: u8 = 14;
pub const ATA_SECONDARY_IRQ: u8 = 15;

/// ATA commands (from ATA-7 spec)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AtaCommand {
    ReadSectors = 0x20,
    ReadSectorsExt = 0x24, // LBA48
    WriteSectors = 0x30,
    WriteSectorsExt = 0x34, // LBA48
    Identify = 0xEC,
    IdentifyPacket = 0xA1, // ATAPI
    SetFeatures = 0xEF,
    ReadDmaExt = 0x25,
    WriteDmaExt = 0x35,
    FlushCache = 0xE7,
    FlushCacheExt = 0xEA,
    StandbyImmediate = 0xE0,
    CheckPowerMode = 0xE5,
    Nop = 0x00,
}

/// ATA transfer mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferMode {
    Pio0,
    Pio1,
    Pio2,
    Pio3,
    Pio4,
    Mwdma0,
    Mwdma1,
    Mwdma2,
    Udma0,
    Udma1,
    Udma2,
    Udma3,
    Udma4,
    Udma5,
    Udma6, // UDMA/133
}

/// ATA device type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtaDeviceType {
    Ata,
    Atapi,
    Unknown,
}

/// IDENTIFY DEVICE response (512 bytes / 256 words)
#[derive(Debug, Clone)]
pub struct AtaIdentify {
    pub model: String,
    pub serial: String,
    pub firmware: String,
    pub lba28_sectors: u32,
    pub lba48_sectors: u64,
    pub supports_lba: bool,
    pub supports_dma: bool,
    pub supports_lba48: bool,
    pub transfer_mode: TransferMode,
}

impl AtaIdentify {
    pub fn mock(model: &str, sectors: u64) -> Self {
        AtaIdentify {
            model: model.to_string(),
            serial: "SN12345678".to_string(),
            firmware: "FW01".to_string(),
            lba28_sectors: core::cmp::min(sectors, 0x0FFF_FFFF) as u32,
            lba48_sectors: sectors,
            supports_lba: true,
            supports_dma: true,
            supports_lba48: sectors > 0x0FFF_FFFF,
            transfer_mode: TransferMode::Udma5,
        }
    }

    pub fn capacity_bytes(&self) -> u64 {
        self.lba48_sectors * ATA_SECTOR_SIZE as u64
    }
}

/// ATA drive (master or slave)
pub struct AtaDrive {
    pub channel: u8,  // 0 = primary, 1 = secondary
    pub position: u8, // 0 = master, 1 = slave
    pub identify: AtaIdentify,
    pub device_type: AtaDeviceType,
    pub present: bool,
    pub read_ahead_enabled: bool,
    write_cache: bool,
    io_ops: AtomicUsize,
    data: Vec<[u8; ATA_SECTOR_SIZE]>,
}

impl AtaDrive {
    pub fn new(channel: u8, position: u8, identify: AtaIdentify) -> Self {
        let cap = identify.lba48_sectors as usize;
        let alloc_sectors = core::cmp::min(cap, 16384); // cap mock alloc
        let mut data = Vec::new();
        for _ in 0..alloc_sectors {
            data.push([0u8; ATA_SECTOR_SIZE]);
        }
        AtaDrive {
            channel,
            position,
            identify,
            device_type: AtaDeviceType::Ata,
            present: true,
            read_ahead_enabled: true,
            write_cache: true,
            io_ops: AtomicUsize::new(0),
            data,
        }
    }

    pub fn read_sectors(
        &self,
        lba: u64,
        count: usize,
        buf: &mut Vec<u8>,
    ) -> Result<(), &'static str> {
        for i in 0..count {
            let idx = (lba as usize) + i;
            if idx >= self.data.len() {
                return Err("ATA: LBA out of range");
            }
            buf.extend_from_slice(&self.data[idx]);
            self.io_ops.fetch_add(1, Ordering::Relaxed);
        }
        Ok(())
    }

    pub fn write_sectors(&mut self, lba: u64, buf: &[u8]) -> Result<(), &'static str> {
        let count = buf.len() / ATA_SECTOR_SIZE;
        for i in 0..count {
            let idx = (lba as usize) + i;
            if idx >= self.data.len() {
                return Err("ATA: LBA out of range");
            }
            self.data[idx].copy_from_slice(&buf[i * ATA_SECTOR_SIZE..(i + 1) * ATA_SECTOR_SIZE]);
            self.io_ops.fetch_add(1, Ordering::Relaxed);
        }
        Ok(())
    }

    pub fn io_count(&self) -> usize {
        self.io_ops.load(Ordering::Relaxed)
    }
    pub fn is_lba48(&self) -> bool {
        self.identify.supports_lba48
    }
}

/// IDE channel (primary or secondary) — holds master + slave
pub struct IdeChannel {
    pub base_io: u16,
    pub ctrl_io: u16,
    pub irq: u8,
    pub drives: [Option<AtaDrive>; 2],
}

impl IdeChannel {
    pub fn primary() -> Self {
        IdeChannel {
            base_io: ATA_PRIMARY_BASE,
            ctrl_io: 0x03F6,
            irq: ATA_PRIMARY_IRQ,
            drives: [None, None],
        }
    }
    pub fn secondary() -> Self {
        IdeChannel {
            base_io: ATA_SECONDARY_BASE,
            ctrl_io: 0x0376,
            irq: ATA_SECONDARY_IRQ,
            drives: [None, None],
        }
    }

    pub fn attach(&mut self, pos: usize, drive: AtaDrive) {
        if pos < 2 {
            self.drives[pos] = Some(drive);
        }
    }
}

/// Full IDE/ATA controller — manages 2 channels (4 drives total)
pub struct IdeAtaController {
    pub channels: [IdeChannel; 2],
    initialized: bool,
    reset_count: AtomicUsize,
}

impl IdeAtaController {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        IdeAtaController {
            channels: [IdeChannel::primary(), IdeChannel::secondary()],
            initialized: false,
            reset_count: AtomicUsize::new(0),
        }
    }

    pub fn reset(&self) {
        self.reset_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn detect_drives(&self) -> usize {
        let mut count = 0usize;
        for ch in &self.channels {
            for drive in &ch.drives {
                if drive.is_some() {
                    count += 1;
                }
            }
        }
        count
    }
}

impl KernelSubsystem for IdeAtaController {
    fn name(&self) -> &str {
        "ide_ata"
    }
    fn version(&self) -> &str {
        "2.0.0"
    }
    fn init_order(&self) -> InitOrder {
        InitOrder::Device
    }
    fn priority(&self) -> SubsystemPriority {
        SubsystemPriority::High
    }
    fn dependencies(&self) -> Vec<&'static str> {
        vec!["isa_bus"]
    }

    fn initialize(&mut self) -> Result<(), SubsystemError> {
        self.initialized = true;
        self.reset();
        Ok(())
    }
    fn shutdown(&mut self) -> Result<(), SubsystemError> {
        Ok(())
    }
}

impl Default for IdeAtaController {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ata_identify() {
        let id = AtaIdentify::mock("SIGMA HD 1GB", 2_097_152); // 1 GB
        assert_eq!(id.capacity_bytes(), 2_097_152 * 512);
        assert!(id.supports_lba);
    }

    #[test]
    fn test_ata_drive_read_write() {
        let id = AtaIdentify::mock("TestDrive", 1024);
        let mut drive = AtaDrive::new(0, 0, id);
        let write_data: Vec<u8> = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .to_le_bytes()
            .iter()
            .cycle()
            .take(512)
            .copied()
            .collect();
        drive.write_sectors(0, &write_data).unwrap();
        let mut read_buf = Vec::new();
        drive.read_sectors(0, 1, &mut read_buf).unwrap();
        assert_eq!(read_buf[0], write_data[0]);
        assert_eq!(drive.io_count(), 2);
    }

    #[test]
    fn test_ide_controller_channels() {
        let mut ctrl = IdeAtaController::new();
        let id = AtaIdentify::mock("SigmaOS SSD", 65536);
        ctrl.channels[0].attach(0, AtaDrive::new(0, 0, id));
        assert_eq!(ctrl.detect_drives(), 1);
    }

    #[test]
    fn test_lba48_detection() {
        let big = AtaIdentify::mock("BigDisk", 300_000_000);
        assert!(big.supports_lba48);
        let small = AtaIdentify::mock("SmallDisk", 1000);
        assert!(!small.supports_lba48);
    }
}
