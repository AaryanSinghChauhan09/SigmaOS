#![no_std]
#![no_main]

/// Custom ELF Loader for SigmaOS
/// Implements ELF binary loading without relying on ld.so
/// Supports ELF32 and ELF64 formats

use core::ptr::{self, NonNull};
use core::mem;

/// ELF magic number
const ELF_MAGIC: [u8; 4] = [0x7f, b'E', b'L', b'F'];

/// ELF class (32-bit or 64-bit)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ElfClass {
    ELFCLASSNONE = 0,
    ELFCLASS32 = 1,
    ELFCLASS64 = 2,
}

/// ELF data encoding
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ElfData {
    ELFDATANONE = 0,
    ELFDATA2LSB = 1, // Little endian
    ELFDATA2MSB = 2, // Big endian
}

/// ELF file type
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ElfType {
    ET_NONE = 0,
    ET_REL = 1,   // Relocatable
    ET_EXEC = 2,  // Executable
    ET_DYN = 3,   // Shared object
    ET_CORE = 4,  // Core file
}

/// ELF machine architecture
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ElfMachine {
    EM_NONE = 0,
    EM_386 = 3,
    EM_X86_64 = 62,
    EM_ARM = 40,
    EM_AARCH64 = 183,
}

/// ELF header (common for 32 and 64 bit)
#[repr(C)]
pub struct ElfHeader {
    e_ident: [u8; 16],
    e_type: u16,
    e_machine: u16,
    e_version: u32,
}

/// ELF32 header
#[repr(C)]
pub struct Elf32Header {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u32,
    pub e_phoff: u32,
    pub e_shoff: u32,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

/// ELF64 header
#[repr(C)]
pub struct Elf64Header {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

/// Program header type
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PhType {
    PT_NULL = 0,
    PT_LOAD = 1,
    PT_DYNAMIC = 2,
    PT_INTERP = 3,
    PT_NOTE = 4,
    PT_SHLIB = 5,
    PT_PHDR = 6,
}

/// ELF32 program header
#[repr(C)]
pub struct Elf32Phdr {
    pub p_type: u32,
    pub p_offset: u32,
    pub p_vaddr: u32,
    pub p_paddr: u32,
    pub p_filesz: u32,
    pub p_memsz: u32,
    pub p_flags: u32,
    pub p_align: u32,
}

/// ELF64 program header
#[repr(C)]
pub struct Elf64Phdr {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

/// Section header type
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ShType {
    SHT_NULL = 0,
    SHT_PROGBITS = 1,
    SHT_SYMTAB = 2,
    SHT_STRTAB = 3,
    SHT_RELA = 4,
    SHT_HASH = 5,
    SHT_DYNAMIC = 6,
    SHT_NOTE = 7,
    SHT_NOBITS = 8,
    SHT_REL = 9,
    SHT_DYNSYM = 11,
}

/// ELF32 section header
#[repr(C)]
pub struct Elf32Shdr {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: u32,
    pub sh_addr: u32,
    pub sh_offset: u32,
    pub sh_size: u32,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u32,
    pub sh_entsize: u32,
}

/// ELF64 section header
#[repr(C)]
pub struct Elf64Shdr {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: u64,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
}

/// ELF binary
pub struct ElfBinary {
    data: *const u8,
    size: usize,
    is_64bit: bool,
    entry_point: u64,
}

impl ElfBinary {
    /// Create ELF binary from data
    pub unsafe fn new(data: *const u8, size: usize) -> Option<Self> {
        if size < 64 {
            return None;
        }

        // Check magic number
        let ident = &*(data as *const [u8; 16]);
        if ident[0..4] != ELF_MAGIC {
            return None;
        }

        // Check class
        let is_64bit = match ident[4] {
            1 => false, // ELFCLASS32
            2 => true,  // ELFCLASS64
            _ => return None,
        };

        // Get entry point
        let entry_point = if is_64bit {
            let header = &*(data as *const Elf64Header);
            header.e_entry
        } else {
            let header = &*(data as *const Elf32Header);
            header.e_entry as u64
        };

        Some(ElfBinary {
            data,
            size,
            is_64bit,
            entry_point,
        })
    }

    /// Check if binary is 64-bit
    pub fn is_64bit(&self) -> bool {
        self.is_64bit
    }

    /// Get entry point
    pub fn entry_point(&self) -> u64 {
        self.entry_point
    }

    /// Load program headers
    pub unsafe fn load_program_headers(&self) -> Result<(), ElfError> {
        if self.is_64bit {
            self.load_program_headers_64()
        } else {
            self.load_program_headers_32()
        }
    }

    unsafe fn load_program_headers_64(&self) -> Result<(), ElfError> {
        let header = &*(self.data as *const Elf64Header);
        let phoff = header.e_phoff as usize;
        let phentsize = header.e_phentsize as usize;
        let phnum = header.e_phnum as usize;

        for i in 0..phnum {
            let phdr_ptr = (self.data as usize + phoff + i * phentsize) as *const Elf64Phdr;
            let phdr = &*phdr_ptr;
            self.load_segment(phdr)?;
        }

        Ok(())
    }

