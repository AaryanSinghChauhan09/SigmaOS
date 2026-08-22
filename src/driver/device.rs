// OOP-based Device Driver Framework for SigmaOS
// Implements device drivers using OOP principles with traits and structs

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Device trait (OOP interface)
pub trait Device {
    fn init(&mut self) -> Result<(), DeviceError>;
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, DeviceError>;
    fn write(&mut self, buffer: &[u8]) -> Result<usize, DeviceError>;
    fn ioctl(&mut self, command: u32, arg: usize) -> Result<usize, DeviceError>;
    fn info(&self) -> DeviceInfo;
    fn shutdown(&mut self) -> Result<(), DeviceError>;
}

/// Device error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceError {
    Success = 0,
    NotInitialized = 1,
    AlreadyInitialized = 2,
    Busy = 3,
    InvalidParameter = 4,
    IoError = 5,
    NotSupported = 6,
    Timeout = 7,
}

/// Device type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Block = 0,
    Character = 1,
    Network = 2,
    Graphics = 3,
    Input = 4,
    Audio = 5,
}

/// Device info
#[derive(Debug, Clone, Copy)]
pub struct DeviceInfo {
    pub device_type: DeviceType,
    pub vendor_id: u16,
    pub device_id: u16,
    pub revision: u8,
    pub irq: u8,
    pub dma: u8,
    pub base_address: u32,
    pub memory_size: usize,
}

impl DeviceInfo {
    pub fn new(device_type: DeviceType) -> Self {
        DeviceInfo {
            device_type,
            vendor_id: 0,
            device_id: 0,
            revision: 0,
            irq: 0,
            dma: 0,
            base_address: 0,
            memory_size: 0,
        }
    }
}

/// Device capability
#[derive(Debug, Clone, Copy)]
pub struct DeviceCapability {
    pub can_read: bool,
    pub can_write: bool,
    pub can_mmap: bool,
    pub can_dma: bool,
    pub can_interrupt: bool,
}

impl DeviceCapability {
    pub fn new() -> Self {
        DeviceCapability {
            can_read: false,
            can_write: false,
            can_mmap: false,
            can_dma: false,
            can_interrupt: false,
        }
    }

    pub fn full() -> Self {
        DeviceCapability {
            can_read: true,
            can_write: true,
            can_mmap: true,
            can_dma: true,
            can_interrupt: true,
        }
    }
}

impl Default for DeviceCapability {
    fn default() -> Self {
        Self::new()
    }
}

/// Device state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceState {
    Uninitialized = 0,
    Initializing = 1,
    Ready = 2,
    Busy = 3,
    Error = 4,
    Shutdown = 5,
}

/// Device descriptor
pub struct DeviceDescriptor {
    pub id: usize,
    pub name: [u8; 64],
    pub device_type: DeviceType,
    pub capability: DeviceCapability,
    pub state: AtomicUsize,
    pub reference_count: AtomicUsize,
}

impl DeviceDescriptor {
    pub fn new(
        id: usize,
        name: &[u8],
        device_type: DeviceType,
        capability: DeviceCapability,
    ) -> Self {
        let mut name_array = [0u8; 64];
        let len = name.len().min(63);
        name_array[..len].copy_from_slice(&name[..len]);

        DeviceDescriptor {
            id,
            name: name_array,
            device_type,
            capability,
            state: AtomicUsize::new(DeviceState::Uninitialized as usize),
            reference_count: AtomicUsize::new(0),
        }
    }

    pub fn get_state(&self) -> DeviceState {
        match self.state.load(Ordering::SeqCst) {
            1 => DeviceState::Initializing,
            2 => DeviceState::Ready,
            3 => DeviceState::Busy,
            4 => DeviceState::Error,
            5 => DeviceState::Shutdown,
            _ => DeviceState::Uninitialized,
        }
    }

    pub fn set_state(&self, state: DeviceState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }
}

/// Foreign driver wrapper running in the Device Driver Environment (DDE) translation layer.
pub struct DdeDeviceWrapper {
    pub id: usize,
    pub name: [u8; 64],
    pub base_addr: u32,
    pub simulated_pci_bar: [u8; 256],
    pub foreign_os_type: [u8; 16],
}

impl DdeDeviceWrapper {
    pub fn new(id: usize, name: &[u8], base_addr: u32, os_type: &[u8]) -> Self {
        let mut name_array = [0u8; 64];
        let len = name.len().min(63);
        name_array[..len].copy_from_slice(&name[..len]);

        let mut os_array = [0u8; 16];
        let os_len = os_type.len().min(15);
        os_array[..os_len].copy_from_slice(&os_type[..os_len]);

        DdeDeviceWrapper {
            id,
            name: name_array,
            base_addr,
            simulated_pci_bar: [0u8; 256],
            foreign_os_type: os_array,
        }
    }
}

impl Device for DdeDeviceWrapper {
    fn init(&mut self) -> Result<(), DeviceError> {
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, DeviceError> {
        let len = buffer.len().min(self.simulated_pci_bar.len());
        buffer[..len].copy_from_slice(&self.simulated_pci_bar[..len]);
        Ok(len)
    }

    fn write(&mut self, buffer: &[u8]) -> Result<usize, DeviceError> {
        let len = buffer.len().min(self.simulated_pci_bar.len());
        self.simulated_pci_bar[..len].copy_from_slice(&buffer[..len]);
        Ok(len)
    }

    fn ioctl(&mut self, command: u32, _arg: usize) -> Result<usize, DeviceError> {
        if command == 0xFF {
            Ok(1)
        } else {
            Ok(0)
        }
    }

    fn info(&self) -> DeviceInfo {
        let mut info = DeviceInfo::new(DeviceType::Character);
        info.base_address = self.base_addr;
        info.vendor_id = 0x8086;
        info.device_id = 0x100e;
        info
    }

    fn shutdown(&mut self) -> Result<(), DeviceError> {
        Ok(())
    }
}

/// Block device trait
pub trait BlockDevice: Device {
    fn read_block(&mut self, block: u64, buffer: &mut [u8]) -> Result<(), DeviceError>;
    fn write_block(&mut self, block: u64, buffer: &[u8]) -> Result<(), DeviceError>;
    fn block_size(&self) -> usize;
    fn total_blocks(&self) -> u64;
}

pub trait CharacterDevice: Device {
    fn read_char(&mut self) -> Result<u8, DeviceError>;
    fn write_char(&mut self, ch: u8) -> Result<(), DeviceError>;
    fn flush(&mut self) -> Result<(), DeviceError> { Ok(()) }
}

pub trait NetworkDevice: Device {
    fn send_packet(&mut self, packet: &[u8]) -> Result<(), DeviceError>;
    fn receive_packet(&mut self, buffer: &mut [u8]) -> Result<usize, DeviceError>;
    fn mac_address(&self) -> [u8; 6];
    fn get_mac_address(&self) -> [u8; 6] { self.mac_address() }
    fn set_mac_address(&mut self, _mac: [u8; 6]) {}
}

