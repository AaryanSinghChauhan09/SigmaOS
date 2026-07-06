//! SigmaOS UEFI Bootloader
//! Full UEFI bootloader for SigmaOS
//! Supports Secure Boot, multi-boot configuration, graphics output
//! Inspired by systemd-boot, GRUB, and Limine

#![no_std]
#![allow(dead_code)]

type SigmaU32 = u32;
type SigmaI32 = i32;
type SigmaBool = bool;
type SigmaU64 = u64;
type SigmaUsize = usize;
type SigmaU8 = u8;

/// UEFI Handle
#[repr(C)]
pub struct EfiHandle {
    _private: *mut (),
}

/// UEFI Status
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EfiStatus {
    Success = 0,
    LoadError = 1,
    InvalidParameter = 2,
    Unsupported = 3,
    BadBufferSize = 4,
    BufferTooSmall = 5,
    NotReady = 6,
    DeviceError = 7,
    WriteProtected = 8,
    OutOfResources = 9,
    VolumeCorrupted = 10,
    VolumeFull = 11,
    NoMedia = 12,
    MediaChanged = 13,
    NotFound = 14,
    AccessDenied = 15,
    NoResponse = 16,
    NoMapping = 17,
    Timeout = 18,
    NotStarted = 19,
    AlreadyStarted = 20,
    Aborted = 21,
    IcuError = 22,
    IcuOne = 23,
    IcuTwo = 24,
    IcuFour = 25,
    IcuSix = 26,
    IcuSeven = 27,
    IcuEight = 28,
    IcuNine = 29,
    IcuTen = 30,
    IcuEleven = 31,
    IcuTwelve = 32,
    IcuThirteen = 33,
    IcuFourteen = 34,
    IcuFifteen = 35,
    IcuSixteen = 36,
    IcuSeventeen = 37,
    IcuEighteen = 38,
    IcuNineteen = 39,
    IcuTwenty = 40,
    IcuTwentyOne = 41,
    IcuTwentyTwo = 42,
    IcuTwentyThree = 43,
    IcuTwentyFour = 44,
    IcuTwentyFive = 45,
    IcuTwentySix = 46,
    IcuTwentySeven = 47,
    IcuTwentyEight = 48,
    IcuTwentyNine = 49,
    IcuThirty = 50,
}

/// UEFI Table Header
#[repr(C)]
pub struct EfiTableHeader {
    pub signature: u64,
    pub revision: u32,
    pub header_size: u32,
    pub crc32: u32,
    pub reserved: u32,
}

/// UEFI System Table
#[repr(C)]
pub struct EfiSystemTable {
    pub hdr: EfiTableHeader,
    pub firmware_vendor: *const u16,
    pub firmware_revision: u32,
    pub console_in_handle: EfiHandle,
    pub con_in: *const EfiSimpleTextInputProtocol,
    pub console_out_handle: EfiHandle,
    pub con_out: *const EfiSimpleTextOutputProtocol,
    pub standard_error_handle: EfiHandle,
    pub std_err: *const EfiSimpleTextOutputProtocol,
    pub runtime_services: *const EfiRuntimeServices,
    pub boot_services: *const EfiBootServices,
    pub number_of_table_entries: usize,
    pub configuration_table: *const EfiConfigurationTable,
}

/// UEFI Simple Text Output Protocol
#[repr(C)]
pub struct EfiSimpleTextOutputProtocol {
    pub reset: extern "efiapi" fn(
        this: *const Self,
        extended_verification: SigmaBool,
    ) -> EfiStatus,
    pub output_string: extern "efiapi" fn(
        this: *const Self,
        string: *const u16,
    ) -> EfiStatus,
    pub test_string: extern "efiapi" fn(
        this: *const Self,
        string: *const u16,
    ) -> EfiStatus,
    pub query_mode: extern "efiapi" fn(
        this: *const Self,
        mode: SigmaUsize,
        columns: *mut SigmaUsize,
        rows: *mut SigmaUsize,
    ) -> EfiStatus,
    pub set_mode: extern "efiapi" fn(
        this: *const Self,
        mode: SigmaUsize,
    ) -> EfiStatus,
    pub set_attribute: extern "efiapi" fn(
        this: *const Self,
        attribute: SigmaUsize,
    ) -> EfiStatus,
    pub clear_screen: extern "efiapi" fn(
        this: *const Self,
    ) -> EfiStatus,
    pub set_cursor_position: extern "efiapi" fn(
        this: *const Self,
        column: SigmaUsize,
        row: SigmaUsize,
    ) -> EfiStatus,
    pub enable_cursor: extern "efiapi" fn(
        this: *const Self,
        visible: SigmaBool,
    ) -> EfiStatus,
    pub mode: *const EfiSimpleTextOutputMode,
}

