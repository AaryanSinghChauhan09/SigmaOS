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

/// Simple block device implementation
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
        let blocks = vec![vec![0u8; block_size]; num_blocks];

        let mut info = DeviceInfo::new(DeviceType::Block);
        info.vendor_id = 0x8086;
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
        name[..len].copy_from_slice(&param_name[..len]);
        Self { name, value }
    }
}

/// Device manager
pub struct DeviceManager {
    devices: Vec<Option<Box<dyn Device>>>,
    probe_entries: Vec<DriverProbeEntry>,
    module_params: Vec<DriverModuleParam>,
    next_device_id: AtomicUsize,
}

impl Default for DeviceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DeviceManager {
    pub fn new() -> Self {
        DeviceManager {
            devices: Vec::new(),
            probe_entries: Vec::new(),
            module_params: Vec::new(),
            next_device_id: AtomicUsize::new(1),
        }
    }

    pub fn register_device(
        &mut self,
        device: Box<dyn Device>,
        _name: &[u8],
        _device_type: DeviceType,
        _capability: DeviceCapability,
    ) -> Result<usize, DeviceError> {
        let id = self.next_device_id.fetch_add(1, Ordering::SeqCst);
        self.devices.push(Some(device));
        Ok(id)
    }

    pub fn unregister_device(&mut self, id: usize) -> Result<(), DeviceError> {
        if id >= self.devices.len() {
            return Err(DeviceError::InvalidParameter);
        }
        self.devices[id] = None;
        Ok(())
    }

    pub fn get_device(&mut self, id: usize) -> Option<&mut Box<dyn Device>> {
        if id < self.devices.len() {
            self.devices[id].as_mut()
        } else {
            None
        }
    }

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

    pub fn auto_probe_and_bind(&mut self, vendor_id: u16, device_id: u16, device_type: DeviceType) -> bool {
        for entry in self.probe_entries.iter() {
            if entry.vendor_id == vendor_id && entry.device_id == device_id && entry.device_type == device_type {
                return true;
            }
        }
        false
    }
}

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

        mgr.set_module_param(b"debug_level", 4);
        assert_eq!(mgr.get_module_param(b"debug_level"), Some(4));
        assert_eq!(mgr.get_module_param(b"non_existent"), None);

        let entry = DriverProbeEntry {
            vendor_id: 0x10EC,
            device_id: 0x8168,
            device_type: DeviceType::Network,
        };
        mgr.register_probe_match(entry);

        assert!(mgr.auto_probe_and_bind(0x10EC, 0x8168, DeviceType::Network));
        assert!(!mgr.auto_probe_and_bind(0xFFFF, 0xFFFF, DeviceType::Network));
    }
}
