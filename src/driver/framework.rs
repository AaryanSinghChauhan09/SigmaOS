// Windows Driver Model (WDM) / Kernel-Mode Driver Framework (KMDF) & UMDF
// Zero-dependency, #![no_std] compliant, highly compatible driver architecture.

#![no_std]

#[cfg(test)]
extern crate std;

extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::mem;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type DriverID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverType {
    Block = 0,
    Char = 1,
    Network = 2,
    Filter = 3,
    MiniFilter = 4,
    Storage = 5,
    Input = 6,
}

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverState {
    Unloaded = 0,
    Loaded = 1,
    Active = 2,
}

pub trait Driver {
    fn id(&self) -> DriverID;
    fn driver_type(&self) -> DriverType;
    fn state(&self) -> DriverState;
    fn set_state(&self, _state: DriverState) {}
    fn init(&mut self) -> Result<(), DriverError> { Ok(()) }
    fn probe(&mut self) -> Result<bool, DriverError> { Ok(true) }
    fn shutdown(&mut self) -> Result<(), DriverError> { Ok(()) }
    fn dependencies(&self) -> &'static [DriverType] { &[] }
    fn load(&mut self) -> Result<(), DriverError>;
    fn unload(&mut self) -> Result<(), DriverError>;
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverError {
    Success = 0,
    LoadFailed = 1,
    UnloadFailed = 2,
    InvalidDevice = 3,
    IrpNotHandled = 4,
    AccessDenied = 5,
    InvalidParameter = 6,
    ProbeFailed = 7,
}

pub type SimpleStorageDriver = SimpleDriver;

#[repr(C)]
pub struct SimpleDriver {
    pub id: DriverID,
    pub driver_type: DriverType,
    pub state: AtomicUsize,
}

impl SimpleDriver {
    pub fn new(id: DriverID, driver_type: DriverType) -> Self {
        SimpleDriver {
            id,
            driver_type,
            state: AtomicUsize::new(DriverState::Unloaded as usize),
        }
    }

    pub fn init(&mut self) -> Result<(), DriverError> {
        Ok(())
    }

    pub fn probe(&self) -> Result<bool, DriverError> {
        Ok(true)
    }

    pub fn shutdown(&mut self) -> Result<(), DriverError> {
        Ok(())
    }
}

impl Driver for SimpleDriver {
    fn id(&self) -> DriverID {
        self.id
    }
    fn driver_type(&self) -> DriverType {
        self.driver_type
    }
    fn state(&self) -> DriverState {
        unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) }
    }
    fn load(&mut self) -> Result<(), DriverError> {
        self.state
            .store(DriverState::Loaded as usize, Ordering::SeqCst);
        Ok(())
    }
    fn unload(&mut self) -> Result<(), DriverError> {
        self.state
            .store(DriverState::Unloaded as usize, Ordering::SeqCst);
        Ok(())
    }
}