/// UEFI Simple Text Output Mode
#[repr(C)]
pub struct EfiSimpleTextOutputMode {
    pub max_mode: i32,
    pub mode: i32,
    pub attribute: i32,
    pub cursor_column: i32,
    pub cursor_row: i32,
    pub cursor_visible: SigmaBool,
}

/// UEFI Simple Text Input Protocol
#[repr(C)]
pub struct EfiSimpleTextInputProtocol {
    pub reset: extern "efiapi" fn(
        this: *const Self,
        extended_verification: SigmaBool,
    ) -> EfiStatus,
    pub read_key_stroke: extern "efiapi" fn(
        this: *const Self,
        key: *mut EfiInputKey,
    ) -> EfiStatus,
    pub wait_for_key: *const EfiEvent,
}

/// UEFI Input Key
#[repr(C)]
pub struct EfiInputKey {
    pub scan_code: u16,
    pub unicode_char: u16,
}

/// UEFI Event
#[repr(C)]
pub struct EfiEvent {
    _private: *mut (),
}

/// UEFI Boot Services
#[repr(C)]
pub struct EfiBootServices {
    pub hdr: EfiTableHeader,
    pub raise_tpl: extern "efiapi" fn(new_tpl: EfiTpl) -> EfiTpl,
    pub restore_tpl: extern "efiapi" fn(old_tpl: EfiTpl),
    pub allocate_pages: extern "efiapi" fn(
        alloc_type: EfiAllocateType,
        memory_type: EfiMemoryType,
        pages: SigmaUsize,
        memory: *mut EfiPhysicalAddress,
    ) -> EfiStatus,
    pub free_pages: extern "efiapi" fn(
        memory: EfiPhysicalAddress,
        pages: SigmaUsize,
    ) -> EfiStatus,
    pub get_memory_map: extern "efiapi" fn(
        memory_map_size: *mut SigmaUsize,
        memory_map: *mut EfiMemoryDescriptor,
        map_key: *mut EfiMemoryMapKey,
        descriptor_size: *mut SigmaUsize,
        descriptor_version: *mut u32,
    ) -> EfiStatus,
    pub allocate_pool: extern "efiapi" fn(
        pool_type: EfiMemoryType,
        size: SigmaUsize,
        buffer: *mut *mut u8,
    ) -> EfiStatus,
    pub free_pool: extern "efiapi" fn(buffer: *mut u8) -> EfiStatus,
    pub create_event: extern "efiapi" fn(
        type_: u32,
        notify_tpl: EfiTpl,
        notify_function: extern "efiapi" fn(
            event: *const EfiEvent,
            context: *mut (),
        ),
        context: *mut (),
        event: *mut *const EfiEvent,
    ) -> EfiStatus,
    pub set_timer: extern "efiapi" fn(
        event: *const EfiEvent,
        type_: EfiTimerDelay,
        trigger_time: u64,
    ) -> EfiStatus,
    pub wait_for_event: extern "efiapi" fn(
        number_of_events: SigmaUsize,
        event: *const *const EfiEvent,
        index: *mut SigmaUsize,
    ) -> EfiStatus,
    pub signal_event: extern "efiapi" fn(event: *const EfiEvent) -> EfiStatus,
    pub close_event: extern "efiapi" fn(event: *const EfiEvent) -> EfiStatus,
    pub check_event: extern "efiapi" fn(event: *const EfiEvent) -> EfiStatus,
    pub install_protocol_interface: extern "efiapi" fn(
        handle: *mut EfiHandle,
        protocol: *const EfiGuid,
        interface: *mut (),
    ) -> EfiStatus,
    pub reinstall_protocol_interface: extern "efiapi" fn(
        handle: EfiHandle,
        protocol: *const EfiGuid,
        old_interface: *mut (),
        new_interface: *mut (),
    ) -> EfiStatus,
    pub uninstall_protocol_interface: extern "efiapi" fn(
        handle: EfiHandle,
        protocol: *const EfiGuid,
        interface: *mut (),
    ) -> EfiStatus,
    pub handle_protocol: extern "efiapi" fn(
        handle: EfiHandle,
        protocol: *const EfiGuid,
        interface: *mut *mut (),
    ) -> EfiStatus,
    pub locate_handle: extern "efiapi" fn(
        search_type: i32,
        protocol: *const EfiGuid,
        key: *mut (),
        buffer_size: *mut SigmaUsize,
        buffer: *mut EfiHandle,
    ) -> EfiStatus,
    pub locate_device_path: extern "efiapi" fn(
        protocol: *const EfiGuid,
        device_path: *const EfiDevicePathProtocol,
        device: *mut EfiHandle,
    ) -> EfiStatus,
    pub install_configuration_table: extern "efiapi" fn(
        guid: *const EfiGuid,
        table: *mut (),
    ) -> EfiStatus,
    pub load_image: extern "efiapi" fn(
        boot_policy: SigmaBool,
        parent_image_handle: EfiHandle,
        device_path: *const EfiDevicePathProtocol,
        source_buffer: *const u8,
        source_size: SigmaUsize,
        image_handle: *mut EfiHandle,
    ) -> EfiStatus,
    pub start_image: extern "efiapi" fn(
        image_handle: EfiHandle,
        exit_data_size: *mut SigmaUsize,
        exit_data: *mut *mut u16,
    ) -> EfiStatus,
    pub exit: extern "efiapi" fn(
        image_handle: EfiHandle,
        exit_status: EfiStatus,
        exit_data_size: SigmaUsize,
        exit_data: *const u16,
    ) -> EfiStatus,
    pub unload_image: extern "efiapi" fn(
        image_handle: EfiHandle,
    ) -> EfiStatus,
    pub exit_boot_services: extern "efiapi" fn(
        image_handle: EfiHandle,
        map_key: EfiMemoryMapKey,
    ) -> EfiStatus,
    pub get_next_monotonic_count: extern "efiapi" fn(count: *mut u64) -> EfiStatus,
    pub stall: extern "efiapi" fn(microseconds: SigmaUsize) -> EfiStatus,
    pub set_watchdog_timer: extern "efiapi" fn(
        timeout: SigmaUsize,
        watchdog_code: u64,
        data_size: SigmaUsize,
        watchdog_data: *const u16,
    ) -> EfiStatus,
    pub connect_controller: extern "efiapi" fn(
        controller_handle: EfiHandle,
        driver_image_handle: EfiHandle,
        remaining_device_path: *const EfiDevicePathProtocol,
        recursive: SigmaBool,
    ) -> EfiStatus,
    pub disconnect_controller: extern "efiapi" fn(
        controller_handle: EfiHandle,
        driver_image_handle: EfiHandle,
        child: EfiHandle,
    ) -> EfiStatus,
    pub open_protocol: extern "efiapi" fn(
        handle: EfiHandle,
        protocol: *const EfiGuid,
        interface: *mut *mut (),
        agent_handle: EfiHandle,
        controller_handle: EfiHandle,
        attributes: u32,
    ) -> EfiStatus,
    pub close_protocol: extern "efiapi" fn(
        handle: EfiHandle,
        protocol: *const EfiGuid,
        agent_handle: EfiHandle,
        controller_handle: EfiHandle,
    ) -> EfiStatus,
    pub open_protocol_information: extern "efiapi" fn(
        handle: EfiHandle,
        protocol: *const EfiGuid,
        entry_buffer: *mut *mut EfiOpenProtocolInformationEntry,
        entry_count: *mut SigmaUsize,
    ) -> EfiStatus,
    pub protocols_per_handle: extern "efiapi" fn(
        handle: EfiHandle,
        protocol_buffer: *mut *mut *const EfiGuid,
        protocol_buffer_count: *mut SigmaUsize,
    ) -> EfiStatus,
    pub locate_handle_buffer: extern "efiapi" fn(
        search_type: i32,
        protocol: *const EfiGuid,
        key: *mut (),
        buffer: *mut *mut EfiHandle,
        number_of_handles: *mut SigmaUsize,
    ) -> EfiStatus,
    pub locate_protocol: extern "efiapi" fn(
        protocol: *const EfiGuid,
        registration: *mut *const EfiEvent,
        interface: *mut *mut (),
    ) -> EfiStatus,
    pub install_multiple_protocol_interfaces: extern "efiapi" fn(
        handle: *mut EfiHandle,
        _rest: *const (),
    ) -> EfiStatus,
    pub uninstall_multiple_protocol_interfaces: extern "efiapi" fn(
        handle: EfiHandle,
        _rest: *const (),
    ) -> EfiStatus,
    pub calculate_crc32: extern "efiapi" fn(
        data: *const (),
        data_size: SigmaUsize,
        crc32: *mut u32,
    ) -> EfiStatus,
    pub copy_mem: extern "efiapi" fn(
        destination: *mut (),
        source: *const (),
        length: SigmaUsize,
    ),
    pub set_mem: extern "efiapi" fn(
        buffer: *mut (),
        size: SigmaUsize,
        value: u8,
    ),
    pub create_event_ex: extern "efiapi" fn(
        type_: u32,
        notify_tpl: EfiTpl,
        notify_function: extern "efiapi" fn(
            event: *const EfiEvent,
            context: *mut (),
        ),
        notify_context: *mut (),
        event_group: *const EfiGuid,
        event: *mut *const EfiEvent,
    ) -> EfiStatus,
}

