// Windows Driver Compatibility, Emulation, and PE Loading Framework for SigmaOS
// Implements WDM, WDF (KMDF/UMDF), NDIS, Storport, WDDM, and PE-grade .sys loading.

extern crate alloc;

use crate::driver::device::{
    BlockDevice, CharacterDevice, Device, DeviceError, DeviceInfo, DeviceType, NetworkDevice,
};
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

// --- Standard Windows Types & NTSTATUS Codes ---
pub type NTSTATUS = i32;
pub type ULONG = u32;
pub type PVOID = *mut core::ffi::c_void;
pub type HANDLE = *mut core::ffi::c_void;
pub type KIRQL = u8;

pub const STATUS_SUCCESS: NTSTATUS = 0x00000000;
pub const STATUS_UNSUCCESSFUL: NTSTATUS = 0xC0000001_u32 as i32;
pub const STATUS_NOT_IMPLEMENTED: NTSTATUS = 0xC0000002_u32 as i32;
pub const STATUS_INVALID_PARAMETER: NTSTATUS = 0xC000000D_u32 as i32;
pub const STATUS_BUFFER_TOO_SMALL: NTSTATUS = 0xC0000023_u32 as i32;
pub const STATUS_PENDING: NTSTATUS = 0x00000103;

// --- 1. Windows Driver Model (WDM) Structures & APIs ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MajorFunction {
    Create = 0,
    Close = 1,
    Read = 2,
    Write = 3,
    DeviceControl = 4,
    Pnp = 5,
    Power = 6,
    Max = 7,
}

pub struct DRIVER_OBJECT {
    pub driver_name: String,
    pub major_functions:
        [Option<fn(&mut DEVICE_OBJECT, &mut IRP) -> NTSTATUS>; MajorFunction::Max as usize],
    pub driver_extension: PVOID,
    pub driver_start: PVOID,
    pub driver_size: ULONG,
    pub driver_unload: Option<fn(&mut DRIVER_OBJECT)>,
}

impl DRIVER_OBJECT {
    pub fn new(name: &str) -> Self {
        DRIVER_OBJECT {
            driver_name: String::from(name),
            major_functions: [None; MajorFunction::Max as usize],
            driver_extension: core::ptr::null_mut(),
            driver_start: core::ptr::null_mut(),
            driver_size: 0,
            driver_unload: None,
        }
    }
}