// =========================================================================
// WDM & WDF (KMDF / UMDF) Specification Subsystems
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DevicePowerState {
    D0 = 0, // Fully working
    D1 = 1, // Low power sleeping
    D2 = 2, // Deeper sleep
    D3 = 3, // Off / unpowered
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PnpMinorFunction {
    StartDevice,
    QueryStopDevice,
    StopDevice,
    CancelStopDevice,
    QueryRemoveDevice,
    RemoveDevice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrpMajorFunction {
    Create,        // IRP_MJ_CREATE
    Close,         // IRP_MJ_CLOSE
    Read,          // IRP_MJ_READ
    Write,         // IRP_MJ_WRITE
    DeviceControl, // IRP_MJ_DEVICE_CONTROL (IOCTL)
    Power,         // IRP_MJ_POWER
    Pnp,           // IRP_MJ_PNP
}

/// Buffer transfer methods for user-kernel communication
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoctlTransferMethod {
    MethodBuffered, // Copies input/output via system-allocated kernel buffer
    MethodInDirect, // Maps input buffer directly via MDL (read access)
    MethodOutDirect, // Maps output buffer directly via MDL (write access)
    MethodNeither,  // Uses raw user-mode virtual addresses directly (unsafe/requires validation)
}

/// Simulated IOCTL Control Code structure
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IoctlCode {
    pub device_type: u32,
    pub function: u32,
    pub method: IoctlTransferMethod,
    pub access: u32,
}

impl IoctlCode {
    /// CTL_CODE macro equivalent: packs control parameters into a single 32-bit integer
    pub const fn build(device_type: u32, function: u32, method: IoctlTransferMethod, access: u32) -> Self {
        Self {
            device_type,
            function,
            method,
            access,
        }
    }

    pub fn to_u32(&self) -> u32 {
        (self.device_type << 16) | (self.access << 14) | (self.function << 2) | (self.method as u32)
    }
}

#[derive(Debug, Clone)]
pub struct Irp {
    pub major_function: IrpMajorFunction,
    pub io_status: DriverError,
    pub byte_offset: u64,
    pub power_state: Option<DevicePowerState>,
    pub pnp_function: Option<PnpMinorFunction>,

    // User-Kernel memory buffers
    pub ioctl_code: Option<IoctlCode>,
    pub input_buffer: Vec<u8>,
    pub output_buffer: Vec<u8>,
    pub user_mode_virtual_address: Option<u64>,
    pub physical_pages_mdl: Vec<u64>, // Simulated locked physical frames for Direct I/O
}

impl Irp {
    pub fn new(major: IrpMajorFunction) -> Self {
        Self {
            major_function: major,
            io_status: DriverError::Success,
            byte_offset: 0,
            power_state: None,
            pnp_function: None,
            ioctl_code: None,
            input_buffer: Vec::new(),
            output_buffer: Vec::new(),
            user_mode_virtual_address: None,
            physical_pages_mdl: Vec::new(),
        }
    }
}

pub type PdriverDispatch = fn(device: &mut DeviceObject, irp: &mut Irp) -> DriverError;

pub struct DriverObject {
    pub driver_name: String,
    pub driver_start: u64,
    pub driver_size: usize,
    pub major_function: [Option<PdriverDispatch>; 10], // Registered Dispatch Routines
    pub driver_unload: Option<fn(&mut DriverObject)>,
}

impl DriverObject {
    pub fn new(name: &str) -> Self {
        Self {
            driver_name: String::from(name),
            driver_start: 0,
            driver_size: 0,
            major_function: [None; 10],
            driver_unload: None,
        }
    }

    pub fn register_dispatch(&mut self, func: IrpMajorFunction, callback: PdriverDispatch) {
        self.major_function[func as usize] = Some(callback);
    }
}

pub struct DeviceObject {
    pub driver_object: *mut DriverObject,
    pub next_device: *mut DeviceObject, // Attached filters/minifilters (Device Stack)
    pub current_power_state: DevicePowerState,
    pub is_kernel_mode: bool, // True for KMDF, False for UMDF
    pub device_extension: u64, // Device-specific state representation
}

impl DeviceObject {
    pub fn new(drv: *mut DriverObject, is_kernel: bool) -> Self {
        Self {
            driver_object: drv,
            next_device: core::ptr::null_mut(),
            current_power_state: DevicePowerState::D0,
            is_kernel_mode: is_kernel,
            device_extension: 0,
        }
    }
}

pub struct IoManager {
    pub devices: Vec<*mut DeviceObject>,
}

impl IoManager {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }

    pub fn register_device(&mut self, device: *mut DeviceObject) {
        self.devices.push(device);
    }

    /// Dispatches an I/O Request Packet (IRP) down the registered device stack
    pub unsafe fn dispatch_irp(&mut self, device: *mut DeviceObject, irp: &mut Irp) -> DriverError {
        if device.is_null() {
            return DriverError::InvalidDevice;
        }

        let drv_ptr = (*device).driver_object;
        if drv_ptr.is_null() {
            return DriverError::InvalidDevice;
        }

        let major_fn = irp.major_function;
        if let Some(dispatch_fn) = (*drv_ptr).major_function[major_fn as usize] {
            let res = dispatch_fn(&mut *device, irp);
            if res == DriverError::Success && !(*device).next_device.is_null() {
                // Pass the IRP down the attached filter/device stack
                return self.dispatch_irp((*device).next_device, irp);
            }
            res
        } else {
            DriverError::IrpNotHandled
        }
    }
}

// =========================================================================
// Real-world Legacy, IOCTL, and Filter Driver Simulators
// =========================================================================

/// Keyboard Filter Driver (captures key logs securely, like PS/2 filter)
pub fn keyboard_filter_dispatch(device: &mut DeviceObject, irp: &mut Irp) -> DriverError {
    if irp.major_function == IrpMajorFunction::Read {
        // Intercept keys, append high entropy codes
        irp.input_buffer.push(0x41); // Simulated keystroke scan-code
    }
    device.current_power_state = DevicePowerState::D0;
    DriverError::Success
}

/// USB Storage Filter Driver (enforces forensic write-blocking and IOCTL handlers)
pub fn usb_forensic_filter_dispatch(device: &mut DeviceObject, irp: &mut Irp) -> DriverError {
    match irp.major_function {
        IrpMajorFunction::Write => {
            // Return write-protected failure simulating the forensic write-blocker
            irp.io_status = DriverError::UnloadFailed;
            return DriverError::UnloadFailed;
        }
        IrpMajorFunction::DeviceControl => {
            if let Some(code) = irp.ioctl_code {
                match code.method {
                    IoctlTransferMethod::MethodBuffered => {
                        // Buffered I/O: read input from system kernel buffer, safely write output
                        if !irp.input_buffer.is_empty() {
                            let cmd = irp.input_buffer[0];
                            if cmd == 0x99 { // Mock unlock command
                                irp.output_buffer.push(0x01); // Success code
                            }
                        }
                    }
                    IoctlTransferMethod::MethodInDirect | IoctlTransferMethod::MethodOutDirect => {
                        // Direct I/O: safely access mapped physical pages via MDL
                        if irp.physical_pages_mdl.is_empty() {
                            return DriverError::InvalidParameter;
                        }
                        irp.output_buffer.push(0xAB);
                    }
                    IoctlTransferMethod::MethodNeither => {
                        // Neither I/O: raw user virtual address requires explicit validation
                        if let Some(user_addr) = irp.user_mode_virtual_address {
                            if user_addr >= 0xFFFF_8000_0000_0000 {
                                return DriverError::AccessDenied; // Attempted kernel space access!
                            }
                            irp.output_buffer.push(0xFE);
                        }
                    }
                }
            }
        }
        _ => {}
    }
    device.current_power_state = DevicePowerState::D0;
    DriverError::Success
}

// =========================================================================
// Interrupt Handling & Interrupt Descriptor Table (IDT) Subsystems
// =========================================================================

pub type PisrHandler = fn(vector: u8, context: u64) -> bool;

#[derive(Debug, Clone, Copy)]
pub struct IdtEntry {
    pub handler_address: u64,
    pub privilege_level: u8, // DPL: 0 for Kernel-Mode, 3 for User-Mode
    pub is_present: bool,
}

pub struct InterruptDescriptorTable {
    pub entries: [Option<IdtEntry>; 256],
}

impl InterruptDescriptorTable {
    pub fn new() -> Self {
        Self {
            entries: [None; 256],
        }
    }

    pub fn register_isr(&mut self, vector: u8, handler: u64, privilege: u8) {
        let entry = IdtEntry {
            handler_address: handler,
            privilege_level: privilege,
            is_present: true,
        };
        self.entries[vector as usize] = Some(entry);
    }

    /// Simulates triggering a hardware interrupt. Validates IDT presence and rings privilege
    pub fn trigger_interrupt(&self, vector: u8, current_ring_privilege: u8, isr: PisrHandler, context: u64) -> Result<bool, &'static str> {
        let entry = self.entries[vector as usize].ok_or("Interrupt vector not registered in IDT")?;
        if !entry.is_present {
            return Err("Interrupt gate is disabled");
        }

        if current_ring_privilege > entry.privilege_level {
            return Err("General Protection Fault: Privilege violation accessing IDT gate");
        }

        // Call Custom Interrupt Handler
        let handled = isr(vector, context);
        Ok(handled)
    }
}

