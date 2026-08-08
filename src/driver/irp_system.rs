// SigmaOS Windows/Linux-Inspired Advanced I/O and Driver subsystem (S-IRP)
// Implements highly-flexible Windows-style IRPs, APCs, DPCs, Buffering Methods,
// Driver & Device Objects, File System Minifilters, and Kernel Callbacks.
// Expanded with a robust IO Manager, Object Manager, Non-Paged Pool, Rootkit Detector, and Union parameters.

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
    DriverLoadFailed = 5,
    ObjectNotFound = 6,
    AccessDenied = 7,
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
}

/// Partially Opaque Structure: Opaque Driver Extension containing critical security
/// keys, locks, and private driver states hidden from direct external read/writes.
pub struct OpaqueDriverExtension {
    pub validation_token: u64,
    pub security_key: [u8; 16],
    pub lock_state: bool,
}

pub type DriverEntry = fn(driver_object: &mut DriverObject) -> IoStatus;

pub struct DriverObject {
    pub driver_name: String,
    pub driver_extension: usize,
    pub dispatch_table: HashMap<u8, fn(device: &DeviceObject, irp: &mut Irp) -> IoStatus>,
    pub original_dispatch_table: HashMap<u8, usize>, // Signatures for rootkit detection (address mappings)
    pub driver_entry: Option<DriverEntry>,
    pub settings: HashMap<String, String>,
    pub opaque_extension: Option<OpaqueDriverExtension>, // Opaque block protecting driver keys
}

impl DriverObject {
    pub fn new(name: &str) -> Self {
        Self {
            driver_name: name.to_string(),
            driver_extension: 0,
            dispatch_table: HashMap::new(),
            original_dispatch_table: HashMap::new(),
            driver_entry: None,
            settings: HashMap::new(),
            opaque_extension: None,
        }
    }

    /// Helper to register custom IRP major function dispatch handlers
    pub fn register_dispatch_routine(
        &mut self,
        major_function: u8,
        routine: fn(device: &DeviceObject, irp: &mut Irp) -> IoStatus,
    ) {
        self.dispatch_table.insert(major_function, routine);
        // Track the raw address to allow the Rootkit Detector to verify integrity
        self.original_dispatch_table.insert(major_function, routine as usize);
    }
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
        for cb in self.track_thread_callbacks_safely() {
            cb(tid, created);
        }
    }

    fn track_thread_callbacks_safely(&self) -> &Vec<fn(tid: usize, created: bool)> {
        &self.thread_callbacks
    }
}

/// Windows-inspired Object Manager Namespace node types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectType {
    Directory,
    Device { device_idx: usize },
    SymbolicLink { target_path: String },
    Alias { target_path: String },
}

/// Object Manager: Mimics the highly organized Windows namespace hierarchy
pub struct ObjectManager {
    pub objects: HashMap<String, ObjectType>, // Path (e.g. "\Device\Harddisk0") -> ObjectType
}

impl ObjectManager {
    pub fn new() -> Self {
        let mut om = Self {
            objects: HashMap::new(),
        };
        // Seed default root directories
        om.create_directory("\\").unwrap();
        om.create_directory("\\Device").unwrap();
        om.create_directory("\\DosDevices").unwrap();
        om
    }

    pub fn create_directory(&mut self, path: &str) -> Result<(), IoStatus> {
        self.objects.insert(path.to_string(), ObjectType::Directory);
        Ok(())
    }

    pub fn create_device_link(&mut self, path: &str, device_idx: usize) -> Result<(), IoStatus> {
        self.objects.insert(path.to_string(), ObjectType::Device { device_idx });
        Ok(())
    }

    pub fn create_symbolic_link(&mut self, path: &str, target_path: &str) -> Result<(), IoStatus> {
        self.objects.insert(
            path.to_string(),
            ObjectType::SymbolicLink {
                target_path: target_path.to_string(),
            },
        );
        Ok(())
    }

    /// Resolves aliases and symbolic links to retrieve the real target path
    pub fn resolve_path(&self, path: &str) -> Result<String, IoStatus> {
        let mut current_path = path.to_string();
        let mut hops = 0;

        while let Some(obj) = self.objects.get(&current_path) {
            if hops > 10 {
                return Err(IoStatus::Cancelled); // Prevent circular symbolic loops
            }
            match obj {
                ObjectType::SymbolicLink { target_path } => {
                    current_path = target_path.clone();
                    hops += 1;
                }
                ObjectType::Alias { target_path } => {
                    current_path = target_path.clone();
                    hops += 1;
                }
                _ => break,
            }
        }

        if self.objects.contains_key(&current_path) {
            Ok(current_path)
        } else {
            Err(IoStatus::ObjectNotFound)
        }
    }
}

/// Non-Paged Pool Memory Manager
/// Allocates critical unpaged physical RAM blocks for drivers
pub struct NonPagedPool {
    pub pool_size_bytes: usize,
    pub allocated_bytes: AtomicUsize,
}

impl NonPagedPool {
    pub const fn new(size: usize) -> Self {
        Self {
            pool_size_bytes: size,
            allocated_bytes: AtomicUsize::new(0),
        }
    }

