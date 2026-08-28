// SigmaOS Windows/Linux-Inspired Advanced I/O and Driver subsystem (S-IRP)
// Implements highly-flexible Windows-style IRPs, APCs, DPCs, Buffering Methods,
// Driver & Device Objects, File System Minifilters, and Kernel Callbacks.
// Enhanced with advanced paradigms from Linux (io_uring), FreeBSD (Uio scatter-gather, kqueue),
// and iOS/macOS (Power state validation, Sandboxed Entitlements clearance).

use crate::klib::HashMap;
use std::sync::atomic::{AtomicU8, Ordering};

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
    pub completion_routine:
        Option<fn(device: &DeviceObject, irp: &mut Irp, context: usize) -> IoStatus>,
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
    pub current_location: usize,                // 1-based index (0 is end of stack, 10 is top)
    pub cancel_routine: Option<fn(irp: &mut Irp)>,
    pub cancelled: bool,
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
            cancel_routine: None,
            cancelled: false,
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

    // --- Windows-style FAST_IO Execution Path ---
    pub fn try_fast_io_read(&self, device: &DeviceObject, buffer: &mut [u8]) -> bool {
        if device.get_power_state() != PowerState::D0Active {
            return false;
        }
        unsafe {
            if let Some(driver) = device.driver_object_ptr.as_ref() {
                if let Some(fast_io) = driver.fast_io {
                    if let Some(fast_read) = fast_io.fast_io_read {
                        return fast_read(device, buffer);
                    }
                }
            }
        }
        false
    }

    pub fn try_fast_io_write(&self, device: &DeviceObject, buffer: &[u8]) -> bool {
        if device.get_power_state() != PowerState::D0Active {
            return false;
        }
        unsafe {
            if let Some(driver) = device.driver_object_ptr.as_ref() {
                if let Some(fast_io) = driver.fast_io {
                    if let Some(fast_write) = fast_io.fast_io_write {
                        return fast_write(device, buffer);
                    }
                }
            }
        }
        false
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

// =========================================================
// 1. Linux-style io_uring Interface (Batch & Async Execution)
// =========================================================
pub const IORING_OP_READ: u8 = 0;
pub const IORING_OP_WRITE: u8 = 1;
pub const IORING_OP_DEVICE_CONTROL: u8 = 2;

#[derive(Debug, Clone, Copy)]
pub struct SubmissionQueueEntry {
    pub opcode: u8,
    pub buffer: *mut u8,
    pub len: usize,
    pub io_control_code: u32,
    pub user_data: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct CompletionQueueEntry {
    pub user_data: u64,
    pub status: IoStatus,
    pub result: usize,
}

pub struct IoUring {
    pub sq: Vec<SubmissionQueueEntry>,
    pub cq: Vec<CompletionQueueEntry>,
}

impl IoUring {
    pub fn new() -> Self {
        Self {
            sq: Vec::new(),
            cq: Vec::new(),
        }
    }

    pub fn submit(&mut self, entry: SubmissionQueueEntry) {
        self.sq.push(entry);
    }

    pub fn poll_completions(&mut self, manager: &IrpManager, device: &DeviceObject) -> usize {
        let count = self.sq.len();
        for entry in self.sq.drain(..) {
            let major_func = match entry.opcode {
                IORING_OP_READ => IRP_MJ_READ,
                IORING_OP_WRITE => IRP_MJ_WRITE,
                IORING_OP_DEVICE_CONTROL => IRP_MJ_DEVICE_CONTROL,
                _ => IRP_MJ_READ,
            };

            let mut irp = Irp::new(major_func, METHOD_BUFFERED, entry.len);
            irp.system_buffer = entry.buffer;
            irp.io_control_code = entry.io_control_code;

            let status = manager.dispatch_irp(device, &mut irp);
            self.cq.push(CompletionQueueEntry {
                user_data: entry.user_data,
                status,
                result: irp.io_status.information,
            });
        }
        count
    }
}

// =========================================================
// 2. BSD-style Scatter-Gather Uio Struct
// =========================================================
#[derive(Debug, Clone, Copy)]
pub struct UioSegment {
    pub buffer: *mut u8,
    pub len: usize,
}

pub struct Uio {
    pub segments: Vec<UioSegment>,
    pub offset: usize,
    pub resid: usize,
}

impl Uio {
    pub fn new(segments: Vec<UioSegment>) -> Self {
        let resid = segments.iter().map(|s| s.len).sum();
        Self {
            segments,
            offset: 0,
            resid,
        }
    }

    /// Read data from linear buffer into segments (scatter read)
    pub fn copy_from_buffer(&mut self, src: &[u8]) -> usize {
        let mut bytes_copied = 0;
        let mut src_offset = 0;

        for seg in &self.segments {
            if bytes_copied >= src.len() || self.resid == 0 {
                break;
            }

            let seg_ptr = seg.buffer;
            let seg_len = seg.len;

            let to_copy = std::cmp::min(seg_len, src.len() - src_offset);
            if to_copy > 0 {
                unsafe {
                    std::ptr::copy_nonoverlapping(src.as_ptr().add(src_offset), seg_ptr, to_copy);
                }
                src_offset += to_copy;
                bytes_copied += to_copy;
                self.resid = self.resid.saturating_sub(to_copy);
            }
        }
        self.offset += bytes_copied;
        bytes_copied
    }

    /// Write data from segments to linear buffer (gather write)
    pub fn copy_to_buffer(&self, dest: &mut [u8]) -> usize {
        let mut bytes_copied = 0;
        let mut dest_offset = 0;

        for seg in &self.segments {
            if bytes_copied >= dest.len() {
                break;
            }

            let seg_ptr = seg.buffer;
            let seg_len = seg.len;

            let to_copy = std::cmp::min(seg_len, dest.len() - dest_offset);
            if to_copy > 0 {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        seg_ptr,
                        dest.as_mut_ptr().add(dest_offset),
                        to_copy,
                    );
                }
                dest_offset += to_copy;
                bytes_copied += to_copy;
            }
        }
        bytes_copied
    }
}

