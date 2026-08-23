#![no_std]
#![cfg_attr(target_os = "none", no_main)]

extern crate alloc;
use alloc::boxed::Box;

use core::mem;
/// OOP-based Device Driver Framework for SigmaOS
/// Implements device drivers using OOP principles with traits and structs
/// No dependency on external driver frameworks
use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone)]
pub struct DdeDeviceWrapper {
    pub id: u32,
    pub name: [u8; 32],
    pub io_base: u16,
    pub persona: [u8; 16],
    pub simulated_pci_bar: u32,
}

impl DdeDeviceWrapper {
    pub fn new(id: u32, name: &[u8], io_base: u16, persona: &[u8]) -> Self {
        let mut name_arr = [0u8; 32];
        let mut persona_arr = [0u8; 16];
        let nlen = name.len().min(31);
        let plen = persona.len().min(15);
        name_arr[..nlen].copy_from_slice(&name[..nlen]);
        persona_arr[..plen].copy_from_slice(&persona[..plen]);
        Self {
            id,
            name: name_arr,
            io_base,
            persona: persona_arr,
            simulated_pci_bar: io_base as u32,
        }
    }
}

/// Device trait (OOP interface)
pub trait Device {
    /// Initialize device
    fn init(&mut self) -> Result<(), DeviceError>;
    /// Read from device
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, DeviceError>;
    /// Write to device
    fn write(&mut self, buffer: &[u8]) -> Result<usize, DeviceError>;
    /// Control device (ioctl)
    fn ioctl(&mut self, command: u32, arg: usize) -> Result<usize, DeviceError>;
    /// Get device info
    fn info(&self) -> DeviceInfo;
    /// Shutdown device
    fn shutdown(&mut self) -> Result<(), DeviceError>;
}

/// Device error types
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
#[derive(Clone, Copy)]
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
#[repr(C)]
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

/// Device descriptor (OOP: Device object)
#[repr(C)]
pub struct DeviceDescriptor {
    pub id: usize,
    pub name: [u8; 64],
    pub device_type: DeviceType,
    pub capability: DeviceCapability,
    pub state: AtomicUsize, // DeviceState as usize
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
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), len);
        }

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
        unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) }
    }

    pub fn set_state(&self, state: DeviceState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }

    pub fn increment_ref(&self) {
        self.reference_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn decrement_ref(&self) -> usize {
        self.reference_count.fetch_sub(1, Ordering::SeqCst) - 1
    }
}

/// Device state
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceState {
    Uninitialized = 0,
    Initializing = 1,
    Ready = 2,
    Busy = 3,
    Error = 4,
    Shutdown = 5,
}

/// Block device trait (OOP: Interface for block devices)
pub trait BlockDevice: Device {
    fn read_block(&mut self, block: u64, buffer: &mut [u8]) -> Result<(), DeviceError>;
    fn write_block(&mut self, block: u64, buffer: &[u8]) -> Result<(), DeviceError>;
    fn block_size(&self) -> usize;
    fn total_blocks(&self) -> u64;
}

/// Character device trait (OOP: Interface for character devices)
pub trait CharacterDevice: Device {
    fn read_char(&mut self) -> Result<u8, DeviceError>;
    fn write_char(&mut self, c: u8) -> Result<(), DeviceError>;
    fn flush(&mut self) -> Result<(), DeviceError>;
}

/// Network device trait (OOP: Interface for network devices)
pub trait NetworkDevice: Device {
    fn send_packet(&mut self, packet: &[u8]) -> Result<(), DeviceError>;
    fn receive_packet(&mut self, buffer: &mut [u8]) -> Result<usize, DeviceError>;
    fn get_mac_address(&self) -> [u8; 6];
    fn set_mac_address(&mut self, mac: [u8; 6]) -> Result<(), DeviceError>;
}

/// Simple block device implementation (OOP: Concrete class)
pub struct SimpleBlockDevice {
    descriptor: DeviceDescriptor,
    blocks: Vec<Vec<u8>>,
    block_size: usize,
    info: DeviceInfo,
}

impl SimpleBlockDevice {
    pub fn new(id: usize, name: &[u8], num_blocks: usize, block_size: usize) -> Self {
        let capability = DeviceCapability {
            can_read: true,
            can_write: true,
            can_mmap: true,
            can_dma: true,
            can_interrupt: false,
        };

        let descriptor = DeviceDescriptor::new(id, name, DeviceType::Block, capability);
        let mut blocks = Vec::new();

        for _ in 0..num_blocks {
            let mut block_data = Vec::new();
            for _ in 0..block_size {
                block_data.push(0);
            }
            blocks.push(block_data);
        }

        let mut info = DeviceInfo::new(DeviceType::Block);
        info.vendor_id = 0x8086; // Intel generic block
        info.device_id = 0x100E;

        SimpleBlockDevice {
            descriptor,
            blocks,
            block_size,
            info,
        }
    }
}

impl Device for SimpleBlockDevice {
    fn init(&mut self) -> Result<(), DeviceError> {
        if self.descriptor.get_state() == DeviceState::Ready {
            return Err(DeviceError::AlreadyInitialized);
        }

        self.descriptor.set_state(DeviceState::Initializing);
        self.descriptor.set_state(DeviceState::Ready);
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, DeviceError> {
        if !self.descriptor.capability.can_read {
            return Err(DeviceError::NotSupported);
        }
        Ok(buffer.len())
    }

    fn write(&mut self, buffer: &[u8]) -> Result<usize, DeviceError> {
        if !self.descriptor.capability.can_write {
            return Err(DeviceError::NotSupported);
        }
        Ok(buffer.len())
    }

    fn ioctl(&mut self, _command: u32, _arg: usize) -> Result<usize, DeviceError> {
        Ok(0)
    }

    fn info(&self) -> DeviceInfo {
        self.info
    }

    fn shutdown(&mut self) -> Result<(), DeviceError> {
        self.descriptor.set_state(DeviceState::Shutdown);
        Ok(())
    }
}

