
#[cfg(not(target_os = "none"))]
extern crate std;

use core::mem;
/// ReactOS-inspired Windows NT Subsystem Compatibility Layer for SigmaOS
/// Provides Portable Executable (PE) parsing, NT Registry Hive management,
/// and NT Object Manager handle tables.
use core::sync::atomic::{AtomicUsize, Ordering};

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NtStatus {
    Success = 0x00000000,
    InvalidHandle = 0xC0000008,
    ObjectNameNotFound = 0xC0000034,
    InvalidImageFormat = 0xC000007B,
    AccessDenied = 0xC0000022,
    WaitTimeout = 0x00000102,
}

pub type NtHandle = usize;

/// Standard NT Object Type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NtObjectType {
    File = 1,
    Process = 2,
    Thread = 3,
    Key = 4,
    Event = 5,
    Mutant = 6,
    Semaphore = 7,
}

/// Entry representing an allocated NT handle
#[derive(Debug)]
pub struct NtHandleEntry {
    pub handle: NtHandle,
    pub object_type: NtObjectType,
    pub name: [u8; 32],
}

impl NtHandleEntry {
    pub fn new(handle: NtHandle, object_type: NtObjectType, name: &[u8]) -> Self {
        let mut name_array = [0u8; 32];
        let len = name.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), len);
        }
        NtHandleEntry {
            handle,
            object_type,
            name: name_array,
        }
    }
}

/// Windows NT Object Manager
pub struct NtObjectManager {
    pub handles: Vec<Option<NtHandleEntry>>,
    pub next_handle: AtomicUsize,
}

impl NtObjectManager {
    pub fn new() -> Self {
        NtObjectManager {
            handles: Vec::new(),
            next_handle: AtomicUsize::new(0x10), // Handle values typically start at 0x10
        }
    }

    /// Allocate and register a new NT handle (NtCreateFile/NtCreateProcess equivalent)
    pub fn create_object(&mut self, object_type: NtObjectType, name: &[u8]) -> NtHandle {
        let handle = self.next_handle.fetch_add(4, Ordering::SeqCst); // Handles typically increment by 4
        let entry = NtHandleEntry::new(handle, object_type, name);
        self.handles.push(Some(entry));
        handle
    }

    /// Retrieve an object entry from a handle (NtQueryObject equivalent)
    pub fn lookup_object(&self, handle: NtHandle) -> Result<&NtHandleEntry, NtStatus> {
        for i in 0..self.handles.len {
            if let Some(ref entry) = self.handles[i] {
                if entry.handle == handle {
                    return Ok(entry);
                }
            }
        }
        Err(NtStatus::InvalidHandle)
    }

    /// Close handle (NtClose equivalent)
    pub fn close_handle(&mut self, handle: NtHandle) -> Result<(), NtStatus> {
        for i in 0..self.handles.len {
            if let Some(ref entry) = self.handles[i] {
                if entry.handle == handle {
                    self.handles[i] = None;
                    return Ok(());
                }
            }
        }
        Err(NtStatus::InvalidHandle)
    }
}

// ==========================================
// NT Virtual Memory Subsystem
// ==========================================

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageProtection {
    NoAccess = 0x01,
    ReadOnly = 0x02,
    ReadWrite = 0x04,
    ExecuteRead = 0x20,
    ExecuteReadWrite = 0x40,
}

#[derive(Debug, Clone)]
pub struct VirtualAllocation {
    pub base_address: usize,
    pub size: usize,
    pub protection: PageProtection,
}

pub struct NtVirtualMemoryManager {
    pub allocations: Vec<Option<VirtualAllocation>>,
    pub next_free_address: usize,
}

impl NtVirtualMemoryManager {
    pub fn new() -> Self {
        NtVirtualMemoryManager {
            allocations: Vec::new(),
            next_free_address: 0x00400000, // Standard User space address start
        }
    }

    /// NtAllocateVirtualMemory equivalent
    pub fn allocate_virtual_memory(
        &mut self,
        size: usize,
        protection: PageProtection,
    ) -> Result<usize, NtStatus> {
        let addr = self.next_free_address;
        self.next_free_address += (size + 4095) & !4095; // Align to 4KB page boundary

        let allocation = VirtualAllocation {
            base_address: addr,
            size,
            protection,
        };
        self.allocations.push(Some(allocation));
        Ok(addr)
    }

