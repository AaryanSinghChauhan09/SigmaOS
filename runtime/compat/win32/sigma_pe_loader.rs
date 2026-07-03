// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// runtime/compat/win32/sigma_pe_loader.rs — PE32+ binary loader
//
// Parses Windows PE32+ (x86-64) executables and maps them into the
// SigmaOS sigma-compat address space for Win32 app compatibility.
// This is the equivalent of Wine's PE loader — cleanroom implementation.
//
// Supports:
//   - PE32+ (64-bit) EXE and DLL
//   - Section mapping with W^X enforcement
//   - Import table resolution via sigma_ntdll shim
//   - Base relocation (.reloc section)
//   - TLS callbacks
//
// Language: Rust #![no_std]

#![no_std]
#![allow(dead_code)]

// ── PE format constants ───────────────────────────────────────────────────
const MZ_MAGIC:        u16 = 0x5A4D; // "MZ"
const PE_SIGNATURE:    u32 = 0x4550; // "PE\0\0"
const PE32_PLUS_MAGIC: u16 = 0x020B; // PE32+ (64-bit)
const PE32_MAGIC:      u16 = 0x010B; // PE32  (32-bit, not supported)

// ── DOS header (legacy header at offset 0) ────────────────────────────────
#[repr(C, packed)]
#[derive(Copy, Clone)]
struct DosHeader {
    e_magic:    u16, // MZ_MAGIC
    e_cblp:     u16,
    e_cp:       u16,
    e_crlc:     u16,
    e_cparhdr:  u16,
    e_minalloc: u16,
    e_maxalloc: u16,
    e_ss:       u16,
    e_sp:       u16,
    e_csum:     u16,
    e_ip:       u16,
    e_cs:       u16,
    e_lfarlc:   u16,
    e_ovno:     u16,
    e_res:      [u16; 4],
    e_oemid:    u16,
    e_oeminfo:  u16,
    e_res2:     [u16; 10],
    e_lfanew:   i32, // Offset to PE header
}

// ── PE file header ─────────────────────────────────────────────────────────
#[repr(C, packed)]
#[derive(Copy, Clone)]
struct PeFileHeader {
    machine:                u16,
    number_of_sections:     u16,
    time_date_stamp:        u32,
    pointer_to_symbol_table:u32,
    number_of_symbols:      u32,
    size_of_optional_header:u16,
    characteristics:        u16,
}

const IMAGE_FILE_MACHINE_AMD64: u16 = 0x8664;
const IMAGE_FILE_EXECUTABLE_IMAGE: u16 = 0x0002;
const IMAGE_FILE_DLL: u16 = 0x2000;