    pub fn allocate(&self, size: usize) -> Result<usize, IoStatus> {
        let current = self.allocated_bytes.load(Ordering::SeqCst);
        if current + size > self.pool_size_bytes {
            return Err(IoStatus::BufferTooSmall);
        }
        self.allocated_bytes.store(current + size, Ordering::SeqCst);
        Ok(size)
    }

    pub fn deallocate(&self, size: usize) {
        let current = self.allocated_bytes.load(Ordering::SeqCst);
        self.allocated_bytes.store(current.saturating_sub(size), Ordering::SeqCst);
    }
}

/// Rootkit Detector
/// Scans driver dispatch tables for hook alterations (hijacked pointer targets)
pub struct RootkitDetector;

impl RootkitDetector {
    pub fn is_driver_compromised(driver: &DriverObject) -> bool {
        for (func, &original_addr) in &driver.original_dispatch_table {
            if let Some(&current_addr) = driver.dispatch_table.get(func) {
                // If current address does not match original registered address,
                // a rootkit has hooked the dispatch routine!
                if current_addr as usize != original_addr {
                    return true;
                }
            }
        }
        false
    }
}

/// IoManager: Highly structured, self-optimising OS-level I/O & Device coordinator
pub struct IoManager {
    apc_queue: Vec<Apc>,
    dpc_queue: Vec<Dpc>,
    minifilters: Vec<Minifilter>,
    pub callbacks: SystemCallbackRegistry,
    pub loaded_drivers: HashMap<String, DriverObject>,
    pub registered_devices: Vec<DeviceObject>,
    pub object_manager: ObjectManager,
    pub non_paged_pool: NonPagedPool,
}

impl IoManager {
    pub fn new() -> Self {
        Self {
            apc_queue: Vec::new(),
            dpc_queue: Vec::new(),
            minifilters: Vec::new(),
            callbacks: SystemCallbackRegistry::new(),
            loaded_drivers: HashMap::new(),
            registered_devices: Vec::new(),
            object_manager: ObjectManager::new(),
            non_paged_pool: NonPagedPool::new(1024 * 1024 * 16), // 16MB Non-Paged Pool
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

    /// Dynamic Driver Loading Process mimicking advanced micro-architectures
    pub fn dynamic_load_driver(
        &mut self,
        name: &str,
        entry: DriverEntry,
        settings: HashMap<String, String>,
    ) -> Result<&DriverObject, IoStatus> {
        let mut driver_obj = DriverObject::new(name);
        driver_obj.driver_entry = Some(entry);
        driver_obj.settings = settings;

        // Initialize opaque security extension
        driver_obj.opaque_extension = Some(OpaqueDriverExtension {
            validation_token: 0xDEADBEEFCAFEBABE,
            security_key: [0x11; 16],
            lock_state: false,
        });

        // Execute Driver Entry Point
        let status = entry(&mut driver_obj);
        if status != IoStatus::Success {
            return Err(IoStatus::DriverLoadFailed);
        }

        self.loaded_drivers.insert(name.to_string(), driver_obj);
        Ok(self.loaded_drivers.get(name).unwrap())
    }

    /// Dynamic Unloading Process of drivers freeing memory pools and device links cleanly
    pub fn dynamic_unload_driver(&mut self, name: &str) -> Result<(), IoStatus> {
        if self.loaded_drivers.remove(name).is_some() {
            // Clean up registered symbolic links under \DosDevices for the driver
            let symlink_path = format!("\\DosDevices\\{}", name);
            self.object_manager.objects.remove(&symlink_path);
            Ok(())
        } else {
            Err(IoStatus::ObjectNotFound)
        }
    }

    /// Default Handler used when a driver does not register a specific Major Function dispatch
    pub fn default_dispatch_handler(&self, _device: &DeviceObject, irp: &mut Irp) -> IoStatus {
        irp.io_status.information = 0;
        IoStatus::InvalidDeviceRequest
    }

    /// Sends an IRP down the driver stack, executing pre/post filters and fallback dispatchers
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
                    // Fallback to the default handler
                    self.default_dispatch_handler(device, irp)
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
        let mut manager = IoManager::new();

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
            driver_entry: None,
            settings: HashMap::new(),
            opaque_extension: None,
            original_dispatch_table: HashMap::new(),
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
        let mut manager = IoManager::new();

        manager.queue_dpc(Dpc {
            routine: mock_dpc_routine,
            context: 0,
        });

        assert_eq!(manager.process_queued_dpcs(), 1);
        unsafe {
            assert!(MOCK_DPC_CALLED);
        }

        manager.callbacks.register_process_callback(mock_process_callback);
        manager.callbacks.trigger_process_event(42, true);
        unsafe {
            assert!(MOCK_PROCESS_CREATION_NOTIFIED);
        }
    }

    #[test]
    fn test_minifilter_blocking() {
        let mut manager = IoManager::new();

        manager.register_minifilter(Minifilter {
            pre_operation: |_irp| false, // block everything
            post_operation: |_irp| {},
        });

        let driver = DriverObject {
            driver_name: "MockDevice".to_string(),
            driver_extension: 0,
            dispatch_table: HashMap::new(),
            driver_entry: None,
            settings: HashMap::new(),
            opaque_extension: None,
            original_dispatch_table: HashMap::new(),
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
