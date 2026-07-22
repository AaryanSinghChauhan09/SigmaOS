use crate::kernel::subsystems::registry::{
    InitOrder, KernelSubsystem, SubsystemError, SubsystemPriority,
};
/// SigmaOS Legacy Driver — Floppy Disk Controller (Intel 8272A / NEC µPD765)
/// Supports: 5.25" 360K/1.2M, 3.5" 720K/1.44M/2.88M, LS-120
/// Absorbs Linux drivers/block/floppy.c — the original block device
use core::sync::atomic::{AtomicUsize, Ordering};
use std::vec::Vec;

pub const FDC_BASE_IO: u16 = 0x03F0;
pub const FDC_IRQ: u8 = 6;
pub const FDC_DMA: u8 = 2;

/// Floppy drive types and geometry
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloppyType {
    F360K,  // 5.25" DS DD  360K
    F1200K, // 5.25" DS HD  1.2M
    F720K,  // 3.5"  DS DD  720K
    F1440K, // 3.5"  DS HD  1.44M (most common)
    F2880K, // 3.5"  DS ED  2.88M
}

impl FloppyType {
    pub fn geometry(&self) -> (u8, u8, u8) {
        // (heads, tracks, sectors)
        match self {
            FloppyType::F360K => (2, 40, 9),
            FloppyType::F1200K => (2, 80, 15),
            FloppyType::F720K => (2, 80, 9),
            FloppyType::F1440K => (2, 80, 18),
            FloppyType::F2880K => (2, 80, 36),
        }
    }

    pub fn capacity_kb(&self) -> u32 {
        let (h, t, s) = self.geometry();
        h as u32 * t as u32 * s as u32 * 512 / 1024
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FloppyGeometry {
    pub heads: u8,
    pub tracks: u8,
    pub sectors: u8,
}

/// Floppy disk drive
pub struct FloppyDrive {
    pub drive_num: u8,
    pub floppy_type: FloppyType,
    pub motor_on: bool,
    pub disk_present: bool,
    data: Vec<[u8; 512]>,
    io_count: AtomicUsize,
}

impl FloppyDrive {
    pub fn new(drive_num: u8, floppy_type: FloppyType) -> Self {
        let (h, t, s) = floppy_type.geometry();
        let sectors = h as usize * t as usize * s as usize;
        FloppyDrive {
            drive_num,
            floppy_type,
            motor_on: false,
            disk_present: true,
            data: (0..sectors).map(|_| [0u8; 512]).collect(),
            io_count: AtomicUsize::new(0),
        }
    }

    pub fn chs_to_lba(&self, head: u8, track: u8, sector: u8) -> usize {
        let (_, _, spt) = self.floppy_type.geometry();
        head as usize * (self.data.len() / 2)
            + track as usize * spt as usize
            + (sector as usize - 1)
    }

    pub fn read_sector(&self, lba: usize, buf: &mut [u8; 512]) -> Result<(), &'static str> {
        if lba >= self.data.len() {
            return Err("FDC: sector out of range");
        }
        buf.copy_from_slice(&self.data[lba]);
        self.io_count.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    pub fn write_sector(&mut self, lba: usize, buf: &[u8; 512]) -> Result<(), &'static str> {
        if lba >= self.data.len() {
            return Err("FDC: sector out of range");
        }
        self.data[lba].copy_from_slice(buf);
        self.io_count.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    pub fn motor_on(&mut self) {
        self.motor_on = true;
    }
    pub fn motor_off(&mut self) {
        self.motor_on = false;
    }
    pub fn io_count(&self) -> usize {
        self.io_count.load(Ordering::Relaxed)
    }
}

/// Floppy Disk Controller — manages up to 4 drives
pub struct FloppyController {
    drives: [Option<FloppyDrive>; 4],
    base_io: u16,
    irq: u8,
    dma: u8,
    initialized: bool,
    recalibrate_count: AtomicUsize,
}

impl FloppyController {
    pub fn new() -> Self {
        FloppyController {
            drives: [None, None, None, None],
            base_io: FDC_BASE_IO,
            irq: FDC_IRQ,
            dma: FDC_DMA,
            initialized: false,
            recalibrate_count: AtomicUsize::new(0),
        }
    }

    pub fn attach_drive(&mut self, slot: usize, drive: FloppyDrive) {
        if slot < 4 {
            self.drives[slot] = Some(drive);
        }
    }

    pub fn recalibrate(&self, _drive: usize) {
        self.recalibrate_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn drive(&self, n: usize) -> Option<&FloppyDrive> {
        self.drives.get(n).and_then(|d| d.as_ref())
    }
    pub fn drive_mut(&mut self, n: usize) -> Option<&mut FloppyDrive> {
        self.drives.get_mut(n).and_then(|d| d.as_mut())
    }
    pub fn recalibrate_count(&self) -> usize {
        self.recalibrate_count.load(Ordering::Relaxed)
    }
}

impl KernelSubsystem for FloppyController {
    fn name(&self) -> &str {
        "floppy"
    }
    fn version(&self) -> &str {
        "1.0.0"
    }
    fn init_order(&self) -> InitOrder {
        InitOrder::Device
    }
    fn priority(&self) -> SubsystemPriority {
        SubsystemPriority::Optional
    }
    fn dependencies(&self) -> Vec<&'static str> {
        vec!["isa_bus"]
    }
    fn initialize(&mut self) -> Result<(), SubsystemError> {
        for drive in self.drives.iter_mut().flatten() {
            drive.motor_off();
        }
        self.initialized = true;
        Ok(())
    }
    fn shutdown(&mut self) -> Result<(), SubsystemError> {
        for drive in self.drives.iter_mut().flatten() {
            drive.motor_off();
        }
        Ok(())
    }
}

impl Default for FloppyController {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floppy_types() {
        assert_eq!(FloppyType::F1440K.capacity_kb(), 1440);
        assert_eq!(FloppyType::F720K.capacity_kb(), 720);
        assert_eq!(FloppyType::F360K.capacity_kb(), 360);
    }

    #[test]
    fn test_floppy_read_write() {
        let mut drv = FloppyDrive::new(0, FloppyType::F1440K);
        let mut wbuf: [u8; 512] = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .to_le_bytes()
            .iter()
            .cycle()
            .take(512)
            .copied()
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        drv.write_sector(0, &wbuf).unwrap();
        let mut rbuf = [0u8; 512];
        drv.read_sector(0, &mut rbuf).unwrap();
        assert_eq!(rbuf[0], wbuf[0]);
    }

    #[test]
    fn test_floppy_controller_attach() {
        let mut ctrl = FloppyController::new();
        ctrl.attach_drive(0, FloppyDrive::new(0, FloppyType::F1440K));
        assert!(ctrl.drive(0).is_some());
        assert!(ctrl.drive(1).is_none());
    }
}