    /// NtFreeVirtualMemory equivalent
    pub fn free_virtual_memory(&mut self, base_address: usize) -> Result<(), NtStatus> {
        for i in 0..self.allocations.len {
            if let Some(ref alloc) = self.allocations[i] {
                if alloc.base_address == base_address {
                    self.allocations[i] = None;
                    return Ok(());
                }
            }
        }
        Err(NtStatus::ObjectNameNotFound)
    }

    /// NtProtectVirtualMemory equivalent
    pub fn protect_virtual_memory(
        &mut self,
        base_address: usize,
        new_protection: PageProtection,
    ) -> Result<PageProtection, NtStatus> {
        for i in 0..self.allocations.len {
            if let Some(ref mut alloc) = self.allocations[i] {
                if alloc.base_address == base_address {
                    let old_protection = alloc.protection;
                    alloc.protection = new_protection;
                    return Ok(old_protection);
                }
            }
        }
        Err(NtStatus::ObjectNameNotFound)
    }
}

// ==========================================
// NT Kernel Synchronization Primitives & Wait Dispatcher
// ==========================================

#[derive(Debug, Clone)]
pub struct NtEventObject {
    pub handle: NtHandle,
    pub signaled: bool,
    pub manual_reset: bool,
}

#[derive(Debug, Clone)]
pub struct NtMutantObject {
    pub handle: NtHandle,
    pub count: i32,
    pub owner_thread_id: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct NtSemaphoreObject {
    pub handle: NtHandle,
    pub count: i32,
    pub limit: i32,
}

pub struct NtSyncManager {
    pub events: Vec<Option<NtEventObject>>,
    pub mutants: Vec<Option<NtMutantObject>>,
    pub semaphores: Vec<Option<NtSemaphoreObject>>,
}

impl NtSyncManager {
    pub fn new() -> Self {
        NtSyncManager {
            events: Vec::new(),
            mutants: Vec::new(),
            semaphores: Vec::new(),
        }
    }

    pub fn create_event(&mut self, handle: NtHandle, manual_reset: bool, initial_state: bool) {
        let event = NtEventObject {
            handle,
            signaled: initial_state,
            manual_reset,
        };
        self.events.push(Some(event));
    }

    pub fn create_mutant(&mut self, handle: NtHandle, initial_owner: bool) {
        let mutant = NtMutantObject {
            handle,
            count: if initial_owner { 1 } else { 0 },
            owner_thread_id: if initial_owner { Some(0x100) } else { None },
        };
        self.mutants.push(Some(mutant));
    }

    pub fn create_semaphore(&mut self, handle: NtHandle, initial_count: i32, limit: i32) {
        let sem = NtSemaphoreObject {
            handle,
            count: initial_count,
            limit,
        };
        self.semaphores.push(Some(sem));
    }

