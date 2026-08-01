#![no_std]
#![no_main]

use core::mem;
/// OOP-based Device Driver Framework for SigmaOS
/// Implements device drivers using OOP principles with traits and structs
/// No dependency on external driver frameworks
use core::ptr::{self, NonNull};
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
#[derive(Debug, Clone, Copy)]
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

impl Default for DeviceCapability {
    fn default() -> Self {
        Self::new()
    }
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
        unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst) as u32) }
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
    blocks: LocalVec<LocalVec<u8>>,
    block_size: usize,
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
        let mut blocks = LocalVec::new();

        for _ in 0..num_blocks {
            let mut block_data = LocalVec::new();
            for _ in 0..block_size {
                block_data.push(0u8);
            }
            blocks.push(block_data);
        }

        SimpleBlockDevice {
            descriptor,
            blocks,
            block_size,
        }
    }
}

impl Device for SimpleBlockDevice {
    fn init(&mut self) -> Result<(), DeviceError> {
        if self.descriptor.get_state() == DeviceState::Ready {
            return Err(DeviceError::AlreadyInitialized);
        }

        self.descriptor.set_state(DeviceState::Initializing);
        // Simulate initialization
        self.descriptor.set_state(DeviceState::Ready);
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, DeviceError> {
        if !self.descriptor.capability.can_read {
            return Err(DeviceError::NotSupported);
        }

        // In a real implementation, this would read from the device
        // For now, return success
        Ok(buffer.len())
    }

    fn write(&mut self, buffer: &[u8]) -> Result<usize, DeviceError> {
        if !self.descriptor.capability.can_write {
            return Err(DeviceError::NotSupported);
        }

        // In a real implementation, this would write to the device
        // For now, return success
        Ok(buffer.len())
    }

    fn ioctl(&mut self, _command: u32, _arg: usize) -> Result<usize, DeviceError> {
        Ok(0)
    }

    fn info(&self) -> DeviceInfo {
        DeviceInfo::new(DeviceType::Block)
    }

    fn shutdown(&mut self) -> Result<(), DeviceError> {
        self.descriptor.set_state(DeviceState::Shutdown);
        Ok(())
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
        buffer[..len].copy_from_slice(&block_data[..len]);

        Ok(())
    }

    fn write_block(&mut self, block: u64, buffer: &[u8]) -> Result<(), DeviceError> {
        let block_index = block as usize;
        if block_index >= self.blocks.len() {
            return Err(DeviceError::InvalidParameter);
        }

        let block_data = &mut self.blocks[block_index];
        let len = buffer.len().min(block_data.len());
        block_data[..len].copy_from_slice(&buffer[..len]);

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
    buffer: LocalVec<u8>,
    read_pos: usize,
    write_pos: usize,
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

        let mut buffer = LocalVec::new();
        for _ in 0..buffer_size {
            buffer.push(0u8);
        }

        SimpleCharacterDevice {
            descriptor,
            buffer,
            read_pos: 0,
            write_pos: 0,
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
        DeviceInfo::new(DeviceType::Character)
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

/// Linux-history inspired Early Boot Parameter Override Entry
/// Maps a device identifier or legacy serial/io port to custom base IO, IRQs, or UDF bytecode overrides.
/// This allows SigmaOS to work with older unsupported devices (ISA cards, custom PC clones, legacy serial, etc.)
/// without needing a massive compiled-in driver binary, satisfying OOP size-reduction goals.
pub struct EarlyBootParameterOverride {
    pub device_name: [u8; 32],
    pub port_io_override: u16,
    pub irq_override: u8,
    pub udf_bytecode: [u8; 16], // Light bytecode override for custom scaling/reg mapping
    pub udf_len: usize,
}

impl EarlyBootParameterOverride {
    pub fn new(device_name: &[u8], port: u16, irq: u8, bytecode: &[u8]) -> Self {
        let mut name_array = [0u8; 32];
        let len = device_name.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(device_name.as_ptr(), name_array.as_mut_ptr(), len);
        }

        let mut bc_array = [0u8; 16];
        let bc_len = bytecode.len().min(16);
        for i in 0..bc_len {
            bc_array[i] = bytecode[i];
        }

        EarlyBootParameterOverride {
            device_name: name_array,
            port_io_override: port,
            irq_override: irq,
            udf_bytecode: bc_array,
            udf_len: bc_len,
        }
    }
}

/// Linux-inspired early parameter override table for unmatched legacy devices
pub struct LinuxEarlyOverrideTable {
    pub overrides: LocalVec<Option<EarlyBootParameterOverride>>,
}

impl LinuxEarlyOverrideTable {
    pub fn new() -> Self {
        LinuxEarlyOverrideTable {
            overrides: LocalVec::new(),
        }
    }

    pub fn register_override(&mut self, entry: EarlyBootParameterOverride) {
        self.overrides.push(Some(entry));
    }

    /// Checks if a legacy device has early boot-override configuration from early Linux history
    pub fn lookup(&self, device_name: &[u8]) -> Option<&EarlyBootParameterOverride> {
        for i in 0..self.overrides.len() {
            if let Some(ref entry) = self.overrides[i] {
                let entry_name_len = entry.device_name.iter().position(|&b| b == 0).unwrap_or(32);
                if &entry.device_name[..entry_name_len] == device_name {
                    return Some(entry);
                }
            }
        }
        None
    }
}

/// Device manager (OOP: Manager class)
pub struct DeviceManager {
    devices: LocalVec<Option<Box<dyn Device>>>,
    descriptors: LocalVec<Option<NonNull<DeviceDescriptor>>>,
    next_device_id: AtomicUsize,
    pub linux_override_table: LinuxEarlyOverrideTable,
}

impl Default for DeviceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DeviceManager {
    pub fn new() -> Self {
        DeviceManager {
            devices: LocalVec::new(),
            descriptors: LocalVec::new(),
            next_device_id: AtomicUsize::new(1),
            linux_override_table: LinuxEarlyOverrideTable::new(),
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
        if id == 0 || id - 1 >= self.devices.len() {
            return Err(DeviceError::InvalidParameter);
        }

        let idx = id - 1;
        self.devices[idx] = None;

        if let Some(descriptor_ptr) = self.descriptors[idx] {
            unsafe {
                core::ptr::drop_in_place(descriptor_ptr.as_ptr());
                free(descriptor_ptr.as_ptr() as *mut u8);
            }
        }

        self.descriptors[idx] = None;
        Ok(())
    }

    pub fn get_device(&mut self, id: usize) -> Option<&mut Box<dyn Device>> {
        if id > 0 && id - 1 < self.devices.len() {
            self.devices[id - 1].as_mut()
        } else {
            None
        }
    }

    pub fn get_descriptor(&self, id: usize) -> Option<&DeviceDescriptor> {
        if id > 0 && id - 1 < self.descriptors.len() {
            self.descriptors[id - 1].map(|ptr| unsafe { &*ptr.as_ptr() })
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

    pub fn get_devices_by_type(&self, device_type: DeviceType) -> LocalVec<usize> {
        let mut ids = LocalVec::new();
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
}

/// DDE Device Wrapper
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DdeDeviceWrapper {
    pub id: u32,
    pub name: [u8; 32],
    pub base_port: u16,
    pub signature: [u8; 8],
    pub simulated_pci_bar: [u8; 256],
}

impl DdeDeviceWrapper {
    pub fn new(id: u32, name_bytes: &[u8], base_port: u16, signature_bytes: &[u8]) -> Self {
        let mut name = [0u8; 32];
        let len = name_bytes.len().min(32);
        name[..len].copy_from_slice(&name_bytes[..len]);

        let mut signature = [0u8; 8];
        let sig_len = signature_bytes.len().min(8);
        signature[..sig_len].copy_from_slice(&signature_bytes[..sig_len]);

        Self {
            id,
            name,
            base_port,
            signature,
            simulated_pci_bar: [0u8; 256],
        }
    }
}

/// Simple Vec implementation for no_std
pub struct LocalVec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> LocalVec<T> {
    pub fn new() -> Self {
        LocalVec {
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

    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        use core::ops::Deref;
        self.deref().iter()
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

impl<T> core::ops::Deref for LocalVec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.len == 0 {
            &[] as &[T]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for LocalVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.len == 0 {
            &mut [] as &mut [T]
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<'a, T> IntoIterator for &'a LocalVec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        use core::ops::Deref;
        self.deref().iter()
    }
}

impl<'a, T> IntoIterator for &'a mut LocalVec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        use core::ops::DerefMut;
        self.deref_mut().iter_mut()
    }
}

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
