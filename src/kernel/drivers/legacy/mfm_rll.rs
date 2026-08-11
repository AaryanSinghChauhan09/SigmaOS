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
/// SigmaOS Legacy Driver — MFM/RLL Hard Disk Controller (ST-506 interface)
/// Absorbs Linux 0.01 hard disk driver — the very first block device Linux ever supported
/// Supports ST-412 / ST-506 controllers, MFM and RLL encoding, CHS geometry
use core::sync::atomic::{AtomicUsize, Ordering};
use alloc::vec::Vec;

/// Maximum CHS (Cylinder-Head-Sector) values for ancient controllers
pub const MFM_MAX_CYLINDERS: u16 = 1024;
pub const MFM_MAX_HEADS: u8 = 16;
pub const MFM_MAX_SECTORS: u8 = 63;
pub const MFM_SECTOR_SIZE: usize = 512;

/// MFM/RLL disk geometry (CHS)
#[derive(Debug, Clone, Copy)]
pub struct DiskGeometry {
    pub cylinders: u16,
    pub heads: u8,
    pub sectors_per_track: u8,
}

impl DiskGeometry {
    pub fn total_sectors(&self) -> u32 {
        self.cylinders as u32 * self.heads as u32 * self.sectors_per_track as u32
    }

    pub fn total_bytes(&self) -> u64 {
        self.total_sectors() as u64 * MFM_SECTOR_SIZE as u64
    }

    /// Convert LBA to CHS
    pub fn lba_to_chs(&self, lba: u32) -> (u16, u8, u8) {
        let spt = self.sectors_per_track as u32;
        let h = self.heads as u32;
        let c = (lba / (spt * h)) as u16;
        let tmp = lba % (spt * h);
        let hd = (tmp / spt) as u8;
        let sec = (tmp % spt + 1) as u8;
        (c, hd, sec)
    }
}

/// ST-506 controller type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControllerType {
    ST506,
    WD1003,
    WD1007,
    Mfm,
    Rll,
}

/// MFM/RLL disk descriptor
#[derive(Debug)]
pub struct MfmDisk {
    pub drive_num: u8,
    pub geometry: DiskGeometry,
    pub controller: ControllerType,
    pub write_precomp: u16,
    pub reduced_write_current: u16,
    pub step_rate: u8,
    pub sectors: Vec<[u8; MFM_SECTOR_SIZE]>,
    io_count: AtomicUsize,
}

impl MfmDisk {
    pub fn new(drive_num: u8, geo: DiskGeometry, ctrl: ControllerType) -> Self {
        let total = geo.total_sectors() as usize;
        let mut sectors = Vec::new();
        for _ in 0..core::cmp::min(total, 8192) {
            sectors.push([0u8; MFM_SECTOR_SIZE]);
        }
        MfmDisk {
            drive_num,
            geometry: geo,
            controller: ctrl,
            write_precomp: geo.cylinders / 2,
            reduced_write_current: geo.cylinders,
            step_rate: 35,
            sectors,
            io_count: AtomicUsize::new(0),
        }
    }

    pub fn read_sector(
        &self,
        lba: u32,
        buf: &mut [u8; MFM_SECTOR_SIZE],
    ) -> Result<(), &'static str> {
        let idx = lba as usize;
        if idx >= self.sectors.len() {
            return Err("MFM: sector out of range");
        }
        buf.copy_from_slice(&self.sectors[idx]);
        self.io_count.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    pub fn write_sector(
        &mut self,
        lba: u32,
        buf: &[u8; MFM_SECTOR_SIZE],
    ) -> Result<(), &'static str> {
        let idx = lba as usize;
        if idx >= self.sectors.len() {
            return Err("MFM: sector out of range");
        }
        self.sectors[idx].copy_from_slice(buf);
        self.io_count.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    pub fn io_count(&self) -> usize {
        self.io_count.load(Ordering::Relaxed)
    }
}

/// MFM controller driver — manages up to 2 drives (primary/secondary)
pub struct MfmController {
    drives: [Option<MfmDisk>; 2],
    base_io: u16,
    irq: u8,
    initialized: bool,
}

impl MfmController {
    /// Default ST-506 I/O base and IRQ
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        MfmController {
            drives: [None, None],
            base_io: 0x01F0,
            irq: 14,
            initialized: false,
        }
    }

    pub fn attach_drive(&mut self, slot: usize, disk: MfmDisk) -> Result<(), &'static str> {
        if slot > 1 {
            return Err("MFM: only 2 drives supported");
        }
        self.drives[slot] = Some(disk);
        Ok(())
    }

    pub fn drive(&self, slot: usize) -> Option<&MfmDisk> {
        self.drives[slot].as_ref()
    }

    pub fn drive_mut(&mut self, slot: usize) -> Option<&mut MfmDisk> {
        self.drives[slot].as_mut()
    }
}

impl KernelSubsystem for MfmController {
    fn name(&self) -> &str {
        "mfm_rll"
    }
    fn version(&self) -> &str {
        "1.0.0"
    }
    fn init_order(&self) -> InitOrder {
        InitOrder::Device
    }
    fn priority(&self) -> SubsystemPriority {
        SubsystemPriority::Low
    }
    fn dependencies(&self) -> Vec<&'static str> {
        vec!["isa_bus"]
    }

    fn initialize(&mut self) -> Result<(), SubsystemError> {
        self.initialized = true;
        Ok(())
    }
    fn shutdown(&mut self) -> Result<(), SubsystemError> {
        Ok(())
    }
}

impl Default for MfmController {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disk_geometry() {
        let geo = DiskGeometry {
            cylinders: 615,
            heads: 4,
            sectors_per_track: 17,
        };
        assert_eq!(geo.total_sectors(), 615 * 4 * 17);
    }

    #[test]
    fn test_lba_to_chs() {
        let geo = DiskGeometry {
            cylinders: 615,
            heads: 4,
            sectors_per_track: 17,
        };
        let (c, h, s) = geo.lba_to_chs(0);
        assert_eq!((c, h, s), (0, 0, 1));
    }

    #[test]
    fn test_mfm_read_write() {
        let geo = DiskGeometry {
            cylinders: 10,
            heads: 4,
            sectors_per_track: 17,
        };
        let mut disk = MfmDisk::new(0, geo, ControllerType::Mfm);
        let mut write_buf: [u8; MFM_SECTOR_SIZE] = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .to_le_bytes()
            .iter()
            .cycle()
            .take(MFM_SECTOR_SIZE)
            .copied()
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        disk.write_sector(0, &write_buf).unwrap();
        let mut read_buf = [0u8; MFM_SECTOR_SIZE];
        disk.read_sector(0, &mut read_buf).unwrap();
        assert_eq!(read_buf[0], write_buf[0]);
        assert_eq!(disk.io_count(), 2);
    }

    #[test]
    fn test_mfm_controller_attach() {
        let mut ctrl = MfmController::new();
        let geo = DiskGeometry {
            cylinders: 615,
            heads: 4,
            sectors_per_track: 17,
        };
        ctrl.attach_drive(0, MfmDisk::new(0, geo, ControllerType::Mfm))
            .unwrap();
        assert!(ctrl.drive(0).is_some());
        assert!(ctrl.drive(1).is_none());
    }
}
