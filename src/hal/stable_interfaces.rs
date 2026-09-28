// SigmaOS Stable Hardware Abstraction Layer (HAL) Interfaces
// Provides canonical traits for MMIO regions, DMA allocation, Interrupt control, and PCI devices.

use std::fmt;

/// HAL Error representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HalError {
    InvalidOffset(usize),
    AccessDenied,
    HardwareFault,
    IoTimeout,
}

impl fmt::Display for HalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HalError::InvalidOffset(off) => write!(f, "Invalid MMIO offset: {}", off),
            HalError::AccessDenied => write!(f, "MMIO access denied"),
            HalError::HardwareFault => write!(f, "Hardware fault"),
            HalError::IoTimeout => write!(f, "I/O operation timeout"),
        }
    }
}

/// Memory-Mapped I/O Region Trait
pub trait MmioRegion {
    fn read32(&self, offset: usize) -> Result<u32, HalError>;
    fn write32(&mut self, offset: usize, value: u32) -> Result<(), HalError>;
}

/// DMA Error representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DmaError {
    OutOfMemory,
    InvalidAlignment,
    BufferTooLarge,
}

/// Contiguous DMA Buffer descriptor
#[derive(Debug, Clone)]
pub struct DmaBuffer {
    pub physical_address: u64,
    pub virtual_address: *mut u8,
    pub size_bytes: usize,
    pub alignment: usize,
}

unsafe impl Send for DmaBuffer {}
unsafe impl Sync for DmaBuffer {}

/// DMA Allocator Trait
pub trait DmaAllocator {
    fn allocate(&mut self, size: usize, align: usize) -> Result<DmaBuffer, DmaError>;
    fn deallocate(&mut self, buffer: DmaBuffer);
}

/// IRQ Error representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrqError {
    IrqVectorInUse(u8),
    InvalidVector(u8),
    ControllerFailure,
}

/// IRQ Handler function type
pub type IrqHandler = fn(irq: u8) -> bool;

/// Interrupt Controller Trait
pub trait InterruptController {
    fn register(&mut self, irq: u8, handler: IrqHandler) -> Result<(), IrqError>;
    fn unregister(&mut self, irq: u8) -> Result<(), IrqError>;
    fn mask(&mut self, irq: u8);
    fn unmask(&mut self, irq: u8);
}

/// PCI Error representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PciError {
    DeviceNotFound,
    BusMasteringFailed,
    InvalidCommandState,
}

/// PCI Device Trait
pub trait PciDevice {
    fn vendor_id(&self) -> u16;
    fn device_id(&self) -> u16;
    fn enable_bus_mastering(&mut self) -> Result<(), PciError>;
}

/// Sample Concrete Memory MMIO Region Implementation
pub struct ConcreteMmioRegion {
    pub buffer: Vec<u32>,
}

impl ConcreteMmioRegion {
    pub fn new(words: usize) -> Self {
        Self {
            buffer: vec![0; words],
        }
    }
}

impl MmioRegion for ConcreteMmioRegion {
    fn read32(&self, offset: usize) -> Result<u32, HalError> {
        let word_idx = offset / 4;
        if word_idx < self.buffer.len() {
            Ok(self.buffer[word_idx])
        } else {
            Err(HalError::InvalidOffset(offset))
        }
    }

    fn write32(&mut self, offset: usize, value: u32) -> Result<(), HalError> {
        let word_idx = offset / 4;
        if word_idx < self.buffer.len() {
            self.buffer[word_idx] = value;
            Ok(())
        } else {
            Err(HalError::InvalidOffset(offset))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stable_hal_mmio_region() {
        let mut mmio = ConcreteMmioRegion::new(16);
        assert_eq!(mmio.read32(0).unwrap(), 0);

        mmio.write32(0x04, 0x12345678).unwrap();
        assert_eq!(mmio.read32(0x04).unwrap(), 0x12345678);

        assert!(mmio.read32(0x100).is_err());
    }
}
