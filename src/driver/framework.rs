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
pub type SdfResult<T> = Result<T, DriverError>;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverType {
    Block = 0,
    Char = 1,
    Network = 2,
    Storage = 3,
    Input = 4,
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
    fn load(&mut self) -> Result<(), DriverError>;
    fn unload(&mut self) -> Result<(), DriverError>;
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverError {
    Success = 0,
    LoadFailed = 1,
    UnloadFailed = 2,
    ProbeFailed = 3,
}

pub trait GraphicsDriver: Driver {
    fn set_resolution(&mut self, width: u32, height: u32) -> Result<(), DriverError>;
    fn flip_buffers(&mut self) -> Result<(), DriverError>;
}

pub trait InputDriver: Driver {
    fn poll_events(&mut self) -> Result<usize, DriverError>;
}

// Concrete Driver Classes (OOP Implementation)

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

    pub fn probe(&mut self) -> Result<bool, DriverError> {
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
        self.state.store(DriverState::Active as usize, Ordering::SeqCst);
        Ok(())
    }
    fn unload(&mut self) -> Result<(), DriverError> {
        self.state.store(DriverState::Unloaded as usize, Ordering::SeqCst);
        Ok(())
    }
}

pub type SimpleStorageDriver = SimpleDriver;

pub trait DriverFramework {
    fn register_driver(&mut self, driver: Box<dyn Driver>) -> Result<DriverID, DriverError>;
    fn load_driver(&mut self, id: DriverID) -> Result<(), DriverError>;
    fn unload_driver(&mut self, id: DriverID) -> Result<(), DriverError>;
    fn get_driver(&self, id: DriverID) -> Option<&dyn Driver>;
}

pub struct SimpleDriverFramework {
    drivers: Vec<Option<Box<dyn Driver>>>,
    next_id: AtomicUsize,
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
        assert_eq!(framework.get_driver(101).unwrap().state(), DriverState::Loaded);

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


#[cfg(test)]
mod tests {
    use super::*;

    static mut OPEN_CALLED: i32 = 0;
    static mut RELEASE_CALLED: i32 = 0;

    fn mock_open() -> i32 {
        unsafe { OPEN_CALLED += 1; }
        0
    }

    fn mock_release() -> i32 {
        unsafe { RELEASE_CALLED += 1; }
        0
    }

    fn mock_read(_buf: &mut [u8]) -> i32 { 0 }
    fn mock_write(_buf: &[u8]) -> i32 { 0 }
    fn mock_ioctl(_cmd: u32, _arg: u64) -> i32 { 0 }

    #[test]
    fn test_linux_driver_shim() {
        let fops = LinuxFileOperations {
            open: mock_open,
            release: mock_release,
            read: mock_read,
            write: mock_write,
            ioctl: mock_ioctl,
        };

        let mut shim = LinuxDriverShim::new(42, "e1000", DriverType::Network, fops);
        assert_eq!(shim.id(), 42);
        assert_eq!(shim.driver_type(), DriverType::Network);

        assert!(shim.init().is_ok());
        unsafe { assert_eq!(OPEN_CALLED, 1); }

        assert!(shim.load().is_ok());
        assert_eq!(shim.state(), DriverState::Active);

        assert!(shim.unload().is_ok());
        unsafe { assert_eq!(RELEASE_CALLED, 1); }
    }

    #[test]
    fn test_procedural_driver_dispatch_table() {
        let table = ProceduralDriverDispatchTable::empty();
        assert_eq!((table.p_init)(10), 0);
        assert_eq!((table.p_open)(10), 0);
        assert_eq!((table.p_close)(10), 0);
        assert_eq!((table.p_read)(10, core::ptr::null_mut(), 0), 0);
        assert_eq!((table.p_write)(10, core::ptr::null(), 0), 0);
        assert_eq!((table.p_ioctl)(10, 0x1234, 0), 0);
    }
}
