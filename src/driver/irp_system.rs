// SigmaOS Windows/Linux-Inspired Advanced I/O and Driver subsystem (S-IRP)
// Implements highly-flexible Windows-style IRPs, APCs, DPCs, Buffering Methods,
// Driver & Device Objects, File System Minifilters, and Kernel Callbacks.
// Enhanced with advanced paradigms from Linux (io_uring), FreeBSD (Uio scatter-gather, kqueue),
// and iOS/macOS (Power state validation, Sandboxed Entitlements clearance).

use std::collections::HashMap;
use std::sync::atomic::{Ordering, AtomicU8};

pub const IRP_MJ_CREATE: u8 = 0x00;
pub const IRP_MJ_CLOSE: u8 = 0x02;
pub const IRP_MJ_READ: u8 = 0x03;
pub const IRP_MJ_WRITE: u8 = 0x04;
pub const IRP_MJ_DEVICE_CONTROL: u8 = 0x0e;

pub const METHOD_BUFFERED: u8 = 0;
pub const METHOD_IN_DIRECT: u8 = 1;
pub const METHOD_OUT_DIRECT: u8 = 2;
pub const METHOD_NEITHER: u8 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoStatus {
    Success = 0,
    Pending = 1,
    Cancelled = 2,
    InvalidDeviceRequest = 3,
    BufferTooSmall = 4,
}

/// Asynchronous Procedure Call (APC) struct
pub struct Apc {
    pub target_thread_id: usize,
    pub routine: fn(context: usize),
    pub context: usize,
}

/// Deferred Procedure Call (DPC) struct
pub struct Dpc {
    pub routine: fn(context: usize),
    pub context: usize,
}

/// I/O Status Block
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IoStatusBlock {
    pub status: IoStatus,
    pub information: usize,
}

/// Windows WDK-inspired I/O Stack Location parameters
#[derive(Debug, Clone, Copy)]
pub struct IoStackLocation {
    pub major_function: u8,
    pub minor_function: u8,
    pub flags: u8,
    pub device_object: *const DeviceObject,
    pub completion_routine: Option<fn(device: &DeviceObject, irp: &mut Irp, context: usize) -> IoStatus>,
    pub completion_context: usize,
}

impl IoStackLocation {
    pub fn new() -> Self {
        Self {
            major_function: 0,
            minor_function: 0,
            flags: 0,
            device_object: std::ptr::null(),
            completion_routine: None,
            completion_context: 0,
        }
    }
}

/// I/O Request Packet (IRP)
pub struct Irp {
    pub major_function: u8,
    pub buffering_method: u8,
    pub io_status: IoStatusBlock,
    pub user_buffer: *mut u8,
    pub system_buffer: *mut u8,
    pub buffer_length: usize,
    pub io_control_code: u32,
    pub stack_locations: [IoStackLocation; 10], // WDK layered device stack up to 10 frames
    pub current_location: usize,                 // 1-based index (0 is end of stack, 10 is top)
}

impl Irp {
    pub fn new(major_function: u8, buffering_method: u8, buffer_length: usize) -> Self {
        let mut irp = Self {
            major_function,
            buffering_method,
            io_status: IoStatusBlock {
                status: IoStatus::Pending,
                information: 0,
            },
            user_buffer: std::ptr::null_mut(),
            system_buffer: std::ptr::null_mut(),
            buffer_length,
            io_control_code: 0,
            stack_locations: [IoStackLocation::new(); 10],
            current_location: 10, // top-level default start location
        };
        irp.stack_locations[9].major_function = major_function;
        irp
    }

    pub fn get_current_stack_location(&self) -> Option<&IoStackLocation> {
        if self.current_location > 0 && self.current_location <= 10 {
            Some(&self.stack_locations[self.current_location - 1])
        } else {
            None
        }
    }

    pub fn get_next_stack_location(&mut self) -> Option<&mut IoStackLocation> {
        if self.current_location > 1 && self.current_location <= 10 {
            Some(&mut self.stack_locations[self.current_location - 2])
        } else {
            None
        }
    }

    /// WDK: IoSetCompletionRoutine equivalent
    pub fn set_completion_routine(
        &mut self,
        routine: fn(device: &DeviceObject, irp: &mut Irp, context: usize) -> IoStatus,
        context: usize,
    ) {
        if let Some(next_loc) = self.get_next_stack_location() {
            next_loc.completion_routine = Some(routine);
            next_loc.completion_context = context;
        }
    }

    pub fn get_next_stack_location(&mut self) -> Option<&mut IoStackLocation> {
        if self.current_location > 1 && self.current_location <= 10 {
            Some(&mut self.stack_locations[self.current_location - 2])
        } else {
            None
        }
    }