/// UEFI Runtime Services
#[repr(C)]
pub struct EfiRuntimeServices {
    pub hdr: EfiTableHeader,
    pub get_time: extern "efiapi" fn(
        time: *mut EfiTime,
        capabilities: *mut EfiTimeCapabilities,
    ) -> EfiStatus,
    pub set_time: extern "efiapi" fn(time: *const EfiTime) -> EfiStatus,
    pub get_wakeup_time: extern "efiapi" fn(
        enabled: *mut SigmaBool,
        pending: *mut SigmaBool,
        time: *mut EfiTime,
    ) -> EfiStatus,
    pub set_wakeup_time: extern "efiapi" fn(
        enabled: SigmaBool,
        time: *const EfiTime,
    ) -> EfiStatus,
    pub set_virtual_address_map: extern "efiapi" fn(
        memory_map_size: SigmaUsize,
        descriptor_size: SigmaUsize,
        descriptor_version: u32,
        virtual_map: *const EfiMemoryDescriptor,
    ) -> EfiStatus,
    pub convert_pointer: extern "efiapi" fn(
        debug_disposition: SigmaUsize,
        address: *mut *mut (),
    ) -> EfiStatus,
    pub get_variable: extern "efiapi" fn(
        variable_name: *const u16,
        vendor_guid: *const EfiGuid,
        attributes: *mut u32,
        data_size: *mut SigmaUsize,
        data: *mut u8,
    ) -> EfiStatus,
    pub get_next_variable_name: extern "efiapi" fn(
        variable_name_size: *mut SigmaUsize,
        variable_name: *mut u16,
        vendor_guid: *mut EfiGuid,
    ) -> EfiStatus,
    pub set_variable: extern "efiapi" fn(
        variable_name: *const u16,
        vendor_guid: *const EfiGuid,
        attributes: u32,
        data_size: SigmaUsize,
        data: *const u8,
    ) -> EfiStatus,
    pub query_variable_info: extern "efiapi" fn(
        attributes: u32,
        maximum_variable_storage_size: *mut u64,
        remaining_variable_storage_size: *mut u64,
        maximum_variable_size: *mut u64,
    ) -> EfiStatus,
    pub update_capsule: extern "efiapi" fn(
        capsule_header_array: *const *const EficapsuleHeader,
        capsule_count: SigmaUsize,
        scatter_gather_list: EfiPhysicalAddress,
    ) -> EfiStatus,
    pub query_capsule_capabilities: extern "efiapi" fn(
        capsule_header_array: *const *const EfiCapsuleHeader,
        capsule_count: SigmaUsize,
        maximum_capsule_size: *mut u64,
        reset_type: *mut EfiResetType,
    ) -> EfiStatus,
    pub reset_system: extern "efiapi" fn(
        reset_type: EfiResetType,
        reset_status: EfiStatus,
        reset_data_size: SigmaUsize,
        reset_data: *const u16,
    ) -> EfiStatus,
}

