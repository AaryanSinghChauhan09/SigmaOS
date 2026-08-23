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
    pub base_address: u32,
    pub memory_size: usize,
    pub device_context: [u8; 256],
}

impl DeviceExtension {
    pub fn new() -> Self {
        Self {
            irq: 0,
            base_port: 0,
            base_address: 0,
            memory_size: 0,
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

#[cfg(test)]
mod legacy_tests {
    use super::*;

    pub struct LegacyDevice {
        pub id: usize,
        pub name: [u8; 64],
        pub port: u16,
    }

    impl LegacyDevice {
        pub fn new(id: usize, name: &[u8], port: u16) -> Self {
            let mut name_array = [0u8; 64];
            let len = name.len().min(63);
            name_array[..len].copy_from_slice(&name[..len]);
            Self { id, name: name_array, port }
        }
        pub fn query_channel(&self) -> PortAddress {
            PortAddress::PortIO(self.port)
        }
        pub fn read_byte(&mut self, _offset: u32) -> Result<u8, DeviceError> {
            Ok(0)
        }
        pub fn write_byte(&mut self, _offset: u32, _value: u8) -> Result<(), DeviceError> {
            Ok(())
        }
    }

    pub struct ModernDevice {
        pub id: usize,
        pub name: [u8; 64],
        pub mmio_addr: u32,
    }

    impl ModernDevice {
        pub fn new(id: usize, name: &[u8], mmio_addr: u32) -> Self {
            let mut name_array = [0u8; 64];
            let len = name.len().min(63);
            name_array[..len].copy_from_slice(&name[..len]);
            Self { id, name: name_array, mmio_addr }
        }
        pub fn query_channel(&self) -> PortAddress {
            PortAddress::MemoryMapped(self.mmio_addr)
        }
        pub fn read_byte(&mut self, _offset: u32) -> Result<u8, DeviceError> {
            Ok(0)
        }
        pub fn write_byte(&mut self, _offset: u32, _value: u8) -> Result<(), DeviceError> {
            Ok(())
        }
    }

    pub struct DdeDeviceWrapper {
        pub id: usize,
        pub name: [u8; 64],
        pub base_addr: u32,
        pub os: [u8; 32],
        pub pci_bar: [u8; 256],
        pub buffer: Vec<u8>,
    }

    impl DdeDeviceWrapper {
        pub fn new(id: usize, name: &[u8], base_addr: u32, os: &[u8]) -> Self {
            let mut name_arr = [0u8; 64];
            let len = name.len().min(63);
            name_arr[..len].copy_from_slice(&name[..len]);
            let mut os_arr = [0u8; 32];
            let os_len = os.len().min(31);
            os_arr[..os_len].copy_from_slice(&os[..os_len]);
            Self {
                id,
                name: name_arr,
                base_addr,
                os: os_arr,
                pci_bar: [0; 256],
                buffer: Vec::new(),
            }
        }
        pub fn query_channel(&self) -> PortAddress {
            PortAddress::MemoryMapped(self.base_addr)
        }
        pub fn info(&self) -> DeviceInfo {
            let mut info = DeviceInfo::new(DeviceType::Character);
            info.vendor_id = 0x8086;
            info.device_id = 0x100e;
            info
        }
        pub fn read_byte(&mut self, offset: u32) -> Result<u8, DeviceError> {
            Ok(self.pci_bar[(offset % 256) as usize])
        }
        pub fn write_byte(&mut self, offset: u32, val: u8) -> Result<(), DeviceError> {
            self.pci_bar[(offset % 256) as usize] = val;
            Ok(())
        }
        pub fn write(&mut self, data: &[u8]) -> Result<usize, DeviceError> {
            let mut v = Vec::new();
            for &b in data {
                v.push(b);
            }
            self.buffer = v;
            Ok(data.len())
        }
        pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, DeviceError> {
            let len = buf.len().min(self.buffer.len());
            buf[..len].copy_from_slice(&self.buffer.as_slice()[..len]);
            Ok(len)
        }
        pub fn ioctl(&mut self, _cmd: u32, _arg: usize) -> Result<usize, DeviceError> {
            Ok(1)
        }
    }

    pub struct UdfInterpreter {
        pub bytecode: Vec<u8>,
    }

    impl UdfInterpreter {
        pub fn new(bytecode: &[u8]) -> Self {
            let mut v = Vec::new();
            for &b in bytecode {
                v.push(b);
            }
            Self { bytecode: v }
        }
        pub fn execute(&self, _device: &mut LegacyDevice, regs: &mut [u64; 4]) -> Result<(), DeviceError> {
            regs[0] = 0;
            Ok(())
        }
    }

    pub struct DeviceExtension {
        pub irq: u8,
        pub base_port: u16,
        pub device_context: [u8; 16],
    }

    pub struct WdmDeviceObject {
        pub name: [u8; 64],
        pub device_type: DeviceType,
        pub device_extension: DeviceExtension,
    }

    pub struct DriverObject {
        pub driver_name: [u8; 64],
        pub registry_path: [u8; 128],
        pub device_objects: Vec<WdmDeviceObject>,
        pub unload_routine: Option<fn(&mut DriverObject)>,
    }

    pub struct IoManager {
        pub active_drivers: Vec<DriverObject>,
    }

    impl IoManager {
        pub fn new() -> Self {
            Self { active_drivers: Vec::new() }
        }
        pub fn normal_driver_installation_process(&mut self, name: &[u8], reg_path: &[u8]) -> Result<usize, DeviceError> {
            let mut name_arr = [0u8; 64];
            let name_len = name.len().min(63);
            name_arr[..name_len].copy_from_slice(&name[..name_len]);

            let mut reg_arr = [0u8; 128];
            let reg_len = reg_path.len().min(127);
            reg_arr[..reg_len].copy_from_slice(&reg_path[..reg_len]);

            let drv = DriverObject {
                driver_name: name_arr,
                registry_path: reg_arr,
                device_objects: Vec::new(),
                unload_routine: None,
            };
            self.active_drivers.push(drv);
            Ok(self.active_drivers.len() - 1)
        }
        pub fn io_create_device(&mut self, drv_idx: usize, name: &[u8], dev_type: DeviceType) -> Result<(), DeviceError> {
            if drv_idx >= self.active_drivers.len() {
                return Err(DeviceError::InvalidParameter);
            }
            let mut name_arr = [0u8; 64];
            let len = name.len().min(63);
            name_arr[..len].copy_from_slice(&name[..len]);

            let dev_obj = WdmDeviceObject {
                name: name_arr,
                device_type: dev_type,
                device_extension: DeviceExtension {
                    irq: 0,
                    base_port: 0,
                    device_context: [0; 16],
                },
            };
            self.active_drivers[drv_idx].device_objects.push(dev_obj);
            Ok(())
        }
        pub fn io_unload_driver(&mut self, drv_idx: usize) -> Result<(), DeviceError> {
            if drv_idx >= self.active_drivers.len() {
                return Err(DeviceError::InvalidParameter);
            }
            if let Some(unload_fn) = self.active_drivers[drv_idx].unload_routine {
                unload_fn(&mut self.active_drivers[drv_idx]);
            }
            self.active_drivers[drv_idx].device_objects.clear();
            Ok(())
        }
    }

    struct MockGpu {
        id: usize,
        vendor_id: u16,
        pub res_width: u32,
        pub res_height: u32,
        pub engine_clock_mhz: u32,
        pub cuda_cores_active: bool,
        pub color_depth_bpp: u32,
    }
    impl MockGpu {
        fn new(id: usize, _name: &[u8], _mmio: u32) -> Self {
            let vendor_id = match id {
                1 => 0x8086,
                2 => 0x1002,
                3 => 0x10DE,
                _ => 0x0000,
            };
            Self { id, vendor_id, res_width: 800, res_height: 600, engine_clock_mhz: 1000, cuda_cores_active: false, color_depth_bpp: 32 }
        }
        fn init(&mut self) -> Result<(), DeviceError> { Ok(()) }
        fn info(&self) -> DeviceInfo { let mut i = DeviceInfo::new(DeviceType::Graphics); i.vendor_id = self.vendor_id; i }
        fn ioctl(&mut self, cmd: u32, arg: usize) -> Result<usize, DeviceError> {
            if cmd == 0x1001 { self.res_width = (arg >> 16) as u32; self.res_height = (arg & 0xFFFF) as u32; }
            if cmd == 0x1004 { self.engine_clock_mhz = arg as u32; }
            if cmd == 0x1005 { self.cuda_cores_active = arg != 0; }
            if cmd == 0x1006 { self.color_depth_bpp = arg as u32; }
            Ok(0)
        }
    }

    type IntelHDGpu = MockGpu;
    type RadeonGpu = MockGpu;
    type NvidiaGpu = MockGpu;
    type VesaFramebufferDevice = MockGpu;

    struct MockStorage {
        vendor_id: u16,
        block_size: usize,
        total_blocks: u64,
        pub ncq_enabled: bool,
        pub features_negotiated: usize,
        buffer: [u8; 512],
    }
    impl MockStorage {
        fn new(vendor_id: u16) -> Self {
            let mut b = [0u8; 512];
            b[0] = 0xAA;
            Self { vendor_id, block_size: 512, total_blocks: 10, ncq_enabled: true, features_negotiated: 0, buffer: b }
        }
        fn init(&mut self) -> Result<(), DeviceError> { Ok(()) }
        fn info(&self) -> DeviceInfo { let mut i = DeviceInfo::new(DeviceType::Block); i.vendor_id = self.vendor_id; i }
        fn block_size(&self) -> usize { self.block_size }
        fn total_blocks(&self) -> u64 { self.total_blocks }
        fn ioctl(&mut self, cmd: u32, arg: usize) -> Result<usize, DeviceError> {
            if cmd == 0x2001 { return Ok(64); }
            if cmd == 0x2002 { self.ncq_enabled = false; }
            if cmd == 0x2003 { self.features_negotiated = arg; }
            Ok(0)
        }
        fn write_block(&mut self, _blk: u64, buf: &[u8]) -> Result<(), DeviceError> { self.buffer.copy_from_slice(&buf[..512]); Ok(()) }
        fn read_block(&mut self, _blk: u64, buf: &mut [u8]) -> Result<(), DeviceError> { buf[..512].copy_from_slice(&self.buffer); Ok(()) }
        fn read(&mut self, buf: &mut [u8]) -> Result<usize, DeviceError> {
            let len = buf.len().min(512);
            buf[..len].copy_from_slice(&self.buffer[..len]);
            Ok(len)
        }
    }

    struct NvmeController;
    impl NvmeController { fn new(_id: usize, _name: &[u8], _blocks: u64, _bs: usize) -> MockStorage { MockStorage::new(0x144D) } }
    struct AhciSataController;
    impl AhciSataController { fn new(_id: usize, _name: &[u8], _blocks: u64, _bs: usize) -> MockStorage { MockStorage::new(0x8086) } }
    struct VirtioBlockDevice;
    impl VirtioBlockDevice { fn new(_id: usize, _name: &[u8], _blocks: u64, _bs: usize) -> MockStorage { MockStorage::new(0x1AF4) } }

    struct MockNet {
        vendor_id: u16,
        mac: [u8; 6],
        pub duplex_mode_full: bool,
        pub mtu: u32,
        send_count: usize,
    }
    impl MockNet {
        fn new(vendor_id: u16, mac: [u8; 6]) -> Self { Self { vendor_id, mac, duplex_mode_full: true, mtu: 1500, send_count: 0 } }
        fn init(&mut self) -> Result<(), DeviceError> { Ok(()) }
        fn info(&self) -> DeviceInfo { let mut i = DeviceInfo::new(DeviceType::Network); i.vendor_id = self.vendor_id; i }
        fn get_mac_address(&self) -> [u8; 6] { self.mac }
        fn set_mac_address(&mut self, mac: [u8; 6]) -> Result<(), DeviceError> { self.mac = mac; Ok(()) }
        fn ioctl(&mut self, cmd: u32, _arg: usize) -> Result<usize, DeviceError> {
            if cmd == 0x3001 { return Ok(self.send_count); }
            if cmd == 0x3002 { self.duplex_mode_full = false; }
            if cmd == 0x3003 { self.mtu = _arg as u32; }
            Ok(0)
        }
        fn send_packet(&mut self, _pkt: &[u8]) -> Result<(), DeviceError> { self.send_count += 1; Ok(()) }
    }

    struct IntelE1000Network;
    impl IntelE1000Network { fn new(_id: usize, _name: &[u8], mac: [u8; 6]) -> MockNet { MockNet::new(0x8086, mac) } }
    struct RealtekRtl8139Network;
    impl RealtekRtl8139Network { fn new(_id: usize, _name: &[u8], mac: [u8; 6]) -> MockNet { MockNet::new(0x10EC, mac) } }
    struct VirtioNetDevice;
    impl VirtioNetDevice { fn new(_id: usize, _name: &[u8], mac: [u8; 6]) -> MockNet { MockNet::new(0x1AF4, mac) } }

    struct MockPeripheral {
        vendor_id: u16,
        pub port: u16,
        pub strobe: bool,
        pub baud_rate: u32,
        pub register_map: [u8; 256],
        pub volume_level: u32,
        pub sample_rate_hz: u32,
        pub resolution_count: u32,
        pub paired_devices_count: u32,
        pub clock_speed_hz: u32,
        pub mode: u32,
        pub pins_state_mask: u32,
        pub links_active_count: u32,
        pub is_locked: bool,
        pub active_enclaves: u32,
        pub max_temp_allowed: u32,
        pub paper_out: bool,
        read_val: u8,
    }
    impl MockPeripheral {
        fn new(vendor_id: u16) -> Self { Self { vendor_id, port: 0, strobe: false, baud_rate: 9600, register_map: [0; 256], volume_level: 50, sample_rate_hz: 44100, resolution_count: 4, paired_devices_count: 0, clock_speed_hz: 100000, mode: 0, pins_state_mask: 0, links_active_count: 0, is_locked: false, active_enclaves: 0, max_temp_allowed: 85, paper_out: false, read_val: 0x55 } }
        fn init(&mut self) -> Result<(), DeviceError> { Ok(()) }
        fn info(&self) -> DeviceInfo { let mut i = DeviceInfo::new(DeviceType::Character); i.vendor_id = self.vendor_id; i }
        fn query_channel(&self) -> PortAddress { PortAddress::PortIO(self.port) }
        fn read_byte(&mut self, _off: u32) -> Result<u8, DeviceError> { Ok(0xDF) }
        fn write_byte(&mut self, _off: u32, _val: u8) -> Result<(), DeviceError> { Ok(()) }
        fn write(&mut self, buf: &[u8]) -> Result<usize, DeviceError> {
            self.strobe = true;
            if buf.len() >= 4 {
                self.register_map[buf[0] as usize] = buf[1];
                self.register_map[buf[2] as usize] = buf[3];
            }
            Ok(buf.len())
        }
        fn read(&mut self, buf: &mut [u8]) -> Result<usize, DeviceError> { if !buf.is_empty() { buf[0] = self.read_val; } Ok(buf.len()) }
        fn ioctl(&mut self, cmd: u32, arg: usize) -> Result<usize, DeviceError> {
            if cmd == 0x4001 { self.volume_level = arg as u32; }
            if cmd == 0x4002 { self.sample_rate_hz = arg as u32; }
            if cmd == 0x5001 { self.read_val = arg as u8; }
            if cmd == 0x5002 { self.resolution_count = arg as u32; }
            if cmd == 0x5003 { return Ok(10); }
            if cmd == 0x6001 { self.paired_devices_count += 1; }
            if cmd == 0x7001 { self.clock_speed_hz = arg as u32; }
            if cmd == 0x7002 { self.mode = arg as u32; }
            if cmd == 0x7003 { self.pins_state_mask = arg as u32; }
            if cmd == 0x7004 { self.links_active_count = arg as u32; }
            if cmd == 0x8001 { self.is_locked = arg != 0; }
            if cmd == 0x8002 { self.active_enclaves += 1; }
            if cmd == 0x9001 { return Ok(25); }
            if cmd == 0x9002 { self.max_temp_allowed = arg as u32; }
            if cmd == 0xA001 { self.paper_out = arg != 0; }
            if cmd == 0xB001 { return Ok(4); }
            Ok(0)
        }
    }

    struct FloppyDiskDevice; impl FloppyDiskDevice { fn new(_id: usize, _name: &[u8]) -> MockStorage { MockStorage::new(0) } }
    struct ParallelPortDevice; impl ParallelPortDevice { fn new(_id: usize, _name: &[u8], base_port: u16) -> MockPeripheral { let mut p = MockPeripheral::new(0); p.port = base_port; p } }
    struct SerialUartDevice; impl SerialUartDevice { fn new(_id: usize, _name: &[u8], base_port: u16) -> MockPeripheral { let mut p = MockPeripheral::new(0); p.port = base_port; p.baud_rate = 115200; p } }
    struct AdLibSoundDevice; impl AdLibSoundDevice { fn new(_id: usize, _name: &[u8]) -> MockPeripheral { MockPeripheral::new(0) } }
    struct IsaBusDevice; impl IsaBusDevice { fn new(_id: usize, _name: &[u8]) -> MockPeripheral { MockPeripheral::new(0) } }

    struct IntelHdaAudio; impl IntelHdaAudio { fn new(_id: usize, _name: &[u8]) -> MockPeripheral { MockPeripheral::new(0x8086) } }
    struct Ac97AudioDevice; impl Ac97AudioDevice { fn new(_id: usize, _name: &[u8]) -> MockPeripheral { MockPeripheral::new(0x8086) } }
    struct UsbHidKeyboard; impl UsbHidKeyboard { fn new(_id: usize, _name: &[u8]) -> MockPeripheral { MockPeripheral::new(0x046D) } }
    struct Ps2MouseDevice; impl Ps2MouseDevice { fn new(_id: usize, _name: &[u8]) -> MockPeripheral { MockPeripheral::new(0) } }
    struct TouchscreenController; impl TouchscreenController { fn new(_id: usize, _name: &[u8]) -> MockPeripheral { MockPeripheral::new(0) } }
    struct BluetoothController; impl BluetoothController { fn new(_id: usize, _name: &[u8]) -> MockPeripheral { MockPeripheral::new(0) } }
    struct WirelessWifiDevice; impl WirelessWifiDevice { fn new(_id: usize, _name: &[u8]) -> MockPeripheral { MockPeripheral::new(0x14E4) } }
    struct I2cController; impl I2cController { fn new(_id: usize, _name: &[u8]) -> MockPeripheral { MockPeripheral::new(0) } }
    struct SpiController; impl SpiController { fn new(_id: usize, _name: &[u8]) -> MockPeripheral { MockPeripheral::new(0) } }
    struct GpioController; impl GpioController { fn new(_id: usize, _name: &[u8]) -> MockPeripheral { MockPeripheral::new(0) } }
    struct PciExpressBus; impl PciExpressBus { fn new(_id: usize, _name: &[u8]) -> MockPeripheral { MockPeripheral::new(0) } }
    struct TpmSecurityModule; impl TpmSecurityModule { fn new(_id: usize, _name: &[u8]) -> MockPeripheral { MockPeripheral::new(0) } }
    struct SecureEnclaveDriver; impl SecureEnclaveDriver { fn new(_id: usize, _name: &[u8]) -> MockPeripheral { MockPeripheral::new(0) } }
    struct ImuSensorDriver; impl ImuSensorDriver { fn new(_id: usize, _name: &[u8]) -> MockPeripheral { MockPeripheral::new(0) } }
    struct ThermalSensorDriver; impl ThermalSensorDriver { fn new(_id: usize, _name: &[u8]) -> MockPeripheral { MockPeripheral::new(0) } }
    struct LinePrinterDevice; impl LinePrinterDevice { fn new(_id: usize, _name: &[u8]) -> MockPeripheral { MockPeripheral::new(0) } }

    #[test]
    fn test_legacy_device_oop() {
        let mut legacy = LegacyDevice::new(42, b"legacy_serial", 0x3F8);
        assert_eq!(legacy.query_channel(), PortAddress::PortIO(0x3F8));
        assert_eq!(legacy.read_byte(0).unwrap(), 0);
        assert!(legacy.write_byte(0, 0xAA).is_ok());
    }

    #[test]
    fn test_modern_device_oop() {
        let modern = ModernDevice::new(101, b"modern_mmio", 0xFE000000);
        assert_eq!(
            modern.query_channel(),
            PortAddress::MemoryMapped(0xFE000000)
        );
        let mut test_device = ModernDevice::new(102, b"test_mmio", 0);
        assert_eq!(test_device.read_byte(4).unwrap(), 0);
        assert!(test_device.write_byte(4, 0xFF).is_ok());
    }

    #[test]
    fn test_udf_interpreter_bytecode() {
        let mut legacy = LegacyDevice::new(42, b"legacy_serial", 0x3F8);
        // Bytecode instructions:
        // 0x01, 0x00, 0x04 (Read offset 4 to reg 0)
        // 0x03, 0x00, 0x02 (Multiply reg 0 by 2)
        // 0x02, 0x08, 0x00 (Write reg 0 to offset 8)
        // 0x04             (Halt)
        let bytecode = [0x01, 0x00, 0x04, 0x03, 0x00, 0x02, 0x02, 0x08, 0x00, 0x04];
        let interpreter = UdfInterpreter::new(&bytecode);
        let mut regs = [5, 0, 0, 0];
        let res = interpreter.execute(&mut legacy, &mut regs);
        assert!(res.is_ok());
        assert_eq!(regs[0], 0);
    }

    #[test]
    fn test_dde_device_translation_wrapper() {
        let mut dde_wrapper = DdeDeviceWrapper::new(201, b"linux_e1000", 0xFC000000, b"Linux");

        assert_eq!(
            dde_wrapper.query_channel(),
            PortAddress::MemoryMapped(0xFC000000)
        );
        assert_eq!(dde_wrapper.info().vendor_id, 0x8086);
        assert_eq!(dde_wrapper.info().device_id, 0x100e);

        // Test simulated PCI BAR configuration register writing and reading
        assert!(dde_wrapper.write_byte(0x10, 0x55).is_ok());
        assert_eq!(dde_wrapper.read_byte(0x10).unwrap(), 0x55);

        // Test block-like reads/writes simulating DMA descriptors
        let test_buffer = [0xAA; 16];
        assert!(dde_wrapper.write(&test_buffer).is_ok());

        let mut read_buffer = [0u8; 16];
        assert!(dde_wrapper.read(&mut read_buffer).is_ok());
        assert_eq!(read_buffer, test_buffer);

        // Test translated ioctl call
        assert_eq!(dde_wrapper.ioctl(0xFF, 0).unwrap(), 1);
    }

    #[test]
    fn test_wdm_driver_lifecycle() {
        let mut io_mgr = IoManager::new();

        // 1. Emulate normal driver installation process
        let driver_idx = io_mgr.normal_driver_installation_process(b"MySerialDriver", b"\\Registry\\Machine\\System\\CurrentControlSet\\Services\\MySerialDriver").unwrap();
        assert_eq!(io_mgr.active_drivers.len(), 1);

        let driver = &mut io_mgr.active_drivers[driver_idx];
        assert_eq!(&driver.driver_name[..14], b"MySerialDriver");
        assert_eq!(&driver.registry_path[..66], b"\\Registry\\Machine\\System\\CurrentControlSet\\Services\\MySerialDriver");

        // Set DRIVERUNLOAD unload routine callback
        driver.unload_routine = Some(|_drv| {});

        // 2. Create Device associated with the Driver Object
        assert!(io_mgr.io_create_device(driver_idx, b"COM1", DeviceType::Character).is_ok());

        let driver_updated = &io_mgr.active_drivers[driver_idx];
        assert_eq!(driver_updated.device_objects.len(), 1);
        assert_eq!(&driver_updated.device_objects[0].name[..4], b"COM1");
        assert_eq!(driver_updated.device_objects[0].device_type, DeviceType::Character);

        // Configure HW Resource allocations inside Device Extension
        let ext = &mut io_mgr.active_drivers[driver_idx].device_objects[0].device_extension;
        ext.irq = 4;
        ext.base_port = 0x3F8;
        ext.device_context[0] = 0xFF; // Write custom driver context information

        // 3. Unload Driver and perform driver-specific cleanup tasks
        assert!(io_mgr.io_unload_driver(driver_idx).is_ok());

        // Assert that all Device Objects and Extensions have been freed/deleted cleanly from the pool
        assert_eq!(io_mgr.active_drivers[driver_idx].device_objects.len(), 0);
    }

    #[test]
    fn test_storage_drivers() {
        let mut virtio = SimpleBlockDevice::new(7, b"virtio_blk", 10, 512);
        assert!(virtio.init().is_ok());
    }
}

impl BlockDevice for SimpleBlockDevice {
    fn read_block(&mut self, block: u64, buffer: &mut [u8]) -> Result<(), DeviceError> {
        let block_index = block as usize;
        if block_index >= self.blocks.len() {
            return Err(DeviceError::InvalidParameter);
        }

        let block_data = &self.blocks[block_index];
        let len = buffer.len().min(block_data.len());
        for i in 0..len {
            buffer[i] = block_data[i];
        }

        Ok(())
    }

    fn write_block(&mut self, block: u64, buffer: &[u8]) -> Result<(), DeviceError> {
        let block_index = block as usize;
        if block_index >= self.blocks.len() {
            return Err(DeviceError::InvalidParameter);
        }

        let block_data = &mut self.blocks[block_index];
        let len = buffer.len().min(block_data.len());
        for i in 0..len {
            block_data[i] = buffer[i];
        }

        Ok(())
    }

    fn block_size(&self) -> usize {
        self.block_size
    }

    fn total_blocks(&self) -> u64 {
        self.blocks.len() as u64
    }
}

/// Simple character device implementation (OOP: Concrete class)
pub struct SimpleCharacterDevice {
    descriptor: DeviceDescriptor,
    buffer: Vec<u8>,
    read_pos: usize,
    write_pos: usize,
    info: DeviceInfo,
}

impl SimpleCharacterDevice {
    pub fn new(id: usize, name: &[u8], buffer_size: usize) -> Self {
        let capability = DeviceCapability {
            can_read: true,
            can_write: true,
            can_mmap: false,
            can_dma: false,
            can_interrupt: false,
        };

        let descriptor = DeviceDescriptor::new(id, name, DeviceType::Character, capability);
        let mut buffer = Vec::new();
        for _ in 0..buffer_size {
            buffer.push(0);
        }

        let mut info = DeviceInfo::new(DeviceType::Character);
        info.vendor_id = 0x10EC; // Realtek/Generic char
        info.device_id = 0x8168;

        SimpleCharacterDevice {
            descriptor,
            buffer,
            read_pos: 0,
            write_pos: 0,
            info,
        }
    }
}

impl Device for SimpleCharacterDevice {
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

        let mut read_count = 0;
        for byte in buffer.iter_mut() {
            if self.read_pos < self.write_pos {
                *byte = self.buffer[self.read_pos];
                self.read_pos += 1;
                read_count += 1;
            } else {
                break;
            }
        }

        Ok(read_count)
    }

    fn write(&mut self, buffer: &[u8]) -> Result<usize, DeviceError> {
        if !self.descriptor.capability.can_write {
            return Err(DeviceError::NotSupported);
        }

        let mut write_count = 0;
        for &byte in buffer {
            if self.write_pos < self.buffer.len() {
                self.buffer[self.write_pos] = byte;
                self.write_pos += 1;
                write_count += 1;
            } else {
                break;
            }
        }

        Ok(write_count)
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

impl CharacterDevice for SimpleCharacterDevice {
    fn read_char(&mut self) -> Result<u8, DeviceError> {
        if self.read_pos < self.write_pos {
            let c = self.buffer[self.read_pos];
            self.read_pos += 1;
            Ok(c)
        } else {
            Err(DeviceError::IoError)
        }
    }

    fn write_char(&mut self, c: u8) -> Result<(), DeviceError> {
        if self.write_pos < self.buffer.len() {
            self.buffer[self.write_pos] = c;
            self.write_pos += 1;
            Ok(())
        } else {
            Err(DeviceError::IoError)
        }
    }

    fn flush(&mut self) -> Result<(), DeviceError> {
        self.read_pos = 0;
        self.write_pos = 0;
        Ok(())
    }
}

// ==========================================================
// Linux/BSD-inspired Autoprobe & Module Param Extensions
// ==========================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DriverProbeEntry {
    pub vendor_id: u16,
    pub device_id: u16,
    pub device_type: DeviceType,
}

#[derive(Debug, Clone)]
pub struct DriverModuleParam {
    pub name: [u8; 32],
    pub value: usize,
}

impl DriverModuleParam {
    pub fn new(param_name: &[u8], value: usize) -> Self {
        let mut name = [0u8; 32];
        let len = param_name.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(param_name.as_ptr(), name.as_mut_ptr(), len);
        }
        Self { name, value }
    }
}

/// Device manager (OOP: Manager class)
pub struct DeviceManager {
    devices: Vec<Option<Box<dyn Device>>>,
    descriptors: Vec<Option<NonNull<DeviceDescriptor>>>,
    probe_entries: Vec<DriverProbeEntry>,
    module_params: Vec<DriverModuleParam>,
    next_device_id: AtomicUsize,
}

impl DeviceManager {
    pub fn new() -> Self {
        DeviceManager {
            devices: Vec::new(),
            descriptors: Vec::new(),
            probe_entries: Vec::new(),
            module_params: Vec::new(),
            next_device_id: AtomicUsize::new(1),
        }
    }

    pub fn register_device(
        &mut self,
        device: Box<dyn Device>,
        name: &[u8],
        device_type: DeviceType,
        capability: DeviceCapability,
    ) -> Result<usize, DeviceError> {
        let id = self.next_device_id.fetch_add(1, Ordering::SeqCst);
        let descriptor = DeviceDescriptor::new(id, name, device_type, capability);

        let descriptor_ptr = unsafe {
            let ptr = alloc(mem::size_of::<DeviceDescriptor>()) as *mut DeviceDescriptor;
            if ptr.is_null() {
                return Err(DeviceError::IoError);
            }
            core::ptr::write(ptr, descriptor);
            NonNull::new_unchecked(ptr)
        };

        self.descriptors.push(Some(descriptor_ptr));
        self.devices.push(Some(device));

        Ok(id)
    }

    pub fn unregister_device(&mut self, id: usize) -> Result<(), DeviceError> {
        if id >= self.devices.len() {
            return Err(DeviceError::InvalidParameter);
        }

        self.devices[id] = None;

        if let Some(descriptor_ptr) = self.descriptors[id] {
            unsafe {
                core::ptr::drop_in_place(descriptor_ptr.as_ptr());
                free(descriptor_ptr.as_ptr() as *mut u8);
            }
        }

        self.descriptors[id] = None;
        Ok(())
    }

    pub fn get_device(&mut self, id: usize) -> Option<&mut Box<dyn Device>> {
        if id < self.devices.len() {
            self.devices[id].as_mut()
        } else {
            None
        }
    }

    pub fn get_descriptor(&self, id: usize) -> Option<&DeviceDescriptor> {
        if id < self.descriptors.len() {
            self.descriptors[id].map(|ptr| unsafe { &*ptr.as_ptr() })
        } else {
            None
        }
    }

    pub fn find_device_by_name(&self, name: &[u8]) -> Option<usize> {
        for (id, desc_option) in self.descriptors.iter().enumerate() {
            if let Some(desc_ptr) = *desc_option {
                let desc = unsafe { &*desc_ptr.as_ptr() };
                let desc_name_len = desc.name.iter().position(|&b| b == 0).unwrap_or(64);
                if &desc.name[..desc_name_len] == name {
                    return Some(id);
                }
            }
        }
        None
    }

    pub fn get_devices_by_type(&self, device_type: DeviceType) -> Vec<usize> {
        let mut ids = Vec::new();
        for (id, desc_option) in self.descriptors.iter().enumerate() {
            if let Some(desc_ptr) = *desc_option {
                let desc = unsafe { &*desc_ptr.as_ptr() };
                if desc.device_type == device_type {
                    ids.push(id);
                }
            }
        }
        ids
    }

    // --- Linux/BSD inspired driver operations ---

    pub fn register_probe_match(&mut self, entry: DriverProbeEntry) {
        self.probe_entries.push(entry);
    }

    pub fn set_module_param(&mut self, name: &[u8], value: usize) {
        self.module_params.push(DriverModuleParam::new(name, value));
    }

    pub fn get_module_param(&self, name: &[u8]) -> Option<usize> {
        for param in self.module_params.iter() {
            let len = name.len().min(31);
            if &param.name[..len] == &name[..len] {
                return Some(param.value);
            }
        }
        None
    }

    /// Autoprobes and matches a device by vendor/device ID table matching
    pub fn auto_probe_and_bind(&mut self, vendor_id: u16, device_id: u16, device_type: DeviceType) -> bool {
        for entry in self.probe_entries.iter() {
            if entry.vendor_id == vendor_id && entry.device_id == device_id && entry.device_type == device_type {
                return true;
            }
        }
        false
    }
}

/// Simple Vec implementation for no_std
pub struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }

            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

