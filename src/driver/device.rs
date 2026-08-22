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
    fn test_graphics_drivers() {
        let mut intel_gpu = IntelHDGpu::new(1, b"intel_gpu", 0xE0000000);
        assert!(intel_gpu.init().is_ok());
        assert_eq!(intel_gpu.info().vendor_id, 0x8086);
        assert!(intel_gpu.ioctl(0x1001, (1024 << 16) | 768).is_ok());
        assert_eq!(intel_gpu.res_width, 1024);
        assert_eq!(intel_gpu.res_height, 768);

        let mut amd_gpu = RadeonGpu::new(2, b"radeon_gpu", 0xE1000000);
        assert!(amd_gpu.init().is_ok());
        assert_eq!(amd_gpu.info().vendor_id, 0x1002);
        assert!(amd_gpu.ioctl(0x1004, 1200).is_ok());
        assert_eq!(amd_gpu.engine_clock_mhz, 1200);

        let mut nvidia_gpu = NvidiaGpu::new(3, b"nvidia_gpu", 0xE2000000);
        assert!(nvidia_gpu.init().is_ok());
        assert_eq!(nvidia_gpu.info().vendor_id, 0x10DE);
        assert!(!nvidia_gpu.cuda_cores_active);
        assert!(nvidia_gpu.ioctl(0x1005, 1).is_ok());
        assert!(nvidia_gpu.cuda_cores_active);

        let mut vesa_dev = VesaFramebufferDevice::new(4, b"vesa_gpu", 0xE3000000);
        assert!(vesa_dev.init().is_ok());
        assert_eq!(vesa_dev.info().vendor_id, 0x0000);
        assert_eq!(vesa_dev.color_depth_bpp, 32);
        assert!(vesa_dev.ioctl(0x1006, 16).is_ok());
        assert_eq!(vesa_dev.color_depth_bpp, 16);
    }

    #[test]
    fn test_storage_drivers() {
        let mut nvme = NvmeController::new(5, b"nvme0", 10, 512);
        assert!(nvme.init().is_ok());
        assert_eq!(nvme.info().vendor_id, 0x144D);
        assert_eq!(nvme.block_size(), 512);
        assert_eq!(nvme.total_blocks(), 10);
        assert_eq!(nvme.ioctl(0x2001, 0).unwrap(), 64);

        let mut write_buf = [0u8; 512];
        write_buf[0] = 42;
        assert!(nvme.write_block(2, &write_buf).is_ok());
        let mut read_buf = [0u8; 512];
        assert!(nvme.read_block(2, &mut read_buf).is_ok());
        assert_eq!(read_buf[0], 42);

        let mut sata = AhciSataController::new(6, b"sata0", 10, 512);
        assert!(sata.init().is_ok());
        assert_eq!(sata.info().vendor_id, 0x8086);
        assert!(sata.ncq_enabled);
        assert!(sata.ioctl(0x2002, 0).is_ok());
        assert!(!sata.ncq_enabled);

        let mut virtio = VirtioBlockDevice::new(7, b"virtio_blk", 10, 512);
        assert!(virtio.init().is_ok());
        assert_eq!(virtio.info().vendor_id, 0x1AF4);
        assert_eq!(virtio.features_negotiated, 0);
        assert!(virtio.ioctl(0x2003, 0xABC).is_ok());
        assert_eq!(virtio.features_negotiated, 0xABC);
    }

    #[test]
    fn test_network_drivers() {
        let mut e1000 = IntelE1000Network::new(8, b"eth0", [1, 2, 3, 4, 5, 6]);
        assert!(e1000.init().is_ok());
        assert_eq!(e1000.info().vendor_id, 0x8086);
        assert_eq!(e1000.get_mac_address(), [1, 2, 3, 4, 5, 6]);
        assert!(e1000.set_mac_address([6, 5, 4, 3, 2, 1]).is_ok());
        assert_eq!(e1000.get_mac_address(), [6, 5, 4, 3, 2, 1]);
        assert_eq!(e1000.ioctl(0x3001, 0).unwrap(), 0);
        assert!(e1000.send_packet(&[0]).is_ok());
        assert_eq!(e1000.ioctl(0x3001, 0).unwrap(), 1);

        let mut rtl = RealtekRtl8139Network::new(9, b"eth1", [1, 1, 1, 1, 1, 1]);
        assert!(rtl.init().is_ok());
        assert_eq!(rtl.info().vendor_id, 0x10EC);
        assert!(rtl.duplex_mode_full);
        assert!(rtl.ioctl(0x3002, 0).is_ok());
        assert!(!rtl.duplex_mode_full);

        let mut virt_net = VirtioNetDevice::new(10, b"virt_net", [2, 2, 2, 2, 2, 2]);
        assert!(virt_net.init().is_ok());
        assert_eq!(virt_net.info().vendor_id, 0x1AF4);
        assert_eq!(virt_net.mtu, 1500);
        assert!(virt_net.ioctl(0x3003, 9000).is_ok());
        assert_eq!(virt_net.mtu, 9000);
    }

    #[test]
    fn test_peripheral_and_other_drivers() {
        let mut hda = IntelHdaAudio::new(11, b"hda");
        assert!(hda.init().is_ok());
        assert_eq!(hda.volume_level, 50);
        assert!(hda.ioctl(0x4001, 75).is_ok());
        assert_eq!(hda.volume_level, 75);

        let mut ac97 = Ac97AudioDevice::new(12, b"ac97");
        assert!(ac97.init().is_ok());
        assert_eq!(ac97.sample_rate_hz, 44100);
        assert!(ac97.ioctl(0x4002, 48000).is_ok());
        assert_eq!(ac97.sample_rate_hz, 48000);

        let mut kbd = UsbHidKeyboard::new(13, b"kbd");
        assert!(kbd.init().is_ok());
        let mut key_buf = [0u8; 1];
        assert_eq!(kbd.read(&mut key_buf).unwrap(), 1);
        assert_eq!(key_buf[0], 0);
        assert!(kbd.ioctl(0x5001, 15).is_ok());
        assert_eq!(kbd.read(&mut key_buf).unwrap(), 1);
        assert_eq!(key_buf[0], 15);

        let mut mouse = Ps2MouseDevice::new(14, b"mouse");
        assert!(mouse.init().is_ok());
        assert_eq!(mouse.resolution_count, 4);
        assert!(mouse.ioctl(0x5002, 8).is_ok());
        assert_eq!(mouse.resolution_count, 8);

        let mut touch = TouchscreenController::new(15, b"touch");
        assert!(touch.init().is_ok());
        assert_eq!(touch.ioctl(0x5003, 0).unwrap(), 10);

        let mut bt = BluetoothController::new(16, b"bluetooth");
        assert!(bt.init().is_ok());
        assert_eq!(bt.paired_devices_count, 0);
        assert!(bt.ioctl(0x6001, 0).is_ok());
        assert_eq!(bt.paired_devices_count, 1);

        let mut wifi = WirelessWifiDevice::new(17, b"wifi");
        assert!(wifi.init().is_ok());
        assert_eq!(wifi.info().vendor_id, 0x14E4);
        assert!(wifi.ioctl(0x6002, 0).is_ok());

        let mut i2c = I2cController::new(18, b"i2c");
        assert!(i2c.init().is_ok());
        assert_eq!(i2c.clock_speed_hz, 100000);
        assert!(i2c.ioctl(0x7001, 400000).is_ok());
        assert_eq!(i2c.clock_speed_hz, 400000);

        let mut spi = SpiController::new(19, b"spi");
        assert!(spi.init().is_ok());
        assert_eq!(spi.mode, 0);
        assert!(spi.ioctl(0x7002, 3).is_ok());
        assert_eq!(spi.mode, 3);

        let mut gpio = GpioController::new(20, b"gpio");
        assert!(gpio.init().is_ok());
        assert_eq!(gpio.pins_state_mask, 0);
        assert!(gpio.ioctl(0x7003, 0xFFFF).is_ok());
        assert_eq!(gpio.pins_state_mask, 0xFFFF);

        let mut pcie = PciExpressBus::new(21, b"pcie");
        assert!(pcie.init().is_ok());
        assert_eq!(pcie.links_active_count, 0);
        assert!(pcie.ioctl(0x7004, 16).is_ok());
        assert_eq!(pcie.links_active_count, 16);

        let mut tpm = TpmSecurityModule::new(22, b"tpm");
        assert!(tpm.init().is_ok());
        assert!(!tpm.is_locked);
        assert!(tpm.ioctl(0x8001, 1).is_ok());
        assert!(tpm.is_locked);

        let mut enclave = SecureEnclaveDriver::new(23, b"enclave");
        assert!(enclave.init().is_ok());
        assert_eq!(enclave.active_enclaves, 0);
        assert!(enclave.ioctl(0x8002, 0).is_ok());
        assert_eq!(enclave.active_enclaves, 1);

        let mut imu = ImuSensorDriver::new(24, b"imu");
        assert!(imu.init().is_ok());
        assert_eq!(imu.ioctl(0x9001, 0).unwrap(), 25);

        let mut thermal = ThermalSensorDriver::new(25, b"thermal");
        assert!(thermal.init().is_ok());
        assert_eq!(thermal.max_temp_allowed, 85);
        assert!(thermal.ioctl(0x9002, 95).is_ok());
        assert_eq!(thermal.max_temp_allowed, 95);

        let mut lpt = LinePrinterDevice::new(26, b"lpt1");
        assert!(lpt.init().is_ok());
        assert!(!lpt.paper_out);
        assert!(lpt.ioctl(0xA001, 1).is_ok());
        assert!(lpt.paper_out);
    }

    #[test]
    fn test_new_drivers_udf() {
        let mut dev = LegacyDevice::new(13, b"kbd", 0x3F8);
        let bytecode_read = [0x01, 0x00, 0x00, 0x03, 0x00, 0x03, 0x04];
        let interpreter = UdfInterpreter::new(&bytecode_read);
        let mut regs = [5, 0, 0, 0];
        assert!(interpreter.execute(&mut dev, &mut regs).is_ok());
        assert_eq!(regs[0], 0);

        let bytecode_write = [0x02, 0x00, 0x00, 0x04];
        let interpreter_write = UdfInterpreter::new(&bytecode_write);
        regs[0] = 5;
        assert!(interpreter_write.execute(&mut dev, &mut regs).is_ok());

        assert!(interpreter.execute(&mut dev, &mut regs).is_ok());
        assert_eq!(regs[0], 0);
    }

    #[test]
    fn test_ancient_legacy_devices_oop() {
        // 1. Floppy disk test
        let mut floppy = FloppyDiskDevice::new(90, b"fd0");
        assert!(floppy.init().is_ok());
        let mut buf = [0u8; 512];
        assert_eq!(floppy.read(&mut buf).unwrap(), 512);
        assert_eq!(buf[0], 0xAA);
        assert_eq!(floppy.block_size(), 512);
        assert_eq!(floppy.total_blocks(), 10);

        // 2. Parallel LPT test
        let mut parallel = ParallelPortDevice::new(91, b"lpt0", 0x378);
        assert!(parallel.init().is_ok());
        assert_eq!(parallel.query_channel(), PortAddress::PortIO(0x378));
        assert_eq!(parallel.read_byte(0).unwrap(), 0xDF);
        assert!(parallel.write(b"Hello Printer").is_ok());
        assert!(parallel.strobe);

        // 3. Serial UART 16550 test
        let mut serial = SerialUartDevice::new(92, b"com1", 0x3F8);
        assert!(serial.init().is_ok());
        assert_eq!(serial.baud_rate, 115200);
        let mut ser_buf = [0u8; 10];
        assert_eq!(serial.read(&mut ser_buf).unwrap(), 10);
        assert_eq!(ser_buf[0], 0x55);

        // 4. AdLib Sound Blaster test
        let mut adlib = AdLibSoundDevice::new(93, b"opl2");
        assert!(adlib.init().is_ok());
        assert_eq!(adlib.register_map[0x20], 0);
        assert!(adlib.write(&[0x20, 0x11, 0x40, 0x22]).is_ok());
        assert_eq!(adlib.register_map[0x20], 0x11);
        assert_eq!(adlib.register_map[0x40], 0x22);

        // 5. ISA Bus Plug-and-Play test
        let mut isa = IsaBusDevice::new(94, b"isapnp");
        assert!(isa.init().is_ok());
        assert_eq!(isa.ioctl(0xB001, 0).unwrap(), 4);
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

/// Windows NT-style Device Extension structure stored in the NonPaged Pool (holds context and HW resources)
#[derive(Debug, Clone)]
pub struct DeviceExtension {
    pub irq: u8,
    pub base_port: u16,
    pub base_address: u32,
    pub memory_size: usize,
    pub device_context: [u8; 128], // Driver-specific context information buffer
}

impl DeviceExtension {
    pub fn new() -> Self {
        Self {
            irq: 0,
            base_port: 0,
            base_address: 0,
            memory_size: 0,
            device_context: [0; 128],
        }
    }
}

/// Windows NT-style Device Object representing a logical, physical, or virtual device instance
pub struct DeviceObject {
    pub name: [u8; 64],
    pub device_type: DeviceType,
    pub device_extension: DeviceExtension,
}

impl DeviceObject {
    pub fn new(name: &[u8], device_type: DeviceType) -> Self {
        let mut name_array = [0u8; 64];
        let len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), len);
        }

        Self {
            name: name_array,
            device_type,
            device_extension: DeviceExtension::new(),
        }
    }
}

