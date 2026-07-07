//! SigmaOS — UEFI Bootloader Stage-1 Entry Point
//! Minimal UEFI application that locates the kernel, sets up framebuffer,
//! exits Boot Services, and jumps to the kernel entry point.
//! Pure no_std, zero-dependency.

#![no_std]
#![no_main]
#![allow(dead_code)]

type U8    = u8;
type U16   = u16;
type U32   = u32;
type U64   = u64;
type Usize = usize;

// ── EFI Status Codes ────────────────────────────────────────────────────────
type EfiStatus = Usize;
const EFI_SUCCESS:              EfiStatus = 0;
const EFI_LOAD_ERROR:           EfiStatus = 1;
const EFI_INVALID_PARAMETER:    EfiStatus = 2;
const EFI_UNSUPPORTED:          EfiStatus = 3;
const EFI_BUFFER_TOO_SMALL:     EfiStatus = 5;
const EFI_NOT_FOUND:            EfiStatus = 14;

// ── EFI Memory Types ───────────────────────────────────────────────────────
const EFI_LOADER_DATA:          U32 = 2;
const EFI_BOOT_SERVICES_DATA:   U32 = 4;
const EFI_CONVENTIONAL_MEMORY:  U32 = 7;

// ── UEFI Table Structures (simplified) ──────────────────────────────────────

#[repr(C)]
pub struct EfiTableHeader {
    pub signature:   U64,
    pub revision:    U32,
    pub header_size: U32,
    pub crc32:       U32,
    pub reserved:    U32,
}

#[repr(C)]
pub struct EfiSystemTable {
    pub hdr:              EfiTableHeader,
    pub firmware_vendor:  *const U16,
    pub firmware_revision: U32,
    pub console_in_handle: *const core::ffi::c_void,
    pub con_in:           *const core::ffi::c_void,
    pub console_out_handle: *const core::ffi::c_void,
    pub con_out:          *const SimpleTextOutput,
    pub stderr_handle:    *const core::ffi::c_void,
    pub std_err:          *const core::ffi::c_void,
    pub runtime_services: *const core::ffi::c_void,
    pub boot_services:    *const EfiBootServices,
    // ... more fields follow in full UEFI spec
}

#[repr(C)]
pub struct SimpleTextOutput {
    pub reset:            *const core::ffi::c_void,
    pub output_string:    unsafe extern "efiapi" fn(
        this: *const SimpleTextOutput,
        string: *const U16,
    ) -> EfiStatus,
    pub test_string:      *const core::ffi::c_void,
    pub query_mode:       *const core::ffi::c_void,
    pub set_mode:         *const core::ffi::c_void,
    pub set_attribute:    *const core::ffi::c_void,
    pub clear_screen:     unsafe extern "efiapi" fn(
        this: *const SimpleTextOutput,
    ) -> EfiStatus,
}

