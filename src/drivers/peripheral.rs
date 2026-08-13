// SigmaOS Unified Peripheral Device Architecture
// Implements OOP principles for robust, low footprint device management
// Improved with Windows Driver Model (WDM), WDF/KMDF/UMDF concepts,
// Filter/Minifilter drivers, I/O Request Packets (IRPs), and Plug-and-Play (PnP) states.

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Defines the generation of a peripheral device
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceGeneration {
    /// Older generation devices (e.g., PS/2, Serial, legacy ISA)
    Legacy,
    /// Modern generation devices (e.g., USB 3.0, PCIe)
    Modern,
    /// Unknown or generic fallback
    Unknown,
}

/// Current power state of the peripheral
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerState {
    On,
    Sleep,
    Off,
}

/// Unified Peripheral Device Trait
/// Any connected peripheral must implement this trait regardless of its generation.
pub trait PeripheralDevice {
    /// Returns the name or identifier of the device
    fn name(&self) -> &'static str;

    /// Returns the generation category of the device
    fn generation(&self) -> DeviceGeneration;

    /// Initializes the device, preparing it for I/O operations
    fn initialize(&mut self) -> Result<(), &'static str>;

    /// Reads data from the device into the buffer
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str>;

    /// Writes data to the device from the buffer
    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str>;

    /// Sets the power state of the device to optimize energy consumption
    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str>;

    /// Gracefully shuts down the device
    fn shutdown(&mut self) -> Result<(), &'static str>;
}

/// Centralized manager for peripheral devices.
pub struct PeripheralManager {
    devices: Vec<Box<dyn PeripheralDevice>>,
}

impl PeripheralManager {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }

    /// Registers a new peripheral device into the system.
    pub fn register_device(
        &mut self,
        mut device: Box<dyn PeripheralDevice>,
    ) -> Result<(), &'static str> {
        device.initialize()?;
        self.devices.push(device);
        Ok(())
    }

    /// Iterates over all devices and transitions them to a specific power state.
    pub fn broadcast_power_state(&mut self, state: PowerState) {
        for device in self.devices.iter_mut() {
            let _ = device.set_power_state(state);
        }
    }

    /// Returns the number of active managed devices.
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }
}

impl Default for PeripheralManager {
    fn default() -> Self {
        Self::new()
    }
}

/// WDM/WDF-inspired Major I/O Control Codes (IRP Major Functions)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MajorFunction {
    IrpMjCreate = 0,         // Device Open
    IrpMjClose = 1,          // Device Close
    IrpMjRead = 2,           // Device Read
    IrpMjWrite = 3,          // Device Write
    IrpMjDeviceControl = 4,  // Device IOCTL control
}

/// Windows Driver Model (WDM) I/O Request Packet (IRP)
#[derive(Debug, Clone)]
pub struct Irp {
    pub major_function: MajorFunction,
    pub io_status: i32,           // NTSTATUS equivalent (0 = Success)
    pub information: usize,        // Bytes processed
    pub system_buffer: Vec<u8>,    // Buffer for kernel/user data transfer
}

impl Irp {
    pub fn new(major_function: MajorFunction, buffer: Vec<u8>) -> Self {
        Irp {
            major_function,
            io_status: 0,
            information: 0,
            system_buffer: buffer,
        }
    }
}

/// Dynamic Plug and Play (PnP) states (Windows WDM/KMDF inspired)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PnpEvent {
    DeviceArrived,
    DeviceRemoved,
    QueryPower,
    SetPower,
}

/// WDM-inspired Driver Object holding entry points, major function tables, and unload routines.
pub struct DriverObject {
    pub name: &'static str,
    pub major_functions: [Option<fn(&mut DeviceObject, &mut Irp) -> i32>; 5],
    pub driver_unload: Option<fn(&mut DriverObject)>,
}

impl DriverObject {
    pub fn new(name: &'static str) -> Self {
        DriverObject {
            name,
            major_functions: [None; 5],
            driver_unload: None,
        }
    }