    /// NtWaitForSingleObject equivalent
    pub fn wait_for_single_object(&mut self, handle: NtHandle, timeout_ms: usize) -> NtStatus {
        if timeout_ms == 0 {
            return NtStatus::WaitTimeout;
        }

        // Search in events
        for i in 0..self.events.len {
            if let Some(ref mut ev) = self.events[i] {
                if ev.handle == handle {
                    if ev.signaled {
                        if !ev.manual_reset {
                            ev.signaled = false;
                        }
                        return NtStatus::Success;
                    } else {
                        return NtStatus::WaitTimeout;
                    }
                }
            }
        }

        // Search in mutants
        for i in 0..self.mutants.len {
            if let Some(ref mut mut_obj) = self.mutants[i] {
                if mut_obj.handle == handle {
                    if mut_obj.count == 0 {
                        mut_obj.count = 1;
                        mut_obj.owner_thread_id = Some(0x100);
                        return NtStatus::Success;
                    } else {
                        return NtStatus::WaitTimeout;
                    }
                }
            }
        }

        // Search in semaphores
        for i in 0..self.semaphores.len {
            if let Some(ref mut sem) = self.semaphores[i] {
                if sem.handle == handle {
                    if sem.count > 0 {
                        sem.count -= 1;
                        return NtStatus::Success;
                    } else {
                        return NtStatus::WaitTimeout;
                    }
                }
            }
        }

        NtStatus::InvalidHandle
    }
}

// ==========================================
// NT Process and Thread Emulation
// ==========================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Peb {
    pub inherited_address_space: u8,
    pub read_image_file_exec_options: u8,
    pub being_debugged: u8,
    pub image_base_address: usize,
    pub loader_data: usize,
    pub process_parameters: usize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Teb {
    pub stack_base: usize,
    pub stack_limit: usize,
    pub active_rpc_handle: usize,
    pub thread_local_storage_pointer: usize,
    pub peb_pointer: usize,
    pub real_client_id: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct ThreadContext {
    pub rip: usize,
    pub rsp: usize,
    pub rbp: usize,
    pub rflags: usize,
    pub rax: usize,
}

pub struct NtProcess {
    pub pid: usize,
    pub peb: Peb,
}

pub struct NtThread {
    pub tid: usize,
    pub teb: Teb,
    pub context: ThreadContext,
    pub priority: u8,
    pub state: usize, // 0: Ready, 1: Running, 2: Waiting, 3: Terminated
}

// ==========================================
// NT I/O Request Packet (IRP) Subsystem
// ==========================================

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrpMajorFunction {
    Create = 0x00,
    Close = 0x02,
    Read = 0x03,
    Write = 0x04,
    DeviceControl = 0x0e,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IoStatusBlock {
    pub status: NtStatus,
    pub information: usize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrpStackLocation {
    pub major_function: IrpMajorFunction,
    pub minor_function: u8,
    pub flags: u8,
    pub parameters_read_length: usize,
    pub parameters_write_length: usize,
    pub parameters_ioctl_code: u32,
}

pub struct Irp {
    pub io_status: IoStatusBlock,
    pub current_stack_location: IrpStackLocation,
    pub user_buffer: *mut u8,
    pub user_buffer_len: usize,
}

pub struct NtDriver {
    pub driver_name: [u8; 32],
    pub dispatch_create: Option<fn(&mut Irp) -> NtStatus>,
    pub dispatch_close: Option<fn(&mut Irp) -> NtStatus>,
    pub dispatch_read: Option<fn(&mut Irp) -> NtStatus>,
    pub dispatch_write: Option<fn(&mut Irp) -> NtStatus>,
    pub dispatch_device_control: Option<fn(&mut Irp) -> NtStatus>,
}

impl NtDriver {
    pub fn new(name: &[u8]) -> Self {
        let mut driver_name = [0u8; 32];
        let len = name.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), driver_name.as_mut_ptr(), len);
        }

        NtDriver {
            driver_name,
            dispatch_create: None,
            dispatch_close: None,
            dispatch_read: None,
            dispatch_write: None,
            dispatch_device_control: None,
        }
    }

    /// Direct dispatch entry routing
    pub fn dispatch_irp(&self, irp: &mut Irp) -> NtStatus {
        match irp.current_stack_location.major_function {
            IrpMajorFunction::Create => {
                if let Some(dispatch) = self.dispatch_create {
                    dispatch(irp)
                } else {
                    NtStatus::Success
                }
            }
            IrpMajorFunction::Close => {
                if let Some(dispatch) = self.dispatch_close {
                    dispatch(irp)
                } else {
                    NtStatus::Success
                }
            }
            IrpMajorFunction::Read => {
                if let Some(dispatch) = self.dispatch_read {
                    dispatch(irp)
                } else {
                    NtStatus::Success
                }
            }
            IrpMajorFunction::Write => {
                if let Some(dispatch) = self.dispatch_write {
                    dispatch(irp)
                } else {
                    NtStatus::Success
                }
            }
            IrpMajorFunction::DeviceControl => {
                if let Some(dispatch) = self.dispatch_device_control {
                    dispatch(irp)
                } else {
                    NtStatus::Success
                }
            }
        }
    }
}

// ==========================================
// Win32 API Emulation Framework (DLL wrapper)
// ==========================================

pub struct Win32DllSubsystem {
    pub object_manager: NtObjectManager,
    pub vm_manager: NtVirtualMemoryManager,
    pub sync_manager: NtSyncManager,
}

impl Win32DllSubsystem {
    pub fn new() -> Self {
        Win32DllSubsystem {
            object_manager: NtObjectManager::new(),
            vm_manager: NtVirtualMemoryManager::new(),
            sync_manager: NtSyncManager::new(),
        }
    }