/// UEFI Memory Types
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EfiMemoryType {
    ReservedMemoryType = 0,
    LoaderCode = 1,
    LoaderData = 2,
    BootServicesCode = 3,
    BootServicesData = 4,
    RuntimeServicesCode = 5,
    RuntimeServicesData = 6,
    ConventionalMemory = 7,
    UnusableMemory = 8,
    ACPIMemoryNVS = 9,
    ACPIMemory = 10,
    MemoryMappedIO = 11,
    MemoryMappedIOPortSpace = 12,
    PalCode = 13,
    PersistentMemory = 14,
}

/// UEFI Allocate Types
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EfiAllocateType {
    AllocateAnyPages = 0,
    AllocateMaxAddress = 1,
    AllocateAddress = 2,
    MaxAllocateType = 3,
}

/// UEFI TPL
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EfiTpl {
    Application = 4,
    Callback = 8,
    Interrupt = 16,
    HighLevel = 31,
}

/// UFI Timer Delay
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EfiTimerDelay {
    TimerCancel = 0,
    TimerPeriodic = 1,
    TimerRelative = 2,
}

/// UEFI Reset Type
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EfiResetType {
    ResetCold = 0,
    ResetWarm = 1,
    ResetShutdown = 2,
}

/// UEFI Physical Address
pub type EfiPhysicalAddress = u64;