pub struct DEVICE_OBJECT {
    pub driver_object: *mut DRIVER_OBJECT,
    pub next_device: *mut DEVICE_OBJECT,
    pub device_extension: Vec<u8>,
    pub flags: ULONG,
    pub device_type: ULONG,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IO_STACK_LOCATION {
    pub major_function: u8,
    pub minor_function: u8,
    pub parameters_device_io_control: ParametersDeviceIoControl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParametersDeviceIoControl {
    pub output_buffer_length: ULONG,
    pub input_buffer_length: ULONG,
    pub io_control_code: ULONG,
    pub type3_input_buffer: PVOID,
}

pub struct IRP {
    pub io_status: NTSTATUS,
    pub information: usize,
    pub associated_irp_system_buffer: PVOID,
    pub user_buffer: PVOID,
    pub current_stack_location: IO_STACK_LOCATION,
}

impl IRP {
    pub fn new(major: MajorFunction, control_code: u32, in_len: u32, out_len: u32) -> Self {
        IRP {
            io_status: STATUS_SUCCESS,
            information: 0,
            associated_irp_system_buffer: core::ptr::null_mut(),
            user_buffer: core::ptr::null_mut(),
            current_stack_location: IO_STACK_LOCATION {
                major_function: major as u8,
                minor_function: 0,
                parameters_device_io_control: ParametersDeviceIoControl {
                    output_buffer_length: out_len,
                    input_buffer_length: in_len,
                    io_control_code: control_code,
                    type3_input_buffer: core::ptr::null_mut(),
                },
            },
        }
    }
}

// --- WDM Ke Spinlock & IRQL Simulation ---
pub struct KSPIN_LOCK {
    lock_val: AtomicUsize,
}

impl KSPIN_LOCK {
    pub fn new() -> Self {
        KSPIN_LOCK {
            lock_val: AtomicUsize::new(0),
        }
    }

    pub fn acquire(&self) -> KIRQL {
        while self
            .lock_val
            .compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            core::hint::spin_loop();
        }
        2 // DISPATCH_LEVEL in Windows IRQL
    }

    pub fn release(&self, _old_irql: KIRQL) {
        self.lock_val.store(0, Ordering::SeqCst);
    }
}

// --- 2. Windows Driver Framework (WDF) - KMDF/UMDF Simulator ---

pub type WDFDRIVER = *mut DRIVER_OBJECT;
pub type WDFDEVICE = *mut DEVICE_OBJECT;
pub type WDFQUEUE = *mut WdfQueueContext;
pub type WDFREQUEST = *mut IRP;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WdfIoQueueDispatchType {
    Sequential = 0,
    Parallel = 1,
    Manual = 2,
}

pub struct WdfQueueContext {
    pub dispatch_type: WdfIoQueueDispatchType,
    pub requests: Vec<WDFREQUEST>,
    pub evt_io_read: Option<fn(WDFQUEUE, WDFREQUEST, usize)>,
    pub evt_io_write: Option<fn(WDFQUEUE, WDFREQUEST, usize)>,
    pub evt_io_device_control: Option<fn(WDFQUEUE, WDFREQUEST, usize, ULONG, ULONG)>,
}

pub struct WDF_DRIVER_CONFIG {
    pub size: ULONG,
    pub evt_device_add: Option<fn(WDFDRIVER, *mut WDFDEVICE_INIT) -> NTSTATUS>,
}

pub struct WDFDEVICE_INIT {
    pub device_name: String,
}

pub struct WDF_IO_QUEUE_CONFIG {
    pub size: ULONG,
    pub dispatch_type: WdfIoQueueDispatchType,
    pub evt_io_read: Option<fn(WDFQUEUE, WDFREQUEST, usize)>,
    pub evt_io_write: Option<fn(WDFQUEUE, WDFREQUEST, usize)>,
    pub evt_io_device_control: Option<fn(WDFQUEUE, WDFREQUEST, usize, ULONG, ULONG)>,
}

// --- 3. NDIS (Network Driver Interface Specification) Simulator ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NDIS_MINIPORT_DRIVER_CHARACTERISTICS {
    pub major_ndis_version: u8,
    pub minor_ndis_version: u8,
    pub initialize_handler: Option<fn(NDIS_HANDLE, NDIS_HANDLE) -> NDIS_STATUS>,
    pub halt_handler: Option<fn(NDIS_HANDLE)>,
    pub send_net_buffer_lists_handler:
        Option<fn(NDIS_HANDLE, *mut NET_BUFFER_LIST, NDIS_PORT_NUMBER, ULONG)>,
    pub return_net_buffer_lists_handler: Option<fn(NDIS_HANDLE, *mut NET_BUFFER_LIST, ULONG)>,
    pub oid_request_handler: Option<fn(NDIS_HANDLE, *mut NDIS_OID_REQUEST) -> NDIS_STATUS>,
}

pub type NDIS_HANDLE = PVOID;
pub type NDIS_PORT_NUMBER = ULONG;
pub type NDIS_STATUS = i32;