    /// kernel32.dll -> VirtualAlloc
    pub fn win32_virtual_alloc(
        &mut self,
        size: usize,
        protection: PageProtection,
    ) -> Result<usize, NtStatus> {
        self.vm_manager.allocate_virtual_memory(size, protection)
    }

    /// kernel32.dll -> CreateFileA
    pub fn win32_create_file_a(&mut self, filename: &[u8]) -> NtHandle {
        self.object_manager
            .create_object(NtObjectType::File, filename)
    }

    /// kernel32.dll -> CloseHandle
    pub fn win32_close_handle(&mut self, handle: NtHandle) -> Result<(), NtStatus> {
        self.object_manager.close_handle(handle)
    }

    /// user32.dll -> CreateWindowExA
    pub fn win32_create_window_ex_a(&mut self, window_name: &[u8]) -> NtHandle {
        self.object_manager
            .create_object(NtObjectType::Event, window_name)
    }
}

// ==========================================
// Representation of a Windows Registry Value
// ==========================================

pub struct RegistryValue {
    pub name: [u8; 32],
    pub data: [u8; 64],
    pub data_len: usize,
}

impl RegistryValue {
    pub fn new(name: &[u8], data: &[u8]) -> Self {
        let mut name_array = [0u8; 32];
        let mut data_array = [0u8; 64];
        let name_len = name.len().min(31);
        let data_len = data.len().min(63);

        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
            core::ptr::copy_nonoverlapping(data.as_ptr(), data_array.as_mut_ptr(), data_len);
        }

        RegistryValue {
            name: name_array,
            data: data_array,
            data_len,
        }
    }
}

/// Windows NT Registry Hive System (HKLM/HKCU configuration database)
pub struct RegistryHive {
    pub keys: Vec<Option<[u8; 32]>>,
    pub values: Vec<Option<RegistryValue>>,
}

impl RegistryHive {
    pub fn new() -> Self {
        RegistryHive {
            keys: Vec::new(),
            values: Vec::new(),
        }
    }

    /// Create registry key (NtCreateKey equivalent)
    pub fn create_key(&mut self, key_name: &[u8]) {
        let mut key_array = [0u8; 32];
        let len = key_name.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(key_name.as_ptr(), key_array.as_mut_ptr(), len);
        }
        self.keys.push(Some(key_array));
    }

    /// Set registry value (NtSetValueKey equivalent)
    pub fn set_value(&mut self, name: &[u8], data: &[u8]) {
        let value = RegistryValue::new(name, data);
        self.values.push(Some(value));
    }

    /// Retrieve registry value (NtQueryValueKey equivalent)
    pub fn query_value(&self, name: &[u8]) -> Result<&RegistryValue, NtStatus> {
        for i in 0..self.values.len {
            if let Some(ref val) = self.values[i] {
                let val_name_len = val.name.iter().position(|&b| b == 0).unwrap_or(32);
                if &val.name[..val_name_len] == name {
                    return Ok(val);
                }
            }
        }
        Err(NtStatus::ObjectNameNotFound)
    }
}

// ==========================================
// Windows Portable Executable (PE) Loader
// ==========================================

pub struct PortableExecutableLoader;

