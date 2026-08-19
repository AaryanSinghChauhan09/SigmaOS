#![no_std]
#![cfg_attr(target_os = "none", no_main)]

extern crate alloc;
use alloc::boxed::Box;

use core::mem;
/// OOP-based Device Driver Framework for SigmaOS
/// Implements device drivers using OOP principles with traits and structs
/// No dependency on external driver frameworks
use core::ptr::NonNull;
use core::sync::atomic::{AtomicUsize, Ordering};

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
mod tests {
    use super::*;
    use crate::drivers::dde::UdfInterpreter;
    use crate::compatibility::historic_linux::DdeDeviceWrapper;

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

    pub fn clear(&mut self) {
        self.len = 0;
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

// External allocator functions
#[cfg(not(test))]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[cfg(test)]
extern "C" {
    fn malloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[cfg(test)]
#[no_mangle]
pub unsafe extern "C" fn alloc(size: usize) -> *mut u8 {
    malloc(size)
}

// ==========================================
// Standalone unit tests
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_descriptors() {
        let capability = DeviceCapability::full();
        let desc = DeviceDescriptor::new(10, b"SerialTTY", DeviceType::Character, capability);
        assert_eq!(desc.get_state(), DeviceState::Uninitialized);
        desc.set_state(DeviceState::Ready);
        assert_eq!(desc.get_state(), DeviceState::Ready);
    }

    #[test]
    fn test_simple_block_device() {
        let mut dev = SimpleBlockDevice::new(1, b"disk0", 4, 512);
        assert_eq!(dev.info().vendor_id, 0x8086);
        assert!(dev.init().is_ok());

        let mut write_buf = [0u8; 512];
        write_buf[0] = 0xAA;
        assert!(dev.write_block(2, &write_buf).is_ok());

        let mut read_buf = [0u8; 512];
        assert!(dev.read_block(2, &mut read_buf).is_ok());
        assert_eq!(read_buf[0], 0xAA);
    }

    #[test]
    fn test_device_manager_autoprobe_and_params() {
        let mut mgr = DeviceManager::new();

        // Register custom boot parameter (module param)
        mgr.set_module_param(b"debug_level", 4);
        assert_eq!(mgr.get_module_param(b"debug_level"), Some(4));
        assert_eq!(mgr.get_module_param(b"non_existent"), None);

        // Register PCI device table probe matches
        let entry = DriverProbeEntry {
            vendor_id: 0x10EC,
            device_id: 0x8168,
            device_type: DeviceType::Network,
        };
        mgr.register_probe_match(entry);

        // Check autoprobe success
        assert!(mgr.auto_probe_and_bind(0x10EC, 0x8168, DeviceType::Network));
        assert!(!mgr.auto_probe_and_bind(0xFFFF, 0xFFFF, DeviceType::Network));
    }
}