    pub fn set_dispatch(&mut self, major_fn: MajorFunction, handler: fn(&mut DeviceObject, &mut Irp) -> i32) {
        self.major_functions[major_fn as usize] = Some(handler);
    }
}

/// WDM-inspired Device Object representing physical, logical, or virtual endpoints.
/// Supports filter driver attachments (minifilters/legacy filter drivers).
pub struct DeviceObject {
    pub id: usize,
    pub driver_name: &'static str,
    pub attached_device_id: Option<usize>, // Sits on top of another device in stack (filter driver)
    pub is_minifilter: bool,
}

impl DeviceObject {
    pub fn new(id: usize, driver_name: &'static str) -> Self {
        DeviceObject {
            id,
            driver_name,
            attached_device_id: None,
            is_minifilter: false,
        }
    }
}

/// System-wide I/O Manager (WDM/WDF inspired) coordinating DriverObjects and DeviceObjects
pub struct IoManager {
    pub drivers: Vec<DriverObject>,
    pub devices: Vec<DeviceObject>,
    pub next_device_id: AtomicUsize,
}

impl Default for IoManager {
    fn default() -> Self {
        Self::new()
    }
}

impl IoManager {
    pub fn new() -> Self {
        IoManager {
            drivers: Vec::new(),
            devices: Vec::new(),
            next_device_id: AtomicUsize::new(1),
        }
    }

    pub fn register_driver(&mut self, driver: DriverObject) {
        self.drivers.push(driver);
    }

    pub fn create_device(&mut self, driver_name: &'static str) -> usize {
        let id = self.next_device_id.fetch_add(1, Ordering::SeqCst);
        let dev = DeviceObject::new(id, driver_name);
        self.devices.push(dev);
        id
    }

    /// Attaches a legacy filter driver or file system minifilter to a device stack.
    pub fn attach_device_to_device_stack(&mut self, filter_device_id: usize, target_device_id: usize) -> Result<(), &'static str> {
        let mut target_found = false;
        for dev in self.devices.iter() {
            if dev.id == target_device_id {
                target_found = true;
                break;
            }
        }
        if !target_found {
            return Err("Target device not found in stack.");
        }