impl PortableExecutableLoader {
    /// Validates MZ DOS stub and PE signature headers for loading Windows executable/driver binaries
    pub fn validate_pe_image(binary: &[u8]) -> Result<(), NtStatus> {
        if binary.len() < 64 {
            return Err(NtStatus::InvalidImageFormat);
        }

        // Validate MZ header ('M' and 'Z')
        if binary[0] != b'M' || binary[1] != b'Z' {
            return Err(NtStatus::InvalidImageFormat);
        }

        // Extract PE header offset from e_lfanew field (at 0x3C)
        let pe_offset = (binary[0x3C] as usize)
            | ((binary[0x3D] as usize) << 8)
            | ((binary[0x3E] as usize) << 16)
            | ((binary[0x3F] as usize) << 24);

        if pe_offset + 4 > binary.len() {
            return Err(NtStatus::InvalidImageFormat);
        }

        // Validate PE signature ('P', 'E', 0, 0)
        if binary[pe_offset] != b'P'
            || binary[pe_offset + 1] != b'E'
            || binary[pe_offset + 2] != 0
            || binary[pe_offset + 3] != 0
        {
            return Err(NtStatus::InvalidImageFormat);
        }

        Ok(())
    }
}

// ==========================================
// Custom Zero-Dependency Vector Collection
// ==========================================

pub struct Vec<T> {
    pub data: *mut T,
    pub len: usize,
    pub capacity: usize,
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

impl<T> core::ops::Index<usize> for Vec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &*self.data.add(index) }
    }
}

impl<T> core::ops::IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &mut *self.data.add(index) }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 {
            unsafe {
                for i in 0..self.len {
                    core::ptr::drop_in_place(self.data.add(i));
                }
                free(self.data as *mut u8);
            }
        }
    }
}

