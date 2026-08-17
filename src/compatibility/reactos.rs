// SigmaOS Windows NT & ReactOS Competitor Parity Subsystem
// Independent, zero-dependency implementations of NT kernel and Win32 GDI subsystems

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use alloc::string::String;
use alloc::string::ToString;
use alloc::format;
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NtHandleEntry {
    pub handle: NtHandle,
    pub object_type: NtObjectType,
    pub name: String,
}

impl NtHandleEntry {
    pub fn new(handle: NtHandle, object_type: NtObjectType, name: &str) -> Self {
        NtHandleEntry {
            handle,
            object_type,
            name: name.to_string(),
        }
    }
}

/// Windows NT Object Manager
pub struct NtObjectManager {
    pub handles: BTreeMap<NtHandle, NtHandleEntry>,
    pub next_handle: AtomicUsize,
}

impl NtObjectManager {
    pub fn new() -> Self {
        NtObjectManager {
            handles: BTreeMap::new(),
            next_handle: AtomicUsize::new(0x10), // Handle values typically start at 0x10
        }
    }

    /// Allocate and register a new NT handle (NtCreateFile/NtCreateProcess equivalent)
    pub fn create_object(&mut self, object_type: NtObjectType, name: &str) -> NtHandle {
        let handle = self.next_handle.fetch_add(4, Ordering::SeqCst); // Handles typically increment by 4
        let entry = NtHandleEntry::new(handle, object_type, name);
        self.handles.insert(handle, entry);
        handle
    }

    /// Retrieve an object entry from a handle (NtQueryObject equivalent)
    pub fn lookup_object(&self, handle: NtHandle) -> Result<&NtHandleEntry, NtStatus> {
        self.handles.get(&handle).ok_or(NtStatus::InvalidHandle)
    }

    /// Close handle (NtClose equivalent)
    pub fn close_handle(&mut self, handle: NtHandle) -> Result<(), NtStatus> {
        self.handles.remove(&handle).map(|_| ()).ok_or(NtStatus::InvalidHandle)
    }
}

impl Default for NtObjectManager {
    fn default() -> Self {
        Self::new()
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
    pub allocations: BTreeMap<usize, VirtualAllocation>,
    pub next_free_address: usize,
}

impl NtVirtualMemoryManager {
    pub fn new() -> Self {
        NtVirtualMemoryManager {
            allocations: BTreeMap::new(),
            next_free_address: 0x00400000, // Standard User space address start
        }
    }

    /// NtAllocateVirtualMemory equivalent
    pub fn allocate_virtual_memory(&mut self, size: usize, protection: PageProtection) -> Result<usize, NtStatus> {
        let addr = self.next_free_address;
        self.next_free_address += (size + 4095) & !4095; // Align to 4KB page boundary

        let allocation = VirtualAllocation {
            base_address: addr,
            size,
            protection,
        };
        self.allocations.insert(addr, allocation);
        Ok(addr)
    }

    /// NtFreeVirtualMemory equivalent
    pub fn free_virtual_memory(&mut self, base_address: usize) -> Result<(), NtStatus> {
        self.allocations.remove(&base_address).map(|_| ()).ok_or(NtStatus::ObjectNameNotFound)
    }

    /// NtProtectVirtualMemory equivalent
    pub fn protect_virtual_memory(&mut self, base_address: usize, new_protection: PageProtection) -> Result<PageProtection, NtStatus> {
        if let Some(alloc) = self.allocations.get_mut(&base_address) {
            let old_protection = alloc.protection;
            alloc.protection = new_protection;
            Ok(old_protection)
        } else {
            Err(NtStatus::ObjectNameNotFound)
        }
    }
}

impl Default for NtVirtualMemoryManager {
    fn default() -> Self {
        Self::new()
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
    pub events: BTreeMap<NtHandle, NtEventObject>,
    pub mutants: BTreeMap<NtHandle, NtMutantObject>,
    pub semaphores: BTreeMap<NtHandle, NtSemaphoreObject>,
}

impl NtSyncManager {
    pub fn new() -> Self {
        NtSyncManager {
            events: BTreeMap::new(),
            mutants: BTreeMap::new(),
            semaphores: BTreeMap::new(),
        }
    }

    pub fn create_event(&mut self, handle: NtHandle, manual_reset: bool, initial_state: bool) {
        let event = NtEventObject {
            handle,
            signaled: initial_state,
            manual_reset,
        };
        self.events.insert(handle, event);
    }

    pub fn create_mutant(&mut self, handle: NtHandle, initial_owner: bool) {
        let mutant = NtMutantObject {
            handle,
            count: if initial_owner { 1 } else { 0 },
            owner_thread_id: if initial_owner { Some(0x100) } else { None },
        };
        self.mutants.insert(handle, mutant);
    }