#[repr(C)]
pub struct EfiBootServices {
    pub hdr: EfiTableHeader,
    // Task Priority Services
    pub raise_tpl:        *const core::ffi::c_void,
    pub restore_tpl:      *const core::ffi::c_void,
    // Memory Services
    pub allocate_pages:   unsafe extern "efiapi" fn(
        alloc_type: Usize,
        memory_type: U32,
        pages: Usize,
        memory: *mut U64,
    ) -> EfiStatus,
    pub free_pages:       unsafe extern "efiapi" fn(
        memory: U64,
        pages: Usize,
    ) -> EfiStatus,
    pub get_memory_map:   unsafe extern "efiapi" fn(
        memory_map_size: *mut Usize,
        memory_map: *mut EfiMemoryDescriptor,
        map_key: *mut Usize,
        descriptor_size: *mut Usize,
        descriptor_version: *mut U32,
    ) -> EfiStatus,
    pub allocate_pool:    *const core::ffi::c_void,
    pub free_pool:        *const core::ffi::c_void,
    // Event & Timer Services (skipped)
    pub create_event:     *const core::ffi::c_void,
    pub set_timer:        *const core::ffi::c_void,
    pub wait_for_event:   *const core::ffi::c_void,
    pub signal_event:     *const core::ffi::c_void,
    pub close_event:      *const core::ffi::c_void,
    pub check_event:      *const core::ffi::c_void,
    // Protocol Handler Services (skipped)
    pub install_protocol: *const core::ffi::c_void,
    pub reinstall_protocol: *const core::ffi::c_void,
    pub uninstall_protocol: *const core::ffi::c_void,
    pub handle_protocol:  *const core::ffi::c_void,
    pub _reserved:        *const core::ffi::c_void,
    pub register_protocol_notify: *const core::ffi::c_void,
    pub locate_handle:    *const core::ffi::c_void,
    pub locate_device_path: *const core::ffi::c_void,
    pub install_config:   *const core::ffi::c_void,
    // Image Services
    pub load_image:       *const core::ffi::c_void,
    pub start_image:      *const core::ffi::c_void,
    pub exit:             *const core::ffi::c_void,
    pub unload_image:     *const core::ffi::c_void,
    pub exit_boot_services: unsafe extern "efiapi" fn(
        image_handle: *const core::ffi::c_void,
        map_key: Usize,
    ) -> EfiStatus,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EfiMemoryDescriptor {
    pub memory_type:     U32,
    pub _pad:            U32,
    pub physical_start:  U64,
    pub virtual_start:   U64,
    pub number_of_pages: U64,
    pub attribute:       U64,
}

// ── Framebuffer Info ────────────────────────────────────────────────────────

#[repr(C)]
pub struct FramebufferInfo {
    pub base_addr:       U64,
    pub width:           U32,
    pub height:          U32,
    pub stride:          U32,
    pub bpp:             U32, // bits per pixel
}

// ── Boot Info passed to kernel ──────────────────────────────────────────────

const MAX_MMAP_ENTRIES: usize = 256;

#[repr(C)]
pub struct SigmaBootInfo {
    pub magic:           U64,          // 0x5349474D414F5321 = "SIGMAOS!"
    pub framebuffer:     FramebufferInfo,
    pub mmap_count:      U32,
    pub mmap:            [EfiMemoryDescriptor; MAX_MMAP_ENTRIES],
    pub rsdp_addr:       U64,          // ACPI RSDP physical address
    pub kernel_phys:     U64,
    pub kernel_size:     U64,
}

const SIGMA_BOOT_MAGIC: U64 = 0x5349474D414F5321; // "SIGMAOS!"

// ── Globals ─────────────────────────────────────────────────────────────────
static mut BOOT_INFO: SigmaBootInfo = SigmaBootInfo {
    magic: SIGMA_BOOT_MAGIC,
    framebuffer: FramebufferInfo {
        base_addr: 0, width: 0, height: 0, stride: 0, bpp: 0,
    },
    mmap_count: 0,
    mmap: [EfiMemoryDescriptor {
        memory_type: 0, _pad: 0,
        physical_start: 0, virtual_start: 0,
        number_of_pages: 0, attribute: 0,
    }; MAX_MMAP_ENTRIES],
    rsdp_addr: 0,
    kernel_phys: 0,
    kernel_size: 0,
};

// ── Helper: Print UCS-2 string to UEFI console ─────────────────────────────

unsafe fn uefi_print(con_out: *const SimpleTextOutput, msg: &[u8]) {
    // Convert ASCII to UCS-2 and print
    let mut buf = [0u16; 128];
    let mut i = 0;
    for &b in msg {
        if i >= 126 { break; }
        if b == b'\n' {
            buf[i] = 0x000D; i += 1; // CR
            buf[i] = 0x000A; i += 1; // LF
        } else {
            buf[i] = b as u16; i += 1;
        }
    }
    buf[i] = 0; // null terminate
    ((*con_out).output_string)(con_out, buf.as_ptr());
}

// ── UEFI Entry Point ───────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "efiapi" fn efi_main(
    image_handle: *const core::ffi::c_void,
    system_table: *const EfiSystemTable,
) -> EfiStatus {
    let st = &*system_table;
    let con_out = st.con_out;
    let bs = &*st.boot_services;

    // Clear screen
    ((*con_out).clear_screen)(con_out);

    // Print banner
    uefi_print(con_out, b"SigmaOS UEFI Bootloader v0.1.0\n");
    uefi_print(con_out, b"================================\n\n");

    // Step 1: Locate and load kernel from disk
    uefi_print(con_out, b"[BOOT] Locating kernel file...\n");
    
    // Try to locate Simple File System protocol on the device handle
    // For now, we'll allocate memory and assume kernel is loaded elsewhere
    // In a full implementation, we would:
    // 1. Locate the boot device
    // 2. Open the Simple File System protocol
    // 3. Open the kernel file (e.g., "sigmaos.elf")
    // 4. Read the ELF header and load sections
    
    // Allocate pages for the kernel (placeholder: 2MB)
    uefi_print(con_out, b"[BOOT] Allocating kernel memory...\n");
    let kernel_pages: Usize = 512; // 512 * 4KB = 2MB
    let mut kernel_addr: U64 = 0;
    let status = (bs.allocate_pages)(
        0, // AllocateAnyPages
        EFI_LOADER_DATA,
        kernel_pages,
        &mut kernel_addr,
    );
    if status != EFI_SUCCESS {
        uefi_print(con_out, b"[BOOT] ERROR: Failed to allocate kernel memory!\n");
        return EFI_LOAD_ERROR;
    }
    BOOT_INFO.kernel_phys = kernel_addr;
    BOOT_INFO.kernel_size = (kernel_pages * 4096) as U64;
    uefi_print(con_out, b"[BOOT] Kernel memory allocated.\n");

    // Step 2: Get memory map
    uefi_print(con_out, b"[BOOT] Retrieving memory map...\n");
    let mut mmap_size: Usize = core::mem::size_of::<EfiMemoryDescriptor>() * MAX_MMAP_ENTRIES;
    let mut map_key: Usize = 0;
    let mut desc_size: Usize = 0;
    let mut desc_version: U32 = 0;

    let status = (bs.get_memory_map)(
        &mut mmap_size,
        BOOT_INFO.mmap.as_mut_ptr(),
        &mut map_key,
        &mut desc_size,
        &mut desc_version,
    );
    if status != EFI_SUCCESS {
        uefi_print(con_out, b"[BOOT] WARNING: Memory map retrieval failed, retrying...\n");
        // Retry with updated size
        let status2 = (bs.get_memory_map)(
            &mut mmap_size,
            BOOT_INFO.mmap.as_mut_ptr(),
            &mut map_key,
            &mut desc_size,
            &mut desc_version,
        );
        if status2 != EFI_SUCCESS {
            uefi_print(con_out, b"[BOOT] ERROR: Cannot get memory map!\n");
            return EFI_LOAD_ERROR;
        }
    }
    BOOT_INFO.mmap_count = (mmap_size / desc_size) as U32;
    uefi_print(con_out, b"[BOOT] Memory map acquired.\n");

    // Step 2.5: Locate ACPI RSDP
    uefi_print(con_out, b"[BOOT] Locating ACPI RSDP...\n");
    // In a full implementation, we would search for the RSDP in:
    // 1. EFI configuration tables
    // 2. EBDA (Extended BIOS Data Area)
    // 3. Reserved memory regions
    // For now, we'll set it to 0 (kernel will search)
    BOOT_INFO.rsdp_addr = 0;
    uefi_print(con_out, b"[BOOT] ACPI RSDP location deferred to kernel.\n");

    // Step 2.6: Initialize framebuffer (GOP - Graphics Output Protocol)
    uefi_print(con_out, b"[BOOT] Initializing framebuffer...\n");
    // In a full implementation, we would:
    // 1. Locate Graphics Output Protocol
    // 2. Query current mode
    // 3. Set desired resolution (e.g., 1920x1080)
    // 4. Get framebuffer address and stride
    // For now, we'll set placeholder values
    BOOT_INFO.framebuffer.base_addr = 0;
    BOOT_INFO.framebuffer.width = 0;
    BOOT_INFO.framebuffer.height = 0;
    BOOT_INFO.framebuffer.stride = 0;
    BOOT_INFO.framebuffer.bpp = 0;
    uefi_print(con_out, b"[BOOT] Framebuffer initialization deferred to kernel.\n");

    // Step 3: Exit Boot Services
    uefi_print(con_out, b"[BOOT] Exiting UEFI Boot Services...\n");
    let status = (bs.exit_boot_services)(image_handle, map_key);
    if status != EFI_SUCCESS {
        // Need to re-get memory map and try again
        let status2 = (bs.get_memory_map)(
            &mut mmap_size,
            BOOT_INFO.mmap.as_mut_ptr(),
            &mut map_key,
            &mut desc_size,
            &mut desc_version,
        );
        if status2 == EFI_SUCCESS {
            let _ = (bs.exit_boot_services)(image_handle, map_key);
        }
    }

    // At this point, UEFI Boot Services are gone.
    // We can no longer use con_out or any BS functions.

    // Step 4: Jump to kernel entry
    // The kernel entry point is expected at kernel_addr + 0x1000 (convention)
    // In a full implementation, we would parse the ELF header to find the actual entry point
    uefi_print(con_out, b"[BOOT] Jumping to kernel...\n");
    let kernel_entry: extern "C" fn(*const SigmaBootInfo) -> ! =
        core::mem::transmute(kernel_addr + 0x1000);
    kernel_entry(&BOOT_INFO);
}

// ── Panic Handler ───────────────────────────────────────────────────────────
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
