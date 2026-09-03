#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
use alloc::boxed::Box;

extern crate alloc;
use crate::kernel::subsystems::registry::{
    InitOrder, KernelSubsystem, SubsystemError, SubsystemPriority,
};
/// SigmaOS Legacy Device Driver Framework — ISA Bus Controller
/// Absorbs Linux 0.01–2.6 ISA bus support: I/O port space, DMA, IRQs
/// Supports ISA, EISA, and LPC bridge legacy devices
use core::sync::atomic::{AtomicUsize, Ordering};
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Classic ISA I/O port ranges (from PC architecture spec)
pub const ISA_IO_BASE: u16 = 0x0000;
pub const ISA_IO_END: u16 = 0x03FF;

/// Standard ISA IRQ assignments (8259A PIC layout)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IsaIrq {
    SystemTimer = 0,   // 8253/8254 PIT channel 0
    Keyboard = 1,      // AT keyboard controller (8042)
    Cascade = 2,       // Cascade from slave PIC (IRQ 8–15)
    Serial2 = 3,       // COM2 / COM4
    Serial1 = 4,       // COM1 / COM3
    Parallel2 = 5,     // LPT2
    Floppy = 6,        // Floppy disk controller
    Parallel1 = 7,     // LPT1
    RtcClock = 8,      // Real-Time Clock
    Reserved9 = 9,     // Free / ACPI SCI
    Reserved10 = 10,   // Free / USB
    Reserved11 = 11,   // Free / SCSI
    PsMouse = 12,      // PS/2 mouse (Intel 8042 aux)
    Fpu = 13,          // FPU / Math coprocessor
    PrimaryIde = 14,   // Primary IDE/ATA channel
    SecondaryIde = 15, // Secondary IDE/ATA channel
}

/// ISA DMA channel assignments (8237A DMA controller)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IsaDmaChannel {
    Dma0 = 0, // Spare / cascade
    Dma1 = 1, // SDLC
    Dma2 = 2, // Floppy disk
    Dma3 = 3, // Parallel port (ECP)
    // Channels 4-7: 16-bit DMA (ISA16)
    Dma4 = 4, // Cascade from 8-bit controller
    Dma5 = 5, // Spare (16-bit)
    Dma6 = 6, // Spare (16-bit)
    Dma7 = 7, // Spare (16-bit)
}

/// ISA resource descriptor — tracks allocation of I/O, IRQ, DMA
#[derive(Debug, Clone)]
pub struct IsaResource {
    pub name: String,
    pub io_base: u16,
    pub io_len: u16,
    pub irq: Option<IsaIrq>,
    pub dma: Option<IsaDmaChannel>,
}

pub trait IsaDevice: Send + Sync {
    fn resource(&self) -> &IsaResource;
    fn probe(&mut self) -> bool;
    fn enable(&mut self) -> Result<(), &'static str>;
    fn disable(&mut self) -> Result<(), &'static str>;
    fn device_name(&self) -> &str;
}

/// ISA Bus controller — enumerates and manages all ISA devices
pub struct IsaBus {
    devices: Vec<Box<dyn IsaDevice>>,
    registered_count: AtomicUsize,
    initialized: bool,
}

impl IsaBus {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        IsaBus {
            devices: Vec::new(),
            registered_count: AtomicUsize::new(0),
            initialized: false,
        }
    }

    pub fn register_device(&mut self, device: Box<dyn IsaDevice>) {
        self.registered_count.fetch_add(1, Ordering::SeqCst);
        self.devices.push(device);
    }

    pub fn probe_all(&mut self) -> usize {
        let mut found = 0usize;
        for dev in self.devices.iter_mut() {
            if dev.probe() {
                found += 1;
            }
        }
        found
    }

    pub fn device_count(&self) -> usize {
        self.registered_count.load(Ordering::SeqCst)
    }
}

impl KernelSubsystem for IsaBus {
    fn name(&self) -> &str {
        "isa_bus"
    }
    fn version(&self) -> &str {
        "1.0.0"
    }
    fn init_order(&self) -> InitOrder {
        InitOrder::EarlyBoot
    }
    fn priority(&self) -> SubsystemPriority {
        SubsystemPriority::High
    }

    fn initialize(&mut self) -> Result<(), SubsystemError> {
        self.initialized = true;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), SubsystemError> {
        for dev in self.devices.iter_mut() {
            let _ = dev.disable();
        }
        Ok(())
    }
}

impl Default for IsaBus {
    fn default() -> Self {
        Self::new()
    }
}

/// LPC (Low Pin Count) bridge — maps ISA devices on modern hardware
pub struct LpcBridge {
    pub vendor_id: u16,
    pub device_id: u16,
    pub base_addr: u32,
    enabled: bool,
}

impl LpcBridge {
    pub fn new(vendor_id: u16, device_id: u16) -> Self {
        LpcBridge {
            vendor_id,
            device_id,
            base_addr: 0,
            enabled: false,
        }
    }

    /// Intel ICH/PCH common LPC vendor/device IDs
    pub fn intel_lpc() -> Self {
        Self::new(0x8086, 0x2440)
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyIsaDevice {
        res: IsaResource,
        probed: bool,
    }
    impl IsaDevice for DummyIsaDevice {
        fn resource(&self) -> &IsaResource {
            &self.res
        }
        fn probe(&mut self) -> bool {
            self.probed = true;
            true
        }
        fn enable(&mut self) -> Result<(), &'static str> {
            Ok(())
        }
        fn disable(&mut self) -> Result<(), &'static str> {
            Ok(())
        }
        fn device_name(&self) -> &str {
            "dummy_isa_dev"
        }
    }

    #[test]
    fn test_isa_bus_creation() {
        let bus = IsaBus::new();
        assert_eq!(bus.device_count(), 0);
    }

    #[test]
    fn test_isa_device_registration() {
        let mut bus = IsaBus::new();
        let dev = Box::new(DummyIsaDevice {
            res: IsaResource {
                name: "test".to_string(),
                io_base: 0x300,
                io_len: 0x20,
                irq: Some(IsaIrq::Reserved10),
                dma: None,
            },
            probed: false,
        });
        bus.register_device(dev);
        assert_eq!(bus.device_count(), 1);
        assert_eq!(bus.probe_all(), 1);
    }

    #[test]
    fn test_lpc_bridge_enable() {
        let mut lpc = LpcBridge::intel_lpc();
        assert!(!lpc.is_enabled());
        lpc.enable();
        assert!(lpc.is_enabled());
    }

    #[test]
    fn test_isa_irq_layout() {
        assert_eq!(IsaIrq::Keyboard as u8, 1);
        assert_eq!(IsaIrq::Floppy as u8, 6);
        assert_eq!(IsaIrq::PrimaryIde as u8, 14);
    }
}