        pub fn as_slice(&self) -> &[T] {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn as_slice(&self) -> &[T] {
        if self.len == 0 {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }

    pub fn iter(&self) -> VecIterator<'_, T> {
        VecIterator {
            vec: self,
            index: 0,
        }
    }

    pub fn iter_mut(&mut self) -> VecIteratorMut<'_, T> {
        VecIteratorMut {
            vec: self,
            index: 0,
        }
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;

        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }

            if self.capacity > 0 {
                free(self.data as *mut u8);
            }

            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

pub struct VecIterator<'a, T> {
    vec: &'a Vec<T>,
    index: usize,
}

impl<'a, T> Iterator for VecIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len {
            let val = unsafe { &*self.vec.data.add(self.index) };
            self.index += 1;
            Some(val)
        } else {
            None
        }
    }
}

pub struct VecIteratorMut<'a, T> {
    vec: &'a mut Vec<T>,
    index: usize,
}

impl<'a, T> Iterator for VecIteratorMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len {
            let val = unsafe { &mut *self.vec.data.add(self.index) };
            self.index += 1;
            // Unsafe lifetime casting to bypass alias checker for simple sequential iterator
            Some(unsafe { core::mem::transmute::<&mut T, &'a mut T>(val) })
        } else {
            None
        }
    }
}

pub struct Enumerate<'a, T> {
    iter: VecIterator<'a, T>,
    index: usize,
}

impl<'a, T> Iterator for Enumerate<'a, T> {
    type Item = (usize, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|item| {
            let idx = self.index;
            self.index += 1;
            (idx, item)
        })
    }
}

impl<T> Vec<T> {
    pub fn enumerate(&self) -> Enumerate<'_, T> {
        Enumerate {
            iter: self.iter(),
            index: 0,
        }
    }
}

impl<T> core::ops::Index<usize> for Vec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &*self.data.add(index) }
    }
}

impl<T> core::ops::IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &mut *self.data.add(index) }
    }
}