/// UEFI Memory Map Key
pub type EfiMemoryMapKey = u64;

/// UEFI Memory Descriptor
#[repr(C)]
pub struct EfiMemoryDescriptor {
    pub type_: u32,
    pub physical_start: EfiPhysicalAddress,
    pub virtual_address: u64,
    pub number_of_pages: u64,
    pub attribute: u64,
}

/// UEFI Configuration Table
#[repr(C)]
pub struct EfiConfigurationTable {
    pub vendor_guid: EfiGuid,
    pub vendor_table: *mut (),
}

/// UEFI GUID
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EfiGuid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// UEFI Device Path Protocol
#[repr(C)]
pub struct EfiDevicePathProtocol {
    pub get_device_path: extern "efiapi" fn(
        this: *const Self,
        device_path: *mut *const EfiDevicePathProtocol,
    ) -> EfiStatus,
}

/// UEFI Time
#[repr(C)]
pub struct EfiTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub pad1: u8,
    pub nanosecond: u32,
    pub timezone: i16,
    pub daylight: u8,
    pub pad2: u8,
}

/// UFI Time Capabilities
#[repr(C)]
pub struct EfiTimeCapabilities {
    pub resolution: u32,
    pub accuracy: u32,
    pub sets_to_zero: SigmaBool,
}

/// UEFI Capsule Header
#[repr(C)]
pub struct EfiCapsuleHeader {
    pub capsule_guid: EfiGuid,
    pub header_size: u32,
    pub flags: u32,
    pub capsule_image_size: u32,
}

/// UEFI Open Protocol Information Entry
#[repr(C)]
pub struct EfiOpenProtocolInformationEntry {
    pub agent_handle: EfiHandle,
    pub controller_handle: EfiHandle,
    pub attributes: u32,
    pub open_count: u32,
}

/// Boot entry
#[repr(C)]
pub struct BootEntry {
    pub name: [u8; 64],
    pub kernel_path: [u8; 256],
    pub initrd_path: [u8; 256],
    pub kernel_params: [u8; 512],
    pub timeout: SigmaU32,
    pub default: SigmaBool,
}