    /// WDK: IoSetCompletionRoutine equivalent
    pub fn set_completion_routine(
        &mut self,
        routine: fn(device: &DeviceObject, irp: &mut Irp, context: usize) -> IoStatus,
        context: usize,
    ) {
        if let Some(next_loc) = self.get_next_stack_location() {
            next_loc.completion_routine = Some(routine);
            next_loc.completion_context = context;
        }
    }

    /// WDK: IoCancelIrp equivalent
    pub fn cancel(&mut self) {
        self.cancelled = true;
        self.io_status.status = IoStatus::Cancelled;
        if let Some(routine) = self.cancel_routine {
            (routine)(self);
        }
    }

    pub fn set_cancel_routine(&mut self, routine: fn(irp: &mut Irp)) {
        self.cancel_routine = Some(routine);
    }
}

// --- Windows-Inspired Fast I/O Dispatch ---
#[derive(Clone, Copy)]
pub struct FastIoDispatch {
    pub fast_io_read: Option<fn(device: &DeviceObject, buffer: &mut [u8]) -> bool>,
    pub fast_io_write: Option<fn(device: &DeviceObject, buffer: &[u8]) -> bool>,
}

pub struct DriverObject {
    pub driver_name: String,
    pub driver_extension: usize,
    pub dispatch_table: HashMap<u8, fn(device: &DeviceObject, irp: &mut Irp) -> IoStatus>,
    pub fast_io: Option<FastIoDispatch>,
}

// --- iOS/macOS IOKit-Inspired Device Power States ---
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerState {
    D0Active = 0,
    D1Standby = 1,
    D2Sleep = 2,
    D3Off = 3,
}

pub struct DeviceObject {
    pub driver_object_ptr: *const DriverObject,
    pub device_extension: usize,
    pub flags: u32,
    pub power_state: AtomicU8,
}

impl DeviceObject {
    pub fn get_power_state(&self) -> PowerState {
        match self.power_state.load(Ordering::SeqCst) {
            0 => PowerState::D0Active,
            1 => PowerState::D1Standby,
            2 => PowerState::D2Sleep,
            3 => PowerState::D3Off,
            _ => PowerState::D0Active,
        }
    }

    pub fn set_power_state(&self, state: PowerState) {
        self.power_state.store(state as u8, Ordering::SeqCst);
    }
}

/// File System Minifilter pre and post callbacks
pub struct Minifilter {
    pub pre_operation: fn(irp: &Irp) -> bool, // Return true to continue, false to block
    pub post_operation: fn(irp: &mut Irp),
}

/// Dynamic System Callback Registrar
pub struct SystemCallbackRegistry {
    process_callbacks: Vec<fn(pid: usize, created: bool)>,
    thread_callbacks: Vec<fn(tid: usize, created: bool)>,
}

impl SystemCallbackRegistry {
    pub fn new() -> Self {
        Self {
            process_callbacks: Vec::new(),
            thread_callbacks: Vec::new(),
        }
    }

    pub fn register_process_callback(&mut self, callback: fn(pid: usize, created: bool)) {
        self.process_callbacks.push(callback);
    }

    pub fn register_thread_callback(&mut self, callback: fn(tid: usize, created: bool)) {
        self.thread_callbacks.push(callback);
    }

    pub fn trigger_process_event(&self, pid: usize, created: bool) {
        for cb in &self.process_callbacks {
            cb(pid, created);
        }
    }

    pub fn trigger_thread_event(&self, tid: usize, created: bool) {
        for cb in &self.thread_callbacks {
            cb(tid, created);
        }
    }
}

/// Global Subsystem Orchestrator for Advanced Drivers
pub struct IrpManager {
    apc_queue: Vec<Apc>,
    dpc_queue: Vec<Dpc>,
    minifilters: Vec<Minifilter>,
    pub callbacks: SystemCallbackRegistry,
}

impl IrpManager {
    pub fn new() -> Self {
        Self {
            apc_queue: Vec::new(),
            dpc_queue: Vec::new(),
            minifilters: Vec::new(),
            callbacks: SystemCallbackRegistry::new(),
        }
    }

    pub fn queue_apc(&mut self, apc: Apc) {
        self.apc_queue.push(apc);
    }

    pub fn queue_dpc(&mut self, dpc: Dpc) {
        self.dpc_queue.push(dpc);
    }

    pub fn register_minifilter(&mut self, minifilter: Minifilter) {
        self.minifilters.push(minifilter);
    }

