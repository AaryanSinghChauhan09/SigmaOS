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

/// Unified representation of communication channels (OOP Abstraction)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortAddress {
    PortIO(u16),      // Legacy 16-bit Port I/O (older generations)
    MemoryMapped(u32) // Modern 32/64-bit Memory Mapped I/O (newer generations)
}

/// Unified Peripheral Object-Oriented Interface (OOP Principle)
pub trait UnifiedPeripheral: Device {
    fn query_channel(&self) -> PortAddress;
    fn read_byte(&mut self, offset: u32) -> Result<u8, DeviceError>;
    fn write_byte(&mut self, offset: u32, value: u8) -> Result<(), DeviceError>;
}

/// Legacy implementation of a peripheral using Port I/O
pub struct LegacyDevice {
    pub base_port: u16,
    pub id: usize,
    pub name: [u8; 64],
}

impl LegacyDevice {
    pub fn new(id: usize, name: &[u8], base_port: u16) -> Self {
        let mut name_array = [0u8; 64];
        let len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), len);
        }
        LegacyDevice { base_port, id, name: name_array }
    }
}

impl Device for LegacyDevice {
    fn init(&mut self) -> Result<(), DeviceError> { Ok(()) }
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, DeviceError> {
        for b in buffer.iter_mut() {
            *b = 0;
        }
        Ok(buffer.len())
    }
    fn write(&mut self, buffer: &[u8]) -> Result<usize, DeviceError> { Ok(buffer.len()) }
    fn ioctl(&mut self, _command: u32, _arg: usize) -> Result<usize, DeviceError> { Ok(0) }
    fn info(&self) -> DeviceInfo { DeviceInfo::new(DeviceType::Character) }
    fn shutdown(&mut self) -> Result<(), DeviceError> { Ok(()) }
}

impl UnifiedPeripheral for LegacyDevice {
    fn query_channel(&self) -> PortAddress { PortAddress::PortIO(self.base_port) }
    fn read_byte(&mut self, _offset: u32) -> Result<u8, DeviceError> {
        Ok(0)
    }
    fn write_byte(&mut self, _offset: u32, _value: u8) -> Result<(), DeviceError> {
        Ok(())
    }
}

/// Modern implementation of a peripheral using MMIO
pub struct ModernDevice {
    pub base_address: u32,
    pub id: usize,
    pub name: [u8; 64],
}

impl ModernDevice {
    pub fn new(id: usize, name: &[u8], base_address: u32) -> Self {
        let mut name_array = [0u8; 64];
        let len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), len);
        }
        ModernDevice { base_address, id, name: name_array }
    }
}

impl Device for ModernDevice {
    fn init(&mut self) -> Result<(), DeviceError> { Ok(()) }
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, DeviceError> {
        for b in buffer.iter_mut() {
            *b = 0;
        }
        Ok(buffer.len())
    }
    fn write(&mut self, buffer: &[u8]) -> Result<usize, DeviceError> { Ok(buffer.len()) }
    fn ioctl(&mut self, _command: u32, _arg: usize) -> Result<usize, DeviceError> { Ok(0) }
    fn info(&self) -> DeviceInfo { DeviceInfo::new(DeviceType::Character) }
    fn shutdown(&mut self) -> Result<(), DeviceError> { Ok(()) }
}

impl UnifiedPeripheral for ModernDevice {
    fn query_channel(&self) -> PortAddress { PortAddress::MemoryMapped(self.base_address) }
    fn read_byte(&mut self, offset: u32) -> Result<u8, DeviceError> {
        unsafe {
            let addr = (self.base_address + offset) as *const u8;
            if self.base_address == 0 {
                return Ok(0);
            }
            Ok(core::ptr::read_volatile(addr))
        }
    }
    fn write_byte(&mut self, offset: u32, value: u8) -> Result<(), DeviceError> {
        unsafe {
            let addr = (self.base_address + offset) as *mut u8;
            if self.base_address != 0 {
                core::ptr::write_volatile(addr, value);
            }
            Ok(())
        }
    }
}

/// User-Defined Function (UDF) Interpreter (Custom Bytecode Runner)
pub struct UdfInterpreter {
    pub bytecode: Vec<u8>,
}

impl UdfInterpreter {
    pub fn new(bytecode: &[u8]) -> Self {
        let mut code_vec = Vec::new();
        for &b in bytecode {
            code_vec.push(b);
        }
        UdfInterpreter { bytecode: code_vec }
    }

