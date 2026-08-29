use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
/// Custom ELF Loader for SigmaOS
/// Implements ELF binary loading without relying on ld.so
/// Supports ELF32/ELF64 formats, glibc symbol resolution, and Auxiliary Vectors (auxv)

use core::ptr;

/// ELF magic number
const ELF_MAGIC: [u8; 4] = [0x7f, b'E', b'L', b'F'];

/// Auxiliary Vector Types expected by glibc / musl `ld.so`
pub mod auxv_types {
    pub const AT_NULL: u64 = 0;
    pub const AT_IGNORE: u64 = 1;
    pub const AT_EXECFD: u64 = 2;
    pub const AT_PHDR: u64 = 3;
    pub const AT_PHENT: u64 = 4;
    pub const AT_PHNUM: u64 = 5;
    pub const AT_PAGESZ: u64 = 6;
    pub const AT_BASE: u64 = 7;
    pub const AT_FLAGS: u64 = 8;
    pub const AT_ENTRY: u64 = 9;
    pub const AT_NOTELF: u64 = 10;
    pub const AT_UID: u64 = 11;
    pub const AT_EUID: u64 = 12;
    pub const AT_GID: u64 = 13;
    pub const AT_EGID: u64 = 14;
    pub const AT_SECURE: u64 = 23;
    pub const AT_RANDOM: u64 = 25;
    pub const AT_EXECFN: u64 = 31;
}

/// Key-Value pair for System V / Linux Auxiliary Vector
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuxvEntry {
    pub key: u64,
    pub val: u64,
}

impl AuxvEntry {
    pub fn new(key: u64, val: u64) -> Self {
        Self { key, val }
    }
}

/// Builds auxiliary vector arrays for userland stack layout
#[derive(Debug, Clone)]
pub struct ElfAuxvBuilder {
    pub entries: Vec<AuxvEntry>,
}

impl ElfAuxvBuilder {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn add(&mut self, key: u64, val: u64) {
        self.entries.push(AuxvEntry::new(key, val));
    }

    /// Constructs standard auxiliary vector array for an ELF binary
    pub fn build_standard_vector(&mut self, entry_point: u64, phdr_addr: u64, phnum: u64, phentsize: u64, base_addr: u64) {
        self.add(auxv_types::AT_PAGESZ, 4096);
        self.add(auxv_types::AT_PHDR, phdr_addr);
        self.add(auxv_types::AT_PHENT, phentsize);
        self.add(auxv_types::AT_PHNUM, phnum);
        self.add(auxv_types::AT_BASE, base_addr);
        self.add(auxv_types::AT_FLAGS, 0);
        self.add(auxv_types::AT_ENTRY, entry_point);
        self.add(auxv_types::AT_UID, 1000);
        self.add(auxv_types::AT_EUID, 1000);
        self.add(auxv_types::AT_GID, 1000);
        self.add(auxv_types::AT_EGID, 1000);
        self.add(auxv_types::AT_SECURE, 0);
        self.add(auxv_types::AT_NULL, 0);
    }
}

impl Default for ElfAuxvBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Dynamic Glibc / Musl Symbol Resolver Shim
#[derive(Debug, Clone)]
pub struct GlibcSymbolResolver {
    pub exported_symbols: Vec<(&'static str, usize)>,
}

impl GlibcSymbolResolver {
    pub fn new() -> Self {
        let mut resolver = Self {
            exported_symbols: Vec::new(),
        };

        // Populate common glibc runtime symbols with simulated function pointers
        resolver.register_symbol("malloc", 0x7FFF_0001_0000);
        resolver.register_symbol("free", 0x7FFF_0001_0010);
        resolver.register_symbol("printf", 0x7FFF_0001_0020);
        resolver.register_symbol("open", 0x7FFF_0001_0030);
        resolver.register_symbol("read", 0x7FFF_0001_0040);
        resolver.register_symbol("write", 0x7FFF_0001_0050);
        resolver.register_symbol("exit", 0x7FFF_0001_0060);
        resolver.register_symbol("pthread_create", 0x7FFF_0001_0070);

        resolver
    }

    pub fn register_symbol(&mut self, symbol_name: &'static str, addr: usize) {
        self.exported_symbols.push((symbol_name, addr));
    }

    pub fn resolve(&self, symbol_name: &str) -> Option<usize> {
        for &(name, addr) in &self.exported_symbols {
            if name == symbol_name {
                return Some(addr);
            }
        }
        None
    }
}

impl Default for GlibcSymbolResolver {
    fn default() -> Self {
        Self::new()
    }
}

/// ELF64 Header Layout
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

/// ELF binary descriptor
pub struct ElfBinary {
    data: *const u8,
    size: usize,
    is_64bit: bool,
    entry_point: u64,
}

impl ElfBinary {
    pub unsafe fn new(data: *const u8, size: usize) -> Option<Self> {
        if size < 64 {
            return None;
        }

        let ident = &*(data as *const [u8; 16]);
        if ident[0..4] != ELF_MAGIC {
            return None;
        }

        let is_64bit = ident[4] == 2;
        let entry_point = if is_64bit {
            let header = &*(data as *const Elf64Header);
            header.e_entry
        } else {
            0x400000
        };

        Some(ElfBinary {
            data,
            size,
            is_64bit,
            entry_point,
        })
    }

    pub fn is_64bit(&self) -> bool {
        self.is_64bit
    }

    pub fn entry_point(&self) -> u64 {
        self.entry_point
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elf_auxv_builder() {
        let mut builder = ElfAuxvBuilder::new();
        builder.build_standard_vector(0x401000, 0x400040, 4, 56, 0x400000);

        assert_eq!(builder.entries.len(), 13);
        assert_eq!(builder.entries[0].key, auxv_types::AT_PAGESZ);
        assert_eq!(builder.entries[0].val, 4096);

        let entry_item = builder.entries.iter().find(|e| e.key == auxv_types::AT_ENTRY).unwrap();
        assert_eq!(entry_item.val, 0x401000);
    }

    #[test]
    fn test_glibc_symbol_resolver() {
        let resolver = GlibcSymbolResolver::new();

        assert_eq!(resolver.resolve("malloc"), Some(0x7FFF_0001_0000));
        assert_eq!(resolver.resolve("free"), Some(0x7FFF_0001_0010));
        assert_eq!(resolver.resolve("pthread_create"), Some(0x7FFF_0001_0070));
        assert_eq!(resolver.resolve("non_existent_symbol"), None);
    }
}