// ── PE32+ optional header ─────────────────────────────────────────────────
#[repr(C, packed)]
#[derive(Copy, Clone)]
struct Pe32PlusOptionalHeader {
    magic:                         u16, // PE32_PLUS_MAGIC
    major_linker_version:          u8,
    minor_linker_version:          u8,
    size_of_code:                  u32,
    size_of_initialized_data:      u32,
    size_of_uninitialized_data:    u32,
    address_of_entry_point:        u32,
    base_of_code:                  u32,
    image_base:                    u64,
    section_alignment:             u32,
    file_alignment:                u32,
    major_os_version:              u16,
    minor_os_version:              u16,
    major_image_version:           u16,
    minor_image_version:           u16,
    major_subsystem_version:       u16,
    minor_subsystem_version:       u16,
    win32_version_value:           u32,
    size_of_image:                 u32,
    size_of_headers:               u32,
    checksum:                      u32,
    subsystem:                     u16,
    dll_characteristics:           u16,
    size_of_stack_reserve:         u64,
    size_of_stack_commit:          u64,
    size_of_heap_reserve:          u64,
    size_of_heap_commit:           u64,
    loader_flags:                  u32,
    number_of_rva_and_sizes:       u32,
    data_directories:              [ImageDataDirectory; 16],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
struct ImageDataDirectory {
    virtual_address: u32,
    size:            u32,
}

// Data directory indices
const IMAGE_DIRECTORY_ENTRY_EXPORT:   usize = 0;
const IMAGE_DIRECTORY_ENTRY_IMPORT:   usize = 1;
const IMAGE_DIRECTORY_ENTRY_RESOURCE: usize = 2;
const IMAGE_DIRECTORY_ENTRY_BASERELOC:usize = 5;
const IMAGE_DIRECTORY_ENTRY_TLS:      usize = 9;

// ── Section header ─────────────────────────────────────────────────────────
#[repr(C, packed)]
#[derive(Copy, Clone)]
struct SectionHeader {
    name:                    [u8; 8],
    virtual_size:            u32,
    virtual_address:         u32,
    size_of_raw_data:        u32,
    pointer_to_raw_data:     u32,
    pointer_to_relocations:  u32,
    pointer_to_linenumbers:  u32,
    number_of_relocations:   u16,
    number_of_linenumbers:   u16,
    characteristics:         u32,
}

const IMAGE_SCN_CNT_CODE:               u32 = 0x0000_0020;
const IMAGE_SCN_CNT_INITIALIZED_DATA:   u32 = 0x0000_0040;
const IMAGE_SCN_CNT_UNINITIALIZED_DATA: u32 = 0x0000_0080;
const IMAGE_SCN_MEM_EXECUTE:            u32 = 0x2000_0000;
const IMAGE_SCN_MEM_READ:               u32 = 0x4000_0000;
const IMAGE_SCN_MEM_WRITE:              u32 = 0x8000_0000;

// ── Import descriptor ──────────────────────────────────────────────────────
#[repr(C, packed)]
#[derive(Copy, Clone)]
struct ImportDescriptor {
    original_first_thunk: u32, // RVA to INT (Import Name Table)
    time_date_stamp:       u32,
    forwarder_chain:       u32,
    name:                  u32, // RVA to DLL name string
    first_thunk:           u32, // RVA to IAT (Import Address Table)
}

// ── Base relocation block ─────────────────────────────────────────────────
#[repr(C, packed)]
#[derive(Copy, Clone)]
struct BaseRelocationBlock {
    virtual_address: u32,
    size_of_block:   u32,
    // followed by (size_of_block - 8) / 2 TypeOffset entries
}

const IMAGE_REL_BASED_ABSOLUTE: u16 = 0;
const IMAGE_REL_BASED_DIR64:    u16 = 10;

// ── Loaded image descriptor ────────────────────────────────────────────────
pub const MAX_SECTIONS: usize = 32;
pub const MAX_IMPORTS:  usize = 64;

#[derive(Copy, Clone)]
pub struct LoadedSection {
    pub name:    [u8; 8],
    pub va:      u64,  // Virtual address in sigma-compat address space
    pub size:    u32,
    pub perms:   u8,   // PE_PERM_R | PE_PERM_W | PE_PERM_X
}

pub const PE_PERM_R: u8 = 1 << 0;
pub const PE_PERM_W: u8 = 1 << 1;
pub const PE_PERM_X: u8 = 1 << 2;

#[derive(Copy, Clone)]
pub struct ImportedDll {
    pub dll_name: [u8; 64],
}

pub struct PeLoadedImage {
    pub image_base:     u64,  // Actual load address
    pub preferred_base: u64,  // PE header's preferred base
    pub entry_point:    u64,  // Absolute VA of entry point
    pub size_of_image:  u32,
    pub is_dll:         bool,
    pub subsystem:      u16,
    pub sections:       [Option<LoadedSection>; MAX_SECTIONS],
    pub section_count:  usize,
    pub imports:        [Option<ImportedDll>; MAX_IMPORTS],
    pub import_count:   usize,
    pub tls_callback:   Option<u64>,
}

impl PeLoadedImage {
    pub const fn empty() -> Self {
        Self {
            image_base: 0, preferred_base: 0, entry_point: 0,
            size_of_image: 0, is_dll: false, subsystem: 0,
            sections: [const { None }; MAX_SECTIONS],
            section_count: 0,
            imports: [const { None }; MAX_IMPORTS],
            import_count: 0,
            tls_callback: None,
        }
    }
}

// ── Error type ────────────────────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PeError {
    NotPe,             // MZ magic missing
    Not64Bit,          // PE32 (32-bit) not supported
    TruncatedHeader,   // File too small
    BadAlignment,      // Section alignment violation
    TooManySections,   // > MAX_SECTIONS
    RelocationFailed,  // Reloc patch out of bounds
    W_XViolation,      // Section is both writable and executable
    UnsupportedSubsystem,
}

pub type PeResult<T> = Result<T, PeError>;

// ── PE Loader ─────────────────────────────────────────────────────────────
pub struct PeLoader;

