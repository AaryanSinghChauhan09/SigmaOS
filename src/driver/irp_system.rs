// SigmaOS Windows/Linux-Inspired Advanced I/O and Driver subsystem (S-IRP)
// Implements highly-flexible Windows-style IRPs, APCs, DPCs, Buffering Methods,
// Driver & Device Objects, File System Minifilters, and Kernel Callbacks.

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

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

/// I/O Request Packet (IRP)
pub struct Irp {
    pub major_function: u8,
    pub buffering_method: u8,
    pub io_status: IoStatusBlock,
    pub user_buffer: *mut u8,
    pub system_buffer: *mut u8,
    pub buffer_length: usize,
    pub io_control_code: u32,
}

impl Irp {
    pub fn new(major_function: u8, buffering_method: u8, buffer_length: usize) -> Self {
        Self {
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
        }
    }
}

pub struct DriverObject {
    pub driver_name: String,
    pub driver_extension: usize,
    pub dispatch_table: HashMap<u8, fn(device: &DeviceObject, irp: &mut Irp) -> IoStatus>,
}

pub struct DeviceObject {
    pub driver_object_ptr: *const DriverObject,
    pub device_extension: usize,
    pub flags: u32,
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

#[cfg(test)]
mod tests {
    use super::*;

    static mut MOCK_DPC_CALLED: bool = false;
    fn mock_dpc_routine(context: usize) {
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

    #[test]
    fn test_irp_dispatch_buffered() {
        let mut manager = IrpManager::new();

        let mut dispatch_table = HashMap::new();
        dispatch_table.insert(
            IRP_MJ_READ,
            (|device: &DeviceObject, irp: &mut Irp| {
                irp.io_status.information = irp.buffer_length;
                IoStatus::Success
            }) as fn(&DeviceObject, &mut Irp) -> IoStatus,
        );

        let driver = DriverObject {
            driver_name: "MockHardDisk".to_string(),
            driver_extension: 0,
            dispatch_table,
        };

        let device = DeviceObject {
            driver_object_ptr: &driver,
            device_extension: 0,
            flags: 0,
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
        };

        let device = DeviceObject {
            driver_object_ptr: &driver,
            device_extension: 0,
            flags: 0,
        };

        let mut irp = Irp::new(IRP_MJ_CREATE, METHOD_NEITHER, 0);
        let status = manager.dispatch_irp(&device, &mut irp);

        assert_eq!(status, IoStatus::Cancelled);
    }
}
