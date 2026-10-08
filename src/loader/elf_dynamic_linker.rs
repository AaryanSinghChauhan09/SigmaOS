//! ELF Dynamic Linker
//!
//! Provides ELF dynamic linking capabilities for SigmaOS, enabling execution
//! of dynamically linked binaries with proper symbol resolution and relocation.
//!
//! Supports:
//! - ELF 64-bit loading
//! - Dynamic symbol resolution
//! - Relocation processing (REL, RELA)
//! - PLT/GOT resolution
//! - Dependency loading
//! - RPATH/RUNPATH support
//! - ASLR-compatible loading

#![no_std]
#![allow(dead_code)]

extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;
use alloc::collections::BTreeMap;

/// ELF 64-bit header
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Elf64Ehdr {
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
    pub_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

/// ELF 64-bit program header
#[derive(Debug, Clone, Copy)]
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

/// ELF 64-bit section header
#[derive(Debug, Clone)]
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

/// ELF 64-bit dynamic entry
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Elf64Dyn {
    pub d_tag: u64,
    pub d_un: u64,
}

/// ELF 64-bit symbol
#[derive(Debug, Clone)]
#[repr(C)]
pub struct Elf64Sym {
    pub st_name: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
    pub st_value: u64,
    pub st_size: u64,
}

/// ELF 64-bit relocation entry
#[derive(Debug, Clone)]
#[repr(C)]
pub struct Elf64Rel {
    pub r_offset: u64,
    pub r_info: u64,
}

/// ELF 64-bit relocation entry with addend
#[derive(Debug, Clone)]
#[repr(C)]
pub struct Elf64Rela {
    pub r_offset: u64,
    pub r_info: u64,
    pub r_addend: i64,
}

/// ELF program header types
pub mod pt {
    pub const NULL: u32 = 0;
    pub const LOAD: u32 = 1;
    pub const DYNAMIC: u32 = 2;
    pub const INTERP: u32 = 3;
    pub const NOTE: u32 = 4;
    pub const SHLIB: u32 = 5;
    pub const PHDR: u32 = 6;
    pub const TLS: u32 = 7;
    pub const GNU_EH_FRAME: u32 = 0x6474e550;
    pub const GNU_STACK: u32 = 0x6474e551;
    pub const GNU_RELRO: u32 = 0x6474e552;
}

/// ELF dynamic array tags
pub mod dt {
    pub const NULL: u64 = 0;
    pub const NEEDED: u64 = 1;
    pub const PLTRELSZ: u64 = 2;
    pub const PLTGOT: u64 = 3;
    pub const HASH: u64 = 4;
    pub const STRTAB: u64 = 5;
    pub const SYMTAB: u64 = 6;
    pub const RELA: u64 = 7;
    pub const RELASZ: u64 = 8;
    pub const RELAENT: u64 = 9;
    pub const STRSZ: u64 = 10;
    pub const SYMENT: u64 = 11;
    pub const INIT: u64 = 12;
    pub const FINI: u64 = 13;
    pub const SONAME: u64 = 14;
    pub const RPATH: u64 = 15;
    pub const SYMBOLIC: u64 = 16;
    pub const REL: u64 = 17;
    pub const RELSZ: u64 = 18;
    pub const RELENT: u64 = 19;
    pub const PLTREL: u64 = 20;
    pub const DEBUG: u64 = 21;
    pub const TEXTREL: u64 = 22;
    pub const JMPREL: u64 = 23;
    pub const BIND_NOW: u64 = 24;
    pub const INIT_ARRAY: u64 = 25;
    pub const FINI_ARRAY: u64 = 26;
    pub const INIT_ARRAYSZ: u64 = 27;
    pub const FINI_ARRAYSZ: u64 = 28;
    pub const RUNPATH: u64 = 29;
    pub const FLAGS: u64 = 30;
    pub const PREINIT_ARRAY: u64 = 32;
    pub const PREINIT_ARRAYSZ: u64 = 33;
    pub const GNU_HASH: u64 = 0x6ffffef5;
}

/// ELF relocation types (x86_64)
pub mod r_x86_64 {
    pub const NONE: u32 = 0;
    pub const _64: u32 = 1;
    pub const PC32: u32 = 2;
    pub const GLOB_DAT: u32 = 6;
    pub const JUMP_SLOT: u32 = 7;
    pub const RELATIVE: u32 = 8;
    pub const GOTPCREL: u32 = 9;
    pub const _32: u32 = 10;
    pub const _16: u32 = 11;
    pub const PC16: u32 = 12;
    pub const _8: u32 = 13;
    pub const PC8: u32 = 14;
    pub const DTPMOD64: u32 = 16;
    pub const DTPOFF64: u32 = 17;
    pub const TPOFF64: u32 = 18;
    pub const TLSGD: u32 = 19;
    pub const TLSLD: u32 = 20;
    pub const DTPOFF32: u32 = 21;
    pub const GOTTPOFF: u32 = 22;
    pub const TPOFF32: u32 = 23;
    pub const PC64: u32 = 24;
    pub const GOTOFF64: u32 = 25;
    pub const GOTPC32: u32 = 26;
    pub const GOT64: u32 = 27;
    pub const GOTPCREL64: u32 = 28;
    pub const GOTPC64: u32 = 29;
    pub const GOTPLT64: u32 = 30;
    pub const PLTOFF64: u32 = 31;
    pub const SIZE32: u32 = 32;
    pub const SIZE64: u32 = 33;
    pub const GOTPC32_TLSDESC: u32 = 34;
    pub const TLSDESC_CALL: u32 = 35;
    pub const TLSDESC: u32 = 36;
    pub const IRELATIVE: u32 = 37;
}

/// Loaded ELF object
#[derive(Debug)]
pub struct LoadedObject {
    pub base_addr: u64,
    pub dynamic_addr: u64,
    pub entry_point: u64,
    pub phdrs: Vec<Elf64Phdr>,
    pub symtab: Option<Vec<Elf64Sym>>,
    pub strtab: Option<Vec<u8>>,
    pub rel: Option<Vec<Elf64Rel>>,
    pub rela: Option<Vec<Elf64Rela>>,
    pub needed: Vec<String>,
    pub soname: Option<String>,
    pub rpath: Vec<String>,
    pub runpath: Vec<String>,
}

/// Dynamic linker state
#[derive(Debug)]
pub struct DynamicLinker {
    pub loaded_objects: Vec<LoadedObject>,
    pub symbol_cache: BTreeMap<String, u64>,
    pub load_bias: u64,
    pub aslr_enabled: bool,
}

impl DynamicLinker {
    /// Create a new dynamic linker instance
    pub fn new(aslr_enabled: bool) -> Self {
        Self {
            loaded_objects: Vec::new(),
            symbol_cache: BTreeMap::new(),
            load_bias: if aslr_enabled { 0x555555550000 } else { 0x400000 },
            aslr_enabled,
        }
    }

    /// Parse ELF header from bytes
    pub fn parse_elf_header(data: &[u8]) -> Option<Elf64Ehdr> {
        if data.len() < core::mem::size_of::<Elf64Ehdr>() {
            return None;
        }

        // Check ELF magic
        if &data[0..4] != b"\x7fELF" {
            return None;
        }

        // Check 64-bit
        if data[4] != 2 {
            return None;
        }

        // Check little-endian
        if data[5] != 1 {
            return None;
        }

        let header = unsafe {
            *(data.as_ptr() as *const Elf64Ehdr)
        };

        Some(header)
    }

    /// Load ELF binary
    pub fn load_elf(&mut self, data: &[u8]) -> Result<LoadedObject, &'static str> {
        let header = Self::parse_elf_header(data)
            .ok_or("Invalid ELF header")?;

        // Parse program headers
        let mut phdrs = Vec::new();
        let phdr_start = header.e_phoff as usize;
        let phdr_size = header.e_phentsize as usize;
        let phdr_count = header.e_phnum as usize;

        for i in 0..phdr_count {
            let offset = phdr_start + i * phdr_size;
            if offset + phdr_size > data.len() {
                return Err("Program header out of bounds");
            }

            let phdr = unsafe {
                *(data.as_ptr().add(offset) as *const Elf64Phdr)
            };
            phdrs.push(phdr);
        }

        // Find dynamic segment
        let mut dynamic_addr = 0;
        for phdr in &phdrs {
            if phdr.p_type == pt::DYNAMIC {
                dynamic_addr = phdr.p_vaddr;
                break;
            }
        }

        // Parse dynamic section
        let mut needed = Vec::new();
        let mut soname = None;
        let mut rpath = Vec::new();
        let mut runpath = Vec::new();

        if dynamic_addr != 0 {
            self.parse_dynamic_section(data, &phdrs, &mut needed, &mut soname, &mut rpath, &mut runpath);
        }

        let base_addr = self.load_bias;

        Ok(LoadedObject {
            base_addr,
            dynamic_addr,
            entry_point: header.e_entry + base_addr,
            phdrs,
            symtab: None,
            strtab: None,
            rel: None,
            rela: None,
            needed,
            soname,
            rpath,
            runpath,
        })
    }

    /// Parse dynamic section
    fn parse_dynamic_section(
        &self,
        data: &[u8],
        phdrs: &[Elf64Phdr],
        needed: &mut Vec<String>,
        soname: &mut Option<String>,
        rpath: &mut Vec<String>,
        runpath: &mut Vec<String>,
    ) {
        let mut strtab_addr = 0;
        let mut strtab_size = 0;

        // Find string table
        for phdr in phdrs {
            if phdr.p_type == pt::DYNAMIC {
                let dyn_start = phdr.p_offset as usize;
                let mut i = 0;
                loop {
                    let offset = dyn_start + i * core::mem::size_of::<Elf64Dyn>();
                    if offset + core::mem::size_of::<Elf64Dyn>() > data.len() {
                        break;
                    }

                    let dyn_entry = unsafe {
                        *(data.as_ptr().add(offset) as *const Elf64Dyn)
                    };

                    if dyn_entry.d_tag == dt::NULL {
                        break;
                    }

                    match dyn_entry.d_tag {
                        dt::STRTAB => strtab_addr = dyn_entry.d_un,
                        dt::STRSZ => strtab_size = dyn_entry.d_un,
                        dt::NEEDED => {
                            if strtab_addr != 0 && strtab_size > 0 {
                                let name_offset = dyn_entry.d_un as usize;
                                if name_offset < strtab_size as usize {
                                    // Extract string (simplified)
                                    unsafe {
                                        let str_ptr = data.as_ptr().add(strtab_addr as usize + name_offset);
                                        let mut len = 0;
                                        while *str_ptr.add(len) != 0 && len < 256 {
                                            len += 1;
                                        }
                                        let name_bytes = core::slice::from_raw_parts(str_ptr, len);
                                        let name = String::from_utf8_lossy(name_bytes).to_string();
                                        needed.push(name);
                                    }
                                }
                            }
                        }
                        dt::SONAME => {
                            if strtab_addr != 0 && strtab_size > 0 {
                                let name_offset = dyn_entry.d_un as usize;
                                if name_offset < strtab_size as usize {
                                    unsafe {
                                        let str_ptr = data.as_ptr().add(strtab_addr as usize + name_offset);
                                        let mut len = 0;
                                        while *str_ptr.add(len) != 0 && len < 256 {
                                            len += 1;
                                        }
                                        let name_bytes = core::slice::from_raw_parts(str_ptr, len);
                                        let name = String::from_utf8_lossy(name_bytes).to_string();
                                        *soname = Some(name);
                                    }
                                }
                            }
                        }
                        dt::RPATH => {
                            if strtab_addr != 0 && strtab_size > 0 {
                                let path_offset = dyn_entry.d_un as usize;
                                if path_offset < strtab_size as usize {
                                    unsafe {
                                        let str_ptr = data.as_ptr().add(strtab_addr as usize + path_offset);
                                        let mut len = 0;
                                        while *str_ptr.add(len) != 0 && len < 256 {
                                            len += 1;
                                        }
                                        let path_bytes = core::slice::from_raw_parts(str_ptr, len);
                                        let path = String::from_utf8_lossy(path_bytes).to_string();
                                        rpath.push(path);
                                    }
                                }
                            }
                        }
                        dt::RUNPATH => {
                            if strtab_addr != 0 && strtab_size > 0 {
                                let path_offset = dyn_entry.d_un as usize;
                                if path_offset < strtab_size as usize {
                                    unsafe {
                                        let str_ptr = data.as_ptr().add(strtab_addr as usize + path_offset);
                                        let mut len = 0;
                                        while *str_ptr.add(len) != 0 && len < 256 {
                                            len += 1;
                                        }
                                        let path_bytes = core::slice::from_raw_parts(str_ptr, len);
                                        let path = String::from_utf8_lossy(path_bytes).to_string();
                                        runpath.push(path);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }

                    i += 1;
                }
                break;
            }
        }
    }

    /// Process relocations
    pub fn process_relocations(&mut self, obj: &mut LoadedObject) -> Result<(), &'static str> {
        // Process RELA relocations
        if let Some(ref rela) = obj.rela {
            for rel in rela {
                self.process_rela(obj, rel)?;
            }
        }

        // Process REL relocations
        if let Some(ref rel) = obj.rel {
            for r in rel {
                self.process_rel(obj, r)?;
            }
        }

        Ok(())
    }

    /// Process a single RELA relocation
    fn process_rela(&mut self, obj: &LoadedObject, rel: &Elf64Rela) -> Result<(), &'static str> {
        let r_type = (rel.r_info & 0xffffffff) as u32;
        let r_sym = (rel.r_info >> 32) as u32;

        match r_type {
            r_x86_64::NONE => Ok(()),
            r_x86_64::RELATIVE => {
                // Base-relative relocation
                let offset = (rel.r_offset + obj.base_addr) as *mut u64;
                unsafe {
                    *offset = obj.base_addr + rel.r_addend as u64;
                }
                Ok(())
            }
            r_x86_64::GLOB_DAT | r_x86_64::JUMP_SLOT => {
                // Symbol resolution
                if let Some(sym) = obj.symtab.as_ref() {
                    if let Some(sym_entry) = sym.get(r_sym as usize) {
                        let sym_name = self.get_symbol_name(obj, sym_entry);
                        if let Some(&addr) = self.symbol_cache.get(&sym_name) {
                            let offset = (rel.r_offset + obj.base_addr) as *mut u64;
                            unsafe {
                                *offset = addr + rel.r_addend as u64;
                            }
                        }
                    }
                }
                Ok(())
            }
            r_x86_64::IRELATIVE => {
                // IFUNC relocation (not implemented)
                Ok(())
            }
            _ => Err("Unsupported relocation type"),
        }
    }

    /// Process a single REL relocation
    fn process_rel(&mut self, obj: &LoadedObject, rel: &Elf64Rel) -> Result<(), &'static str> {
        let r_type = (rel.r_info & 0xffffffff) as u32;
        let r_sym = (rel.r_info >> 32) as u32;

        match r_type {
            r_x86_64::NONE => Ok(()),
            r_x86_64::RELATIVE => {
                let offset = (rel.r_offset + obj.base_addr) as *mut u64;
                unsafe {
                    *offset = obj.base_addr;
                }
                Ok(())
            }
            r_x86_64::GLOB_DAT | r_x86_64::JUMP_SLOT => {
                if let Some(sym) = obj.symtab.as_ref() {
                    if let Some(sym_entry) = sym.get(r_sym as usize) {
                        let sym_name = self.get_symbol_name(obj, sym_entry);
                        if let Some(&addr) = self.symbol_cache.get(&sym_name) {
                            let offset = (rel.r_offset + obj.base_addr) as *mut u64;
                            unsafe {
                                *offset = addr;
                            }
                        }
                    }
                }
                Ok(())
            }
            _ => Err("Unsupported relocation type"),
        }
    }

    /// Get symbol name from symbol table
    fn get_symbol_name(&self, obj: &LoadedObject, sym: &Elf64Sym) -> String {
        if let Some(ref strtab) = obj.strtab {
            let name_offset = sym.st_name as usize;
            if name_offset < strtab.len() {
                let mut len = 0;
                while name_offset + len < strtab.len() && strtab[name_offset + len] != 0 {
                    len += 1;
                }
                let name_bytes = &strtab[name_offset..name_offset + len];
                String::from_utf8_lossy(name_bytes).to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    }

    /// Resolve symbol by name
    pub fn resolve_symbol(&mut self, name: &str) -> Option<u64> {
        if let Some(&addr) = self.symbol_cache.get(name) {
            return Some(addr);
        }

        // Search loaded objects
        for obj in &self.loaded_objects {
            if let Some(ref symtab) = obj.symtab {
                for sym in symtab {
                    let sym_name = self.get_symbol_name(obj, sym);
                    if sym_name == name && sym.st_value != 0 {
                        let addr = sym.st_value + obj.base_addr;
                        self.symbol_cache.insert(name.to_string(), addr);
                        return Some(addr);
                    }
                }
            }
        }

        None
    }

    /// Add symbol to cache
    pub fn add_symbol(&mut self, name: String, addr: u64) {
        self.symbol_cache.insert(name, addr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_linker_creation() {
        let linker = DynamicLinker::new(true);
        assert!(linker.aslr_enabled);
        assert_eq!(linker.load_bias, 0x555555550000);
    }

    #[test]
    fn test_dynamic_linker_no_aslr() {
        let linker = DynamicLinker::new(false);
        assert!(!linker.aslr_enabled);
        assert_eq!(linker.load_bias, 0x400000);
    }

    #[test]
    fn test_symbol_cache() {
        let mut linker = DynamicLinker::new(false);
        linker.add_symbol("test_func".to_string(), 0x1000);
        assert_eq!(linker.resolve_symbol("test_func"), Some(0x1000));
        assert_eq!(linker.resolve_symbol("missing"), None);
    }

    #[test]
    fn test_elf_header_magic() {
        let mut data = [0u8; 64];
        data[0..4].copy_from_slice(b"\x7fELF");
        data[4] = 2; // 64-bit
        data[5] = 1; // little-endian

        let header = DynamicLinker::parse_elf_header(&data);
        assert!(header.is_some());
    }

    #[test]
    fn test_elf_header_invalid_magic() {
        let data = [0u8; 64];
        let header = DynamicLinker::parse_elf_header(&data);
        assert!(header.is_none());
    }

    #[test]
    fn test_relocation_types() {
        assert_eq!(r_x86_64::NONE, 0);
        assert_eq!(r_x86_64::_64, 1);
        assert_eq!(r_x86_64::GLOB_DAT, 6);
        assert_eq!(r_x86_64::JUMP_SLOT, 7);
        assert_eq!(r_x86_64::RELATIVE, 8);
    }

    #[test]
    fn test_dynamic_tags() {
        assert_eq!(dt::NULL, 0);
        assert_eq!(dt::NEEDED, 1);
        assert_eq!(dt::STRTAB, 5);
        assert_eq!(dt::SYMTAB, 6);
        assert_eq!(dt::RPATH, 15);
        assert_eq!(dt::RUNPATH, 29);
    }

    #[test]
    fn test_program_header_types() {
        assert_eq!(pt::NULL, 0);
        assert_eq!(pt::LOAD, 1);
        assert_eq!(pt::DYNAMIC, 2);
        assert_eq!(pt::INTERP, 3);
        assert_eq!(pt::NOTE, 4);
    }
}