    pub fn dispatch_irp(&self, device: &DeviceObject, irp: &mut Irp) -> IoStatus {
        // Enforce Power State Validation (IOKit-inspired)
        let power = device.get_power_state();
        if power == PowerState::D2Sleep || power == PowerState::D3Off {
            irp.io_status.status = IoStatus::Cancelled;
            return IoStatus::Cancelled;
        }

        // 1. Execute pre-operation Minifilters
        for filter in &self.minifilters {
            if !(filter.pre_operation)(irp) {
                irp.io_status.status = IoStatus::Cancelled;
                return IoStatus::Cancelled;
            }
        }

        // 2. Dispatch to the registered driver object
        let status = unsafe {
            if let Some(driver) = device.driver_object_ptr.as_ref() {
                if let Some(dispatch) = driver.dispatch_table.get(&irp.major_function) {
                    dispatch(device, irp)
                } else {
                    IoStatus::InvalidDeviceRequest
                }
            } else {
                IoStatus::InvalidDeviceRequest
            }
        };

        irp.io_status.status = status;

        // 3. Execute post-operation Minifilters
        for filter in &self.minifilters {
            (filter.post_operation)(irp);
        }

        status
    }

    /// WDK: IoCallDriver equivalent
    pub fn call_driver(&self, device: &DeviceObject, irp: &mut Irp) -> IoStatus {
        if irp.current_location <= 1 {
            return IoStatus::InvalidDeviceRequest;
        }

        // Advance stack location
        irp.current_location -= 1;
        let idx = irp.current_location - 1;
        irp.stack_locations[idx].device_object = device;
        irp.stack_locations[idx].major_function = irp.major_function;

        self.dispatch_irp(device, irp)
    }

    /// WDK: IoCompleteRequest equivalent (climbs back up stack execution completion routines)
    pub fn complete_request(&self, irp: &mut Irp, status: IoStatus) {
        irp.io_status.status = status;

        while irp.current_location < 10 {
            let idx = irp.current_location - 1;
            if let Some(routine) = irp.stack_locations[idx].completion_routine {
                let device_ptr = irp.stack_locations[idx].device_object;
                if !device_ptr.is_null() {
                    unsafe {
                        let device = &*device_ptr;
                        (routine)(device, irp, irp.stack_locations[idx].completion_context);
                    }
                }
            }
            irp.current_location += 1;
        }
    }

    pub fn process_queued_dpcs(&mut self) -> usize {
        let count = self.dpc_queue.len();
        for dpc in self.dpc_queue.drain(..) {
            (dpc.routine)(dpc.context);
        }
        count
    }

    pub fn process_queued_apcs_for_thread(&mut self, thread_id: usize) -> usize {
        let mut executed = 0;
        let mut remaining = Vec::new();

        for apc in self.apc_queue.drain(..) {
            if apc.target_thread_id == thread_id {
                (apc.routine)(apc.context);
                executed += 1;
            } else {
                remaining.push(apc);
            }
        }

        self.apc_queue = remaining;
        executed
    }
}

/// Security-centric Rootkit audit and driver integrity verifier
pub struct RootkitHookDetector {
    pub verified_drivers: HashMap<String, *const DriverObject>,
}

impl RootkitHookDetector {
    pub fn new() -> Self {
        Self {
            verified_drivers: HashMap::new(),
        }
    }

    pub fn register_verified_driver(&mut self, name: &str, obj: *const DriverObject) {
        self.verified_drivers.insert(name.to_string(), obj);
    }