    pub fn create_semaphore(&mut self, handle: NtHandle, initial_count: i32, limit: i32) {
        let sem = NtSemaphoreObject {
            handle,
            count: initial_count,
            limit,
        };
        self.semaphores.insert(handle, sem);
    }

    /// NtWaitForSingleObject equivalent
    pub fn wait_for_single_object(&mut self, handle: NtHandle, timeout_ms: usize) -> NtStatus {
        if timeout_ms == 0 {
            return NtStatus::WaitTimeout;
        }

        // Search in events
        if let Some(ev) = self.events.get_mut(&handle) {
            if ev.signaled {
                if !ev.manual_reset {
                    ev.signaled = false;
                }
                return NtStatus::Success;
            } else {
                return NtStatus::WaitTimeout;
            }
        }

        // Search in mutants
        if let Some(mut_obj) = self.mutants.get_mut(&handle) {
            if mut_obj.count == 0 {
                mut_obj.count = 1;
                mut_obj.owner_thread_id = Some(0x100);
                return NtStatus::Success;
            } else {
                return NtStatus::WaitTimeout;
            }
        }

        // Search in semaphores
        if let Some(sem) = self.semaphores.get_mut(&handle) {
            if sem.count > 0 {
                sem.count -= 1;
                return NtStatus::Success;
            } else {
                return NtStatus::WaitTimeout;
            }
        }

        NtStatus::InvalidHandle
    }
}

impl Default for NtSyncManager {
    fn default() -> Self {
        Self::new()
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
    pub driver_name: String,
    pub dispatch_create: Option<fn(&mut Irp) -> NtStatus>,
    pub dispatch_close: Option<fn(&mut Irp) -> NtStatus>,
    pub dispatch_read: Option<fn(&mut Irp) -> NtStatus>,
    pub dispatch_write: Option<fn(&mut Irp) -> NtStatus>,
    pub dispatch_device_control: Option<fn(&mut Irp) -> NtStatus>,
}

impl NtDriver {
    pub fn new(name: &str) -> Self {
        NtDriver {
            driver_name: name.to_string(),
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
    pub fn win32_virtual_alloc(&mut self, size: usize, protection: PageProtection) -> Result<usize, NtStatus> {
        self.vm_manager.allocate_virtual_memory(size, protection)
    }

    /// kernel32.dll -> CreateFileA
    pub fn win32_create_file_a(&mut self, filename: &str) -> NtHandle {
        self.object_manager.create_object(NtObjectType::File, filename)
    }

    /// kernel32.dll -> CloseHandle
    pub fn win32_close_handle(&mut self, handle: NtHandle) -> Result<(), NtStatus> {
        self.object_manager.close_handle(handle)
    }

    /// user32.dll -> CreateWindowExA
    pub fn win32_create_window_ex_a(&mut self, window_name: &str) -> NtHandle {
        self.object_manager.create_object(NtObjectType::Event, window_name)
    }
}

impl Default for Win32DllSubsystem {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// Representation of a Windows Registry Value
// ==========================================

pub struct RegistryValue {
    pub name: String,
    pub data: Vec<u8>,
}

impl RegistryValue {
    pub fn new(name: &str, data: &[u8]) -> Self {
        RegistryValue {
            name: name.to_string(),
            data: data.to_vec(),
        }
    }
}

/// Windows NT Registry Hive System (HKLM/HKCU configuration database)
pub struct RegistryHive {
    pub keys: Vec<String>,
    pub values: BTreeMap<String, RegistryValue>,
}

impl RegistryHive {
    pub fn new() -> Self {
        RegistryHive {
            keys: Vec::new(),
            values: BTreeMap::new(),
        }
    }

    /// Create registry key (NtCreateKey equivalent)
    pub fn create_key(&mut self, key_name: &str) {
        self.keys.push(key_name.to_string());
    }

    /// Set registry value (NtSetValueKey equivalent)
    pub fn set_value(&mut self, name: &str, data: &[u8]) {
        let value = RegistryValue::new(name, data);
        self.values.insert(name.to_string(), value);
    }

    /// Retrieve registry value (NtQueryValueKey equivalent)
    pub fn query_value(&self, name: &str) -> Result<&RegistryValue, NtStatus> {
        self.values.get(&name.to_string()).ok_or(NtStatus::ObjectNameNotFound)
    }
}

impl Default for RegistryHive {
    fn default() -> Self {
        Self::new()
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

// =========================================================================
// 1. NT NATIVE SYSTEM CALLS DISPATCHER
// =========================================================================

pub struct NtNativeSystemCalls {
    pub object_manager: NtObjectManager,
    pub active_driver: Option<NtDriver>,
}

impl NtNativeSystemCalls {
    pub fn new() -> Self {
        Self {
            object_manager: NtObjectManager::new(),
            active_driver: None,
        }
    }

    pub fn register_driver_dispatch(&mut self, driver: NtDriver) {
        self.active_driver = Some(driver);
    }

    /// NtOpenFile/NtCreateFile native syscall equivalent routing back to the active driver.
    pub fn nt_open_file(&mut self, filename: &str) -> Result<NtHandle, NtStatus> {
        if filename.is_empty() {
            return Err(NtStatus::ObjectNameNotFound);
        }
        let handle = self.object_manager.create_object(NtObjectType::File, filename);
        Ok(handle)
    }

    /// NtReadFile native syscall equivalent routing to our active IRP dispatcher.
    pub fn nt_read_file(&self, handle: NtHandle, buffer: &mut [u8]) -> Result<usize, NtStatus> {
        let _ = self.object_manager.lookup_object(handle)?;

        let driver = self.active_driver.as_ref().ok_or(NtStatus::AccessDenied)?;

        let mut irp = Irp {
            io_status: IoStatusBlock {
                status: NtStatus::AccessDenied,
                information: 0,
            },
            current_stack_location: IrpStackLocation {
                major_function: IrpMajorFunction::Read,
                minor_function: 0,
                flags: 0,
                parameters_read_length: buffer.len(),
                parameters_write_length: 0,
                parameters_ioctl_code: 0,
            },
            user_buffer: buffer.as_mut_ptr(),
            user_buffer_len: buffer.len(),
        };

        let status = driver.dispatch_irp(&mut irp);
        if status == NtStatus::Success {
            Ok(irp.io_status.information)
        } else {
            Err(status)
        }
    }
}

impl Default for NtNativeSystemCalls {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 2. WIN32 GRAPHIC DEVICE INTERFACE (GDI) ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GdiObjectType {
    Pen,
    Brush,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GdiObject {
    pub object_type: GdiObjectType,
    pub color_rgb: u32,
}

pub struct Win32GdiEngine {
    pub gdi_resources: BTreeMap<NtHandle, GdiObject>,
    pub graphics_buffer: Vec<u32>, // Simulated screen buffer representing active DC paint
    pub width: usize,
    pub height: usize,
}

impl Win32GdiEngine {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            gdi_resources: BTreeMap::new(),
            graphics_buffer: vec![0u32; w * h],
            width: w,
            height: h,
        }
    }

    pub fn create_gdi_object(&mut self, handle: NtHandle, obj_type: GdiObjectType, color: u32) {
        let obj = GdiObject {
            object_type: obj_type,
            color_rgb: color,
        };
        self.gdi_resources.insert(handle, obj);
    }

    /// Simulates painting/drawing a filled rectangle on the GDI device context.
    pub fn gdi_rectangle(
        &mut self,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
        brush_handle: NtHandle,
    ) -> Result<(), &'static str> {
        let brush = self
            .gdi_resources
            .get(&brush_handle)
            .ok_or("GDI Error: Invalid GDI handle")?;

        if brush.object_type != GdiObjectType::Brush {
            return Err("GDI Error: GDI handle is not a valid Brush object");
        }

        // Paint rect pixels inside bounds
        let end_x = x2.min(self.width);
        let end_y = y2.min(self.height);

        for y in y1..end_y {
            for x in x1..end_x {
                let idx = y * self.width + x;
                self.graphics_buffer[idx] = brush.color_rgb;
            }
        }

        Ok(())
    }
}

// =========================================================================
// 3. STRUCTURED EXCEPTION HANDLING (SEH) DISPATCHER
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SehExceptionCode {
    AccessViolation = 0xC0000005,
    IntegerDivideByZero = 0xC0000094,
    PageFault = 0xC0000006,
}

pub struct SehRegistrationFrame {
    pub handler_rip: usize,
    pub exception_code_mask: SehExceptionCode,
}

pub struct SehExceptionDispatcher {
    pub nested_frames: Vec<SehRegistrationFrame>,
}

impl SehExceptionDispatcher {
    pub fn new() -> Self {
        Self {
            nested_frames: Vec::new(),
        }
    }

    pub fn register_seh_frame(&mut self, handler: usize, code: SehExceptionCode) {
        self.nested_frames.push(SehRegistrationFrame {
            handler_rip: handler,
            exception_code_mask: code,
        });
    }

    /// Simulates unwinding exception frames back to the registered SEH handler.
    pub fn raise_and_unwind_exception(&self, code: SehExceptionCode) -> Result<usize, &'static str> {
        // Unwind descending (last registered frame has highest priority)
        for frame in self.nested_frames.iter().rev() {
            if frame.exception_code_mask == code {
                return Ok(frame.handler_rip); // Returns target exception handler address
            }
        }
        Err("SEH Error: Unhandled exception! Thread terminated.")
    }
}

impl Default for SehExceptionDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// UNIT TESTS MODULE
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nt_object_manager_handles() {
        let mut manager = NtObjectManager::new();
        let h1 = manager.create_object(NtObjectType::File, "DeviceKeyboard");
        let h2 = manager.create_object(NtObjectType::Process, "explorer.exe");

        assert_eq!(h1, 0x10);
        assert_eq!(h2, 0x14);

        let entry = manager.lookup_object(h1).unwrap();
        assert_eq!(entry.object_type, NtObjectType::File);
        assert_eq!(entry.name, "DeviceKeyboard");

        assert!(manager.close_handle(h1).is_ok());
        assert_eq!(
            manager.lookup_object(h1).unwrap_err(),
            NtStatus::InvalidHandle
        );
    }

    #[test]
    fn test_registry_hive_queries() {
        let mut hive = RegistryHive::new();
        hive.create_key("SOFTWARE\\SigmaOS");
        hive.set_value("Theme", b"SovereignDark");

        let val = hive.query_value("Theme").unwrap();
        assert_eq!(val.data, b"SovereignDark".to_vec());
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
            PortableExecutableLoader::validate_pe_image(&invalid_pe).unwrap_err(),
            NtStatus::InvalidImageFormat
        );
    }

    #[test]
    fn test_nt_virtual_memory_apis() {
        let mut vmm = NtVirtualMemoryManager::new();
        let size = 8192; // 2 pages
        let addr = vmm.allocate_virtual_memory(size, PageProtection::ReadWrite).unwrap();
        assert_eq!(addr, 0x00400000);

        let old_prot = vmm.protect_virtual_memory(addr, PageProtection::ExecuteRead).unwrap();
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
        assert_eq!(sync.wait_for_single_object(handle, 100), NtStatus::WaitTimeout);

        sync.create_semaphore(handle + 4, 3, 5);
        assert_eq!(sync.wait_for_single_object(handle + 4, 100), NtStatus::Success);
    }

    #[test]
    fn test_nt_irp_subsystem() {
        let mut driver = NtDriver::new("SigmaDiskDriver");
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
        let h_file = win32.win32_create_file_a("C:\\sigma_config.ini");
        assert_eq!(h_file, 0x10);

        let addr = win32.win32_virtual_alloc(4096, PageProtection::ReadWrite).unwrap();
        assert_eq!(addr, 0x00400000);

        assert!(win32.win32_close_handle(h_file).is_ok());
    }

    #[test]
    fn test_nt_native_system_calls() {
        let mut ntsys = NtNativeSystemCalls::new();
        let mut driver = NtDriver::new("VirtualMemoryDisk");

        fn mock_read(irp: &mut Irp) -> NtStatus {
            irp.io_status.status = NtStatus::Success;
            irp.io_status.information = 24;
            // Write mock values to user buffer
            unsafe {
                let ptr = irp.user_buffer;
                if !ptr.is_null() {
                    for i in 0..24 {
                        *ptr.add(i) = (i + 10) as u8;
                    }
                }
            }
            NtStatus::Success
        }
        driver.dispatch_read = Some(mock_read);
        ntsys.register_driver_dispatch(driver);

        let handle = ntsys.nt_open_file("C:\\boot_config.ini").unwrap();
        assert_eq!(handle, 0x10);

        let mut read_buf = [0u8; 24];
        let bytes_transferred = ntsys.nt_read_file(handle, &mut read_buf).unwrap();
        assert_eq!(bytes_transferred, 24);
        assert_eq!(read_buf[0], 10);
        assert_eq!(read_buf[23], 33);
    }

    #[test]
    fn test_win32_gdi_device_context_drawing() {
        let mut gdi = Win32GdiEngine::new(640, 480);
        let h_brush = 0x50;
        gdi.create_gdi_object(h_brush, GdiObjectType::Brush, 0xFF00FF); // Magenta

        assert!(gdi.gdi_rectangle(10, 20, 50, 40, h_brush).is_ok());
        // Verify index paint
        let idx = 25 * 640 + 30; // Row 25, Col 30 (inside rect bounds)
        assert_eq!(gdi.graphics_buffer[idx], 0xFF00FF);

        // Fail for invalid brush object
        assert!(gdi.gdi_rectangle(10, 20, 50, 40, 0x999).is_err());
    }

    #[test]
    fn test_seh_exception_unwinding() {
        let mut seh = SehExceptionDispatcher::new();
        seh.register_seh_frame(0x1000, SehExceptionCode::IntegerDivideByZero);
        seh.register_seh_frame(0x2000, SehExceptionCode::AccessViolation);

        // Raise Access Violation exception (highest priority / last nested frame wins)
        let rip = seh.raise_and_unwind_exception(SehExceptionCode::AccessViolation).unwrap();
        assert_eq!(rip, 0x2000);

        // Raise non-registered exception
        assert!(seh.raise_and_unwind_exception(SehExceptionCode::PageFault).is_err());
    }
}