    pub fn execute(&self, peripheral: &mut dyn UnifiedPeripheral, registers: &mut [u32; 4]) -> Result<(), DeviceError> {
        let mut pc = 0;
        while pc < self.bytecode.len() {
            let op = self.bytecode[pc];
            match op {
                0x01 => {
                    if pc + 2 >= self.bytecode.len() { return Err(DeviceError::InvalidParameter); }
                    let reg_idx = self.bytecode[pc + 1] as usize;
                    let offset = self.bytecode[pc + 2] as u32;
                    if reg_idx < registers.len() {
                        registers[reg_idx] = peripheral.read_byte(offset)? as u32;
                    }
                    pc += 3;
                }
                0x02 => {
                    if pc + 2 >= self.bytecode.len() { return Err(DeviceError::InvalidParameter); }
                    let offset = self.bytecode[pc + 1] as u32;
                    let reg_idx = self.bytecode[pc + 2] as usize;
                    if reg_idx < registers.len() {
                        peripheral.write_byte(offset, registers[reg_idx] as u8)?;
                    }
                    pc += 3;
                }
                0x03 => {
                    if pc + 2 >= self.bytecode.len() { return Err(DeviceError::InvalidParameter); }
                    let reg_idx = self.bytecode[pc + 1] as usize;
                    let factor = self.bytecode[pc + 2] as u32;
                    if reg_idx < registers.len() {
                        registers[reg_idx] = registers[reg_idx].wrapping_mul(factor);
                    }
                    pc += 3;
                }
                0x04 => {
                    return Ok(());
                }
                _ => {
                    return Err(DeviceError::NotSupported);
                }
            }
        }
        Ok(())
    }
}

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
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), len);
        }

        let mut os_array = [0u8; 16];
        let os_len = os_type.len().min(15);
        unsafe {
            core::ptr::copy_nonoverlapping(os_type.as_ptr(), os_array.as_mut_ptr(), os_len);
        }

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
    fn init(&mut self) -> Result<(), DeviceError> { Ok(()) }
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
    fn shutdown(&mut self) -> Result<(), DeviceError> { Ok(()) }
}

impl UnifiedPeripheral for DdeDeviceWrapper {
    fn query_channel(&self) -> PortAddress {
        PortAddress::MemoryMapped(self.base_addr)
    }

    fn read_byte(&mut self, offset: u32) -> Result<u8, DeviceError> {
        let idx = offset as usize;
        if idx < self.simulated_pci_bar.len() {
            Ok(self.simulated_pci_bar[idx])
        } else {
            Err(DeviceError::InvalidParameter)
        }
    }

    fn write_byte(&mut self, offset: u32, value: u8) -> Result<(), DeviceError> {
        let idx = offset as usize;
        if idx < self.simulated_pci_bar.len() {
            self.simulated_pci_bar[idx] = value;
            Ok(())
        } else {
            Err(DeviceError::InvalidParameter)
        }
    }
}

pub struct DeviceExtension {
    pub irq: u8,
    pub base_port: u16,
    pub device_context: [u8; 256],
}

impl DeviceExtension {
    pub fn new() -> Self {
        Self {
            irq: 0,
            base_port: 0,
            device_context: [0u8; 256],
        }
    }
}

pub struct DeviceObject {
    pub name: [u8; 64],
    pub device_type: DeviceType,
    pub device_extension: DeviceExtension,
}

impl DeviceObject {
    pub fn new(name: &[u8], device_type: DeviceType) -> Self {
        let mut name_arr = [0u8; 64];
        let len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_arr.as_mut_ptr(), len);
        }
        Self {
            name: name_arr,
            device_type,
            device_extension: DeviceExtension::new(),
        }
    }
}

pub struct DriverObject {
    pub driver_name: [u8; 64],
    pub registry_path: [u8; 128],
    pub device_objects: Vec<DeviceObject>,
    pub unload_routine: Option<fn(&mut DriverObject)>,
}

impl DriverObject {
    pub fn new(driver_name: &[u8], registry_path: &[u8]) -> Self {
        let mut name_arr = [0u8; 64];
        let nlen = driver_name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(driver_name.as_ptr(), name_arr.as_mut_ptr(), nlen);
        }

        let mut reg_arr = [0u8; 128];
        let rlen = registry_path.len().min(127);
        unsafe {
            core::ptr::copy_nonoverlapping(registry_path.as_ptr(), reg_arr.as_mut_ptr(), rlen);
        }

        Self {
            driver_name: name_arr,
            registry_path: reg_arr,
            device_objects: Vec::new(),
            unload_routine: None,
        }
    }
}

pub struct IoManager {
    pub active_drivers: Vec<DriverObject>,
}

impl IoManager {
    pub fn new() -> Self {
        Self {
            active_drivers: Vec::new(),
        }
    }

    pub fn normal_driver_installation_process(
        &mut self,
        driver_name: &[u8],
        registry_path: &[u8],
    ) -> Result<usize, DeviceError> {
        let drv = DriverObject::new(driver_name, registry_path);
        let idx = self.active_drivers.len();
        self.active_drivers.push(drv);
        Ok(idx)
    }

    pub fn io_create_device(
        &mut self,
        driver_idx: usize,
        name: &[u8],
        device_type: DeviceType,
    ) -> Result<(), DeviceError> {
        if driver_idx >= self.active_drivers.len() {
            return Err(DeviceError::InvalidParameter);
        }
        let dev_obj = DeviceObject::new(name, device_type);
        self.active_drivers[driver_idx].device_objects.push(dev_obj);
        Ok(())
    }

    pub fn io_unload_driver(&mut self, driver_idx: usize) -> Result<(), DeviceError> {
        if driver_idx >= self.active_drivers.len() {
            return Err(DeviceError::InvalidParameter);
        }
        if let Some(unload_fn) = self.active_drivers[driver_idx].unload_routine {
            unload_fn(&mut self.active_drivers[driver_idx]);
        }
        self.active_drivers[driver_idx].device_objects.clear();
        Ok(())
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortAddress {
    PortIO(u16),
    MemoryMapped(u32),
}

pub trait UnifiedPeripheral {
    fn query_channel(&self) -> PortAddress;
    fn read_byte(&mut self, offset: u32) -> Result<u8, DeviceError>;
    fn write_byte(&mut self, offset: u32, value: u8) -> Result<(), DeviceError>;
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