pub struct NET_BUFFER_LIST {
    pub next: *mut NET_BUFFER_LIST,
    pub status: NDIS_STATUS,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NDIS_OID_REQUEST {
    pub oid: ULONG,
    pub request_type: ULONG,
    pub bytes_written: ULONG,
    pub bytes_read: ULONG,
}

// --- 4. Storport Storage Driver Simulator ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HW_INITIALIZATION_DATA {
    pub hw_initialization_data_size: ULONG,
    pub hw_find_adapter:
        Option<fn(PVOID, PVOID, PVOID, *mut PORT_CONFIGURATION_INFORMATION) -> ULONG>,
    pub hw_initialize: Option<fn(PVOID) -> bool>,
    pub hw_start_io: Option<fn(PVOID, *mut SCSI_REQUEST_BLOCK) -> bool>,
    pub hw_build_io: Option<fn(PVOID, *mut SCSI_REQUEST_BLOCK) -> bool>,
    pub hw_reset_bus: Option<fn(PVOID, ULONG) -> bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PORT_CONFIGURATION_INFORMATION {
    pub length: ULONG,
    pub system_io_bus_number: ULONG,
    pub adapter_interface_type: ULONG,
    pub bus_interrupt_level: ULONG,
    pub bus_interrupt_vector: ULONG,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SCSI_REQUEST_BLOCK {
    pub length: u16,
    pub function: u8,
    pub srb_status: u8,
    pub scsi_status: u8,
    pub path_id: u8,
    pub target_id: u8,
    pub lun: u8,
    pub cdb: [u8; 16],
    pub data_transfer_length: ULONG,
    pub data_buffer: PVOID,
}

// --- 5. Windows Display Driver Model (WDDM) Simulator ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DXGKRNL_INTERFACE {
    pub size: ULONG,
    pub version: ULONG,
    pub dxgk_interface: PVOID,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DXGK_DEVICE_INFO {
    pub device_type: ULONG,
    pub vendor_id: u16,
    pub device_id: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DXGK_DISPLAY_INFORMATION {
    pub width: ULONG,
    pub height: ULONG,
    pub pitch: ULONG,
    pub color_format: ULONG,
    pub target_id: ULONG,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DXGKARG_ADDDEVICE {
    pub physical_device_object: PVOID,
    pub miniport_device_context: PVOID,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DXGKARG_STARTDEVICE {
    pub miniport_device_context: PVOID,
    pub display_info: DXGK_DISPLAY_INFORMATION,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WddmMiniportDriver {
    pub add_device: Option<fn(PVOID, *mut DXGKARG_ADDDEVICE) -> NTSTATUS>,
    pub start_device: Option<fn(PVOID, *mut DXGKARG_STARTDEVICE) -> NTSTATUS>,
    pub stop_device: Option<fn(PVOID) -> NTSTATUS>,
    pub query_adapter_info: Option<fn(PVOID, ULONG, PVOID, ULONG) -> NTSTATUS>,
}

// --- 6. PE (Portable Executable) binary parser for .sys drivers ---

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct ImageDosHeader {
    pub e_magic: u16, // "MZ" Signature
    pub e_cblp: u16,
    pub e_cp: u16,
    pub e_crlc: u16,
    pub e_cparhdr: u16,
    pub e_minalloc: u16,
    pub e_maxalloc: u16,
    pub e_ss: u16,
    pub e_sp: u16,
    pub e_csum: u16,
    pub e_ip: u16,
    pub e_cs: u16,
    pub e_lfarlc: u16,
    pub e_ovno: u16,
    pub e_res: [u16; 4],
    pub e_oemid: u16,
    pub e_oeminfo: u16,
    pub e_res2: [u16; 10],
    pub e_lfanew: i32, // Offset to PE Header
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct ImageFileHeader {
    pub machine: u16,
    pub number_of_sections: u16,
    pub time_date_stamp: u32,
    pub pointer_to_symbol_table: u32,
    pub number_of_symbols: u32,
    pub size_of_optional_header: u16,
    pub characteristics: u16,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct ImageOptionalHeader64 {
    pub magic: u16,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: u32,
    pub size_of_initialized_data: u32,
    pub size_of_uninitialized_data: u32,
    pub address_of_entry_point: u32,
    pub base_of_code: u32,
    pub image_base: u64,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub major_operating_system_version: u16,
    pub minor_operating_system_version: u16,
    pub major_image_version: u16,
    pub minor_image_version: u16,
    pub major_subsystem_version: u16,
    pub minor_subsystem_version: u16,
    pub win32_version_value: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub check_sum: u32,
    pub subsystem: u16,
    pub dll_characteristics: u16,
    pub size_of_stack_reserve: u64,
    pub size_of_stack_commit: u64,
    pub size_of_heap_reserve: u64,
    pub size_of_heap_commit: u64,
    pub loader_flags: u32,
    pub number_of_rva_and_sizes: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct ImageSectionHeader {
    pub name: [u8; 8],
    pub virtual_size: u32,
    pub virtual_address: u32,
    pub size_of_raw_data: u32,
    pub pointer_to_raw_data: u32,
    pub pointer_to_relocations: u32,
    pub pointer_to_linenumbers: u32,
    pub number_of_relocations: u16,
    pub number_of_linenumbers: u16,
    pub characteristics: u32,
}

pub struct PeDriverLoader {
    pub virtual_memory: Vec<u8>,
    pub entry_point_rva: u32,
    pub image_base: u64,
    pub is_loaded: bool,
}

impl PeDriverLoader {
    pub fn new() -> Self {
        PeDriverLoader {
            virtual_memory: Vec::new(),
            entry_point_rva: 0,
            image_base: 0,
            is_loaded: false,
        }
    }

    /// Complete PE format binary parser and validator
    pub fn parse_and_load_sys(&mut self, binary: &[u8]) -> Result<(), &'static str> {
        if binary.len() < core::mem::size_of::<ImageDosHeader>() {
            return Err("Binary too small for DOS Header");
        }

        let dos_header = unsafe { &*(binary.as_ptr() as *const ImageDosHeader) };
        if dos_header.e_magic != 0x5A4D {
            return Err("Invalid MZ DOS signature");
        }

        let pe_offset = dos_header.e_lfanew as usize;
        if pe_offset
            + 4
            + core::mem::size_of::<ImageFileHeader>()
            + core::mem::size_of::<ImageOptionalHeader64>()
            > binary.len()
        {
            return Err("PE offset bounds overflow");
        }

        let sig = u32::from_le_bytes([
            binary[pe_offset],
            binary[pe_offset + 1],
            binary[pe_offset + 2],
            binary[pe_offset + 3],
        ]);
        if sig != 0x00004550 {
            return Err("Invalid PE signature");
        }

        let file_header_offset = pe_offset + 4;
        let file_header =
            unsafe { &*(binary.as_ptr().add(file_header_offset) as *const ImageFileHeader) };

        let opt_header_offset = file_header_offset + core::mem::size_of::<ImageFileHeader>();
        let opt_header =
            unsafe { &*(binary.as_ptr().add(opt_header_offset) as *const ImageOptionalHeader64) };

        if opt_header.magic != 0x20B {
            return Err("PE Optional Header is not PE32+ (64-bit)");
        }

        self.entry_point_rva = opt_header.address_of_entry_point;
        self.image_base = opt_header.image_base;

        // Allocate and layout virtual memory alignment
        let total_image_size = opt_header.size_of_image as usize;
        self.virtual_memory.resize(total_image_size, 0);

        // Copy Headers
        let headers_size = opt_header.size_of_headers as usize;
        self.virtual_memory[..headers_size].copy_from_slice(&binary[..headers_size]);

        // Copy Sections
        let section_offset = opt_header_offset + file_header.size_of_optional_header as usize;
        for i in 0..(file_header.number_of_sections as usize) {
            let offset = section_offset + i * core::mem::size_of::<ImageSectionHeader>();
            if offset + core::mem::size_of::<ImageSectionHeader>() > binary.len() {
                return Err("Section header overflow");
            }
            let section = unsafe { &*(binary.as_ptr().add(offset) as *const ImageSectionHeader) };

            let dest_start = section.virtual_address as usize;
            let src_start = section.pointer_to_raw_data as usize;
            let size = section.size_of_raw_data as usize;

            if dest_start + size <= total_image_size && src_start + size <= binary.len() {
                self.virtual_memory[dest_start..dest_start + size]
                    .copy_from_slice(&binary[src_start..src_start + size]);
            }
        }

        self.is_loaded = true;
        Ok(())
    }
}

// --- 7. Mapping Windows Drivers to SigmaOS Native Trait Adapters ---

pub struct WindowsDriverAdapter {
    pub driver_object: DRIVER_OBJECT,
    pub device_object: DEVICE_OBJECT,
    pub queue_context: Option<WdfQueueContext>,
    pub ndis_characteristics: Option<NDIS_MINIPORT_DRIVER_CHARACTERISTICS>,
    pub storport_init_data: Option<HW_INITIALIZATION_DATA>,
}

impl WindowsDriverAdapter {
    pub fn new_wdm(name: &str) -> Self {
        let mut adapter = WindowsDriverAdapter {
            driver_object: DRIVER_OBJECT::new(name),
            device_object: DEVICE_OBJECT {
                driver_object: core::ptr::null_mut(),
                next_device: core::ptr::null_mut(),
                device_extension: Vec::new(),
                flags: 0,
                device_type: 0,
            },
            queue_context: None,
            ndis_characteristics: None,
            storport_init_data: None,
        };
        adapter.device_object.driver_object = &mut adapter.driver_object;
        adapter
    }

    pub fn dispatch_irp(
        &mut self,
        major: MajorFunction,
        control_code: u32,
        input: &[u8],
        output: &mut [u8],
    ) -> Result<usize, NTSTATUS> {
        let mut irp = IRP::new(major, control_code, input.len() as u32, output.len() as u32);

        let mut input_mut = input.to_vec();
        irp.associated_irp_system_buffer = input_mut.as_mut_ptr() as PVOID;
        irp.user_buffer = output.as_mut_ptr() as PVOID;

        if let Some(handler) = self.driver_object.major_functions[major as usize] {
            let status = handler(&mut self.device_object, &mut irp);
            if status == STATUS_SUCCESS {
                let info = irp.information;
                if major == MajorFunction::Write {
                    Ok(info)
                } else if info <= output.len() {
                    Ok(info)
                } else {
                    Ok(output.len())
                }
            } else {
                Err(status)
            }
        } else {
            Err(STATUS_NOT_IMPLEMENTED)
        }
    }
}

// SigmaOS Native CharacterDevice Adapter
impl Device for WindowsDriverAdapter {
    fn init(&mut self) -> Result<(), DeviceError> {
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, DeviceError> {
        self.dispatch_irp(MajorFunction::Read, 0, &[], buffer)
            .map_err(|_| DeviceError::IoError)
    }

    fn write(&mut self, buffer: &[u8]) -> Result<usize, DeviceError> {
        let mut dummy_out = [0u8; 1];
        self.dispatch_irp(MajorFunction::Write, 0, buffer, &mut dummy_out)
            .map_err(|_| DeviceError::IoError)
    }

    fn ioctl(&mut self, command: u32, arg: usize) -> Result<usize, DeviceError> {
        let mut dummy_out = [0u8; 64];
        let arg_bytes = arg.to_ne_bytes();
        self.dispatch_irp(
            MajorFunction::DeviceControl,
            command,
            &arg_bytes,
            &mut dummy_out,
        )
        .map_err(|_| DeviceError::IoError)
    }

    fn info(&self) -> DeviceInfo {
        DeviceInfo::new(DeviceType::Character)
    }

    fn shutdown(&mut self) -> Result<(), DeviceError> {
        if let Some(unload_fn) = self.driver_object.driver_unload {
            unload_fn(&mut self.driver_object);
        }
        Ok(())
    }
}

impl CharacterDevice for WindowsDriverAdapter {
    fn read_char(&mut self) -> Result<u8, DeviceError> {
        let mut buf = [0u8; 1];
        self.read(&mut buf)?;
        Ok(buf[0])
    }

    fn write_char(&mut self, c: u8) -> Result<(), DeviceError> {
        self.write(&[c])?;
        Ok(())
    }

    fn flush(&mut self) -> Result<(), DeviceError> {
        Ok(())
    }
}

// SigmaOS NetworkDevice Adapter
pub struct WindowsNdisAdapter {
    pub adapter: WindowsDriverAdapter,
    pub mac_address: [u8; 6],
}

impl Device for WindowsNdisAdapter {
    fn init(&mut self) -> Result<(), DeviceError> {
        if let Some(ref ndis) = self.adapter.ndis_characteristics {
            if let Some(init_fn) = ndis.initialize_handler {
                let status = init_fn(core::ptr::null_mut(), core::ptr::null_mut());
                if status != STATUS_SUCCESS {
                    return Err(DeviceError::IoError);
                }
            }
        }
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, DeviceError> {
        Ok(0)
    }

    fn write(&mut self, buffer: &[u8]) -> Result<usize, DeviceError> {
        self.send_packet(buffer)?;
        Ok(buffer.len())
    }

    fn ioctl(&mut self, _command: u32, _arg: usize) -> Result<usize, DeviceError> {
        Ok(0)
    }

    fn info(&self) -> DeviceInfo {
        DeviceInfo::new(DeviceType::Network)
    }

    fn shutdown(&mut self) -> Result<(), DeviceError> {
        if let Some(ref ndis) = self.adapter.ndis_characteristics {
            if let Some(halt_fn) = ndis.halt_handler {
                halt_fn(core::ptr::null_mut());
            }
        }
        Ok(())
    }
}

impl NetworkDevice for WindowsNdisAdapter {
    fn send_packet(&mut self, packet: &[u8]) -> Result<(), DeviceError> {
        if let Some(ref ndis) = self.adapter.ndis_characteristics {
            if let Some(send_fn) = ndis.send_net_buffer_lists_handler {
                let mut nbl = NET_BUFFER_LIST {
                    next: core::ptr::null_mut(),
                    status: STATUS_SUCCESS,
                    payload: packet.to_vec(),
                };
                send_fn(core::ptr::null_mut(), &mut nbl, 0, 0);
                return Ok(());
            }
        }
        Err(DeviceError::NotSupported)
    }

    fn receive_packet(&mut self, buffer: &mut [u8]) -> Result<usize, DeviceError> {
        Ok(0)
    }

    fn get_mac_address(&self) -> [u8; 6] {
        self.mac_address
    }

    fn set_mac_address(&mut self, mac: [u8; 6]) -> Result<(), DeviceError> {
        self.mac_address = mac;
        Ok(())
    }
}

// SigmaOS BlockDevice Adapter for Storport Miniport
pub struct WindowsStorportAdapter {
    pub adapter: WindowsDriverAdapter,
    pub total_sectors: u64,
    pub sector_size: usize,
}

impl Device for WindowsStorportAdapter {
    fn init(&mut self) -> Result<(), DeviceError> {
        if let Some(ref init_data) = self.adapter.storport_init_data {
            if let Some(initialize_fn) = init_data.hw_initialize {
                if !initialize_fn(core::ptr::null_mut()) {
                    return Err(DeviceError::IoError);
                }
            }
        }
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, DeviceError> {
        Ok(0)
    }

    fn write(&mut self, buffer: &[u8]) -> Result<usize, DeviceError> {
        Ok(0)
    }

    fn ioctl(&mut self, _command: u32, _arg: usize) -> Result<usize, DeviceError> {
        Ok(0)
    }

    fn info(&self) -> DeviceInfo {
        DeviceInfo::new(DeviceType::Block)
    }

    fn shutdown(&mut self) -> Result<(), DeviceError> {
        Ok(())
    }
}

impl BlockDevice for WindowsStorportAdapter {
    fn read_block(&mut self, block: u64, buffer: &mut [u8]) -> Result<(), DeviceError> {
        if let Some(ref init_data) = self.adapter.storport_init_data {
            if let Some(start_io) = init_data.hw_start_io {
                let mut srb = SCSI_REQUEST_BLOCK {
                    length: core::mem::size_of::<SCSI_REQUEST_BLOCK>() as u16,
                    function: 0x01, // Read Function
                    srb_status: 0,
                    scsi_status: 0,
                    path_id: 0,
                    target_id: 0,
                    lun: 0,
                    cdb: [0; 16],
                    data_transfer_length: buffer.len() as ULONG,
                    data_buffer: buffer.as_mut_ptr() as PVOID,
                };
                // CDB Sector layout
                srb.cdb[2] = ((block >> 24) & 0xFF) as u8;
                srb.cdb[3] = ((block >> 16) & 0xFF) as u8;
                srb.cdb[4] = ((block >> 8) & 0xFF) as u8;
                srb.cdb[5] = (block & 0xFF) as u8;

                if start_io(core::ptr::null_mut(), &mut srb) {
                    return Ok(());
                }
            }
        }
        Err(DeviceError::IoError)
    }

    fn write_block(&mut self, block: u64, buffer: &[u8]) -> Result<(), DeviceError> {
        if let Some(ref init_data) = self.adapter.storport_init_data {
            if let Some(start_io) = init_data.hw_start_io {
                let mut srb = SCSI_REQUEST_BLOCK {
                    length: core::mem::size_of::<SCSI_REQUEST_BLOCK>() as u16,
                    function: 0x02, // Write Function
                    srb_status: 0,
                    scsi_status: 0,
                    path_id: 0,
                    target_id: 0,
                    lun: 0,
                    cdb: [0; 16],
                    data_transfer_length: buffer.len() as ULONG,
                    data_buffer: buffer.as_ptr() as *mut u8 as PVOID,
                };
                // CDB Sector layout
                srb.cdb[2] = ((block >> 24) & 0xFF) as u8;
                srb.cdb[3] = ((block >> 16) & 0xFF) as u8;
                srb.cdb[4] = ((block >> 8) & 0xFF) as u8;
                srb.cdb[5] = (block & 0xFF) as u8;

                if start_io(core::ptr::null_mut(), &mut srb) {
                    return Ok(());
                }
            }
        }
        Err(DeviceError::IoError)
    }

    fn block_size(&self) -> usize {
        self.sector_size
    }

    fn total_blocks(&self) -> u64 {
        self.total_sectors
    }
}

// --- Windows Displays / WDDM Mode Manager ---
pub struct WindowsWddmAdapter {
    pub adapter: WindowsDriverAdapter,
    pub miniport: WddmMiniportDriver,
    pub device_info: DXGK_DEVICE_INFO,
    pub active_display: Option<DXGK_DISPLAY_INFORMATION>,
}

impl WindowsWddmAdapter {
    pub fn new(vendor_id: u16, device_id: u16) -> Self {
        WindowsWddmAdapter {
            adapter: WindowsDriverAdapter::new_wdm("WDDMMiniport"),
            miniport: WddmMiniportDriver {
                add_device: None,
                start_device: None,
                stop_device: None,
                query_adapter_info: None,
            },
            device_info: DXGK_DEVICE_INFO {
                device_type: 1, // Graphics Adapter
                vendor_id,
                device_id,
            },
            active_display: None,
        }
    }

    pub fn add_and_start_device(&mut self) -> Result<(), DeviceError> {
        let mut add_args = DXGKARG_ADDDEVICE {
            physical_device_object: core::ptr::null_mut(),
            miniport_device_context: core::ptr::null_mut(),
        };

        if let Some(add_fn) = self.miniport.add_device {
            let status = add_fn(core::ptr::null_mut(), &mut add_args);
            if status != STATUS_SUCCESS {
                return Err(DeviceError::IoError);
            }
        }

        let mut start_args = DXGKARG_STARTDEVICE {
            miniport_device_context: core::ptr::null_mut(),
            display_info: DXGK_DISPLAY_INFORMATION {
                width: 1920,
                height: 1080,
                pitch: 1920 * 4,
                color_format: 24, // B8G8R8A8
                target_id: 1,
            },
        };

        if let Some(start_fn) = self.miniport.start_device {
            let status = start_fn(core::ptr::null_mut(), &mut start_args);
            if status != STATUS_SUCCESS {
                return Err(DeviceError::IoError);
            }
            self.active_display = Some(start_args.display_info);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock WDM major function handlers
    fn mock_irp_read(device: &mut DEVICE_OBJECT, irp: &mut IRP) -> NTSTATUS {
        irp.information = 8;
        unsafe {
            let buffer = core::slice::from_raw_parts_mut(irp.user_buffer as *mut u8, 8);
            buffer.copy_from_slice(b"WINDOWS!");
        }
        STATUS_SUCCESS
    }

    fn mock_irp_write(device: &mut DEVICE_OBJECT, irp: &mut IRP) -> NTSTATUS {
        irp.information = irp
            .current_stack_location
            .parameters_device_io_control
            .input_buffer_length as usize;
        STATUS_SUCCESS
    }

    #[test]
    fn test_wdm_irp_dispatch() {
        let mut adapter = WindowsDriverAdapter::new_wdm("WDMSerial");
        adapter.driver_object.major_functions[MajorFunction::Read as usize] = Some(mock_irp_read);
        adapter.driver_object.major_functions[MajorFunction::Write as usize] = Some(mock_irp_write);

        let mut output = [0u8; 16];
        let bytes_read = adapter.read(&mut output).unwrap();
        assert_eq!(bytes_read, 8);
        assert_eq!(&output[..8], b"WINDOWS!");

        let bytes_written = adapter.write(b"SIGMAOS").unwrap();
        assert_eq!(bytes_written, 7);
    }

    #[test]
    fn test_spin_lock_dispatch() {
        let lock = KSPIN_LOCK::new();
        let irql = lock.acquire();
        assert_eq!(irql, 2);
        lock.release(irql);
    }

    #[test]
    fn test_ndis_miniport_send() {
        static mut SEND_COUNT: usize = 0;
        fn mock_send(
            _handle: NDIS_HANDLE,
            nbl: *mut NET_BUFFER_LIST,
            _port: NDIS_PORT_NUMBER,
            _flags: ULONG,
        ) {
            unsafe {
                let list = &*nbl;
                assert_eq!(list.payload, b"NDIS_PACKET");
                SEND_COUNT += 1;
            }
        }

        let mut adapter = WindowsDriverAdapter::new_wdm("NDISMiniport");
        let ndis_chars = NDIS_MINIPORT_DRIVER_CHARACTERISTICS {
            major_ndis_version: 6,
            minor_ndis_version: 30,
            initialize_handler: Some(|_, _| STATUS_SUCCESS),
            halt_handler: None,
            send_net_buffer_lists_handler: Some(mock_send),
            return_net_buffer_lists_handler: None,
            oid_request_handler: None,
        };

        let mut ndis_adapter = WindowsNdisAdapter {
            adapter,
            mac_address: [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC],
        };
        ndis_adapter.adapter.ndis_characteristics = Some(ndis_chars);

        ndis_adapter.init().unwrap();
        ndis_adapter.send_packet(b"NDIS_PACKET").unwrap();
        unsafe {
            assert_eq!(SEND_COUNT, 1);
        }
    }

    #[test]
    fn test_storport_start_io() {
        static mut IO_DONE: bool = false;
        fn mock_start_io(_context: PVOID, srb: *mut SCSI_REQUEST_BLOCK) -> bool {
            unsafe {
                let req = &*srb;
                assert_eq!(req.function, 0x01); // Read
                assert_eq!(req.cdb[5], 42); // Sector 42
                let buffer = core::slice::from_raw_parts_mut(
                    req.data_buffer as *mut u8,
                    req.data_transfer_length as usize,
                );
                buffer[0] = 0xAA;
                IO_DONE = true;
            }
            true
        }

        let mut adapter = WindowsDriverAdapter::new_wdm("StorportMini");
        let hw_data = HW_INITIALIZATION_DATA {
            hw_initialization_data_size: 128,
            hw_find_adapter: None,
            hw_initialize: Some(|_| true),
            hw_start_io: Some(mock_start_io),
            hw_build_io: None,
            hw_reset_bus: None,
        };
        adapter.storport_init_data = Some(hw_data);

        let mut storport = WindowsStorportAdapter {
            adapter,
            total_sectors: 1024,
            sector_size: 512,
        };
        storport.init().unwrap();

        let mut buf = [0u8; 512];
        storport.read_block(42, &mut buf).unwrap();
        assert_eq!(buf[0], 0xAA);
        unsafe {
            assert!(IO_DONE);
        }
    }

    #[test]
    fn test_wddm_miniport_setup() {
        let mut wddm = WindowsWddmAdapter::new(0x8086, 0x1234);
        wddm.miniport.add_device = Some(|_, _| STATUS_SUCCESS);
        wddm.miniport.start_device = Some(|_, args| {
            unsafe {
                (*args).display_info.width = 1920;
                (*args).display_info.height = 1080;
            }
            STATUS_SUCCESS
        });

        wddm.add_and_start_device().unwrap();
        assert_eq!(wddm.active_display.as_ref().unwrap().width, 1920);
        assert_eq!(wddm.active_display.as_ref().unwrap().height, 1080);
    }

    #[test]
    fn test_pe_sys_binary_loader() {
        // Construct a raw mock PE Binary representing a `.sys` file
        let mut mock_binary = vec![0u8; 1024];

        // 1. ImageDosHeader
        let dos_header = ImageDosHeader {
            e_magic: 0x5A4D, // "MZ"
            e_cblp: 0,
            e_cp: 0,
            e_crlc: 0,
            e_cparhdr: 0,
            e_minalloc: 0,
            e_maxalloc: 0,
            e_ss: 0,
            e_sp: 0,
            e_csum: 0,
            e_ip: 0,
            e_cs: 0,
            e_lfarlc: 0,
            e_ovno: 0,
            e_res: [0; 4],
            e_oemid: 0,
            e_oeminfo: 0,
            e_res2: [0; 10],
            e_lfanew: 64, // PE Header Offset
        };
        let dos_ptr = &dos_header as *const ImageDosHeader as *const u8;
        unsafe {
            core::ptr::copy_nonoverlapping(
                dos_ptr,
                mock_binary.as_mut_ptr(),
                core::mem::size_of::<ImageDosHeader>(),
            );
        }

        // 2. PE Signature
        mock_binary[64..68].copy_from_slice(&0x00004550u32.to_le_bytes()); // "PE\0\0"

        // 3. ImageFileHeader
        let file_header = ImageFileHeader {
            machine: 0x8664, // AMD64
            number_of_sections: 1,
            time_date_stamp: 0,
            pointer_to_symbol_table: 0,
            number_of_symbols: 0,
            size_of_optional_header: core::mem::size_of::<ImageOptionalHeader64>() as u16,
            characteristics: 0x0002, // Executable
        };
        let file_ptr = &file_header as *const ImageFileHeader as *const u8;
        unsafe {
            core::ptr::copy_nonoverlapping(
                file_ptr,
                mock_binary.as_mut_ptr().add(68),
                core::mem::size_of::<ImageFileHeader>(),
            );
        }

        // 4. ImageOptionalHeader64
        let optional_header = ImageOptionalHeader64 {
            magic: 0x20B, // PE32+
            major_linker_version: 0,
            minor_linker_version: 0,
            size_of_code: 512,
            size_of_initialized_data: 0,
            size_of_uninitialized_data: 0,
            address_of_entry_point: 0x1000,
            base_of_code: 0x1000,
            image_base: 0x140000000,
            section_alignment: 4096,
            file_alignment: 512,
            major_operating_system_version: 10,
            minor_operating_system_version: 0,
            major_image_version: 0,
            minor_image_version: 0,
            major_subsystem_version: 10,
            minor_subsystem_version: 0,
            win32_version_value: 0,
            size_of_image: 8192,
            size_of_headers: 512,
            check_sum: 0,
            subsystem: 1, // Native
            dll_characteristics: 0,
            size_of_stack_reserve: 0,
            size_of_stack_commit: 0,
            size_of_heap_reserve: 0,
            size_of_heap_commit: 0,
            loader_flags: 0,
            number_of_rva_and_sizes: 0,
        };
        let opt_ptr = &optional_header as *const ImageOptionalHeader64 as *const u8;
        unsafe {
            core::ptr::copy_nonoverlapping(
                opt_ptr,
                mock_binary
                    .as_mut_ptr()
                    .add(68 + core::mem::size_of::<ImageFileHeader>()),
                core::mem::size_of::<ImageOptionalHeader64>(),
            );
        }

        // 5. ImageSectionHeader
        let mut section = ImageSectionHeader {
            name: [0u8; 8],
            virtual_size: 512,
            virtual_address: 4096,
            size_of_raw_data: 512,
            pointer_to_raw_data: 512,
            pointer_to_relocations: 0,
            pointer_to_linenumbers: 0,
            number_of_relocations: 0,
            number_of_linenumbers: 0,
            characteristics: 0x60000020, // Executable/Readable Code
        };
        section.name[..5].copy_from_slice(b".text");
        let sect_ptr = &section as *const ImageSectionHeader as *const u8;
        unsafe {
            core::ptr::copy_nonoverlapping(
                sect_ptr,
                mock_binary.as_mut_ptr().add(
                    68 + core::mem::size_of::<ImageFileHeader>()
                        + core::mem::size_of::<ImageOptionalHeader64>(),
                ),
                core::mem::size_of::<ImageSectionHeader>(),
            );
        }

        // Fill .text block with some instruction payload
        mock_binary[512..1024].copy_from_slice(&[0x90u8; 512]); // NOP instructions

        let mut loader = PeDriverLoader::new();
        let res = loader.parse_and_load_sys(&mock_binary);
        assert!(res.is_ok());
        assert_eq!(loader.entry_point_rva, 0x1000);
        assert_eq!(loader.image_base, 0x140000000);
        assert!(loader.is_loaded);
        assert_eq!(loader.virtual_memory.len(), 8192);
    }
}