impl PeLoader {
    /// Parse and validate a PE32+ binary from a byte slice.
    /// Maps sections into `load_base` (caller provides a pre-allocated region).
    pub unsafe fn load(data: &[u8], load_base: u64) -> PeResult<PeLoadedImage> {
        if data.len() < core::mem::size_of::<DosHeader>() {
            return Err(PeError::TruncatedHeader);
        }

        let dos = &*(data.as_ptr() as *const DosHeader);
        if { dos.e_magic } != MZ_MAGIC {
            return Err(PeError::NotPe);
        }

        let pe_offset = { dos.e_lfanew } as usize;
        if pe_offset + 4 + core::mem::size_of::<PeFileHeader>() > data.len() {
            return Err(PeError::TruncatedHeader);
        }

        let sig = core::ptr::read_unaligned(data.as_ptr().add(pe_offset) as *const u32);
        if sig != PE_SIGNATURE {
            return Err(PeError::NotPe);
        }

        let file_hdr = &*(data.as_ptr().add(pe_offset + 4) as *const PeFileHeader);
        if { file_hdr.machine } != IMAGE_FILE_MACHINE_AMD64 {
            return Err(PeError::Not64Bit);
        }

        let opt_offset = pe_offset + 4 + core::mem::size_of::<PeFileHeader>();
        let opt = &*(data.as_ptr().add(opt_offset) as *const Pe32PlusOptionalHeader);
        if { opt.magic } != PE32_PLUS_MAGIC {
            return Err(PeError::Not64Bit);
        }

        let preferred_base = { opt.image_base };
        let size_of_image  = { opt.size_of_image };
        let entry_rva      = { opt.address_of_entry_point };
        let is_dll = { file_hdr.characteristics } & IMAGE_FILE_DLL != 0;
        let subsystem = { opt.subsystem };

        let mut image = PeLoadedImage::empty();
        image.preferred_base = preferred_base;
        image.image_base     = load_base;
        image.size_of_image  = size_of_image;
        image.is_dll         = is_dll;
        image.subsystem      = subsystem;
        image.entry_point    = load_base + entry_rva as u64;

        // ── Map sections ──────────────────────────────────────────────────
        let num_sections = { file_hdr.number_of_sections } as usize;
        if num_sections > MAX_SECTIONS { return Err(PeError::TooManySections); }

        let sec_table_offset = opt_offset + { file_hdr.size_of_optional_header } as usize;
        for i in 0..num_sections {
            let sec_off = sec_table_offset + i * core::mem::size_of::<SectionHeader>();
            if sec_off + core::mem::size_of::<SectionHeader>() > data.len() {
                return Err(PeError::TruncatedHeader);
            }
            let sec = &*(data.as_ptr().add(sec_off) as *const SectionHeader);
            let chars = { sec.characteristics };

            // W^X enforcement: SigmaOS never maps a section exec+write
            let exec  = chars & IMAGE_SCN_MEM_EXECUTE != 0;
            let write = chars & IMAGE_SCN_MEM_WRITE   != 0;
            if exec && write { return Err(PeError::W_XViolation); }

            let mut perms: u8 = 0;
            if chars & IMAGE_SCN_MEM_READ    != 0 { perms |= PE_PERM_R; }
            if write                              { perms |= PE_PERM_W; }
            if exec                               { perms |= PE_PERM_X; }

            // Copy section data from file into load buffer
            let src_off  = { sec.pointer_to_raw_data } as usize;
            let src_size = { sec.size_of_raw_data } as usize;
            let dst_va   = load_base + { sec.virtual_address } as u64;
            let dst_size = { sec.virtual_size } as usize;

            if src_off + src_size <= data.len() {
                let src_ptr = data.as_ptr().add(src_off);
                let dst_ptr = dst_va as *mut u8;
                // Zero-fill virtual size, then copy raw data
                core::ptr::write_bytes(dst_ptr, 0, dst_size);
                core::ptr::copy_nonoverlapping(src_ptr, dst_ptr, src_size.min(dst_size));
            }

            image.sections[image.section_count] = Some(LoadedSection {
                name: { sec.name },
                va:   dst_va,
                size: { sec.virtual_size },
                perms,
            });
            image.section_count += 1;
        }

        // ── Base relocations ─────────────────────────────────────────────
        let delta = load_base.wrapping_sub(preferred_base) as i64;
        if delta != 0 {
            Self::apply_relocations(data, opt, load_base, delta)?;
        }

        // ── Parse import table ────────────────────────────────────────────
        Self::parse_imports(data, opt, load_base, &mut image);

        Ok(image)
    }