    unsafe fn load_program_headers_32(&self) -> Result<(), ElfError> {
        let header = &*(self.data as *const Elf32Header);
        let phoff = header.e_phoff as usize;
        let phentsize = header.e_phentsize as usize;
        let phnum = header.e_phnum as usize;

        for i in 0..phnum {
            let phdr_ptr = (self.data as usize + phoff + i * phentsize) as *const Elf32Phdr;
            let phdr = &*phdr_ptr;
            self.load_segment_32(phdr)?;
        }

        Ok(())
    }

    unsafe fn load_segment(&self, phdr: &Elf64Phdr) -> Result<(), ElfError> {
        if phdr.p_type != PhType::PT_LOAD as u32 {
            return Ok(());
        }

        let vaddr = phdr.p_vaddr as usize;
        let filesz = phdr.p_filesz as usize;
        let memsz = phdr.p_memsz as usize;
        let offset = phdr.p_offset as usize;

        // Allocate memory for segment
        let mem = alloc(memsz);
        if mem.is_null() {
            return Err(ElfError::AllocationFailed);
        }

        // Zero initialize memory
        ptr::write_bytes(mem, 0, memsz);

        // Copy segment data
        if filesz > 0 {
            let src = (self.data as usize + offset) as *const u8;
            ptr::copy_nonoverlapping(src, mem, filesz);
        }

        // Set memory permissions based on flags
        let is_readable = (phdr.p_flags & 0x1) != 0;
        let is_writable = (phdr.p_flags & 0x2) != 0;
        let is_executable = (phdr.p_flags & 0x4) != 0;

        self.set_memory_permissions(mem, memsz, is_readable, is_writable, is_executable);

        Ok(())
    }

    unsafe fn load_segment_32(&self, phdr: &Elf32Phdr) -> Result<(), ElfError> {
        if phdr.p_type != PhType::PT_LOAD as u32 {
            return Ok(());
        }

        let vaddr = phdr.p_vaddr as usize;
        let filesz = phdr.p_filesz as usize;
        let memsz = phdr.p_memsz as usize;
        let offset = phdr.p_offset as usize;

        // Allocate memory for segment
        let mem = alloc(memsz);
        if mem.is_null() {
            return Err(ElfError::AllocationFailed);
        }

        // Zero initialize memory
        ptr::write_bytes(mem, 0, memsz);

        // Copy segment data
        if filesz > 0 {
            let src = (self.data as usize + offset) as *const u8;
            ptr::copy_nonoverlapping(src, mem, filesz);
        }

        // Set memory permissions
        let is_readable = (phdr.p_flags & 0x1) != 0;
        let is_writable = (phdr.p_flags & 0x2) != 0;
        let is_executable = (phdr.p_flags & 0x4) != 0;

        self.set_memory_permissions(mem, memsz, is_readable, is_writable, is_executable);

        Ok(())
    }

    unsafe fn set_memory_permissions(&self, ptr: *mut u8, size: usize, read: bool, write: bool, exec: bool) {
        // In a real implementation, this would use mprotect or similar
        // For now, this is a placeholder
        let _ = (ptr, size, read, write, exec);
    }

    /// Relocate symbols
    pub unsafe fn relocate(&self) -> Result<(), ElfError> {
        // In a real implementation, this would handle relocations
        // For now, this is a placeholder
        Ok(())
    }

    /// Resolve symbols
    pub unsafe fn resolve_symbols(&self) -> Result<(), ElfError> {
        // In a real implementation, this would resolve symbols from shared libraries
        // For now, this is a placeholder
        Ok(())
    }
}

/// ELF error types
#[derive(Debug)]
pub enum ElfError {
    InvalidMagic,
    InvalidClass,
    AllocationFailed,
    InvalidHeader,
    InvalidSegment,
}

/// ELF loader
pub struct ElfLoader {
    loaded_binaries: [*const ElfBinary; 16],
    binary_count: usize,
}

impl ElfLoader {
    pub fn new() -> Self {
        ElfLoader {
            loaded_binaries: [ptr::null(); 16],
            binary_count: 0,
        }
    }

    /// Load ELF binary
    pub unsafe fn load(&mut self, data: *const u8, size: usize) -> Result<*const ElfBinary, ElfError> {
        let binary = ElfBinary::new(data, size)?;
        
        if self.binary_count >= 16 {
            return Err(ElfError::AllocationFailed);
        }

        binary.load_program_headers()?;
        binary.relocate()?;
        binary.resolve_symbols()?;

        let binary_ptr = alloc(mem::size_of::<ElfBinary>()) as *mut ElfBinary;
        if binary_ptr.is_null() {
            return Err(ElfError::AllocationFailed);
        }

        ptr::write(binary_ptr, binary);
        self.loaded_binaries[self.binary_count] = binary_ptr;
        self.binary_count += 1;

        Ok(binary_ptr)
    }

    /// Get entry point of loaded binary
    pub unsafe fn get_entry_point(&self, binary: *const ElfBinary) -> u64 {
        (*binary).entry_point()
    }

    /// Execute loaded binary
    pub unsafe fn execute(&self, binary: *const ElfBinary) -> ! {
        let entry = (*binary).entry_point();
        let entry_fn: extern "C" fn() = core::mem::transmute(entry);
        entry_fn();
        loop {}
    }
}

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