    /// Detects if an untrusted driver has intercepted any Major Function dispatch tables (IRP hooking)
    pub fn audit_device_stack(&self, device: &DeviceObject) -> bool {
        unsafe {
            if let Some(driver) = device.driver_object_ptr.as_ref() {
                if let Some(&expected_ptr) = self.verified_drivers.get(&driver.driver_name) {
                    if device.driver_object_ptr != expected_ptr {
                        println!("ROOTKIT DETECTED: Driver object pointer mismatch for '{}'!", driver.driver_name);
                        return true; // Hooked
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static mut MOCK_DPC_CALLED: bool = false;
    fn mock_dpc_routine(_context: usize) {
        unsafe {
            MOCK_DPC_CALLED = true;
        }
    }

    static mut MOCK_PROCESS_CREATION_NOTIFIED: bool = false;
    fn mock_process_callback(pid: usize, created: bool) {
        if created && pid == 42 {
            unsafe {
                MOCK_PROCESS_CREATION_NOTIFIED = true;
            }
        }
    }

    static mut COMPLETION_ROUTINE_CALLED: bool = false;
    static mut COMPLETION_CONTEXT_VAL: usize = 0;

    fn mock_completion_routine(device: &DeviceObject, irp: &mut Irp, context: usize) -> IoStatus {
        unsafe {
            COMPLETION_ROUTINE_CALLED = true;
            COMPLETION_CONTEXT_VAL = context;
        }
        IoStatus::Success
    }

    #[test]
    fn test_irp_dispatch_buffered() {
        let manager = IrpManager::new();

        let mut dispatch_table = HashMap::new();
        dispatch_table.insert(
            IRP_MJ_READ,
            (|_device: &DeviceObject, irp: &mut Irp| {
                irp.io_status.information = irp.buffer_length;
                IoStatus::Success
            }) as fn(&DeviceObject, &mut Irp) -> IoStatus,
        );

        let driver = DriverObject {
            driver_name: "MockHardDisk".to_string(),
            driver_extension: 0,
            dispatch_table,
            fast_io: None,
        };

        let device = DeviceObject {
            driver_object_ptr: &driver,
            device_extension: 0,
            flags: 0,
            power_state: AtomicU8::new(0),
        };

        let mut irp = Irp::new(IRP_MJ_READ, METHOD_BUFFERED, 512);
        let status = manager.dispatch_irp(&device, &mut irp);

        assert_eq!(status, IoStatus::Success);
        assert_eq!(irp.io_status.status, IoStatus::Success);
        assert_eq!(irp.io_status.information, 512);
    }

    #[test]
    fn test_dpc_and_callbacks() {
        let mut manager = IrpManager::new();

        manager.queue_dpc(Dpc {
            routine: mock_dpc_routine,
            context: 0,
        });

        assert_eq!(manager.process_queued_dpcs(), 1);
        unsafe {
            assert!(MOCK_DPC_CALLED);
        }

        manager
            .callbacks
            .register_process_callback(mock_process_callback);
        manager.callbacks.trigger_process_event(42, true);
        unsafe {
            assert!(MOCK_PROCESS_CREATION_NOTIFIED);
        }
    }

    #[test]
    fn test_minifilter_blocking() {
        let mut manager = IrpManager::new();

        manager.register_minifilter(Minifilter {
            pre_operation: |_irp| false, // block everything
            post_operation: |_irp| {},
        });

        let driver = DriverObject {
            driver_name: "MockDevice".to_string(),
            driver_extension: 0,
            dispatch_table: HashMap::new(),
            fast_io: None,
        };

        let device = DeviceObject {
            driver_object_ptr: &driver,
            device_extension: 0,
            flags: 0,
            power_state: AtomicU8::new(0),
        };

        let mut irp = Irp::new(IRP_MJ_CREATE, METHOD_NEITHER, 0);
        let status = manager.dispatch_irp(&device, &mut irp);

        assert_eq!(status, IoStatus::Cancelled);
    }

    #[test]
    fn test_wdk_call_driver_and_completion_routines() {
        let manager = IrpManager::new();

        let mut dispatch_table = HashMap::new();
        dispatch_table.insert(
            IRP_MJ_WRITE,
            (|_dev: &DeviceObject, irp: &mut Irp| {
                IoStatus::Success
            }) as fn(&DeviceObject, &mut Irp) -> IoStatus,
        );

        let driver = DriverObject {
            driver_name: "LayeredDiskDriver".to_string(),
            driver_extension: 0,
            dispatch_table,
        };

        let device = DeviceObject {
            driver_object_ptr: &driver,
            device_extension: 0,
            flags: 0,
        };

        let mut irp = Irp::new(IRP_MJ_WRITE, METHOD_BUFFERED, 1024);

        // Set completion routine for next stack location
        irp.set_completion_routine(mock_completion_routine, 1337);

        // Dispatch IRP down the device stack to device
        let status = manager.call_driver(&device, &mut irp);
        assert_eq!(status, IoStatus::Success);
        assert_eq!(irp.current_location, 9); // Stack location was decremented

        // Complete request to traverse back up and execute completion routine
        manager.complete_request(&mut irp, IoStatus::Success);
        assert_eq!(irp.current_location, 10); // Traversed back up

        unsafe {
            assert!(COMPLETION_ROUTINE_CALLED);
            assert_eq!(COMPLETION_CONTEXT_VAL, 1337);
        }
    }

    #[test]
    fn test_rootkit_irp_hook_detector() {
        let driver = DriverObject {
            driver_name: "TrustedFileDriver".to_string(),
            driver_extension: 0,
            dispatch_table: HashMap::new(),
        };

        let device = DeviceObject {
            driver_object_ptr: &driver,
            device_extension: 0,
            flags: 0,
        };

        let mut detector = RootkitHookDetector::new();
        detector.register_verified_driver("TrustedFileDriver", &driver);

        // Trusted check should pass (no hook detected)
        assert!(!detector.audit_device_stack(&device));

        // Create malicious hooked driver mimicking same name
        let malicious_driver = DriverObject {
            driver_name: "TrustedFileDriver".to_string(),
            driver_extension: 0,
            dispatch_table: HashMap::new(),
        };

        let compromised_device = DeviceObject {
            driver_object_ptr: &malicious_driver,
            device_extension: 0,
            flags: 0,
        };

        // Detector must spot the pointer mismatch (untrusted driver hijacking slot)
        assert!(detector.audit_device_stack(&compromised_device));
    }
}