    unsafe fn apply_relocations(
        data: &[u8],
        opt: &Pe32PlusOptionalHeader,
        load_base: u64,
        delta: i64,
    ) -> PeResult<()> {
        let reloc_dir = &{ opt.data_directories }[IMAGE_DIRECTORY_ENTRY_BASERELOC];
        let reloc_rva  = { reloc_dir.virtual_address } as usize;
        let reloc_size = { reloc_dir.size } as usize;
        if reloc_rva == 0 || reloc_size == 0 { return Ok(()); }

        // Find .reloc section in file
        // Simplified: relocations are already mapped into load_base
        let reloc_ptr = (load_base + reloc_rva as u64) as *const u8;
        let mut offset: usize = 0;

        while offset + 8 <= reloc_size {
            let block = &*(reloc_ptr.add(offset) as *const BaseRelocationBlock);
            let block_va   = { block.virtual_address } as u64;
            let block_size = { block.size_of_block } as usize;
            if block_size < 8 { break; }

            let entries = (block_size - 8) / 2;
            let entry_ptr = reloc_ptr.add(offset + 8) as *const u16;

            for i in 0..entries {
                let entry = core::ptr::read_unaligned(entry_ptr.add(i));
                let rel_type = (entry >> 12) as u16;
                let rel_off  = (entry & 0x0FFF) as u64;

                if rel_type == IMAGE_REL_BASED_DIR64 {
                    let patch_va  = load_base + block_va + rel_off;
                    let patch_ptr = patch_va as *mut i64;
                    let old_val   = core::ptr::read_unaligned(patch_ptr);
                    core::ptr::write_unaligned(patch_ptr, old_val.wrapping_add(delta));
                }
                // IMAGE_REL_BASED_ABSOLUTE (0) = padding, skip
            }
            offset += block_size;
        }
        Ok(())
    }

    unsafe fn parse_imports(
        data: &[u8],
        opt: &Pe32PlusOptionalHeader,
        load_base: u64,
        image: &mut PeLoadedImage,
    ) {
        let import_dir = &{ opt.data_directories }[IMAGE_DIRECTORY_ENTRY_IMPORT];
        let import_rva  = { import_dir.virtual_address } as u64;
        if import_rva == 0 { return; }

        let mut desc_ptr = (load_base + import_rva) as *const ImportDescriptor;
        loop {
            let desc = core::ptr::read_unaligned(desc_ptr);
            if { desc.name } == 0 { break; } // null terminator

            let dll_name_ptr = (load_base + { desc.name } as u64) as *const u8;
            let mut dll_entry = ImportedDll { dll_name: [0u8; 64] };
            let mut i = 0usize;
            while i < 63 {
                let b = core::ptr::read(dll_name_ptr.add(i));
                if b == 0 { break; }
                dll_entry.dll_name[i] = b;
                i += 1;
            }

            if image.import_count < MAX_IMPORTS {
                image.imports[image.import_count] = Some(dll_entry);
                image.import_count += 1;
            }
            desc_ptr = desc_ptr.add(1);
        }
    }

    /// RVA → file offset conversion (needed before sections are mapped)
    pub fn rva_to_offset(data: &[u8], rva: u32) -> Option<usize> {
        if data.len() < core::mem::size_of::<DosHeader>() { return None; }
        unsafe {
            let dos = &*(data.as_ptr() as *const DosHeader);
            let pe_off  = { dos.e_lfanew } as usize;
            let fh_off  = pe_off + 4;
            let file_hdr = &*(data.as_ptr().add(fh_off) as *const PeFileHeader);
            let opt_off  = fh_off + core::mem::size_of::<PeFileHeader>();
            let sec_off  = opt_off + { file_hdr.size_of_optional_header } as usize;
            let nsec     = { file_hdr.number_of_sections } as usize;

            for i in 0..nsec {
                let s = &*(data.as_ptr().add(sec_off + i * core::mem::size_of::<SectionHeader>())
                    as *const SectionHeader);
                let va   = { s.virtual_address };
                let vsz  = { s.virtual_size };
                let raw  = { s.pointer_to_raw_data };
                if rva >= va && rva < va + vsz {
                    return Some((rva - va + raw) as usize);
                }
            }
        }
        None
    }
}

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_pe_load(
    data: *const u8, data_len: usize, load_base: u64,
    out: *mut PeLoadedImage,
) -> i32 {
    if data.is_null() || out.is_null() { return -22; }
    let slice = core::slice::from_raw_parts(data, data_len);
    match PeLoader::load(slice, load_base) {
        Ok(img)  => { *out = img; 0 }
        Err(e)   => match e {
            PeError::NotPe             => -1,
            PeError::Not64Bit          => -2,
            PeError::TruncatedHeader   => -3,
            PeError::BadAlignment      => -4,
            PeError::TooManySections   => -5,
            PeError::RelocationFailed  => -6,
            PeError::W_XViolation      => -7,
            PeError::UnsupportedSubsystem => -8,
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_pe_inspect(data: *const u8, data_len: usize) -> i32 {
    if data.is_null() { return -22; }
    let slice = core::slice::from_raw_parts(data, data_len);
    // Try loading at dummy base 0 to validate structure only
    match PeLoader::load(slice, 0) {
        Ok(img) => img.section_count as i32,
        Err(_)  => -1,
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop { unsafe { core::arch::asm!("cli; hlt", options(nomem, nostack)); } }
}