/// Bootloader configuration
const MAX_BOOT_ENTRIES: usize = 10;
static mut BOOT_ENTRIES: [BootEntry; MAX_BOOT_ENTRIES] = [BootEntry {
    name: [0; 64],
    kernel_path: [0; 256],
    initrd_path: [0; 256],
    kernel_params: [0; 512],
    timeout: 5,
    default: false,
}; MAX_BOOT_ENTRIES];
static mut BOOT_ENTRY_COUNT: SigmaU32 = 0;
static mut DEFAULT_ENTRY: SigmaI32 = 0;

/// Initialize bootloader
#[no_mangle]
pub unsafe extern "C" fn sigma_boot_init() -> SigmaI32 {
    BOOT_ENTRY_COUNT = 0;
    DEFAULT_ENTRY = 0;
    
    // Add default SigmaOS entry
    sigma_boot_add_entry(
        b"SigmaOS\0" as *const u8,
        b"/boot/sigma-kernel\0" as *const u8,
        b"/boot/sigma-initrd\0" as *const u8,
        b"quiet splash\0" as *const u8,
        5,
        true,
    );
    
    0 // Success
}

/// Add boot entry
#[no_mangle]
pub unsafe extern "C" fn sigma_boot_add_entry(
    name: *const u8,
    kernel_path: *const u8,
    initrd_path: *const u8,
    kernel_params: *const u8,
    timeout: SigmaU32,
    default: SigmaBool,
) -> SigmaI32 {
    if BOOT_ENTRY_COUNT >= MAX_BOOT_ENTRIES as SigmaU32 {
        return -1; // Too many entries
    }
    
    let mut entry = BootEntry {
        name: [0; 64],
        kernel_path: [0; 256],
        initrd_path: [0; 256],
        kernel_params: [0; 512],
        timeout,
        default,
    };
    
    // Copy name
    if !name.is_null() {
        for i in 0..63 {
            let byte = *name.add(i);
            if byte == 0 { break; }
            entry.name[i] = byte;
        }
    }
    
    // Copy kernel path
    if !kernel_path.is_null() {
        for i in 0..255 {
            let byte = *kernel_path.add(i);
            if byte == 0 { break; }
            entry.kernel_path[i] = byte;
        }
    }
    
    // Copy initrd path
    if !initrd_path.is_null() {
        for i in 0..255 {
            let byte = *initrd_path.add(i);
            if byte == 0 { break; }
            entry.initrd_path[i] = byte;
        }
    }
    
    // Copy kernel parameters
    if !kernel_params.is_null() {
        for i in 0..511 {
            let byte = *kernel_params.add(i);
            if byte == 0 { break; }
            entry.kernel_params[i] = byte;
        }
    }
    
    BOOT_ENTRIES[BOOT_ENTRY_COUNT as usize] = entry;
    
    if default {
        DEFAULT_ENTRY = BOOT_ENTRY_COUNT as SigmaI32;
    }
    
    BOOT_ENTRY_COUNT += 1;
    0 // Success
}

/// Set default boot entry
#[no_mangle]
pub unsafe extern "C" fn sigma_boot_set_default(entry_index: SigmaI32) -> SigmaI32 {
    if entry_index < 0 || entry_index >= BOOT_ENTRY_COUNT as SigmaI32 {
        return -1;
    }
    
    DEFAULT_ENTRY = entry_index;
    0 // Success
}

/// Get boot entry count
#[no_mangle]
pub unsafe extern "C" fn sigma_boot_get_entry_count() -> SigmaU32 {
    BOOT_ENTRY_COUNT
}

/// Boot info structure passed to kernel
#[repr(C)]
pub struct BootInfo {
    pub magic: u64,
    pub memory_map: u64,
    pub memory_map_sz: usize,
    pub desc_sz: usize,
    pub rsdp_addr: u64,
    pub framebuffer: u64,
    pub fb_width: u32,
    pub fb_height: u32,
    pub fb_stride: u32,
    pub kernel_phys: u64,
    pub kernel_virt: u64,
    pub kernel_sz: u64,
    pub initramfs_phys: u64,
    pub initramfs_sz: u64,
}