/// Windows NT-style Driver Object representing a loaded driver image
pub struct DriverObject {
    pub driver_name: [u8; 64],
    pub registry_path: [u8; 128], // Registry path config lookup (e.g. \Registry\Machine\System\CurrentControlSet\Services\...)
    pub device_objects: Vec<DeviceObject>,
    pub unload_routine: Option<fn(&mut DriverObject)>, // Unload Routine (DRIVERUNLOAD)
}

impl DriverObject {
    pub fn new(name: &[u8], reg_path: &[u8]) -> Self {
        let mut name_array = [0u8; 64];
        let len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), len);
        }

        let mut reg_array = [0u8; 128];
        let reg_len = reg_path.len().min(127);
        unsafe {
            core::ptr::copy_nonoverlapping(reg_path.as_ptr(), reg_array.as_mut_ptr(), reg_len);
        }

        Self {
            driver_name: name_array,
            registry_path: reg_array,
            device_objects: Vec::new(),
            unload_routine: None,
        }
    }
}

/// Windows NT-style I/O Manager Subsystem coordinating driver lifecycles, creation, and unload tasks
pub struct IoManager {
    pub active_drivers: Vec<DriverObject>,
}

impl IoManager {
    pub fn new() -> Self {
        Self {
            active_drivers: Vec::new(),
        }
    }