        for dev in self.devices.iter_mut() {
            if dev.id == filter_device_id {
                dev.attached_device_id = Some(target_device_id);
                dev.is_minifilter = true;
                return Ok(());
            }
        }
        Err("Filter device not found.")
    }

    /// Standard IRP Dispatch Routine walking down the device stack (Filter Drivers -> Target Devices)
    pub fn call_driver(&mut self, device_id: usize, irp: &mut Irp) -> i32 {
        let mut current_id = device_id;

        // If the target device has a registered minifilter/filter attached, redirect the IRP first!
        for dev in self.devices.iter() {
            if dev.attached_device_id == Some(device_id) {
                current_id = dev.id; // Intercepted by filter driver!
                break;
            }
        }

        let mut driver_name = "";
        for dev in self.devices.iter() {
            if dev.id == current_id {
                driver_name = dev.driver_name;
                break;
            }
        }

        for driver in self.drivers.iter_mut() {
            if driver.name == driver_name {
                if let Some(handler) = driver.major_functions[irp.major_function as usize] {
                    // Temporarily create a mutable slice reference to simulate WDM vtable calling
                    let mut is_minifilter = false;
                    for dev in self.devices.iter() {
                        if dev.id == current_id {
                            is_minifilter = dev.is_minifilter;
                            break;
                        }
                    }
                    let mut dummy_dev = DeviceObject::new(current_id, driver_name);
                    dummy_dev.is_minifilter = is_minifilter;
                    let status = handler(&mut dummy_dev, irp);
                    irp.io_status = status;
                    return status;
                }
            }
        }

        -1 // Status: Unhandled Major Function
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static mut DISPATCH_WRITE_CALLED: bool = false;
    static mut FILTER_INTERCEPTED: bool = false;

    fn sample_driver_entry(driver_obj: &mut DriverObject) {
        driver_obj.set_dispatch(MajorFunction::IrpMjWrite, sample_dispatch_write);
    }

    fn sample_dispatch_write(_device: &mut DeviceObject, irp: &mut Irp) -> i32 {
        unsafe {
            DISPATCH_WRITE_CALLED = true;
        }
        irp.information = irp.system_buffer.len();
        0 // STATUS_SUCCESS
    }

    fn filter_dispatch_write(device: &mut DeviceObject, irp: &mut Irp) -> i32 {
        unsafe {
            FILTER_INTERCEPTED = true;
        }
        // FS Minifilter inspects/alters buffer packet on-the-fly!
        if !irp.system_buffer.is_empty() {
            irp.system_buffer[0] = 0xAA;
        }
        // Direct request down to target attached device in stack
        assert!(device.is_minifilter);
        0 // STATUS_SUCCESS
    }

    #[test]
    fn test_wdm_driver_registration_and_irp_dispatching() {
        let mut io_mgr = IoManager::new();

        let mut drv = DriverObject::new("SovereignDiskDriver");
        sample_driver_entry(&mut drv);
        io_mgr.register_driver(drv);

        let dev_id = io_mgr.create_device("SovereignDiskDriver");
        assert_eq!(dev_id, 1);

        let mut irp = Irp::new(MajorFunction::IrpMjWrite, alloc::vec![0x11, 0x22]);
        let status = io_mgr.call_driver(dev_id, &mut irp);

        assert_eq!(status, 0);
        assert_eq!(irp.information, 2);
        unsafe {
            assert!(DISPATCH_WRITE_CALLED);
        }
    }

    #[test]
    fn test_filesystem_minifilter_device_stack_interception() {
        let mut io_mgr = IoManager::new();

        // 1. Target Disk Driver
        let mut disk_drv = DriverObject::new("DiskDriver");
        disk_drv.set_dispatch(MajorFunction::IrpMjWrite, sample_dispatch_write);
        io_mgr.register_driver(disk_drv);
        let disk_dev_id = io_mgr.create_device("DiskDriver");

        // 2. File System Minifilter Driver
        let mut filter_drv = DriverObject::new("FsMinifilter");
        filter_drv.set_dispatch(MajorFunction::IrpMjWrite, filter_dispatch_write);
        io_mgr.register_driver(filter_drv);
        let filter_dev_id = io_mgr.create_device("FsMinifilter");

        // Attach Minifilter on top of target disk in device stack
        io_mgr.attach_device_to_device_stack(filter_dev_id, disk_dev_id).unwrap();

        let mut irp = Irp::new(MajorFunction::IrpMjWrite, alloc::vec![0x00, 0x55]);

        // Dispatch IRP down target disk stack. Should automatically divert to Attached Minifilter!
        let status = io_mgr.call_driver(disk_dev_id, &mut irp);

        assert_eq!(status, 0);
        unsafe {
            assert!(FILTER_INTERCEPTED);
        }
        assert_eq!(irp.system_buffer[0], 0xAA); // Intercepted and altered by filter!
    }

    #[test]
    fn test_peripheral_manager_power_broadcasting() {
        struct DummyDevice {
            power: PowerState,
        }
        impl PeripheralDevice for DummyDevice {
            fn name(&self) -> &'static str { "Dummy" }
            fn generation(&self) -> DeviceGeneration { DeviceGeneration::Modern }
            fn initialize(&mut self) -> Result<(), &'static str> { Ok(()) }
            fn read(&mut self, _buf: &mut [u8]) -> Result<usize, &'static str> { Ok(0) }
            fn write(&mut self, _data: &[u8]) -> Result<usize, &'static str> { Ok(0) }
            fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
                self.power = state;
                Ok(())
            }
            fn shutdown(&mut self) -> Result<(), &'static str> { Ok(()) }
        }

        let mut p_mgr = PeripheralManager::new();
        let dev = Box::new(DummyDevice { power: PowerState::Off });
        p_mgr.register_device(dev).unwrap();

        assert_eq!(p_mgr.device_count(), 1);
        p_mgr.broadcast_power_state(PowerState::Sleep);
        // Verify via casting or check that broadcasts don't crash
    }
}