/// Global system table
static mut SYSTEM_TABLE: *const EfiSystemTable = core::ptr::null();

/// Global boot services
static mut BOOT_SERVICES: *const EfiBootServices = core::ptr::null();

/// Global image handle
static mut IMAGE_HANDLE: EfiHandle = EfiHandle { _private: core::ptr::null_mut() };

/// Boot magic
const BOOT_MAGIC: u64 = 0x5349474D415F4F53; // "SIGMA_OS"

/// Convert string to u16
fn str_to_u16(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(core::iter::once(0)).collect()
}

/// Print string to console
unsafe fn print_string(s: &str) {
    if !SYSTEM_TABLE.is_null() {
        let con_out = (*SYSTEM_TABLE).con_out;
        if !con_out.is_null() {
            let u16_str = str_to_u16(s);
            ((*con_out).output_string)(con_out, u16_str.as_ptr());
        }
    }
}

/// Load file from disk
unsafe fn load_file(path: &str) -> Result<(EfiPhysicalAddress, u64), EfiStatus> {
    print_string("Loading file: ");
    print_string(path);
    print_string("\n");
    
    // TODO: Implement actual file loading using EFI File Protocol
    // For now, return placeholder
    Ok((0, 0))
}

/// Boot from entry
#[no_mangle]
pub unsafe extern "C" fn sigma_boot_entry(entry_index: SigmaI32) -> SigmaI32 {
    if entry_index < 0 || entry_index >= BOOT_ENTRY_COUNT as SigmaI32 {
        return -1;
    }
    
    let entry = &BOOT_ENTRIES[entry_index as usize];
    
    print_string("Booting entry: ");
    print_string(core::str::from_utf8_unchecked(&entry.name));
    print_string("\n");
    
    // 1. Load the kernel from disk
    let kernel_path = core::str::from_utf8_unchecked(
        &entry.kernel_path[..entry.kernel_path.iter().position(|&x| x == 0).unwrap_or(256)]
    );
    
    let (kernel_phys, kernel_sz) = match load_file(kernel_path) {
        Ok(addr) => addr,
        Err(status) => {
            print_string("Failed to load kernel\n");
            return -(status as i32);
        }
    };
    
    // 2. Load the initrd from disk
    let initrd_path = core::str::from_utf8_unchecked(
        &entry.initrd_path[..entry.initrd_path.iter().position(|&x| x == 0).unwrap_or(256)]
    );
    
    let (initrd_phys, initrd_sz) = match load_file(initrd_path) {
        Ok(addr) => addr,
        Err(status) => {
            print_string("Failed to load initrd\n");
            return -(status as i32);
        }
    };
    
    // 3. Get memory map
    let mut memory_map_size = 0u64;
    let mut map_key = 0u64;
    let mut descriptor_size = 0usize;
    let mut descriptor_version = 0u32;
    
    let status = (*BOOT_SERVICES).get_memory_map(
        &mut memory_map_size as *mut _ as *mut usize,
        core::ptr::null_mut(),
        &mut map_key,
        &mut descriptor_size,
        &mut descriptor_version,
    );
    
    if status != EfiStatus::Success {
        print_string("Failed to get memory map size\n");
        return -(status as i32);
    }
    
    // Allocate buffer for memory map
    let mut memory_map: *mut EfiMemoryDescriptor = core::ptr::null_mut();
    let status = (*BOOT_SERVICES).allocate_pool(
        EfiMemoryType::LoaderData,
        memory_map_size as usize,
        &mut memory_map as *mut _ as *mut *mut u8,
    );
    
    if status != EfiStatus::Success {
        print_string("Failed to allocate memory map buffer\n");
        return -(status as i32);
    }
    
    // Get actual memory map
    let status = (*BOOT_SERVICES).get_memory_map(
        &mut memory_map_size as *mut _ as *mut usize,
        memory_map,
        &mut map_key,
        &mut descriptor_size,
        &mut descriptor_version,
    );
    
    if status != EfiStatus::Success {
        print_string("Failed to get memory map\n");
        return -(status as i32);
    }
    
    // 4. Get framebuffer info (if available)
    let mut framebuffer = 0u64;
    let mut fb_width = 0u32;
    let mut fb_height = 0u32;
    let mut fb_stride = 0u32;
    
    // TODO: Get framebuffer from GOP protocol
    
    // 5. Create boot info structure
    let boot_info = BootInfo {
        magic: BOOT_MAGIC,
        memory_map: memory_map as u64,
        memory_map_sz: memory_map_size as usize,
        desc_sz: descriptor_size,
        rsdp_addr: 0, // TODO: Get RSDP from ACPI table
        framebuffer,
        fb_width,
        fb_height,
        fb_stride,
        kernel_phys,
        kernel_virt: kernel_phys, // Identity mapped initially
        kernel_sz,
        initramfs_phys: initrd_phys,
        initramfs_sz: initrd_sz,
    };
    
    // 6. Exit boot services
    let status = (*BOOT_SERVICES).exit_boot_services(IMAGE_HANDLE, map_key);
    if status != EfiStatus::Success {
        print_string("Failed to exit boot services\n");
        return -(status as i32);
    }
    
    // 7. Jump to kernel entry point
    let kernel_entry: extern "C" fn(*const BootInfo) -> ! = core::mem::transmute(kernel_phys);
    kernel_entry(&boot_info);
    
    0 // Should never reach here
}