// =========================================================
// 3. BSD-style kqueue event subscriber
// =========================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KqueueFilter {
    Read,
    Write,
    Complete,
}

#[derive(Debug, Clone, Copy)]
pub struct Kevent {
    pub ident: usize,
    pub filter: KqueueFilter,
    pub flags: u32,
    pub data: usize,
}

pub struct IrpKqueue {
    pub events: Vec<Kevent>,
}

impl IrpKqueue {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn register(&mut self, event: Kevent) {
        self.events.push(event);
    }

    pub fn trigger(&mut self, ident: usize, filter: KqueueFilter, data: usize) -> usize {
        let mut triggered_count = 0;
        for ev in &mut self.events {
            if ev.ident == ident && ev.filter == filter {
                ev.data = data;
                ev.flags |= 0x1; // EV_TRIGGERED
                triggered_count += 1;
            }
        }
        triggered_count
    }
}

// =========================================================
// 4. iOS/macOS Sandbox Security Entitlements
// =========================================================
#[derive(Debug, Clone)]
pub struct Entitlement {
    pub key: String,
    pub is_privileged: bool,
}

pub fn check_irp_entitlement(irp: &Irp, entitlements: &[Entitlement], required_key: &str) -> bool {
    if irp.major_function == IRP_MJ_DEVICE_CONTROL {
        for ent in entitlements {
            if ent.key == required_key && ent.is_privileged {
                return true;
            }
        }
        false
    } else {
        true
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
                        println!(
                            "ROOTKIT DETECTED: Driver object pointer mismatch for '{}'!",
                            driver.driver_name
                        );
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

    fn mock_completion_routine(_device: &DeviceObject, _irp: &mut Irp, context: usize) -> IoStatus {
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
            (|_dev: &DeviceObject, _irp: &mut Irp| IoStatus::Success)
                as fn(&DeviceObject, &mut Irp) -> IoStatus,
        );

        let driver = DriverObject {
            driver_name: "LayeredDiskDriver".to_string(),
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
            assert_eq!(std::ptr::addr_of!(COMPLETION_CONTEXT_VAL).read(), 1337);
        }
    }

    #[test]
    fn test_rootkit_irp_hook_detector() {
        let driver = DriverObject {
            driver_name: "TrustedFileDriver".to_string(),
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

        let mut detector = RootkitHookDetector::new();
        detector.register_verified_driver("TrustedFileDriver", &driver);

        // Trusted check should pass (no hook detected)
        assert!(!detector.audit_device_stack(&device));

        // Create malicious hooked driver mimicking same name
        let malicious_driver = DriverObject {
            driver_name: "TrustedFileDriver".to_string(),
            driver_extension: 0,
            dispatch_table: HashMap::new(),
            fast_io: None,
        };

        let compromised_device = DeviceObject {
            driver_object_ptr: &malicious_driver,
            device_extension: 0,
            flags: 0,
            power_state: AtomicU8::new(0),
        };

        // Detector must spot the pointer mismatch (untrusted driver hijacking slot)
        assert!(detector.audit_device_stack(&compromised_device));
    }

    // --- New Tests for Multi-OS Inspired Paradigms ---

    static mut MOCK_CANCEL_CALLED: bool = false;
    fn mock_cancel_routine(_irp: &mut Irp) {
        unsafe {
            MOCK_CANCEL_CALLED = true;
        }
    }

    #[test]
    fn test_irp_cancellation() {
        let mut irp = Irp::new(IRP_MJ_READ, METHOD_BUFFERED, 128);
        irp.set_cancel_routine(mock_cancel_routine);
        assert!(!irp.cancelled);

        irp.cancel();
        assert!(irp.cancelled);
        assert_eq!(irp.io_status.status, IoStatus::Cancelled);
        unsafe {
            assert!(MOCK_CANCEL_CALLED);
        }
    }

    fn mock_fast_read(_device: &DeviceObject, buffer: &mut [u8]) -> bool {
        if buffer.len() >= 4 {
            buffer[..4].copy_from_slice(b"FAST");
            true
        } else {
            false
        }
    }

    #[test]
    fn test_windows_fast_io() {
        let manager = IrpManager::new();
        let fast_io = FastIoDispatch {
            fast_io_read: Some(mock_fast_read),
            fast_io_write: None,
        };

        let driver = DriverObject {
            driver_name: "FastDisk".to_string(),
            driver_extension: 0,
            dispatch_table: HashMap::new(),
            fast_io: Some(fast_io),
        };

        let device = DeviceObject {
            driver_object_ptr: &driver,
            device_extension: 0,
            flags: 0,
            power_state: AtomicU8::new(0), // D0Active
        };

        let mut buf = [0u8; 8];
        assert!(manager.try_fast_io_read(&device, &mut buf));
        assert_eq!(&buf[..4], b"FAST");

        // If the device is in Sleep mode, fast I/O should fail immediately (iOS/macOS inspired)
        device.set_power_state(PowerState::D2Sleep);
        assert!(!manager.try_fast_io_read(&device, &mut buf));
    }

    #[test]
    fn test_linux_io_uring() {
        let manager = IrpManager::new();
        let mut uring = IoUring::new();

        let mut dispatch_table = HashMap::new();
        dispatch_table.insert(
            IRP_MJ_READ,
            (|_device: &DeviceObject, irp: &mut Irp| {
                irp.io_status.information = irp.buffer_length;
                IoStatus::Success
            }) as fn(&DeviceObject, &mut Irp) -> IoStatus,
        );

        let driver = DriverObject {
            driver_name: "RingDisk".to_string(),
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

        let mut data_buffer = [0u8; 256];
        let sqe = SubmissionQueueEntry {
            opcode: IORING_OP_READ,
            buffer: data_buffer.as_mut_ptr(),
            len: 128,
            io_control_code: 0,
            user_data: 4242,
        };

        uring.submit(sqe);
        assert_eq!(uring.sq.len(), 1);

        let processed = uring.poll_completions(&manager, &device);
        assert_eq!(processed, 1);
        assert_eq!(uring.cq.len(), 1);
        assert_eq!(uring.cq[0].user_data, 4242);
        assert_eq!(uring.cq[0].status, IoStatus::Success);
        assert_eq!(uring.cq[0].result, 128);
    }

    #[test]
    fn test_bsd_uio_scatter_gather() {
        let mut b1 = [0u8; 4];
        let mut b2 = [0u8; 6];

        let segs = vec![
            UioSegment {
                buffer: b1.as_mut_ptr(),
                len: 4,
            },
            UioSegment {
                buffer: b2.as_mut_ptr(),
                len: 6,
            },
        ];

        let mut uio = Uio::new(segs);
        assert_eq!(uio.resid, 10);

        let payload = b"HELLO_WORL"; // exactly 10 bytes
        let copied = uio.copy_from_buffer(payload);
        assert_eq!(copied, 10);
        assert_eq!(uio.resid, 0);

        // Check scatter target buffers
        assert_eq!(&b1, b"HELL");
        assert_eq!(&b2, b"O_WORL");

        // Gather test
        let mut gather_dest = [0u8; 10];
        let gathered = uio.copy_to_buffer(&mut gather_dest);
        assert_eq!(gathered, 10);
        assert_eq!(&gather_dest, b"HELLO_WORL");
    }

    #[test]
    fn test_bsd_kqueue() {
        let mut kq = IrpKqueue::new();
        kq.register(Kevent {
            ident: 101,
            filter: KqueueFilter::Complete,
            flags: 0,
            data: 0,
        });

        let triggered = kq.trigger(101, KqueueFilter::Complete, 999);
        assert_eq!(triggered, 1);
        assert_eq!(kq.events[0].data, 999);
        assert_eq!(kq.events[0].flags & 0x1, 1); // EV_TRIGGERED flag
    }

    #[test]
    fn test_ios_sandbox_entitlements() {
        let irp_read = Irp::new(IRP_MJ_READ, METHOD_BUFFERED, 128);
        let irp_ioctl = Irp::new(IRP_MJ_DEVICE_CONTROL, METHOD_BUFFERED, 128);

        let ents = vec![Entitlement {
            key: "com.apple.developer.driverkit.transport".to_string(),
            is_privileged: true,
        }];

        // Non-ioctl IRPs bypass entitlement check
        assert!(check_irp_entitlement(
            &irp_read,
            &ents,
            "com.apple.developer.driverkit.transport"
        ));

        // IOCTL requires specific entitlement key
        assert!(check_irp_entitlement(
            &irp_ioctl,
            &ents,
            "com.apple.developer.driverkit.transport"
        ));
        assert!(!check_irp_entitlement(
            &irp_ioctl,
            &ents,
            "com.apple.developer.driverkit.pci"
        ));
    }
}