// =========================================================================
// Existing DriverFramework implementation & extensions
// =========================================================================

pub trait DriverFramework {
    fn register_driver(&mut self, driver: Box<dyn Driver>) -> Result<DriverID, DriverError>;
    fn load_driver(&mut self, id: DriverID) -> Result<(), DriverError>;
    fn unload_driver(&mut self, id: DriverID) -> Result<(), DriverError>;
    fn get_driver(&self, id: DriverID) -> Option<&dyn Driver>;
}

#[allow(dead_code)]
pub struct SimpleDriverFramework {
    drivers: Vec<Option<Box<dyn Driver>>>,
    next_id: AtomicUsize,
}

impl Default for SimpleDriverFramework {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleDriverFramework {
    pub fn new() -> Self {
        SimpleDriverFramework {
            drivers: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl DriverFramework for SimpleDriverFramework {
    fn register_driver(&mut self, driver: Box<dyn Driver>) -> Result<DriverID, DriverError> {
        let id = driver.id();
        self.drivers.push(Some(driver));
        Ok(id)
    }
    fn load_driver(&mut self, id: DriverID) -> Result<(), DriverError> {
        for driver_option in self.drivers.iter_mut() {
            if let Some(ref mut driver) = *driver_option {
                if driver.id() == id {
                    return driver.load();
                }
            }
        }
        Err(DriverError::LoadFailed)
    }
    fn unload_driver(&mut self, id: DriverID) -> Result<(), DriverError> {
        for driver_option in self.drivers.iter_mut() {
            if let Some(ref mut driver) = *driver_option {
                if driver.id() == id {
                    return driver.unload();
                }
            }
        }
        Err(DriverError::UnloadFailed)
    }
    fn get_driver(&self, id: DriverID) -> Option<&dyn Driver> {
        for driver_option in self.drivers.iter() {
            if let Some(ref driver) = *driver_option {
                if driver.id() == id {
                    return Some(driver.as_ref());
                }
            }
        }
        None
    }
}

// ==========================================
// Unit Tests
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_framework_lifecycle() {
        let mut framework = SimpleDriverFramework::new();
        let driver = Box::new(SimpleStorageDriver::new(101, DriverType::Block));

        let reg_id = framework.register_driver(driver).unwrap();
        assert_eq!(reg_id, 101);

        assert_eq!(framework.get_driver(101).unwrap().state(), DriverState::Unloaded);

        framework.load_driver(101).unwrap();
        assert_eq!(framework.get_driver(101).unwrap().state(), DriverState::Active);

        framework.unload_driver(101).unwrap();
        assert_eq!(framework.get_driver(101).unwrap().state(), DriverState::Unloaded);
    }

    #[test]
    fn test_wdm_irp_device_stack_flow() {
        unsafe {
            let mut drv_obj = DriverObject::new("TestDriver");
            let mut dev_obj = DeviceObject::new(&mut drv_obj as *mut DriverObject, true);
            dev_obj.current_power_state = DevicePowerState::D0;

            // Register Create & Close Dispatch Routines
            fn test_create_dispatch(device: &mut DeviceObject, irp: &mut Irp) -> DriverError {
                device.device_extension = 0xAA55;
                irp.input_buffer.push(0xCC);
                DriverError::Success
            }

            drv_obj.register_dispatch(IrpMajorFunction::Create, test_create_dispatch);

            let mut io_mgr = IoManager::new();
            io_mgr.register_device(&mut dev_obj as *mut DeviceObject);

            let mut create_irp = Irp::new(IrpMajorFunction::Create);

            let res = io_mgr.dispatch_irp(&mut dev_obj as *mut DeviceObject, &mut create_irp);
            assert_eq!(res, DriverError::Success);
            assert_eq!(dev_obj.device_extension, 0xAA55);
            assert_eq!(create_irp.input_buffer[0], 0xCC);
        }
    }

    #[test]
    fn test_keyboard_and_usb_storage_filters() {
        unsafe {
            let mut keyboard_drv = DriverObject::new("KbdFilter");
            keyboard_drv.register_dispatch(IrpMajorFunction::Read, keyboard_filter_dispatch);

            let mut keyboard_dev = DeviceObject::new(&mut keyboard_drv as *mut DriverObject, true);

            let mut usb_drv = DriverObject::new("UsbStorageFilter");
            usb_drv.register_dispatch(IrpMajorFunction::Write, usb_forensic_filter_dispatch);

            let mut usb_dev = DeviceObject::new(&mut usb_drv as *mut DriverObject, true);

            let mut io_mgr = IoManager::new();

            // Test Keyboard Filter keystroke interception
            let mut kbd_irp = Irp::new(IrpMajorFunction::Read);
            let res_kbd = io_mgr.dispatch_irp(&mut keyboard_dev as *mut DeviceObject, &mut kbd_irp);
            assert_eq!(res_kbd, DriverError::Success);
            assert_eq!(kbd_irp.input_buffer[0], 0x41); // Key logged correctly!

            // Test USB Forensic Filter write protection
            let mut usb_irp = Irp::new(IrpMajorFunction::Write);
            let res_usb = io_mgr.dispatch_irp(&mut usb_dev as *mut DeviceObject, &mut usb_irp);
            assert_eq!(res_usb, DriverError::UnloadFailed); // Write blocked successfully!
        }
    }

    #[test]
    fn test_ioctl_buffered_direct_neither_io() {
        unsafe {
            let mut usb_drv = DriverObject::new("UsbIOCTL");
            usb_drv.register_dispatch(IrpMajorFunction::DeviceControl, usb_forensic_filter_dispatch);

            let mut usb_dev = DeviceObject::new(&mut usb_drv as *mut DriverObject, true);
            let mut io_mgr = IoManager::new();

            // 1. Test METHOD_BUFFERED (valid unlock command)
            let mut irp_buffered = Irp::new(IrpMajorFunction::DeviceControl);
            irp_buffered.ioctl_code = Some(IoctlCode::build(0x00000022, 0x801, IoctlTransferMethod::MethodBuffered, 0x01));
            irp_buffered.input_buffer.push(0x99); // Unlock code

            let res_buffered = io_mgr.dispatch_irp(&mut usb_dev as *mut DeviceObject, &mut irp_buffered);
            assert_eq!(res_buffered, DriverError::Success);
            assert_eq!(irp_buffered.output_buffer[0], 0x01);

            // 2. Test METHOD_IN_DIRECT with locked MDL pages
            let mut irp_direct = Irp::new(IrpMajorFunction::DeviceControl);
            irp_direct.ioctl_code = Some(IoctlCode::build(0x00000022, 0x802, IoctlTransferMethod::MethodInDirect, 0x01));
            irp_direct.physical_pages_mdl.push(0x1000_1000); // Mock MDL page

            let res_direct = io_mgr.dispatch_irp(&mut usb_dev as *mut DeviceObject, &mut irp_direct);
            assert_eq!(res_direct, DriverError::Success);
            assert_eq!(irp_direct.output_buffer[0], 0xAB);

            // 3. Test METHOD_NEITHER (safe user address)
            let mut irp_neither_safe = Irp::new(IrpMajorFunction::DeviceControl);
            irp_neither_safe.ioctl_code = Some(IoctlCode::build(0x00000022, 0x803, IoctlTransferMethod::MethodNeither, 0x01));
            irp_neither_safe.user_mode_virtual_address = Some(0x0000_7FFF_FFFF_F000); // User space

            let res_neither_safe = io_mgr.dispatch_irp(&mut usb_dev as *mut DeviceObject, &mut irp_neither_safe);
            assert_eq!(res_neither_safe, DriverError::Success);
            assert_eq!(irp_neither_safe.output_buffer[0], 0xFE);

            // 4. Test METHOD_NEITHER (malicious address in kernel space)
            let mut irp_neither_malicious = Irp::new(IrpMajorFunction::DeviceControl);
            irp_neither_malicious.ioctl_code = Some(IoctlCode::build(0x00000022, 0x803, IoctlTransferMethod::MethodNeither, 0x01));
            irp_neither_malicious.user_mode_virtual_address = Some(0xFFFF_8000_0000_0000); // Kernel space

            let res_neither_malicious = io_mgr.dispatch_irp(&mut usb_dev as *mut DeviceObject, &mut irp_neither_malicious);
            assert_eq!(res_neither_malicious, DriverError::AccessDenied);
        }
    }

    #[test]
    fn test_interrupt_descriptor_table_and_gates() {
        let mut idt = InterruptDescriptorTable::new();

        // Register ISR for vector 0x21 (Keyboard Interrupt)
        idt.register_isr(0x21, 0x1000_5000, 0); // DPL: 0 (Kernel-Mode)

        // 1. Trigger from Kernel space (Ring 0) -> Succeeded!
        static mut ISR_RAN: bool = false;
        fn mock_keyboard_isr(vector: u8, context: u64) -> bool {
            assert_eq!(vector, 0x21);
            assert_eq!(context, 0x99AA);
            unsafe {
                ISR_RAN = true;
            }
            true
        }

        let res_kernel = idt.trigger_interrupt(0x21, 0, mock_keyboard_isr, 0x99AA);
        assert!(res_kernel.is_ok());
        assert!(res_kernel.unwrap());
        assert!(unsafe { ISR_RAN });

        // 2. Trigger from User space (Ring 3) to Ring 0 Gate -> Fails with privilege error!
        let res_user = idt.trigger_interrupt(0x21, 3, mock_keyboard_isr, 0x99AA);
        assert!(res_user.is_err());
        assert_eq!(res_user.unwrap_err(), "General Protection Fault: Privilege violation accessing IDT gate");
    }
}