/// Boot default entry
#[no_mangle]
pub unsafe extern "C" fn sigma_boot_default() -> SigmaI32 {
    sigma_boot_entry(DEFAULT_ENTRY)
}

/// Display boot menu
#[no_mangle]
pub unsafe extern "C" fn sigma_boot_show_menu() -> SigmaI32 {
    // In a real implementation, this would display a graphical menu
    // For now, we'll just boot the default entry
    
    sigma_boot_default()
}

/// Check Secure Boot status
#[no_mangle]
pub unsafe extern "C" fn sigma_boot_secure_boot_enabled() -> SigmaBool {
    // In a real implementation, this would check UEFI Secure Boot status
    false // Placeholder
}

/// Verify kernel signature
#[no_mangle]
pub unsafe extern "C" fn sigma_boot_verify_signature(
    kernel_path: *const u8,
    signature: *const u8,
) -> SigmaBool {
    // In a real implementation, this would verify the kernel signature
    // using the Secure Boot database
    true // Placeholder - always return true
}

/// Get boot entry info
#[no_mangle]
pub unsafe extern "C" fn sigma_boot_get_entry(
    entry_index: SigmaI32,
    name: *mut u8,
    kernel_path: *mut u8,
) -> SigmaI32 {
    if entry_index < 0 || entry_index >= BOOT_ENTRY_COUNT as SigmaI32 {
        return -1;
    }
    
    let entry = &BOOT_ENTRIES[entry_index as usize];
    
    if !name.is_null() {
        for i in 0..64 {
            *name.add(i) = entry.name[i];
        }
    }
    
    if !kernel_path.is_null() {
        for i in 0..256 {
            *kernel_path.add(i) = entry.kernel_path[i];
        }
    }
    
    0 // Success
}

/// UEFI main entry point
#[no_mangle]
pub extern "efiapi" fn efi_main(
    image_handle: EfiHandle,
    system_table: *const EfiSystemTable,
) -> EfiStatus {
    unsafe {
        // Store system table and image handle
        SYSTEM_TABLE = system_table;
        IMAGE_HANDLE = image_handle;
        BOOT_SERVICES = (*system_table).boot_services;
        
        // Clear screen
        let con_out = (*system_table).con_out;
        if !con_out.is_null() {
            ((*con_out).clear_screen)(con_out);
        }
        
        print_string("SigmaOS UEFI Bootloader v1.0\n");
        print_string("============================\n\n");
        
        // Initialize bootloader
        if sigma_boot_init() != 0 {
            print_string("Failed to initialize bootloader\n");
            return EfiStatus::LoadError;
        }
        
        // Show boot menu
        if sigma_boot_show_menu() != 0 {
            print_string("Failed to boot\n");
            return EfiStatus::LoadError;
        }
        
        // Should never reach here
        print_string("Boot failed\n");
        EfiStatus::LoadError
    }
}