#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    extern crate std;
    use std::alloc::{alloc as std_alloc, Layout};
    let layout = Layout::from_size_align(size, 8).unwrap();
    std_alloc(layout)
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8) {
    let _ = ptr;
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

// =========================================================================
// NT KERNEL APIS & ADVANCED ARCHITECTURE COMPATIBILITY EXTENSIONS
// =========================================================================

/// 1. DeviceIoControl & IRP Dispatch Routing
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoctlMethod {
    Buffered,
    InDirect,
    OutDirect,
    Neither,
}

pub struct DeviceIoControlEngine;

impl DeviceIoControlEngine {
    pub fn send_device_io_control(
        driver: &NtDriver,
        ioctl_code: u32,
        _in_buffer: *mut u8,
        in_len: usize,
        out_buffer: *mut u8,
        out_len: usize,
    ) -> NtStatus {
        let mut irp = Irp {
            io_status: IoStatusBlock {
                status: NtStatus::Success,
                information: out_len,
            },
            current_stack_location: IrpStackLocation {
                major_function: IrpMajorFunction::DeviceControl,
                minor_function: 0,
                flags: 0,
                parameters_read_length: in_len,
                parameters_write_length: out_len,
                parameters_ioctl_code: ioctl_code,
            },
            user_buffer: out_buffer,
            user_buffer_len: out_len,
        };
        driver.dispatch_irp(&mut irp)
    }
}

/// 2. KeInitializeApc (Kernel Asynchronous Procedure Call)
#[derive(Debug, Clone, Copy)]
pub struct KApc {
    pub target_thread_id: usize,
    pub kernel_routine: Option<fn(usize)>,
    pub normal_routine: Option<fn(usize)>,
    pub is_initialized: bool,
}

pub struct KeApcManager;

impl KeApcManager {
    pub fn ke_initialize_apc(
        apc: &mut KApc,
        thread_id: usize,
        kernel_routine: Option<fn(usize)>,
        normal_routine: Option<fn(usize)>,
    ) {
        apc.target_thread_id = thread_id;
        apc.kernel_routine = kernel_routine;
        apc.normal_routine = normal_routine;
        apc.is_initialized = true;
    }
}

/// 3. MmGetPhysicalAddress (Virtual-to-Physical Address Translation)
pub struct MmPhysicalMemoryTranslator;

impl MmPhysicalMemoryTranslator {
    pub fn mm_get_physical_address(virtual_addr: usize) -> u64 {
        // High-memory physical mapping offset simulation
        if virtual_addr >= 0x00400000 {
            (virtual_addr as u64) & 0x0FFF_FFFF
        } else {
            virtual_addr as u64
        }
    }
}

/// 4. PsCreateSystemThread (Kernel System Thread Creation)
pub struct PsThreadManager;

impl PsThreadManager {
    pub fn ps_create_system_thread(
        target_process_id: usize,
        start_routine: fn(usize),
        context_param: usize,
    ) -> NtThread {
        NtThread {
            tid: 0x200 + target_process_id,
            teb: Teb {
                stack_base: 0x00800000,
                stack_limit: 0x00700000,
                active_rpc_handle: 0,
                thread_local_storage_pointer: 0,
                peb_pointer: 0,
                real_client_id: target_process_id,
            },
            context: ThreadContext {
                rip: start_routine as usize,
                rsp: 0x00800000,
                rbp: 0x00800000,
                rflags: 0x200,
                rax: context_param,
            },
            priority: 8,
            state: 0, // Ready state
        }
    }
}

/// 5. WinDbg Extension SDK Subsystem (`!process`, `!thread`, `!devobj`, `!pool`)
pub struct WinDbgExtensionSdk;

impl WinDbgExtensionSdk {
    pub fn inspect_process_ext(proc: &NtProcess) -> [u8; 64] {
        let mut buffer = [0u8; 64];
        buffer[0] = b'P';
        buffer[1] = b'R';
        buffer[2] = b'O';
        buffer[3] = b'C';
        buffer[4] = (proc.pid & 0xFF) as u8;
        buffer
    }

    pub fn inspect_thread_ext(thread: &NtThread) -> [u8; 64] {
        let mut buffer = [0u8; 64];
        buffer[0] = b'T';
        buffer[1] = b'H';
        buffer[2] = b'R';
        buffer[3] = b'D';
        buffer[4] = (thread.tid & 0xFF) as u8;
        buffer
    }
}

/// 6. Application Program Status Register (APSR) Model
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApsrRegister {
    pub negative: bool,   // N flag
    pub zero: bool,       // Z flag
    pub carry: bool,      // C flag
    pub overflow: bool,   // V flag
    pub saturation: bool, // Q flag
}

impl ApsrRegister {
    pub fn new() -> Self {
        Self {
            negative: false,
            zero: true,
            carry: false,
            overflow: false,
            saturation: false,
        }
    }

    pub fn update_flags(&mut self, result: i64) {
        self.zero = result == 0;
        self.negative = result < 0;
    }
}

/// 7. Approximation Partial Ordering Scheduler (BSD DAG Inspired)
pub struct ApproximationPartialOrdering {
    pub task_dependencies: [(usize, usize); 8], // (dependent_task, prerequisite_task)
    pub count: usize,
}

impl ApproximationPartialOrdering {
    pub fn new() -> Self {
        Self {
            task_dependencies: [(0, 0); 8],
            count: 0,
        }
    }

    pub fn add_dependency(&mut self, dependent: usize, prereq: usize) -> Result<(), &'static str> {
        if self.count >= 8 {
            return Err("DAG Dependency Table Full");
        }
        self.task_dependencies[self.count] = (dependent, prereq);
        self.count += 1;
        Ok(())
    }

    pub fn is_executable(&self, task_id: usize, completed_tasks: &[usize]) -> bool {
        for i in 0..self.count {
            let (dep, prereq) = self.task_dependencies[i];
            if dep == task_id {
                let mut prereq_done = false;
                for &done in completed_tasks {
                    if done == prereq {
                        prereq_done = true;
                        break;
                    }
                }
                if !prereq_done {
                    return false;
                }
            }
        }
        true
    }
}

/// 8. Arbitrary Kernel Context Switcher
pub struct ArbitraryKernelContext {
    pub active_cr3_page_dir: u64,
}

impl ArbitraryKernelContext {
    pub fn new(page_dir: u64) -> Self {
        Self {
            active_cr3_page_dir: page_dir,
        }
    }