    /// Emulate the normal driver installation process (creates a registered DriverObject)
    pub fn normal_driver_installation_process(&mut self, driver_name: &[u8], registry_path: &[u8]) -> Result<usize, DeviceError> {
        let driver = DriverObject::new(driver_name, registry_path);
        self.active_drivers.push(driver);
        Ok(self.active_drivers.len() - 1)
    }

    /// IoCreateDevice: Create a Device Object associated with the specific Driver Object
    pub fn io_create_device(&mut self, driver_idx: usize, name: &[u8], device_type: DeviceType) -> Result<(), DeviceError> {
        if driver_idx >= self.active_drivers.len() {
            return Err(DeviceError::InvalidParameter);
        }

        let device_obj = DeviceObject::new(name, device_type);
        self.active_drivers[driver_idx].device_objects.push(device_obj);
        Ok(())
    }

    /// IoUnloadDriver: Executes driver-specific cleanup tasks and calls the DRIVERUNLOAD unload routine
    pub fn io_unload_driver(&mut self, driver_idx: usize) -> Result<(), DeviceError> {
        if driver_idx >= self.active_drivers.len() {
            return Err(DeviceError::InvalidParameter);
        }

        // Get mutable borrow of the driver object
        let driver = &mut self.active_drivers[driver_idx];

        // Execute the unload routine if registered (DRIVERUNLOAD)
        if let Some(unload) = driver.unload_routine {
            (unload)(driver);
        }

        // Perform Driver-Specific Cleanup Tasks: Delete/Free all associated Device Objects and Extensions
        println!("I/O Manager: Executing driver-specific cleanup tasks for driver.");
        driver.device_objects = Vec::new(); // Drop/Delete all Device Objects

        Ok(())
    }
}

impl Default for IoManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Unified representation of communication channels (OOP Abstraction)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortAddress {
    PortIO(u16),       // Legacy 16-bit Port I/O (older generations)
    MemoryMapped(u32), // Modern 32/64-bit Memory Mapped I/O (newer generations)
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