    pub fn switch_context(&mut self, new_page_dir: u64) -> u64 {
        let old = self.active_cr3_page_dir;
        self.active_cr3_page_dir = new_page_dir;
        old
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nt_object_manager_handles() {
        let mut manager = NtObjectManager::new();
        let h1 = manager.create_object(NtObjectType::File, b"DeviceKeyboard");
        let h2 = manager.create_object(NtObjectType::Process, b"explorer.exe");

        assert_eq!(h1, 0x10);
        assert_eq!(h2, 0x14);

        let entry = manager.lookup_object(h1).unwrap();
        assert_eq!(entry.object_type, NtObjectType::File);

        let mut entry_name = [0u8; 14];
        for i in 0..14 {
            entry_name[i] = entry.name[i];
        }
        assert_eq!(&entry_name, b"DeviceKeyboard");

        assert!(manager.close_handle(h1).is_ok());
        assert_eq!(
            manager.lookup_object(h1).unwrap_err() as usize,
            NtStatus::InvalidHandle as usize
        );
    }

    #[test]
    fn test_registry_hive_queries() {
        let mut hive = RegistryHive::new();
        hive.create_key(b"SOFTWARE\\SigmaOS");
        hive.set_value(b"Theme", b"SovereignDark");

        let val = hive.query_value(b"Theme").unwrap();
        assert_eq!(val.data_len, 13);

        let mut val_data = [0u8; 13];
        for i in 0..13 {
            val_data[i] = val.data[i];
        }
        assert_eq!(&val_data, b"SovereignDark");
    }

    #[test]
    fn test_portable_executable_parsing() {
        // Construct a mock minimal valid Windows PE image buffer
        let mut pe_binary = [0u8; 128];
        pe_binary[0] = b'M';
        pe_binary[1] = b'Z';

        // e_lfanew offset field at 0x3C points to PE header location (0x40)
        pe_binary[0x3C] = 0x40;

        // Write standard PE signature at 0x40: 'P', 'E', 0, 0
        pe_binary[0x40] = b'P';
        pe_binary[0x41] = b'E';
        pe_binary[0x42] = 0;
        pe_binary[0x43] = 0;

        assert!(PortableExecutableLoader::validate_pe_image(&pe_binary).is_ok());

        // Invalid MZ signature
        let mut invalid_pe = pe_binary;
        invalid_pe[0] = b'X';
        assert_eq!(
            PortableExecutableLoader::validate_pe_image(&invalid_pe).unwrap_err() as usize,
            NtStatus::InvalidImageFormat as usize
        );
    }

    #[test]
    fn test_nt_virtual_memory_apis() {
        let mut vmm = NtVirtualMemoryManager::new();
        let size = 8192; // 2 pages
        let addr = vmm
            .allocate_virtual_memory(size, PageProtection::ReadWrite)
            .unwrap();
        assert_eq!(addr, 0x00400000);

        let old_prot = vmm
            .protect_virtual_memory(addr, PageProtection::ExecuteRead)
            .unwrap();
        assert_eq!(old_prot, PageProtection::ReadWrite);

        assert!(vmm.free_virtual_memory(addr).is_ok());
    }

    #[test]
    fn test_nt_sync_primitives() {
        let mut sync = NtSyncManager::new();
        let handle = 0x20;

        sync.create_event(handle, false, true);
        assert_eq!(sync.wait_for_single_object(handle, 100), NtStatus::Success);
        // Auto-reset check
        assert_eq!(
            sync.wait_for_single_object(handle, 100),
            NtStatus::WaitTimeout
        );

        sync.create_semaphore(handle + 4, 3, 5);
        assert_eq!(
            sync.wait_for_single_object(handle + 4, 100),
            NtStatus::Success
        );
    }

    #[test]
    fn test_nt_irp_subsystem() {
        let mut driver = NtDriver::new(b"SigmaDiskDriver");
        fn mock_read_irp(irp: &mut Irp) -> NtStatus {
            irp.io_status.status = NtStatus::Success;
            irp.io_status.information = 512;
            NtStatus::Success
        }
        driver.dispatch_read = Some(mock_read_irp);

        let mut irp = Irp {
            io_status: IoStatusBlock {
                status: NtStatus::AccessDenied,
                information: 0,
            },
            current_stack_location: IrpStackLocation {
                major_function: IrpMajorFunction::Read,
                minor_function: 0,
                flags: 0,
                parameters_read_length: 512,
                parameters_write_length: 0,
                parameters_ioctl_code: 0,
            },
            user_buffer: core::ptr::null_mut(),
            user_buffer_len: 0,
        };

        let status = driver.dispatch_irp(&mut irp);
        assert_eq!(status, NtStatus::Success);
        assert_eq!(irp.io_status.status, NtStatus::Success);
        assert_eq!(irp.io_status.information, 512);
    }

    #[test]
    fn test_win32_dll_subsystem() {
        let mut win32 = Win32DllSubsystem::new();
        let h_file = win32.win32_create_file_a(b"C:\\sigma_config.ini");
        assert_eq!(h_file, 0x10);

        let addr = win32
            .win32_virtual_alloc(4096, PageProtection::ReadWrite)
            .unwrap();
        assert_eq!(addr, 0x00400000);

        assert!(win32.win32_close_handle(h_file).is_ok());
    }

    #[test]
    fn test_device_io_control_routing() {
        let mut driver = NtDriver::new(b"StorageFilter");
        fn mock_ioctl_dispatch(irp: &mut Irp) -> NtStatus {
            irp.io_status.information = 1024;
            NtStatus::Success
        }
        driver.dispatch_device_control = Some(mock_ioctl_dispatch);

        let status = DeviceIoControlEngine::send_device_io_control(
            &driver,
            0x222000, // IOCTL code
            core::ptr::null_mut(),
            0,
            core::ptr::null_mut(),
            1024,
        );
        assert_eq!(status, NtStatus::Success);
    }

    #[test]
    fn test_ke_initialize_apc() {
        let mut apc = KApc {
            target_thread_id: 0,
            kernel_routine: None,
            normal_routine: None,
            is_initialized: false,
        };
        fn kernel_sub_apc(_param: usize) {}
        KeApcManager::ke_initialize_apc(&mut apc, 0x100, Some(kernel_sub_apc), None);
        assert!(apc.is_initialized);
        assert_eq!(apc.target_thread_id, 0x100);
    }

    #[test]
    fn test_mm_get_physical_address() {
        let phys1 = MmPhysicalMemoryTranslator::mm_get_physical_address(0x00400000);
        assert_eq!(phys1, 0x00400000);

        let phys2 = MmPhysicalMemoryTranslator::mm_get_physical_address(0x10400000);
        assert_eq!(phys2, 0x00400000); // Mapped mask simulation
    }

    #[test]
    fn test_ps_create_system_thread() {
        fn mock_system_thread(_context: usize) {}
        let thread = PsThreadManager::ps_create_system_thread(0x10, mock_system_thread, 0xAA);
        assert_eq!(thread.tid, 0x210);
        assert_eq!(thread.context.rax, 0xAA);
        assert_eq!(thread.state, 0); // Ready
    }

    #[test]
    fn test_windbg_extension_sdk() {
        let proc = NtProcess {
            pid: 42,
            peb: Peb {
                inherited_address_space: 0,
                read_image_file_exec_options: 0,
                being_debugged: 0,
                image_base_address: 0x400000,
                loader_data: 0,
                process_parameters: 0,
            },
        };
        let buf = WinDbgExtensionSdk::inspect_process_ext(&proc);
        assert_eq!(&buf[..4], b"PROC");
        assert_eq!(buf[4], 42);
    }

    #[test]
    fn test_apsr_register_flags() {
        let mut apsr = ApsrRegister::new();
        assert!(apsr.zero);
        apsr.update_flags(-5);
        assert!(!apsr.zero);
        assert!(apsr.negative);
    }

    #[test]
    fn test_approximation_partial_ordering_dag() {
        let mut po = ApproximationPartialOrdering::new();
        assert!(po.add_dependency(2, 1).is_ok()); // Task 2 depends on Task 1

        assert!(!po.is_executable(2, &[])); // Prereq 1 not completed
        assert!(po.is_executable(2, &[1])); // Prereq 1 completed!
    }

    #[test]
    fn test_arbitrary_kernel_context_switch() {
        let mut ctx = ArbitraryKernelContext::new(0x1000);
        let old_cr3 = ctx.switch_context(0x2000);
        assert_eq!(old_cr3, 0x1000);
        assert_eq!(ctx.active_cr3_page_dir, 0x2000);
    }
}
